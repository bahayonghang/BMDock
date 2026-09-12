import { useEffect, useState, type ReactNode } from "react";
import {
  FIXTURE_PROJECT,
  TREE_PAGE_SIZE,
  copyFixtureRoute,
  invokeTyped,
  type CapabilitiesDto,
  type ConfigDiscoveryDto,
  type EngineProfile,
  type IpcResponse,
  type NoteReadDto,
  type PreflightDto,
  type ProjectCatalogDto,
  type RuntimeStateDto,
  type TreeEntryDto,
} from "./ipc";
import { t } from "./i18n";
import {
  errorCategoryLabel,
  failureKindLabel,
  readPreflightSnapshot,
  readShellSnapshot,
  runtimeStatusLabel,
  type PreflightLoadState,
  type ShellLoadState,
} from "./shell";

const SECTIONS = ["workbench", "runtime", "projects", "preflight", "about"] as const;
type SectionId = (typeof SECTIONS)[number];

function sectionLabel(id: SectionId): string {
  switch (id) {
    case "workbench":
      return t("navWorkbench");
    case "runtime":
      return t("navRuntime");
    case "projects":
      return t("navProjects");
    case "preflight":
      return t("navPreflight");
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
    case "projects":
      return <ProjectPanel load={load} onRefresh={onRefresh} />;
    case "preflight":
      return <PreflightPanel />;
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
      return (
        <section className="panel" data-state="status" aria-labelledby="workbench-title" aria-busy="true">
          <p className="state-badge">{t("statusBadge")}</p>
          <h2 id="workbench-title">{t("workbenchTitle")}</h2>
          <p role="status">{t("workbenchLoading")}</p>
        </section>
      );
    case "ready":
      return <WorkbenchLibrary onRefresh={onRefresh} />;
    default: {
      const exhaustive: never = load;
      return exhaustive;
    }
  }
}

type WorkbenchError = {
  category: "policy" | "schema" | "unsupported" | "invoke";
  message: string;
};

function unexpectedWorkbenchResponse(): WorkbenchError {
  return { category: "schema", message: t("unexpectedTree") };
}

