// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use adzan_reminder_lib::{AppError, PrayerService, Kota, JadwalResponse};
use tauri::Manager;

fn main() {
    let service = PrayerService::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![get_cities, get_today_schedule])
        .setup(move |app| {
            app.manage(service);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    adzan_reminder_lib::run()
}

#[tauri::command]
async fn get_cities(service: tauri::State<'_, PrayerService>) -> Result<Vec<Kota>, String> {
    service
        .get_cities()
        .await
        .map_err(|e: AppError| e.to_string())
}

#[tauri::command]
async fn get_today_schedule(
    city_id: String,
    service: tauri::State<'_, PrayerService>,
) -> Result<JadwalResponse, String> {
    service
        .get_today_schedule(&city_id)
        .await
        .map_err(|e: AppError| e.to_string())
}
