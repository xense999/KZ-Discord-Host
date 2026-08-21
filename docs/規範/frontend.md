# 前端（store＋UI）— 模組規範（2026-08-21）

> 本模組契約的唯一 owner。建檔後記得回寫 `docs/規範.md` 歸屬總表的規範檔欄。

## 公開介面

- Pinia store `useBotsStore`：`bots`／`statuses`／`logs`／`selectedId`／`selected`／`startupNotice`／`error`／`hostAutostart`／`configPath`；actions `init`、`refresh`、`select`、`start`、`stop`、`restart`、`upsert`、`remove`、`importFolder`、`setAutostart`、`setHostAutostart`、`openLogsDir`、`clearLogView`。
- 元件：`App.vue`（依 `getCurrentWindow().label` 選 view）、`MainView`（清單＋log＋展開視窗寬度）、`FormWindow`／`SettingsWindow`（子視窗殼）、`TitleBar`（拖曳區、最小化、關閉＝hide）、`BotList`、`BotPanel`、`BotForm`、`SettingsPanel`。
- 型別集中在 `src/types.ts`（與 Rust 型別一一對應）。
- `src/theme.ts`：`currentTheme()`／`applyTheme(theme)`，主題存 localStorage（`kz-bot-host.theme`），預設 dark；以 `<html data-theme>` 切換。

## 單一來源

- 所有 IPC 呼叫與事件訂閱只在 store；元件不直接 `invoke`。例外：dialog 選檔、以及視窗 API（`getCurrentWindow()` 的 label／minimize／hide／setSize）——這些是視窗殼層行為不是業務 IPC。
- 跨視窗同步：`bots-changed`（清單變動→各視窗 refresh）、`theme-changed`（主題）、`route`（子視窗換路由）。
- `LOG_RING`（TS）與 `RING_CAPACITY`（Rust）都是 500：前端只是顯示上限，以 Rust 為準。
- 設計 token 只在 `src/styles.css`：`:root` 是暗色（預設）、`:root[data-theme="light"]` 是 minimalist-ui 原暖色淺色版；元件不得寫死色碼（hover／邊框／晶片一律用 token）。

## 不變量

- 前端不持有權威狀態：狀態只來自 `list_status` 與 `bot-state` 事件。
- `init` 先訂閱事件再拉快照，中間不漏事件；每隻 bot 的 ring tail 只拉一次（`tailLoaded`），之後靠事件。
- log 事件合批（rAF）且每隻最多 500 行。

## 禁止

- 不用 emoji、不用 Lucide/Feather 圖示、不用漸層重陰影（minimalist-ui）；按鈕尺寸變體只用 `styles.css` 的 `.btn-sm`，元件不自訂 `.small`。
