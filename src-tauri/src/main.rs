// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::api::process::{Command, CommandEvent};
use tauri::{Manager, State};

// ポートをアプリ状態として保持 (React がイベントを見逃した場合のフォールバック用)
struct AppState {
    backend_port: Mutex<Option<u16>>,
}

// React から直接ポートを取得するコマンド
#[tauri::command]
fn get_backend_port(state: State<AppState>) -> Option<u16> {
    *state.backend_port.lock().unwrap()
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            backend_port: Mutex::new(None),
        })
        .setup(|app| {
            let app_handle = app.handle();

            // データディレクトリを準備し、SQLite DB のパスを生成
            let data_dir = app_handle
                .path_resolver()
                .app_data_dir()
                .expect("app_data_dir の取得に失敗しました");
            std::fs::create_dir_all(&data_dir)
                .expect("データディレクトリの作成に失敗しました");
            let db_path = data_dir.join("app.db");

            // Go サイドカーを起動 (--db で DB パスを渡す)
            let (mut rx, _child) = Command::new_sidecar("backend")
                .expect("バックエンドバイナリが見つかりません")
                .args(["--db", db_path.to_str().unwrap()])
                .spawn()
                .expect("バックエンドの起動に失敗しました");

            // stdout を監視し、PORT:xxxxx を検出したら React に通知
            let app_handle_clone = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                while let Some(event) = rx.recv().await {
                    match event {
                        CommandEvent::Stdout(line) => {
                            if line.starts_with("PORT:") {
                                let port_str = line.trim_start_matches("PORT:").trim();
                                if let Ok(port) = port_str.parse::<u16>() {
                                    if let Some(state) = app_handle_clone.try_state::<AppState>() {
                                        *state.backend_port.lock().unwrap() = Some(port);
                                    }
                                    let _ = app_handle_clone.emit_all("backend-ready", port);
                                }
                            }
                        }
                        CommandEvent::Stderr(line) => {
                            eprintln!("[backend stderr] {}", line);
                        }
                        _ => {}
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_backend_port])
        .run(tauri::generate_context!())
        .expect("Tauriアプリの起動中にエラーが発生しました");
}
