import {
  Badge,
  Button,
  FluentProvider,
  ProgressBar,
  Text,
  Tooltip,
  webDarkTheme,
} from "@fluentui/react-components";
import {
  Add24Regular,
  ArrowSync24Regular,
  Box24Regular,
  Home24Filled,
  Library24Regular,
  Person24Regular,
  Play24Filled,
  Settings24Regular,
} from "@fluentui/react-icons";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import "./App.css";

type ApplicationInfo = {
  name: string;
  version: string;
  coreStatus: string;
};

const navigation = [
  { label: "主页", icon: <Home24Filled />, active: true },
  { label: "实例", icon: <Box24Regular /> },
  { label: "资源库", icon: <Library24Regular /> },
  { label: "账户", icon: <Person24Regular /> },
];

function App() {
  const [application, setApplication] = useState<ApplicationInfo>({
    name: "Aether Launcher",
    version: "0.1.0",
    coreStatus: "foundation",
  });

  useEffect(() => {
    if ("__TAURI_INTERNALS__" in window) {
      void invoke<ApplicationInfo>("application_info").then(setApplication);
    }
  }, []);

  return (
    <FluentProvider theme={webDarkTheme}>
      <div className="app-shell">
        <aside className="sidebar">
          <div className="brand" aria-label={application.name}>
            <span className="brand-mark">A</span>
            <div>
              <Text weight="semibold" size={400}>Aether</Text>
              <Text className="brand-version" size={100}>
                {application.version}
              </Text>
            </div>
          </div>

          <nav className="navigation" aria-label="主要导航">
            {navigation.map((item) => (
              <button
                className={`nav-item${item.active ? " active" : ""}`}
                key={item.label}
                type="button"
              >
                {item.icon}
                <span>{item.label}</span>
              </button>
            ))}
          </nav>

          <Tooltip content="设置" relationship="label">
            <button className="nav-item settings" type="button">
              <Settings24Regular />
              <span>设置</span>
            </button>
          </Tooltip>
        </aside>

        <main className="workspace">
          <header className="topbar">
            <div>
              <Text as="h1" size={600} weight="semibold">晚上好</Text>
              <Text className="muted">
                实例仍属于你，下载链路也应该看得见
              </Text>
            </div>
            <Button appearance="subtle" icon={<Person24Regular />}>
              添加账户
            </Button>
          </header>

          <section className="launch-surface" aria-labelledby="current-instance">
            <div className="instance-art" aria-hidden="true">
              <span>1.21</span>
            </div>
            <div className="instance-copy">
              <div className="instance-heading">
                <Text id="current-instance" as="h2" size={500} weight="semibold">
                  尚未导入实例
                </Text>
                <Badge appearance="tint" color="informative">早期开发</Badge>
              </div>
              <Text className="muted">
                从现有 .minecraft 或其他启动器导入，Aether 只补齐缺失内容
              </Text>
              <div className="instance-actions">
                <Button appearance="primary" icon={<Add24Regular />}>
                  导入实例
                </Button>
                <Button icon={<ArrowSync24Regular />}>扫描本机</Button>
              </div>
            </div>
            <Tooltip content="导入实例后即可启动" relationship="description">
              <Button
                className="launch-button"
                appearance="primary"
                icon={<Play24Filled />}
                size="large"
                disabled
              >
                启动
              </Button>
            </Tooltip>
          </section>

          <section className="status-grid" aria-label="启动器状态">
            <article className="status-panel">
              <div className="panel-heading">
                <Text as="h2" size={400} weight="semibold">环境检查</Text>
                <Badge appearance="outline">待扫描</Badge>
              </div>
              <dl className="status-list">
                <div><dt>Java 运行时</dt><dd>尚未检测</dd></div>
                <div><dt>游戏实例</dt><dd>0</dd></div>
                <div><dt>核心阶段</dt><dd>{application.coreStatus}</dd></div>
              </dl>
            </article>

            <article className="status-panel">
              <div className="panel-heading">
                <Text as="h2" size={400} weight="semibold">下载队列</Text>
                <Text className="muted">0 项</Text>
              </div>
              <ProgressBar value={0} />
              <div className="empty-state">
                <Text weight="semibold">没有进行中的任务</Text>
                <Text className="muted">
                  下载计划将显示来源、校验值与失败原因
                </Text>
              </div>
            </article>
          </section>
        </main>
      </div>
    </FluentProvider>
  );
}

export default App;
