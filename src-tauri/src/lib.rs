// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::AppHandle;

#[tauri::command]
fn process_robot_command(message: String, _app: AppHandle) {
    // In the future, this is where LibDS functions will live
    match message.as_str() {
        "ROBOT_ENABLED" => {
            println!("🚀 [RUST]: Sending ENABLE packet to RoboRIO...");
            // TODO: libds::enable_robot();
        },
        "ROBOT_DISABLED" => {
            println!("🛑 [RUST]: Sending DISABLE packet to RoboRIO...");
            // TODO: libds::disable_robot();
        },
        _ => println!("📝 [RUST]: Log Received: {}", message),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![process_robot_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
