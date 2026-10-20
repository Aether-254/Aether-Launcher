<div align="center">

# Aether Launcher

**透明、可靠、可迁移的 Minecraft Java 版实例管理器**

[![CI](https://github.com/AetherLauncher-org/Aether-Launcher/actions/workflows/ci.yml/badge.svg)](https://github.com/AetherLauncher-org/Aether-Launcher/actions/workflows/ci.yml)
[![CodeQL](https://github.com/AetherLauncher-org/Aether-Launcher/actions/workflows/codeql.yml/badge.svg)](https://github.com/AetherLauncher-org/Aether-Launcher/actions/workflows/codeql.yml)
[![License](https://img.shields.io/github/license/Aether-254/Aether-Launcher)](LICENSE)
[![GitHub Release](https://img.shields.io/github/v/release/Aether-254/Aether-Launcher?include_prereleases)](https://github.com/AetherLauncher-org/Aether-Launcher/releases)

<img src="https://ld.xh.do/ld-badge.svg" alt="认可linux.do">

</div>

> [!IMPORTANT]
> Aether Launcher 目前处于早期开发阶段。仓库已经包含可构建的桌面壳、领域模型和自动化，但尚未完成游戏下载、Microsoft 登录或游戏启动流程。请不要把当前版本当作可替代日常启动器的发行版。

## 为什么做 Aether

Minecraft 实例不应该被某一个启动器、下载源或服务入口锁住。Aether 将本地导入、来源透明、文件校验和故障可解释性作为基础能力：

- **本地优先**：已有实例可以直接导入，只补齐缺失文件
- **来源透明**：下载计划记录 provider、目标地址、大小与预期散列
- **校验优先**：任何来源都不能绕过完整性校验
- **实例隔离**：版本、模组、Java 与启动参数互不污染
- **跨平台**：以 Windows 为首发平台，共享 Rust 核心并支持 Linux 与 macOS
- **最小权限**：前端只能调用窄化的领域命令，不能传入任意系统命令

我们支持 Linux.DO 社区与开放、友善、重视技术质量的开发者社区。

## 技术栈

```text
Rust workspace
├── Tauri 2                 桌面壳与受约束 IPC
├── aether-core             实例与版本领域模型
├── aether-manifest         Minecraft 版本元数据
├── aether-download         下载计划、来源与校验
├── aether-auth             账户模型与认证边界
├── aether-runtime          Java 运行时发现
├── aether-launch           启动计划与进程生命周期
├── aether-modloader        Fabric / Forge / NeoForge / Quilt
└── aether-storage          本地持久化边界

React + TypeScript
├── Fluent UI React v9      Fluent 2 组件与设计 token
├── Vite                    前端构建
└── Vitest                  前端测试
```

## 本地开发

### 环境要求

- Node.js 24 或当前维护中的 LTS
- npm 11+
- Rust stable
- Tauri 2 对应的平台构建依赖

Windows 需要 Visual Studio Build Tools 的“使用 C++ 的桌面开发”工作负载。Linux 需要 WebKitGTK 4.1 等 Tauri 系统依赖。

### 启动与检查

```powershell
npm ci
npm run tauri dev
```

提交前运行：

```powershell
npm run check
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

仅检查不依赖桌面 WebView 的 Rust 核心：

```powershell
cargo test --workspace --exclude aether-desktop
```

## 当前里程碑

- [x] Rust workspace 与 Tauri 2 桌面壳
- [x] React、TypeScript 与 Fluent UI v9 基础界面
- [x] 实例、清单、下载、账户、Java 与启动计划的基础类型
- [x] CI、安全扫描、依赖更新与跨平台 Release 工作流
- [ ] 扫描并导入本地 Minecraft 实例
- [ ] 解析版本继承与生成 classpath
- [ ] Java 发现、兼容性选择与启动日志
- [ ] Microsoft / Xbox / Minecraft 正版认证
- [ ] 可审计的多来源下载计划与缓存
- [ ] Fabric、Forge、NeoForge 与 Quilt 安装

具体需求请使用 [Issues](https://github.com/AetherLauncher-org/Aether-Launcher/issues)，设计讨论和问答请前往 [Discussions](https://github.com/AetherLauncher-org/Aether-Launcher/discussions)。

## 贡献

开始编码前请阅读 [CONTRIBUTING.md](CONTRIBUTING.md)。安全问题不要提交公开 Issue，请按照 [SECURITY.md](SECURITY.md) 使用 GitHub 私密漏洞报告。

## 不接受赞助

Aether Launcher 不接受项目赞助，也不会设置 GitHub Sponsors、爱发电或其他项目收款入口。如果你原本打算赞助本项目，可以考虑把这份心意交给真正需要帮助的人：

- [腾讯公益](https://gongyi.qq.com/succor/project_list.htm)
- [支付宝公益](https://love.alipay.com/donate/index.htm)
- [壹基金](https://onefoundation.cn/donate/)
- [GlobalGiving](https://www.globalgiving.org/)
- [GoFundMe](https://www.gofundme.com/)
- [DonorsChoose](https://www.donorschoose.org/)

以上链接仅作为公益捐赠入口参考，不代表 Aether Launcher 与这些平台存在合作、隶属、授权或背书关系。捐赠前请自行核验项目主体、资金用途、所在地区的可用性以及平台条款。

## 法律与品牌说明

Aether Launcher 是独立的社区项目，与 Mojang Studios、Microsoft、Minecraft 或 Linux.DO 不存在官方隶属、授权或背书关系。“Minecraft”是其权利人的商标。本项目不随仓库分发 Minecraft 游戏文件、账户凭据或第三方受版权保护的内容。

## 许可证

代码以 [Apache License 2.0](LICENSE) 发布。
