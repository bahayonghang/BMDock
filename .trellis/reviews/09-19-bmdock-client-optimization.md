---
skill: trellis-plan-review
version: 0.5.0
task_dir: D:/Documents/Code/Agents/BMDock/.trellis/tasks/09-19-bmdock-client-optimization
task_name: 09-19-bmdock-client-optimization
task_status: planning
review_scope: task-tree
task_count: 8
task_members:
  - 09-19-bmdock-client-optimization
  - 09-19-c01-baseline-contracts
  - 09-19-c02-engine-read-session
  - 09-19-c03-query-execution
  - 09-19-c04-interaction-correctness
  - 09-19-c05-workspace-layout
  - 09-19-c06-search-experience
  - 09-19-c07-integration-acceptance
task_statuses:
  09-19-bmdock-client-optimization: planning
  09-19-c01-baseline-contracts: planning
  09-19-c02-engine-read-session: planning
  09-19-c03-query-execution: planning
  09-19-c04-interaction-correctness: planning
  09-19-c05-workspace-layout: planning
  09-19-c06-search-experience: planning
  09-19-c07-integration-acceptance: planning
verdict: 可执行
blocking: 0
should_fix: 0
notes: 0
generated_at: 2026-09-19T03:26:24Z
---

# Trellis 规划审阅报告

## 审阅范围

根任务为 `09-19-bmdock-client-optimization`，模式为 `task-tree`，共 8 个任务；基准产品代码为 `251c74b`。本报告覆盖根任务及其全部递归子任务的 `task.json`、`prd.md`、`design.md`、`implement.md`、`implement.jsonl`、`check.jsonl`，以及根任务的六份研究/验证材料。顺序只表示父子成员关系，不代表执行依赖。

| 有序成员 | 状态 |
|---|---|
| 09-19-bmdock-client-optimization | planning |
| 09-19-c01-baseline-contracts | planning |
| 09-19-c02-engine-read-session | planning |
| 09-19-c03-query-execution | planning |
| 09-19-c04-interaction-correctness | planning |
| 09-19-c05-workspace-layout | planning |
| 09-19-c06-search-experience | planning |
| 09-19-c07-integration-acceptance | planning |

已读取项目 `.trellis/workflow.md`、注册包索引、跨层指南、`supervisor-state.md` 及相关 typed IPC 条款。既有 `.trellis/tasks/09-12-bmdock-product-completion/` 不属于本审阅树，未修改。审阅者未修改任何规划或产品文件；本轮发现的问题由规划所有者修订后复核。报告是唯一持久写入。

## 结论

**可执行 — 阻断 0 / 应修 0 / 提示 0。**

**GO：作为后续实施审查的完整规划交接。NO-GO：本轮直接启动实现、宣布性能达标或产品可发布。** 用户已授权分析和创建规划任务；该授权不包含 `task.py start`、产品改动、安装依赖、原生 UI 操作、真实 vault 接入或发布。

最终文本对范围、依赖、所有权、验收及证据层级保持一致。不存在尚未处理的机制缺口；C01 仍须通过实际契约捕获冻结后续实现依赖的字段、测量方法和测试入口。规划结构通过不代表这些未来运行证据已经存在。

## 问题清单

无未解决问题。下文“已复核的修订”记录审阅过程中关闭的两项实质问题；它们不计入最终待修数量。

## 未能核实

1. **真实桌面连接及生命周期**：没有运行新适配器、握手、延迟读取、关闭或故障注入；当前产品仍安装空提供者。C02 必须用实际生产 dispatch/session 路径完成两份独立 profile 证据，不能用探针或测试替身替代。
2. **运行时返回结构与全文真实性**：源码支持 profile 差异和 frontmatter 参数，但本审阅未捕获 MCP wire payload。C01 须记录包装结构、错误文本、UTF-8/YAML/CRLF、上游规范化及索引就绪状态；未知结构阻止依赖它的适配器验收。
3. **性能及召回**：未测量检索 p95、IPC 开销、RSS、独立标注召回或 1/5 MiB 输入延迟。50/100 ms 和相对开销均为候选工程预算。C01 须冻结机器、场景、原始样本、时钟边界及呈现帧观测方法；动画帧回调或 React commit 时间不能自行等同屏幕已呈现。
4. **原生布局、输入与辅助技术**：未启动或操作 Tauri 窗口；720×480、1200×800、200% 文本缩放、中文 IME、读屏器及 Windows 进程树证据均未取得。C05/C06/C07 未取得相应证据时保持相关 AC 未完成。
5. **实现后工具链、全部历史 gate 与其余外部项目**：未运行未来 Rust/frontend/行为/性能检查；研究材料中的 34 项结构检查及历史 unit 失败按作者本轮记录引用，审阅者未将其重新计为执行通过。只重新打开核实了 Basic Memory MCP、Cloud Web App、MCP tools 规范与 Joplin Search 四份关键一方页面，其余比较资料未逐页复查。G0–G7、完整 CI、真实数据、安装签名和发布不在本规划的通过结论内。

