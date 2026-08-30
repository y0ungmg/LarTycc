export type AudioDevice = { id: string; name: string; isDefault: boolean };
export type TransportSnapshot = {
  playing: boolean;
  positionFrames: number;
  sampleRate: number;
  callbackCount: number;
};
export type HostCommand =
  | { type: "transport.play"; deviceId: string }
  | { type: "transport.stop" }
  | { type: "transport.seek"; frame: number };

export interface HostBridge {
  listDevices(): Promise<AudioDevice[]>;
  send(command: HostCommand): Promise<void>;
  subscribeTransport(listener: (snapshot: TransportSnapshot) => void): () => void;
}

declare global {
  interface Window {
    lartyccHost?: {
      invoke<T>(command: string, payload?: unknown): Promise<T>;
      onTransport(listener: (snapshot: TransportSnapshot) => void): () => void;
    };
  }
}

class NativeHostBridge implements HostBridge {
  constructor(private readonly native: NonNullable<Window["lartyccHost"]>) {}
  listDevices() { return this.native.invoke<AudioDevice[]>("audio.listDevices"); }
  send(command: HostCommand) { return this.native.invoke<void>(command.type, command); }
  subscribeTransport(listener: (snapshot: TransportSnapshot) => void) {
    return this.native.onTransport(listener);
  }
}

export class PreviewHostBridge implements HostBridge {
  private snapshot: TransportSnapshot = { playing: false, positionFrames: 0, sampleRate: 48_000, callbackCount: 0 };
  private readonly listeners = new Set<(snapshot: TransportSnapshot) => void>();
  private timer: ReturnType<typeof setInterval> | undefined;

  async listDevices() {
    return [{ id: "preview", name: "Browser preview (no hardware audio)", isDefault: true }];
  }

  async send(command: HostCommand) {
    if (command.type === "transport.play") {
      this.snapshot = { ...this.snapshot, playing: true };
      this.timer ??= setInterval(() => {
        this.snapshot = { ...this.snapshot, positionFrames: this.snapshot.positionFrames + 128, callbackCount: this.snapshot.callbackCount + 1 };
        this.emit();
      }, 16);
    } else if (command.type === "transport.stop") {
      if (this.timer) clearInterval(this.timer);
      this.timer = undefined;
      this.snapshot = { ...this.snapshot, playing: false, positionFrames: 0 };
    } else {
      this.snapshot = { ...this.snapshot, positionFrames: command.frame };
    }
    this.emit();
  }

  subscribeTransport(listener: (snapshot: TransportSnapshot) => void) {
    this.listeners.add(listener);
    listener(this.snapshot);
    return () => {
      this.listeners.delete(listener);
      if (this.listeners.size === 0 && this.timer) {
        clearInterval(this.timer);
        this.timer = undefined;
      }
    };
  }

  private emit() { this.listeners.forEach((listener) => listener(this.snapshot)); }
}

export function createHostBridge(): HostBridge {
  return window.lartyccHost ? new NativeHostBridge(window.lartyccHost) : new PreviewHostBridge();
}
