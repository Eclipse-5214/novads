import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { load } from "@tauri-apps/plugin-store";

export interface DSSettings {
    teamNumber: number;
    alliance: number;
    station: number;
}

const STORE_FILE = "settings.json";
const STORE_DEFAULTS = { teamNumber: 9206, alliance: 0, station: 1 };

export async function loadSettings(): Promise<DSSettings> {
    const store = await load(STORE_FILE, { defaults: STORE_DEFAULTS });
    return {
        teamNumber: (await store.get<number>("teamNumber")) ?? STORE_DEFAULTS.teamNumber,
        alliance: (await store.get<number>("alliance")) ?? STORE_DEFAULTS.alliance,
        station: (await store.get<number>("station")) ?? STORE_DEFAULTS.station,
    };
}

export async function saveSettings(s: DSSettings): Promise<void> {
    const store = await load(STORE_FILE, { defaults: STORE_DEFAULTS });
    await store.set("teamNumber", s.teamNumber);
    await store.set("alliance", s.alliance);
    await store.set("station", s.station);
}

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
