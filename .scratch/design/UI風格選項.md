# 你電腦裡裝的 taste／設計 skill 全清單，與「KZ Bot Host 這種桌面小工具」的適用性

## 可直接套在 app 視窗上的（完整版面規範）
| skill | 長相 | 套在管家上會怎樣 |
|---|---|---|
| `minimalist-ui` | 暖色單色調、靠字體層次與留白、平面 bento、無漸層無重陰影 | 左邊安靜的 bot 清單、右邊展開 log；最貼「預設極簡」 |
| `industrial-brutalist-ui` | 瑞士排版＋軍規終端機：硬網格、等寬字、巨大字級對比、類比雜訊 | 像機房監控面板，看 log 很搭，但視覺重 |
| `design-taste-frontend`（主 taste skill） | 本身不是一種風格，是「讀需求→挑方向」的流程；它內建兩張表： | |
| └ 2.A 真設計系統 | Fluent（微軟）/ Material 3 / Carbon / Primer(GitHub) / Radix / shadcn… | **Fluent＝Windows 原生感**，放在系統匣旁最不突兀；Primer＝GitHub 開發工具感 |
| └ 2.B 純美學方向 | Glassmorphism / Bento / Brutalism / Editorial / **Dark tech・hacker(終端機)** / Aurora / Kinetic / Apple Liquid Glass | Dark tech＝深色＋等寬＋單一螢光強調色，log 工具常見 |
| KZ 家族 Liquid Glass | kz-auto／久世登入器同一套毛玻璃 | 品牌一致；taste skill 表裡也有列（註明 web 只是近似） |

註：`design-taste-frontend` 第 13 章把 dashboard／工具型 app 列 OUT OF SCOPE，意思是它的「落地頁流程」（AIDA、scroll 動畫）不適用；但上面兩張表的設計系統／美學方向仍可借用。

## 不適用這個案子的
| skill | 為什麼 |
|---|---|
| `high-end-visual-design` | 給「看起來很貴的網站」：字體/陰影/卡片/動畫，是行銷頁取向 |
| `gpt-taste` | GSAP 捲動動畫＋AIDA 落地頁結構，app 視窗沒有捲動敘事 |
| `design-taste-frontend-v1` | 舊版 taste，同上 |
| `redesign-existing-projects` | 給既有網站升級用，現在是從零 |
| `stitch-design-taste` | 產 Google Stitch 用的 DESIGN.md，不是給 Tauri |
| `image-to-code` / `imagegen-frontend-web` / `imagegen-frontend-mobile` / `brandkit` | 先生圖再寫碼／只生圖；網站或手機 app 取向，小工具用不到 |
| `full-output-enforcement` | 不是設計，是「禁止省略輸出」 |
