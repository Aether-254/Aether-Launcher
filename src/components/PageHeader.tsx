import { Text } from "@fluentui/react-components";
import type { ReactNode } from "react";

type PageHeaderProps = {
  title: string;
  description: string;
  action?: ReactNode;
};

export function PageHeader({ title, description, action }: PageHeaderProps) {
  return (
    <header className="topbar">
      <div>
        <Text as="h1" size={600} weight="semibold">
          {title}
        </Text>
        <Text className="muted">{description}</Text>
      </div>
      {action}
    </header>
  );
}
