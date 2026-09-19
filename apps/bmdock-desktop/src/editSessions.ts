import { copyFixtureRoute, invokeTyped, type IpcCommand, type IpcResponse, type DraftResultDto, type NoteWriteDto, type NoteEditDto, type NoteMoveDto, type NoteDeleteDto } from "./ipc";
import { t } from "./i18n";

export type EditorError = { category: "policy" | "schema" | "unsupported" | "invoke"; message: string };
export type CrudResult =
  | ({ kind: "note_written" } & NoteWriteDto) | ({ kind: "note_edited" } & NoteEditDto)
  | ({ kind: "note_moved" } & NoteMoveDto) | ({ kind: "note_deleted" } & NoteDeleteDto);
export type EditorState = {
  identifier: string; title: string; body: string; destination: string; confirmDelete: boolean;
  diskBody: string | null; draftResult: DraftResultDto | null; crudResult: CrudResult | null;
  error: EditorError | null; pending: boolean; revision: number;
};

/** App-lifetime editor text only. No storage, persistence inference or eviction. */
export class EditSessions {
  private sessions = new Map<string, EditorState>();
  private listeners = new Set<() => void>();
  subscribe = (listener: () => void) => { this.listeners.add(listener); return () => this.listeners.delete(listener); };
  get(key: string, seed: { identifier: string; title?: string; body: string }): EditorState {
    if (!this.sessions.has(key)) this.sessions.set(key, {
      ...seed, title: seed.title ?? "", destination: "", confirmDelete: false,
      diskBody: null, draftResult: null, crudResult: null, error: null, pending: false, revision: 0,
    });
    return this.sessions.get(key)!;
  }
  update(key: string, patch: Partial<EditorState>) {
    const current = this.sessions.get(key);
    if (!current) return;
    const editing = (patch.body !== undefined && patch.body !== current.body) ||
      (patch.identifier !== undefined && patch.identifier !== current.identifier) ||
      (patch.title !== undefined && patch.title !== current.title);
    this.sessions.set(key, { ...current, ...patch, revision: current.revision + Number(editing),
      ...(patch.identifier !== undefined && patch.identifier !== current.identifier ? { confirmDelete: false } : {}),
    });
    this.listeners.forEach((listener) => listener());
  }
  discard(key: string, seed: { identifier: string; title?: string; body: string }) {
    if (this.sessions.get(key)?.pending) return;
    this.sessions.delete(key);
    this.get(key, seed);
    this.listeners.forEach((listener) => listener());
  }
  async run(key: string, work: (submitted: EditorState) => Promise<void>) {
    const submitted = this.sessions.get(key);
    if (!submitted || submitted.pending) return;
    this.update(key, { pending: true, error: null });
    try { await work(submitted); } finally { this.update(key, { pending: false }); }
  }
  async draft(key: string, operation: "save_draft" | "load_draft", invoke: (command: IpcCommand) => Promise<IpcResponse> = invokeTyped) {
    await this.run(key, async (submitted) => {
      try {
        const args = { ...copyFixtureRoute(), identifier: submitted.identifier };
        const response = await invoke(operation === "save_draft"
          ? { command: "save_draft", args: { ...args, body: submitted.body } }
          : { command: "load_draft", args });
        if (this.sessions.get(key)?.identifier !== submitted.identifier) return;
        if (response.kind === "error") { this.update(key, { error: response }); return; }
        if (response.kind !== (operation === "save_draft" ? "draft_saved" : "draft_loaded") ||
            (response.kind !== "draft_saved" && response.kind !== "draft_loaded") || response.identifier !== submitted.identifier) {
          this.update(key, { error: { category: "schema", message: t("unexpectedDraft") } }); return;
        }
        const current = this.sessions.get(key)!;
        // A changed target has no relationship to this acknowledgement.
        if (current.identifier !== submitted.identifier) return;
        const verified = response.observation.disk_verified && !response.engine_persisted;
        this.update(key, {
          draftResult: response,
          diskBody: verified ? response.body : current.diskBody,
          // Explicit reload may replace only the revision that requested it.
          ...(operation === "load_draft" && current.revision === submitted.revision ? { body: response.body } : {}),
        });
      } catch (cause) {
        if (this.sessions.get(key)?.identifier !== submitted.identifier) return;
        this.update(key, { error: { category: "invoke", message: cause instanceof Error ? cause.message : String(cause) } });
      }
    });
  }
}
