import type { SearchPageDto, EngineHitDto, SearchHitDto } from "./ipc";

// Only search pages live here. History retains request cursors, never evicted bodies.
export class SearchWindow {
  pages: { cursor: string | undefined; value: SearchPageDto }[] = [];
  history: (string | undefined)[] = [];
  activeResult: string | null = null;
  cursor: string | undefined;
  reset() { this.pages = []; this.history = []; this.activeResult = null; this.cursor = undefined; }
  previous(): { available: boolean; cursor: string | undefined } {
    const index = this.history.indexOf(this.cursor);
    return { available: index > 0, cursor: this.history[index - 1] };
  }
  accept(value: SearchPageDto, cursor?: string): SearchPageDto {
    this.cursor = cursor;
    if (!this.history.includes(cursor)) this.history.push(cursor);
    this.pages = this.pages.filter((page) => page.cursor !== cursor);
    this.pages.push({ cursor, value: { ...value, hits: value.hits.slice(0, 50) } as SearchPageDto });
    while (this.pages.length > 3) {
      const index = this.pages.findIndex((page) => !page.value.hits.some((hit) => hit.identifier === this.activeResult));
      this.pages.splice(index < 0 ? 0 : index, 1);
    }
    // Engine result identities can share an owning note; never deduplicate by owner.
    const ordered = [...this.pages].sort((a, b) => this.history.indexOf(a.cursor) - this.history.indexOf(b.cursor));
    const hits = ordered.flatMap<EngineHitDto | SearchHitDto>((page) => page.value.hits).filter((hit, index, all) => all.findIndex((candidate) => candidate.identifier === hit.identifier) === index);
    return { ...value, hits } as SearchPageDto;
  }
}

// A removed next-page button has no natural focus destination. Keep an existing
// control untouched; otherwise return to the persistent results summary.
export function restoreSearchFocus(control: { isConnected: boolean } | null, fallback: { focus(): void } | null, focusUnowned: boolean) {
  if (focusUnowned && control && !control.isConnected) fallback?.focus();
}
