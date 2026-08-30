import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { App } from "./App";

afterEach(cleanup);

describe("App", () => {
  it("renders the product name", () => {
    render(<App />);
    expect(screen.getByRole("heading", { name: "LarTycc" })).toBeTruthy();
  });

  it("controls transport state", () => {
    render(<App />);
    fireEvent.click(screen.getByRole("button", { name: "Play" }));
    expect(screen.getByText(/Playing/)).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: "Stop" }));
    expect(screen.getByText(/Ready/)).toBeTruthy();
  });
});
