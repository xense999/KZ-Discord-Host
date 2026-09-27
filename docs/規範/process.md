# process — 模組規範（2026-08-21）

> 本模組契約的唯一 owner。建檔後記得回寫 `docs/規範.md` 歸屬總表的規範檔欄。

## 公開介面

- `Job::new()`：kill-on-close Job Object；`Job::assign(&Child)`。
- `spawn(&BotSpec) -> io::Result<(Child, Job)>`：每次啟動開一個自己的 Job；CREATE_NO_WINDOW、stdin null、stdout/stderr piped、繼承環境＋spec.env 覆蓋（**value 為空的變數不設定**，讓 bot 自己報「缺環境變數」）、cwd＝`effective_cwd()`、spawn 後立即 assign、`kill_on_drop`。
- `decode_output(&[u8]) -> String`：一行 bot 輸出轉文字；合法 UTF-8 照用，否則用系統 ANSI 碼頁（中文 Windows＝Big5）解。

## 單一來源

- 所有 Win32 呼叫（Job Object、creation flags、碼頁轉換）只在這裡。

## 不變量

- 任何由本模組 spawn 出的子程序都掛在它自己的 Job 下，連同它再開出來的程序；Job 被 drop（停止、重啟、管家結束／被殺）整棵程序樹一律終止。
- 子程序不會出現主控台視窗。

## 禁止

- 不可在別處直接 `Command::new` 啟動 bot —— 正面做法：走 `process::spawn`。
- v1 不送 Ctrl+C／graceful stop（spec Out of Scope）。