function WorkbenchLibrary({ onRefresh }: { onRefresh: () => void }) {
  const [reloadToken, setReloadToken] = useState(0);
  const [phase, setPhase] = useState<"loading" | "ready" | "empty" | "error">("loading");
  const [entries, setEntries] = useState<TreeEntryDto[]>([]);
  const [nextCursor, setNextCursor] = useState<string | null>(null);
  const [note, setNote] = useState<NoteReadDto | null>(null);
  const [error, setError] = useState<WorkbenchError | null>(null);

  useEffect(() => {
    let cancelled = false;
    setPhase("loading");
    setEntries([]);
    setNextCursor(null);
    setNote(null);
    setError(null);
    void (async () => {
      try {
        const route = copyFixtureRoute();
        const response = await invokeTyped<IpcResponse>({
          command: "list_tree",
          args: {
            workspace: route.workspace,
            project: route.project,
            page_size: TREE_PAGE_SIZE,
          },
        });
        if (cancelled) {
          return;
        }
        switch (response.kind) {
          case "error":
            setError({ category: response.category, message: response.message });
            setPhase("error");
            return;
          case "tree_page":
            if (response.truncated) {
              setError({ category: "schema", message: t("unexpectedTree") });
              setPhase("error");
              return;
            }
            setEntries(response.entries);
            setNextCursor(response.next_cursor);
            setPhase(response.entries.length === 0 ? "empty" : "ready");
            return;
          case "capabilities":
          case "runtime_state":
          case "project_selected":
          case "project_catalog":
          case "preflight":
          case "config_discovery":
          case "note_read":
            setError(unexpectedWorkbenchResponse());
            setPhase("error");
            return;
          default: {
            const exhaustive: never = response;
            return exhaustive;
          }
        }
      } catch (cause) {
        if (!cancelled) {
          setError({
            category: "invoke",
            message: cause instanceof Error ? cause.message : String(cause),
          });
          setPhase("error");
        }
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [reloadToken]);

  const refresh = (
    <button
      type="button"
      className="action"
      onClick={() => {
        setReloadToken((token) => token + 1);
        onRefresh();
      }}
    >
      {t("workbenchRefresh")}
    </button>
  );

  if (phase === "loading") {
    return (
      <section className="panel" data-state="status" aria-labelledby="workbench-title" aria-busy="true">
        <p className="state-badge">{t("statusBadge")}</p>
        <h2 id="workbench-title">{t("workbenchTitle")}</h2>
        <p role="status">{t("workbenchLoading")}</p>
      </section>
    );
  }

  if (phase === "error" && error) {
    return (
      <section className="panel" data-state="error" aria-labelledby="workbench-error-title" role="alert">
        <p className="state-badge">{t("errorBadge")}</p>
        <h2 id="workbench-error-title">{t("workbenchErrorTitle")}</h2>
        <p>
          {errorCategoryLabel(error.category)}：{error.message}
        </p>
        <p>{t("workbenchErrorBody")}</p>
        {refresh}
      </section>
    );
  }

  const empty = phase === "empty";
  return (
    <section
      className="panel"
      data-state={empty ? "empty" : "status"}
      aria-labelledby="workbench-title"
    >
      <p className="state-badge">{empty ? t("emptyBadge") : t("statusBadge")}</p>
      <h2 id="workbench-title">{empty ? t("workbenchEmptyTitle") : t("workbenchReadyTitle")}</h2>
      <p>{empty ? t("workbenchEmptyBody") : t("workbenchReadyBody")}</p>
      <h3>{t("workbenchTreeTitle")}</h3>
      {empty ? null : (
        <ul className="tree-list">
          {entries.map((entry) => (
            <li key={entry.identifier}>
              <button
                type="button"
                className="tree-item"
                onClick={() => {
                  if (entry.kind === "note") {
                    void openNote(entry.identifier, setNote, setError, setPhase);
                  }
                }}
              >
                {entry.title}
              </button>
            </li>
          ))}
        </ul>
      )}
      {nextCursor ? (
        <button
          type="button"
          className="action"
          onClick={() => {
            void loadMoreTree(nextCursor, entries, setEntries, setNextCursor, setError, setPhase);
          }}
        >
          {t("workbenchLoadMore")}
        </button>
      ) : null}
      <NotePreview note={note} />
      {refresh}
    </section>
  );
}

async function openNote(
  identifier: string,
  setNote: (note: NoteReadDto | null) => void,
  setError: (error: WorkbenchError | null) => void,
  setPhase: (phase: "loading" | "ready" | "empty" | "error") => void,
): Promise<void> {
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "read_note",
      args: {
        workspace: route.workspace,
        project: route.project,
        identifier,
      },
    });
    switch (response.kind) {
      case "error":
        setError({ category: response.category, message: response.message });
        setPhase("error");
        return;
      case "note_read":
        setError(null);
        setNote({
          title: response.title,
          identifier: response.identifier,
          body: response.body,
          observation: response.observation,
        });
        return;
      case "capabilities":
      case "runtime_state":
      case "project_selected":
      case "project_catalog":
      case "preflight":
      case "config_discovery":
      case "tree_page":
        setError({ category: "schema", message: t("unexpectedNote") });
        setPhase("error");
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
    setPhase("error");
  }
}

async function loadMoreTree(
  cursor: string,
  current: TreeEntryDto[],
  setEntries: (entries: TreeEntryDto[]) => void,
  setNextCursor: (cursor: string | null) => void,
  setError: (error: WorkbenchError | null) => void,
  setPhase: (phase: "loading" | "ready" | "empty" | "error") => void,
): Promise<void> {
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "list_tree",
      args: {
        workspace: route.workspace,
        project: route.project,
        cursor,
        page_size: TREE_PAGE_SIZE,
      },
    });
    switch (response.kind) {
      case "error":
        setError({ category: response.category, message: response.message });
        setPhase("error");
        return;
      case "tree_page":
        if (response.truncated) {
          setError({ category: "schema", message: t("unexpectedTree") });
          setPhase("error");
          return;
        }
        setEntries([...current, ...response.entries]);
        setNextCursor(response.next_cursor);
        setPhase(current.length + response.entries.length === 0 ? "empty" : "ready");
        return;
      case "capabilities":
      case "runtime_state":
      case "project_selected":
      case "project_catalog":
      case "preflight":
      case "config_discovery":
      case "note_read":
        setError(unexpectedWorkbenchResponse());
        setPhase("error");
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
    setPhase("error");
  }
}

