# Spec：KZ Bot Host v1（Discord bot 管家）

Status: ready-for-agent
Date: 2026-08-21
來源：`.scratch/design/共識.md`（訪談定案全文）

## Problem Statement

使用者有多隻 Discord bot（自己的 Xense bot、之後別人的），每隻都是「開一個黑色 cmd 視窗跑著」，電腦重開要手動一隻一隻開、掛了不會自己起、視窗多又亂、看不到誰掛了為什麼掛。

## Solution

一個 Windows 系統匣常駐的桌面程式 KZ Bot Host：把每隻 bot 當一個可執行檔程序管理——開機自啟、靜默起在匣、自動拉起有勾的 bot、掛了指數退避自動重啟、log 即時看＋寫檔、X 只是藏起來、結束時所有 bot 一起收掉。新 bot 用 `bot.toml`「匯入資料夾」接入，或手動填表。只跑 exe，不內建任何語言執行環境。

## User Stories

### 管理 bot
1. As a 使用者, I want 主畫面一眼看到所有 bot 的名稱與狀態（停止／啟動中／執行中／等待重啟／停止中）, so that 知道誰活著。
2. As a 使用者, I want 點某隻 bot 在右側展開它的 log 與操作, so that 預設畫面保持極簡。
3. As a 使用者, I want 每隻 bot 有 啟動／停止／重啟 三個動作, so that 能手動控制。
4. As a 使用者, I want 「匯入資料夾」選一個含 `bot.toml` 的資料夾就建好一筆 bot, so that 接入新 bot 不用手抄路徑。
5. As a 使用者, I want 匯入後只需補填被標記為秘密的環境變數（token）, so that 其他欄位都自動帶好。
6. As a 使用者, I want 匯入時 `bot.toml` 缺欄位或 exe 不存在要有明確錯誤訊息, so that 知道要改哪裡。
7. As a 使用者, I want 也能手動新增一筆 bot（名稱、exe、參數、工作目錄、環境變數）, so that 沒有 bot.toml 的 exe 也能管。
8. As a 使用者, I want 編輯既有 bot 的任何欄位, so that 改 token 或路徑不用刪掉重建。
9. As a 使用者, I want 環境變數可以標記為秘密、UI 預設遮罩並可切換顯示, so that 螢幕分享時不外洩。
10. As a 使用者, I want 移除一筆 bot 時若它在跑會先停掉, so that 不會留下沒人管的程序。
11. As a 使用者, I want 每隻 bot 有「隨管家啟動」開關, so that 暫時不想跑的 bot 不用刪。
12. As a 使用者, I want 同一隻 bot 不能被啟動兩次（按鈕在執行中時 disable、後端也擋）, so that 不會兩個同 token 的 bot 互搶。

### 程序生命週期
13. As a 使用者, I want bot 非預期結束時自動重啟，3 s → 6 s → 12 s … 最長 5 分鐘, so that 不會因為暫時性錯誤停擺，也不會 token 錯了就狂重啟。
14. As a 使用者, I want bot 穩定跑滿 60 s 後退避歸零, so that 偶發 crash 不會累積成長等待。
15. As a 使用者, I want UI 顯示「等待重啟（N 秒後）」與目前第幾次重啟, so that 知道它在等。
16. As a 使用者, I want 我手動按「停止」的 bot 不會被自動重啟, so that 停止真的是停止。
17. As a 使用者, I want bot 的子程序不跳出 cmd 黑視窗, so that 桌面乾淨。
18. As a 使用者, I want 管家結束或被工作管理員殺掉時所有 bot 子程序一律跟著結束, so that 不會留孤兒 python/exe 在背景跑。
19. As a 使用者, I want 管家啟動時有勾「隨管家啟動」的 bot 自動拉起, so that 開機後什麼都不用按。

### Log
20. As a 使用者, I want 選中的 bot 面板即時顯示 stdout/stderr（stderr 視覺區別）, so that 看得到它在幹嘛。
21. As a 使用者, I want 切換到別隻 bot 再切回來時能看到它最近幾百行, so that 不用一直盯著。
22. As a 使用者, I want 每隻 bot 的 log 同時寫到檔案、超過大小自動輪替, so that 過夜掛掉隔天查得到原因、硬碟不爆。
23. As a 使用者, I want 一鍵開啟 log 資料夾, so that 找檔不用翻 AppData。
24. As a 使用者, I want log 面板有清除畫面（只清 UI 不動檔案）與自動捲到底, so that 好讀。

