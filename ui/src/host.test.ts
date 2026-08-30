import { describe, expect, it } from "vitest";
import { NativeHostBridge, PreviewHostBridge, type HostRequest, type HostResponse } from "./host";

describe("PreviewHostBridge", () => {
  it("publishes typed transport commands", async () => {
    const host = new PreviewHostBridge();
    let playing = false;
    const unsubscribe = host.subscribeTransport((snapshot) => { playing = snapshot.playing; });
    await host.send({ type: "transport.play", deviceId: "preview" });
    expect(playing).toBe(true);
    await host.send({ type: "transport.stop" });
    expect(playing).toBe(false);
    unsubscribe();
  });
});

describe("NativeHostBridge", () => {
  it("sends a versioned request envelope", async () => {
    let captured: HostRequest | undefined;
    const native = {
      async invoke<T>(request: HostRequest): Promise<HostResponse<T>> {
        captured = request;
        return { version: 1, id: request.id, ok: true, result: [] as T };
      },
      onTransport() { return () => undefined; },
    };
    const host = new NativeHostBridge(native);
    await host.listDevices();
    expect(captured).toMatchObject({ version: 1, command: "audio.listDevices" });
    expect(captured?.id).toMatch(/^ui-/);
  });

  it("routes project mutations with optimistic revision", async () => {
    let captured: HostRequest | undefined;
    const native = {
      async invoke<T>(request: HostRequest): Promise<HostResponse<T>> {
        captured = request;
        return {
          version: 1,
          id: request.id,
          ok: true,
          result: { schemaVersion: 1, projectId: "test", revision: 4, tempo: 148, tracks: [] } as T,
        };
      },
      onTransport() { return () => undefined; },
    };
    await new NativeHostBridge(native).setTempo(148, 3);
    expect(captured).toMatchObject({
      version: 1,
      command: "project.setTempo",
      payload: { bpm: 148 },
      expectedProjectRevision: 3,
    });
  });

  it("surfaces stable native error codes", async () => {
    const native = {
      async invoke<T>(request: HostRequest): Promise<HostResponse<T>> {
        return {
          version: 1,
          id: request.id,
          ok: false,
          error: { code: "revision_conflict", message: "stale edit" },
        };
      },
      onTransport() { return () => undefined; },
    };
    await expect(new NativeHostBridge(native).listDevices()).rejects.toThrow("revision_conflict");
  });
});
