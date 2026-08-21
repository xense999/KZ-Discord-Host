# 06 — README（bot.toml 規格＋可丟給 AI 的接入教學）與文件回寫

**What to build:** README 有：功能一句話、安裝／啟動、`bot.toml` 每欄說明、以 Xense-Bot-rs 為範例、以及一段可整段複製給 AI 的「把我的 bot 接進 KZ Bot Host」指令文（含：改成讀環境變數、前景執行不背景化、stdout 逐行 flush、產生 bot.toml、建議改寫成 Rust 單一 exe 或自行打包 exe）。docs/現況.md、任務清單回寫。

**Blocked by:** 04, 05

**Status:** ready-for-agent

- [ ] README 繁體中文；AI 教學段落用一個獨立 code block 方便整段複製
- [ ] bot.toml 欄位表與 spec 一致（spec 為單一來源，README 只轉述）
- [ ] docs/現況.md「怎麼跑／目前可運作／實機驗收清單」更新；任務清單勾選
