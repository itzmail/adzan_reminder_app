// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use adzan_reminder_lib::{AppError, JadwalResponse, Kota, PrayerService};
use serde_json::json;
use tauri::Manager;
use tauri_plugin_store::StoreExt;

fn main() {
    let service = PrayerService::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            get_cities,
            get_today_schedule,
            save_selected_city,
            load_selected_city,
            load_selected_city
        ])
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

#[tauri::command]
async fn save_selected_city(city_id: String, app: tauri::AppHandle) -> Result<(), String> {
    let store_result = app.store("settings.json");

    let store = match store_result {
        Ok(s) => s,
        Err(e) => return Err(format!("Failed to load store: {}", e)),
    };

    store.set("selected_city_id".to_string(), json!(city_id));

    if let Err(e) = store.save() {
        return Err(format!("Failed to save store: {}", e));
    }

    Ok(())
}

#[tauri::command]
async fn load_selected_city(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let store_result = app.store("settings.json");

    let store = match store_result {
        Ok(s) => s,
        Err(e) => return Err(format!("Failed to load store: {}", e)),
    };

    if let Some(value) = store.get("selected_city_id") {
        if let Some(id) = value.as_str() {
            return Ok(Some(id.to_string()));
        }
    }

    Ok(None)
}
