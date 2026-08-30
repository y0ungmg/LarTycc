use std::error::Error;
use std::ffi::{c_char, c_float, c_uint, c_void, CStr, CString};
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ptr::NonNull;

mod ffi {
    #![allow(unsafe_code)]

    use super::{c_char, c_float, c_uint, c_void};

    unsafe extern "C" {
        pub fn lartycc_audio_host_create() -> *mut c_void;
        pub fn lartycc_audio_host_destroy(host: *mut c_void);
        pub fn lartycc_audio_host_available(host: *const c_void) -> bool;
        pub fn lartycc_audio_host_refresh_devices(host: *mut c_void) -> usize;
        pub fn lartycc_audio_host_device_info(
            host: *const c_void,
            index: usize,
            id: *mut c_char,
            id_capacity: usize,
            name: *mut c_char,
            name_capacity: usize,
            is_default: *mut bool,
        ) -> bool;
        pub fn lartycc_audio_host_load_mono(
            host: *mut c_void,
            samples: *const c_float,
            sample_count: usize,
        ) -> bool;
        pub fn lartycc_audio_host_start(
            host: *mut c_void,
            device_id: *const c_char,
            sample_rate: c_uint,
            period_frames: c_uint,
        ) -> bool;
        pub fn lartycc_audio_host_stop(host: *mut c_void);
        pub fn lartycc_audio_host_play(host: *mut c_void) -> bool;
        pub fn lartycc_audio_host_seek(host: *mut c_void, frame: usize) -> bool;
        pub fn lartycc_audio_host_set_gain(host: *mut c_void, gain: c_float);
        pub fn lartycc_audio_host_is_playing(host: *const c_void) -> bool;
        pub fn lartycc_audio_host_position(host: *const c_void) -> usize;
        pub fn lartycc_audio_host_callback_count(host: *const c_void) -> usize;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioError {
    CreationFailed,
    DeviceInfoFailed,
    InvalidDeviceId,
    LoadFailed,
    StartFailed,
    PlayFailed,
    SeekFailed,
}

impl Display for AudioError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Self::CreationFailed => "could not create the native audio host",
                Self::DeviceInfoFailed => "could not read native audio device information",
                Self::InvalidDeviceId => "audio device ID contains a null byte",
                Self::LoadFailed => "could not load the mono sample",
                Self::StartFailed => "could not start the selected audio device",
                Self::PlayFailed => "audio transport could not start",
                Self::SeekFailed => "audio transport seek was rejected",
            }
        )
    }
}

impl Error for AudioError {}

pub struct AudioHost {
    raw: NonNull<c_void>,
    _not_send_or_sync: PhantomData<*mut ()>,
}

