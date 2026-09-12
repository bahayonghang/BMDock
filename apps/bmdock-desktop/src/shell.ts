import {
  invokeTyped,
  type CapabilitiesDto,
  type ConfigDiscoveryDto,
  type DrainPhase,
  type ErrorCategory,
  type FailureKind,
  type IpcResponse,
  type PreflightDto,
  type ProjectCatalogDto,
  type RuntimeStateDto,
  type RuntimeStatus,
} from "./ipc";
import { t } from "./i18n";

export type ShellLoadState =
  | { phase: "loading" }
  | {
      phase: "ready";
      capabilities: CapabilitiesDto;
      runtime: RuntimeStateDto;
      catalog: ProjectCatalogDto;
    }
  | { phase: "error"; category: ErrorCategory | "invoke"; message: string };

export type PreflightLoadState =
  | { phase: "loading" }
  | { phase: "ready"; preflight: PreflightDto; discovery: ConfigDiscoveryDto }
  | { phase: "error"; category: ErrorCategory | "invoke"; message: string };

export async function readShellSnapshot(): Promise<Exclude<ShellLoadState, { phase: "loading" }>> {
  try {
    const capabilitiesResponse = await invokeTyped<IpcResponse>({
      command: "get_capabilities",
      args: {},
    });
    if (capabilitiesResponse.kind === "error") {
      return {
        phase: "error",
        category: capabilitiesResponse.category,
        message: capabilitiesResponse.message,
      };
    }
    if (capabilitiesResponse.kind !== "capabilities") {
      return {
        phase: "error",
        category: "schema",
        message: t("unexpectedCapabilities"),
      };
    }

    const runtimeResponse = await invokeTyped<IpcResponse>({
      command: "get_runtime_state",
      args: {},
    });
    if (runtimeResponse.kind === "error") {
      return {
        phase: "error",
        category: runtimeResponse.category,
        message: runtimeResponse.message,
      };
    }
    if (runtimeResponse.kind !== "runtime_state") {
      return {
        phase: "error",
        category: "schema",
        message: t("unexpectedRuntime"),
      };
    }

    const catalogResponse = await invokeTyped<IpcResponse>({
      command: "list_projects",
      args: {},
    });
    if (catalogResponse.kind === "error") {
      return {
        phase: "error",
        category: catalogResponse.category,
        message: catalogResponse.message,
      };
    }
    if (catalogResponse.kind !== "project_catalog") {
      return {
        phase: "error",
        category: "schema",
        message: t("unexpectedCatalog"),
      };
    }

    return {
      phase: "ready",
      capabilities: {
        commands: capabilitiesResponse.commands,
        events: capabilitiesResponse.events,
        policy: capabilitiesResponse.policy,
      },
      runtime: {
        status: runtimeResponse.status,
        project: runtimeResponse.project,
        profile: runtimeResponse.profile,
        failure: runtimeResponse.failure,
        shutdown: runtimeResponse.shutdown,
        host_drain: runtimeResponse.host_drain,
        semantic_model_loaded: false,
      },
      catalog: {
        workspaces: catalogResponse.workspaces,
        projects: catalogResponse.projects,
        scanned_user_obsidian_vault: catalogResponse.scanned_user_obsidian_vault,
        scanned_user_basic_memory_home: catalogResponse.scanned_user_basic_memory_home,
        cross_project_search_allowed: catalogResponse.cross_project_search_allowed,
        implicit_current_project_writes: catalogResponse.implicit_current_project_writes,
        cloud_or_credential_required: catalogResponse.cloud_or_credential_required,
        local_offline: catalogResponse.local_offline,
        files_written: catalogResponse.files_written,
      },
    };
  } catch (cause) {
    return {
      phase: "error",
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    };
  }
}

export async function readPreflightSnapshot(): Promise<Exclude<PreflightLoadState, { phase: "loading" }>> {
  try {
    const preflightResponse = await invokeTyped<IpcResponse>({
      command: "run_preflight",
      args: {},
    });
    if (preflightResponse.kind === "error") {
      return {
        phase: "error",
        category: preflightResponse.category,
        message: preflightResponse.message,
      };
    }
    if (preflightResponse.kind !== "preflight") {
      return {
        phase: "error",
        category: "schema",
        message: t("unexpectedPreflight"),
      };
    }

    const discoveryResponse = await invokeTyped<IpcResponse>({
      command: "discover_config",
      args: {},
    });
    if (discoveryResponse.kind === "error") {
      return {
        phase: "error",
        category: discoveryResponse.category,
        message: discoveryResponse.message,
      };
    }
    if (discoveryResponse.kind !== "config_discovery") {
      return {
        phase: "error",
        category: "schema",
        message: t("unexpectedDiscovery"),
      };
    }

    return {
      phase: "ready",
      preflight: {
        profiles: preflightResponse.profiles,
        host: preflightResponse.host,
      },
      discovery: {
        root: discoveryResponse.root,
        candidates: discoveryResponse.candidates,
        scanned_user_basic_memory_home: discoveryResponse.scanned_user_basic_memory_home,
        copied_or_rewrote_production_config: discoveryResponse.copied_or_rewrote_production_config,
      },
    };
  } catch (cause) {
    return {
      phase: "error",
      category: "invoke",
      message: cause instanceof Error ? cause.message : String(cause),
    };
  }
}

export function runtimeStatusLabel(status: RuntimeStatus): string {
  switch (status) {
    case "not_started":
      return t("statusNotStarted");
    case "starting":
      return t("statusStarting");
    case "connected":
      return t("statusConnected");
    case "stopping":
      return t("statusStopping");
    case "stopped":
      return t("statusStopped");
    case "failed":
      return t("statusFailed");
    default: {
      const exhaustive: never = status;
      return exhaustive;
    }
  }
}

export function drainPhaseLabel(phase: DrainPhase): string {
  switch (phase) {
    case "idle":
      return t("runtimeDrainIdle");
    case "draining":
      return t("runtimeDrainDraining");
    case "drained":
      return t("runtimeDrainDrained");
    default: {
      const exhaustive: never = phase;
      return exhaustive;
    }
  }
}

export function failureKindLabel(failure: FailureKind): string {
  switch (failure) {
    case "policy":
      return t("failurePolicy");
    case "transport":
      return t("failureTransport");
    case "timeout_unknown":
      return t("failureTimeoutUnknown");
    case "process":
      return t("failureProcess");
    case "unverified":
      return t("failureUnverified");
    default: {
      const exhaustive: never = failure;
      return exhaustive;
    }
  }
}

export function errorCategoryLabel(category: ErrorCategory | "invoke"): string {
  switch (category) {
    case "policy":
      return t("failurePolicy");
    case "schema":
      return t("errorSchema");
    case "unsupported":
      return t("errorUnsupported");
    case "invoke":
      return t("invokeFailure");
    default: {
      const exhaustive: never = category;
      return exhaustive;
    }
  }
}
