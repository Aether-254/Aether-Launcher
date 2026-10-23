import { afterEach, describe, expect, it, vi } from "vitest";
import {
  loadThemePreference,
  resolveTheme,
  saveThemePreference,
  themeStorageKey,
} from "./theme";

afterEach(() => {
  vi.restoreAllMocks();
  window.localStorage.clear();
});

describe("resolveTheme", () => {
  it("follows the system only when asked to", () => {
    expect(resolveTheme("system", true)).toBe("dark");
    expect(resolveTheme("system", false)).toBe("light");
    expect(resolveTheme("dark", false)).toBe("dark");
    expect(resolveTheme("light", true)).toBe("light");
  });
});

describe("theme preference storage", () => {
  it("defaults to dark when nothing is stored", () => {
    expect(loadThemePreference()).toBe("dark");
  });

  it("round-trips a saved preference", () => {
    saveThemePreference("light");
    expect(window.localStorage.getItem(themeStorageKey)).toBe("light");
    expect(loadThemePreference()).toBe("light");
  });

  it("ignores values it does not recognise", () => {
    window.localStorage.setItem(themeStorageKey, "purple");
    expect(loadThemePreference()).toBe("dark");
  });

  it("survives unavailable storage", () => {
    vi.spyOn(Storage.prototype, "getItem").mockImplementation(() => {
      throw new Error("denied");
    });
    vi.spyOn(Storage.prototype, "setItem").mockImplementation(() => {
      throw new Error("denied");
    });

    expect(loadThemePreference()).toBe("dark");
    expect(() => saveThemePreference("light")).not.toThrow();
  });
});
