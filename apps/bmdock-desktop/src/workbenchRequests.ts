/** Owns pending work for one mounted workbench. Invalidation is logical only. */
export class WorkbenchRequests {
  private slots = new Map<string, { identity: string; token: object }>();
  private listeners = new Set<() => void>();
  private pending: readonly string[] = [];
  private lifetime = 0;
  private detailIntent = 0;
  generation = () => this.lifetime;
  subscribe = (listener: () => void) => { this.listeners.add(listener); return () => this.listeners.delete(listener); };
  snapshot = () => this.pending;
  private changed() {
    this.pending = [...this.slots.keys()];
    this.listeners.forEach((listener) => listener());
  }
  invalidate(...operations: string[]) {
    if (operations.length === 0) ++this.lifetime;
    for (const operation of operations.length ? operations : [...this.slots.keys()]) this.slots.delete(operation);
    this.changed();
  }
  async run(
    operation: string,
    identity: string,
    work: (scope: { current: () => boolean; detailsCurrent: () => boolean; guard: <T extends unknown[]>(callback: (...args: T) => void) => (...args: T) => void }) => Promise<void>,
  ): Promise<void> {
    // Old rendered page controls may fire before React paints replacement data.
    // Reject their admission synchronously, not only through disabled buttons.
    if (operation === "loadMoreGraph" && (this.slots.has("loadGraph") || this.slots.has("openNote"))) return;
    if (operation === "loadMoreSearch" && this.slots.has("runSearch")) return;
    if (operation === "loadMoreCli" && this.slots.has("loadCli")) return;
    if (operation === "loadMoreTree" && this.slots.has("loadTree")) return;
    if (operation === "loadMoreActivity" && this.slots.has("loadActivity")) return;
    if (operation === "loadMoreResources" && this.slots.has("loadResources")) return;
    if (operation === "loadMorePrompts" && this.slots.has("loadPrompts")) return;
    if (this.slots.get(operation)?.identity === identity) return;
    if (operation === "openNote") this.invalidate("loadMoreGraph", "loadGraph", "loadContextPreview", "loadRelations", "runSchemaValidate");
    if (operation === "runSearch") this.invalidate("loadMoreSearch", "runInspectSearch", "loadContextPreview");
    if (operation === "loadGraph") this.invalidate("loadMoreGraph");
    if (operation === "loadTree") this.invalidate("loadMoreTree");
    if (operation === "loadActivity") this.invalidate("loadMoreActivity");
    if (operation === "loadResources") this.invalidate("loadMoreResources");
    if (operation === "loadPrompts") this.invalidate("loadMorePrompts");
    if (operation === "loadCli") this.invalidate("loadMoreCli");
    if (["openNote", "runSearch", "loadContextPreview", "loadGraph", "loadMoreGraph"].includes(operation)) ++this.detailIntent;
    const detailIntent = this.detailIntent;
    const token = {};
    this.slots.set(operation, { identity, token });
    this.changed();
    const current = () => this.slots.get(operation)?.token === token;
    try {
      await work({ current, detailsCurrent: () => current() && this.detailIntent === detailIntent,
        guard: (callback) => (...args) => { if (current()) callback(...args); } });
    } finally {
      if (current()) { this.slots.delete(operation); this.changed(); }
    }
  }

  changeDiagnosticProfile() { this.invalidate("loadTools", "loadCli", "loadApiAudit", "loadMoreCli"); }
}
