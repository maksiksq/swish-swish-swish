use tracing::info;
use uuid::{uuid, Uuid};
use windows::Devices::Radios::{Radio, RadioKind};

// const DEVICE_NAME: &str = "ESP32_LED_Control";
const CHARACTERISTIC_UUID: Uuid = uuid!("beb5483e-36e1-4688-b7f5-ea07361b26a8");

// here we send the command to our ESP32 via BLE
#[tauri::command]
pub async fn send_ble_command(cmd: String) -> bool {
    info!("Rust received the command: {}", cmd.clone());
    let data = cmd.as_bytes();
    let handler = tauri_plugin_blec::get_handler().unwrap();
    let start = std::time::Instant::now();
    handler
        .send_data(
            CHARACTERISTIC_UUID,
            data,
            tauri_plugin_blec::models::WriteType::WithoutResponse,
        )
        .await
        .unwrap();
    let response = handler.recv_data(CHARACTERISTIC_UUID).await.unwrap();
    info!("Response: {:?}", response.clone());
    let time = start.elapsed();
    info!("Time elapsed: {:?}", time);
    assert_eq!(response, data);
    true
}

