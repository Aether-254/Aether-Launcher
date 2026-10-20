import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import App from "./App";

describe("Aether Launcher shell", () => {
  it("shows the import-first empty state", () => {
    render(<App />);

    expect(
      screen.getByRole("heading", { name: "尚未导入实例" }),
    ).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "导入实例" })).toBeEnabled();
    expect(screen.getByRole("button", { name: "启动" })).toBeDisabled();
  });
});
