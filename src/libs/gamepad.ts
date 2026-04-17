import { robotApi } from "./api";

export interface SlotState {
    assigned: boolean;
    name: string;
    gamepadIndex: number | null;
    axes: number[];
    buttons: boolean[];
}

export interface AvailableGamepad {
    index: number;
    name: string;
}

// Manages gamepad connections, slot assignments, and live updates to the robot API
export class GamepadManager {
    private slotAssignments: (number | null)[] = new Array(6).fill(null);
    private wasConnected = false;
    private onStatusChange: (connected: boolean, name: string) => void;
    private onLiveUpdate: (slots: SlotState[], available: AvailableGamepad[]) => void;

    constructor(callbacks: { onStatusChange: (connected: boolean, name: string) => void; onLiveUpdate: (slots: SlotState[], available: AvailableGamepad[]) => void }) {
        this.onStatusChange = callbacks.onStatusChange;
        this.onLiveUpdate = callbacks.onLiveUpdate;
    }

    public start() {
        this.poll();
    }

    public assignSlot(slot: number, gamepadIndex: number | null) {
        this.slotAssignments[slot] = gamepadIndex;
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

        // Auto-assign first available to slot 0 if nothing is assigned yet
        if (!this.slotAssignments.some((s) => s !== null) && available.length > 0) {
            this.slotAssignments[0] = available[0].index;
        }

        // Header connection indicator
        const anyConnected = available.length > 0;
        if (anyConnected !== this.wasConnected) {
            this.wasConnected = anyConnected;
            this.onStatusChange(anyConnected, anyConnected ? available[0].name : "No Controller");
        }

        const slots: SlotState[] = this.slotAssignments.map((gpIdx) => {
            if (gpIdx === null) return { assigned: false, name: "", gamepadIndex: null, axes: [], buttons: [] };
            const gp = raw[gpIdx];
            if (!gp) return { assigned: false, name: "", gamepadIndex: null, axes: [], buttons: [] };
            return {
                assigned: true,
                name: gp.id.split(" (")[0],
                gamepadIndex: gpIdx,
                axes: Array.from(gp.axes).map(Number),
                buttons: gp.buttons.map((b) => b.pressed),
            };
        });

        this.onLiveUpdate(slots, available);

        await robotApi.updateJoysticks(slots.map((s) => (s.assigned ? { axes: s.axes, buttons: s.buttons } : null)));

        requestAnimationFrame(this.poll);
    };
}
