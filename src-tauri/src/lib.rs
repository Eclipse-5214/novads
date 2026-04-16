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

#[derive(serde::Deserialize)]
struct JoystickSlot {
    axes: Vec<f32>,
    buttons: Vec<bool>,
}

static JOYSTICK_CONFIG: std::sync::OnceLock<std::sync::Mutex<Vec<Option<(usize, usize)>>>> = std::sync::OnceLock::new();

#[tauri::command]
fn update_joystick_data(joysticks: Vec<Option<JoystickSlot>>) {
    let new_config: Vec<Option<(usize, usize)>> = joysticks.iter()
        .map(|s| s.as_ref().map(|d| (d.axes.len(), d.buttons.len())))
        .collect();

    let config_lock = JOYSTICK_CONFIG.get_or_init(|| std::sync::Mutex::new(Vec::new()));
    let mut config = config_lock.lock().unwrap();

    unsafe {
        if *config != new_config {
            DS_JoysticksReset();
            for slot in &new_config {
                match slot {
                    Some((axes, buttons)) => DS_JoysticksAdd(*axes as i32, 0, *buttons as i32),
                    None => DS_JoysticksAdd(0, 0, 0),
                }
            }
            *config = new_config;
        }

        for (i, slot) in joysticks.iter().enumerate() {
            if let Some(data) = slot {
                for (j, &val) in data.axes.iter().enumerate() {
                    DS_SetJoystickAxis(i as i32, j as i32, val);
                }
                for (j, &pressed) in data.buttons.iter().enumerate() {
                    DS_SetJoystickButton(i as i32, j as i32, if pressed { 1 } else { 0 });
                }
            }
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
    println!("🌐 [RUST]: Setting robot address to: '{}'", address);
    unsafe {
        let c_addr = std::ffi::CString::new(address).unwrap();
        DS_SetCustomRobotAddress(c_addr.as_ptr());
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
                            DS_EventType_DS_NETCONSOLE_NEW_MESSAGE => {
                                let msg_ptr = event.netconsole.message;
                                if !msg_ptr.is_null() {
                                    let msg = std::ffi::CStr::from_ptr(msg_ptr).to_string_lossy().into_owned();
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
