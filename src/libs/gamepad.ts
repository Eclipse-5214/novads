import { robotApi } from "./api";

export class GamepadManager {
    private connected = false;
    private onStatusChange: (status: { connected: boolean; name: string }) => void;

    constructor(onStatusChange: (status: { connected: boolean; name: string }) => void) {
        this.onStatusChange = onStatusChange;
    }

    public start() {
        this.poll();
    }

    private poll = async () => {
        const gp = navigator.getGamepads()[0];

        if (gp && !this.connected) {
            this.connected = true;
            this.onStatusChange({ connected: true, name: gp.id.split(" (")[0] });
        } else if (!gp && this.connected) {
            this.connected = false;
            this.onStatusChange({ connected: false, name: "No Controller" });
        }

        if (gp) {
            const axes = Array.from(gp.axes);
            const buttons = gp.buttons.map((b) => b.pressed);
            await robotApi.updateJoysticks(axes, buttons);
        }

        requestAnimationFrame(this.poll);
    };
}
