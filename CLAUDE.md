# tauri-app-template-win7 プロジェクト指針

## 技術スタック

- **フロントエンド**: Vite 5 + React 18 + TypeScript + Bootstrap 5 + FontAwesome
- **デスクトップ基盤**: **Tauri v1.6 (Rust)** ← Win 7 SP1 サポート維持のため v2 ではなく v1 を採用
- **バックエンド**: Go (Tauri サイドカー)
- **動作対象**: Windows 7 SP1 / POSReady 7 / Embedded Standard 7 / Windows 10 / 11

## バックエンドアーキテクチャ

```
Handler → Service → Repository (interface)
                         ↓
              memory / sqlite / (将来のDB)
```

- `backend/model/` — データ構造体
- `backend/repository/` — Repository インターフェース定義
- `backend/repository/memory/` — インメモリ実装
- `backend/repository/sqlite/` — SQLite 実装 (`modernc.org/sqlite`、CGO 不要)
- `backend/service/` — ビジネスロジック (Repository interface のみ依存)
- `backend/handler/` — HTTP ハンドラー (Service 呼び出し、JSON 入出力)
- `backend/server.go` — ルーティング定義
- `backend/infra/db.go` — SQLite 接続・テーブル自動マイグレーション
- `backend/main.go` — DI・サイドカー起動 (`--db` フラグで DB パス受け取る)

## 設計ルール

### Repository
- インターフェースは `backend/repository/{entity}_repository.go`
- `memory/` と `sqlite/` の両方を必ず実装
- メソッド名: `FindByID`, `FindAll`, `Save`, `Delete` (新規 ID=0 / 更新 ID>0 で `Save` を共用)

### Service
- `repository.XxxRepository` インターフェースのみ受け取る
- エラーは `errors.New("xxx not found")` 等で簡潔に

### Handler
- `ServeHTTP` でパスとメソッドを switch
- 正常系: HTTP ステータス + JSON
- エラー系: `http.Error`

### DB (SQLite)
- テーブル追加は `backend/infra/db.go` の `NewSQLite()` 内のマイグレーション SQL に追記
- `modernc.org/sqlite` は CGO 不要のため Win/Mac/Linux クロスビルド可能

### main.go の DI
- `--db` フラグなし → インメモリ
- `--db` フラグあり → SQLite

## Tauri v1 固有の注意点 (v2 から移ってきた場合)

- `src-tauri/src/lib.rs` は使わない、`main.rs` 一本で実装 (mobile ビルドが無いため)
- `tauri.conf.json` は **v1 スキーマ** (v2 とは構造が大きく違う)
- `Cargo.toml`: `tauri = "1.6"`、`tauri-build = "1.5"`
- API: `tauri::api::process::{Command, CommandEvent}` でサイドカー起動
- React 側: `import { invoke } from "@tauri-apps/api/tauri"` (v2 は `/core`)
- プラグイン: 本テンプレートでは v1 コアのみで完結 (tauri-plugin-shell / tauri-plugin-opener は使わない)

## ビルドコマンド

```bash
# Go バイナリビルド (Go ファイル変更時に必要、tauri build は自動で呼ぶ)
./build-backend.sh    # macOS/Linux
./build-backend.ps1   # Windows (pwsh)

# 開発起動
npm run tauri dev

# 本番ビルド (Go + Vite + Tauri bundle 自動)
npm run tauri build
```

## Tauri ⇔ Go 連携

- Go サイドカーは `net.Listen(":0")` で空きポート取得し、`PORT:xxxxx` を stdout
- Rust (`main.rs`) が stdout を監視 → React に `backend-ready` イベント発火
- React の `useBackend` フックが port を受け取り `apiBase` を設定
- DB パスは `app.path_resolver().app_data_dir()` → `--db` フラグで Go に渡す

## Win 7 SP1 デプロイ前提

- WebView2 Runtime 必須 (`webviewInstallMode.type = downloadBootstrapper` で自動同梱)
- KB2533623 / KB4474419 が当たっていることを確認
- 配布形式は `.msi` (WiX) または `.exe` (NSIS) を `tauri.conf.json` の `bundle.targets` で選択
