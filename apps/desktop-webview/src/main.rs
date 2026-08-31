use lartycc_desktop::webview_contract::{
    asset_relative_path, content_type, resolution_script, APP_SCHEME, APP_URL,
    INITIALIZATION_SCRIPT, MAX_IPC_BYTES,
};
use lartycc_desktop::HostRouter;
use std::borrow::Cow;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tao::window::WindowBuilder;
use wry::http::{Request, Response, StatusCode};
use wry::{NewWindowResponse, WebViewBuilder};

enum UserEvent {
    Invoke(String),
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = std::env::args_os().skip(1);
    let project_path = arguments
        .next()
        .map_or_else(|| PathBuf::from("LarTycc-demo.json"), PathBuf::from);
    let ui_dist = arguments.next().map(PathBuf::from);
    run(&project_path, ui_dist.as_deref())
}

fn run(project_path: &Path, ui_dist: Option<&Path>) -> Result<(), Box<dyn Error>> {
    let asset_root = locate_assets(ui_dist)?;
    let mut router = HostRouter::open(project_path)?;
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let window = WindowBuilder::new()
        .with_title("LarTycc")
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 800.0))
        .with_min_inner_size(tao::dpi::LogicalSize::new(960.0, 600.0))
        .build(&event_loop)?;

    let proxy = event_loop.create_proxy();
    let builder = WebViewBuilder::new()
        .with_custom_protocol(APP_SCHEME.to_owned(), move |_webview_id, request| {
            asset_response(&asset_root, request)
        })
        .with_initialization_script(INITIALIZATION_SCRIPT)
        .with_ipc_handler(move |request: Request<String>| {
            let body = if request.body().len() <= MAX_IPC_BYTES {
                request.body().clone()
            } else {
                "{".to_owned()
            };
            let _ = proxy.send_event(UserEvent::Invoke(body));
        })
        .with_navigation_handler(is_app_url)
        .with_new_window_req_handler(|_, _| NewWindowResponse::Deny)
        .with_download_started_handler(|_, _| false)
        .with_clipboard(false)
        .with_hotkeys_zoom(false)
        .with_devtools(cfg!(debug_assertions))
        .with_url(APP_URL);

    #[cfg(target_os = "linux")]
    let webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let container = window
            .default_vbox()
            .ok_or("Tao did not create a GTK container")?;
        builder.build_gtk(container)?
    };
    #[cfg(not(target_os = "linux"))]
    let webview = builder.build(&window)?;

    let mut webview = Some(webview);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::UserEvent(UserEvent::Invoke(request)) => {
                let response = router.invoke_json(&request);
                if let (Some(webview), Ok(script)) =
                    (webview.as_ref(), resolution_script(&response))
                {
                    let _ = webview.evaluate_script(&script);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                let _ = webview.take();
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

fn locate_assets(explicit: Option<&Path>) -> Result<PathBuf, Box<dyn Error>> {
    let root = explicit.map_or_else(
        || {
            std::env::var_os("LARTYCC_UI_DIST")
                .map_or_else(|| PathBuf::from("ui/dist"), PathBuf::from)
        },
        Path::to_path_buf,
    );
    let canonical = fs::canonicalize(&root)?;
    if !canonical.join("index.html").is_file() {
        return Err(format!("{} does not contain index.html", canonical.display()).into());
    }
    Ok(canonical)
}

fn is_app_url(url: String) -> bool {
    url.starts_with("lartycc://localhost/") || url.starts_with("http://lartycc.localhost/")
}

fn asset_response(root: &Path, request: Request<Vec<u8>>) -> Response<Cow<'static, [u8]>> {
    let Some(relative) = asset_relative_path(request.uri().path()) else {
        return response(
            StatusCode::BAD_REQUEST,
            "text/plain; charset=utf-8",
            b"invalid path".to_vec(),
        );
    };
    let candidate = root.join(relative);
    let Ok(canonical) = fs::canonicalize(candidate) else {
        return response(
            StatusCode::NOT_FOUND,
            "text/plain; charset=utf-8",
            b"not found".to_vec(),
        );
    };
    if !canonical.starts_with(root) || !canonical.is_file() {
        return response(
            StatusCode::FORBIDDEN,
            "text/plain; charset=utf-8",
            b"forbidden".to_vec(),
        );
    }
    match fs::read(canonical) {
        Ok(body) => response(StatusCode::OK, content_type(relative), body),
        Err(_) => response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "text/plain; charset=utf-8",
            b"asset read failed".to_vec(),
        ),
    }
}

fn response(status: StatusCode, mime: &'static str, body: Vec<u8>) -> Response<Cow<'static, [u8]>> {
    Response::builder()
        .status(status)
        .header("content-type", mime)
        .header("x-content-type-options", "nosniff")
        .header("cache-control", "no-store")
        .header(
            "content-security-policy",
            "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'none'; object-src 'none'; base-uri 'none'; frame-ancestors 'none'",
        )
        .body(Cow::Owned(body))
        .unwrap_or_else(|_| Response::new(Cow::Owned(b"response build failed".to_vec())))
}
