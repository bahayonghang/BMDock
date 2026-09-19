# 后续验收任务 Prompt

下面代码块是可复制到新会话执行的完整指令。其原生操作和完整矩阵授权
仅在用户发送该指令时生效；当前提交/归档工作不会启动这些操作。

```text
继续 D:\Documents\Code\Agents\BMDock 的客户端优化验收。

先读 AGENTS.md、Trellis 当前上下文，以及：
- .trellis/tasks/09-19-bmdock-client-optimization/prd.md
- .trellis/tasks/09-19-bmdock-client-optimization/research/implementation-progress.md
- .trellis/tasks/09-19-c07-integration-acceptance/research/integration-acceptance.md
- .trellis/tasks/09-19-c07-integration-acceptance/research/delivery-review.md（如存在）
- execution/evidence/c03-representative-summary.json
- execution/evidence/client-raw-receipts-summary.json（如存在）

沿用现有父任务和 C01/C03/C05/C06/C07，不重复创建同名任务。
C02、C04 已完成，其资料在 .trellis/tasks/archive/2026-09/ 下；不要重复实施。
核对实时 Git 状态和远端提交，保留与本轮无关的文件，尤其原有
.trellis/tasks/09-12-bmdock-product-completion/ 中的 6 个文件。

我授权以下后续验收及必要修复：
1. 仅对生成的 bmdock-fixture 启动并操作本项目 Tauri 窗口；允许相关
   浏览器辅助布局检查。使用可用的原生工具或明确归因的人工执行记录。
   浏览器结果不能替代 WebView2、原生 IME、屏幕阅读器或实际呈现证据。
2. 检查 1200x800、720x480、200% 文本缩放、键盘焦点、中文组合输入、
   搜索/目录切换、同一笔记的不同命中行、失败后的选中项、草稿保留及
   实际挂载行数。修复发现的问题并做针对性回归。
3. 按冻结的 native-input-latency-protocol.md 采集 1 MiB/5 MiB 各 30 次
   提交输入（开头/中间/末尾各 10 次）到首个已呈现更新帧的时延；
   p95 上限分别为 50/100 ms。补齐有意义 UI 场景的基线及候选比较，
   候选 p95 <= 基线 110%。没有可用原生追踪关联就保留 UNVERIFIED，
   不得使用 SSR、requestAnimationFrame 或普通 paint 代理宣称通过。
4. 执行原先延后的完整性能矩阵，预留约 2–3 小时并按实际运行修正预计。
   先核对 c01-measurement-manifest-v3.json 和 c01-baseline-coverage.json：
   历史仅 16/428 个单元通过，剩余 412 个待验收。区分仍可复用的基线
   与需要重测的当前版本；每个适用场景保持 5 冷/30 暖样本，按 profile
   独立记录。不得把 C03 的 140 个配对样本直接算作 C01 覆盖单元。
5. 先解决 C01 Probe.raw 与 C03 query-driver 的时钟边界可比性，再用于
   完整性能验收。不要直接混合旧分布。保留所有原始样本、失败尝试、
   哈希、冷暖区分、p50/p95、响应字节、命令数和真实硬件信息；区分
   driver/launcher 与 engine process-tree RSS。无法取得的指标保持未知。

release 固定 c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048；
main-preview 固定 3452c821d76c083823d020984d71e06904a1ff1e。
官方引擎是索引和排序的唯一所有者，不加第二索引或持久查询缓存。
保持 immutable 查询的身份/顺序/分数/分页精确一致；只有外部修改后的
三类 refresh 配对允许记录分数变化，不得把该例外扩展到稳定语料或计时。
适配层暖 p95 开销上限仍为 max(50 ms, 25% direct p95)。

本地忽略的大型原始 JSON 可能在新克隆中不存在。使用已提交摘要、
紧凑契约 fixture 和再生成入口；不得把缺失原始证据说成重新验证通过。
遵守已有大型原始文件忽略规则；默认提交摘要，新增大型原始产物留本地，
不要无意加入 Git 历史。

按 Trellis 使用 trellis-implement / trellis-check 子代理，并使用
frontend_ui_engineer 处理原生交互与页面问题；划清文件所有权，主会话负责
协调、规范更新和验收映射。没有变化的已通过检查无需反复重跑。
先完成验收范围内的修复，不扩展到任意技术债清理。

禁止真实用户 vault 读写、全局配置/凭据修改、依赖或模型下载、Cloud/
provider 激活、引擎升级、阈值放宽、伪造历史 G0–G7 状态或安装包发布。
历史 phase-order 和 Clippy 问题单列；若不属于本验收范围，不静默修复。
若无法取得必要工具或证据，完成其他可做工作并报告确切缺口。

完成后更新 C01/C03/C05/C06/C07 与父任务的逐项验收状态，给出证据路径
和剩余阻塞。仅归档全部自身验收项成立的任务。授权按逻辑提交本次范围内
修改并正常推送；禁止 force push，不纳入其他工作。父任务只有在所有
所属验收要求满足时才能归档。最终给出提交、推送、归档结果及下一步。
```
