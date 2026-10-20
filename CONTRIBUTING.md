# Contributing to Aether Launcher

感谢你愿意改进 Aether Launcher。

## 开始之前

- Bug、功能建议和下载源问题请使用对应的 Issue Form。
- 较大的交互或兼容性改动应先在 Discussions 说明目标、边界和替代方案。
- 安全漏洞不要提交公开 Issue，按照 [SECURITY.md](SECURITY.md) 私密报告。
- 本项目不接受用于绕过正版认证、分发游戏本体、窃取凭据或关闭文件校验的改动。

## 开发流程

1. Fork 仓库并从 `main` 创建短生命周期分支。
2. 保持改动聚焦，避免把格式化或无关重构混入功能提交。
3. 为行为变化添加 Rust 单元测试或前端测试。
4. 用户行为、开发接口或兼容性发生变化时，直接更新相关文档。
5. 在本地完成全部检查。

```powershell
npm ci
npm run check
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## 提交与 PR

推荐使用 Conventional Commits：

```text
feat(download): add immutable artifact plan
fix(runtime): reject incompatible Java major version
docs(auth): document credential storage rules
```

PR 描述必须说明改动原因、验证方法、实际证据层、风险与回滚方式。UI 改动还应附截图或录屏。

## 代码边界

- React 不处理凭据、路径校验、Java 命令拼装或下载完整性。
- Tauri IPC 必须是领域动作，不得提供任意 shell 或无限制文件操作。
- provider 回退不能改变期望散列或跳过校验。
- 错误必须保留可诊断信息，但日志不得包含 token、密码或完整账户标识。

提交代码即表示你同意按照仓库的 Apache-2.0 许可证发布贡献。
