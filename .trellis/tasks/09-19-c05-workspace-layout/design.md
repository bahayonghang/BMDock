# C05 — Focused workspace and accessible plain-text reading: design

## Source evidence
- apps/bmdock-desktop/src/App.tsx:91 — 18 navigation destinations.
- apps/bmdock-desktop/src/App.tsx:592 — primary reader, diagnostics and editors render in one sequence.
- apps/bmdock-desktop/src/styles.css:127, :189 and :554 — 46rem panel cap, fixed fact tracks and six-column narrow navigation.
- apps/bmdock-desktop/src/App.tsx:1264 — escaped text preview.
- apps/bmdock-desktop/src-tauri/tauri.conf.json:17 — native dimensions.

Parent research: ../09-19-bmdock-client-optimization/research/frontend-audit.md. Follow parent design.md D2/D4 and the finalized official-and-projects.md for cross-layer contracts. Source conclusions are not native acceptance.

## Mechanisms, requirements and acceptance mapping
### M1
M1 -> R1 -> AC1,AC3: Compact primary navigation, list/reader and optional related-content region; explicit diagnostics destination. Suggested grouping: Workspace, Projects, Diagnostics/Settings, finalized against actual functionality. Keep compact fixture/offline/provenance status visible.

### M2
M2 -> R2,R3 -> AC2,AC3: Small existing-CSS token set and semantic control states, preserving labels and focus outline; no theme configuration platform.

### M3
M3 -> R2,R3 -> AC2,AC3: Flexible zero-minimum tracks, wrapping identifiers and narrow stacked/region-switch layout. Do not promise three simultaneous panes at 720px. Session ownership stays in C04, above transient regions.

### M4
M4 -> R4 -> AC4: Safe source reader with readable spacing/line length and independent editing of the complete engine-delivered string. C02 supplies explicit frontmatter-inclusive reads; preserve YAML/newline characters delivered by the engine, and distinguish upstream normalization from raw disk-byte fidelity. Rich Markdown remains deferred; never inject raw HTML.

### M5
M5 -> R1-R4 -> AC5: Viewport/keyboard behavior checks followed by separately documented manual or explicitly authorized native acceptance.

## Ownership
App.tsx/view modules, styles.css, relevant i18n.ts and focused tests. Reuse C04 state ownership. No native configuration, backend, dependency or global preference edits.

## Unresolved evidence and implementation choices
All visual conclusions currently derive from source. Exact layout, focus movement, IME and screen-reader behavior need interaction evidence. Inspect existing tools before proposing test dependencies.

No generic query cache, virtualizer, docking framework or state platform is implied. Use existing tools first; future new dependencies require authorization. No mutation retry follows logical cancellation or timeout_unknown.
