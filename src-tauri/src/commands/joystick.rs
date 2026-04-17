use std::sync::{Mutex, OnceLock};
use crate::ffi::*;

#[derive(serde::Deserialize)]
pub struct JoystickSlot {
    pub axes: Vec<f32>,
    pub buttons: Vec<bool>,
}

static JOYSTICK_CONFIG: OnceLock<Mutex<Vec<Option<(usize, usize)>>>> = OnceLock::new();

#[tauri::command]
pub fn update_joystick_data(joysticks: Vec<Option<JoystickSlot>>) {
    let new_config: Vec<Option<(usize, usize)>> = joysticks.iter()
        .map(|s| s.as_ref().map(|d| (d.axes.len(), d.buttons.len())))
        .collect();

    let config_lock = JOYSTICK_CONFIG.get_or_init(|| Mutex::new(Vec::new()));
    let mut config = config_lock.lock().unwrap();

    unsafe {
        if *config != new_config {
            DS_JoysticksReset();
            for slot in &new_config {
                if let Some((axes, buttons)) = slot {
                    DS_JoysticksAdd(*axes as i32, 0, *buttons as i32);
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
pub fn reset_joysticks() {
    unsafe { DS_JoysticksReset(); }
}
