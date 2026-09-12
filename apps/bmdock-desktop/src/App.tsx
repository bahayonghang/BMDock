import { useEffect, useState, type ReactNode } from "react";
import {
  FIXTURE_PROJECT,
  OWNED_KIND,
  TREE_PAGE_SIZE,
  copyFixtureRoute,
  invokeTyped,
  type BackupCatalogDto,
  type BackupRecordDto,
  type CapabilitiesDto,
  type ConfigDiscoveryDto,
  type DrainResultDto,
  type EngineProfile,
  type IpcResponse,
  type NoteReadDto,
  type RelationListDto,
  type RelationDto,
  type GraphPageDto,
  type GraphNodeDto,
  type SearchPageDto,
  type SearchHitDto,
  type SearchInspectorDto,
  type RecallBenchmarkDto,
  type ContextPreviewDto,
  type ActivityPageDto,
  type ActivityEntryDto,
  type PreflightDto,
  type ProjectCatalogDto,
  type RestoreResultDto,
  type RuntimeStateDto,
  type TreeEntryDto,
  type WindowsRuntimeDto,
  type DraftResultDto,
  type NoteWriteDto,
  type NoteEditDto,
  type NoteMoveDto,
  type NoteDeleteDto,
  type NoteCrudClass,
  type FailureKind,
} from "./ipc";
import { t } from "./i18n";
import {
  classifyBody,
  type LineEndingClass,
} from "./contentSafety";
import {
  errorCategoryLabel,
  drainPhaseLabel,
  failureKindLabel,
  readPreflightSnapshot,
  readShellSnapshot,
  runtimeStatusLabel,
  type PreflightLoadState,
  type ShellLoadState,
} from "./shell";

const SECTIONS = ["workbench", "runtime", "projects", "preflight", "backups", "about"] as const;
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
    case "backups":
      return t("navBackups");
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
      return (
        <div className="stack">
          <RuntimePanel load={load} onRefresh={onRefresh} />
          <WindowsRuntimeCard />
        </div>
      );
    case "projects":
      return <ProjectPanel load={load} onRefresh={onRefresh} />;
    case "preflight":
      return <PreflightPanel />;
    case "backups":
      return <BackupPanel />;
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
      return <WorkbenchLibrary onRefresh={onRefresh} runtimeFailure={load.runtime.failure} />;
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

