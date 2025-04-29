// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _, WriteType, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use std::time::Duration;
use std::sync::Arc;
use uuid::Uuid;
use tauri::command;
use futures::stream::StreamExt;
use tokio::time;
use log::{info, error};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


const DEVICE_NAME: &str = "ESP32_LED_Control";
const SERVICE_UUID: &str = "4fafc201-1fb5-459e-8fcc-c5c9c331914b";
const CHARACTERISTIC_UUID: &str = "beb5483e-36e1-4688-b7f5-ea07361b26a8";

#[command]
async fn send_ble_command(cmd: String) -> Result<String, String> {
    info!("Ble goes wroom");
    let manager = Manager::new().await.map_err(|e| e.to_string())?;
    let adapters = manager.adapters().await.map_err(|e| e.to_string())?;
    let adapter = adapters.into_iter().nth(0).ok_or("No BLE adapter found")?;

    let mut events = adapter.events().await.map_err(|e| e.to_string())?;
    adapter.start_scan(ScanFilter::default()).await.map_err(|e| e.to_string())?;

    println!("Scanning for BLE devices...");

    while let Some(event) = events.next().await {
        if let CentralEvent::DeviceDiscovered(id) | CentralEvent::DeviceUpdated(id) = event {
            println!("Discovered device ID: {:?}", id);
            let peripheral = adapter.peripheral(&id).await.map_err(|e| e.to_string())?;
            let properties = peripheral.properties().await.map_err(|e| e.to_string())?;
            println!("Properties: {:?}", properties);

            if let Some(local_name) = properties.and_then(|p| p.local_name) {
                println!("Found something");
                println!("{}", local_name.to_string());

                if local_name == DEVICE_NAME {
                    println!("Found target device: {}", local_name);
                    adapter.stop_scan().await.ok();

                    peripheral.connect().await.map_err(|e| e.to_string())?;
                    peripheral.discover_services().await.map_err(|e| e.to_string())?;

                    let service_uuid = Uuid::parse_str(SERVICE_UUID).unwrap();
                    let characteristic_uuid = Uuid::parse_str(CHARACTERISTIC_UUID).unwrap();

                    for service in peripheral.services() {
                        if service.uuid == service_uuid {
                            for characteristic in &service.characteristics {
                                if characteristic.uuid == characteristic_uuid {
                                    println!("Sending command: {}", cmd);
                                    peripheral
                                        .write(characteristic, cmd.as_bytes(), WriteType::WithoutResponse)
                                        .await
                                        .map_err(|e| e.to_string())?;
                                    peripheral.disconnect().await.ok();
                                    return Ok("Command sent successfully.".to_string());
                                }
                            }
                        }
                    }

                    return Err("Matching characteristic not found".into());
                }
            }
        }
    }

    Err("Device not found".into())
}


use tauri_plugin_log::{Target, TargetKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![send_ble_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
