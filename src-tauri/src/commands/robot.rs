use crate::ffi::*;

#[tauri::command]
pub fn process_robot_command(message: String) {
    unsafe {
        match message.as_str() {
            "ENABLE" => {
                DS_SetRobotEnabled(1);
                println!("🚀 [RUST]: Robot Enable Sent");
            }
            "DISABLE" => {
                DS_SetRobotEnabled(0);
                println!("🛑 [RUST]: Robot Disable Sent");
            }
            _ => println!("📝 [RUST]: Received {}", message),
        }
    }
}

#[tauri::command]
pub fn update_ds_settings(team: i32, alliance: i32, position: i32) {
    unsafe {
        DS_SetTeamNumber(team);
        DS_SetAlliance(alliance as u32);
        DS_SetPosition(position as u32);
        println!("⚙️ [RUST]: Team {} | Alliance {} | Position {}", team, alliance, position);
    }
}

#[tauri::command]
pub fn set_robot_mode(mode: i32) {
    unsafe { DS_SetControlMode(mode as u32); }
}

#[tauri::command]
pub fn set_robot_address(address: String) {
    println!("🌐 [RUST]: Setting robot address to: '{}'", address);
    unsafe {
        let c_addr = std::ffi::CString::new(address).unwrap();
        DS_SetCustomRobotAddress(c_addr.as_ptr());
    }
}