#[allow(unsafe_code)]
impl AudioHost {
    /// Creates the process-local native audio host.
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::CreationFailed`] when native allocation or backend
    /// initialization cannot create the opaque host.
    pub fn new() -> Result<Self, AudioError> {
        // SAFETY: The constructor takes no pointers and returns an owned opaque handle.
        let raw = unsafe { ffi::lartycc_audio_host_create() };
        Ok(Self {
            raw: NonNull::new(raw).ok_or(AudioError::CreationFailed)?,
            _not_send_or_sync: PhantomData,
        })
    }

    #[must_use]
    pub fn is_available(&self) -> bool {
        // SAFETY: `raw` remains valid for the lifetime of `self`.
        unsafe { ffi::lartycc_audio_host_available(self.raw.as_ptr()) }
    }

    /// Refreshes and returns the playback-device snapshot.
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::DeviceInfoFailed`] if a native device record cannot
    /// be copied into Rust-owned strings.
    pub fn devices(&mut self) -> Result<Vec<DeviceInfo>, AudioError> {
        // SAFETY: `raw` is exclusively borrowed while the native cache is refreshed.
        let count = unsafe { ffi::lartycc_audio_host_refresh_devices(self.raw.as_ptr()) };
        (0..count).map(|index| self.device(index)).collect()
    }

    fn device(&self, index: usize) -> Result<DeviceInfo, AudioError> {
        const BUFFER_SIZE: usize = 1024;
        let mut id = [0_i8; BUFFER_SIZE];
        let mut name = [0_i8; BUFFER_SIZE];
        let mut is_default = false;
        // SAFETY: Both writable buffers have the supplied capacity and `raw` is valid.
        let success = unsafe {
            ffi::lartycc_audio_host_device_info(
                self.raw.as_ptr(),
                index,
                id.as_mut_ptr(),
                id.len(),
                name.as_mut_ptr(),
                name.len(),
                &mut is_default,
            )
        };
        if !success {
            return Err(AudioError::DeviceInfoFailed);
        }
        // SAFETY: The C API guarantees null termination on success.
        let id = unsafe { CStr::from_ptr(id.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        // SAFETY: The C API guarantees null termination on success.
        let name = unsafe { CStr::from_ptr(name.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        Ok(DeviceInfo {
            id,
            name,
            is_default,
        })
    }

    /// Copies a prepared mono sample into the native engine.
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::LoadFailed`] for an empty sample or while the device
    /// callback is running.
    pub fn load_mono(&mut self, samples: &[f32]) -> Result<(), AudioError> {
        // SAFETY: The slice pointer is valid for its length for the duration of the call.
        unsafe {
            ffi::lartycc_audio_host_load_mono(self.raw.as_ptr(), samples.as_ptr(), samples.len())
        }
        .then_some(())
        .ok_or(AudioError::LoadFailed)
    }

    /// Opens the selected device, or the backend default when no ID is supplied.
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::InvalidDeviceId`] for an embedded null byte and
    /// [`AudioError::StartFailed`] when the device cannot be configured or opened.
    pub fn start(
        &mut self,
        device_id: Option<&str>,
        sample_rate: u32,
        period_frames: u32,
    ) -> Result<(), AudioError> {
        let device_id = device_id
            .map(CString::new)
            .transpose()
            .map_err(|_| AudioError::InvalidDeviceId)?;
        let pointer = device_id
            .as_ref()
            .map_or(std::ptr::null(), |id| id.as_ptr());
        // SAFETY: The optional C string remains alive through the call and `raw` is valid.
        unsafe {
            ffi::lartycc_audio_host_start(self.raw.as_ptr(), pointer, sample_rate, period_frames)
        }
        .then_some(())
        .ok_or(AudioError::StartFailed)
    }

    /// Starts transport after a sample is loaded and a device is running.
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::PlayFailed`] when those preconditions are not met.
    pub fn play(&mut self) -> Result<(), AudioError> {
        // SAFETY: `raw` is valid and exclusively borrowed.
        unsafe { ffi::lartycc_audio_host_play(self.raw.as_ptr()) }
            .then_some(())
            .ok_or(AudioError::PlayFailed)
    }

    pub fn stop(&mut self) {
        // SAFETY: `raw` is valid and exclusively borrowed.
        unsafe { ffi::lartycc_audio_host_stop(self.raw.as_ptr()) };
    }

    /// Moves transport to a frame within the loaded sample.
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::SeekFailed`] when the frame is outside the sample.
    pub fn seek(&mut self, frame: usize) -> Result<(), AudioError> {
        // SAFETY: `raw` is valid and exclusively borrowed.
        unsafe { ffi::lartycc_audio_host_seek(self.raw.as_ptr(), frame) }
            .then_some(())
            .ok_or(AudioError::SeekFailed)
    }

    pub fn set_gain(&mut self, gain: f32) {
        // SAFETY: `raw` is valid and exclusively borrowed.
        unsafe { ffi::lartycc_audio_host_set_gain(self.raw.as_ptr(), gain) };
    }

    #[must_use]
    pub fn is_playing(&self) -> bool {
        // SAFETY: `raw` remains valid for the lifetime of `self`.
        unsafe { ffi::lartycc_audio_host_is_playing(self.raw.as_ptr()) }
    }

    #[must_use]
    pub fn position(&self) -> usize {
        // SAFETY: `raw` remains valid for the lifetime of `self`.
        unsafe { ffi::lartycc_audio_host_position(self.raw.as_ptr()) }
    }

    #[must_use]
    pub fn callback_count(&self) -> usize {
        // SAFETY: `raw` remains valid for the lifetime of `self`.
        unsafe { ffi::lartycc_audio_host_callback_count(self.raw.as_ptr()) }
    }
}

#[allow(unsafe_code)]
impl Drop for AudioHost {
    fn drop(&mut self) {
        // SAFETY: This is the single matching destroy call for the owned handle.
        unsafe { ffi::lartycc_audio_host_destroy(self.raw.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::AudioHost;

    #[test]
    fn native_host_loads_sample_and_tracks_seek() {
        let mut host = AudioHost::new().expect("native host");
        let _available = host.is_available();
        let _devices = host.devices().expect("device snapshot");
        host.load_mono(&[0.0, 0.5, -0.5, 0.0]).expect("sample");
        host.seek(2).expect("seek");
        assert_eq!(host.position(), 2);
        assert!(!host.is_playing());
        assert_eq!(host.callback_count(), 0);
    }

    #[test]
    fn empty_sample_is_rejected() {
        let mut host = AudioHost::new().expect("native host");
        assert!(host.load_mono(&[]).is_err());
    }
}
