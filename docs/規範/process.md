# process — 模組規範（2026-08-21）

> 本模組契約的唯一 owner。建檔後記得回寫 `docs/規範.md` 歸屬總表的規範檔欄。

## 公開介面

- `Job::new()`：kill-on-close Job Object；`Job::assign(&Child)`。
- `spawn(&BotSpec, &Job) -> io::Result<tokio::process::Child>`：CREATE_NO_WINDOW、stdin null、stdout/stderr piped、繼承環境＋spec.env 覆蓋（**value 為空的變數不設定**，讓 bot 自己報「缺環境變數」）、cwd＝`effective_cwd()`、spawn 後立即 assign、`kill_on_drop`。

## 單一來源

- 所有 Win32 呼叫（Job Object、creation flags）只在這裡。

## 不變量

- 任何由本模組 spawn 出的子程序都掛在 Job 下；Job 被 drop（管家結束／被殺）子程序一律終止。
- 子程序不會出現主控台視窗。

## 禁止

- 不可在別處直接 `Command::new` 啟動 bot —— 正面做法：走 `process::spawn`。
- v1 不送 Ctrl+C／graceful stop（spec Out of Scope）。
