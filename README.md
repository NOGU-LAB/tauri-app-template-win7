# tauri-app-template-win7

[![Tauri](https://img.shields.io/badge/Tauri-v1.6-FFC131?logo=tauri&logoColor=white)](https://v1.tauri.app/)
[![React](https://img.shields.io/badge/React-18-61DAFB?logo=react&logoColor=white)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.8-3178C6?logo=typescript&logoColor=white)](#)
[![Go](https://img.shields.io/badge/Go-1.21+-00ADD8?logo=go&logoColor=white)](https://go.dev/)
[![Windows 7 SP1](https://img.shields.io/badge/Windows%207%20SP1-✓-0078D4?logo=windows&logoColor=white)](#)
[![Windows 10/11](https://img.shields.io/badge/Windows%2010%2F11-✓-0078D4?logo=windows&logoColor=white)](#)
[![License](https://img.shields.io/badge/License-MIT-yellow)](LICENSE)

**Windows 7 SP1 でも動く** Tauri v1 + React + Go のデスクトップアプリテンプレート。

## なぜ Tauri v1 ?

Tauri v2 は **Windows 10 1809 以降** が公式サポート要件のため、Win 7 SP1 / POSReady 7 / Embedded Standard 7 では起動できません。Tauri v1 系は **Win 7 SP1 + WebView2 Runtime + KB2533623** で公式サポートされており、本テンプレートはそれを土台にしています。

レガシー POS 端末や古い業務 PC に新規 GUI を乗せたい等のシナリオで利用できます。

## Win 7 SP1 動作要件

実行先 PC に以下が必要 (なければインストール):

| 必須 | 内容 | 入手元 |
|---|---|---|
| Windows 7 SP1 | POSReady 7 / Embedded Standard 7 含む | — |
| **KB2533623** (Universal C Runtime / SHA-2) | Rust 製バイナリの依存 | Microsoft Update Catalog |
| **KB4474419** (SHA-2 コード署名) | Tauri v1 のインストーラ署名検証 | Microsoft Update Catalog |
| **Microsoft Edge WebView2 Runtime** | WebView 本体 | https://developer.microsoft.com/microsoft-edge/webview2/ |
| .NET Framework 4.0+ | (任意) OPOS で周辺機器を叩く場合 | Win 7 SP1 標準 |

`tauri.conf.json` の `webviewInstallMode.type = downloadBootstrapper` により、ビルドで生成されるインストーラに **WebView2 ランタイムインストーラが同梱** されます。実行時に未導入なら自動 DL → 導入。

## 技術スタック

| レイヤー | 技術 |
|---|---|
| フロントエンド | Vite 5 + React 18 + TypeScript |
| UI ライブラリ | Bootstrap 5 + React-Bootstrap + FontAwesome |
| デスクトップ基盤 | **Tauri v1.6 (Rust)** |
| バックエンド | Go (サイドカー) |
| アーキテクチャ | Handler → Service → Repository |

## 接続構成

```
   開発機 (Win 10/11 / macOS)             POS 端末 (Win 7 SP1)
 +----------------------------+         +-------------------------+
 |                            |         |                         |
 |  npm run tauri build       |         |  installer.msi 実行     |
 |    │                       |         |    │                    |
 |    ├─ Vite build (React)   |         |    ├─ WebView2 Runtime  |
 |    ├─ go build  (sidecar)  |         |    │   自動 DL/導入     |
 |    └─ Tauri bundle         |         |    │                    |
 |        │                   |         |    └─ アプリ展開        |
 |        ▼                   |         |        │                |
 |  *.msi / *-setup.exe       |=======> |        ▼                |
 |                            |         |  ┌─────────────────┐    |
 +----------------------------+         |  │ tauri-app.exe   │    |
                                        |  │  ├─ WebView2    │    |
                                        |  │  │   (HTML/JS)  │    |
                                        |  │  └─ backend.exe │    |
                                        |  │     (Go HTTP)   │    |
                                        |  └─────────────────┘    |
                                        +-------------------------+
```

## ディレクトリ構成

```
tauri-app-template-win7/
├── src/                  # React + TS フロントエンド
│   ├── App.tsx
│   ├── main.tsx
│   └── hooks/useBackend.ts
├── src-tauri/            # Tauri (Rust) コア
│   ├── Cargo.toml        # Tauri v1.6 依存
│   ├── tauri.conf.json   # v1 schema
│   ├── icons/
│   └── src/main.rs       # Sidecar 起動 + ポート監視
├── backend/              # Go サイドカー
│   ├── handler/
│   ├── service/
│   ├── repository/
│   ├── infra/db.go
│   └── main.go
├── build-backend.sh      # Go ビルド (macOS/Linux)
├── build-backend.ps1     # 同 (Windows pwsh)
├── package.json
└── README.md
```

## 開発環境セットアップ

開発機の前提:
- Rust toolchain (`rustup`)
- Node.js 18+
- Go 1.21+
- (Windows のみ) Visual Studio Build Tools 2019+ または C++ Build Tools

```bash
git clone https://github.com/NOGU-LAB/tauri-app-template-win7
cd tauri-app-template-win7
npm install
```

## 開発起動

```bash
npm run tauri dev
```

WebView ウィンドウが立ち上がり、Go サイドカーがバックグラウンドで動きます。

## 本番ビルド (Win 7 SP1 で動く .msi / .exe)

```bash
npm run tauri build
```

出力:
- `src-tauri/target/release/bundle/msi/tauri-app_0.1.0_x64_en-US.msi`
- `src-tauri/target/release/bundle/nsis/tauri-app_0.1.0_x64-setup.exe`

> **注意**: macOS から Windows ターゲットをビルドするのは公式非対応です。**Win 10/11 開発機でビルド → 成果物を Win 7 SP1 機に持っていく** 流れを推奨。

## Win 7 SP1 へのデプロイ

1. ビルド済の `.msi` または `.exe` を Win 7 SP1 機にコピー
2. (初回のみ) WebView2 Runtime を入れる (インストーラ初回起動時に自動 DL される)
3. インストーラを実行
4. スタートメニュー / デスクトップから起動

## バックエンドアーキテクチャ

```
[React (Tauri WebView)]
       │
       │ HTTP fetch (apiBase = http://localhost:<dynamic port>)
       ▼
[Go Sidecar]
   ├─ Handler 層 (HTTP I/O)
   ├─ Service 層 (ビジネスロジック)
   └─ Repository 層 (interface)
        ├─ memory   (開発・テスト用)
        └─ sqlite   (本番、modernc.org/sqlite で CGO 不要)
```

## Tauri ⇔ Go 連携シーケンス

1. Tauri (Rust) が起動時、`Command::new_sidecar("backend")` で Go バイナリを spawn
2. Go が `net.Listen(":0")` で空きポート取得 → `PORT:xxxxx` を stdout
3. Rust (`main.rs`) が stdout を監視 → React に `backend-ready` イベント発行
4. React の `useBackend` フックが port を受け取り `apiBase` 設定 (取りこぼし時は `invoke('get_backend_port')` でフォールバック)

## ライセンス

MIT
