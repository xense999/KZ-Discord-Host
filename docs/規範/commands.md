# commands（Tauri IPC）— 模組規範

> 本模組契約的唯一 owner。建檔後記得回寫 `docs/規範.md` 歸屬總表的規範檔欄。

## 公開介面（前端可 invoke）

`list_bots`、`list_status`、`startup_notice`、`upsert_bot`、`remove_bot`、`import_bot_folder`、`start_bot`、`stop_bot`、`restart_bot`、`get_log_tail`、`get_host_autostart`、`set_host_autostart`、`open_logs_dir`、`config_path`。
事件：`bot-state`（StateEvent）、`bot-log`（LogEvent），名稱常數在 lib.rs。

## 單一來源

- `AppState { supervisor, startup_notice }` 與 `persist()`（app_state.rs）是「改設定就存檔」的唯一路徑。

## 不變量

- 每個命令只做轉呼叫＋錯誤轉字串；不含業務邏輯。
- `upsert_bot` 與 `remove_bot` 成功後必 `persist`。

## 禁止

- 不可在命令裡直接操作檔案或程序 —— 正面做法：呼叫對應模組。
