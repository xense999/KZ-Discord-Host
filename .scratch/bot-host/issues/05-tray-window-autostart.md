# 05 — 系統匣、視窗生命週期、單一實例、開機自啟

**What to build:** 視窗 X 藏到匣；匣左鍵／雙擊顯示、右鍵「顯示」「結束」（結束＝shutdown_all 後退出、不確認）；帶 `--minimized` 啟動不顯示視窗；第二次開 exe 只把既有視窗叫出；設定列「開機自動啟動」開關寫 HKCU Run（帶 --minimized）；「開啟 log 資料夾」。

**Blocked by:** 03

**Status:** ready-for-agent

- [ ] tray-icon feature＋選單；close→prevent+hide
- [ ] single-instance plugin → show+focus
- [ ] autostart plugin（args --minimized）；get/set IPC；UI 開關反映實際狀態
- [ ] opener 開 logs 資料夾
- [ ] 用工作管理員殺掉管家後 bot 子程序也消失（Job Object 實測）
