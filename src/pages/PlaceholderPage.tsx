import { PageHeader } from "../components/PageHeader";
import { PendingPanel } from "../components/PendingPanel";
import type { PageDefinition } from "../navigation";

export function PlaceholderPage({ page }: { page: PageDefinition }) {
  return (
    <>
      <PageHeader title={page.label} description={page.summary} />
      <PendingPanel title={page.pendingTitle} body={page.pendingBody} />
    </>
  );
}
