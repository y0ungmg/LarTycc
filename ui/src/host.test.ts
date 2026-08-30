import { describe, expect, it } from "vitest";
import { PreviewHostBridge } from "./host";

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
