# 启动与进程

已定的设计结论，来自对 #12 的讨论。实现按这里对齐，要改先改这份文档。

## 编排层

新增一层编排（暂名 `aether-app`），持有「实例 → 进程」的状态机，串起：

1. 加载版本清单并合并 `inheritsFrom`
2. 选择 Java
3. 准备 native
4. 取账户
5. 生成启动计划
6. 启动并监督进程
7. 进程退出后触发崩溃采集（见 [crash-analysis.md](crash-analysis.md)）

`src-tauri` 只做 IPC 的参数和结果转换，不放业务逻辑。业务逻辑放在编排层，脱离 GUI 也能测。

## 启动命令

- IPC 只提供 `launch_instance(instance_id)`。Java 路径、参数、账号全由后端按 id 解析，前端不传 argv，也没有「执行任意程序」的入口。
- 启动计划拆成两个类型：
  - 无密钥的计划：可以 `Serialize`、写日志，每个参数记录来源（清单、加载器、用户自定义）。
  - 带密钥的最终参数：不 `Serialize`，`Debug` 打码，只在 spawn 之前生成。
- access token 出现在 argv 里避免不了（任务管理器和 `ps` 都看得到）。目标是它不进日志、IPC 和持久化。

## 进程

- 游戏独立于启动器存活，关闭启动器不杀游戏。
- 用户点「强制停止」时杀整棵进程树：Windows 用 Job Object，Unix 用进程组。不要给游戏进程设置 `KILL_ON_JOB_CLOSE` 这类随启动器退出的标志。
- 界面用的日志队列留在内存里，有上限。崩溃分析的证据从磁盘取，不依赖这个队列。

## Java

- 只认精确主版本，不放宽到更高版本。没有匹配的 Java 时给出明确的错误。
- 缺失时可以下载 Mojang 官方运行时，下载前必须征得用户同意，并展示来源和大小。
- 探测 Java 要有超时。Windows 下探测不弹出控制台窗口。

## 下载与信任

- 信任根是官方源。版本清单优先从官方取；镜像只作为 `ArtifactSource` 候选，用于下载文件体，不改变哈希。哈希必须来自官方清单。
- 下载计划里的 `destination` 必须是安全的相对路径（`aether-download` 已实现）。
- 解压和安装 native 的代码从 `aether-download` 拆出，成为独立的安装层。过渡期可以先分 mod，目标是拆成独立 crate。

## 交付

#12 等拆成正常的 PR 再审，按依赖顺序：runtime → manifest loader → download / native → launch plan → process。