function observationLabel(note: NoteReadDto): string {
  switch (note.observation.classified_as) {
    case "body_matches_disk":
      return t("workbenchObservationDisk");
    case "accepted_unverified":
      return t("workbenchObservationUnverified");
    case "empty":
      return t("workbenchObservationEmpty");
    case "unclassified":
      return t("workbenchObservationUnclassified");
    default: {
      const exhaustive: never = note.observation.classified_as;
      return exhaustive;
    }
  }
}

function NotePreview({ note }: { note: NoteReadDto | null }) {
  if (!note) {
    return (
      <section className="subpanel" data-state="empty" aria-labelledby="note-preview-title">
        <p className="state-badge">{t("emptyBadge")}</p>
        <h3 id="note-preview-title">{t("workbenchPreviewTitle")}</h3>
        <p>{t("workbenchPreviewEmpty")}</p>
      </section>
    );
  }
  return (
    <section className="subpanel" data-state="status" aria-labelledby="note-preview-title">
      <p className="state-badge">{t("statusBadge")}</p>
      <h3 id="note-preview-title">{note.title}</h3>
      <p>
        {t("workbenchIdentifierLabel")}：{note.identifier}
      </p>
      <p>{observationLabel(note)}</p>
      <pre className="note-body">{note.body}</pre>
    </section>
  );
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

function ProjectPanel({
  load,
  onRefresh,
}: {
  load: ShellLoadState;
  onRefresh: () => void;
}) {
  const [selectError, setSelectError] = useState<{
    category: "policy" | "schema" | "unsupported" | "invoke";
    message: string;
  } | null>(null);

  const refresh = (
    <button
      type="button"
      className="action"
      onClick={() => {
        setSelectError(null);
        onRefresh();
      }}
    >
      {t("projectsRefresh")}
    </button>
  );

  switch (load.phase) {
    case "loading":
      return (
        <section className="panel" data-state="status" aria-labelledby="projects-title" aria-busy="true">
          <p className="state-badge">{t("statusBadge")}</p>
          <h2 id="projects-title">{t("projectsTitle")}</h2>
          <p role="status">{t("projectsLoading")}</p>
        </section>
      );
    case "error":
      return (
        <section className="panel" data-state="error" aria-labelledby="projects-error-title" role="alert">
          <p className="state-badge">{t("errorBadge")}</p>
          <h2 id="projects-error-title">{t("projectsErrorTitle")}</h2>
          <p>
            {errorCategoryLabel(load.category)}：{load.message}
          </p>
          {refresh}
        </section>
      );
    case "ready":
      if (selectError) {
        return (
          <section className="panel" data-state="error" aria-labelledby="projects-error-title" role="alert">
            <p className="state-badge">{t("errorBadge")}</p>
            <h2 id="projects-error-title">{t("projectsErrorTitle")}</h2>
            <p>
              {errorCategoryLabel(selectError.category)}：{selectError.message}
            </p>
            {refresh}
          </section>
        );
      }
      return (
        <ReadyProjects
          catalog={load.catalog}
          selectedProject={load.runtime.project}
          refresh={refresh}
          onSelectFixture={async () => {
            try {
              const response = await invokeTyped<IpcResponse>({
                command: "select_project",
                args: { project: FIXTURE_PROJECT },
              });
              switch (response.kind) {
                case "error":
                  setSelectError({
                    category: response.category,
                    message: response.message,
                  });
                  return;
                case "project_selected":
                  setSelectError(null);
                  onRefresh();
                  return;
                case "capabilities":
                case "runtime_state":
                case "project_catalog":
                case "preflight":
                case "config_discovery":
                case "tree_page":
                case "note_read":
                  setSelectError({
                    category: "schema",
                    message: t("unexpectedCatalog"),
                  });
                  return;
                default: {
                  const exhaustive: never = response;
                  return exhaustive;
                }
              }
            } catch (cause) {
              setSelectError({
                category: "invoke",
                message: cause instanceof Error ? cause.message : String(cause),
              });
            }
          }}
        />
      );
    default: {
      const exhaustive: never = load;
      return exhaustive;
    }
  }
}

function ReadyProjects({
  catalog,
  selectedProject,
  refresh,
  onSelectFixture,
}: {
  catalog: ProjectCatalogDto;
  selectedProject: RuntimeStateDto["project"];
  refresh: ReactNode;
  onSelectFixture: () => Promise<void>;
}) {
  const noneFound = catalog.projects.length === 0;
  const selected = selectedProject === FIXTURE_PROJECT;
  const state = selected ? "status" : "empty";
  const title = noneFound
    ? t("projectsEmptyNoneFoundTitle")
    : selected
      ? t("projectsReadyTitle")
      : t("projectsEmptyNoneSelectedTitle");
  const body = noneFound
    ? t("projectsEmptyNoneFoundBody")
    : selected
      ? t("projectsReadyBody")
      : t("projectsEmptyNoneSelectedBody");

  return (
    <section className="panel" data-state={state} aria-labelledby="projects-title">
      <p className="state-badge">{selected ? t("statusBadge") : t("emptyBadge")}</p>
      <h2 id="projects-title">{title}</h2>
      <p>{body}</p>
      <h3>{t("projectsCatalogTitle")}</h3>
      {noneFound ? null : (
        <dl className="facts">
          <div>
            <dt>{t("projectsWorkspaceLabel")}</dt>
            <dd>{catalog.workspaces[0]?.id ?? t("runtimeNone")}</dd>
          </div>
          <div>
            <dt>{t("projectsProjectLabel")}</dt>
            <dd>{catalog.projects[0]?.id ?? t("runtimeNone")}</dd>
          </div>
        </dl>
      )}
      <ul className="policy-list">
        <li>{t("projectsNoSearch")}</li>
        <li>{t("projectsNoImplicitWrite")}</li>
        <li>{t("projectsLocalOffline")}</li>
        <li>{t("projectsNoVault")}</li>
      </ul>
      {noneFound || selected ? null : (
        <button type="button" className="action" onClick={() => void onSelectFixture()}>
          {t("projectsSelectFixture")}
        </button>
      )}
      {refresh}
    </section>
  );
}

function PreflightPanel() {
  const [reloadToken, setReloadToken] = useState(0);
  const [load, setLoad] = useState<PreflightLoadState>({ phase: "loading" });

  useEffect(() => {
    let cancelled = false;
    setLoad({ phase: "loading" });
    void readPreflightSnapshot().then((next) => {
      if (!cancelled) {
        setLoad(next);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [reloadToken]);

  const refresh = (
    <button type="button" className="action" onClick={() => setReloadToken((token) => token + 1)}>
      {t("preflightRefresh")}
    </button>
  );

  switch (load.phase) {
    case "loading":
      return (
        <section className="panel" data-state="status" aria-labelledby="preflight-title" aria-busy="true">
          <p className="state-badge">{t("statusBadge")}</p>
          <h2 id="preflight-title">{t("preflightTitle")}</h2>
          <p role="status">{t("preflightLoading")}</p>
        </section>
      );
    case "error":
      return (
        <section className="panel" data-state="error" aria-labelledby="preflight-error-title" role="alert">
          <p className="state-badge">{t("errorBadge")}</p>
          <h2 id="preflight-error-title">{t("preflightErrorTitle")}</h2>
          <p>
            {errorCategoryLabel(load.category)}：{load.message}
          </p>
          <p>{t("preflightBody")}</p>
          {refresh}
        </section>
      );
    case "ready":
      return <ReadyPreflight preflight={load.preflight} discovery={load.discovery} refresh={refresh} />;
    default: {
      const exhaustive: never = load;
      return exhaustive;
    }
  }
}

function ReadyPreflight({
  preflight,
  discovery,
  refresh,
}: {
  preflight: PreflightDto;
  discovery: ConfigDiscoveryDto;
  refresh: ReactNode;
}) {
  const discoveryEmpty = discovery.candidates.length === 0;

  return (
    <section className="panel" data-state="status" aria-labelledby="preflight-title">
      <p className="state-badge">{t("statusBadge")}</p>
      <h2 id="preflight-title">{t("preflightReadyTitle")}</h2>
      <p>{t("preflightBody")}</p>
      <h3>{t("preflightProfilesTitle")}</h3>
      <ul className="policy-list">
        {preflight.profiles.map((profile) => (
          <li key={profile.id}>
            {profileLabel(profile.id)} · {t("preflightToolsLabel")} {profile.expected_tools} ·{" "}
            {t("preflightCommitLabel")} {profile.commit.slice(0, 8)}
          </li>
        ))}
      </ul>
      <h3>{t("preflightHostTitle")}</h3>
      <dl className="facts">
        <div>
          <dt>{t("preflightSupervisorLabel")}</dt>
          <dd>{preflight.host.supervisor_idle ? t("preflightIdle") : t("preflightNotIdle")}</dd>
        </div>
        <div>
          <dt>{t("runtimeStatusLabel")}</dt>
          <dd>{runtimeStatusLabel(preflight.host.supervisor_status)}</dd>
        </div>
        <div>
          <dt>{t("preflightEngineSpawnedLabel")}</dt>
          <dd>
            {preflight.host.engine_spawned
              ? t("preflightEnginePresent")
              : t("preflightEngineAbsent")}
          </dd>
        </div>
        <div>
          <dt>{t("preflightFilesWrittenLabel")}</dt>
          <dd>
            {preflight.host.files_written ? t("preflightWroteFiles") : t("preflightNoWrite")}
          </dd>
        </div>
      </dl>
      <ul className="policy-list">
        <li>{t("preflightNoCloud")}</li>
        <li>{t("preflightNoSpawn")}</li>
        <li>{t("preflightLocalOffline")}</li>
      </ul>
      <DiscoveryBlock discovery={discovery} empty={discoveryEmpty} />
      {refresh}
    </section>
  );
}

function DiscoveryBlock({
  discovery,
  empty,
}: {
  discovery: ConfigDiscoveryDto;
  empty: boolean;
}) {
  return (
    <section
      className="subpanel"
      data-state={empty ? "empty" : "status"}
      aria-labelledby="discovery-title"
    >
      <p className="state-badge">{empty ? t("emptyBadge") : t("statusBadge")}</p>
      <h3 id="discovery-title">{empty ? t("discoveryEmptyTitle") : t("discoveryReadyTitle")}</h3>
      {empty ? <p>{t("discoveryEmptyBody")}</p> : null}
      <dl className="facts">
        <div>
          <dt>{t("discoveryRootLabel")}</dt>
          <dd>{discovery.root === "none" ? t("discoveryNone") : discovery.root}</dd>
        </div>
      </dl>
      {empty ? null : (
        <ul className="policy-list">
          {discovery.candidates.map((candidate) => (
            <li key={candidate.path}>
              {candidate.kind}：{candidate.path}
            </li>
          ))}
        </ul>
      )}
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