## 可靠部分

### 结构、阶段与上下文

`plan_precheck.py --include-descendants` 最终返回 0，8 个成员均为 `planning`，每个成员六份核心规划产物齐全，无树完整性错误、缺失文件、越界行号或阻断模板残留。`git diff --check` 通过；该命令不覆盖未跟踪文件的内容质量，本报告另行读取了本树产物。

C02/C03 的加粗 R 定义及 Mapping 表未被机械解析器计为标准 R/AC；C03 有四处短路径被机械标记 ambiguous。人工按同段的 desktop/release 限定和父研究全文路径定位后核实了代码与映射，未将解析器限制误报为缺失需求。全树需求到 AC 再到设计机制已人工检查，不能把脚本的 0 个 blocking 当作这一步的替代。

16 份清单全部引用真实 spec/research；不含 `_example` 或产品代码注入。约 188 KB 的全量 typed IPC 规范已改为 `research/contract-boundaries.md` 这一有来源的加载索引，仍要求实施前读取被修改条款。所读清单单文件均小于 32 KiB，根清单引用总量小于 90 KiB，未超过当前 32 KiB 单文件与 128 KiB 总量默认限制；不需要更改全局注入配置。规则依据为 `.trellis/scripts/common/config.py:302`。

元数据 `package=null` 避免错误地把桌面工作路由至唯一注册的 probe 包；`meta.affected_package` 明确指向 `apps/bmdock-desktop`。父 `implement.md:9` 的 DAG 与子任务 `meta.depends_on` 一致：C01 → C02 → C03；C01 → C04 → C05；C03/C04/C05 → C06；C01–C06 → C07。共享 `App.tsx` 工作顺序及单一 `ipc.ts` 写入者有明确约束。

父 `implement.md:19`、C06 `implement.md:25` 和 C07 `implement.md:5` 明确：C05/C06 各自在完成前取得其必需的原生证据，C07 汇总有效的同版本证据并只重跑被后续集成改动影响的场景。未将 C06 的完成条件推给依赖 C06 完成的 C07，避免了验收依赖环。

### 每个成员的验收机制

| 成员 | 已核实的 R / AC 覆盖 | 机制与约束 |
|---|---|---|
| Parent | R1–R7 / AC1–AC7 | `design.md` D1–D5 和映射表分配给 C01–C07；排除真实 vault、写入、第二索引与发布；保留原生/性能未完成状态。 |
| C01 | R1–R3 / AC1–AC4 | D1 捕获独立 profile 契约和完整源码文本；D2 确定语料、独立标签与预算；D3 建立延迟 IPC 行为检查入口，显式保留旧 gate 失败。 |
| C02 | R1–R5 / AC1–AC5 | 短锁快照、会话句柄、真实握手、代次检查、受控启动与有界关闭；独立生产 dispatch 集成证据；T17 receipt 不被偷换为实时进程关闭。 |
| C03 | R1–R5 / AC1–AC5 | 官方结果顺序/种类/分值/分页保持；完整请求键；owner-local 有界 admission；refresh 代次与索引就绪；错误文本不能伪装为空结果。 |
| C04 | R1–R4 / AC1–AC4 | M1 对结果、错误及后续请求检查身份；M2 保留编辑会话和已提交版本基线；M3 本地 pending/error；M4 可控延迟的行为回归。 |
| C05 | R1–R4 / AC1–AC5 | M1 主列表/阅读区域及次级诊断；M2/M3 控件状态、零最小宽度、换行及窄布局；M4 转义和完整返回文本；M5 独立原生验收。 |
| C06 | R1–R4 / AC1–AC5 | M1 搜索命中进入统一阅读选择；M2 按需诊断；M3 50×3 行窗口和焦点退路；M4 同语料计量；M5 延后全文信息扫描与重复预览，绑定编辑修订号。 |
| C07 | R1–R4 / AC1–AC5 | D1 两份实际桌面读取链路；D2 固定协议的配对性能与标签复查；D3 原生会话及父子 AC 证据映射；缺少证据不关闭 AC。 |

### 仓库断言与外部契约

`apps/bmdock-desktop/src-tauri/src/main.rs:35` 在整个同步 dispatch 内持有全局锁，`:75`/`:77` 安装 `EmptyLibrary`/`EmptyDraftStore`；`library.rs:3600` 和 `:4536` 将 FixtureLibrary 限定为测试。`library.rs:3673` 按标识排序，`:3759` 逐笔记全文检索，`:4667` 在收集后切页。因此“生产尚无连接”和“fixture 扫描不是已测生产瓶颈”的结论有代码支持，接入真正读取路径先于速度宣称是合理顺序。

