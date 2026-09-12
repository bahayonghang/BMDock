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
  type SchemaValidateDto,
  type SchemaVerdict,
  type ContextPreviewDto,
  type ActivityPageDto,
  type ActivityEntryDto,
  type ResourcePageDto,
  type ResourceEntryDto,
  type PromptPageDto,
  type PromptEntryDto,
  type ToolInspectionDto,
  type InspectedToolDto,
  type ToolAdmission,
  type CliInventoryDto,
  type CliLeafDto,
  type PreflightDto,
  type ProjectCatalogDto,
  type RestoreResultDto,
  type ImportResultDto,
  type ImportClass,
  type ApiAuditDto,
  type ExtrasCatalogDto,
  type ExtraEntryDto,
  type IngestResultDto,
  type CloudInspectionDto,
  type SyncInspectionDto,
  type ShareCatalogDto,
  type HookInspectionDto,
  type ProviderInspectionDto,
  type AuditedApiLeafDto,
  type AuditedCliLeafDto,
  type AuditedIpcCommandDto,
  type UnavailableCapabilityDto,
  type AuditCoverage,
  type CapabilityStatus,
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

const SECTIONS = ["workbench", "runtime", "projects", "preflight", "backups", "import", "extras", "cloud", "sync", "hooks", "providers", "about"] as const;
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
    case "import":
      return t("navImport");
    case "extras":
      return t("navExtras");
    case "cloud":
      return t("navCloud");
    case "sync":
      return t("navSync");
    case "hooks":
      return t("navHooks");
    case "providers":
      return t("navProviders");
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
    case "import":
      return <ImportPanel />;
    case "extras":
      return <ExtrasPanel />;
    case "cloud":
      return <CloudPanel />;
    case "sync":
      return <SyncPanel />;
    case "hooks":
      return <HookPanel />;
    case "providers":
      return <ProviderPanel />;
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
  const [schema, setSchema] = useState<SchemaValidateDto | null>(null);
  const [schemaError, setSchemaError] = useState<WorkbenchError | null>(null);
  const [resources, setResources] = useState<ResourcePageDto | null>(null);
  const [resourcesError, setResourcesError] = useState<WorkbenchError | null>(null);
  const [prompts, setPrompts] = useState<PromptPageDto | null>(null);
  const [promptsError, setPromptsError] = useState<WorkbenchError | null>(null);
  const [toolProfile, setToolProfile] = useState<EngineProfile>("release");
  const [tools, setTools] = useState<ToolInspectionDto | null>(null);
  const [toolsError, setToolsError] = useState<WorkbenchError | null>(null);
  const [cli, setCli] = useState<CliInventoryDto | null>(null);
  const [cliError, setCliError] = useState<WorkbenchError | null>(null);
  const [audit, setAudit] = useState<ApiAuditDto | null>(null);
  const [auditError, setAuditError] = useState<WorkbenchError | null>(null);
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
    setSchema(null);
    setSchemaError(null);
    setResources(null);
    setResourcesError(null);
    setPrompts(null);
    setPromptsError(null);
    setTools(null);
    setToolsError(null);
    setCli(null);
    setCliError(null);
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
            void loadResources(setResources, setResourcesError);
            void loadPrompts(setPrompts, setPromptsError);
            void loadTools(toolProfile, setTools, setToolsError);
            void loadCli(toolProfile, setCli, setCliError);
            void loadApiAudit(toolProfile, setAudit, setAuditError);
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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

  useEffect(() => {
    if (!note?.identifier) {
      return;
    }
    void runSchemaValidate(note.identifier, null, setSchema, setSchemaError);
  }, [note?.identifier]);

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
      <SchemaWorkbenchPanel
        seedIdentifier={note?.identifier ?? null}
        schema={schema}
        error={schemaError}
        onValidate={(identifier, schemaId) => {
          void runSchemaValidate(identifier, schemaId, setSchema, setSchemaError);
        }}
      />
      <ResourceCatalogPanel
        resources={resources}
        error={resourcesError}
        onLoadMore={() => {
          if (resources?.next_cursor) {
            void loadMoreResources(resources, setResources, setResourcesError);
          }
        }}
      />
      <PromptCatalogPanel
        prompts={prompts}
        error={promptsError}
        onLoadMore={() => {
          if (prompts?.next_cursor) {
            void loadMorePrompts(prompts, setPrompts, setPromptsError);
          }
        }}
      />
      <ToolsCenterPanel
        profile={toolProfile}
        tools={tools}
        error={toolsError}
        onProfile={(profile) => {
          setToolProfile(profile);
          void loadTools(profile, setTools, setToolsError);
          void loadCli(profile, setCli, setCliError);
          void loadApiAudit(profile, setAudit, setAuditError);
        }}
      />
      <CliInventoryPanel
        profile={toolProfile}
        cli={cli}
        error={cliError}
        onLoadMore={() => {
          if (cli?.next_cursor) {
            void loadMoreCli(toolProfile, cli, setCli, setCliError);
          }
        }}
      />
      <ApiAuditPanel profile={toolProfile} audit={audit} error={auditError} />
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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

function unexpectedSchemaResponse(): WorkbenchError {
  return { category: "schema", message: t("unexpectedSchema") };
}

function asSchemaValidate(
  response: Extract<IpcResponse, { kind: "schema_validated" }>,
): SchemaValidateDto {
  return {
    identifier: response.identifier,
    schema_id: response.schema_id,
    verdict: response.verdict,
    required_fields: response.required_fields,
    missing_fields: response.missing_fields,
    observed_title: response.observed_title,
    observed_body: response.observed_body,
    observation: response.observation,
    engine_schema: false,
    scanned_user_obsidian_vault: false,
    scanned_user_basic_memory_home: false,
    files_written: false,
  };
}

