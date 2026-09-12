export const LOCALE = "zh-CN" as const;

export const messages = {
  appName: "BMDock",
  tagline: "Basic Memory 桌面工作台",
  skipToMain: "跳到主要内容",
  navLabel: "工作台分区",
  navWorkbench: "工作台",
  navRuntime: "运行状态",
  navAbout: "说明",
  g0Notice: "G0 尚未通过。本界面是布局壳，不表示生产写入安全。",
  emptyBadge: "空态",
  errorBadge: "错误",
  statusBadge: "状态",
  workbenchEmptyTitle: "还没有可显示的笔记",
  workbenchEmptyBody:
    "当前只提供分区导航与中文界面壳。项目路由、分页目录、草稿和笔记读写属于后续任务；本页不会连接真实资料库，也不会写入磁盘。",
  workbenchErrorTitle: "无法读取工作台状态",
  workbenchErrorBody:
    "只读快照失败。下面是分类后的错误，不会启动 Supervisor，也不会写入资料库。",
  unexpectedCapabilities: "能力响应格式不符合预期",
  unexpectedRuntime: "运行状态响应格式不符合预期",
  runtimeTitle: "运行状态",
  runtimeLoading: "正在读取运行状态…",
  runtimeEmptyTitle: "引擎尚未启动",
  runtimeEmptyBody:
    "默认状态为未启动。本页只读取 typed 快照，不会启动或停止 Supervisor，也不会调用 raw callTool。",
  runtimeErrorTitle: "无法读取运行状态",
  runtimeFailedTitle: "引擎处于失败状态",
  runtimeRefresh: "重新读取状态",
  runtimeStatusLabel: "生命周期",
  runtimeProfileLabel: "引擎配置",
  runtimeProjectLabel: "项目",
  runtimeFailureLabel: "失败分类",
  runtimeNone: "无",
  capabilitiesTitle: "能力边界",
  capabilitiesCommands: "已公开命令",
  policyFixture: "仅 bmdock-fixture",
  policyNoPaths: "禁止任意路径",
  policyNoCallTool: "禁止 raw callTool",
  aboutTitle: "说明",
  aboutIntro:
    "这是 T08 桌面布局壳。默认界面语言为简体中文。完整编辑器属于后续任务，帮助与无障碍完备性也不在本任务关闭。",
  aboutSafety:
    "只读调用 get_capabilities 与 get_runtime_state。默认运行状态为未启动。不会启动引擎、选择真实项目或写入 vault。",
  aboutProfiles:
    "release（c0bd87c6，21 个工具）与 main-preview（3452c821，27 个工具）保持隔离。本界面不把工具清单或 just contract 当作功能验收。",
  aboutCommands:
    "just dev 启动本桌面壳；just build 仍编译 G0 探针，因为 CI 继续使用该入口。just contract 与 just contract-main 仍是探针。",
  statusNotStarted: "未启动",
  statusStarting: "正在启动",
  statusConnected: "已连接",
  statusStopping: "正在停止",
  statusStopped: "已停止",
  statusFailed: "失败",
  profileRelease: "release",
  profileMainPreview: "main-preview",
  failurePolicy: "策略拒绝",
  failureTransport: "传输错误",
  failureTimeoutUnknown: "超时未知",
  failureProcess: "进程错误",
  failureUnverified: "未验证",
  invokeFailure: "调用失败",
  errorSchema: "数据格式错误",
  errorUnsupported: "能力不受支持",
} as const;

export type MessageKey = keyof typeof messages;

export function t(key: MessageKey): string {
  return messages[key];
}
