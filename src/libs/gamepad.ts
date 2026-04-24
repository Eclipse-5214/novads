import { robotApi } from "./api";

export interface SlotState {
    assigned: boolean;
    name: string;
    gamepadIndex: number | null;
    axes: number[];
    buttons: boolean[];
    splitTriggers: boolean;
}

export interface AvailableGamepad {
    index: number;
    name: string;
}

// Remap a controller's axes to FRC-standard layout: [LX, LY, LT, RT, RX, RY].
// Reads triggers from buttons[6/7].value — works on Windows, macOS, and Android
// regardless of how the browser reports trigger axes.
function applyTriggerSplit(gp: Gamepad): number[] {
    const axes = Array.from(gp.axes).map(Number);
    return [
        axes[0] ?? 0,              // Left X
        axes[1] ?? 0,              // Left Y
        gp.buttons[6]?.value ?? 0, // Left Trigger
        gp.buttons[7]?.value ?? 0, // Right Trigger
        axes[2] ?? 0,              // Right X
        axes[3] ?? 0,              // Right Y
    ];
}

export class GamepadManager {
    private slotAssignments: (number | null)[] = new Array(6).fill(null);
    private prevGamepadIndex: (number | null)[] = new Array(6).fill(null);
    private splitTriggers: boolean[] = new Array(6).fill(false);
    private wasConnected = false;
    private onStatusChange: (connected: boolean, name: string) => void;
    private onLiveUpdate: (slots: SlotState[], available: AvailableGamepad[]) => void;

    constructor(callbacks: {
        onStatusChange: (connected: boolean, name: string) => void;
        onLiveUpdate: (slots: SlotState[], available: AvailableGamepad[]) => void;
    }) {
        this.onStatusChange = callbacks.onStatusChange;
        this.onLiveUpdate = callbacks.onLiveUpdate;
    }

    public start() {
        this.poll();
    }

    public assignSlot(slot: number, gamepadIndex: number | null) {
        this.slotAssignments[slot] = gamepadIndex;
    }

    public toggleSplitTriggers(slot: number) {
        this.splitTriggers[slot] = !this.splitTriggers[slot];
    }

    private poll = async () => {
        const raw = navigator.getGamepads();
        const available: AvailableGamepad[] = [];

        for (let i = 0; i < raw.length; i++) {
            const gp = raw[i];
            if (gp) available.push({ index: i, name: gp.id.split(" (")[0] });
        }

        // Unassign slots whose gamepad disconnected
        for (let i = 0; i < this.slotAssignments.length; i++) {
            const idx = this.slotAssignments[i];
            if (idx !== null && !raw[idx]) this.slotAssignments[i] = null;
        }

        // Auto-assign first available gamepad to slot 0
        if (!this.slotAssignments.some((s) => s !== null) && available.length > 0) {
            this.slotAssignments[0] = available[0].index;
        }

        // When a slot's assignment changes, reset split triggers to ON (default)
        for (let i = 0; i < this.slotAssignments.length; i++) {
            const gpIdx = this.slotAssignments[i];
            if (gpIdx !== this.prevGamepadIndex[i]) {
                this.splitTriggers[i] = gpIdx !== null;
                this.prevGamepadIndex[i] = gpIdx;
            }
        }

        const anyConnected = available.length > 0;
        if (anyConnected !== this.wasConnected) {
            this.wasConnected = anyConnected;
            this.onStatusChange(anyConnected, anyConnected ? available[0].name : "No Controller");
        }

        const slots: SlotState[] = this.slotAssignments.map((gpIdx, i) => {
            if (gpIdx === null) return { assigned: false, name: "", gamepadIndex: null, axes: [], buttons: [], splitTriggers: false };
            const gp = raw[gpIdx];
            if (!gp) return { assigned: false, name: "", gamepadIndex: null, axes: [], buttons: [], splitTriggers: false };
            const axes = this.splitTriggers[i]
                ? applyTriggerSplit(gp)
                : Array.from(gp.axes).map(Number);
            return {
                assigned: true,
                name: gp.id.split(" (")[0],
                gamepadIndex: gpIdx,
                axes,
                buttons: gp.buttons.map((b) => b.pressed),
                splitTriggers: this.splitTriggers[i],
            };
        });

        this.onLiveUpdate(slots, available);

        await robotApi.updateJoysticks(slots.map((s) => (s.assigned ? { axes: s.axes, buttons: s.buttons } : null)));

        requestAnimationFrame(this.poll);
    };
}
