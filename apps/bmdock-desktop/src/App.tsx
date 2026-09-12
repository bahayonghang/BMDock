import { useEffect, useState, type ReactNode } from "react";
import type { CapabilitiesDto, EngineProfile, RuntimeStateDto } from "./ipc";
import { t } from "./i18n";
import {
  errorCategoryLabel,
  failureKindLabel,
  readShellSnapshot,
  runtimeStatusLabel,
  type ShellLoadState,
} from "./shell";

const SECTIONS = ["workbench", "runtime", "about"] as const;
type SectionId = (typeof SECTIONS)[number];

function sectionLabel(id: SectionId): string {
  switch (id) {
    case "workbench":
      return t("navWorkbench");
    case "runtime":
      return t("navRuntime");
    case "about":
      return t("navAbout");
    default: {
      const exhaustive: never = id;
      return exhaustive;
    }
  }
}

function profileLabel(profile: EngineProfile): string {
  switch (profile) {
    case "release":
      return t("profileRelease");
    case "main-preview":
      return t("profileMainPreview");
    default: {
      const exhaustive: never = profile;
      return exhaustive;
    }
  }
}

function App() {
  const [section, setSection] = useState<SectionId>("workbench");
  const [reloadToken, setReloadToken] = useState(0);
  const [load, setLoad] = useState<ShellLoadState>({ phase: "loading" });

  useEffect(() => {
    let cancelled = false;
    setLoad({ phase: "loading" });
    void readShellSnapshot().then((next) => {
      if (!cancelled) {
        setLoad(next);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [reloadToken]);

  return (
    <div className="app">
      <a className="skip-link" href="#main">
        {t("skipToMain")}
      </a>
      <header>
        <div className="brand-block">
          <p className="eyebrow">{t("appName")}</p>
          <h1>{t("tagline")}</h1>
        </div>
        <p className="notice">{t("g0Notice")}</p>
      </header>
      <div className="layout">
        <nav aria-label={t("navLabel")}>
          <ul className="nav-list">
            {SECTIONS.map((id) => (
              <li key={id}>
                <button
                  type="button"
                  className="nav-button"
                  aria-current={section === id ? "page" : undefined}
                  onClick={() => setSection(id)}
                >
                  {sectionLabel(id)}
                </button>
              </li>
            ))}
          </ul>
        </nav>
        <main id="main">
          <SectionBody
            section={section}
            load={load}
            onRefresh={() => setReloadToken((token) => token + 1)}
          />
        </main>
      </div>
    </div>
  );
}

function SectionBody({
  section,
  load,
  onRefresh,
}: {
  section: SectionId;
  load: ShellLoadState;
  onRefresh: () => void;
}) {
  switch (section) {
    case "workbench":
      return <WorkbenchPanel load={load} onRefresh={onRefresh} />;
    case "runtime":
      return <RuntimePanel load={load} onRefresh={onRefresh} />;
    case "about":
      return <AboutPanel />;
    default: {
      const exhaustive: never = section;
      return exhaustive;
    }
  }
}

function WorkbenchPanel({
  load,
  onRefresh,
}: {
  load: ShellLoadState;
  onRefresh: () => void;
}) {
  switch (load.phase) {
    case "error":
      return (
        <section className="panel" data-state="error" aria-labelledby="workbench-error-title" role="alert">
          <p className="state-badge">{t("errorBadge")}</p>
          <h2 id="workbench-error-title">{t("workbenchErrorTitle")}</h2>
          <p>
            {errorCategoryLabel(load.category)}：{load.message}
          </p>
          <p>{t("workbenchErrorBody")}</p>
          <button type="button" className="action" onClick={onRefresh}>
            {t("runtimeRefresh")}
          </button>
        </section>
      );
    case "loading":
    case "ready":
      return (
        <section className="panel" data-state="empty" aria-labelledby="workbench-title">
          <p className="state-badge">{t("emptyBadge")}</p>
          <h2 id="workbench-title">{t("workbenchEmptyTitle")}</h2>
          <p>{t("workbenchEmptyBody")}</p>
        </section>
      );
    default: {
      const exhaustive: never = load;
      return exhaustive;
    }
  }
}

function RuntimePanel({
  load,
  onRefresh,
}: {
  load: ShellLoadState;
  onRefresh: () => void;
}) {
  const refresh = (
    <button type="button" className="action" onClick={onRefresh}>
      {t("runtimeRefresh")}
    </button>
  );

  switch (load.phase) {
    case "loading":
      return (
        <section className="panel" data-state="status" aria-labelledby="runtime-title" aria-busy="true">
          <p className="state-badge">{t("statusBadge")}</p>
          <h2 id="runtime-title">{t("runtimeTitle")}</h2>
          <p role="status">{t("runtimeLoading")}</p>
        </section>
      );
    case "error":
      return (
        <section className="panel" data-state="error" aria-labelledby="runtime-error-title" role="alert">
          <p className="state-badge">{t("errorBadge")}</p>
          <h2 id="runtime-error-title">{t("runtimeErrorTitle")}</h2>
          <p>
            {errorCategoryLabel(load.category)}：{load.message}
          </p>
          {refresh}
        </section>
      );
    case "ready":
      return (
        <ReadyRuntime
          capabilities={load.capabilities}
          runtime={load.runtime}
          refresh={refresh}
        />
      );
    default: {
      const exhaustive: never = load;
      return exhaustive;
    }
  }
}

function ReadyRuntime({
  capabilities,
  runtime,
  refresh,
}: {
  capabilities: CapabilitiesDto;
  runtime: RuntimeStateDto;
  refresh: ReactNode;
}) {
  const empty = runtime.status === "not_started";
  const failed = runtime.status === "failed";
  const state = failed ? "error" : empty ? "empty" : "status";
  const badge = failed ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const title = failed
    ? t("runtimeFailedTitle")
    : empty
      ? t("runtimeEmptyTitle")
      : t("runtimeTitle");
  const description = empty ? t("runtimeEmptyBody") : null;

  return (
    <section
      className="panel"
      data-state={state}
      aria-labelledby="runtime-title"
      role={failed ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h2 id="runtime-title">{title}</h2>
      {description ? <p>{description}</p> : null}
      <dl className="facts">
        <div>
          <dt>{t("runtimeStatusLabel")}</dt>
          <dd>{runtimeStatusLabel(runtime.status)}</dd>
        </div>
        <div>
          <dt>{t("runtimeProfileLabel")}</dt>
          <dd>{runtime.profile ? profileLabel(runtime.profile) : t("runtimeNone")}</dd>
        </div>
        <div>
          <dt>{t("runtimeProjectLabel")}</dt>
          <dd>{runtime.project ?? t("runtimeNone")}</dd>
        </div>
        <div>
          <dt>{t("runtimeFailureLabel")}</dt>
          <dd>{runtime.failure ? failureKindLabel(runtime.failure) : t("runtimeNone")}</dd>
        </div>
      </dl>
      <h3>{t("capabilitiesTitle")}</h3>
      <p>
        {t("capabilitiesCommands")}：{capabilities.commands.join("、")}
      </p>
      <ul className="policy-list">
        <li>
          {t("policyFixture")}：{capabilities.policy.project}
        </li>
        <li>{t("policyNoPaths")}</li>
        <li>{t("policyNoCallTool")}</li>
      </ul>
      {refresh}
    </section>
  );
}

function AboutPanel() {
  return (
    <section className="panel" data-state="status" aria-labelledby="about-title">
      <p className="state-badge">{t("statusBadge")}</p>
      <h2 id="about-title">{t("aboutTitle")}</h2>
      <p>{t("aboutIntro")}</p>
      <p>{t("aboutSafety")}</p>
      <p>{t("aboutProfiles")}</p>
      <p>{t("aboutCommands")}</p>
    </section>
  );
}

export default App;
