# Agent 工作约定

给在这个仓库里工作的 AI agent。动手前先读：

- `README.md`、`CONTRIBUTING.md`
- `docs/architecture.md`：crate 分层和依赖方向
- `docs/design/*.md`：已定的设计，与之冲突时先停下来问

## 安全不变量

以下几条不能违反，也不要为了让测试通过而放宽。

1. 前端只能通过 `src/ipc/` 调用已登记的窄化命令。不增加执行任意命令、读写任意路径的入口。
2. 凭据（access token 等）不进 `Serialize` / `Debug` 输出、日志、IPC 和持久化。专门的加密存储除外。
3. 下载目标必须是安全的相对路径，所有下载都校验哈希，哈希必须来自官方清单。镜像不是信任根。
4. 清单、日志、崩溃报告都是不可信输入。解析要有大小和深度上限，不执行其中的内容。

## 边界

- 一个 agent 负责一个 crate，除非任务明确跨 crate。
- 不新增反向依赖。改 crate 间依赖之前先看 `docs/architecture.md` 的依赖图。
- 设计问题（边界怎么划、策略怎么选）写进 PR 描述或文档的「待定」，不要自己拍板。

## 许可

仓库是 Apache-2.0。不要把 GPL / LGPL 项目（HMCL、PCL 等）的代码或测试数据复制进来，只参考行为，并在文档里写明参考了什么。

## 工作方式

- 从 `main` 开短分支，命名 `feat/<crate>-<主题>`。PR 小而聚焦，一个关注点，不混入格式化和无关重构。
- 容易冲突的文件：根 `Cargo.toml`、`Cargo.lock`、`docs/architecture.md`、`CHANGELOG.md`。改之前先 fetch，改动尽量小，冲突时 rebase，不要整文件覆盖。
- 推送前跑完 `CONTRIBUTING.md` 里的全部检查。PR 描述写实际跑了什么、没跑什么，没验证的项不要勾。
- 提交用 Conventional Commits，提交信息用英文。
- 行为或接口变了，同步更新 `docs/`。文档写得直接，不写成免责声明。