async function runSchemaValidate(
  identifier: string,
  schemaId: string | null,
  setSchema: (schema: SchemaValidateDto | null) => void,
  setSchemaError: (error: WorkbenchError | null) => void,
): Promise<void> {
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "schema_validate",
      args: {
        workspace: route.workspace,
        project: route.project,
        identifier,
        ...(schemaId ? { schema_id: schemaId } : {}),
      },
    });
    switch (response.kind) {
      case "error":
        setSchema(null);
        setSchemaError({ category: response.category, message: response.message });
        return;
      case "schema_validated":
        setSchemaError(null);
        setSchema(asSchemaValidate(response));
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
      case "recall_benchmark":
      case "context_preview":
      case "activity_page":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "backup_catalog":
      case "fixture_restored":
      case "windows_runtime":
      case "draft_saved":
      case "draft_loaded":
      case "note_written":
      case "note_edited":
      case "note_moved":
      case "note_deleted":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
      case "shutdown_begun":
        setSchema(null);
        setSchemaError(unexpectedSchemaResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setSchema(null);
    setSchemaError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

function schemaVerdictLabel(verdict: SchemaVerdict): string {
  switch (verdict) {
    case "valid":
      return t("schemaVerdictValid");
    case "invalid":
      return t("schemaVerdictInvalid");
    case "empty":
      return t("schemaVerdictEmpty");
    case "unsupported":
      return t("schemaVerdictUnsupported");
    default: {
      const exhaustive: never = verdict;
      return exhaustive;
    }
  }
}

function SchemaWorkbenchPanel({
  seedIdentifier,
  schema,
  error,
  onValidate,
}: {
  seedIdentifier: string | null;
  schema: SchemaValidateDto | null;
  error: WorkbenchError | null;
  onValidate: (identifier: string, schemaId: string | null) => void;
}) {
  const [identifier, setIdentifier] = useState("");
  const [schemaId, setSchemaId] = useState("");
  useEffect(() => {
    if (seedIdentifier) {
      setIdentifier(seedIdentifier);
    }
  }, [seedIdentifier]);
  const empty = schema === null || schema.verdict === "empty";
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error
    ? t("schemaErrorTitle")
    : empty
      ? t("schemaEmptyTitle")
      : t("schemaReadyTitle");
  return (
    <section
      className="subpanel"
      data-state={state}
      aria-labelledby="schema-workbench-title"
      role={error ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h3 id="schema-workbench-title">{heading}</h3>
      <p>
        {error
          ? `${errorCategoryLabel(error.category)}：${error.message}`
          : empty
            ? t("schemaEmptyBody")
            : t("schemaReadyBody")}
      </p>
      <p>{t("schemaDiskOwned")}</p>
      <p>{t("schemaNotOfficialMcp")}</p>
      <form
        className="search-form"
        onSubmit={(event) => {
          event.preventDefault();
          const nextIdentifier = identifier.trim();
          if (nextIdentifier === "") {
            return;
          }
          const nextSchemaId = schemaId.trim();
          onValidate(nextIdentifier, nextSchemaId === "" ? null : nextSchemaId);
        }}
      >
        <label htmlFor="schema-identifier">{t("schemaIdentifierLabel")}</label>
        <input
          id="schema-identifier"
          type="text"
          value={identifier}
          autoComplete="off"
          spellCheck={false}
          onChange={(event) => setIdentifier(event.target.value)}
        />
        <label htmlFor="schema-id">{t("schemaIdLabel")}</label>
        <input
          id="schema-id"
          type="text"
          value={schemaId}
          autoComplete="off"
          spellCheck={false}
          onChange={(event) => setSchemaId(event.target.value)}
        />
        <button type="submit" className="action">
          {t("schemaSubmit")}
        </button>
      </form>
      {schema ? (
        <dl className="facts">
          <div>
            <dt>{t("schemaVerdictLabel")}</dt>
            <dd data-schema-verdict={schema.verdict}>{schemaVerdictLabel(schema.verdict)}</dd>
          </div>
          <div>
            <dt>{t("schemaIdValueLabel")}</dt>
            <dd>{schema.schema_id}</dd>
          </div>
          <div>
            <dt>{t("schemaObservedTitle")}</dt>
            <dd>{schema.observed_title ? t("runtimeYes") : t("runtimeNo")}</dd>
          </div>
          <div>
            <dt>{t("schemaObservedBody")}</dt>
            <dd>{schema.observed_body ? t("runtimeYes") : t("runtimeNo")}</dd>
          </div>
          <div>
            <dt>classified_as</dt>
            <dd data-observation={schema.observation.classified_as}>
              {observationClassLabel(schema.observation.classified_as)}
            </dd>
          </div>
          <div>
            <dt>{t("schemaEngineLabel")}</dt>
            <dd>{t("schemaEngineFalse")}</dd>
          </div>
        </dl>
      ) : null}
      {schema && schema.missing_fields.length > 0 ? (
        <ul className="schema-list">
          {schema.missing_fields.map((field) => (
            <li key={field}>
              {t("schemaMissingField")}: {field}
            </li>
          ))}
        </ul>
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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

function unexpectedResourceResponse(): WorkbenchError {
  return { category: "schema", message: t("unexpectedResources") };
}

function asResourcePage(response: Extract<IpcResponse, { kind: "resource_page" }>): ResourcePageDto {
  return {
    entries: response.entries,
    next_cursor: response.next_cursor,
    page: response.page,
    truncated: response.truncated,
    observation: response.observation,
    engine_resources: false,
    scanned_user_obsidian_vault: false,
    scanned_user_basic_memory_home: false,
    files_written: response.files_written,
  };
}

function mergeResourcePage(current: ResourcePageDto, next: ResourcePageDto): ResourcePageDto {
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

async function loadResources(
  setResources: (resources: ResourcePageDto | null) => void,
  setResourcesError: (error: WorkbenchError | null) => void,
): Promise<void> {
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "list_resources",
      args: {
        workspace: route.workspace,
        project: route.project,
        page_size: TREE_PAGE_SIZE,
      },
    });
    switch (response.kind) {
      case "error":
        setResources(null);
        setResourcesError({ category: response.category, message: response.message });
        return;
      case "resource_page":
        if (response.truncated) {
          setResources(null);
          setResourcesError(unexpectedResourceResponse());
          return;
        }
        setResourcesError(null);
        setResources(asResourcePage(response));
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
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
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
      case "schema_validated":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
      case "shutdown_begun":
        setResources(null);
        setResourcesError(unexpectedResourceResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setResources(null);
    setResourcesError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

async function loadMoreResources(
  current: ResourcePageDto,
  setResources: (resources: ResourcePageDto | null) => void,
  setResourcesError: (error: WorkbenchError | null) => void,
): Promise<void> {
  if (!current.next_cursor) {
    return;
  }
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "list_resources",
      args: {
        workspace: route.workspace,
        project: route.project,
        cursor: current.next_cursor,
        page_size: TREE_PAGE_SIZE,
      },
    });
    switch (response.kind) {
      case "error":
        setResourcesError({ category: response.category, message: response.message });
        return;
      case "resource_page":
        if (response.truncated) {
          setResourcesError(unexpectedResourceResponse());
          return;
        }
        setResourcesError(null);
        setResources(mergeResourcePage(current, asResourcePage(response)));
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
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
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
      case "schema_validated":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
      case "shutdown_begun":
        setResourcesError(unexpectedResourceResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setResourcesError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

function ResourceCatalogPanel({
  resources,
  error,
  onLoadMore,
}: {
  resources: ResourcePageDto | null;
  error: WorkbenchError | null;
  onLoadMore: () => void;
}) {
  const empty = resources === null || resources.entries.length === 0;
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error
    ? t("resourcesErrorTitle")
    : empty
      ? t("resourcesEmptyTitle")
      : t("resourcesReadyTitle");
  return (
    <section
      className="subpanel"
      data-state={state}
      aria-labelledby="resources-title"
      role={error ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h3 id="resources-title">{heading}</h3>
      <p>
        {error
          ? `${errorCategoryLabel(error.category)}：${error.message}`
          : empty
            ? t("resourcesEmptyBody")
            : t("resourcesReadyBody")}
      </p>
      <p>{t("resourcesPermalinkNotPath")}</p>
      <p>{t("resourcesNotOfficialMcp")}</p>
      <p>
        {t("resourcesEngineLabel")}: {t("resourcesEngineFalse")}
      </p>
      {resources && resources.entries.length > 0 ? (
        <ul className="resource-list">
          {resources.entries.map((entry: ResourceEntryDto) => (
            <li key={entry.identifier}>
              <span>{entry.identifier}</span>
              <span>{entry.title}</span>
            </li>
          ))}
        </ul>
      ) : null}
      {resources?.next_cursor ? (
        <div className="activity-actions">
          <button type="button" className="action" onClick={onLoadMore}>
            {t("resourcesLoadMore")}
          </button>
        </div>
      ) : null}
    </section>
  );
}

function unexpectedPromptResponse(): WorkbenchError {
  return { category: "schema", message: t("unexpectedPrompts") };
}

function asPromptPage(response: Extract<IpcResponse, { kind: "prompt_page" }>): PromptPageDto {
  return {
    entries: response.entries,
    next_cursor: response.next_cursor,
    page: response.page,
    truncated: response.truncated,
    observation: response.observation,
    engine_prompts: false,
    scanned_user_obsidian_vault: false,
    scanned_user_basic_memory_home: false,
    files_written: response.files_written,
  };
}

function mergePromptPage(current: PromptPageDto, next: PromptPageDto): PromptPageDto {
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

async function loadPrompts(
  setPrompts: (prompts: PromptPageDto | null) => void,
  setPromptsError: (error: WorkbenchError | null) => void,
): Promise<void> {
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "list_prompts",
      args: {
        workspace: route.workspace,
        project: route.project,
        page_size: TREE_PAGE_SIZE,
      },
    });
    switch (response.kind) {
      case "error":
        setPrompts(null);
        setPromptsError({ category: response.category, message: response.message });
        return;
      case "prompt_page":
        if (response.truncated) {
          setPrompts(null);
          setPromptsError(unexpectedPromptResponse());
          return;
        }
        setPromptsError(null);
        setPrompts(asPromptPage(response));
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
      case "resource_page":
      case "tool_inspection":
      case "cli_inventory":
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
      case "schema_validated":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
      case "shutdown_begun":
        setPrompts(null);
        setPromptsError(unexpectedPromptResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setPrompts(null);
    setPromptsError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

async function loadMorePrompts(
  current: PromptPageDto,
  setPrompts: (prompts: PromptPageDto | null) => void,
  setPromptsError: (error: WorkbenchError | null) => void,
): Promise<void> {
  if (!current.next_cursor) {
    return;
  }
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "list_prompts",
      args: {
        workspace: route.workspace,
        project: route.project,
        cursor: current.next_cursor,
        page_size: TREE_PAGE_SIZE,
      },
    });
    switch (response.kind) {
      case "error":
        setPromptsError({ category: response.category, message: response.message });
        return;
      case "prompt_page":
        if (response.truncated) {
          setPromptsError(unexpectedPromptResponse());
          return;
        }
        setPromptsError(null);
        setPrompts(mergePromptPage(current, asPromptPage(response)));
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
      case "resource_page":
      case "tool_inspection":
      case "cli_inventory":
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
      case "schema_validated":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
      case "shutdown_begun":
        setPromptsError(unexpectedPromptResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setPromptsError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

function PromptCatalogPanel({
  prompts,
  error,
  onLoadMore,
}: {
  prompts: PromptPageDto | null;
  error: WorkbenchError | null;
  onLoadMore: () => void;
}) {
  const empty = prompts === null || prompts.entries.length === 0;
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error
    ? t("promptsErrorTitle")
    : empty
      ? t("promptsEmptyTitle")
      : t("promptsReadyTitle");
  return (
    <section
      className="subpanel"
      data-state={state}
      aria-labelledby="prompts-title"
      role={error ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h3 id="prompts-title">{heading}</h3>
      <p>
        {error
          ? `${errorCategoryLabel(error.category)}：${error.message}`
          : empty
            ? t("promptsEmptyBody")
            : t("promptsReadyBody")}
      </p>
      <p>{t("promptsPermalinkNotPath")}</p>
      <p>{t("promptsNotOfficialMcp")}</p>
      <p>
        {t("promptsEngineLabel")}: {t("promptsEngineFalse")}
      </p>
      {prompts && prompts.entries.length > 0 ? (
        <ul className="prompt-list">
          {prompts.entries.map((entry: PromptEntryDto) => (
            <li key={entry.identifier}>
              <span>{entry.identifier}</span>
              <span>{entry.title}</span>
            </li>
          ))}
        </ul>
      ) : null}
      {prompts?.next_cursor ? (
        <div className="activity-actions">
          <button type="button" className="action" onClick={onLoadMore}>
            {t("promptsLoadMore")}
          </button>
        </div>
      ) : null}
    </section>
  );
}

function unexpectedToolsResponse(): WorkbenchError {
  return { category: "schema", message: t("unexpectedTools") };
}

function asToolInspection(response: Extract<IpcResponse, { kind: "tool_inspection" }>): ToolInspectionDto {
  return {
    profile_id: response.profile_id,
    expected_tool_count: response.expected_tool_count,
    tools: response.tools,
    mixed_profiles: false,
    observation: response.observation,
    engine_tools: false,
    call_tool_allowed: false,
    scanned_user_obsidian_vault: false,
    scanned_user_basic_memory_home: false,
    files_written: false,
  };
}

function toolAdmissionLabel(admission: ToolAdmission): string {
  switch (admission) {
    case "allowlisted":
      return t("toolsAdmissionAllowlisted");
    case "denied":
      return t("toolsAdmissionDenied");
    default: {
      const exhaustive: never = admission;
      return exhaustive;
    }
  }
}

async function loadTools(
  profile: EngineProfile,
  setTools: (tools: ToolInspectionDto | null) => void,
  setToolsError: (error: WorkbenchError | null) => void,
): Promise<void> {
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "inspect_tools",
      args: {
        workspace: route.workspace,
        project: route.project,
        profile_id: profile,
      },
    });
    switch (response.kind) {
      case "error":
        setTools(null);
        setToolsError({ category: response.category, message: response.message });
        return;
      case "tool_inspection":
        if (response.mixed_profiles || response.engine_tools || response.call_tool_allowed) {
          setTools(null);
          setToolsError(unexpectedToolsResponse());
          return;
        }
        setToolsError(null);
        setTools(asToolInspection(response));
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
      case "resource_page":
      case "prompt_page":
      case "cli_inventory":
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
      case "schema_validated":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
      case "shutdown_begun":
        setTools(null);
        setToolsError(unexpectedToolsResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setTools(null);
    setToolsError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

function ToolsCenterPanel({
  profile,
  tools,
  error,
  onProfile,
}: {
  profile: EngineProfile;
  tools: ToolInspectionDto | null;
  error: WorkbenchError | null;
  onProfile: (profile: EngineProfile) => void;
}) {
  const empty = tools === null || tools.tools.length === 0;
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error
    ? t("toolsErrorTitle")
    : empty
      ? t("toolsEmptyTitle")
      : t("toolsReadyTitle");
  return (
    <section
      className="subpanel"
      data-state={state}
      aria-labelledby="tools-title"
      role={error ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h3 id="tools-title">{heading}</h3>
      <p>
        {error
          ? `${errorCategoryLabel(error.category)}：${error.message}`
          : empty
            ? t("toolsEmptyBody")
            : t("toolsReadyBody")}
      </p>
      <div className="activity-actions">
        <label htmlFor="tools-profile">{t("toolsProfileLabel")}</label>
        <select
          id="tools-profile"
          value={profile}
          onChange={(event) => {
            const next = event.target.value;
            if (next === "release" || next === "main-preview") {
              onProfile(next);
            }
          }}
        >
          <option value="release">{t("profileRelease")}</option>
          <option value="main-preview">{t("profileMainPreview")}</option>
        </select>
      </div>
      <p>
        {t("toolsCountLabel")}: {tools?.expected_tool_count ?? 0}
      </p>
      <p>{t("toolsIdentityDistinct")}</p>
      <p>{t("toolsNotOfficialMcp")}</p>
      <p>
        {t("toolsEngineLabel")}: {t("toolsEngineFalse")}
      </p>
      {tools && tools.tools.length > 0 ? (
        <ul className="tool-list">
          {tools.tools.map((entry: InspectedToolDto) => (
            <li key={`${entry.identity}:${entry.name}`}>
              <span>{entry.name}</span>
              <span>{toolAdmissionLabel(entry.admission)}</span>
            </li>
          ))}
        </ul>
      ) : null}
    </section>
  );
}

function unexpectedCliResponse(): WorkbenchError {
  return { category: "schema", message: t("unexpectedCli") };
}

function asCliInventory(response: Extract<IpcResponse, { kind: "cli_inventory" }>): CliInventoryDto {
  return {
    profile_id: response.profile_id,
    leaves: response.leaves,
    next_cursor: response.next_cursor,
    page: response.page,
    truncated: response.truncated,
    observation: response.observation,
    engine_cli: false,
    executed: false,
    mixed_profiles: false,
    scanned_user_obsidian_vault: false,
    scanned_user_basic_memory_home: false,
    files_written: response.files_written,
  };
}

function mergeCliInventory(current: CliInventoryDto, next: CliInventoryDto): CliInventoryDto {
  const leaves = [...current.leaves];
  for (const leaf of next.leaves) {
    const key = leaf.path.join("/");
    if (!leaves.some((existing) => existing.path.join("/") === key)) {
      leaves.push(leaf);
    }
  }
  return {
    ...next,
    leaves,
  };
}

async function loadCli(
  profile: EngineProfile,
  setCli: (cli: CliInventoryDto | null) => void,
  setCliError: (error: WorkbenchError | null) => void,
): Promise<void> {
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "list_cli_inventory",
      args: {
        workspace: route.workspace,
        project: route.project,
        profile_id: profile,
        page_size: TREE_PAGE_SIZE,
      },
    });
    switch (response.kind) {
      case "error":
        setCli(null);
        setCliError({ category: response.category, message: response.message });
        return;
      case "cli_inventory":
        if (response.truncated || response.executed || response.engine_cli) {
          setCli(null);
          setCliError(unexpectedCliResponse());
          return;
        }
        setCliError(null);
        setCli(asCliInventory(response));
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
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
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
      case "schema_validated":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
      case "shutdown_begun":
        setCli(null);
        setCliError(unexpectedCliResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setCli(null);
    setCliError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

async function loadMoreCli(
  profile: EngineProfile,
  current: CliInventoryDto,
  setCli: (cli: CliInventoryDto | null) => void,
  setCliError: (error: WorkbenchError | null) => void,
): Promise<void> {
  if (!current.next_cursor) {
    return;
  }
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "list_cli_inventory",
      args: {
        workspace: route.workspace,
        project: route.project,
        profile_id: profile,
        cursor: current.next_cursor,
        page_size: TREE_PAGE_SIZE,
      },
    });
    switch (response.kind) {
      case "error":
        setCliError({ category: response.category, message: response.message });
        return;
      case "cli_inventory":
        if (response.truncated || response.executed || response.engine_cli) {
          setCliError(unexpectedCliResponse());
          return;
        }
        setCliError(null);
        setCli(mergeCliInventory(current, asCliInventory(response)));
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
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
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
      case "schema_validated":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
      case "shutdown_begun":
        setCliError(unexpectedCliResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setCliError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

function CliInventoryPanel({
  profile,
  cli,
  error,
  onLoadMore,
}: {
  profile: EngineProfile;
  cli: CliInventoryDto | null;
  error: WorkbenchError | null;
  onLoadMore: () => void;
}) {
  const empty = cli === null || cli.leaves.length === 0;
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error ? t("cliErrorTitle") : empty ? t("cliEmptyTitle") : t("cliReadyTitle");
  return (
    <section
      className="subpanel"
      data-state={state}
      aria-labelledby="cli-title"
      role={error ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h3 id="cli-title">{heading}</h3>
      <p>
        {error
          ? `${errorCategoryLabel(error.category)}：${error.message}`
          : empty
            ? t("cliEmptyBody")
            : t("cliReadyBody")}
      </p>
      <p>
        {t("toolsProfileLabel")}: {profile}
      </p>
      <p>{t("cliNotExecuted")}</p>
      <p>{t("cliLeafNotBucket")}</p>
      <p>{t("cliNotLive")}</p>
      <p>
        {t("cliEngineLabel")}: {t("cliEngineFalse")}
      </p>
      {cli && cli.leaves.length > 0 ? (
        <ul className="cli-list">
          {cli.leaves.map((entry: CliLeafDto) => (
            <li key={entry.path.join("/")}>
              <span>{entry.path.join(" ")}</span>
              <span>{t("cliNotExecuted")}</span>
            </li>
          ))}
        </ul>
      ) : null}
      {cli?.next_cursor ? (
        <div className="activity-actions">
          <button type="button" className="action" onClick={onLoadMore}>
            {t("cliLoadMore")}
          </button>
        </div>
      ) : null}
    </section>
  );
}

function unexpectedAuditResponse(): WorkbenchError {
  return { category: "schema", message: t("unexpectedAudit") };
}

function asApiAudit(response: Extract<IpcResponse, { kind: "api_audit" }>): ApiAuditDto {
  return {
    profile_id: response.profile_id,
    expected_tool_count: response.expected_tool_count,
    api_leaves: response.api_leaves,
    cli_leaves: response.cli_leaves,
    ipc_commands: response.ipc_commands,
    uncovered: response.uncovered,
    unavailable: response.unavailable,
    mixed_profiles: false,
    full_api_coverage: false,
    semantic_enabled: false,
    model_loaded: false,
    observation: response.observation,
    engine_tools: false,
    engine_cli: false,
    engine_schema: false,
    live_mcp: false,
    live_cli: false,
    call_tool_allowed: false,
    scanned_user_obsidian_vault: false,
    scanned_user_basic_memory_home: false,
    files_written: false,
  };
}

function auditCoverageLabel(coverage: AuditCoverage): string {
  switch (coverage) {
    case "present":
      return t("auditCoveragePresent");
    case "missing":
      return t("auditCoverageMissing");
    case "unverified":
      return t("auditCoverageUnverified");
    default: {
      const exhaustive: never = coverage;
      return exhaustive;
    }
  }
}

function capabilityStatusLabel(status: CapabilityStatus): string {
  switch (status) {
    case "unavailable":
      return t("auditCapabilityUnavailable");
    case "unverified":
      return t("auditCapabilityUnverified");
    default: {
      const exhaustive: never = status;
      return exhaustive;
    }
  }
}

async function loadApiAudit(
  profile: EngineProfile,
  setAudit: (audit: ApiAuditDto | null) => void,
  setAuditError: (error: WorkbenchError | null) => void,
): Promise<void> {
  const route = copyFixtureRoute();
  try {
    const response = await invokeTyped<IpcResponse>({
      command: "inspect_api_audit",
      args: {
        workspace: route.workspace,
        project: route.project,
        profile_id: profile,
      },
    });
    switch (response.kind) {
      case "error":
        setAudit(null);
        setAuditError({ category: response.category, message: response.message });
        return;
      case "api_audit":
        if (
          response.mixed_profiles ||
          response.engine_tools ||
          response.engine_cli ||
          response.live_mcp ||
          response.live_cli ||
          response.full_api_coverage ||
          response.semantic_enabled ||
          response.model_loaded ||
          response.call_tool_allowed
        ) {
          setAudit(null);
          setAuditError(unexpectedAuditResponse());
          return;
        }
        setAuditError(null);
        setAudit(asApiAudit(response));
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
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
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
      case "schema_validated":
      case "notes_imported":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
      case "shutdown_begun":
        setAudit(null);
        setAuditError(unexpectedAuditResponse());
        return;
      default: {
        const exhaustive: never = response;
        return exhaustive;
      }
    }
  } catch (cause) {
    setAudit(null);
    setAuditError({
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    });
  }
}

function ApiAuditPanel({
  profile,
  audit,
  error,
}: {
  profile: EngineProfile;
  audit: ApiAuditDto | null;
  error: WorkbenchError | null;
}) {
  const empty =
    audit === null ||
    (audit.api_leaves.length === 0 &&
      audit.cli_leaves.length === 0 &&
      audit.ipc_commands.length === 0);
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error
    ? t("auditErrorTitle")
    : empty
      ? t("auditEmptyTitle")
      : t("auditReadyTitle");
  const presentApi = (audit?.api_leaves ?? []).filter(
    (leaf: AuditedApiLeafDto) => leaf.coverage === "present",
  );
  const missingApi = (audit?.api_leaves ?? []).filter(
    (leaf: AuditedApiLeafDto) => leaf.coverage === "missing",
  );
  const unverifiedCli = (audit?.cli_leaves ?? []).filter(
    (leaf: AuditedCliLeafDto) => leaf.coverage === "unverified" || leaf.coverage === "missing",
  );
  const presentIpc = (audit?.ipc_commands ?? []).filter(
    (command: AuditedIpcCommandDto) => leafIsPresent(command.coverage),
  );
  return (
    <section
      className="subpanel"
      data-state={state}
      aria-labelledby="audit-title"
      role={error ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h3 id="audit-title">{heading}</h3>
      <p>
        {error
          ? `${errorCategoryLabel(error.category)}：${error.message}`
          : empty
            ? t("auditEmptyBody")
            : t("auditReadyBody")}
      </p>
      <p>
        {t("toolsProfileLabel")}: {profile}
      </p>
      <p>
        {t("toolsCountLabel")}: {audit?.expected_tool_count ?? 0}
      </p>
      <p>{t("auditNotFullCoverage")}</p>
      <p>{t("auditSemanticDisabled")}</p>
      <p>{t("auditNotLive")}</p>
      <p>
        {t("auditEngineLabel")}: {t("auditEngineFalse")}
      </p>
      {presentIpc.length > 0 ? (
        <div>
          <h4>{t("auditPresentTitle")}</h4>
          <ul className="audit-list">
            {presentIpc.map((command: AuditedIpcCommandDto) => (
              <li key={`ipc-${command.name}`}>
                <span>{command.name}</span>
                <span>{auditCoverageLabel(command.coverage)}</span>
              </li>
            ))}
            {presentApi.map((leaf: AuditedApiLeafDto) => (
              <li key={`api-present-${leaf.name}`}>
                <span>{leaf.name}</span>
                <span>{auditCoverageLabel(leaf.coverage)}</span>
              </li>
            ))}
          </ul>
        </div>
      ) : null}
      {missingApi.length > 0 || unverifiedCli.length > 0 || (audit?.uncovered.length ?? 0) > 0 ? (
        <div>
          <h4>{t("auditMissingTitle")}</h4>
          <ul className="audit-list">
            {missingApi.map((leaf: AuditedApiLeafDto) => (
              <li key={`api-missing-${leaf.name}`}>
                <span>{leaf.name}</span>
                <span>{auditCoverageLabel(leaf.coverage)}</span>
              </li>
            ))}
            {unverifiedCli.map((leaf: AuditedCliLeafDto) => (
              <li key={`cli-${leaf.path.join("/")}`}>
                <span>{leaf.path.join(" ")}</span>
                <span>{auditCoverageLabel(leaf.coverage)}</span>
              </li>
            ))}
          </ul>
        </div>
      ) : null}
      {(audit?.unavailable.length ?? 0) > 0 ? (
        <div>
          <h4>{t("auditUnavailableTitle")}</h4>
          <ul className="audit-list">
            {audit?.unavailable.map((capability: UnavailableCapabilityDto) => (
              <li key={capability.name}>
                <span>{capability.name}</span>
                <span>{capabilityStatusLabel(capability.status)}</span>
              </li>
            ))}
          </ul>
        </div>
      ) : null}
    </section>
  );
}

function leafIsPresent(coverage: AuditCoverage): boolean {
  switch (coverage) {
    case "present":
      return true;
    case "missing":
    case "unverified":
      return false;
    default: {
      const exhaustive: never = coverage;
      return exhaustive;
    }
  }
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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
        <li>{t("policyNoCloud")}</li>
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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

type ImportError = {
  category: "policy" | "schema" | "unsupported" | "invoke";
  message: string;
};

function isFixtureSourceId(id: string): boolean {
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

function unexpectedImportResponse(): ImportError {
  return { category: "schema", message: t("unexpectedImport") };
}

function importObservationLabel(classified: ImportClass): string {
  switch (classified) {
    case "disk_verified":
      return t("importObservationDisk");
    case "accepted_unverified":
      return t("importObservationUnverified");
    case "empty":
      return t("importObservationEmpty");
    case "unclassified":
      return t("importObservationUnclassified");
    default: {
      const exhaustive: never = classified;
      return exhaustive;
    }
  }
}

function ImportPanel() {
  const [sourceId, setSourceId] = useState("");
  const [phase, setPhase] = useState<"empty" | "ready" | "error">("empty");
  const [result, setResult] = useState<ImportResultDto | null>(null);
  const [error, setError] = useState<ImportError | null>(null);

  const runImport = async () => {
    if (!isFixtureSourceId(sourceId)) {
      setError({ category: "policy", message: t("importSourceDenied") });
      setPhase("error");
      return;
    }
    const route = copyFixtureRoute();
    try {
      const response = await invokeTyped<IpcResponse>({
        command: "import_notes",
        args: {
          workspace: route.workspace,
          project: route.project,
          source_id: sourceId.trim(),
        },
      });
      switch (response.kind) {
        case "error":
          setError({ category: response.category, message: response.message });
          setResult(null);
          setPhase("error");
          return;
        case "notes_imported":
          if (
            response.engine_import ||
            response.scanned_user_obsidian_vault ||
            (response.observation.disk_verified && !response.files_written) ||
            (response.observation.classified_as === "disk_verified" && !response.files_written)
          ) {
            setError(unexpectedImportResponse());
            setResult(null);
            setPhase("error");
            return;
          }
          setError(null);
          setResult({
            source_id: response.source_id,
            files: response.files,
            files_written: response.files_written,
            observation: response.observation,
            engine_import: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
          });
          setPhase(response.files.length === 0 ? "empty" : "ready");
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
        case "relation_list":
        case "graph_page":
        case "search_page":
        case "context_preview":
        case "activity_page":
        case "search_inspector":
        case "recall_benchmark":
        case "schema_validated":
        case "resource_page":
        case "prompt_page":
        case "tool_inspection":
        case "cli_inventory":
        case "api_audit":
        case "extras_catalog":
        case "document_ingested":
        case "cloud_inspection":
        case "sync_inspection":
        case "share_catalog":
        case "hook_inspection":
        case "provider_inspection":
        case "shutdown_begun":
          setError(unexpectedImportResponse());
          setResult(null);
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
      setResult(null);
      setPhase("error");
    }
  };

  const empty = phase === "empty" && !error;
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error
    ? t("importErrorTitle")
    : empty
      ? t("importEmptyTitle")
      : t("importReadyTitle");

  return (
    <section
      className="panel"
      data-state={state}
      aria-labelledby="import-title"
      role={error ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h2 id="import-title">{heading}</h2>
      <p>
        {error
          ? `${errorCategoryLabel(error.category)}：${error.message}`
          : empty
            ? t("importEmptyBody")
            : t("importReadyBody")}
      </p>
      <p>{t("importNotRestore")}</p>
      <p>{t("importNotOfficial")}</p>
      <p>{t("importEnvelopeNote")}</p>
      <div className="crud-editor">
        <label htmlFor="import-source-id">{t("importSourceLabel")}</label>
        <input
          id="import-source-id"
          type="text"
          value={sourceId}
          autoComplete="off"
          spellCheck={false}
          onChange={(event) => {
            setSourceId(event.target.value);
            setError(null);
          }}
        />
      </div>
      <button type="button" className="action" onClick={() => void runImport()}>
        {t("importSubmit")}
      </button>
      <ul className="policy-list">
        <li>{t("importNoVault")}</li>
        <li>
          {t("importFilesWrittenLabel")}：
          {result?.files_written ? t("importWroteFiles") : t("importNoWrite")}
        </li>
        <li>
          {t("importEngineLabel")}：{t("importEngineFalse")}
        </li>
      </ul>
      {result ? (
        <ul className="import-list">
          {result.files.map((file) => (
            <li key={file.identifier}>
              <span>{file.identifier}</span>
              <span>{importObservationLabel(result.observation.classified_as)}</span>
            </li>
          ))}
        </ul>
      ) : null}
      {result ? <p>{importObservationLabel(result.observation.classified_as)}</p> : null}
    </section>
  );
}

type ExtrasError = {
  category: "policy" | "schema" | "unsupported" | "invoke";
  message: string;
};

function unexpectedExtrasResponse(): ExtrasError {
  return { category: "schema", message: t("unexpectedExtras") };
}

function extrasObservationLabel(classified: ImportClass | NoteCrudClass): string {
  switch (classified) {
    case "disk_verified":
      return t("extrasObservationDisk");
    case "accepted_unverified":
      return t("extrasObservationUnverified");
    case "empty":
      return t("extrasObservationEmpty");
    case "unclassified":
      return t("extrasObservationUnclassified");
    case "conflict":
      return t("extrasObservationUnverified");
    default: {
      const exhaustive: never = classified;
      return exhaustive;
    }
  }
}

function ExtrasPanel() {
  const [sourceId, setSourceId] = useState("");
  const [phase, setPhase] = useState<"empty" | "ready" | "error">("empty");
  const [catalog, setCatalog] = useState<ExtrasCatalogDto | null>(null);
  const [ingested, setIngested] = useState<IngestResultDto | null>(null);
  const [error, setError] = useState<ExtrasError | null>(null);

  const loadExtras = async (extraId?: string) => {
    const route = copyFixtureRoute();
    try {
      const response = await invokeTyped<IpcResponse>({
        command: "inspect_extras",
        args: {
          workspace: route.workspace,
          project: route.project,
          ...(extraId ? { extra_id: extraId } : {}),
        },
      });
      switch (response.kind) {
        case "error":
          setError({ category: response.category, message: response.message });
          setCatalog(null);
          setPhase("error");
          return;
        case "extras_catalog":
          if (
            response.engine_extras ||
            response.official_pdf_office ||
            response.semantic_enabled ||
            response.model_loaded ||
            response.scanned_user_obsidian_vault ||
            (response.extras_enabled &&
              (response.extras.length === 0 || !response.observation.disk_verified))
          ) {
            setError(unexpectedExtrasResponse());
            setCatalog(null);
            setPhase("error");
            return;
          }
          setError(null);
          setCatalog({
            extra_id: response.extra_id,
            extras: response.extras,
            extras_enabled: response.extras_enabled,
            semantic_enabled: false,
            model_loaded: false,
            observation: response.observation,
            engine_extras: false,
            official_pdf_office: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            files_written: false,
          });
          setPhase(response.extras.length === 0 ? "empty" : "ready");
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
        case "relation_list":
        case "graph_page":
        case "search_page":
        case "context_preview":
        case "activity_page":
        case "search_inspector":
        case "recall_benchmark":
        case "schema_validated":
        case "resource_page":
        case "prompt_page":
        case "tool_inspection":
        case "cli_inventory":
        case "notes_imported":
        case "api_audit":
        case "document_ingested":
        case "cloud_inspection":
        case "sync_inspection":
        case "share_catalog":
        case "hook_inspection":
        case "provider_inspection":
        case "shutdown_begun":
          setError(unexpectedExtrasResponse());
          setCatalog(null);
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
      setCatalog(null);
      setPhase("error");
    }
  };

  const runIngest = async () => {
    if (!isFixtureSourceId(sourceId)) {
      setError({ category: "policy", message: t("extrasSourceDenied") });
      setPhase("error");
      return;
    }
    const route = copyFixtureRoute();
    try {
      const response = await invokeTyped<IpcResponse>({
        command: "ingest_document",
        args: {
          workspace: route.workspace,
          project: route.project,
          source_id: sourceId.trim(),
        },
      });
      switch (response.kind) {
        case "error":
          setError({ category: response.category, message: response.message });
          setIngested(null);
          setPhase("error");
          return;
        case "document_ingested":
          if (
            response.engine_extras ||
            response.official_pdf_office ||
            response.scanned_user_obsidian_vault ||
            (response.extras_enabled && response.files.length === 0 && !response.files_written) ||
            (response.observation.disk_verified && !response.files_written)
          ) {
            setError(unexpectedExtrasResponse());
            setIngested(null);
            setPhase("error");
            return;
          }
          setError(null);
          setIngested({
            source_id: response.source_id,
            extra_id: response.extra_id,
            files: response.files,
            files_written: response.files_written,
            observation: response.observation,
            extras_enabled: response.extras_enabled,
            engine_extras: false,
            official_pdf_office: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
          });
          setPhase(response.files.length === 0 ? "empty" : "ready");
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
        case "relation_list":
        case "graph_page":
        case "search_page":
        case "context_preview":
        case "activity_page":
        case "search_inspector":
        case "recall_benchmark":
        case "schema_validated":
        case "resource_page":
        case "prompt_page":
        case "tool_inspection":
        case "cli_inventory":
        case "notes_imported":
        case "api_audit":
        case "extras_catalog":
        case "cloud_inspection":
        case "sync_inspection":
        case "share_catalog":
        case "hook_inspection":
        case "provider_inspection":
        case "shutdown_begun":
          setError(unexpectedExtrasResponse());
          setIngested(null);
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
      setIngested(null);
      setPhase("error");
    }
  };

  const empty = phase === "empty" && !error;
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error
    ? t("extrasErrorTitle")
    : empty
      ? t("extrasEmptyTitle")
      : t("extrasReadyTitle");

  return (
    <section
      className="panel"
      data-state={state}
      aria-labelledby="extras-title"
      role={error ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h2 id="extras-title">{heading}</h2>
      <p>
        {error
          ? `${errorCategoryLabel(error.category)}：${error.message}`
          : empty
            ? t("extrasEmptyBody")
            : t("extrasReadyBody")}
      </p>
      <p>{t("extrasNotImport")}</p>
      <p>{t("extrasNotOfficial")}</p>
      <p>{t("extrasEnvelopeNote")}</p>
      <ul className="policy-list">
        <li>
          {t("extrasEnabledLabel")}：
          {catalog?.extras_enabled ? t("extrasEnabledTrue") : t("extrasEnabledFalse")}
        </li>
        <li>
          {t("extrasSemanticLabel")}：{t("extrasSemanticFalse")}
        </li>
        <li>
          {t("extrasModelLabel")}：{t("extrasModelFalse")}
        </li>
      </ul>
      <button type="button" className="action" onClick={() => void loadExtras()}>
        {t("extrasInspect")}
      </button>
      <div className="crud-editor">
        <label htmlFor="extras-source-id">{t("extrasSourceLabel")}</label>
        <input
          id="extras-source-id"
          type="text"
          value={sourceId}
          autoComplete="off"
          spellCheck={false}
          onChange={(event) => {
            setSourceId(event.target.value);
            setError(null);
          }}
        />
      </div>
      <button type="button" className="action" onClick={() => void runIngest()}>
        {t("extrasIngest")}
      </button>
      <ul className="policy-list">
        <li>{t("extrasNoVault")}</li>
        <li>
          {t("extrasFilesWrittenLabel")}：
          {ingested?.files_written ? t("extrasWroteFiles") : t("extrasNoWrite")}
        </li>
      </ul>
      {catalog ? (
        <ul className="extras-list">
          {catalog.extras.map((entry: ExtraEntryDto) => (
            <li key={entry.extra_id}>
              <span>{entry.extra_id}</span>
              <span>{entry.kind}</span>
            </li>
          ))}
        </ul>
      ) : null}
      {catalog ? <p>{extrasObservationLabel(catalog.observation.classified_as)}</p> : null}
      {ingested ? (
        <ul className="extras-list">
          {ingested.files.map((file) => (
            <li key={file.identifier}>
              <span>{file.identifier}</span>
              <span>{extrasObservationLabel(ingested.observation.classified_as)}</span>
            </li>
          ))}
        </ul>
      ) : null}
    </section>
  );
}

type CloudError = {
  category: "policy" | "schema" | "unsupported" | "invoke";
  message: string;
};

function unexpectedCloudResponse(): CloudError {
  return { category: "schema", message: t("unexpectedCloud") };
}

function CloudPanel() {
  const [phase, setPhase] = useState<"empty" | "ready" | "error">("empty");
  const [report, setReport] = useState<CloudInspectionDto | null>(null);
  const [error, setError] = useState<CloudError | null>(null);

  const loadCloud = async () => {
    const route = copyFixtureRoute();
    try {
      const response = await invokeTyped<IpcResponse>({
        command: "inspect_cloud",
        args: {
          workspace: route.workspace,
          project: route.project,
        },
      });
      switch (response.kind) {
        case "error":
          setError({ category: response.category, message: response.message });
          setReport(null);
          setPhase("error");
          return;
        case "cloud_inspection":
          if (
            response.cloud_enabled ||
            response.remote_auth ||
            response.credentials_present ||
            response.cloud_allowed ||
            response.connected ||
            response.authenticated ||
            response.cloud_claimed ||
            response.live_official_cloud_session ||
            response.remote_hosts_contacted ||
            response.secrets_stored ||
            response.env_tokens_read ||
            response.mixed_profiles ||
            response.engine_cloud ||
            response.scanned_user_obsidian_vault ||
            response.files_written ||
            !response.local_offline
          ) {
            setError(unexpectedCloudResponse());
            setReport(null);
            setPhase("error");
            return;
          }
          setError(null);
          setReport({
            cloud_enabled: false,
            remote_auth: false,
            credentials_present: false,
            cloud_allowed: false,
            connected: false,
            authenticated: false,
            cloud_claimed: false,
            local_offline: true,
            live_official_cloud_session: false,
            remote_hosts_contacted: false,
            secrets_stored: false,
            env_tokens_read: false,
            mixed_profiles: false,
            observation: response.observation,
            engine_cloud: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            files_written: false,
          });
          setPhase("ready");
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
        case "relation_list":
        case "graph_page":
        case "search_page":
        case "context_preview":
        case "activity_page":
        case "search_inspector":
        case "recall_benchmark":
        case "schema_validated":
        case "resource_page":
        case "prompt_page":
        case "tool_inspection":
        case "cli_inventory":
        case "notes_imported":
        case "api_audit":
        case "extras_catalog":
        case "document_ingested":
        case "shutdown_begun":
        case "sync_inspection":
        case "share_catalog":
        case "hook_inspection":
        case "provider_inspection":
          setError(unexpectedCloudResponse());
          setReport(null);
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
      setReport(null);
      setPhase("error");
    }
  };

  const empty = phase === "empty" && !error;
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error
    ? t("cloudErrorTitle")
    : empty
      ? t("cloudEmptyTitle")
      : t("cloudReadyTitle");
  const unauthorized =
    error?.category === "policy" || error?.category === "unsupported";

  return (
    <section
      className="panel"
      data-state={state}
      aria-labelledby="cloud-title"
      role={error ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h2 id="cloud-title">{heading}</h2>
      <p>
        {error
          ? `${errorCategoryLabel(error.category)}：${error.message}`
          : empty
            ? t("cloudEmptyBody")
            : t("cloudReadyBody")}
      </p>
      <p>{unauthorized ? t("cloudUnauthorized") : t("cloudNotConnected")}</p>
      <ul className="policy-list">
        <li>
          {t("cloudEnabledLabel")}：{t("cloudEnabledFalse")}
        </li>
        <li>
          {t("cloudRemoteAuthLabel")}：{t("cloudRemoteAuthFalse")}
        </li>
        <li>
          {t("cloudCredentialsLabel")}：{t("cloudCredentialsFalse")}
        </li>
        <li>
          {t("cloudAllowedLabel")}：{t("cloudAllowedFalse")}
        </li>
        <li>
          {t("cloudConnectedLabel")}：{t("cloudNotConnected")}
        </li>
        <li>
          {t("cloudAuthenticatedLabel")}：{t("cloudUnauthorized")}
        </li>
      </ul>
      <button type="button" className="action" onClick={() => void loadCloud()}>
        {t("cloudInspect")}
      </button>
      <ul className="policy-list">
        <li>{t("cloudNoVault")}</li>
        <li>{t("cloudNoSecrets")}</li>
        <li>{t("cloudNoRemote")}</li>
        <li>{t("cloudNotOfficial")}</li>
        <li>
          {t("cloudFilesWrittenLabel")}：
          {report?.files_written ? t("cloudWroteFiles") : t("cloudNoWrite")}
        </li>
      </ul>
      {report ? (
        <ul className="cloud-list">
          <li>
            <span>{t("cloudLocalOfflineLabel")}</span>
            <span>{report.local_offline ? t("cloudYes") : t("cloudNo")}</span>
          </li>
          <li>
            <span>{t("cloudObservationLabel")}</span>
            <span>
              {report.observation.classified_as === "empty"
                ? t("cloudObservationEmpty")
                : t("cloudObservationUnverified")}
            </span>
          </li>
        </ul>
      ) : null}
    </section>
  );
}

type SyncError = {
  category: "policy" | "schema" | "unsupported" | "invoke";
  message: string;
};

function unexpectedSyncResponse(): SyncError {
  return { category: "schema", message: t("unexpectedSync") };
}

function SyncPanel() {
  const [phase, setPhase] = useState<"empty" | "ready" | "error">("empty");
  const [report, setReport] = useState<SyncInspectionDto | null>(null);
  const [shares, setShares] = useState<ShareCatalogDto | null>(null);
  const [error, setError] = useState<SyncError | null>(null);

  const loadSync = async () => {
    const route = copyFixtureRoute();
    try {
      const syncResponse = await invokeTyped<IpcResponse>({
        command: "inspect_sync",
        args: {
          workspace: route.workspace,
          project: route.project,
        },
      });
      switch (syncResponse.kind) {
        case "error":
          setError({ category: syncResponse.category, message: syncResponse.message });
          setReport(null);
          setShares(null);
          setPhase("error");
          return;
        case "sync_inspection":
          if (
            syncResponse.sync_enabled ||
            syncResponse.sharing_enabled ||
            syncResponse.remote_restore ||
            syncResponse.last_sync !== "none" ||
            syncResponse.synced ||
            syncResponse.shared ||
            syncResponse.sync_claimed ||
            syncResponse.live_official_cloud_session ||
            syncResponse.remote_hosts_contacted ||
            syncResponse.secrets_stored ||
            syncResponse.env_tokens_read ||
            syncResponse.mixed_profiles ||
            syncResponse.engine_sync ||
            syncResponse.scanned_user_obsidian_vault ||
            syncResponse.files_written ||
            !syncResponse.local_offline
          ) {
            setError(unexpectedSyncResponse());
            setReport(null);
            setShares(null);
            setPhase("error");
            return;
          }
          break;
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
        case "relation_list":
        case "graph_page":
        case "search_page":
        case "context_preview":
        case "activity_page":
        case "search_inspector":
        case "recall_benchmark":
        case "schema_validated":
        case "resource_page":
        case "prompt_page":
        case "tool_inspection":
        case "cli_inventory":
        case "notes_imported":
        case "api_audit":
        case "extras_catalog":
        case "document_ingested":
        case "cloud_inspection":
        case "share_catalog":
        case "hook_inspection":
        case "provider_inspection":
        case "shutdown_begun":
          setError(unexpectedSyncResponse());
          setReport(null);
          setShares(null);
          setPhase("error");
          return;
        default: {
          const exhaustive: never = syncResponse;
          return exhaustive;
        }
      }

      const shareResponse = await invokeTyped<IpcResponse>({
        command: "list_shares",
        args: {
          workspace: route.workspace,
          project: route.project,
        },
      });
      switch (shareResponse.kind) {
        case "error":
          setError({ category: shareResponse.category, message: shareResponse.message });
          setReport(null);
          setShares(null);
          setPhase("error");
          return;
        case "share_catalog":
          if (
            shareResponse.sharing_enabled ||
            shareResponse.share_claimed ||
            shareResponse.live_shared_remote ||
            shareResponse.remote_restore ||
            shareResponse.live_official_cloud_session ||
            shareResponse.remote_hosts_contacted ||
            shareResponse.secrets_stored ||
            shareResponse.env_tokens_read ||
            shareResponse.mixed_profiles ||
            shareResponse.engine_share ||
            shareResponse.scanned_user_obsidian_vault ||
            shareResponse.files_written ||
            shareResponse.shares.length > 0 ||
            !shareResponse.local_offline
          ) {
            setError(unexpectedSyncResponse());
            setReport(null);
            setShares(null);
            setPhase("error");
            return;
          }
          setError(null);
          setReport({
            sync_enabled: false,
            sharing_enabled: false,
            remote_restore: false,
            last_sync: "none",
            synced: false,
            shared: false,
            sync_claimed: false,
            local_offline: true,
            live_official_cloud_session: false,
            remote_hosts_contacted: false,
            secrets_stored: false,
            env_tokens_read: false,
            mixed_profiles: false,
            observation: syncResponse.observation,
            engine_sync: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            files_written: false,
          });
          setShares({
            shares: [],
            sharing_enabled: false,
            share_claimed: false,
            live_shared_remote: false,
            remote_restore: false,
            local_offline: true,
            live_official_cloud_session: false,
            remote_hosts_contacted: false,
            secrets_stored: false,
            env_tokens_read: false,
            mixed_profiles: false,
            observation: shareResponse.observation,
            engine_share: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            files_written: false,
          });
          setPhase("ready");
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
        case "relation_list":
        case "graph_page":
        case "search_page":
        case "context_preview":
        case "activity_page":
        case "search_inspector":
        case "recall_benchmark":
        case "schema_validated":
        case "resource_page":
        case "prompt_page":
        case "tool_inspection":
        case "cli_inventory":
        case "notes_imported":
        case "api_audit":
        case "extras_catalog":
        case "document_ingested":
        case "cloud_inspection":
        case "sync_inspection":
        case "hook_inspection":
        case "provider_inspection":
        case "shutdown_begun":
          setError(unexpectedSyncResponse());
          setReport(null);
          setShares(null);
          setPhase("error");
          return;
        default: {
          const exhaustive: never = shareResponse;
          return exhaustive;
        }
      }
    } catch (cause) {
      setError({
        category: "invoke",
        message: cause instanceof Error ? cause.message : String(cause),
      });
      setReport(null);
      setShares(null);
      setPhase("error");
    }
  };

  const empty = phase === "empty" && !error;
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error
    ? t("syncErrorTitle")
    : empty
      ? t("syncEmptyTitle")
      : t("syncReadyTitle");

  return (
    <section
      className="panel"
      data-state={state}
      aria-labelledby="sync-title"
      role={error ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h2 id="sync-title">{heading}</h2>
      <p>
        {error
          ? `${errorCategoryLabel(error.category)}：${error.message}`
          : empty
            ? t("syncEmptyBody")
            : t("syncReadyBody")}
      </p>
      <p>{t("syncNotSynced")}</p>
      <p>{t("syncNotShared")}</p>
      <ul className="policy-list">
        <li>
          {t("syncEnabledLabel")}：{t("syncEnabledFalse")}
        </li>
        <li>
          {t("sharingEnabledLabel")}：{t("sharingEnabledFalse")}
        </li>
        <li>
          {t("remoteRestoreLabel")}：{t("remoteRestoreFalse")}
        </li>
        <li>
          {t("lastSyncLabel")}：{t("lastSyncNone")}
        </li>
      </ul>
      <button type="button" className="action" onClick={() => void loadSync()}>
        {t("syncInspect")}
      </button>
      <ul className="policy-list">
        <li>{t("syncNotMaintenance")}</li>
        <li>{t("syncNoVault")}</li>
        <li>{t("syncNoSecrets")}</li>
        <li>{t("syncNoRemote")}</li>
        <li>{t("syncNotOfficial")}</li>
        <li>
          {t("syncFilesWrittenLabel")}：
          {report?.files_written || shares?.files_written ? t("syncWroteFiles") : t("syncNoWrite")}
        </li>
      </ul>
      {report ? (
        <ul className="sync-list">
          <li>
            <span>{t("syncLocalOfflineLabel")}</span>
            <span>{report.local_offline ? t("syncYes") : t("syncNo")}</span>
          </li>
          <li>
            <span>{t("syncObservationLabel")}</span>
            <span>
              {report.observation.classified_as === "empty"
                ? t("syncObservationEmpty")
                : t("syncObservationUnverified")}
            </span>
          </li>
        </ul>
      ) : null}
      {shares ? (
        <ul className="share-list">
          <li>
            <span>{t("shareCatalogLabel")}</span>
            <span>{t("syncNotShared")}</span>
          </li>
        </ul>
      ) : null}
    </section>
  );
}

type HookError = {
  category: "policy" | "schema" | "unsupported" | "invoke";
  message: string;
};

function unexpectedHookResponse(): HookError {
  return { category: "schema", message: t("unexpectedHooks") };
}

function HookPanel() {
  const [phase, setPhase] = useState<"empty" | "ready" | "error">("empty");
  const [report, setReport] = useState<HookInspectionDto | null>(null);
  const [error, setError] = useState<HookError | null>(null);

  const loadHooks = async () => {
    const route = copyFixtureRoute();
    try {
      const response = await invokeTyped<IpcResponse>({
        command: "inspect_hooks",
        args: {
          workspace: route.workspace,
          project: route.project,
        },
      });
      switch (response.kind) {
        case "error":
          setError({ category: response.category, message: response.message });
          setReport(null);
          setPhase("error");
          return;
        case "hook_inspection":
          if (
            response.hooks_enabled ||
            response.agent_connected ||
            response.files_written ||
            response.installed ||
            response.hook_claimed ||
            response.live_official_agent_session ||
            response.remote_hosts_contacted ||
            response.secrets_stored ||
            response.env_tokens_read ||
            response.mixed_profiles ||
            response.engine_hooks ||
            response.scanned_user_obsidian_vault ||
            response.scanned_cursor_rules ||
            response.scanned_user_agent_config ||
            response.hooks.length > 0 ||
            !response.local_offline
          ) {
            setError(unexpectedHookResponse());
            setReport(null);
            setPhase("error");
            return;
          }
          setError(null);
          setReport({
            hooks: [],
            hooks_enabled: false,
            agent_connected: false,
            files_written: false,
            installed: false,
            hook_claimed: false,
            local_offline: true,
            live_official_agent_session: false,
            remote_hosts_contacted: false,
            secrets_stored: false,
            env_tokens_read: false,
            mixed_profiles: false,
            observation: response.observation,
            engine_hooks: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            scanned_cursor_rules: false,
            scanned_user_agent_config: false,
          });
          setPhase("ready");
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
        case "relation_list":
        case "graph_page":
        case "search_page":
        case "context_preview":
        case "activity_page":
        case "search_inspector":
        case "recall_benchmark":
        case "schema_validated":
        case "resource_page":
        case "prompt_page":
        case "tool_inspection":
        case "cli_inventory":
        case "notes_imported":
        case "api_audit":
        case "extras_catalog":
        case "document_ingested":
        case "cloud_inspection":
        case "sync_inspection":
        case "share_catalog":
        case "provider_inspection":
        case "shutdown_begun":
          setError(unexpectedHookResponse());
          setReport(null);
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
      setReport(null);
      setPhase("error");
    }
  };

  const empty = phase === "empty" && !error;
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error
    ? t("hooksErrorTitle")
    : empty
      ? t("hooksEmptyTitle")
      : t("hooksReadyTitle");
  const disconnected =
    error?.category === "policy" || error?.category === "unsupported";

  return (
    <section
      className="panel"
      data-state={state}
      aria-labelledby="hooks-title"
      role={error ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h2 id="hooks-title">{heading}</h2>
      <p>
        {error
          ? `${errorCategoryLabel(error.category)}：${error.message}`
          : empty
            ? t("hooksEmptyBody")
            : t("hooksReadyBody")}
      </p>
      <p>{disconnected ? t("hooksNotConnected") : t("hooksNotEnabled")}</p>
      <ul className="policy-list">
        <li>
          {t("hooksEnabledLabel")}：{t("hooksEnabledFalse")}
        </li>
        <li>
          {t("agentConnectedLabel")}：{t("hooksNotConnected")}
        </li>
        <li>
          {t("hooksInstalledLabel")}：{t("hooksNotEnabled")}
        </li>
        <li>
          {t("hooksCatalogLabel")}：{t("hooksCatalogEmpty")}
        </li>
      </ul>
      <button type="button" className="action" onClick={() => void loadHooks()}>
        {t("hooksInspect")}
      </button>
      <ul className="policy-list">
        <li>{t("hooksNoVault")}</li>
        <li>{t("hooksNoCursorRules")}</li>
        <li>{t("hooksNoSecrets")}</li>
        <li>{t("hooksNoRemote")}</li>
        <li>{t("hooksNotOfficial")}</li>
        <li>
          {t("hooksFilesWrittenLabel")}：
          {report?.files_written ? t("hooksWroteFiles") : t("hooksNoWrite")}
        </li>
      </ul>
      {report ? (
        <ul className="hook-list">
          <li>
            <span>{t("hooksLocalOfflineLabel")}</span>
            <span>{report.local_offline ? t("hooksYes") : t("hooksNo")}</span>
          </li>
          <li>
            <span>{t("hooksObservationLabel")}</span>
            <span>
              {report.observation.classified_as === "empty"
                ? t("hooksObservationEmpty")
                : t("hooksObservationUnverified")}
            </span>
          </li>
        </ul>
      ) : null}
    </section>
  );
}

type ProviderError = {
  category: "policy" | "schema" | "unsupported" | "invoke";
  message: string;
};

function unexpectedProviderResponse(): ProviderError {
  return { category: "schema", message: t("unexpectedProviders") };
}

function ProviderPanel() {
  const [phase, setPhase] = useState<"empty" | "ready" | "error">("empty");
  const [report, setReport] = useState<ProviderInspectionDto | null>(null);
  const [error, setError] = useState<ProviderError | null>(null);

  const loadProviders = async () => {
    const route = copyFixtureRoute();
    try {
      const response = await invokeTyped<IpcResponse>({
        command: "inspect_providers",
        args: {
          workspace: route.workspace,
          project: route.project,
        },
      });
      switch (response.kind) {
        case "error":
          setError({ category: response.category, message: response.message });
          setReport(null);
          setPhase("error");
          return;
        case "provider_inspection":
          if (
            response.provider_enabled ||
            response.semantic_enabled ||
            response.model_loaded ||
            response.files_written ||
            response.connected ||
            response.provider_claimed ||
            response.live_provider_session ||
            response.official_semantic ||
            response.official_search ||
            response.official_fetch ||
            !response.search_fetch_distinct ||
            response.cloud_credential_route ||
            response.remote_hosts_contacted ||
            response.secrets_stored ||
            response.env_tokens_read ||
            response.mixed_profiles ||
            response.engine_providers ||
            response.scanned_user_obsidian_vault ||
            response.providers.length > 0 ||
            response.embedding_backend !== "none" ||
            response.backend_tier !== "disabled" ||
            !response.local_offline ||
            response.unavailable.some(
              (provider) => provider.verified || provider.status !== "unavailable",
            )
          ) {
            setError(unexpectedProviderResponse());
            setReport(null);
            setPhase("error");
            return;
          }
          setError(null);
          setReport({
            providers: [],
            unavailable: response.unavailable,
            provider_enabled: false,
            semantic_enabled: false,
            model_loaded: false,
            embedding_backend: "none",
            backend_tier: "disabled",
            files_written: false,
            connected: false,
            provider_claimed: false,
            local_offline: true,
            live_provider_session: false,
            official_semantic: false,
            official_search: false,
            official_fetch: false,
            search_fetch_distinct: true,
            cloud_credential_route: false,
            remote_hosts_contacted: false,
            secrets_stored: false,
            env_tokens_read: false,
            mixed_profiles: false,
            observation: response.observation,
            engine_providers: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
          });
          setPhase("ready");
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
        case "relation_list":
        case "graph_page":
        case "search_page":
        case "context_preview":
        case "activity_page":
        case "search_inspector":
        case "recall_benchmark":
        case "schema_validated":
        case "resource_page":
        case "prompt_page":
        case "tool_inspection":
        case "cli_inventory":
        case "notes_imported":
        case "api_audit":
        case "extras_catalog":
        case "document_ingested":
        case "cloud_inspection":
        case "sync_inspection":
        case "share_catalog":
        case "hook_inspection":
        case "shutdown_begun":
          setError(unexpectedProviderResponse());
          setReport(null);
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
      setReport(null);
      setPhase("error");
    }
  };

  const empty = phase === "empty" && !error;
  const state = error ? "error" : empty ? "empty" : "status";
  const badge = error ? t("errorBadge") : empty ? t("emptyBadge") : t("statusBadge");
  const heading = error
    ? t("providersErrorTitle")
    : empty
      ? t("providersEmptyTitle")
      : t("providersReadyTitle");

  return (
    <section
      className="panel"
      data-state={state}
      aria-labelledby="providers-title"
      role={error ? "alert" : undefined}
    >
      <p className="state-badge">{badge}</p>
      <h2 id="providers-title">{heading}</h2>
      <p>
        {error
          ? `${errorCategoryLabel(error.category)}：${error.message}`
          : empty
            ? t("providersEmptyBody")
            : t("providersReadyBody")}
      </p>
      <p>{t("providersNotEnabled")}</p>
      <ul className="policy-list">
        <li>
          {t("providerEnabledLabel")}：{t("providerEnabledFalse")}
        </li>
        <li>
          {t("providersSemanticLabel")}：{t("providersSemanticFalse")}
        </li>
        <li>
          {t("providersModelLabel")}：{t("providersModelFalse")}
        </li>
        <li>
          {t("providersCatalogLabel")}：{t("providersCatalogEmpty")}
        </li>
        <li>
          {t("providersOpenaiLabel")}：{t("providersUnavailable")}
        </li>
        <li>
          {t("providersAnthropicLabel")}：{t("providersUnavailable")}
        </li>
        <li>
          {t("providersHuggingfaceLabel")}：{t("providersUnavailable")}
        </li>
        <li>
          {t("providersCloudEmbeddingsLabel")}：{t("providersUnavailable")}
        </li>
      </ul>
      <button type="button" className="action" onClick={() => void loadProviders()}>
        {t("providersInspect")}
      </button>
      <ul className="policy-list">
        <li>{t("providersNoVault")}</li>
        <li>{t("providersNoSecrets")}</li>
        <li>{t("providersNoRemote")}</li>
        <li>{t("providersNoCloudRoute")}</li>
        <li>{t("providersNotOfficial")}</li>
        <li>
          {t("providersFilesWrittenLabel")}：
          {report?.files_written ? t("providersWroteFiles") : t("providersNoWrite")}
        </li>
      </ul>
      {report ? (
        <ul className="provider-list">
          <li>
            <span>{t("providersLocalOfflineLabel")}</span>
            <span>{report.local_offline ? t("providersYes") : t("providersNo")}</span>
          </li>
          <li>
            <span>{t("providersObservationLabel")}</span>
            <span>
              {report.observation.classified_as === "empty"
                ? t("providersObservationEmpty")
                : t("providersObservationUnverified")}
            </span>
          </li>
          {report.unavailable.map((provider) => (
            <li key={provider.identifier}>
              <span>{provider.identifier}</span>
              <span>{t("providersUnavailable")}</span>
            </li>
          ))}
        </ul>
      ) : null}
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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
      case "schema_validated":
      case "resource_page":
      case "prompt_page":
      case "tool_inspection":
      case "cli_inventory":
      case "notes_imported":
      case "api_audit":
      case "extras_catalog":
      case "document_ingested":
      case "cloud_inspection":
      case "sync_inspection":
      case "share_catalog":
      case "hook_inspection":
      case "provider_inspection":
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
