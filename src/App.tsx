import { FluentProvider, webDarkTheme } from "@fluentui/react-components";
import { useEffect, useState } from "react";
import "./App.css";
import { Sidebar } from "./components/Sidebar";
import {
  defaultApplicationInfo,
  getApplicationInfo,
  isTauri,
} from "./ipc";
import type { ApplicationInfo } from "./ipc";
import { currentGreeting } from "./lib/greeting";
import { allPages } from "./navigation";
import type { PageId } from "./navigation";
import { HomePage } from "./pages/HomePage";
import { PlaceholderPage } from "./pages/PlaceholderPage";

function App() {
  const [application, setApplication] =
    useState<ApplicationInfo>(defaultApplicationInfo);
  const [pageId, setPageId] = useState<PageId>("home");
  const [greeting] = useState(currentGreeting);

  useEffect(() => {
    if (!isTauri()) {
      return;
    }

    let cancelled = false;
    void getApplicationInfo()
      .then((info) => {
        if (!cancelled) {
          setApplication(info);
        }
      })
      .catch(() => undefined);

    return () => {
      cancelled = true;
    };
  }, []);

  const page = allPages.find((candidate) => candidate.id === pageId) ?? allPages[0];

  return (
    <FluentProvider theme={webDarkTheme}>
      <div className="app-shell">
        <Sidebar
          application={application}
          current={pageId}
          onNavigate={setPageId}
        />
        <main className="workspace">
          {page.id === "home" ? (
            <HomePage
              application={application}
              greeting={greeting}
              summary={page.summary}
            />
          ) : (
            <PlaceholderPage page={page} />
          )}
        </main>
      </div>
    </FluentProvider>
  );
}

export default App;
