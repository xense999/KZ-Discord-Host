# config — 模組規範（2026-08-21）

> 本模組契約的唯一 owner。建檔後記得回寫 `docs/規範.md` 歸屬總表的規範檔欄。

## 公開介面

- 型別 `Config { schema_version, bots }`、`BotSpec { id, name, exe, args, cwd?, env, autostart }`、`EnvVar { name, value, secret, description? }`。
- `load(path) -> Result<Config, ConfigError>`：檔案不存在＝預設；壞檔＝Err 且原檔不動。
- `load_or_back_up(path) -> (Config, Option<String>)`：啟動用；讀失敗時先把原檔複製成 `config.json.broken-<時間>`，回空白設定＋給 UI 的提示字串。
- `save(path, &Config)`：寫 `.json.tmp` 再 rename（原子）。
- `app_dir()`／`config_path()`／`logs_dir()`：`%APPDATA%\KZ Bot Host\…` 的唯一出處。
- `BotSpec::new_id()`、`BotSpec::effective_cwd()`、`BotSpec::normalize() -> Result<(), String>`（trim／空名空 exe 拒絕／補 id）。
- `ensure_logs_dir() -> io::Result<PathBuf>`。

## 單一來源

- 設定檔路徑與 schema 版本常數只在這裡。
- 「秘密值明文存檔」是決策(2026-08-21)，不在此模組加密。

## 不變量

- `save` 後 `load` 必等值（round-trip 測試）。
- 壞檔在被 `persist` 覆寫前一定已有備份（備份失敗時提示字串會明講）。
- `persist` 以 `AppState.persist_lock` 串行，兩個存檔不會同時寫同一個 `.tmp`。

## 禁止

- 不可在其他模組自行組 AppData 路徑 —— 正面做法：呼叫本模組的 path 函式。
