# 02 — supervisor 核心：啟動／log／停止／退避重啟（Rust 端＋最小 IPC）

**What to build:** 程式啟動後，設定檔裡 autostart 的 bot 被拉起（無黑視窗、掛 Job Object）、stdout/stderr 逐行進環形緩衝＋寫檔輪替、並以事件推給前端；bot 非預期退出依 3s×2^n（上限 300s）重啟、穩定 60s 歸零；手動 stop 不重啟；管家退出時 shutdown_all。先以 dev 前端按鈕或 Tauri devtools 呼叫 IPC 驗證。

**Blocked by:** 01

**Status:** ready-for-agent

- [ ] 狀態機 Stopped／Starting／Running／Backoff／Stopping；事件 `bot-state`、`bot-log`
- [ ] 子程序 CREATE_NO_WINDOW、env 覆蓋、cwd 預設 exe 目錄、assign 到 kill-on-close Job
- [ ] logs：每隻 500 行環形；檔案 5 MB 輪替保留 3 份；行首 ISO 時間＋stream
- [ ] `delay_for(attempt)` 表格測試；用 `cmd /c exit 1` 實測一輪退避重啟
- [ ] IPC：list_bots／start_bot／stop_bot／restart_bot／get_log_tail
- [ ] 同一 bot 重複 start 被拒（Err），不會起第二個程序
