# 久世 Discord Host

![Tauri](https://img.shields.io/badge/Tauri-2-blue) ![Vue](https://img.shields.io/badge/Vue-3-42b883) ![Rust](https://img.shields.io/badge/Rust-orange)

替你顧著 Discord bot 的小管家：開機自己在系統匣待命，把登記過的 bot 一隻一隻拉起來，當掉了就自動重開。

## 功能

- 一個視窗管所有 bot（`左邊清單看狀態，點一隻才往右展開它的紀錄與操作`）
- 開機自動啟動（`登入 Windows 後直接收在系統匣，不跳視窗`）
- 每隻 bot 各自的「開啟時自啟」開關（`程式一開就把它拉起來`）
- 當掉自動重開（`等 3 秒、6 秒、12 秒……越常當等越久，最多 5 分鐘；穩定跑滿 60 秒就重新計算`）
- 手動停止就不重開（`按了「停止」的 bot 會一直停著，直到你再按「啟動」`）
- 連同子程序一起收乾淨（`停止、重開或結束本程式時，bot 自己開出來的程式也一起關掉，不留殘影`）
- 即時紀錄（`每隻 bot 的輸出即時顯示；中文輸出不會變亂碼，也不會讓紀錄中斷`）
- 紀錄寫檔（`每隻 bot 一個 .log，超過 5 MB 自動換新檔、保留三份`）
- 匯入資料夾（`資料夾裡有 bot.toml 的話，名稱、執行檔與要填的欄位自動帶入`）
- token 一律遮住（`環境變數的值平常只顯示圓點，按「顯示」才看得到`）
- 關閉收進系統匣（`按 X 只是收起來，bot 繼續跑；對系統匣圖示按右鍵「結束」才會全部停止`）
- 設定檔壞掉會先備份（`讀不到時以空白設定啟動，原檔另存一份，不會被覆蓋掉`）

## 系統需求

- Windows 10 / 11
- bot 必須是**執行檔（.exe）**。本程式不內建 Python、Node.js 等執行環境，
  還不是 exe 的 bot 請看下方〈[讓 AI 把你的 bot 轉成可以掛的版本](#讓-ai-把你的-bot-轉成可以掛的版本)〉

## 下載安裝

從 [GitHub Releases](../../releases) 下載最新版本的 `.exe` 安裝檔。

用過舊版「KZ Bot Host」的話，第一次開啟會自動把舊的 bot 與 token 搬過來、開機自啟也會換成新的；
**舊版請自己到「設定 → 應用程式」解除安裝**，兩個同時開會把同一隻 bot 跑兩份。

## 使用方式

:small_orange_diamond:**加入 bot** `兩種方式擇一`

1. **匯入**：按清單下方的「匯入」，選一個有 `bot.toml` 的資料夾，名稱、執行檔、要填的環境變數都會帶進來，補上 token 按「儲存」
2. **新增**：按「新增」手動填——名稱、執行檔（按「瀏覽」選 exe），再到「環境變數」按「新增一列」填 token（例如 `DISCORD_TOKEN`）
3. 名稱右邊的「開啟時自啟」打開的話，每次開本程式它都會自己起來

:small_orange_diamond:**看狀態、開關 bot** `點清單上的那一隻`

1. 清單每一列前面的圓點是狀態：綠＝執行中、橘＝等著自動重開、青＝啟動或停止中、灰＝已停止
2. 點一隻 bot，視窗往右展開：上方是啟動／停止／重開，下方是即時紀錄（`紅字是錯誤輸出`）
3. 紀錄往上捲就會停在原地看，按「跳到最新」回到底部
4. 再點同一隻 bot 就收回成小視窗
5. 「編輯」改設定、「移除」要在 3 秒內再按一次才會真的刪掉
6. 改的是**正在跑**的 bot 的話，要按「重開」新設定才會生效

:small_orange_diamond:**設定** `標題列左上角的齒輪`

1. 開機自動啟動：打開後登入 Windows 就會在系統匣待命
2. Log 資料夾：按「開啟」直接打開存紀錄的資料夾
3. 設定檔：點一下展開看路徑；所有 bot 的設定（含 token）都在這個檔，換電腦整個複製過去即可
4. 每個項目的說明，滑鼠移到標題上就會出現

:small_orange_diamond:**`bot.toml` 格式** `放在 bot 資料夾的最上層，給「匯入」讀`

```toml
name = "Xense Bot"                 # 必填：顯示名稱
exe = "xense-bot.exe"              # 必填：執行檔，相對於這個資料夾（也可以寫完整路徑）
autostart = true                   # 選填：開啟時自啟，預設 true

[[env]]                            # 每個需要的環境變數一段，可以重複
name = "DISCORD_TOKEN"             # 必填
secret = true                      # 選填：true＝匯入時不帶值，由使用者自己填
description = "Discord bot token"  # 選填：顯示在輸入框裡的提示
```

## 讓 AI 把你的 bot 轉成可以掛的版本

手上的 Discord bot 如果是 Python 腳本、Node.js 專案，或是 token 直接寫在程式碼／`.bat` 裡，
本程式沒辦法直接掛。把下面這整段連同 bot 的原始碼一起丟給 AI（Claude、ChatGPT 都可以），
它會幫你改成「一個 exe＋一份 bot.toml」，改完就能用「匯入」接上。

**步驟**

1. 把 bot 的整個專案資料夾（或主要的程式檔）交給 AI
2. 複製下面方框裡的整段文字貼給它
3. 照 AI 的說明 build 出 exe，確認資料夾裡有 exe 和 `bot.toml`
4. 回到本程式按「匯入」選那個資料夾，填 token、按「儲存」、按「啟動」，看紀錄確認上線
5. **舊的 token 如果曾經寫在程式碼或 .bat 裡、上傳過任何地方，請到 Discord Developer Portal 重新產生一組**

````text
請幫我把這個 Discord bot 改成能被「久世 Discord Host」掛載的版本。

久世 Discord Host 是一個 Windows 常駐程式，它只做這幾件事：
用指定的環境變數啟動一個 .exe、即時讀取它的 stdout/stderr、它結束了就自動重新啟動。
它不會安裝或提供 Python、Node.js 等任何執行環境。

請照下面的要求修改，並在最後告訴我怎麼 build：

1. 產出單一可執行檔（.exe），在沒有裝任何開發環境的 Windows 10/11 上能直接執行。
   - 首選：改寫成 Rust（serenity 或 poise），`cargo build --release` 產出一個 exe、不需要其他檔案。
   - 次選：保留原本的語言，但打包成單一 exe
     （Python 用 PyInstaller `--onefile`、Node.js 用 `pkg` 或 Node 的 single executable application）。
   - 功能、指令名稱、回覆文字請維持跟原本一樣。

2. 所有機密（Discord token、API key、資料庫密碼）一律改成從「環境變數」讀取，
   不可以寫死在程式碼、設定檔或 .bat 裡。
   例：Python `os.environ["DISCORD_TOKEN"]`、Node `process.env.DISCORD_TOKEN`、Rust `std::env::var("DISCORD_TOKEN")`。
   少了必要的環境變數時，印出清楚的錯誤訊息並以非 0 的 exit code 結束。

3. 在前景一直執行到被關掉為止：
   - 不要自己轉背景、不要自己註冊開機啟動、不要開新的主控台視窗。
   - 不要自己寫「當掉自動重啟」的迴圈，這件事由久世 Discord Host 負責。
   - 不需要 Flask / HTTP 保活伺服器，也不要依賴 Render、Replit 之類平台專用的環境變數。

4. 所有輸出寫到 stdout / stderr，而且每一行即時送出
   （Python 請用 `print(..., flush=True)` 或設定 `PYTHONUNBUFFERED=1`），
   錯誤寫到 stderr。久世 Discord Host 會即時顯示並寫成紀錄檔。

5. 程式需要讀的檔案（資料、設定、資料庫）請用「exe 所在的資料夾」當基準找，
   不要假設目前的工作目錄在別的地方。

6. 在 exe 所在的資料夾放一份 `bot.toml`，格式如下；
   程式需要的每一個環境變數都要列一段 `[[env]]`，機密的加上 `secret = true`、不要寫值：

   name = "<bot 名稱>"
   exe = "<exe 檔名>.exe"
   autostart = true

   [[env]]
   name = "DISCORD_TOKEN"
   secret = true
   description = "Discord bot token"

7. 最後用條列告訴我：
   - build 的指令，以及 exe 會產生在哪裡
   - 需要哪些環境變數、各是什麼
   - 發佈時資料夾裡要放哪些檔案
````

## 免責聲明

本軟體**不是** Discord Inc. 所開發的官方程式，與其無任何關聯。

- bot 的 token 以**明文**存在本機的設定檔（`%APPDATA%\KZ Discord Host\config.json`），不會上傳到任何地方；請自行保管好這台電腦與這個檔案。
- 本程式只負責啟動與監看你指定的執行檔，bot 本身的行為由 bot 作者負責。
- 因使用本工具導致的任何損失，作者概不負責。

## 貢獻者

- [Kuze](https://github.com/xense999) — 作者
- [Claude](https://claude.ai) — AI 協作開發

---

## 開發

```powershell
npm install
npm run tauri dev     # dev server 固定 1460 埠（1420 是久世登入器）
npm run build         # vue-tsc 型別檢查 + 前端打包
cargo test            # 在 src-tauri 底下跑，後端的監看、紀錄、設定檔與 bot.toml
```

每個模組的 owner 程式與契約文件在 [`docs/規範.md`](docs/規範.md)，那張表是唯一來源。
起手看三支就夠：`src-tauri/src/supervisor.rs`（啟動／停止／自動重開的唯一權威）、
`src/stores/bots.ts`（前端狀態與頁面切換）、`src/styles.css`（全站唯一的樣式來源，色票跟久世管理器的深色一致）。

幾條鐵則：

- **每次啟動 bot 都開一個自己的 Job Object**，停止＝關掉它，bot 開出來的子程序才會一起結束。
- **bot 的輸出以原始位元組讀**，不是 UTF-8 就用系統碼頁解；一行解不開就停止讀取的話，bot 會卡在寫入。
- **視窗一律在 `tauri.conf.json` 宣告、關閉一律 `hide`**（主視窗收進系統匣、設定視窗下次重用）。
- **dev 版不動開機自啟的舊登錄項**；要驗 dev 請另給一份 `APPDATA`，別讓它搬走安裝版的資料。

## 發版

目前**還沒有** CI，安裝檔要在本機 build：

```powershell
npm run tauri build   # 產物在 src-tauri/target/release/bundle/
```

注意：`productName` 是中文，GitHub Release 上傳時會把檔名裡的中文砍掉，
上傳前請先把安裝檔改成英文檔名（例如 `KuZe-Discord-Host_<版號>_x64-setup.exe`）。
