import "@testing-library/jest-dom/vitest";
import { cleanup } from "@testing-library/react";
import { afterEach } from "vitest";

// vitest 没开 globals，RTL 不会自动清理，这里手动处理
afterEach(() => {
  cleanup();
  window.localStorage.clear();
});