### 視窗／系統匣／開機
25. As a 使用者, I want 按視窗 X 只是藏到系統匣, so that 不會誤關。
26. As a 使用者, I want 系統匣圖示左鍵／雙擊顯示視窗、右鍵有「顯示」「結束」, so that 隨時叫得回來。
27. As a 使用者, I want 右鍵「結束」直接退出不跳確認, so that 結束是明確動作。
28. As a 使用者, I want 設定頁有「開機自動啟動」總開關, so that 不用自己去 shell:startup。
29. As a 使用者, I want 開機自啟時管家不跳視窗、只出現匣圖示, so that 登入桌面不被打擾。
30. As a 使用者, I want 再開一次管家 exe 只會把既有視窗叫出來, so that 不會跑兩個管家。

### 設定檔
31. As a 使用者, I want 所有設定存在 `%APPDATA%\KZ Bot Host\config.json`（明文）, so that 換電腦可以直接複製。
32. As a 使用者, I want 設定檔寫入是原子的（先寫暫存再替換）, so that 寫到一半斷電不會壞檔。
33. As a 使用者, I want 設定檔壞掉時管家仍能啟動並提示、不覆蓋原檔, so that 不會丟資料。

### 文件
34. As a 別的 bot 作者, I want README 有一段可以直接整段複製丟給 AI 的「把我的 bot 接進 KZ Bot Host」教學, so that 不用懂管家也能接入。
35. As a 使用者, I want README 寫清楚 `bot.toml` 每個欄位, so that 自己也能手寫。

## Module Breakdown（模組切分）

新開模組（新專案）。歸屬總表＝`docs/規範.md`。

| 模組 | 公開介面（對內或對前端） |
|---|---|
| config | `Config { host_autostart, bots: Vec<BotSpec> }`；`load() -> Result<Config>`、`save(&Config)`、`config_dir()`、`logs_dir()`；`BotSpec { id, name, exe, args, cwd, env: Vec<EnvVar{name,value,secret}>, autostart }` |
| bot_toml | `parse(dir) -> Result<BotSpec>`（讀 `<dir>/bot.toml`，相對路徑以 dir 為基準解析，secret 的 value 為空） |
| process | `Job::new()`（kill-on-close Job Object）、`spawn(spec, job) -> Child`（no-window、stdout/stderr piped）、`kill(child)` |
| logs | `BotLogger::new(bot_id)`；`append(stream, line)`（寫檔＋輪替）；`tail(n)`（UI 回補用環形緩衝） |
| supervisor | `Supervisor::new(app_handle, config)`；`start(id)`、`stop(id)`、`restart(id)`、`apply_config(config)`、`state(id)`、`shutdown_all()`；對前端發事件 `bot-state`、`bot-log` |
| autostart | `is_enabled()`、`set_enabled(bool)`（包 tauri-plugin-autostart，args 帶 `--minimized`） |
| tray | 建匣圖示＋選單、視窗 close→hide、`--minimized` 起手不 show、結束時呼叫 supervisor.shutdown_all |
| commands | Tauri IPC：`list_bots`、`upsert_bot`、`remove_bot`、`import_bot_folder`、`start_bot`、`stop_bot`、`restart_bot`、`get_log_tail`、`get_host_autostart`、`set_host_autostart`、`open_logs_dir` |
| 前端 store（Pinia） | `bots`、`states`、`logs`、`selectedId`、對應 actions；訂閱 `bot-state`／`bot-log` |
| 前端 UI | `App.vue` 版面（左清單／右面板）、`BotList`、`BotPanel`（log＋操作）、`BotForm`（新增／編輯）、`SettingsBar` |

邊界：commands 只轉呼叫 supervisor／config／autostart，不含邏輯；supervisor 用 process 與 logs 但不直接碰檔案格式；前端不持有權威狀態。

## Implementation Decisions

