# bot_toml — 模組規範（2026-08-21）

> 本模組契約的唯一 owner。建檔後記得回寫 `docs/規範.md` 歸屬總表的規範檔欄。

## 公開介面

- `import(dir) -> Result<BotSpec, ImportError>`：讀 `<dir>/bot.toml`。
- `MANIFEST_FILE_NAME`。

## 單一來源

- `bot.toml` 欄位定義（name／exe／args／cwd／autostart／[[env]] name,secret,description,value）只在此模組的 `Manifest` 型別；`description`（頂層）目前無消費者，解析時忽略（serde 預設略過未知欄位），README 仍允許寫。

## 不變量

- 相對路徑一律以 bot.toml 所在資料夾解析；`cwd` 預設該資料夾。
- `secret = true` 的 value 一律忽略（永遠空白，交 UI 補填）。
- exe 不存在＝錯誤，不產生半套 BotSpec。
- 每次 import 產生新的 `id`。

## 禁止

- 不可在此模組讀寫 config.json —— 正面做法：回傳 BotSpec 草稿，存檔由 commands/AppState 負責。
