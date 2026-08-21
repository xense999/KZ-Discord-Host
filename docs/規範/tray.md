# tray — 模組規範

> 本模組契約的唯一 owner。建檔後記得回寫 `docs/規範.md` 歸屬總表的規範檔欄。

## 公開介面

- `setup(&AppHandle)`：建匣圖示＋選單（顯示／結束）、左鍵／雙擊顯示。
- `show_main`／`hide_main`／`quit`（先 `shutdown_all` 再 exit，可重入）。
- 常數 `MAIN_WINDOW`。

## 單一來源

- 視窗 label 與匣選單文案只在此模組。

## 不變量

- 視窗 X 永遠只是 hide（lib.rs 的 on_window_event 負責呼叫）。
- 「結束」不跳確認。

## 禁止

- 不可在其他地方呼叫 `app.exit` —— 正面做法：呼叫 `tray::quit`。
