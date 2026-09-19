import { type IpcResponse, type IpcCommand, type invokeTyped } from "../src/ipc";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import App, { NotePreview, SearchPanel, ContentSafetyFacts, DemandPanel } from "../src/App";
import type { EngineSearchPageDto } from "../src/ipc";
export { SearchWindow, restoreSearchFocus } from "../src/searchWindow";
export { captureContentDiagnostics, diagnosticIsCurrent } from "../src/contentSafety";
export { openNote, runSearch, loadMoreSearch, loadTools, loadMoreTree, runEditNote, loadContextPreview, loadMoreCli, loadGraph, loadMoreGraph, loadTree } from "../src/App";
export { WorkbenchRequests } from "../src/workbenchRequests";
export { EditSessions } from "../src/editSessions";
export { deliverPrimaryNote } from "../src/App";
export { deferredResponse } from "./shell-fixtures";
import { deferredResponse } from "./shell-fixtures";

export const unsupported = { kind: "error", category: "unsupported", message: "Not available" } satisfies IpcResponse;
const observation = { classified_as: "empty", disk_verified: false, envelope_is_not_disk_proof: true } as const;
export function note(identifier: string): IpcResponse {
  return { kind: "note_read", identifier, title: identifier, body: "中文\r\n" + identifier, observation };
}
export function page(query: string, cursor: string | null = null): Extract<IpcResponse, { kind: "search_page" }> {
  return {
    kind: "search_page", query, hits: [{ identifier: query, lexical_score: 1, semantic_score: 0 }],
    next_cursor: cursor, page: cursor ? 1 : 2, truncated: false, observation,
    semantic_enabled: false, engine_search: false, scanned_user_obsidian_vault: false,
    scanned_user_basic_memory_home: false, files_written: false,
  };
}
export function saved(identifier: string, body: string): IpcResponse {
  return {
    kind: "draft_saved", identifier, body, files_written: true, engine_persisted: false,
    scanned_user_obsidian_vault: false, scanned_user_basic_memory_home: false,
    observation: { classified_as: "disk_verified", disk_verified: true, envelope_is_not_disk_proof: true },
  };
}
export function cli(profile: "release" | "main-preview"): Extract<IpcResponse, { kind: "cli_inventory" }> {
  return { kind: "cli_inventory", profile_id: profile, leaves: [{ path: [profile], executed: false }],
    next_cursor: "next", page: 1, truncated: false, observation, engine_cli: false, executed: false,
    mixed_profiles: false, scanned_user_obsidian_vault: false, scanned_user_basic_memory_home: false, files_written: false };
}
export function graph(identifier: string): Extract<IpcResponse, { kind: "graph_page" }> {
  return { kind: "graph_page", identifier, nodes: [{ identifier, classified_as: "present" }], edges: [],
    next_cursor: "next", page: 1, truncated: false, observation, engine_graph: false,
    scanned_user_obsidian_vault: false, scanned_user_basic_memory_home: false, files_written: false, depth: 1 };
}
export function renderNoteSource(body: string, identifier = "fixture/a.md") {
  return renderToStaticMarkup(createElement(NotePreview, { note: { identifier, title: "笔记", body, observation } }));
}
export function renderAppShell() { return renderToStaticMarkup(createElement(App)); }
export function enginePage(pageNumber: number, count = 50): EngineSearchPageDto {
  return { engine_search: true, session: { profile: "release", generation: 1 },
    request: { session: { profile: "release", generation: 1 }, workspace: "bmdock-workspace", project: "bmdock-fixture", operation: "search_notes", arguments: { query: "test" }, request_generation: 1 },
    query: "test", page: pageNumber, page_size: 50, next_cursor: pageNumber < 8 ? String(pageNumber + 1) : null, has_more: pageNumber < 8, total: 400, total_is_exact: true,
    hits: Array.from({ length: count }, (_, index) => ({ identifier: `result-${pageNumber}-${index}`, note_identifier: "shared-owner-uuid", result_kind: "observation", title: `Title ${index}`, excerpt: "<script>escaped snippet</script>", score: -0.5,
      file_path: "folder/note.md", category: "fact", relation_type: null, from_entity: null, to_entity: null, to_name: null, created_at: null })) };
}
export function renderEngineSearch(page: EngineSearchPageDto, selected = page.hits[0]?.identifier) {
  return renderToStaticMarkup(createElement(SearchPanel, { selectedIdentifier: "returned/permalink", selectedResultIdentifier: selected, pending: false, search: page,
    error: null, onSearch: () => {}, onPreview: () => {}, onLoadMore: () => {} }));
}
export function renderClosedDiagnostics(body: string, revision: string, inspect: (body: string) => ReturnType<typeof import("../src/contentSafety").classifyBody>) {
  return renderToStaticMarkup(createElement(ContentSafetyFacts, { body, revision, previewId: "closed-check", inspect }));
}
export function renderClosedDemand(onLoad: () => void) {
  return renderToStaticMarkup(createElement(DemandPanel, { title: "resources", pending: false, onLoad, children: "unmounted details" }));
}
export function renderSearchSelection(selectedIdentifier: string | null) {
  return renderToStaticMarkup(createElement(SearchPanel, { selectedIdentifier, pending: false, search: page("result-note"),
    error: null, onSearch: () => {}, onPreview: () => {}, onLoadMore: () => {} }));
}
export function transport() {
  const calls: IpcCommand[] = [];
  const pending = deferredResponse();
  const invoke: typeof invokeTyped = async <T extends IpcResponse>(command: IpcCommand) => {
    calls.push(command);
    // All fixtures are checked IPC values; generic narrowing is the transport's responsibility.
    return (calls.length === 1 ? await pending.promise : unsupported) as T;
  };
  return { calls, pending, invoke };
}
