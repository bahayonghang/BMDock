# Research: Native committed-input to presented-frame latency

- Query: Freeze the C01 Windows Tauri/WebView2 large-note measurement protocol; C06 owns native acceptance and C07 integrates applicable evidence.
- Scope: mixed, read-only source/tool availability research. No application/browser operation, screen capture, tracing session, installation, or product edit was performed.
- Date: 2026-09-19
- Status: protocol defined; native acquisition and per-frame correlation remain **UNVERIFIED**. The installed tools alone do not establish direct presented-frame evidence.

## Findings

### Local feasibility

Read-only inspection found `C:\Windows\System32\wpr.exe` version `10.0.26100.9444`, `C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit\wpa.exe` version `11.7.395.48728`, and a WebView2 runtime directory named `153.0.4234.32`. The last item is an installed-directory observation, not a running-runtime identity. `PresentMon` was not found on PATH; GPUView was not found on PATH or at the checked WPT location. No broad installation inventory was attempted.

`apps/bmdock-desktop/src-tauri/Cargo.toml:13` pins Tauri `2.11.5`; `tauri.conf.json:13` defines the native window. There is no currently verified BMDock input-to-display trace receipt. Parent `design.md:55` contains the timing budget; `implement.md:19` correctly places child-native acceptance before C07. The design sentence assigning native pass/fail collection to C07 needs alignment by the parent owner.