- Tauri 2（`tray-icon` feature）、plugins：`autostart`（args `--minimized`）、`single-instance`（第二實例→show+focus）、`dialog`（選資料夾）、`opener`（開 log 資料夾）。
- Rust 端 supervisor 單一權威：每隻 bot 一個 tokio task；狀態機 `Stopped | Starting | Running{pid,since} | Backoff{until,attempt} | Stopping`。使用者手動 stop 設 `desired=Stopped`，退出偵測到 `desired=Stopped` 不重啟。
- 退避公式：`delay = min(3s * 2^attempt, 300s)`；Running 連續 ≥ 60 s 後 `attempt=0`。
- 子程序：`CREATE_NO_WINDOW`；stdin 不接；stdout/stderr 各一個讀取 task 逐行送 supervisor；環境變數＝繼承父環境＋spec.env 覆蓋；cwd 預設＝exe 所在目錄。
- Job Object：程式啟動建一個，`JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`；每個子程序 spawn 後立即 `AssignProcessToJobObject`（子程序以 `CREATE_SUSPENDED` 建立→assign→resume 避免競態視為加分，v1 先 spawn 後 assign 即可）。管家結束時不依賴 Job、主動 `shutdown_all` 先 kill 再退出，Job 只是保險。
- 停止＝`TerminateProcess`（Discord bot 無需 graceful）；v1 不送 Ctrl+C。
- Log：環形緩衝每隻 500 行（UI 回補）；檔案 `logs/<id>.log`，超過 5 MB 輪替 `<id>.1.log`～`<id>.3.log`；每行前綴 ISO 時間與 stream 標記。UI 事件 `bot-log { id, ts, stream, line }` 逐行推、前端合批渲染。
- 設定：serde JSON、`schema_version: 1`；儲存＝寫 `config.json.tmp` 再 rename；讀失敗→保留原檔、UI 橫幅提示、以空設定啟動（不寫回直到使用者改動）。
- `bot.toml` 規格：
  ```toml
  name = "Xense Bot"                 # 必填
  description = "..."                # 選填
  exe = "xense-bot.exe"              # 必填；相對於 bot.toml 所在資料夾
  args = []                          # 選填
  cwd = "."                          # 選填；預設 bot.toml 所在資料夾
  autostart = true                   # 選填；預設 true

  [[env]]
  name = "DISCORD_TOKEN"             # 必填
  secret = true                      # 選填；預設 false
  description = "Discord bot token"  # 選填
  value = ""                         # 選填；secret 的 value 一律忽略、由 UI 補填
  ```
  匯入時 exe 不存在→錯誤；`id` 由管家產生（uuid v4）。
- 視窗：`--minimized` 起手不顯示；close 事件 `prevent_close` + hide；匣選單「顯示」「結束」；左鍵／雙擊 show+focus。
- UI：套 `minimalist-ui` skill；左欄固定寬 bot 清單（名稱、狀態點、autostart 開關、點選高亮），右欄空白提示「選一隻 bot」或 BotPanel；頂列：匯入資料夾／新增／設定（開機自啟開關、開 log 資料夾）。沒有 dashboard 卡片堆疊，用分隔線分組。
- 前端文案繁體中文；秘密欄預設 `type=password` 可切換。

## Testing Decisions

- 只測外部行為。Rust 單元測試：
  - config：序列化往返、缺欄位預設、壞 JSON 回 Err 不覆寫（temp dir）。
  - bot_toml：完整／最小／缺 name／exe 不存在／相對路徑解析／secret value 被忽略。
  - backoff：`delay_for(attempt)` 表格（0→3s、1→6s、…、上限 300s）。
  - logs：輪替（temp dir 寫超過門檻→產生 .1.log）、tail(n) 回最近 n 行。
  - process（整合、Windows）：spawn `cmd /c echo hi` 收到 stdout 行；kill 後 wait 回傳。
- supervisor 狀態機的「退出→Backoff→重啟」以注入假 process（trait）或用 `cmd /c exit 1` 實測一次循環（允許小延遲 1–2 s 的測試）。
- 前端：不加測試框架（無 prior art、UI 薄），以實機驗收。
- 無既有 prior art（新專案）。

## Out of Scope

- 內建 Python／uv／任何 runtime 下載。
- 秘密加密（DPAPI）——使用者選明文。
- 非 Windows 平台。
- graceful stop（Ctrl+C／SIGTERM）。
- 結束確認對話框。
- 遠端控制、通知、統計。
- 打包／發版（另叫 release-profile，使用者說「打包」才做）。

## Further Notes

- Xense-Bot-rs 是第一隻接入的 bot，也是 README 的範本；README 的 AI 教學要引用它的 `bot.toml`。
- 風險：Job Object 在管家被強制斷電時無從保證（本來就無法）；子程序 spawn 到 assign 之間的微小競態 v1 接受。