function WorkbenchLibrary({
  onRefresh,
  runtimeFailure,
}: {
  onRefresh: () => void;
  runtimeFailure: FailureKind | null;
}) {
  const [reloadToken, setReloadToken] = useState(0);
  const [phase, setPhase] = useState<"loading" | "ready" | "empty" | "error">("loading");
  const [entries, setEntries] = useState<TreeEntryDto[]>([]);
  const [nextCursor, setNextCursor] = useState<string | null>(null);
  const [note, setNote] = useState<NoteReadDto | null>(null);
  const [relations, setRelations] = useState<RelationListDto | null>(null);
  const [relationsError, setRelationsError] = useState<WorkbenchError | null>(null);
  const [graph, setGraph] = useState<GraphPageDto | null>(null);
  const [graphError, setGraphError] = useState<WorkbenchError | null>(null);
  const [search, setSearch] = useState<SearchPageDto | null>(null);
  const [searchError, setSearchError] = useState<WorkbenchError | null>(null);
  const [inspector, setInspector] = useState<SearchInspectorDto | null>(null);
  const [inspectorError, setInspectorError] = useState<WorkbenchError | null>(null);
  const [recall, setRecall] = useState<RecallBenchmarkDto | null>(null);
  const [recallError, setRecallError] = useState<WorkbenchError | null>(null);
  const [preview, setPreview] = useState<ContextPreviewDto | null>(null);
  const [previewError, setPreviewError] = useState<WorkbenchError | null>(null);
  const [activity, setActivity] = useState<ActivityPageDto | null>(null);
  const [activityError, setActivityError] = useState<WorkbenchError | null>(null);
  const [crudObservation, setCrudObservation] = useState<{
    identifier: string;
    classified_as: NoteCrudClass;
  } | null>(null);
  const [error, setError] = useState<WorkbenchError | null>(null);

  useEffect(() => {
    let cancelled = false;
    setPhase("loading");
    setEntries([]);
    setNextCursor(null);
    setNote(null);
    setRelations(null);
    setRelationsError(null);
    setGraph(null);
    setGraphError(null);
    setSearch(null);
    setSearchError(null);
    setInspector(null);
    setInspectorError(null);
    setRecall(null);
    setRecallError(null);
    setPreview(null);
    setPreviewError(null);
    setActivity(null);
    setActivityError(null);
    setCrudObservation(null);
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
            void loadActivity(setActivity, setActivityError);
            return;
          case "capabilities":
          case "runtime_state":
          case "project_selected":
          case "project_catalog":
          case "preflight":
          case "config_discovery":
          case "note_read":
          case "backup_catalog":
          case "fixture_restored":
          case "windows_runtime":
          case "draft_saved":
          case "draft_loaded":
          case "note_written":
          case "note_edited":
          case "note_moved":
          case "note_deleted":
          case "relation_list":
      case "graph_page":
      case "search_page":
      case "context_preview":
      case "activity_page":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
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
                    void openNote(
                      entry.identifier,
                      setNote,
                      setRelations,
                      setRelationsError,
                      setGraph,
                      setGraphError,
                      setPreview,
                      setPreviewError,
                      setError,
                      setPhase,
                    );
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
      <ObservationPanel
        note={note}
        relations={relations}
        relationsError={relationsError}
        crudObservation={crudObservation}
        graph={graph}
      />
      <RelationPanel relations={relations} error={relationsError} />
      <GraphPanel
        graph={graph}
        error={graphError}
        onLoadMore={() => {
          if (graph?.next_cursor) {
            void loadMoreGraph(graph, setGraph, setGraphError);
          }
        }}
        onExpand={(identifier) => {
          void loadGraph(identifier, setGraph, setGraphError);
        }}
      />
      <SearchPanel
        search={search}
        error={searchError}
        onSearch={(query) => {
          void runSearch(query, setSearch, setSearchError);
          void runInspectSearch(query, null, setInspector, setInspectorError);
        }}
        onPreview={(identifier, query) => {
          void loadContextPreview(identifier, query, setPreview, setPreviewError);
          void runInspectSearch(query, identifier, setInspector, setInspectorError);
        }}
        onLoadMore={() => {
          if (search?.next_cursor) {
            void loadMoreSearch(search, setSearch, setSearchError);
          }
        }}
      />
      <SearchInspectorPanel
        inspector={inspector}
        error={inspectorError}
        onInspect={(query, identifier) => {
          void runInspectSearch(query, identifier, setInspector, setInspectorError);
        }}
      />
      <RecallBenchmarkPanel
        recall={recall}
        error={recallError}
        onRun={(k) => {
          void runRecallBenchmark(k, setRecall, setRecallError);
        }}
      />
      <ContextPreviewPanel preview={preview} error={previewError} />
      <ActivityPanel
        activity={activity}
        error={activityError}
        onLoadMore={() => {
          if (activity?.next_cursor) {
            void loadMoreActivity(activity, setActivity, setActivityError);
          }
        }}
      />
      <NoteCrudPanel
        seedIdentifier={note?.identifier ?? null}
        seedTitle={note?.title ?? null}
        seedBody={note?.body ?? null}
        runtimeFailure={runtimeFailure}
        onMutated={() => setReloadToken((token) => token + 1)}
        onObservation={(identifier, classified_as) => {
          setCrudObservation({ identifier, classified_as });
        }}
      />
      <DraftEditor seedIdentifier={note?.identifier ?? null} seedBody={note?.body ?? null} />
      {refresh}
    </section>
  );
}

async function openNote(
  identifier: string,
  setNote: (note: NoteReadDto | null) => void,
  setRelations: (relations: RelationListDto | null) => void,
  setRelationsError: (error: WorkbenchError | null) => void,
  setGraph: (graph: GraphPageDto | null) => void,
  setGraphError: (error: WorkbenchError | null) => void,
  setPreview: (preview: ContextPreviewDto | null) => void,
  setPreviewError: (error: WorkbenchError | null) => void,
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
        await loadRelations(identifier, setRelations, setRelationsError);
        await loadGraph(identifier, setGraph, setGraphError);
        await loadContextPreview(identifier, null, setPreview, setPreviewError);
        return;
      case "capabilities":
      case "runtime_state":
      case "project_selected":
      case "project_catalog":
      case "preflight":
      case "config_discovery":
      case "tree_page":
      case "relation_list":
      case "backup_catalog":
      case "fixture_restored":
      case "windows_runtime":
      case "draft_saved":
      case "draft_loaded":
      case "note_written":
      case "note_edited":
      case "note_moved":
      case "note_deleted":
      case "graph_page":
      case "search_page":
      case "context_preview":
      case "activity_page":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
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

async function loadRelations(
  identifier: string,
  setRelations: (relations: RelationListDto | null) => void,
  setRelationsError: (error: WorkbenchError | null) => void,
): Promise<void> {
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "list_relations",
      args: {
        workspace: route.workspace,
        project: route.project,
        identifier,
      },
    });
    switch (response.kind) {
      case "error":
        setRelations(null);
        setRelationsError({ category: response.category, message: response.message });
        return;
      case "relation_list":
        setRelationsError(null);
        setRelations({
          identifier: response.identifier,
          relations: response.relations,
          observation: response.observation,
          engine_graph: false,
          scanned_user_obsidian_vault: false,
          scanned_user_basic_memory_home: false,
          files_written: response.files_written,
        });
        return;
      case "capabilities":
      case "runtime_state":
      case "project_selected":
      case "project_catalog":
      case "preflight":
      case "config_discovery":
      case "tree_page":
      case "note_read":
      case "backup_catalog":
      case "fixture_restored":
      case "windows_runtime":
      case "draft_saved":
      case "draft_loaded":
      case "note_written":
      case "note_edited":
      case "note_moved":
      case "note_deleted":
      case "graph_page":
      case "search_page":
      case "context_preview":
      case "activity_page":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
        setRelations(null);
        setRelationsError({ category: "schema", message: t("unexpectedRelations") });
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setRelations(null);
    setRelationsError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

function unexpectedGraphResponse(): WorkbenchError {
  return { category: "schema", message: t("unexpectedGraph") };
}

function asGraphPage(response: Extract<IpcResponse, { kind: "graph_page" }>): GraphPageDto {
  return {
    identifier: response.identifier,
    nodes: response.nodes,
    edges: response.edges,
    next_cursor: response.next_cursor,
    page: response.page,
    truncated: response.truncated,
    observation: response.observation,
    engine_graph: false,
    scanned_user_obsidian_vault: false,
    scanned_user_basic_memory_home: false,
    files_written: response.files_written,
    depth: 1,
  };
}

function mergeGraphPage(current: GraphPageDto, next: GraphPageDto): GraphPageDto {
  const nodes = [...current.nodes];
  for (const node of next.nodes) {
    if (!nodes.some((existing) => existing.identifier === node.identifier)) {
      nodes.push(node);
    }
  }
  return {
    ...next,
    nodes,
    edges: [...current.edges, ...next.edges],
  };
}

async function loadGraph(
  identifier: string,
  setGraph: (graph: GraphPageDto | null) => void,
  setGraphError: (error: WorkbenchError | null) => void,
): Promise<void> {
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "expand_graph",
      args: {
        workspace: route.workspace,
        project: route.project,
        identifier,
        page_size: TREE_PAGE_SIZE,
      },
    });
    switch (response.kind) {
      case "error":
        setGraph(null);
        setGraphError({ category: response.category, message: response.message });
        return;
      case "graph_page":
        if (response.truncated) {
          setGraph(null);
          setGraphError(unexpectedGraphResponse());
          return;
        }
        setGraphError(null);
        setGraph(asGraphPage(response));
        return;
      case "capabilities":
      case "runtime_state":
      case "project_selected":
      case "project_catalog":
      case "preflight":
      case "config_discovery":
      case "tree_page":
      case "note_read":
      case "relation_list":
      case "backup_catalog":
      case "fixture_restored":
      case "windows_runtime":
      case "draft_saved":
      case "draft_loaded":
      case "note_written":
      case "note_edited":
      case "note_moved":
      case "note_deleted":
      case "search_page":
      case "context_preview":
      case "activity_page":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
        setGraph(null);
        setGraphError(unexpectedGraphResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setGraph(null);
    setGraphError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

async function loadMoreGraph(
  current: GraphPageDto,
  setGraph: (graph: GraphPageDto | null) => void,
  setGraphError: (error: WorkbenchError | null) => void,
): Promise<void> {
  if (!current.next_cursor) {
    return;
  }
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "expand_graph",
      args: {
        workspace: route.workspace,
        project: route.project,
        identifier: current.identifier,
        cursor: current.next_cursor,
        page_size: TREE_PAGE_SIZE,
      },
    });
    switch (response.kind) {
      case "error":
        setGraphError({ category: response.category, message: response.message });
        return;
      case "graph_page":
        if (response.truncated) {
          setGraphError(unexpectedGraphResponse());
          return;
        }
        setGraphError(null);
        setGraph(mergeGraphPage(current, asGraphPage(response)));
        return;
      case "capabilities":
      case "runtime_state":
      case "project_selected":
      case "project_catalog":
      case "preflight":
      case "config_discovery":
      case "tree_page":
      case "note_read":
      case "relation_list":
      case "backup_catalog":
      case "fixture_restored":
      case "windows_runtime":
      case "draft_saved":
      case "draft_loaded":
      case "note_written":
      case "note_edited":
      case "note_moved":
      case "note_deleted":
      case "search_page":
      case "context_preview":
      case "activity_page":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
        setGraphError(unexpectedGraphResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setGraphError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
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
      case "backup_catalog":
      case "fixture_restored":
      case "windows_runtime":
      case "draft_saved":
      case "draft_loaded":
      case "note_written":
      case "note_edited":
      case "note_moved":
      case "note_deleted":
      case "relation_list":
      case "graph_page":
      case "search_page":
      case "context_preview":
      case "activity_page":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
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

function lineEndingLabel(kind: LineEndingClass): string {
  switch (kind) {
    case "none":
      return t("contentSafetyLineNone");
    case "lf":
      return t("contentSafetyLineLf");
    case "crlf":
      return t("contentSafetyLineCrlf");
    case "mixed":
      return t("contentSafetyLineMixed");
    default: {
      const exhaustive: never = kind;
      return exhaustive;
    }
  }
}

function ContentSafetyFacts({ body, previewId }: { body: string; previewId: string }) {
  const safety = classifyBody(body);
  return (
    <div className="content-safety">
      <h4 id={`${previewId}-safety`}>{t("contentSafetyTitle")}</h4>
      <dl className="facts">
        <div>
          <dt>{t("contentSafetyHtmlLabel")}</dt>
          <dd data-unsafe-html={safety.unsafe_html_present ? "true" : "false"}>
            {safety.unsafe_html_present ? t("contentSafetyHtmlPresent") : t("contentSafetyHtmlAbsent")}
          </dd>
        </div>
        <div>
          <dt>{t("contentSafetyExecutedLabel")}</dt>
          <dd data-executed="false">{t("contentSafetyExecutedNo")}</dd>
        </div>
        <div>
          <dt>{t("contentSafetyLineLabel")}</dt>
          <dd data-line-endings={safety.line_endings}>{lineEndingLabel(safety.line_endings)}</dd>
        </div>
      </dl>
      <p>{t("contentSafetyImeUnverified")}</p>
      <p>{t("contentSafetyHelpNotT39")}</p>
      <p id={previewId}>{t("contentSafetyPreviewLabel")}</p>
      <pre className="note-body" data-preview="text" data-executed="false" aria-labelledby={previewId}>
        {body}
      </pre>
    </div>
  );
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
      <ContentSafetyFacts body={note.body} previewId="note-preview-text" />
    </section>
  );
}

function mapReadObservation(note: NoteReadDto): NoteCrudClass {
  switch (note.observation.classified_as) {
    case "body_matches_disk":
      return "disk_verified";
    case "accepted_unverified":
      return "accepted_unverified";
    case "empty":
      return "empty";
    case "unclassified":
      return "unclassified";
    default: {
      const exhaustive: never = note.observation.classified_as;
      return exhaustive;
    }
  }
}

function observationClassLabel(classified: NoteCrudClass): string {
  switch (classified) {
    case "disk_verified":
      return t("observationDisk");
    case "accepted_unverified":
      return t("observationUnverified");
    case "conflict":
      return t("observationConflict");
    case "empty":
      return t("observationEmpty");
    case "unclassified":
      return t("observationUnclassified");
    default: {
      const exhaustive: never = classified;
      return exhaustive;
    }
  }
}

function ObservationPanel({
  note,
  relations,
  relationsError,
  crudObservation,
  graph,
}: {
  note: NoteReadDto | null;
  relations: RelationListDto | null;
  relationsError: WorkbenchError | null;
  crudObservation: { identifier: string; classified_as: NoteCrudClass } | null;
  graph: GraphPageDto | null;
}) {
  const currentId =
    note?.identifier ??
    relations?.identifier ??
    graph?.identifier ??
    crudObservation?.identifier ??
    null;
  let classified: NoteCrudClass = "empty";
  if (crudObservation && (currentId === null || crudObservation.identifier === currentId)) {
    classified = crudObservation.classified_as;
  } else if (graph) {
    classified = graph.observation.classified_as;
  } else if (relations) {
    classified = relations.observation.classified_as;
  } else if (note) {
    classified = mapReadObservation(note);
  }
  if (relationsError && !note && !relations && !crudObservation && !graph) {
    return (
      <section className="subpanel" data-state="error" aria-labelledby="observation-title" role="alert">
        <p className="state-badge">{t("errorBadge")}</p>
        <h3 id="observation-title">{t("observationErrorTitle")}</h3>
        <p>
          {errorCategoryLabel(relationsError.category)}：{relationsError.message}
        </p>
      </section>
    );
  }
  const empty = classified === "empty" && !note && !relations && !crudObservation && !graph;
  return (
    <section
      className="subpanel"
      data-state={empty ? "empty" : "status"}
      aria-labelledby="observation-title"
    >
      <p className="state-badge">{empty ? t("emptyBadge") : t("statusBadge")}</p>
      <h3 id="observation-title">{empty ? t("observationEmptyTitle") : t("observationReadyTitle")}</h3>
      <p>{empty ? t("observationEmptyBody") : t("observationReadyBody")}</p>
      <p>{t("observationDistinctFromRelations")}</p>
      <dl className="facts">
        <div>
          <dt>{t("workbenchIdentifierLabel")}</dt>
          <dd>{currentId ?? "—"}</dd>
        </div>
        <div>
          <dt>classified_as</dt>
          <dd data-observation={classified}>{observationClassLabel(classified)}</dd>
        </div>
      </dl>
      <ul className="observation-legend">
        <li>{t("observationDisk")}</li>
        <li>{t("observationUnverified")}</li>
        <li>{t("observationConflict")}</li>
        <li>{t("observationEmpty")}</li>
      </ul>
    </section>
  );
}

function relationTargetLabel(classified: RelationDto["classified_as"]): string {
  switch (classified) {
    case "present":
      return t("relationsPresent");
    case "empty":
      return t("relationsMissing");
    case "unsupported":
      return t("relationsUnsupported");
    default: {
      const exhaustive: never = classified;
      return exhaustive;
    }
  }
}

function RelationPanel({
  relations,
  error,
}: {
  relations: RelationListDto | null;
  error: WorkbenchError | null;
}) {
  if (error) {
    return (
      <section className="subpanel" data-state="error" aria-labelledby="relations-title" role="alert">
        <p className="state-badge">{t("errorBadge")}</p>
        <h3 id="relations-title">{t("relationsErrorTitle")}</h3>
        <p>
          {errorCategoryLabel(error.category)}：{error.message}
        </p>
      </section>
    );
  }
  const empty = relations === null || relations.relations.length === 0;
  return (
    <section
      className="subpanel"
      data-state={empty ? "empty" : "status"}
      aria-labelledby="relations-title"
    >
      <p className="state-badge">{empty ? t("emptyBadge") : t("statusBadge")}</p>
      <h3 id="relations-title">{empty ? t("relationsEmptyTitle") : t("relationsReadyTitle")}</h3>
      <p>{empty ? t("relationsEmptyBody") : t("relationsReadyBody")}</p>
      <p>{t("relationsPermalinkNotPath")}</p>
      <p>{t("relationsNotEngineGraph")}</p>
      <p>{t("relationsMcpUnverified")}</p>
      {relations && relations.relations.length > 0 ? (
        <ul className="relation-list">
          {relations.relations.map((item) => (
            <li key={item.identifier} data-relation={item.classified_as}>
              <span>{item.identifier}</span>
              <span>{relationTargetLabel(item.classified_as)}</span>
            </li>
          ))}
        </ul>
      ) : null}
    </section>
  );
}

function graphNodeLabel(classified: GraphNodeDto["classified_as"]): string {
  switch (classified) {
    case "present":
      return t("graphNodePresent");
    case "empty":
      return t("graphNodeEmpty");
    default: {
      const exhaustive: never = classified;
      return exhaustive;
    }
  }
}

function GraphPanel({
  graph,
  error,
  onLoadMore,
  onExpand,
}: {
  graph: GraphPageDto | null;
  error: WorkbenchError | null;
  onLoadMore: () => void;
  onExpand: (identifier: string) => void;
}) {
  if (error) {
    return (
      <section className="subpanel" data-state="error" aria-labelledby="graph-title" role="alert">
        <p className="state-badge">{t("errorBadge")}</p>
        <h3 id="graph-title">{t("graphErrorTitle")}</h3>
        <p>
          {errorCategoryLabel(error.category)}：{error.message}
        </p>
      </section>
    );
  }
  const empty = graph === null || (graph.nodes.length === 0 && graph.edges.length === 0);
  return (
    <section
      className="subpanel"
      data-state={empty ? "empty" : "status"}
      aria-labelledby="graph-title"
    >
      <p className="state-badge">{empty ? t("emptyBadge") : t("statusBadge")}</p>
      <h3 id="graph-title">{empty ? t("graphEmptyTitle") : t("graphReadyTitle")}</h3>
      <p>{empty ? t("graphEmptyBody") : t("graphReadyBody")}</p>
      <p>{t("graphObservationDistinct")}</p>
      <p>{t("graphNotEngineGraph")}</p>
      <p>{t("graphChineseKept")}</p>
      <p>{t("graphBounded")}</p>
      <p>{t("graphDepthOne")}</p>
      {graph ? (
        <dl className="facts">
          <div>
            <dt>{t("workbenchIdentifierLabel")}</dt>
            <dd>{graph.identifier}</dd>
          </div>
          <div>
            <dt>classified_as</dt>
            <dd data-observation={graph.observation.classified_as}>
              {observationClassLabel(graph.observation.classified_as)}
            </dd>
          </div>
        </dl>
      ) : null}
      {graph && graph.nodes.length > 0 ? (
        <>
          <h4>{t("graphNodesTitle")}</h4>
          <ul className="graph-list">
            {graph.nodes.map((node) => (
              <li key={node.identifier} data-graph-node={node.classified_as}>
                <span>{node.identifier}</span>
                <span>{graphNodeLabel(node.classified_as)}</span>
                {node.identifier === graph.identifier ? null : (
                  <button type="button" className="action" onClick={() => onExpand(node.identifier)}>
                    {t("graphExpand")}
                  </button>
                )}
              </li>
            ))}
          </ul>
        </>
      ) : null}
      {graph && graph.edges.length > 0 ? (
        <>
          <h4>{t("graphEdgesTitle")}</h4>
          <ul className="graph-list">
            {graph.edges.map((edge) => (
              <li key={`${edge.source}->${edge.target}`}>
                <span>
                  {edge.source} → {edge.target}
                </span>
              </li>
            ))}
          </ul>
        </>
      ) : null}
      {graph?.next_cursor ? (
        <div className="graph-actions">
          <button type="button" className="action" onClick={onLoadMore}>
            {t("graphLoadMore")}
          </button>
        </div>
      ) : null}
    </section>
  );
}

function unexpectedSearchResponse(): WorkbenchError {
  return { category: "schema", message: t("unexpectedSearch") };
}

function asSearchPage(response: Extract<IpcResponse, { kind: "search_page" }>): SearchPageDto {
  return {
    query: response.query,
    hits: response.hits,
    next_cursor: response.next_cursor,
    page: response.page,
    truncated: response.truncated,
    observation: response.observation,
    semantic_enabled: false,
    engine_search: false,
    scanned_user_obsidian_vault: false,
    scanned_user_basic_memory_home: false,
    files_written: response.files_written,
  };
}

function mergeSearchPage(current: SearchPageDto, next: SearchPageDto): SearchPageDto {
  const hits = [...current.hits];
  for (const hit of next.hits) {
    if (!hits.some((existing) => existing.identifier === hit.identifier)) {
      hits.push(hit);
    }
  }
  return {
    ...next,
    hits,
  };
}

async function runSearch(
  query: string,
  setSearch: (search: SearchPageDto | null) => void,
  setSearchError: (error: WorkbenchError | null) => void,
): Promise<void> {
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "search_notes",
      args: {
        workspace: route.workspace,
        project: route.project,
        query,
        page_size: TREE_PAGE_SIZE,
      },
    });
    switch (response.kind) {
      case "error":
        setSearch(null);
        setSearchError({ category: response.category, message: response.message });
        return;
      case "search_page":
        if (response.truncated) {
          setSearch(null);
          setSearchError(unexpectedSearchResponse());
          return;
        }
        setSearchError(null);
        setSearch(asSearchPage(response));
        return;
      case "capabilities":
      case "runtime_state":
      case "project_selected":
      case "project_catalog":
      case "preflight":
      case "config_discovery":
      case "tree_page":
      case "note_read":
      case "relation_list":
      case "graph_page":
      case "backup_catalog":
      case "fixture_restored":
      case "windows_runtime":
      case "draft_saved":
      case "draft_loaded":
      case "note_written":
      case "note_edited":
      case "note_moved":
      case "note_deleted":
      case "context_preview":
      case "activity_page":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
        setSearch(null);
        setSearchError(unexpectedSearchResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setSearch(null);
    setSearchError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

async function loadMoreSearch(
  current: SearchPageDto,
  setSearch: (search: SearchPageDto | null) => void,
  setSearchError: (error: WorkbenchError | null) => void,
): Promise<void> {
  if (!current.next_cursor) {
    return;
  }
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "search_notes",
      args: {
        workspace: route.workspace,
        project: route.project,
        query: current.query,
        cursor: current.next_cursor,
        page_size: TREE_PAGE_SIZE,
      },
    });
    switch (response.kind) {
      case "error":
        setSearchError({ category: response.category, message: response.message });
        return;
      case "search_page":
        if (response.truncated) {
          setSearchError(unexpectedSearchResponse());
          return;
        }
        setSearchError(null);
        setSearch(mergeSearchPage(current, asSearchPage(response)));
        return;
      case "capabilities":
      case "runtime_state":
      case "project_selected":
      case "project_catalog":
      case "preflight":
      case "config_discovery":
      case "tree_page":
      case "note_read":
      case "relation_list":
      case "graph_page":
      case "backup_catalog":
      case "fixture_restored":
      case "windows_runtime":
      case "draft_saved":
      case "draft_loaded":
      case "note_written":
      case "note_edited":
      case "note_moved":
      case "note_deleted":
      case "context_preview":
      case "activity_page":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
        setSearchError(unexpectedSearchResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setSearchError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

function SearchPanel({
  search,
  error,
  onSearch,
  onPreview,
  onLoadMore,
}: {
  search: SearchPageDto | null;
  error: WorkbenchError | null;
  onSearch: (query: string) => void;
  onPreview: (identifier: string, query: string) => void;
  onLoadMore: () => void;
}) {
  const [query, setQuery] = useState("");
  const empty = search === null || search.hits.length === 0;
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error ? t("searchErrorTitle") : empty ? t("searchEmptyTitle") : t("searchReadyTitle");
  return (
    <section
      className="subpanel"
      data-state={state}
      aria-labelledby="search-title"
      role={error ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h3 id="search-title">{heading}</h3>
      <p>
        {error ? `${errorCategoryLabel(error.category)}：${error.message}` : empty ? t("searchEmptyBody") : t("searchReadyBody")}
      </p>
      <p>{t("searchLexicalNotSemantic")}</p>
      <p>{t("searchPermalinkNotPath")}</p>
      <p>{t("searchMcpUnverified")}</p>
      <form
        className="search-form"
        onSubmit={(event) => {
          event.preventDefault();
          const next = query.trim();
          if (next === "") {
            return;
          }
          onSearch(next);
        }}
      >
        <label htmlFor="search-query">{t("searchQueryLabel")}</label>
        <input
          id="search-query"
          type="text"
          value={query}
          autoComplete="off"
          spellCheck={false}
          onChange={(event) => setQuery(event.target.value)}
        />
        <button type="submit" className="action">
          {t("searchSubmit")}
        </button>
      </form>
      {search ? (
        <dl className="facts">
          <div>
            <dt>{t("searchQueryLabel")}</dt>
            <dd>{search.query}</dd>
          </div>
          <div>
            <dt>classified_as</dt>
            <dd data-observation={search.observation.classified_as}>
              {observationClassLabel(search.observation.classified_as)}
            </dd>
          </div>
          <div>
            <dt>{t("searchSemanticEnabledLabel")}</dt>
            <dd>{search.semantic_enabled ? t("searchSemanticOn") : t("searchSemanticOff")}</dd>
          </div>
        </dl>
      ) : null}
      {search && search.hits.length > 0 ? (
        <ul className="search-list">
          {search.hits.map((hit: SearchHitDto) => (
            <li key={hit.identifier}>
              <button
                type="button"
                className="tree-item"
                onClick={() => onPreview(hit.identifier, search.query)}
              >
                {hit.identifier}
              </button>
              <span>
                {t("searchLexicalScore")}: {hit.lexical_score}
              </span>
              <span>
                {t("searchSemanticScore")}: {hit.semantic_score}
              </span>
            </li>
          ))}
        </ul>
      ) : null}
      {search?.next_cursor ? (
        <div className="search-actions">
          <button type="button" className="action" onClick={onLoadMore}>
            {t("searchLoadMore")}
          </button>
        </div>
      ) : null}
    </section>
  );
}

function unexpectedInspectorResponse(): WorkbenchError {
  return { category: "schema", message: t("unexpectedInspector") };
}

function asSearchInspector(
  response: Extract<IpcResponse, { kind: "search_inspector" }>,
): SearchInspectorDto {
  return {
    query: response.query,
    identifier: response.identifier,
    hits: response.hits,
    observation: response.observation,
    semantic_enabled: false,
    model_id: null,
    model_loaded: false,
    embedding_backend: "none",
    model_class: "unclassified",
    engine_search: false,
    files_written: false,
    scanned_user_obsidian_vault: false,
    scanned_user_basic_memory_home: false,
    semantic_disabled_reason: response.semantic_disabled_reason,
  };
}

async function runInspectSearch(
  query: string,
  identifier: string | null,
  setInspector: (inspector: SearchInspectorDto | null) => void,
  setInspectorError: (error: WorkbenchError | null) => void,
): Promise<void> {
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "inspect_search",
      args: {
        workspace: route.workspace,
        project: route.project,
        query,
        ...(identifier ? { identifier } : {}),
      },
    });
    switch (response.kind) {
      case "error":
        setInspector(null);
        setInspectorError({ category: response.category, message: response.message });
        return;
      case "search_inspector":
        setInspectorError(null);
        setInspector(asSearchInspector(response));
        return;
      case "capabilities":
      case "runtime_state":
      case "project_selected":
      case "project_catalog":
      case "preflight":
      case "config_discovery":
      case "tree_page":
      case "note_read":
      case "relation_list":
      case "graph_page":
      case "search_page":
      case "context_preview":
      case "activity_page":
      case "backup_catalog":
      case "fixture_restored":
      case "windows_runtime":
      case "draft_saved":
      case "draft_loaded":
      case "note_written":
      case "note_edited":
      case "note_moved":
      case "note_deleted":
      case "recall_benchmark":
      case "shutdown_begun":
        setInspector(null);
        setInspectorError(unexpectedInspectorResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setInspector(null);
    setInspectorError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

function SearchInspectorPanel({
  inspector,
  error,
  onInspect,
}: {
  inspector: SearchInspectorDto | null;
  error: WorkbenchError | null;
  onInspect: (query: string, identifier: string | null) => void;
}) {
  const [query, setQuery] = useState("");
  const [identifier, setIdentifier] = useState("");
  const empty = inspector === null || inspector.hits.length === 0;
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error
    ? t("inspectorErrorTitle")
    : empty
      ? t("inspectorEmptyTitle")
      : t("inspectorReadyTitle");
  return (
    <section
      className="subpanel"
      data-state={state}
      aria-labelledby="search-inspector-title"
      role={error ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h3 id="search-inspector-title">{heading}</h3>
      <p>
        {error
          ? `${errorCategoryLabel(error.category)}：${error.message}`
          : empty
            ? t("inspectorEmptyBody")
            : t("inspectorReadyBody")}
      </p>
      <p>{t("inspectorLexicalNotSemantic")}</p>
      <p>{t("inspectorModelUnavailable")}</p>
      <p>{t("inspectorReadOnly")}</p>
      <p>{t("inspectorNotT24")}</p>
      <form
        className="search-form"
        onSubmit={(event) => {
          event.preventDefault();
          const nextQuery = query.trim();
          if (nextQuery === "") {
            return;
          }
          const nextIdentifier = identifier.trim();
          onInspect(nextQuery, nextIdentifier === "" ? null : nextIdentifier);
        }}
      >
        <label htmlFor="inspector-query">{t("inspectorQueryLabel")}</label>
        <input
          id="inspector-query"
          type="text"
          value={query}
          autoComplete="off"
          spellCheck={false}
          onChange={(event) => setQuery(event.target.value)}
        />
        <label htmlFor="inspector-identifier">{t("inspectorIdentifierLabel")}</label>
        <input
          id="inspector-identifier"
          type="text"
          value={identifier}
          autoComplete="off"
          spellCheck={false}
          onChange={(event) => setIdentifier(event.target.value)}
        />
        <button type="submit" className="action">
          {t("inspectorSubmit")}
        </button>
      </form>
      {inspector ? (
        <dl className="facts">
          <div>
            <dt>{t("inspectorQueryLabel")}</dt>
            <dd>{inspector.query}</dd>
          </div>
          <div>
            <dt>{t("inspectorIdentifierLabel")}</dt>
            <dd>{inspector.identifier ?? t("runtimeNone")}</dd>
          </div>
          <div>
            <dt>classified_as</dt>
            <dd data-observation={inspector.observation.classified_as}>
              {observationClassLabel(inspector.observation.classified_as)}
            </dd>
          </div>
          <div>
            <dt>{t("searchSemanticEnabledLabel")}</dt>
            <dd>
              {inspector.semantic_enabled ? t("searchSemanticOn") : t("searchSemanticOff")}
            </dd>
          </div>
          <div>
            <dt>{t("inspectorModelLoadedLabel")}</dt>
            <dd>
              {inspector.model_loaded ? t("inspectorModelLoadedTrue") : t("inspectorModelLoadedFalse")}
            </dd>
          </div>
          <div>
            <dt>{t("inspectorModelIdLabel")}</dt>
            <dd>{inspector.model_id ?? t("inspectorModelNone")}</dd>
          </div>
          <div>
            <dt>{t("inspectorEmbeddingLabel")}</dt>
            <dd>{t("inspectorEmbeddingNone")}</dd>
          </div>
          <div>
            <dt>{t("inspectorModelClassLabel")}</dt>
            <dd>{t("inspectorModelUnclassified")}</dd>
          </div>
        </dl>
      ) : null}
      {inspector ? <p>{inspector.semantic_disabled_reason}</p> : null}
      {inspector && inspector.hits.length > 0 ? (
        <ul className="inspector-list">
          {inspector.hits.map((hit: SearchHitDto) => (
            <li key={hit.identifier}>
              <span>{hit.identifier}</span>
              <span>
                {t("searchLexicalScore")}: {hit.lexical_score}
              </span>
              <span>
                {t("searchSemanticScore")}: {hit.semantic_score}
              </span>
            </li>
          ))}
        </ul>
      ) : null}
    </section>
  );
}

function unexpectedRecallResponse(): WorkbenchError {
  return { category: "schema", message: t("unexpectedRecall") };
}

function asRecallBenchmark(
  response: Extract<IpcResponse, { kind: "recall_benchmark" }>,
): RecallBenchmarkDto {
  return {
    k: response.k,
    query_count: response.query_count,
    recall_hits: response.recall_hits,
    recall_relevant: response.recall_relevant,
    queries: response.queries,
    chinese_permalinks: response.chinese_permalinks,
    search_elapsed_ms: response.search_elapsed_ms,
    expand_elapsed_ms: response.expand_elapsed_ms,
    observation: response.observation,
    semantic_enabled: false,
    engine_search: false,
    native_gui: false,
    scanned_user_obsidian_vault: false,
    scanned_user_basic_memory_home: false,
    files_written: false,
  };
}

async function runRecallBenchmark(
  k: number | null,
  setRecall: (recall: RecallBenchmarkDto | null) => void,
  setRecallError: (error: WorkbenchError | null) => void,
): Promise<void> {
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "run_recall_benchmark",
      args: {
        workspace: route.workspace,
        project: route.project,
        ...(k === null ? {} : { k }),
      },
    });
    switch (response.kind) {
      case "error":
        setRecall(null);
        setRecallError({ category: response.category, message: response.message });
        return;
      case "recall_benchmark":
        setRecallError(null);
        setRecall(asRecallBenchmark(response));
        return;
      case "capabilities":
      case "runtime_state":
      case "project_selected":
      case "project_catalog":
      case "preflight":
      case "config_discovery":
      case "tree_page":
      case "note_read":
      case "relation_list":
      case "graph_page":
      case "search_page":
      case "search_inspector":
      case "context_preview":
      case "activity_page":
      case "backup_catalog":
      case "fixture_restored":
      case "windows_runtime":
      case "draft_saved":
      case "draft_loaded":
      case "note_written":
      case "note_edited":
      case "note_moved":
      case "note_deleted":
      case "shutdown_begun":
        setRecall(null);
        setRecallError(unexpectedRecallResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setRecall(null);
    setRecallError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

function RecallBenchmarkPanel({
  recall,
  error,
  onRun,
}: {
  recall: RecallBenchmarkDto | null;
  error: WorkbenchError | null;
  onRun: (k: number | null) => void;
}) {
  const [k, setK] = useState("");
  const empty = recall === null || recall.query_count === 0 || recall.recall_relevant === 0;
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error
    ? t("recallErrorTitle")
    : empty
      ? t("recallEmptyTitle")
      : t("recallReadyTitle");
  return (
    <section
      className="subpanel"
      data-state={state}
      aria-labelledby="recall-benchmark-title"
      role={error ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h3 id="recall-benchmark-title">{heading}</h3>
      <p>
        {error
          ? `${errorCategoryLabel(error.category)}：${error.message}`
          : empty
            ? t("recallEmptyBody")
            : t("recallReadyBody")}
      </p>
      <p>{t("recallDiskGold")}</p>
      <p>{t("recallChineseKept")}</p>
      <p>{t("recallNativeUnverified")}</p>
      <form
        className="search-form"
        onSubmit={(event) => {
          event.preventDefault();
          const raw = k.trim();
          if (raw === "") {
            onRun(null);
            return;
          }
          const parsed = Number.parseInt(raw, 10);
          if (!Number.isFinite(parsed)) {
            return;
          }
          onRun(parsed);
        }}
      >
        <label htmlFor="recall-k">{t("recallKLabel")}</label>
        <input
          id="recall-k"
          type="text"
          inputMode="numeric"
          value={k}
          autoComplete="off"
          spellCheck={false}
          onChange={(event) => setK(event.target.value)}
        />
        <button type="submit" className="action">
          {t("recallSubmit")}
        </button>
      </form>
      {recall ? (
        <dl className="facts">
          <div>
            <dt>{t("recallAtKLabel")}</dt>
            <dd>
              {recall.recall_hits}/{recall.recall_relevant} @{recall.k}
            </dd>
          </div>
          <div>
            <dt>{t("recallQueryCountLabel")}</dt>
            <dd>{recall.query_count}</dd>
          </div>
          <div>
            <dt>{t("recallSearchElapsed")}</dt>
            <dd>{recall.search_elapsed_ms}</dd>
          </div>
          <div>
            <dt>{t("recallExpandElapsed")}</dt>
            <dd>{recall.expand_elapsed_ms}</dd>
          </div>
          <div>
            <dt>classified_as</dt>
            <dd data-observation={recall.observation.classified_as}>
              {observationClassLabel(recall.observation.classified_as)}
            </dd>
          </div>
          <div>
            <dt>{t("searchSemanticEnabledLabel")}</dt>
            <dd>
              {recall.semantic_enabled ? t("searchSemanticOn") : t("searchSemanticOff")}
            </dd>
          </div>
          <div>
            <dt>{t("recallNativeGuiLabel")}</dt>
            <dd>{t("recallNativeGuiFalse")}</dd>
          </div>
        </dl>
      ) : null}
      {recall && recall.queries.length > 0 ? (
        <ul className="benchmark-list">
          {recall.queries.map((query) => (
            <li key={query.query}>
              <span>{query.query}</span>
              <span>
                {t("recallAtKLabel")}: {query.retrieved_relevant}/{query.relevant_count}
              </span>
            </li>
          ))}
        </ul>
      ) : null}
      {recall && recall.chinese_permalinks.length > 0 ? (
        <p>
          {t("recallChinesePermalinks")}: {recall.chinese_permalinks.join("、")}
        </p>
      ) : null}
    </section>
  );
}

function unexpectedPreviewResponse(): WorkbenchError {
  return { category: "schema", message: t("unexpectedPreview") };
}

function asContextPreview(
  response: Extract<IpcResponse, { kind: "context_preview" }>,
): ContextPreviewDto {
  return {
    identifier: response.identifier,
    query: response.query,
    snippet: response.snippet,
    executed: false,
    unsafe_html_present: response.unsafe_html_present,
    observation: response.observation,
    engine_context: false,
    scanned_user_obsidian_vault: false,
    scanned_user_basic_memory_home: false,
    files_written: response.files_written,
  };
}

async function loadContextPreview(
  identifier: string,
  query: string | null,
  setPreview: (preview: ContextPreviewDto | null) => void,
  setPreviewError: (error: WorkbenchError | null) => void,
): Promise<void> {
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "preview_context",
      args: {
        workspace: route.workspace,
        project: route.project,
        identifier,
        ...(query ? { query } : {}),
      },
    });
    switch (response.kind) {
      case "error":
        setPreview(null);
        setPreviewError({ category: response.category, message: response.message });
        return;
      case "context_preview":
        setPreviewError(null);
        setPreview(asContextPreview(response));
        return;
      case "capabilities":
      case "runtime_state":
      case "project_selected":
      case "project_catalog":
      case "preflight":
      case "config_discovery":
      case "tree_page":
      case "note_read":
      case "relation_list":
      case "graph_page":
      case "search_page":
      case "activity_page":
      case "backup_catalog":
      case "fixture_restored":
      case "windows_runtime":
      case "draft_saved":
      case "draft_loaded":
      case "note_written":
      case "note_edited":
      case "note_moved":
      case "note_deleted":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
        setPreview(null);
        setPreviewError(unexpectedPreviewResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setPreview(null);
    setPreviewError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

function ContextPreviewPanel({
  preview,
  error,
}: {
  preview: ContextPreviewDto | null;
  error: WorkbenchError | null;
}) {
  const empty = preview === null || preview.snippet === "";
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error
    ? t("previewErrorTitle")
    : empty
      ? t("previewEmptyTitle")
      : t("previewReadyTitle");
  return (
    <section
      className="subpanel"
      data-state={state}
      aria-labelledby="context-preview-title"
      role={error ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h3 id="context-preview-title">{heading}</h3>
      <p>
        {error
          ? `${errorCategoryLabel(error.category)}：${error.message}`
          : empty
            ? t("previewEmptyBody")
            : t("previewReadyBody")}
      </p>
      <p>{t("previewNotEngineContext")}</p>
      <p>{t("previewHtmlIsText")}</p>
      {preview ? (
        <dl className="facts">
          <div>
            <dt>{t("workbenchIdentifierLabel")}</dt>
            <dd>{preview.identifier}</dd>
          </div>
          <div>
            <dt>classified_as</dt>
            <dd data-observation={preview.observation.classified_as}>
              {observationClassLabel(preview.observation.classified_as)}
            </dd>
          </div>
          <div>
            <dt>{t("contentSafetyExecutedLabel")}</dt>
            <dd data-executed="false">{t("contentSafetyExecutedNo")}</dd>
          </div>
        </dl>
      ) : null}
      {preview && preview.snippet !== "" ? (
        <pre
          className="note-body"
          data-preview="text"
          data-executed="false"
          aria-labelledby="context-preview-title"
        >
          {preview.snippet}
        </pre>
      ) : null}
    </section>
  );
}

function unexpectedActivityResponse(): WorkbenchError {
  return { category: "schema", message: t("unexpectedActivity") };
}

function asActivityPage(response: Extract<IpcResponse, { kind: "activity_page" }>): ActivityPageDto {
  return {
    entries: response.entries,
    next_cursor: response.next_cursor,
    page: response.page,
    truncated: response.truncated,
    observation: response.observation,
    engine_activity: false,
    scanned_user_obsidian_vault: false,
    scanned_user_basic_memory_home: false,
    files_written: response.files_written,
  };
}

function mergeActivityPage(current: ActivityPageDto, next: ActivityPageDto): ActivityPageDto {
  const entries = [...current.entries];
  for (const entry of next.entries) {
    if (!entries.some((existing) => existing.identifier === entry.identifier)) {
      entries.push(entry);
    }
  }
  return {
    ...next,
    entries,
  };
}

async function loadActivity(
  setActivity: (activity: ActivityPageDto | null) => void,
  setActivityError: (error: WorkbenchError | null) => void,
): Promise<void> {
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "list_activity",
      args: {
        workspace: route.workspace,
        project: route.project,
        page_size: TREE_PAGE_SIZE,
      },
    });
    switch (response.kind) {
      case "error":
        setActivity(null);
        setActivityError({ category: response.category, message: response.message });
        return;
      case "activity_page":
        if (response.truncated) {
          setActivity(null);
          setActivityError(unexpectedActivityResponse());
          return;
        }
        setActivityError(null);
        setActivity(asActivityPage(response));
        return;
      case "capabilities":
      case "runtime_state":
      case "project_selected":
      case "project_catalog":
      case "preflight":
      case "config_discovery":
      case "tree_page":
      case "note_read":
      case "relation_list":
      case "graph_page":
      case "search_page":
      case "context_preview":
      case "backup_catalog":
      case "fixture_restored":
      case "windows_runtime":
      case "draft_saved":
      case "draft_loaded":
      case "note_written":
      case "note_edited":
      case "note_moved":
      case "note_deleted":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
        setActivity(null);
        setActivityError(unexpectedActivityResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setActivity(null);
    setActivityError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

async function loadMoreActivity(
  current: ActivityPageDto,
  setActivity: (activity: ActivityPageDto | null) => void,
  setActivityError: (error: WorkbenchError | null) => void,
): Promise<void> {
  if (!current.next_cursor) {
    return;
  }
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "list_activity",
      args: {
        workspace: route.workspace,
        project: route.project,
        cursor: current.next_cursor,
        page_size: TREE_PAGE_SIZE,
      },
    });
    switch (response.kind) {
      case "error":
        setActivityError({ category: response.category, message: response.message });
        return;
      case "activity_page":
        if (response.truncated) {
          setActivityError(unexpectedActivityResponse());
          return;
        }
        setActivityError(null);
        setActivity(mergeActivityPage(current, asActivityPage(response)));
        return;
      case "capabilities":
      case "runtime_state":
      case "project_selected":
      case "project_catalog":
      case "preflight":
      case "config_discovery":
      case "tree_page":
      case "note_read":
      case "relation_list":
      case "graph_page":
      case "search_page":
      case "context_preview":
      case "backup_catalog":
      case "fixture_restored":
      case "windows_runtime":
      case "draft_saved":
      case "draft_loaded":
      case "note_written":
      case "note_edited":
      case "note_moved":
      case "note_deleted":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
        setActivityError(unexpectedActivityResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setActivityError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

function ActivityPanel({
  activity,
  error,
  onLoadMore,
}: {
  activity: ActivityPageDto | null;
  error: WorkbenchError | null;
  onLoadMore: () => void;
}) {
  const empty = activity === null || activity.entries.length === 0;
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error
    ? t("activityErrorTitle")
    : empty
      ? t("activityEmptyTitle")
      : t("activityReadyTitle");
  return (
    <section
      className="subpanel"
      data-state={state}
      aria-labelledby="activity-title"
      role={error ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h3 id="activity-title">{heading}</h3>
      <p>
        {error
          ? `${errorCategoryLabel(error.category)}：${error.message}`
          : empty
            ? t("activityEmptyBody")
            : t("activityReadyBody")}
      </p>
      <p>{t("activityPermalinkNotPath")}</p>
      <p>{t("activityMcpUnverified")}</p>
      {activity && activity.entries.length > 0 ? (
        <ul className="activity-list">
          {activity.entries.map((entry: ActivityEntryDto) => (
            <li key={entry.identifier}>
              <span>{entry.identifier}</span>
              <span>
                {t("activityMtimeLabel")}: {entry.observed_mtime}
              </span>
            </li>
          ))}
        </ul>
      ) : null}
      {activity?.next_cursor ? (
        <div className="activity-actions">
          <button type="button" className="action" onClick={onLoadMore}>
            {t("activityLoadMore")}
          </button>
        </div>
      ) : null}
    </section>
  );
}

function isFixtureNoteIdentifier(value: string): boolean {
  const identifier = value.trim();
  if (identifier === "") {
    return false;
  }
  return !(
    identifier.includes("\\") ||
    identifier.includes("..") ||
    identifier.includes("%") ||
    identifier.includes(":") ||
    identifier.startsWith("/") ||
    identifier.includes(".obsidian") ||
    identifier.includes(".basic-memory")
  );
}

function unexpectedCrudResponse(): WorkbenchError {
  return { category: "schema", message: t("unexpectedCrud") };
}

function crudObservationLabel(classified: NoteCrudClass): string {
  switch (classified) {
    case "disk_verified":
      return t("crudObservationDisk");
    case "accepted_unverified":
      return t("crudObservationUnverified");
    case "conflict":
      return t("crudObservationConflict");
    case "empty":
      return t("crudObservationEmpty");
    case "unclassified":
      return t("crudObservationUnclassified");
    default: {
      const exhaustive: never = classified;
      return exhaustive;
    }
  }
}

type NoteCrudResult =
  | ({ kind: "note_written" } & NoteWriteDto)
  | ({ kind: "note_edited" } & NoteEditDto)
  | ({ kind: "note_moved" } & NoteMoveDto)
  | ({ kind: "note_deleted" } & NoteDeleteDto);

function NoteCrudPanel({
  seedIdentifier,
  seedTitle,
  seedBody,
  runtimeFailure,
  onMutated,
  onObservation,
}: {
  seedIdentifier: string | null;
  seedTitle: string | null;
  seedBody: string | null;
  runtimeFailure: FailureKind | null;
  onMutated: () => void;
  onObservation: (identifier: string, classified_as: NoteCrudClass) => void;
}) {
  const [identifier, setIdentifier] = useState("");
  const [title, setTitle] = useState("");
  const [body, setBody] = useState("");
  const [destination, setDestination] = useState("");
  const [confirmDelete, setConfirmDelete] = useState(false);
  const [result, setResult] = useState<NoteCrudResult | null>(null);
  const [error, setError] = useState<WorkbenchError | null>(null);

  const recordResult = (next: NoteCrudResult | null) => {
    setResult(next);
    if (next) {
      const target = next.kind === "note_moved" ? next.destination : next.identifier;
      onObservation(target, next.observation.classified_as);
    }
  };

  useEffect(() => {
    if (!seedIdentifier) {
      return;
    }
    setIdentifier(seedIdentifier);
    setTitle(seedTitle ?? "");
    setBody(seedBody ?? "");
    setDestination("");
    setConfirmDelete(false);
    setResult(null);
    setError(null);
  }, [seedIdentifier, seedTitle, seedBody]);

  const empty = identifier.trim() === "" && body === "" && result === null && error === null;
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error ? t("crudErrorTitle") : empty ? t("crudEmptyTitle") : t("crudReadyTitle");
  const fixtureOk = isFixtureNoteIdentifier(identifier);
  const destinationOk = destination.trim() === "" || isFixtureNoteIdentifier(destination);

  return (
    <section className="subpanel" data-state={state} aria-labelledby="note-crud-title" role={error ? "alert" : undefined}>
      <p className="state-badge">{badge}</p>
      <h3 id="note-crud-title">{heading}</h3>
      <p>{error ? `${errorCategoryLabel(error.category)}：${error.message}` : empty ? t("crudEmptyBody") : t("crudReadyBody")}</p>
      <p>{t("crudCoordinationTitle")}</p>
      <ul className="coordination-legend">
        <li>{t("crudObservationConflict")}</li>
        <li>{t("failureTimeoutUnknown")}</li>
        <li>{t("crudObservationDisk")}</li>
        <li>{t("crudObservationUnverified")}</li>
      </ul>
      <dl className="facts">
        <div>
          <dt>{t("crudTimeoutUnknownLabel")}</dt>
          <dd>{runtimeFailure === "timeout_unknown" ? t("failureTimeoutUnknown") : t("runtimeNone")}</dd>
        </div>
        <div>
          <dt>{t("crudRecoveryNoteLabel")}</dt>
          <dd>{t("crudRecoveryNotT17")}</dd>
        </div>
      </dl>
      <div className="crud-editor">
        <label htmlFor="crud-identifier">{t("crudIdentifierLabel")}</label>
        <input
          id="crud-identifier"
          type="text"
          value={identifier}
          autoComplete="off"
          spellCheck={false}
          onChange={(event) => {
            setIdentifier(event.target.value);
            setConfirmDelete(false);
            setError(null);
          }}
        />
        <label htmlFor="crud-title">{t("crudTitleLabel")}</label>
        <input
          id="crud-title"
          type="text"
          value={title}
          autoComplete="off"
          spellCheck={false}
          onChange={(event) => {
            setTitle(event.target.value);
            setError(null);
          }}
        />
        <label htmlFor="crud-body">{t("crudBodyLabel")}</label>
        <textarea
          id="crud-body"
          className="crud-body"
          value={body}
          spellCheck={false}
          onChange={(event) => {
            setBody(event.target.value);
            setError(null);
          }}
        />
        <ContentSafetyFacts body={body} previewId="crud-preview-text" />
        <label htmlFor="crud-destination">{t("crudDestinationLabel")}</label>
        <input
          id="crud-destination"
          type="text"
          value={destination}
          autoComplete="off"
          spellCheck={false}
          onChange={(event) => {
            setDestination(event.target.value);
            setError(null);
          }}
        />
      </div>
      <dl className="facts">
        <div>
          <dt>{t("crudEnginePersistedLabel")}</dt>
          <dd>{result?.engine_persisted ? t("crudEngineYes") : t("crudEngineNo")}</dd>
        </div>
        <div>
          <dt>{t("crudFilesWrittenLabel")}</dt>
          <dd>{result?.files_written ? t("crudWroteFiles") : t("crudNoWrite")}</dd>
        </div>
      </dl>
      {result ? (
        <p data-observation={result.observation.classified_as}>
          {crudObservationLabel(result.observation.classified_as)}
        </p>
      ) : null}
      {!fixtureOk && identifier.trim() !== "" ? <p>{t("crudFixtureOnly")}</p> : null}
      {!destinationOk ? <p>{t("crudFixtureOnly")}</p> : null}
      <button
        type="button"
        className="action"
        disabled={!fixtureOk || title.trim() === ""}
        onClick={() => {
          void runWriteNote(identifier, title, body, recordResult, setError, onMutated);
        }}
      >
        {t("crudWrite")}
      </button>
      <button
        type="button"
        className="action"
        disabled={!fixtureOk}
        onClick={() => {
          void runEditNote(identifier, body, recordResult, setError, onMutated);
        }}
      >
        {t("crudEdit")}
      </button>
      <button
        type="button"
        className="action"
        disabled={!fixtureOk || !isFixtureNoteIdentifier(destination)}
        onClick={() => {
          void runMoveNote(identifier, destination, recordResult, setIdentifier, setError, onMutated);
        }}
      >
        {t("crudMove")}
      </button>
      {confirmDelete ? (
        <>
          <p>{t("crudDeleteConfirmBody")}</p>
          <button
            type="button"
            className="action"
            onClick={() => {
              setConfirmDelete(false);
              void runDeleteNote(identifier, recordResult, setError, onMutated);
            }}
          >
            {t("crudDeleteConfirm")}
          </button>
          <button
            type="button"
            className="action"
            onClick={() => setConfirmDelete(false)}
          >
            {t("crudDeleteCancel")}
          </button>
        </>
      ) : (
        <button
          type="button"
          className="action"
          disabled={!fixtureOk}
          onClick={() => setConfirmDelete(true)}
        >
          {t("crudDelete")}
        </button>
      )}
    </section>
  );
}

async function applyCrudResponse(
  response: IpcResponse,
  setResult: (result: NoteCrudResult | null) => void,
  setError: (error: WorkbenchError | null) => void,
  onMutated: () => void,
  onMoved?: (destination: string) => void,
): Promise<void> {
  switch (response.kind) {
    case "error":
      setError({ category: response.category, message: response.message });
      return;
    case "note_written":
      setError(null);
      setResult({
        kind: "note_written",
        identifier: response.identifier,
        title: response.title,
        body: response.body,
        files_written: response.files_written,
        engine_persisted: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        observation: response.observation,
      });
      if (response.observation.disk_verified) {
        onMutated();
      }
      return;
    case "note_edited":
      setError(null);
      setResult({
        kind: "note_edited",
        identifier: response.identifier,
        body: response.body,
        files_written: response.files_written,
        engine_persisted: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        observation: response.observation,
      });
      if (response.observation.disk_verified) {
        onMutated();
      }
      return;
    case "note_moved":
      setError(null);
      setResult({
        kind: "note_moved",
        identifier: response.identifier,
        destination: response.destination,
        body: response.body,
        files_written: response.files_written,
        engine_persisted: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        observation: response.observation,
      });
      if (response.observation.disk_verified) {
        onMoved?.(response.destination);
        onMutated();
      }
      return;
    case "note_deleted":
      setError(null);
      setResult({
        kind: "note_deleted",
        identifier: response.identifier,
        files_written: response.files_written,
        engine_persisted: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        observation: response.observation,
      });
      if (response.observation.disk_verified) {
        onMutated();
      }
      return;
    case "capabilities":
    case "runtime_state":
    case "project_selected":
    case "project_catalog":
    case "preflight":
    case "config_discovery":
    case "tree_page":
    case "note_read":
    case "backup_catalog":
    case "fixture_restored":
    case "windows_runtime":
    case "draft_saved":
    case "draft_loaded":
    case "relation_list":
      case "graph_page":
      case "search_page":
      case "context_preview":
      case "activity_page":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
      setError(unexpectedCrudResponse());
      return;
    default: {
      const exhaustive: never = response;
      return exhaustive;
    }
  }
}

async function runWriteNote(
  identifier: string,
  title: string,
  body: string,
  setResult: (result: NoteCrudResult | null) => void,
  setError: (error: WorkbenchError | null) => void,
  onMutated: () => void,
): Promise<void> {
  if (!isFixtureNoteIdentifier(identifier)) {
    setError({ category: "policy", message: t("crudFixtureOnly") });
    return;
  }
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "write_note",
      args: {
        workspace: route.workspace,
        project: route.project,
        identifier,
        title,
        body,
      },
    });
    await applyCrudResponse(response, setResult, setError, onMutated);
  } catch (cause) {
    setError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

async function runEditNote(
  identifier: string,
  body: string,
  setResult: (result: NoteCrudResult | null) => void,
  setError: (error: WorkbenchError | null) => void,
  onMutated: () => void,
): Promise<void> {
  if (!isFixtureNoteIdentifier(identifier)) {
    setError({ category: "policy", message: t("crudFixtureOnly") });
    return;
  }
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "edit_note",
      args: {
        workspace: route.workspace,
        project: route.project,
        identifier,
        body,
      },
    });
    await applyCrudResponse(response, setResult, setError, onMutated);
  } catch (cause) {
    setError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

async function runMoveNote(
  identifier: string,
  destination: string,
  setResult: (result: NoteCrudResult | null) => void,
  setIdentifier: (identifier: string) => void,
  setError: (error: WorkbenchError | null) => void,
  onMutated: () => void,
): Promise<void> {
  if (!isFixtureNoteIdentifier(identifier) || !isFixtureNoteIdentifier(destination)) {
    setError({ category: "policy", message: t("crudFixtureOnly") });
    return;
  }
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "move_note",
      args: {
        workspace: route.workspace,
        project: route.project,
        identifier,
        destination,
      },
    });
    await applyCrudResponse(response, setResult, setError, onMutated, setIdentifier);
  } catch (cause) {
    setError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

async function runDeleteNote(
  identifier: string,
  setResult: (result: NoteCrudResult | null) => void,
  setError: (error: WorkbenchError | null) => void,
  onMutated: () => void,
): Promise<void> {
  if (!isFixtureNoteIdentifier(identifier)) {
    setError({ category: "policy", message: t("crudFixtureOnly") });
    return;
  }
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "delete_note",
      args: {
        workspace: route.workspace,
        project: route.project,
        identifier,
      },
    });
    await applyCrudResponse(response, setResult, setError, onMutated);
  } catch (cause) {
    setError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

function unexpectedDraftResponse(): WorkbenchError {
  return { category: "schema", message: t("unexpectedDraft") };
}

function draftObservationLabel(result: DraftResultDto): string {
  switch (result.observation.classified_as) {
    case "disk_verified":
      return t("draftObservationDisk");
    case "accepted_unverified":
      return t("draftObservationUnverified");
    case "empty":
      return t("draftObservationEmpty");
    case "unclassified":
      return t("draftObservationUnclassified");
    default: {
      const exhaustive: never = result.observation.classified_as;
      return exhaustive;
    }
  }
}

function sessionPersistenceLabel(dirty: boolean, result: DraftResultDto | null): string {
  if (dirty) {
    return t("draftSessionDirty");
  }
  if (result?.observation.disk_verified) {
    return t("draftSessionDisk");
  }
  return t("draftSessionEmpty");
}

function DraftEditor({
  seedIdentifier,
  seedBody,
}: {
  seedIdentifier: string | null;
  seedBody: string | null;
}) {
  const [identifier, setIdentifier] = useState("");
  const [body, setBody] = useState("");
  const [diskBody, setDiskBody] = useState<string | null>(null);
  const [result, setResult] = useState<DraftResultDto | null>(null);
  const [error, setError] = useState<WorkbenchError | null>(null);

  useEffect(() => {
    if (!seedIdentifier) {
      return;
    }
    setIdentifier(seedIdentifier);
    setBody(seedBody ?? "");
    setDiskBody(null);
    setResult(null);
    setError(null);
  }, [seedIdentifier, seedBody]);

  const dirty = diskBody === null || body !== diskBody;
  const empty = identifier.trim() === "" && body === "" && result === null && error === null;
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const title = error ? t("draftErrorTitle") : empty ? t("draftEmptyTitle") : t("draftReadyTitle");

  return (
    <section className="subpanel" data-state={state} aria-labelledby="draft-editor-title" role={error ? "alert" : undefined}>
      <p className="state-badge">{badge}</p>
      <h3 id="draft-editor-title">{title}</h3>
      <p>{error ? `${errorCategoryLabel(error.category)}：${error.message}` : empty ? t("draftEmptyBody") : t("draftReadyBody")}</p>
      <div className="draft-editor">
        <label htmlFor="draft-identifier">{t("draftIdentifierLabel")}</label>
        <input
          id="draft-identifier"
          type="text"
          value={identifier}
          autoComplete="off"
          spellCheck={false}
          onChange={(event) => {
            setIdentifier(event.target.value);
            setError(null);
          }}
        />
        <label htmlFor="draft-body">{t("draftBodyLabel")}</label>
        <textarea
          id="draft-body"
          className="draft-body"
          value={body}
          spellCheck={false}
          onChange={(event) => {
            setBody(event.target.value);
            setError(null);
          }}
        />
        <ContentSafetyFacts body={body} previewId="draft-preview-text" />
      </div>
      <dl className="facts">
        <div>
          <dt>{t("draftSessionLabel")}</dt>
          <dd>{sessionPersistenceLabel(dirty, result)}</dd>
        </div>
        <div>
          <dt>{t("draftEnginePersistedLabel")}</dt>
          <dd>{result?.engine_persisted ? t("draftEngineYes") : t("draftEngineNo")}</dd>
        </div>
        <div>
          <dt>{t("draftFilesWrittenLabel")}</dt>
          <dd>{result?.files_written ? t("draftWroteFiles") : t("draftNoWrite")}</dd>
        </div>
      </dl>
      {result ? <p>{draftObservationLabel(result)}</p> : null}
      <button
        type="button"
        className="action"
        disabled={identifier.trim() === ""}
        onClick={() => {
          void persistDraft(identifier, body, setResult, setDiskBody, setBody, setError);
        }}
      >
        {t("draftSave")}
      </button>
      <button
        type="button"
        className="action"
        disabled={identifier.trim() === ""}
        onClick={() => {
          void reloadDraft(identifier, setResult, setDiskBody, setBody, setError);
        }}
      >
        {t("draftReload")}
      </button>
    </section>
  );
}

async function persistDraft(
  identifier: string,
  body: string,
  setResult: (result: DraftResultDto | null) => void,
  setDiskBody: (body: string | null) => void,
  setBody: (body: string) => void,
  setError: (error: WorkbenchError | null) => void,
): Promise<void> {
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "save_draft",
      args: {
        workspace: route.workspace,
        project: route.project,
        identifier,
        body,
      },
    });
    switch (response.kind) {
      case "error":
        setError({ category: response.category, message: response.message });
        return;
      case "draft_saved":
        setError(null);
        setResult({
          identifier: response.identifier,
          body: response.body,
          files_written: response.files_written,
          engine_persisted: false,
          scanned_user_obsidian_vault: false,
          scanned_user_basic_memory_home: false,
          observation: response.observation,
        });
        if (response.observation.disk_verified && !response.engine_persisted) {
          setDiskBody(response.body);
          setBody(response.body);
        }
        return;
      case "capabilities":
      case "runtime_state":
      case "project_selected":
      case "project_catalog":
      case "preflight":
      case "config_discovery":
      case "tree_page":
      case "note_read":
      case "backup_catalog":
      case "fixture_restored":
      case "windows_runtime":
      case "draft_loaded":
      case "note_written":
      case "note_edited":
      case "note_moved":
      case "note_deleted":
      case "relation_list":
      case "graph_page":
      case "search_page":
      case "context_preview":
      case "activity_page":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
        setError(unexpectedDraftResponse());
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
  }
}

async function reloadDraft(
  identifier: string,
  setResult: (result: DraftResultDto | null) => void,
  setDiskBody: (body: string | null) => void,
  setBody: (body: string) => void,
  setError: (error: WorkbenchError | null) => void,
): Promise<void> {
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "load_draft",
      args: {
        workspace: route.workspace,
        project: route.project,
        identifier,
      },
    });
    switch (response.kind) {
      case "error":
        setError({ category: response.category, message: response.message });
        return;
      case "draft_loaded":
        setError(null);
        setResult({
          identifier: response.identifier,
          body: response.body,
          files_written: response.files_written,
          engine_persisted: false,
          scanned_user_obsidian_vault: false,
          scanned_user_basic_memory_home: false,
          observation: response.observation,
        });
        setBody(response.body);
        if (response.observation.disk_verified && !response.engine_persisted) {
          setDiskBody(response.body);
        } else {
          setDiskBody(null);
        }
        return;
      case "capabilities":
      case "runtime_state":
      case "project_selected":
      case "project_catalog":
      case "preflight":
      case "config_discovery":
      case "tree_page":
      case "note_read":
      case "backup_catalog":
      case "fixture_restored":
      case "windows_runtime":
      case "draft_saved":
      case "note_written":
      case "note_edited":
      case "note_moved":
      case "note_deleted":
      case "relation_list":
      case "graph_page":
      case "search_page":
      case "context_preview":
      case "activity_page":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
        setError(unexpectedDraftResponse());
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
          onRefresh={onRefresh}
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
  onRefresh,
}: {
  capabilities: CapabilitiesDto;
  runtime: RuntimeStateDto;
  refresh: ReactNode;
  onRefresh: () => void;
}) {
  const [drainResult, setDrainResult] = useState<DrainResultDto | null>(null);
  const [drainError, setDrainError] = useState<{
    category: "policy" | "schema" | "unsupported" | "invoke";
    message: string;
  } | null>(null);
  const empty = runtime.status === "not_started" && runtime.host_drain === "idle";
  const failed = runtime.status === "failed";
  const state = drainError ? "error" : failed ? "error" : empty ? "empty" : "status";
  const badge = drainError || failed ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const title = drainError
    ? t("runtimeErrorTitle")
    : failed
      ? t("runtimeFailedTitle")
      : empty
        ? t("runtimeEmptyTitle")
        : t("runtimeTitle");
  const description = empty ? t("runtimeEmptyBody") : null;
  const hostDrain = drainResult?.host_drain ?? runtime.host_drain;
  const shutdown = drainResult?.shutdown ?? runtime.shutdown;
  const timeoutUnknown =
    runtime.failure === "timeout_unknown" || shutdown?.timeout_unknown === true;

  return (
    <section
      className="panel"
      data-state={state}
      data-drain={hostDrain}
      aria-labelledby="runtime-title"
      role={failed || drainError ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h2 id="runtime-title">{title}</h2>
      {description ? <p>{description}</p> : null}
      {drainError ? (
        <p>
          {errorCategoryLabel(drainError.category)}：{drainError.message}
        </p>
      ) : null}
      <p>{t("runtimeDrainLegendTitle")}</p>
      <ul className="drain-legend">
        <li>{t("runtimeDrainIdleLabel")}</li>
        <li>{t("runtimeDrainDrainingLabel")}</li>
        <li>{t("runtimeDrainTimeoutUnknownLabel")}</li>
        <li>{t("runtimeDrainConflictLabel")}</li>
      </ul>
      <dl className="facts">
        <div>
          <dt>{t("runtimeStatusLabel")}</dt>
          <dd>{runtimeStatusLabel(runtime.status)}</dd>
        </div>
        <div>
          <dt>{t("runtimeDrainLabel")}</dt>
          <dd>{drainPhaseLabel(hostDrain)}</dd>
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
        <div>
          <dt>{t("runtimeDrainTimeoutUnknownLabel")}</dt>
          <dd>{timeoutUnknown ? t("failureTimeoutUnknown") : t("runtimeNone")}</dd>
        </div>
        <div>
          <dt>{t("runtimeEngineSpawnedLabel")}</dt>
          <dd>{drainResult?.engine_spawned ? t("runtimeYes") : t("runtimeNo")}</dd>
        </div>
        <div>
          <dt>{t("runtimeSemanticModelLoadedLabel")}</dt>
          <dd>{runtime.semantic_model_loaded ? t("runtimeYes") : t("runtimeNo")}</dd>
        </div>
        <div>
          <dt>{t("runtimeChildKilledLabel")}</dt>
          <dd>{drainResult?.child_killed ? t("runtimeYes") : t("runtimeNo")}</dd>
        </div>
        <div>
          <dt>{t("runtimeShutdownForcedLabel")}</dt>
          <dd>{shutdown?.forced ? t("runtimeYes") : t("runtimeNo")}</dd>
        </div>
        <div>
          <dt>{t("runtimeShutdownChildExitedLabel")}</dt>
          <dd>{shutdown?.child_exited ? t("runtimeYes") : t("runtimeNo")}</dd>
        </div>
        <div>
          <dt>{t("runtimeShutdownTransportLabel")}</dt>
          <dd>{shutdown?.transport_cancelled ? t("runtimeYes") : t("runtimeNo")}</dd>
        </div>
        <div>
          <dt>{t("runtimeDrainUnknownKeysLabel")}</dt>
          <dd>
            {drainResult && drainResult.inflight_unknown.length > 0
              ? drainResult.inflight_unknown.join("、")
              : t("runtimeNone")}
          </dd>
        </div>
      </dl>
      <p>{t("runtimeDrainNotRestore")}</p>
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
      <button
        type="button"
        className="action"
        onClick={() => {
          void runBeginShutdown(setDrainResult, setDrainError, onRefresh);
        }}
      >
        {t("runtimeDrainStart")}
      </button>
      {refresh}
    </section>
  );
}

async function runBeginShutdown(
  setResult: (next: DrainResultDto) => void,
  setError: (
    next: { category: "policy" | "schema" | "unsupported" | "invoke"; message: string } | null,
  ) => void,
  onRefresh: () => void,
): Promise<void> {
  setError(null);
  try {
    const response = await invokeTyped({
      command: "begin_shutdown",
      args: {},
    });
    if (response.kind === "error") {
      setError({ category: response.category, message: response.message });
      return;
    }
    if (response.kind !== "shutdown_begun") {
      setError({ category: "schema", message: t("unexpectedDrain") });
      return;
    }
    setResult(response);
    onRefresh();
  } catch (cause) {
    setError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
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
                case "backup_catalog":
                case "fixture_restored":
                case "windows_runtime":
                case "draft_saved":
                case "draft_loaded":
                case "note_written":
                case "note_edited":
                case "note_moved":
                case "note_deleted":
                case "relation_list":
      case "graph_page":
      case "search_page":
      case "context_preview":
      case "activity_page":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
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

type BackupError = {
  category: "policy" | "schema" | "unsupported" | "invoke";
  message: string;
};

function isFixtureBackupId(id: string): boolean {
  if (id.trim() === "" || !id.startsWith("fixture-")) {
    return false;
  }
  return !(
    id.includes("\\") ||
    id.includes("/") ||
    id.includes("..") ||
    id.includes("%") ||
    id.includes(":") ||
    id.includes(".obsidian") ||
    id.includes(".basic-memory")
  );
}

function unexpectedBackupResponse(): BackupError {
  return { category: "schema", message: t("unexpectedBackups") };
}

function BackupPanel() {
  const [reloadToken, setReloadToken] = useState(0);
  const [phase, setPhase] = useState<"loading" | "ready" | "empty" | "error">("loading");
  const [catalog, setCatalog] = useState<BackupCatalogDto | null>(null);
  const [restoreResult, setRestoreResult] = useState<RestoreResultDto | null>(null);
  const [error, setError] = useState<BackupError | null>(null);

  useEffect(() => {
    let cancelled = false;
    setPhase("loading");
    setCatalog(null);
    setRestoreResult(null);
    setError(null);
    void (async () => {
      try {
        const route = copyFixtureRoute();
        const response = await invokeTyped<IpcResponse>({
          command: "list_backups",
          args: {
            workspace: route.workspace,
            project: route.project,
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
          case "backup_catalog":
            setCatalog({
              backups: response.backups,
              scanned_user_obsidian_vault: response.scanned_user_obsidian_vault,
              scanned_user_basic_memory_home: response.scanned_user_basic_memory_home,
              cloud_or_credential_required: response.cloud_or_credential_required,
              local_offline: response.local_offline,
              files_written: response.files_written,
            });
            setPhase(response.backups.length === 0 ? "empty" : "ready");
            return;
          case "capabilities":
          case "runtime_state":
          case "project_selected":
          case "project_catalog":
          case "preflight":
          case "config_discovery":
          case "tree_page":
          case "note_read":
          case "fixture_restored":
          case "windows_runtime":
          case "draft_saved":
          case "draft_loaded":
          case "note_written":
          case "note_edited":
          case "note_moved":
          case "note_deleted":
          case "relation_list":
      case "graph_page":
      case "search_page":
      case "context_preview":
      case "activity_page":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
            setError(unexpectedBackupResponse());
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
    <button type="button" className="action" onClick={() => setReloadToken((token) => token + 1)}>
      {t("backupsRefresh")}
    </button>
  );

  if (phase === "loading") {
    return (
      <section className="panel" data-state="status" aria-labelledby="backups-title" aria-busy="true">
        <p className="state-badge">{t("statusBadge")}</p>
        <h2 id="backups-title">{t("backupsTitle")}</h2>
        <p role="status">{t("backupsLoading")}</p>
      </section>
    );
  }

  if (phase === "error" && error) {
    return (
      <section className="panel" data-state="error" aria-labelledby="backups-error-title" role="alert">
        <p className="state-badge">{t("errorBadge")}</p>
        <h2 id="backups-error-title">{t("backupsErrorTitle")}</h2>
        <p>
          {errorCategoryLabel(error.category)}：{error.message}
        </p>
        <p>{t("backupsErrorBody")}</p>
        {refresh}
      </section>
    );
  }

  const empty = phase === "empty" || !catalog || catalog.backups.length === 0;
  return (
    <section
      className="panel"
      data-state={empty ? "empty" : "status"}
      aria-labelledby="backups-title"
    >
      <p className="state-badge">{empty ? t("emptyBadge") : t("statusBadge")}</p>
      <h2 id="backups-title">{empty ? t("backupsEmptyTitle") : t("backupsReadyTitle")}</h2>
      <p>{empty ? t("backupsEmptyBody") : t("backupsReadyBody")}</p>
      <p>{t("backupsUnverifiedNote")}</p>
      <ul className="policy-list">
        <li>{t("backupsNoVault")}</li>
        <li>
          {t("backupsFilesWrittenLabel")}：
          {catalog?.files_written ? t("backupsWroteFiles") : t("backupsNoWrite")}
        </li>
      </ul>
      {empty || !catalog ? null : (
        <ul className="tree-list">
          {catalog.backups.map((backup) => (
            <BackupRow
              key={backup.id}
              backup={backup}
              onRestored={(result) => {
                setRestoreResult(result);
                setCatalog({
                  ...catalog,
                  files_written: catalog.files_written || result.files_written,
                });
              }}
              onError={(next) => {
                setError(next);
                setPhase("error");
              }}
            />
          ))}
        </ul>
      )}
      <RestoreObservation result={restoreResult} />
      {refresh}
    </section>
  );
}

function BackupRow({
  backup,
  onRestored,
  onError,
}: {
  backup: BackupRecordDto;
  onRestored: (result: RestoreResultDto) => void;
  onError: (error: BackupError) => void;
}) {
  const canRestore = backup.kind === OWNED_KIND && isFixtureBackupId(backup.id);
  return (
    <li>
      <p>
        {t("backupsIdLabel")}：{backup.id}
      </p>
      {canRestore ? (
        <button
          type="button"
          className="action"
          onClick={() => {
            void restoreNamedFixture(backup.id, onRestored, onError);
          }}
        >
          {t("backupsRestore")}
        </button>
      ) : null}
    </li>
  );
}

async function restoreNamedFixture(
  backupId: string,
  onRestored: (result: RestoreResultDto) => void,
  onError: (error: BackupError) => void,
): Promise<void> {
  if (!isFixtureBackupId(backupId)) {
    onError({ category: "policy", message: t("backupsRestoreDenied") });
    return;
  }
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "restore_fixture",
      args: {
        workspace: route.workspace,
        project: route.project,
        backup_id: backupId,
      },
    });
    switch (response.kind) {
      case "error":
        onError({ category: response.category, message: response.message });
        return;
      case "fixture_restored":
        onRestored({
          backup_id: response.backup_id,
          files: response.files,
          files_written: response.files_written,
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
      case "note_read":
      case "backup_catalog":
      case "windows_runtime":
      case "draft_saved":
      case "draft_loaded":
      case "note_written":
      case "note_edited":
      case "note_moved":
      case "note_deleted":
      case "relation_list":
      case "graph_page":
      case "search_page":
      case "context_preview":
      case "activity_page":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
        onError({ category: "schema", message: t("unexpectedRestore") });
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    onError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

function restoreObservationLabel(result: RestoreResultDto): string {
  switch (result.observation.classified_as) {
    case "disk_verified":
      return t("backupsObservationDisk");
    case "accepted_unverified":
      return t("backupsObservationUnverified");
    case "empty":
      return t("backupsObservationEmpty");
    case "unclassified":
      return t("backupsObservationUnclassified");
    default: {
      const exhaustive: never = result.observation.classified_as;
      return exhaustive;
    }
  }
}

function RestoreObservation({ result }: { result: RestoreResultDto | null }) {
  if (!result) {
    return (
      <section className="subpanel" data-state="empty" aria-labelledby="restore-preview-title">
        <p className="state-badge">{t("emptyBadge")}</p>
        <h3 id="restore-preview-title">{t("backupsRestoreTitle")}</h3>
        <p>{t("backupsRestoreEmpty")}</p>
      </section>
    );
  }
  return (
    <section className="subpanel" data-state="status" aria-labelledby="restore-preview-title">
      <p className="state-badge">{t("statusBadge")}</p>
      <h3 id="restore-preview-title">{t("backupsRestoreTitle")}</h3>
      <p>
        {t("backupsIdLabel")}：{result.backup_id}
      </p>
      <p>{restoreObservationLabel(result)}</p>
      <p>
        {t("backupsFilesWrittenLabel")}：
        {result.files_written ? t("backupsWroteFiles") : t("backupsNoWrite")}
      </p>
      {result.files.length === 0 ? null : (
        <ul className="policy-list">
          {result.files.map((file) => (
            <li key={file.identifier}>{file.identifier}</li>
          ))}
        </ul>
      )}
    </section>
  );
}

type WindowsError = {
  category: "policy" | "schema" | "unsupported" | "invoke";
  message: string;
};

function unexpectedWindowsResponse(): WindowsError {
  return { category: "schema", message: t("unexpectedWindowsRuntime") };
}

function hostOsLabel(os: WindowsRuntimeDto["host_os"]): string {
  switch (os) {
    case "windows":
      return t("windowsHostWindows");
    case "other":
      return t("windowsHostOther");
    default: {
      const exhaustive: never = os;
      return exhaustive;
    }
  }
}

function observedFlag(value: boolean): string {
  return value ? t("windowsYes") : t("windowsNo");
}

function unverifiedFlag(verified: boolean): string {
  return verified ? t("windowsVerified") : t("windowsUnverified");
}

function WindowsRuntimeCard() {
  const [reloadToken, setReloadToken] = useState(0);
  const [phase, setPhase] = useState<"loading" | "ready" | "empty" | "error">("loading");
  const [dto, setDto] = useState<WindowsRuntimeDto | null>(null);
  const [error, setError] = useState<WindowsError | null>(null);

  useEffect(() => {
    let cancelled = false;
    setPhase("loading");
    setDto(null);
    setError(null);
    void (async () => {
      try {
        const response = await invokeTyped<IpcResponse>({
          command: "inspect_windows_runtime",
          args: {},
        });
        if (cancelled) {
          return;
        }
        switch (response.kind) {
          case "error":
            setError({ category: response.category, message: response.message });
            setPhase("error");
            return;
          case "windows_runtime":
            setDto({
              host_os: response.host_os,
              webview2_files_present: response.webview2_files_present,
              webview2_session_verified: response.webview2_session_verified,
              job_object_assigned: response.job_object_assigned,
              job_object_api_documented: response.job_object_api_documented,
              installer_bundle_active: response.installer_bundle_active,
              files_written: response.files_written,
              scanned_user_obsidian_vault: response.scanned_user_obsidian_vault,
              scanned_user_basic_memory_home: response.scanned_user_basic_memory_home,
            });
            setPhase(response.host_os === "windows" ? "ready" : "empty");
            return;
          case "capabilities":
          case "runtime_state":
          case "project_selected":
          case "project_catalog":
          case "preflight":
          case "config_discovery":
          case "tree_page":
          case "note_read":
          case "backup_catalog":
          case "fixture_restored":
          case "draft_saved":
          case "draft_loaded":
          case "note_written":
          case "note_edited":
          case "note_moved":
          case "note_deleted":
          case "relation_list":
      case "graph_page":
      case "search_page":
      case "context_preview":
      case "activity_page":
      case "search_inspector":
      case "recall_benchmark":
      case "shutdown_begun":
            setError(unexpectedWindowsResponse());
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
    <button type="button" className="action" onClick={() => setReloadToken((token) => token + 1)}>
      {t("windowsRefresh")}
    </button>
  );

  if (phase === "loading") {
    return (
      <section className="panel" data-state="status" aria-labelledby="windows-runtime-title" aria-busy="true">
        <p className="state-badge">{t("statusBadge")}</p>
        <h2 id="windows-runtime-title">{t("windowsTitle")}</h2>
        <p role="status">{t("windowsLoading")}</p>
      </section>
    );
  }

  if (phase === "error" && error) {
    return (
      <section className="panel" data-state="error" aria-labelledby="windows-runtime-error-title" role="alert">
        <p className="state-badge">{t("errorBadge")}</p>
        <h2 id="windows-runtime-error-title">{t("windowsErrorTitle")}</h2>
        <p>
          {errorCategoryLabel(error.category)}：{error.message}
        </p>
        <p>{t("windowsErrorBody")}</p>
        {refresh}
      </section>
    );
  }

  if (!dto) {
    return (
      <section className="panel" data-state="empty" aria-labelledby="windows-runtime-title">
        <p className="state-badge">{t("emptyBadge")}</p>
        <h2 id="windows-runtime-title">{t("windowsEmptyTitle")}</h2>
        <p>{t("windowsEmptyBody")}</p>
        {refresh}
      </section>
    );
  }

  const empty = phase === "empty" || dto.host_os !== "windows";
  return (
    <section
      className="panel"
      data-state={empty ? "empty" : "status"}
      aria-labelledby="windows-runtime-title"
    >
      <p className="state-badge">{empty ? t("emptyBadge") : t("statusBadge")}</p>
      <h2 id="windows-runtime-title">{empty ? t("windowsEmptyTitle") : t("windowsReadyTitle")}</h2>
      <p>{empty ? t("windowsEmptyBody") : t("windowsReadyBody")}</p>
      <h3>{t("windowsObservedTitle")}</h3>
      <dl className="facts wide">
        <div>
          <dt>{t("windowsHostLabel")}</dt>
          <dd>{hostOsLabel(dto.host_os)}</dd>
        </div>
        <div>
          <dt>{t("windowsWebview2FilesLabel")}</dt>
          <dd>{observedFlag(dto.webview2_files_present)}</dd>
        </div>
        <div>
          <dt>{t("windowsJobApiLabel")}</dt>
          <dd>{observedFlag(dto.job_object_api_documented)}</dd>
        </div>
        <div>
          <dt>{t("windowsBundleLabel")}</dt>
          <dd>{observedFlag(dto.installer_bundle_active)}</dd>
        </div>
        <div>
          <dt>{t("windowsFilesWrittenLabel")}</dt>
          <dd>{observedFlag(dto.files_written)}</dd>
        </div>
        <div>
          <dt>{t("windowsScannedVaultLabel")}</dt>
          <dd>{observedFlag(dto.scanned_user_obsidian_vault)}</dd>
        </div>
        <div>
          <dt>{t("windowsScannedHomeLabel")}</dt>
          <dd>{observedFlag(dto.scanned_user_basic_memory_home)}</dd>
        </div>
      </dl>
      <h3>{t("windowsUnverifiedTitle")}</h3>
      <dl className="facts wide unverified-facts">
        <div>
          <dt>{t("windowsWebview2SessionLabel")}</dt>
          <dd>{unverifiedFlag(dto.webview2_session_verified)}</dd>
        </div>
        <div>
          <dt>{t("windowsJobAssignedLabel")}</dt>
          <dd>{unverifiedFlag(dto.job_object_assigned)}</dd>
        </div>
      </dl>
      <h3>{t("windowsTaxonomyTitle")}</h3>
      <ul className="policy-list">
        <li>{t("windowsTaxonomyCompiled")}</li>
        <li>{t("windowsTaxonomyWebview2")}</li>
        <li>{t("windowsTaxonomyJob")}</li>
        <li>{t("windowsTaxonomyContract")}</li>
        <li>{t("windowsTaxonomyRestore")}</li>
      </ul>
      <p>{t("windowsUnsignedNote")}</p>
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
