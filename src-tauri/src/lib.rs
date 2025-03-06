use std::collections::HashMap;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_weather_data() -> HashMap<String, String> {
    // TODO 调用Windows API获取天气信息逻辑
    let mut weather = HashMap::new();
    weather.insert(String::from("city"), String::from("Shanghai"));
    weather.insert(String::from("weather"), String::from("多云"));
    weather.insert(String::from("currentTemp"), String::from("13 ℃"));
    weather
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_weather_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
