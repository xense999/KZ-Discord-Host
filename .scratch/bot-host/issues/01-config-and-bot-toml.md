# 01 — config 模組與 bot.toml 解析

**What to build:** Rust 端能讀寫 `%APPDATA%\KZ Bot Host\config.json`（原子寫入、壞檔不覆寫），並能把一個含 `bot.toml` 的資料夾解析成 BotSpec（秘密值空白）；`cargo test` 驗證。

**Blocked by:** None — can start immediately

**Status:** ready-for-agent

- [ ] Config／BotSpec／EnvVar 型別，`schema_version: 1`，缺欄位有預設
- [ ] save＝寫 tmp 再 rename；load 壞 JSON → Err 且原檔不動
- [ ] bot.toml：完整／最小／缺 name／缺 exe／exe 不存在／相對路徑以資料夾為基準／secret 的 value 忽略／autostart 預設 true／cwd 預設資料夾
- [ ] 測試用 temp dir，不碰真 AppData
