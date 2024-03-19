// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod user;
#[tauri::command]
fn greet(id: &str) -> String {
   format!("You Clicked on, {}!", id)
}

#[tauri::command]
fn start_game() -> String {
   return user::start_game();
}
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet,start_game])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}