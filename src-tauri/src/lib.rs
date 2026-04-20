mod commands;

pub mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use ffi::*;
use tauri::Emitter;
use std::thread;
use std::time::Duration;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let handle = app.handle().clone();

            unsafe {
                DS_Init();
                let mut frc2026 = DS_GetProtocolFRC_2026();
                DS_ConfigureProtocol(&mut frc2026);
            }

            #[allow(non_upper_case_globals)]
            thread::spawn(move || loop {
                unsafe {
                    let mut event = DS_Event { type_: 0 };
                    while DS_PollEvent(&mut event) != 0 {
                        match event.type_ {
                            DS_EventType_DS_ROBOT_VOLTAGE_CHANGED => {
                                let _ = handle.emit("battery-update", event.robot.voltage);
                            }
                            DS_EventType_DS_ROBOT_COMMS_CHANGED => {
                                let _ = handle.emit("comms-update", event.robot.connected != 0);
                            }
                            DS_EventType_DS_ROBOT_CODE_CHANGED => {
                                let _ = handle.emit("code-update", event.robot.code != 0);
                            }
                            DS_EventType_DS_NETCONSOLE_NEW_MESSAGE => {
                                let msg_ptr = event.netconsole.message;
                                if !msg_ptr.is_null() {
                                    let msg = std::ffi::CStr::from_ptr(msg_ptr)
                                        .to_string_lossy()
                                        .into_owned();
                                    let _ = handle.emit("console-message", msg);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                thread::sleep(Duration::from_millis(20));
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::process_robot_command,
            commands::update_ds_settings,
            commands::set_robot_mode,
            commands::update_joystick_data,
            commands::reset_joysticks,
            commands::set_robot_address,
            commands::get_ds_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
