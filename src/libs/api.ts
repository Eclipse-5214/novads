import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export const robotApi = {
    enable: () => invoke("process_robot_command", { message: "ENABLE" }),
    disable: () => invoke("process_robot_command", { message: "DISABLE" }),
    setMode: (mode: number) => invoke("set_robot_mode", { mode }),
    updateSettings: (team: number, alliance: number, position: number) => invoke("update_ds_settings", { team, alliance, position }),
    updateJoysticks: (joysticks: Array<{ axes: number[]; buttons: boolean[] } | null>) => invoke("update_joystick_data", { joysticks }),
    setRobotAddress: (address: string) => invoke("set_robot_address", { address }),
};

export async function setupTauriListeners(callbacks: { onBattery: (v: number) => void; onComms: (v: boolean) => void; onCode: (v: boolean) => void; onConsole: (msg: string) => void }) {
    return [
        await listen<number>("battery-update", (e) => callbacks.onBattery(e.payload)),
        await listen<boolean>("comms-update", (e) => callbacks.onComms(e.payload)),
        await listen<boolean>("code-update", (e) => callbacks.onCode(e.payload)),
        await listen<string>("console-message", (e) => callbacks.onConsole(e.payload)),
    ];
}
