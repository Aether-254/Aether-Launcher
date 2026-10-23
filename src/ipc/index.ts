import { invoke } from "@tauri-apps/api/core";

/**
 * 前端能调用的全部后端命令都登记在这里。
 * 新增命令时先在 src-tauri 注册，再把名字和返回类型补到 CommandResults。
 */
export type ApplicationInfo = {
  name: string;
  version: string;
  coreStatus: string;
};

type CommandResults = {
  application_info: ApplicationInfo;
};

export const defaultApplicationInfo: ApplicationInfo = {
  name: "Aether Launcher",
  version: "0.1.0",
  coreStatus: "foundation",
};

/** 在浏览器里单独跑前端（npm run dev）时没有 Tauri 运行时。 */
export function isTauri(): boolean {
  return "__TAURI_INTERNALS__" in window;
}

function call<Name extends keyof CommandResults>(
  name: Name,
): Promise<CommandResults[Name]> {
  return invoke<CommandResults[Name]>(name);
}

export function getApplicationInfo(): Promise<ApplicationInfo> {
  return call("application_info");
}
