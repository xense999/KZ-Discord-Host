# 前端（store＋UI）— 模組規範

> 本模組契約的唯一 owner。建檔後記得回寫 `docs/規範.md` 歸屬總表的規範檔欄。

## 公開介面

- Pinia store `useBotsStore`：`bots`／`statuses`／`logs`／`selectedId`／`selected`／`startupNotice`／`error`／`hostAutostart`／`configPath`；actions `init`、`select`、`start`、`stop`、`restart`、`upsert`、`remove`、`importFolder`、`setAutostart`、`setHostAutostart`、`openLogsDir`、`clearLogView`。
- 元件：`App.vue`（版面／pane 狀態）、`BotList`、`BotPanel`、`BotForm`、`SettingsPanel`。
- 型別集中在 `src/types.ts`（與 Rust 型別一一對應）。

## 單一來源

- 所有 IPC 呼叫與事件訂閱只在 store；元件不直接 `invoke`（dialog 選檔除外）。
- 設計 token 只在 `src/styles.css`（minimalist-ui 規範）。

## 不變量

- 前端不持有權威狀態：狀態只來自 `list_status` 與 `bot-state` 事件。
- log 事件合批（rAF）且每隻最多 500 行。

## 禁止

- 不用 emoji、不用 Lucide/Feather 圖示、不用漸層重陰影（minimalist-ui）。
