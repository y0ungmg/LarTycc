import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { App } from "./App";
import { PreviewHostBridge } from "./host";

afterEach(cleanup);

describe("App", () => {
  it("renders the product name", () => {
    render(<App host={new PreviewHostBridge()} />);
    expect(screen.getByRole("heading", { name: "LarTycc" })).toBeTruthy();
  });

  it("controls transport state", () => {
    render(<App host={new PreviewHostBridge()} />);
    fireEvent.click(screen.getByRole("button", { name: "Play" }));
    expect(screen.getByText(/Playing/)).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: "Stop" }));
    expect(screen.getByText(/Ready/)).toBeTruthy();
  });
});
