import {
  Box24Regular,
  Home24Filled,
  Library24Regular,
  Person24Regular,
  Settings24Regular,
} from "@fluentui/react-icons";
import type { ReactElement } from "react";

export type PageId = "home" | "instances" | "library" | "accounts" | "settings";

export type PageDefinition = {
  id: PageId;
  label: string;
  icon: ReactElement;
  /** 页头下方的一句话说明 */
  summary: string;
  /** 功能还没做时，页面里显示的标题和说明 */
  pendingTitle: string;
  pendingBody: string;
};

export const mainPages: readonly PageDefinition[] = [
  {
    id: "home",
    label: "主页",
    icon: <Home24Filled />,
    summary: "实例仍属于你，下载链路也应该看得见",
    pendingTitle: "",
    pendingBody: "",
  },
  {
    id: "instances",
    label: "实例",
    icon: <Box24Regular />,
    summary: "每个实例的版本、模组、Java 与启动参数相互隔离",
    pendingTitle: "实例列表尚未实现",
    pendingBody: "导入和扫描本地实例的功能还在开发，完成后实例会列在这里。",
  },
  {
    id: "library",
    label: "资源库",
    icon: <Library24Regular />,
    summary: "模组、资源包与它们的下载来源",
    pendingTitle: "资源库尚未实现",
    pendingBody: "下载计划和来源校验完成后，这里会列出可用资源及其来源。",
  },
  {
    id: "accounts",
    label: "账户",
    icon: <Person24Regular />,
    summary: "微软正版账户与离线账户",
    pendingTitle: "账户管理尚未实现",
    pendingBody: "账户模型已经定义，微软登录流程还在开发中。",
  },
];

export const settingsPage: PageDefinition = {
  id: "settings",
  label: "设置",
  icon: <Settings24Regular />,
  summary: "Java 运行时、内存与下载来源",
  pendingTitle: "设置尚未实现",
  pendingBody: "这里将提供 Java 运行时选择、内存上限和下载来源的配置。",
};

export const allPages: readonly PageDefinition[] = [...mainPages, settingsPage];
