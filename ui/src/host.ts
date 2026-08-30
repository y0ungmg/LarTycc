export type AudioDevice = { id: string; name: string; isDefault: boolean };
export type TransportSnapshot = {
  playing: boolean;
  positionFrames: number;
  sampleRate: number;
  callbackCount: number;
};
export type ProjectSnapshot = {
  schemaVersion: number;
  projectId: string;
  revision: number;
  tempo: number;
  tracks: Array<{ id: string; name: string; kind: "audio" }>;
};
export type HostState = {
  project: ProjectSnapshot;
  transport: TransportSnapshot;
  sampleLoaded: boolean;
  audioAvailable: boolean;
};
export type HostCommand =
  | { type: "transport.play"; deviceId: string }
  | { type: "transport.stop" }
  | { type: "transport.seek"; frame: number };

export type HostRequest = {
  version: 1;
  id: string;
  command: string;
  payload?: unknown;
  expectedProjectRevision?: number;
};

export type HostResponse<T> =
  | { version: 1; id: string; ok: true; result: T }
  | { version: 1; id: string; ok: false; error: { code: string; message: string } };

export interface HostBridge {
  getState(): Promise<HostState>;
  listDevices(): Promise<AudioDevice[]>;
  send(command: HostCommand): Promise<void>;
  setTempo(bpm: number, expectedRevision: number): Promise<ProjectSnapshot>;
  createTrack(id: string, name: string, expectedRevision: number): Promise<ProjectSnapshot>;
  undo(expectedRevision: number): Promise<ProjectSnapshot>;
  redo(expectedRevision: number): Promise<ProjectSnapshot>;
  save(expectedRevision: number): Promise<ProjectSnapshot>;
  subscribeTransport(listener: (snapshot: TransportSnapshot) => void): () => void;
}

declare global {
  interface Window {
    lartyccHost?: {
      invoke<T>(request: HostRequest): Promise<HostResponse<T>>;
      onTransport(listener: (snapshot: TransportSnapshot) => void): () => void;
    };
  }
}

let requestSequence = 0;

export class NativeHostBridge implements HostBridge {
  constructor(private readonly native: NonNullable<Window["lartyccHost"]>) {}
  getState() { return this.invoke<HostState>("host.getState"); }
  listDevices() { return this.invoke<AudioDevice[]>("audio.listDevices"); }
  send(command: HostCommand) { return this.invoke<void>(command.type, command); }
  setTempo(bpm: number, revision: number) {
    return this.invoke<ProjectSnapshot>("project.setTempo", { bpm }, revision);
  }
  createTrack(id: string, name: string, revision: number) {
    return this.invoke<ProjectSnapshot>("project.createTrack", { id, name }, revision);
  }
  undo(revision: number) { return this.invoke<ProjectSnapshot>("project.undo", undefined, revision); }
  redo(revision: number) { return this.invoke<ProjectSnapshot>("project.redo", undefined, revision); }
  save(revision: number) { return this.invoke<ProjectSnapshot>("project.save", undefined, revision); }
  subscribeTransport(listener: (snapshot: TransportSnapshot) => void) {
    return this.native.onTransport(listener);
  }

  private async invoke<T>(command: string, payload?: unknown, expectedProjectRevision?: number): Promise<T> {
    const response = await this.native.invoke<T>({
      version: 1,
      id: `ui-${++requestSequence}`,
      command,
      payload,
      expectedProjectRevision,
    });
    if (!response.ok) {
      throw new Error(`${response.error.code}: ${response.error.message}`);
    }
    return response.result;
  }
}

export class PreviewHostBridge implements HostBridge {
  private snapshot: TransportSnapshot = { playing: false, positionFrames: 0, sampleRate: 48_000, callbackCount: 0 };
  private project: ProjectSnapshot = {
    schemaVersion: 1,
    projectId: "browser-preview",
    revision: 0,
    tempo: 145,
    tracks: [{ id: "1", name: "Audio 1", kind: "audio" }],
  };
  private readonly listeners = new Set<(snapshot: TransportSnapshot) => void>();
  private timer: ReturnType<typeof setInterval> | undefined;

  async getState() {
    return { project: this.project, transport: this.snapshot, sampleLoaded: true, audioAvailable: false };
  }

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

  async setTempo(bpm: number, expectedRevision: number) {
    this.requireRevision(expectedRevision);
    this.project = { ...this.project, tempo: bpm, revision: this.project.revision + 1 };
    return this.project;
  }

  async createTrack(id: string, name: string, expectedRevision: number) {
    this.requireRevision(expectedRevision);
    this.project = {
      ...this.project,
      revision: this.project.revision + 1,
      tracks: [...this.project.tracks, { id, name, kind: "audio" }],
    };
    return this.project;
  }

  async undo(expectedRevision: number): Promise<ProjectSnapshot> {
    this.requireRevision(expectedRevision);
    throw new Error("command_rejected: preview history is empty");
  }

  async redo(expectedRevision: number): Promise<ProjectSnapshot> {
    this.requireRevision(expectedRevision);
    throw new Error("command_rejected: preview history is empty");
  }

  async save(expectedRevision: number) {
    this.requireRevision(expectedRevision);
    return this.project;
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

  private requireRevision(expected: number) {
    if (expected !== this.project.revision) throw new Error("revision_conflict: stale edit");
  }
}

export function createHostBridge(): HostBridge {
  return window.lartyccHost ? new NativeHostBridge(window.lartyccHost) : new PreviewHostBridge();
}
