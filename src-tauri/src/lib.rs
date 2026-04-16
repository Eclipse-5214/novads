#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use tauri::Emitter;
use std::thread;
use std::time::Duration;

#[tauri::command]
fn process_robot_command(message: String) {
    unsafe {
        match message.as_str() {
            "ENABLE" => {
                DS_SetRobotEnabled(1);
                println!("🚀 [RUST]: Robot Enable Sent");
            },
            "DISABLE" => {
                DS_SetRobotEnabled(0);
                println!("🛑 [RUST]: Robot Disable Sent");
            },
            _ => println!("📝 [RUST]: Received {}", message),
        }
    }
}

#[tauri::command]
fn update_ds_settings(team: i32, alliance: i32, position: i32) {
    unsafe {
        DS_SetTeamNumber(team);
        DS_SetAlliance(alliance as u32);
        DS_SetPosition(position as u32);

        println!("⚙️ [RUST]: Team {} | Alliance {} | Position {}", team, alliance, position);
    }
}

#[tauri::command]
fn set_robot_mode(mode: i32) {
    unsafe { DS_SetControlMode(mode as u32); }
}

#[tauri::command]
fn update_joystick_data(axes: Vec<f32>, buttons: Vec<bool>) {
    unsafe {
        if DS_GetJoystickCount() == 0 {
            DS_JoysticksAdd(axes.len() as i32, 0, buttons.len() as i32);
        }

        for (i, val) in axes.iter().enumerate() {
            DS_SetJoystickAxis(0, i as i32, *val as f32);
        }

        for (i, pressed) in buttons.iter().enumerate() {
            DS_SetJoystickButton(0, i as i32, if *pressed { 1 } else { 0 });
        }
    }
}

#[tauri::command]
fn reset_joysticks() {
    unsafe {
        DS_JoysticksReset();
    }
}

#[tauri::command]
fn set_robot_address(address: String) {
    unsafe {
        if address.is_empty() {
            DS_SetCustomRobotAddress(std::ptr::null());
        } else {
            let c_addr = std::ffi::CString::new(address).unwrap();
            DS_SetCustomRobotAddress(c_addr.as_ptr());
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();

            unsafe {
                DS_Init();
                DS_SetTeamNumber(9206);
                let mut frc2020 = DS_GetProtocolFRC_2020();
                DS_ConfigureProtocol(&mut frc2020);
            }
            println!("💜 [RUST]: LibDS Engine Started");

            thread::spawn(move || loop {
                unsafe {
                    let mut event = DS_Event { type_: 0 };
                    while DS_PollEvent(&mut event) != 0 {
                        match event.type_ {
                            DS_EventType_DS_ROBOT_VOLTAGE_CHANGED => {
                                let voltage = event.robot.voltage;
                                let _ = handle.emit("battery-update", voltage);
                            }
                            DS_EventType_DS_ROBOT_COMMS_CHANGED => {
                                let connected = event.robot.connected != 0;
                                let _ = handle.emit("comms-update", connected);
                            }
                            DS_EventType_DS_ROBOT_CODE_CHANGED => {
                                let _ = handle.emit("code-update", event.robot.code != 0);
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
            process_robot_command,
            update_ds_settings,
            set_robot_mode,
            update_joystick_data,
            reset_joysticks,
            set_robot_address
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
