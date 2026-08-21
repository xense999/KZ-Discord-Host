# 04 — 新增／編輯／移除 bot 與「匯入資料夾」

**What to build:** 頂列「匯入資料夾」選含 `bot.toml` 的資料夾→自動帶好欄位、只需補秘密值→儲存；「新增」手動填表；面板內「編輯」改任何欄位（秘密欄遮罩可切換）、「移除」會先停掉在跑的 bot；存檔即寫 config.json 並套用到 supervisor。

**Blocked by:** 03

**Status:** ready-for-agent

- [ ] IPC：upsert_bot／remove_bot／import_bot_folder（用 dialog plugin 選資料夾）
- [ ] 匯入錯誤（缺欄位／exe 不存在）在 UI 顯示可讀訊息
- [ ] 表單：名稱、exe、args（每行一個或空白分隔，擇一並在 UI 註明）、cwd、env 列表（name／value／secret）、autostart
- [ ] 移除在跑的 bot 先 stop 再刪