Microsoft documents WebView2 access to CDP through its existing native API, without requiring the optional .NET helper package. A process-local `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=<reserved-local-port>` is another documented attachment route; applying it and launching/attaching are future authorized native work. Use neither a user-global environment variable nor a registry change. [WebView2 CDP](https://learn.microsoft.com/en-us/microsoft-edge/webview2/how-to/chromium-devtools-protocol), [WebView2 debugger attachment](https://learn.microsoft.com/en-us/microsoft-edge/webview2/how-to/debug-visual-studio-code).

### Frozen protocol

1. **Workload and environment.** Use the frozen synthetic 1,048,576-byte and 5,242,880-byte Markdown fixtures. Record exact fixture hash/bytes, app revision/build mode, actual running WebView2 version, OS, CPU/GPU/driver, refresh rate, DPI/zoom, viewport, power mode, and trace categories. The editor must be visible, focused, unobscured, already loaded, with the insertion location on screen. Initial note loading, scroll-to-position, fixture reset, and focus changes occur outside measured intervals. Capture 30 separately identifiable committed single-character insertions per size: 10 near the beginning, 10 midpoint, 10 end. Freeze offsets and character sequence in the fixture manifest. Use native non-composing character input for the base run; record `isComposing=false`. Any additional IME run is separately labeled and counts only a verified final commit, never pre-edit/composition updates. Do not pool two sizes or profiles.
2. **Capture on the actual Tauri WebView.** Qualify the runtime before measured samples: record CDP `Browser.getVersion`, the target identity, and `Tracing.getCategories`; attach to BMDock, not a browser-only Vite page. Start CDP `Tracing.start` with `transferMode=ReturnAsStream` and supported categories including `blink.user_timing`, `devtools.timeline`, `disabled-by-default-devtools.timeline.frame`, `cc`, `benchmark`, `input`, `viz`, and `gpu`. These are a requested category set, not a claim that every category is available in the installed runtime. Preserve the accepted configuration. Stop with `Tracing.end`, wait for `Tracing.tracingComplete`, retain `dataLossOccurred`, and drain the returned stream with `IO.read` until EOF. Preserve original raw trace data; do not keep only a summary. Missing categories/fields or data loss are qualification failures, not reasons to replace the endpoint with rAF.
3. **Identify committed input and the changed content.** A measurement-only capture-phase input observer records sample ID, `event.timeStamp`, event type/inputType/data, composition state, editor identity, UTF-16 selection position, and a small surrounding text window. Use a backdated User Timing mark with `startTime=event.timeStamp`, plus a separate handler-entry mark, so both endpoints can be interpreted on the browser trace's monotonic timeline. Do not substitute handler-entry time for the specified input-event timestamp. Record the authoritative updated editor value/version and a bounded local text check after application handling; defer whole-note hashing until outside the interval. Trace the actual changed-text paint/commit through its compositor frame. A diagnostic marker painted elsewhere is not by itself proof that the edited glyph painted in the same frame.
4. **Resolve presentation, not merely rendering.** Use the runtime's input/commit/frame-flow identities to join the committed edit to the corresponding updated main-thread compositor frame, then its successful native presentation feedback. Relevant Chromium fields include `event_latency_id`, `surface_frame_trace_id`, and `display_trace_id`; frame reporting distinguishes fully presented, partial, dropped, and no-update states. Accept the earliest successfully presented frame whose paint/content evidence establishes the edited character and whose IDs establish that it is the frame being timed. Do not select the next arbitrary swap, cursor-blink frame, screenshot timestamp, or an EventLatency termination lacking a presented-frame path. If native feedback is estimated or cannot be distinguished from actual display completion, cross-check the same frame with Windows presentation evidence. If the runtime lacks the identities or content-to-frame join, classify the sample `UNVERIFIED`; temporal proximity alone is insufficient.
5. **Calculate and retain.** For each valid sample compute `latency_ms = (presented_trace_time_us - committed_input_trace_time_us) / 1000`. Retain the sample ID, raw event timestamp/mark, frame/flow IDs, presentation endpoint and evidence type, content check, timestamps, units, and any clock uncertainty. With exactly 30 valid samples, use nearest-rank p95: sorted sample **29**. Acceptance is p95 <=50 ms for 1 MiB and <=100 ms for 5 MiB. Report all 30, maximum, and per-position samples; do not trim slow observations. Missing/dropped/unattributable samples must be disclosed and prevent a passing result until the required valid workload is measured. Never subtract a refresh interval or tracing overhead to manufacture a pass.

The acquisition API and extraction acceptance rules are frozen here, rather than deferred to C07. The runtime qualification is mandatory because Chromium's trace data are implementation details, and the cited main-branch sources are not a guarantee about this machine's Edge build.

### What the native sources establish

| Source | Verified capability or limit | Consequence |
| --- | --- | --- |
| [CDP browser protocol: Tracing](https://raw.githubusercontent.com/ChromeDevTools/devtools-protocol/master/json/browser_protocol.json) | Category discovery, start/end, stream transfer, clock-sync markers, and data-loss flag exist. The protocol does not define a universal DOM-input-to-physical-frame result. | Discover the actual protocol at capture time; retain raw flow data. `recordClockSyncMarker` is not automatic synchronization with an independent ETW clock. |
| [Chromium EventLatency recorder](https://raw.githubusercontent.com/chromium/chromium/main/cc/metrics/event_latency_tracing_recorder.cc) | Keyboard events and input/surface/display trace IDs are supported; events can also terminate without compositor stage history. | A duration named EventLatency is not sufficient evidence that the character was presented. |
| [Chromium compositor frame reporter](https://raw.githubusercontent.com/chromium/chromium/main/cc/metrics/compositor_frame_reporter.cc) | PipelineReporter distinguishes fully presented, partial, dropped, and no-update frames. Partial presentation may still contain old main-thread content. | Follow the updated text's main-thread frame, not any compositor output. |
| [Chromium presentation feedback](https://raw.githubusercontent.com/chromium/chromium/main/ui/gfx/presentation_feedback.h) | Timestamp describes scan-out start, while flags distinguish hardware clock/completion from estimates and failure. `display_trace_id` identifies a traced swap where available. | Record actual feedback provenance. A platform estimate cannot silently satisfy an actual-presented-frame requirement. |
| [User Timing](https://www.w3.org/TR/user-timing/) | Marks can carry a specified start time and detail. | Use event-origin marks plus raw timestamp data; instrumentation is an input reference, not a presentation sensor. |

`EventLatency` source also contains a WebView presentation limitation comment; it must not be generalized to Windows WebView2 without runtime evidence because Chromium has other WebView platforms. The important general limit is that missing/estimated presentation data cannot be promoted to actual display evidence.

### ETW fallback and its boundary

Microsoft's [official ETW instructions](https://github.com/MicrosoftEdge/WebView2Feedback/blob/main/diagnostics/etw.md) supply `WebView2_CPU.wprp` and document an elevated `wpr -start WebView2_CPU.wprp -filemode`, reproduction, then `wpr -stop <trace.etl>`. These commands are documented acquisition steps only; none ran. The Microsoft profile was not found locally by the bounded check and was not downloaded. The documentation supports WebView2/system diagnosis; it does not promise an edited-glyph-to-display join or complete DWM/DXGI present data. Any future ETW capture must select/record the presentation providers it actually needs and must not stop an unrelated tracing session.

[PresentMon's first-party console documentation](https://raw.githubusercontent.com/GameTechDev/PresentMon/main/README-ConsoleApplication.md) supports ETL ingestion, process filtering, QPC timing, present IDs, and display latency; it also distinguishes undisplayed frames. However, the WebView2 GPU/presentation process may differ from the app's renderer/host. Process ID and swap-chain identity must be matched. Its earliest keyboard/mouse-to-frame metric is not necessarily the committed `input` event or a frame containing the changed glyph. PresentMon is not currently located by the bounded tool check, and installing it is outside this follow-up.

When combining CDP and ETW, retain explicit paired clock calibration markers and their bounded error; never subtract a `performance.now()` value from an unrelated QPC/ETW value directly. [DXGI frame statistics](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ns-dxgi-dxgi_frame_statistics) distinguish presentation count, refresh count and QPC synchronization, and also warn that `Present()` call counts are not display counts. Merely having those APIs does not give BMDock access to WebView2-owned swap-chain objects.

### Evidence that cannot close the metric

- `requestAnimationFrame`, including two callbacks, runs as part of animation-frame processing and is not a display-completion receipt. [HTML animation-frame specification](https://html.spec.whatwg.org/multipage/imagebitmap-and-animations.html#animation-frames).
- React commit time, event-handler duration, paint/raster completion, average FPS, or a successful `Present()` call omits part of the required endpoint or does not prove changed text.
- DevTools screenshots/filmstrips and CDP screencast timestamps may corroborate content, but without a matching compositor/presentation identity they do not time the first physically presented updated frame.
- A fixed 16.7 ms addition, a generic INP score, or PresentMon's earliest-input metric changes the stated metric and cannot be substituted without an explicit acceptance change.

## Files found

- `.trellis/tasks/09-19-bmdock-client-optimization/design.md:55`: existing 30-edit/50 ms/100 ms acceptance statement; current C07 wording conflicts with child-native ownership.
- `.trellis/tasks/09-19-bmdock-client-optimization/implement.md:19`: C05/C06 obtain their own native acceptance before C07.
- `apps/bmdock-desktop/src-tauri/Cargo.toml`: current native framework version.
- `apps/bmdock-desktop/src-tauri/tauri.conf.json`: actual desktop window configuration.
- Installed WPR/WPA/WebView2 locations above: bounded read-only availability evidence.

## Related specs

`.trellis/spec/bmdock-probe/backend/supervisor-state.md` separates real native evidence from unit fakes; `.trellis/workflow.md` requires independently verifiable child acceptance. No spec changes were made.

## Caveats / Not Found

**Direct presented-frame evidence cannot currently be established from the available read-only findings.** CDP and ETW are feasible acquisition mechanisms, but the actual WebView2 trace fields, hardware-feedback provenance, and changed-glyph-to-present join have not been exercised. There is no current authorized native UI operation, existing native receipt, or demonstrated parser satisfying that join. C01 can freeze this protocol and declare that boundary now; C06 cannot pass its native criterion until the qualified acquisition and correlation actually succeed. C07 integrates applicable same-revision receipts; it must not become the first owner of C06 evidence.

All external references were accessed on 2026-09-19. Chromium/CDP main-branch source may differ from the installed WebView2 build. No dependency, browser flag, configuration, native process, trace session, or UI state was changed. This file was persisted solely because the dispatched research role requires durable research output.
