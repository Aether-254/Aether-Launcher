# Security Policy

## Supported versions

Aether Launcher 尚未发布稳定版本。目前只对 `main` 分支最新提交进行安全修复。

## Reporting a vulnerability

请使用 GitHub 的 [私密漏洞报告](https://github.com/Aether-254/Aether-Launcher/security/advisories/new)，不要创建公开 Issue、Discussion 或 PR。

报告请尽量包含受影响提交或版本、前置条件、可复现步骤或最小 PoC、预期影响与已知缓解方式。

请不要在验证过程中访问他人账户、数据或服务器，也不要公开未修复漏洞。维护者确认问题后会在私密 Security Advisory 中协调修复和披露。

## High-value security properties

- Microsoft/Xbox/Minecraft token 不进入 WebView 状态、遥测或普通日志
- Tauri IPC 不提供任意命令执行
- 下载来源切换不能跳过散列校验
- 解压不能写出暂存目录
- 实例路径不能越过用户明确选择的根目录
- 更新包必须来自项目发布链路并通过完整性验证
