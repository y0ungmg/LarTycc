import { useEffect, useMemo, useState } from "react";
import { createHostBridge, type AudioDevice, type HostBridge, type TransportSnapshot } from "./host";
import "./styles.css";

const peaks = [18, 42, 64, 35, 78, 52, 88, 45, 68, 30, 72, 54, 82, 38, 60, 24];
const initialTransport: TransportSnapshot = { playing: false, positionFrames: 0, sampleRate: 48_000, callbackCount: 0 };

export function App({ host: injectedHost }: { host?: HostBridge }) {
  const host = useMemo(() => injectedHost ?? createHostBridge(), [injectedHost]);
  const [transport, setTransport] = useState(initialTransport);
  const [tempo, setTempo] = useState(120);
  const [devices, setDevices] = useState<AudioDevice[]>([]);
  const [deviceId, setDeviceId] = useState("");

  useEffect(() => {
    const unsubscribe = host.subscribeTransport(setTransport);
    void host.getState().then((state) => setTempo(state.project.tempo));
    void host.listDevices().then((items) => {
      setDevices(items);
      setDeviceId(items.find((item) => item.isDefault)?.id ?? items[0]?.id ?? "");
    });
    return unsubscribe;
  }, [host]);

  const position = Math.min(100, transport.positionFrames / transport.sampleRate * 25);

  return (
    <main className="shell">
      <header className="topbar">
        <h1 className="brand" aria-label="LarTycc">LarTycc <span>Easy</span></h1>
        <div className="transport" aria-label="Transport controls">
          <button className={transport.playing ? "active" : ""} onClick={() => void host.send({ type: "transport.play", deviceId })} aria-label="Play">▶</button>
          <button onClick={() => void host.send({ type: "transport.stop" })} aria-label="Stop">■</button>
          <strong>{tempo} <small>BPM</small></strong>
        </div>
        <select aria-label="Audio output" value={deviceId} onChange={(event) => setDeviceId(event.target.value)}>
          {devices.length === 0 && <option value="">No audio device</option>}
          {devices.map((device) => <option key={device.id} value={device.id}>{device.name}</option>)}
        </select>
      </header>

      <section className="workspace">
        <aside><p className="label">Sounds</p>{['Kicks', 'Snares', 'Hi-hats', '808', 'Samples'].map((sound) => <button key={sound}>{sound}</button>)}</aside>
        <div className="timeline">
          <div className="ruler"><span>1</span><span>2</span><span>3</span><span>4</span></div>
          <div className="track">
            <div className="track-name"><strong>Audio 1</strong><small>sample.wav</small></div>
            <button className="clip" aria-label="Audio clip" onClick={() => void host.send({ type: "transport.seek", frame: 96_000 })}>
              <span className="waveform" aria-hidden="true">{peaks.map((peak, index) => <i key={index} style={{ height: `${peak}%` }} />)}</span>
              <span className="clip-title">sample.wav</span>
            </button>
          </div>
          <div className="playhead" style={{ left: `calc(180px + ${position}%)` }} />
        </div>
      </section>

      <footer>
        <span className="spark">✦</span><div><strong>AI Producer</strong><p>Co chcesz teraz zrobić?</p></div>
        <span className="status">{transport.playing ? 'Playing' : 'Ready'} · {transport.sampleRate / 1000} kHz · {transport.callbackCount} callbacks</span>
      </footer>
    </main>
  );
}
