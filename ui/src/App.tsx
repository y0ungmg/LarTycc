import { useState } from "react";
import "./styles.css";

const peaks = [18, 42, 64, 35, 78, 52, 88, 45, 68, 30, 72, 54, 82, 38, 60, 24];

export function App() {
  const [playing, setPlaying] = useState(false);
  const [position, setPosition] = useState(0);

  function stop() {
    setPlaying(false);
    setPosition(0);
  }

  return (
    <main className="shell">
      <header className="topbar">
        <h1 className="brand" aria-label="LarTycc">LarTycc <span>Easy</span></h1>
        <div className="transport" aria-label="Transport controls">
          <button className={playing ? "active" : ""} onClick={() => setPlaying(true)} aria-label="Play">▶</button>
          <button onClick={stop} aria-label="Stop">■</button>
          <strong>145 <small>BPM</small></strong>
        </div>
        <button className="mode">Switch to Pro</button>
      </header>

      <section className="workspace">
        <aside>
          <p className="label">Sounds</p>
          {['Kicks', 'Snares', 'Hi-hats', '808', 'Samples'].map((sound) => <button key={sound}>{sound}</button>)}
        </aside>
        <div className="timeline">
          <div className="ruler"><span>1</span><span>2</span><span>3</span><span>4</span></div>
          <div className="track">
            <div className="track-name"><strong>Audio 1</strong><small>sample.wav</small></div>
            <button className="clip" aria-label="Audio clip" onClick={() => setPosition(48)}>
              <span className="waveform" aria-hidden="true">
                {peaks.map((peak, index) => <i key={index} style={{ height: `${peak}%` }} />)}
              </span>
              <span className="clip-title">sample.wav</span>
            </button>
          </div>
          <div className="playhead" style={{ left: `${Math.max(180, position * 6)}px` }} />
        </div>
      </section>

      <footer>
        <span className="spark">✦</span>
        <div><strong>AI Producer</strong><p>Co chcesz teraz zrobić?</p></div>
        <span className="status">{playing ? 'Playing' : 'Ready'} · 48 kHz</span>
      </footer>
    </main>
  );
}
