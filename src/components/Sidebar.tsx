import { Text } from "@fluentui/react-components";
import type { ApplicationInfo } from "../ipc";
import { mainPages, settingsPage } from "../navigation";
import type { PageDefinition, PageId } from "../navigation";

type SidebarProps = {
  application: ApplicationInfo;
  current: PageId;
  onNavigate: (page: PageId) => void;
};

export function Sidebar({ application, current, onNavigate }: SidebarProps) {
  const renderItem = (page: PageDefinition, extraClass = "") => {
    const active = page.id === current;
    return (
      <button
        className={`nav-item${active ? " active" : ""}${extraClass}`}
        key={page.id}
        type="button"
        aria-label={page.label}
        aria-current={active ? "page" : undefined}
        onClick={() => onNavigate(page.id)}
      >
        {page.icon}
        <span>{page.label}</span>
      </button>
    );
  };

  return (
    <aside className="sidebar">
      <div className="brand" aria-label={application.name}>
        <span className="brand-mark">A</span>
        <div>
          <Text weight="semibold" size={400}>
            Aether
          </Text>
          <Text className="brand-version" size={100}>
            {application.version}
          </Text>
        </div>
      </div>

      <nav className="navigation" aria-label="主要导航">
        {mainPages.map((page) => renderItem(page))}
      </nav>

      {renderItem(settingsPage, " settings")}
    </aside>
  );
}
