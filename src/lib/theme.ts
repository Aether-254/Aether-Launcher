export type ThemePreference = "system" | "dark" | "light";
export type ResolvedTheme = "dark" | "light";

export const themeStorageKey = "aether.theme";

const darkQuery = "(prefers-color-scheme: dark)";

export function isThemePreference(value: unknown): value is ThemePreference {
  return value === "system" || value === "dark" || value === "light";
}

/** 默认深色，保持启动器原有的外观。 */
export function loadThemePreference(): ThemePreference {
  try {
    const stored = window.localStorage.getItem(themeStorageKey);
    return isThemePreference(stored) ? stored : "dark";
  } catch {
    return "dark";
  }
}

export function saveThemePreference(preference: ThemePreference): void {
  try {
    window.localStorage.setItem(themeStorageKey, preference);
  } catch {
    // 存储不可用时，选择只在本次运行内生效
  }
}

/** 没有 matchMedia 的环境（比如 jsdom）按深色处理。 */
export function systemPrefersDark(): boolean {
  if (typeof window.matchMedia !== "function") {
    return true;
  }
  return window.matchMedia(darkQuery).matches;
}

export function subscribeSystemTheme(
  onChange: (prefersDark: boolean) => void,
): () => void {
  if (typeof window.matchMedia !== "function") {
    return () => undefined;
  }
  const query = window.matchMedia(darkQuery);
  const listener = (event: MediaQueryListEvent) => onChange(event.matches);
  query.addEventListener("change", listener);
  return () => query.removeEventListener("change", listener);
}

export function resolveTheme(
  preference: ThemePreference,
  systemDark: boolean,
): ResolvedTheme {
  if (preference === "system") {
    return systemDark ? "dark" : "light";
  }
  return preference;
}
