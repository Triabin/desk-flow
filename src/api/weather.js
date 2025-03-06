import { invoke } from "@tauri-apps/api/core";

export const weatherData = async () => await invoke('get_weather_data');