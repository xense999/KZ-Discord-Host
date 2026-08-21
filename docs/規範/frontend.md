# 前端（store＋UI）— 模組規範（2026-08-21）

> 本模組契約的唯一 owner。建檔後記得回寫 `docs/規範.md` 歸屬總表的規範檔欄。

## 公開介面

- Pinia store `useBotsStore`：`bots`／`statuses`／`logs`／`selectedId`／`selected`／`startupNotice`／`error`／`hostAutostart`／`configPath`；actions `init`、`refresh`、`select`、`start`、`stop`、`restart`、`upsert`、`remove`、`importFolder`、`setAutostart`、`setHostAutostart`、`openLogsDir`、`clearLogView`。
- 元件：`App.vue`（版面／pane 狀態／resize grip）、`TitleBar`（拖曳區、最小化、藏到匣）、`BotList`、`BotPanel`、`BotForm`、`SettingsPanel`。
- 型別集中在 `src/types.ts`（與 Rust 型別一一對應）。

## 單一來源

- 所有 IPC 呼叫與事件訂閱只在 store；元件不直接 `invoke`。例外：dialog 選檔、以及視窗 API（`getCurrentWindow().minimize/hide/startResizeDragging`，TitleBar／App）——這些是視窗殼層行為不是業務 IPC。
- `LOG_RING`（TS）與 `RING_CAPACITY`（Rust）都是 500：前端只是顯示上限，以 Rust 為準。
- 設計 token 只在 `src/styles.css`（minimalist-ui 規範）。

## 不變量

- 前端不持有權威狀態：狀態只來自 `list_status` 與 `bot-state` 事件。
- `init` 先訂閱事件再拉快照，中間不漏事件；每隻 bot 的 ring tail 只拉一次（`tailLoaded`），之後靠事件。
- log 事件合批（rAF）且每隻最多 500 行。

## 禁止

- 不用 emoji、不用 Lucide/Feather 圖示、不用漸層重陰影（minimalist-ui）；按鈕尺寸變體只用 `styles.css` 的 `.btn-sm`，元件不自訂 `.small`。
