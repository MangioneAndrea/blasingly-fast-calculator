// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub(crate) mod generic_error;
pub(crate) mod operations;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn calculate(input: &str) -> String {
    operations::token_set::TokenSet::new(input);
    format!("{}", input)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calculate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
