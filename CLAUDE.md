# KZ Bot Host — AI 入口（先讀我）

Discord bot 管家：Windows 系統匣常駐的 Tauri 2 + Vue 3 + Pinia + Rust 桌面程式，負責啟動／監看／自動重啟多隻外部 bot 執行檔；只跑 exe、不內建任何語言執行環境。

> **冷啟動第一步：讀 [`docs/現況.md`](docs/現況.md)** —— 它是「現在做到哪、怎麼跑、已知問題、下一步」的唯一真實來源。

## 文件地圖（要查什麼就讀哪一份）

| 文件 | 職責 |
|------|------|
| [`docs/現況.md`](docs/現況.md) | 現在做到哪、怎麼跑/停、已知問題、下一步 — **冷啟動先讀** |
| [`docs/架構藍圖.md`](docs/架構藍圖.md) | 整個程式的架構＋決策（`決策(日期)` 條目） |
| [`docs/術語表.md`](docs/術語表.md) | ubiquitous language（術語 ↔ 概念） |
| [`docs/規範.md`](docs/規範.md) | 模組歸屬總表（純索引：模組 → owner 程式 → 規範檔） — **改任何模組前先讀，再展開該模組的 `docs/規範/<模組名>.md`** |
| [`docs/任務清單.md`](docs/任務清單.md) | 各階段完成狀態（checkbox、穩定 ID） |
| [`README.md`](README.md) | 給使用者／給別人的 bot 作者：安裝、`bot.toml` 規格、可直接丟給 AI 的接入教學 |

## 鐵則（本體＝`project-conventions` skill 的 `模組化鐵則.md`，動工前讀它，此處只列名）

- 單一來源鐵則／防孤兒鐵則／模組化四判準（落位・邊界・抽取・介面窄）／文件鐵則／規範檔結構（模組契約 `docs/規範/<模組名>.md`、引用指總表）／trivial 判準。
- 繁體中文溝通；程式碼變數/函數/注解英文。
- 打包只在使用者說「打包」時做；發版時用 `release-profile` skill。

## Agent skills

### Issue tracker

本機 markdown：規格與 ticket 放在 `.scratch/<feature-slug>/`。See `docs/agents/issue-tracker.md`.

### Domain docs

Owned by the `project-conventions` skill (`docs/術語表.md` + `docs/架構藍圖.md`).
