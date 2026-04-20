use crate::ffi::*;

#[tauri::command]
pub fn process_robot_command(message: String) {
    unsafe {
        match message.as_str() {
            "ENABLE" => {
                DS_SetRobotEnabled(1);
            }
            "DISABLE" => {
                DS_SetRobotEnabled(0);
            }
            _ => {}
        }
    }
}

#[tauri::command]
pub fn update_ds_settings(team: i32, alliance: i32, position: i32) {
    unsafe {
        DS_SetTeamNumber(team);
        DS_SetAlliance(alliance as u32);
        DS_SetPosition(position as u32);
    }
}

#[tauri::command]
pub fn set_robot_mode(mode: i32) {
    unsafe { DS_SetControlMode(mode as u32); }
}

#[tauri::command]
pub fn get_ds_status() -> (bool, bool, f32) {
    unsafe {
        (
            DS_GetRobotCommunications() != 0,
            DS_GetRobotCode() != 0,
            DS_GetRobotVoltage(),
        )
    }
}

#[tauri::command]
pub fn set_robot_address(address: String) {
    unsafe {
        let c_addr = std::ffi::CString::new(address).unwrap();
        DS_SetCustomRobotAddress(c_addr.as_ptr());
    }
}
