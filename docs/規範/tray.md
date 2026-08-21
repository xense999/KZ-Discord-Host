# tray — 模組規範（2026-08-21）

> 本模組契約的唯一 owner。建檔後記得回寫 `docs/規範.md` 歸屬總表的規範檔欄。

## 公開介面

- `setup(&AppHandle)`：建匣圖示＋選單（顯示／結束）、左鍵／雙擊顯示。
- `show_main`／`quit`（只 `app.exit(0)`；停 bot 的唯一路徑是 lib.rs 的 `RunEvent::Exit` handler，Job Object 為保險）。
- `open_child(app, label, query)`：把 query 存進 `AppState.routes`、發 `EVENT_ROUTE`、show＋focus。
- `is_child(label)`、常數 `CHILD_WINDOWS`（form／settings）、`EVENT_ROUTE`。
- 常數 `MAIN_WINDOW`。

## 單一來源

- 視窗 label 與匣選單文案只在此模組。

## 不變量

- 三個視窗（main／form／settings）都宣告在 `tauri.conf.json`，永遠只 hide 不 destroy：自訂標題列的 X（TitleBar.vue `win.hide()`）與 CloseRequested（lib.rs `on_window_event` → `prevent_close` + hide）。
- 子視窗的 query 由 `AppState.routes` 傳遞，取用一次即清除（`take_route`）。
- 「結束」不跳確認。

## 禁止

- 不可在其他地方呼叫 `app.exit` —— 正面做法：呼叫 `tray::quit`。
