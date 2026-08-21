# autostart — 模組規範

> 本模組契約的唯一 owner。建檔後記得回寫 `docs/規範.md` 歸屬總表的規範檔欄。

## 公開介面

- `is_enabled(&AppHandle)`、`set_enabled(&AppHandle, bool)`（包 tauri-plugin-autostart，HKCU Run）。
- `launched_minimized()`；常數 `MINIMIZED_FLAG = "--minimized"`。

## 單一來源

- `--minimized` 旗標字串只在此模組（plugin 註冊與判斷共用）。
- 開機自啟「狀態」的真實來源是登錄檔，不存 config.json。

## 不變量

- 自啟一律帶 `--minimized`。

## 禁止

- 不可自己寫登錄檔 —— 正面做法：走 plugin。
