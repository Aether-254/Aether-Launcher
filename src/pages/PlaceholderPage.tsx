import { Badge, Text } from "@fluentui/react-components";
import { PageHeader } from "../components/PageHeader";
import type { PageDefinition } from "../navigation";

export function PlaceholderPage({ page }: { page: PageDefinition }) {
  return (
    <>
      <PageHeader title={page.label} description={page.summary} />
      <section className="status-panel placeholder-panel">
        <Badge appearance="tint" color="informative">
          早期开发
        </Badge>
        <Text as="h2" size={400} weight="semibold">
          {page.pendingTitle}
        </Text>
        <Text className="muted">{page.pendingBody}</Text>
      </section>
    </>
  );
}