`App.tsx:718` 的笔记/详情链、`:1618` 搜索和 `:1709` 旧页合并缺少所需用户意图检查；`:4456`/`:4927` 重置编辑种子，`:5048` 保存回执回写 body。源码能证明缺少守卫和覆盖路径，规划没有把它们伪装成已复现的 native 故障。`App.tsx:616`/`:620` 额外触发 inspector；`:422` 后自动加载六个次级目录，支持按需加载验收。`styles.css:127` 的 46rem 限宽与 `:554` 的窄窗六列导航有源码支持，实际可用性仍待视觉验证。

官方 release 的 `.work/engines/release/src/basic_memory/schemas/search.py:125` 定义结果种类、所属笔记身份和分值，`:156` 定义 `total_is_exact`/`has_more`；SQLite repository `:1072` 在 LIMIT/OFFSET 前排序；`mcp/tools/search.py:1233` 的 JSON 成功与 `:1240` 的错误指导文本路径确实不同。主预览 `mcp/tools/search.py:919` 提供 compact、`:887` 提供 valid_at，release 不具有这组相同参数。release `repository/search_repository_base.py:2617` 的融合公式可产生大于 1 的分值，不能截断为概率。

`compatibility/profiles.json:6`/`:13` 的 SHA 和 21/27 工具集合保持独立。`scripts/core.py:112` 禁用语义搜索并强制本地隔离；对应计划没有要求通过隐式模型下载、云路由或假语义分数完成验收。`.trellis/spec/bmdock-probe/backend/supervisor-state.md:17` 的 receipt-only T17 限制与 C02 的新增生命周期所有权明确区分。

重新打开的一方来源支持所采用的边界：Basic Memory 当前文档明确 `include_frontmatter` 默认 false、项目及筛选字段的含义；MCP 规范区分结构化结果和工具错误；Cloud Web App 的浏览布局与 Joplin 的搜索说明可用作交互参考，不能证明本地固定版本或 BMDock 的性能。[Basic Memory MCP reference](https://docs.basicmemory.com/reference/mcp-tools-reference)、[MCP tools](https://modelcontextprotocol.io/specification/2025-11-25/server/tools)、[Cloud Web App](https://docs.basicmemory.com/cloud/web-app)、[Joplin Search](https://joplinapp.org/help/apps/search/)。

### 定量复算

`50 rows/page × 3 pages = 150 rows`；30 次 warm 观测的 nearest-rank p95 为 `ceil(0.95×30)=29`，即零基下标 28。30 次编辑由 10+10+10 个位置样本构成。1 MiB=1,048,576 bytes，5 MiB=5,242,880 bytes。约 4 KiB 语料明确要求记录实际物理字节数，未用近似体积替代测量记录。

适配器预算明确是两个 warm p95 之差，单位为 ms；`max(50 ms, 0.25×direct-engine p95)` 在 direct p95=200 ms 处切换分支。UI 的 `1.10×baseline` 是允许至多 10% 回退的上限，并非 10% 提速承诺。五次 cold 样本、30 次 warm 样本和原生编辑预算各自独立；不以 EmptyLibrary 或未启用语义模式产生的零值计算改进。

### 已复核的修订

| 初审问题 | 源码证据 | 最终处理 |
|---|---|---|
| 完整源码承诺未指定 JSON frontmatter 行为 | release `mcp/tools/read_note.py:91` 默认 false；`mcp/note_reads.py:97` 后选择去 YAML 的 body；main-preview `mcp/tools/read_note.py:96` 增加可选行范围 | Parent D2、C01 D1、C02 协议段、C03 读取段及 C05 R4/AC4 统一为显式 frontmatter、无行切片的完整返回字符串；上游规范化与磁盘字节等同性分开记录。 |
| “大笔记可用”缺少确定度量和承担优化的机制 | `App.tsx:1240` 每次渲染全文分类，`:1264` 重复 `<pre>`；旧 C07 只写“usable under measured limits” | Parent D4/C01 冻结 30 次编辑和 1/5 MiB 的 50/100 ms 候选 native p95；C06 R4/M5 拥有按需信息扫描/预览及修订号检查；C07 AC4 给出判据，保留未测为未完成。C06 执行与回滚条款同步包含该工作。 |

以上为技术一致性修订，不需要重新选择产品范围。原计划已记录的只读 fixture、纯文本阅读、不建立第二索引、不默认引入缓存/虚拟列表框架等决定未被推翻。

## 盲区

Agent 审阅 Agent 的规划不构成完全独立的第二意见，双方仍可能共享盲区。没有未解决发现仅表示本轮未再发现问题；本报告是分诊与交接记录，不是用户批准，也不是产品、原生体验或性能的验收证据。
