import {
  Badge,
  Button,
  ProgressBar,
  Text,
  Tooltip,
} from "@fluentui/react-components";
import {
  Add24Regular,
  ArrowSync24Regular,
  Person24Regular,
  Play24Filled,
} from "@fluentui/react-icons";
import { PageHeader } from "../components/PageHeader";
import type { ApplicationInfo } from "../ipc";

type HomePageProps = {
  application: ApplicationInfo;
  greeting: string;
  summary: string;
};

export function HomePage({ application, greeting, summary }: HomePageProps) {
  return (
    <>
      <PageHeader
        title={greeting}
        description={summary}
        action={
          <Button appearance="subtle" icon={<Person24Regular />}>
            添加账户
          </Button>
        }
      />

      <section className="launch-surface" aria-labelledby="current-instance">
        <div className="instance-art" aria-hidden="true">
          <span>1.21</span>
        </div>
        <div className="instance-copy">
          <div className="instance-heading">
            <Text id="current-instance" as="h2" size={500} weight="semibold">
              尚未导入实例
            </Text>
            <Badge appearance="tint" color="informative">
              早期开发
            </Badge>
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
            <Text as="h2" size={400} weight="semibold">
              环境检查
            </Text>
            <Badge appearance="outline">待扫描</Badge>
          </div>
          <dl className="status-list">
            <div>
              <dt>Java 运行时</dt>
              <dd>尚未检测</dd>
            </div>
            <div>
              <dt>游戏实例</dt>
              <dd>0</dd>
            </div>
            <div>
              <dt>核心阶段</dt>
              <dd>{application.coreStatus}</dd>
            </div>
          </dl>
        </article>

        <article className="status-panel">
          <div className="panel-heading">
            <Text as="h2" size={400} weight="semibold">
              下载队列
            </Text>
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
    </>
  );
}
