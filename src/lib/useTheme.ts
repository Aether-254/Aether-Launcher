import { useCallback, useEffect, useState } from "react";
import {
  loadThemePreference,
  resolveTheme,
  saveThemePreference,
  subscribeSystemTheme,
  systemPrefersDark,
} from "./theme";
import type { ResolvedTheme, ThemePreference } from "./theme";

export function useTheme(): {
  preference: ThemePreference;
  resolved: ResolvedTheme;
  setPreference: (preference: ThemePreference) => void;
} {
  const [preference, setPreferenceState] =
    useState<ThemePreference>(loadThemePreference);
  const [systemDark, setSystemDark] = useState(systemPrefersDark);

  useEffect(() => subscribeSystemTheme(setSystemDark), []);

  const setPreference = useCallback((next: ThemePreference) => {
    setPreferenceState(next);
    saveThemePreference(next);
  }, []);

  return {
    preference,
    resolved: resolveTheme(preference, systemDark),
    setPreference,
  };
}
