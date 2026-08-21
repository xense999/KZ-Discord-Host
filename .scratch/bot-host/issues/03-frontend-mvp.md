# 03 — 前端 MVP：清單＋側欄 log＋啟停重啟（minimalist-ui）

**What to build:** 打開視窗看到極簡 bot 清單（名稱、狀態點、autostart 開關）；點一隻右側展開 log 面板（stderr 區別、自動捲底、清畫面）與 啟動／停止／重啟；狀態與 log 即時更新；切換 bot 回補最近 500 行；Backoff 顯示倒數與第幾次。

**Blocked by:** 02

**Status:** ready-for-agent

- [ ] Pinia store 訂閱 `bot-state`／`bot-log`，log 合批渲染
- [ ] 執行中 start 鈕 disable、停止中 stop 鈕 disable
- [ ] 套 minimalist-ui：暖色單色、字體層次、分隔線分組、無漸層無重陰影；文案繁體中文
- [ ] 未選任何 bot 時右側只有一行提示
