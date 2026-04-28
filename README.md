# tauri-app-template-win7

[![DEPRECATED](https://img.shields.io/badge/Status-DEPRECATED-red)](#-このリポジトリは廃止-deprecated-されました)
[![Tauri](https://img.shields.io/badge/Tauri-v1.6-FFC131?logo=tauri&logoColor=white)](https://v1.tauri.app/)
[![React](https://img.shields.io/badge/React-18-61DAFB?logo=react&logoColor=white)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.8-3178C6?logo=typescript&logoColor=white)](#)
[![Go](https://img.shields.io/badge/Go-1.21+-00ADD8?logo=go&logoColor=white)](https://go.dev/)
[![Windows 10/11](https://img.shields.io/badge/Windows%2010%2F11-✓-0078D4?logo=windows&logoColor=white)](#)
[![License](https://img.shields.io/badge/License-MIT-yellow)](LICENSE)

## ⚠️ このリポジトリは廃止 (DEPRECATED) されました

**Tauri v1 + Rust stable の依存エコシステムが 2026 年 4 月時点で Windows 7 SP1 ビルドを実用的に成立させられない** ことが複数の検証で確定したため、本テンプレートは **構造例 (Tauri v1 + Go サイドカー + React) として残し、メンテは終了** します。

詳細な経緯は下記「[Win 7 SP1 ビルドの実状](#%EF%B8%8F-win-7-sp1-ビルドの実状-2026-04-時点)」セクション参照。

### 代替

| 用途 | 推奨 |
|---|---|
| **Web 技術 + Win 7 SP1 で動かしたい** | **Electron v22** (2023-01 リリース、Win 7 公式サポート最終版、Chromium 108 / Node.js 16 同梱) |
| **POS / 業務アプリ + Win 7 SP1** | **C# WPF + .NET Framework 4.8 + POS for .NET (Microsoft.PointOfService)** が業界標準 |
| **Win 10/11 のみで OK** | Tauri v2 を素直に使う |

---

**Windows 7 SP1 を視野に入れていた** Tauri v1 + React + Go のデスクトップアプリテンプレート (Win 10/11 では動作)。

## なぜ Tauri v1 ?

Tauri v2 は **Windows 10 1809 以降** が公式サポート要件のため、Win 7 SP1 / POSReady 7 / Embedded Standard 7 では起動できません。Tauri v1 系は **Win 7 SP1 + WebView2 Runtime + KB2533623** で公式サポートが謳われており、本テンプレートはそれを土台にしています。

レガシー POS 端末や古い業務 PC に新規 GUI を乗せたい等のシナリオを想定。

## ⚠️ Win 7 SP1 ビルドの実状 (2026-04 時点)

本テンプレートは Tauri v1 + Win 7 SP1 を **公式サポート構成として明記** していますが、実際にビルドして Win 7 上で起動させるには **エコシステムの新旧互換性問題** があり、2026 年 4 月現在 **stable Rust + 最新依存をそのままビルドしても起動しません**。

### 既知の障害

| 問題 | 詳細 |
|---|---|
| **Rust 1.78+ の `ProcessPrng`** | std の乱数生成が Win 10 21H2+ 限定 API `ProcessPrng@bcryptprimitives.dll` を使う。Win 7 にこの関数は無いため、起動時に「エントリ ポイントが見つかりません」ダイアログで強制終了 ([Tauri Issue #10008](https://github.com/tauri-apps/tauri/issues/10008)、[Rust PR #121337](https://github.com/rust-lang/rust/pull/121337)) |
| **transitive crate の edition 2024 要求** | `time-0.3.47`, `ignore-0.4.25`, `icu_*` 系などが Rust 1.85+ で stable 化された Rust Edition 2024 を要求。Rust 1.75 にダウングレードすると Cargo resolver が解決に失敗。個別 pin は新しい crate が次々 edition 2024 化するため持続不可能 |
| **Tauri 公式の対応状況** | [Issue #11829](https://github.com/tauri-apps/tauri/issues/11829) で edition 2024 互換性問題が報告されたが **Closed as not planned** (公式対応見込みなし) |
| **Tier 3 target `x86_64-win7-windows-msvc`** | Rust 1.78+ で追加されたが、stable では `rustup target add` 不可 (`no prebuilt artifacts` エラー)。**nightly + `-Z build-std`** が必須 |
| **Go 1.21+ の Win 7 サポート切れ** | Go 1.20.x が最後の Win 7 互換版 ([Go Issue #57003](https://github.com/golang/go/issues/57003)) |

### 検証済みの組み合わせ

| 組み合わせ | 結果 |
|---|---|
| Rust 1.95 stable + Tauri 1.8 + 最新依存 | ✅ Win 11 ビルド + 起動 OK / ❌ Win 7 起動 NG (ProcessPrng) |
| Rust 1.75 stable + Tauri 1.8 + Cargo.lock 削除 | ❌ `time-0.3.47` (edition 2024) で resolver 失敗 |
| Rust 1.75 stable + 個別 crate (~20個) pin | ❌ `ignore-0.4.25` も edition 2024 化、whack-a-mole |
| stable + `rustup target add x86_64-win7-windows-msvc` | ❌ stable に prebuilt なし (nightly 必須) |
| **nightly + Tier 3 target + `-Z build-std`** | 未検証 (理論的には動く、参照: [RustDesk Discussion #7503](https://github.com/rustdesk/rustdesk/discussions/7503) - ただし RustDesk は Tauri ではない) |

### Win 7 SP1 で実用的にビルドする現実的アプローチ

下記いずれかを **読者側の追加作業として** 想定:

1. **nightly Rust + `-Z build-std` + Tier 3 target** で 1 度 Cargo.lock を生成 → git にコミット → 以後 `cargo build --locked` で再現
2. **アプリ全体を Tauri から離れる** (Sciter / Electron 旧版 v22 / **C# WPF + .NET Framework 4.8 + POS for .NET** 等)
3. **将来 Tauri / Rust エコシステムが Win 7 互換性を改善** するのを待つ

### 本テンプレートの位置づけ

- **Win 10 / 11 用 Tauri v1 + Go テンプレート**として完全に動作 (`npm run tauri build` で MSI/EXE 生成、Win 11 で起動確認済)
- **Win 7 SP1 も視野に入れた構造**ではあるが、上記理由で 2026 年現在「ビルド成功は読者の追加作業を要する」状態
- **将来 Tauri / Rust エコシステムが Win 7 互換性を改善した場合**、本構造のまま動く想定

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
