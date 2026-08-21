# supervisor — 模組規範（2026-08-21）

> 本模組契約的唯一 owner。建檔後記得回寫 `docs/規範.md` 歸屬總表的規範檔欄。

## 公開介面

- `Supervisor::new(sink, logs_dir, &[BotSpec])`、`upsert(spec)`、`remove(id)`、`specs()`、`statuses()`、`log_tail(id)`、`start(id)`、`stop(id)`、`restart(id)`、`shutdown_all()`。
- `EventSink` trait（`on_state`／`on_log`）：Tauri 端以 emit 實作，測試以 channel 實作。
- 型別 `BotState { Stopped | Starting | Running{pid,since_ms} | Backoff{until_ms,attempt} | Stopping }`、`StateEvent`、`LogEvent`。
- `backoff_delay(attempt)`、常數 `BACKOFF_BASE/MAX`、`STABLE_AFTER`。

## 單一來源

- 權威狀態只在 Supervisor；前端不得自行推算狀態。
- 退避公式只在 `backoff_delay`。
- 清單順序＝插入順序（Vec），是 UI 與 config.json 的排序來源。

## 不變量

- 同一 bot 同時最多一個 `Run`（重複 start 回 Err）；`stop` 進行中 slot 仍被占住，start 一樣被拒，直到 task 真的結束（逾時 10 s 則 abort，子程序 kill_on_drop）。
- 使用者 `stop` 後不自動重啟；`stop` 回傳時子程序已結束。
- 錯誤以 `String` 回傳（給 IPC 直接用）——是刻意的簡化，不是漏做 enum。
- 非預期退出必進 Backoff；Running 滿 60 s 歸零 attempt。
- `shutdown_all` 後所有 bot 為 Stopped。

## 禁止

- 不可在 supervisor 內直接 emit Tauri 事件 —— 正面做法：透過 `EventSink`（保持可測）。
- 不可在持有 `slots` 鎖時 await。
