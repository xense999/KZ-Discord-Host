# logs — 模組規範（2026-08-21）

> 本模組契約的唯一 owner。建檔後記得回寫 `docs/規範.md` 歸屬總表的規範檔欄。

## 公開介面

- `BotLogger::new(logs_dir, bot_id)`；`append(Stream, line) -> LogLine`；`tail() -> Vec<LogLine>`；`path()`。
- 型別 `LogLine { ts, stream, line }`、`Stream { Stdout, Stderr, System }`。
- 常數 `RING_CAPACITY=500`、`MAX_FILE_BYTES=5MB`、`KEEP_ROTATED=3`。

## 單一來源

- 檔名規則 `<id>.log`、`<id>.1.log`…與行格式 `ts [OUT|ERR|SYS] line` 只在這裡。

## 不變量

- 環形緩衝永不超過 `RING_CAPACITY`。
- 寫檔失敗不會讓 `append` panic 或回傳錯誤（硬碟滿不拖垮 supervisor）。

## 禁止

- 不可讓前端直接讀 log 檔當即時來源 —— 正面做法：事件 `bot-log` ＋ `get_log_tail`。
