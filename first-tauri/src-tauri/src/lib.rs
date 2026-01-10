mod types;
pub mod git;

use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use types::RepositoryInfo;

fn get_data_path(app: &AppHandle) -> PathBuf {
    app.path().app_data_dir().unwrap().join("userdata.json")
}

#[tauri::command]
fn save_repositories(repos: Vec<RepositoryInfo>, app: AppHandle) -> Result<(), String> {
    let data_dir = app.path().app_data_dir().unwrap();

    // 디렉토리가 없으면 생성
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    }

    let path = get_data_path(&app);
    let json = serde_json::to_string_pretty(&repos).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;

    println!("Saved {} repositories", repos.len());
    Ok(())
}

#[tauri::command]
fn load_repositories(app: AppHandle) -> Result<Vec<RepositoryInfo>, String> {
    let path = get_data_path(&app);

    if path.exists() {
        let json = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let repos: Vec<RepositoryInfo> = serde_json::from_str(&json).map_err(|e| e.to_string())?;
        println!("Loaded {} repositories", repos.len());
        Ok(repos)
    } else {
        println!("No saved repositories found");
        Ok(Vec::new())
    }
}

// 기존 greet 함수 유지
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn my_custom_command() {
    println!("I was invoked from JavaScript!");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            my_custom_command,
            save_repositories,
            load_repositories
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
