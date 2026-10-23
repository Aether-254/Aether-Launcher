import { Badge, Text } from "@fluentui/react-components";

type PendingPanelProps = {
  title: string;
  body: string;
};

export function PendingPanel({ title, body }: PendingPanelProps) {
  return (
    <section className="status-panel placeholder-panel">
      <Badge appearance="tint" color="informative">
        早期开发
      </Badge>
      <Text as="h2" size={400} weight="semibold">
        {title}
      </Text>
      <Text className="muted">{body}</Text>
    </section>
  );
}
