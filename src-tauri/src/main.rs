// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}
 
use std::{thread,time};
use tauri::Window;
// init a background process on the command, and emit periodic events only to the window that used the command
#[tauri::command]
fn init_process(window: Window) {
  std::thread::spawn(move || {
    loop {
      window.emit("event-name", Payload { message: "Tauri is awesome!".into() }).unwrap();
      thread::sleep(time::Duration::from_millis(500));
    }
  });
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,init_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
