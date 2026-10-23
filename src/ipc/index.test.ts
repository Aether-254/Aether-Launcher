import { invoke } from "@tauri-apps/api/core";
import { afterEach, describe, expect, it, vi } from "vitest";
import { getApplicationInfo, isTauri } from "./index";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

afterEach(() => {
  Reflect.deleteProperty(window, "__TAURI_INTERNALS__");
  vi.mocked(invoke).mockReset();
});

describe("isTauri", () => {
  it("is false in a plain browser", () => {
    expect(isTauri()).toBe(false);
  });

  it("is true once the Tauri runtime is injected", () => {
    Object.defineProperty(window, "__TAURI_INTERNALS__", {
      value: {},
      configurable: true,
    });
    expect(isTauri()).toBe(true);
  });
});

describe("getApplicationInfo", () => {
  it("calls the application_info command and returns its result", async () => {
    const info = {
      name: "Aether Launcher",
      version: "0.1.0",
      coreStatus: "foundation",
    };
    vi.mocked(invoke).mockResolvedValue(info);

    await expect(getApplicationInfo()).resolves.toEqual(info);
    expect(invoke).toHaveBeenCalledWith("application_info");
  });
});
