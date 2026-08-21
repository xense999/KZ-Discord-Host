# KZ Bot Host

Windows 系統匣常駐的 Discord bot 管家：把每隻 bot 當成一個執行檔來管——開機自啟、靜默起在系統匣、掛了自動重啟（越掛越久等）、log 即時看＋寫檔、關視窗只是藏起來、結束時所有 bot 一起收掉。

只跑 **exe**（任何語言都行，只要能產生可執行檔）。不內建 Python、Node 或任何執行環境。

## 安裝／啟動

目前為原始碼專案（Tauri 2 + Vue 3 + Rust）：

```powershell
npm install
npm run tauri dev      # 開發模式
```

打包／發版照專案慣例另行處理（見 `CLAUDE.md`）。

## 畫面

- 預設是一個 480×640 的小視窗（自訂標題列，可從右下角拖曳放大）：只有 bot 清單（名稱、狀態點、「隨管家啟動」開關）。
- 點一隻 bot 才展開它的面板：狀態、啟動／停止／重啟、編輯／移除，與即時 log（stderr 紅字、管家訊息藍字，最近 500 行）。窄視窗時面板在清單下方，拉寬到 700px 以上會變成左清單右面板。
- 標題列右側：匯入資料夾／新增／設定（開機自啟、log 資料夾、設定檔位置）。
- 標題列的 X ＝ 藏到系統匣；系統匣圖示左鍵顯示、右鍵「顯示／結束」。

## 行為規則

| 事情 | 規則 |
|---|---|
| bot 掛掉 | 自動重啟，等待 3 s → 6 s → 12 s … 最長 5 分鐘；穩定跑滿 60 s 後歸零 |
| 手動按「停止」 | 不會自動重啟 |
| 管家結束／被殺 | 所有 bot 子程序跟著結束（Windows Job Object），不留孤兒 |
| 開機自啟 | HKCU Run 鍵，不需管理員；自啟時不跳視窗 |
| Log | `%APPDATA%\KZ Bot Host\logs\<id>.log`，超過 5 MB 輪替、保留 3 份 |
| 設定 | `%APPDATA%\KZ Bot Host\config.json`，明文（含 token），換電腦可直接複製 |

## `bot.toml` 規格

放在 bot 資料夾根目錄，管家「匯入資料夾」就會讀它；秘密值（token）不寫在檔案裡，匯入後在管家補填。

```toml
name = "Xense Bot"                 # 必填：顯示名稱
description = "工廠領料 bot"        # 選填
exe = "xense-bot.exe"              # 必填：執行檔，相對於本檔所在資料夾（也可絕對路徑）
args = []                          # 選填：命令列參數
cwd = "."                          # 選填：工作目錄，預設＝本檔所在資料夾
autostart = true                   # 選填：隨管家啟動，預設 true

[[env]]                            # 可重複；每個需要的環境變數一段
name = "DISCORD_TOKEN"             # 必填
secret = true                      # 選填：true＝UI 遮罩、值由使用者在管家填，檔案裡的 value 會被忽略
description = "Discord bot token"  # 選填：顯示在輸入框提示

[[env]]
name = "MODE"
value = "prod"                     # 非秘密可直接給預設值
```

範例：[`Xense-Bot-rs`](../Xense-Bot-rs) 的 `bot.toml`。

## 把別人的 bot 接進來（可整段複製丟給 AI）

下面這段文字是寫給 AI 助理看的。把它連同你的 bot 原始碼一起給 AI，它就知道要改什麼。

````text
請把這個 Discord bot 改成能被「KZ Bot Host」管理。KZ Bot Host 是 Windows 程式，只會做一件事：
用指定的環境變數啟動一個可執行檔、讀它的 stdout/stderr、掛了就重啟。請照下面要求改：

1. 產生單一可執行檔（.exe）。
   - 最好：改寫成 Rust（serenity/poise），`cargo build --release` 產出一個 exe、零依賴。
   - 次好：維持原語言但打包成 exe（Python 用 PyInstaller --onefile、Node 用 pkg 等），
     並確認 exe 在沒裝該語言的 Windows 上能跑。
2. 所有秘密（Discord token、API key）一律改成讀「環境變數」，不可寫死在程式碼或 .bat 裡。
   例：Python `os.environ["DISCORD_TOKEN"]`、Rust `std::env::var("DISCORD_TOKEN")`。
   缺環境變數時印出明確錯誤並以非 0 的 exit code 結束。
3. 前景執行：程式啟動後就一直跑到被終止，不要自己背景化／daemonize、不要自己註冊開機啟動、
   不要開新的主控台視窗；不要自己實作「掛了自動重啟」，那是管家的事。
4. 輸出走 stdout/stderr 並逐行即時 flush（Python 請設 PYTHONUNBUFFERED=1 或 print(..., flush=True)），
   管家會即時顯示並寫檔。
5. 不要依賴 Flask/HTTP 保活、不要依賴 Render/Replit 專用環境變數。
6. 在專案根目錄新增 `bot.toml`，格式如下，`exe` 填發佈時 exe 相對於這個資料夾的路徑，
   每個需要的環境變數都列一段 `[[env]]`，秘密請標 `secret = true`（值留空，由使用者在管家填）：

   name = "<bot 名稱>"
   description = "<一句話>"
   exe = "<相對路徑>/<名稱>.exe"
   args = []
   autostart = true

   [[env]]
   name = "DISCORD_TOKEN"
   secret = true
   description = "Discord bot token"

7. 在 README 說明：怎麼 build、需要哪些環境變數、怎麼用 KZ Bot Host「匯入資料夾」接入。

參考範本：Xense-Bot-rs 專案（Rust + serenity、單一 exe、只吃 DISCORD_TOKEN、附 bot.toml），路徑由使用者一併提供。
````

接入步驟（你自己操作管家）：

1. 在 KZ Bot Host 按「匯入資料夾」，選含 `bot.toml` 的資料夾。
2. 補填標記為秘密的環境變數（token），儲存。
3. 按「啟動」；看 log 面板確認上線。

## 專案文件

AI／維護者入口：`CLAUDE.md` → `docs/現況.md`。設計定案見 `.scratch/design/共識.md`，規格與 ticket 在 `.scratch/bot-host/`。
