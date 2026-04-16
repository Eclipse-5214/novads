<script lang="ts">
  import { onMount } from "svelte";
  import { robotApi, setupTauriListeners } from "../libs/api";
  import { GamepadManager, type SlotState, type AvailableGamepad } from "../libs/gamepad";
  import StatusHeader from "../libs/components/StatusHeader.svelte";
  import ControlPanel from "../libs/components/ControlPanel.svelte";
  import GamepadCard from "../libs/components/GamepadCard.svelte";
  import Logger from "../libs/components/Logger.svelte";
  import Settings from "../libs/components/Settings.svelte";
  import JoystickConfig from "../libs/components/JoystickConfig.svelte";

  let connected = $state(false);
  let isEnabled = $state(false);
  let controllerName = $state("No Controller");
  let logs = $state(["System Initialized..."]);
  let battery = $state(0.0);
  let hasComms = $state(false);
  let hasRobotCode = $state(false);

  let teamNumber = $state(9206);
  let selectedMode = $state(0);
  let alliance = $state(0);
  let station = $state(1);
  let showSettings = $state(false);
  let showJoystickConfig = $state(false);
  let slotStates = $state<SlotState[]>(new Array(6).fill({ assigned: false, name: "", gamepadIndex: null, axes: [], buttons: [] }));
  let availableGamepads = $state<AvailableGamepad[]>([]);

  let gm: GamepadManager;

  function addLog(msg: string) {
    const time = new Date().toLocaleTimeString().split(' ')[0];
    logs = [`[${time}] ${msg}`, ...logs.slice(0, 20)];
  }

  async function updateSettings() {
    await robotApi.updateSettings(teamNumber, alliance, station);
    addLog(`Settings Sync: Team ${teamNumber}`);
  }

  async function changeMode(m: number) {
    if (isEnabled) {
      addLog("Safety: Disable robot to change mode");
      return;
    }
    selectedMode = m;
    await robotApi.setMode(m);
    addLog(`Mode: ${['Teleop', 'Auto', 'Test'][m]}`);
  }

  function toggleEnable() {
    if (!isEnabled && (!hasComms || !hasRobotCode)) {
      addLog("Safety: Cannot enable without Comms & Code");
      return;
    }
    isEnabled = !isEnabled;
    isEnabled ? robotApi.enable() : robotApi.disable();
    addLog(isEnabled ? "Robot Enabled" : "Robot Disabled");
  }

  onMount(() => {
    let unlisteners: (() => void)[] = [];

    gm = new GamepadManager({
      onStatusChange: (isConnected, name) => {
        connected = isConnected;
        controllerName = name;
        if (isConnected) addLog(`Linked: ${name}`);
        else addLog("Link Lost: Controller Disconnected");
      },
      onLiveUpdate: (slots, available) => {
        slotStates = slots;
        availableGamepads = available;
      },
    });
    gm.start();

    const initEvents = async () => {
      await updateSettings();
      const subs = await setupTauriListeners({
        onBattery: (v) => { battery = parseFloat(v.toFixed(2)); },
        onComms: (v) => {
          hasComms = v;
          if (!hasComms && isEnabled) {
            isEnabled = false;
            addLog("Safety: Comms Lost - Auto Disabled");
          }
        },
        onCode: (v) => { hasRobotCode = v; },
        onConsole: (msg) => { addLog(`RIO: ${msg.trim()}`); },
      });
      unlisteners = subs;
    };

    initEvents();

    return () => unlisteners.forEach(fn => fn());
  });
</script>

<main class="h-screen w-screen bg-neutral-950 text-white p-4 flex flex-col gap-4 overflow-hidden select-none">
  <StatusHeader
    {hasComms}
    {hasRobotCode}
    hasJoysticks={connected}
    {battery}
    onOpenSettings={() => showSettings = true}
  />

  <div class="grid grid-cols-[1.2fr_1fr] gap-4 flex-1 min-h-0">
    <div class="flex flex-col gap-4 min-h-0">
      <ControlPanel
        {isEnabled}
        {hasComms}
        {hasRobotCode}
        {selectedMode}
        onToggle={toggleEnable}
        onModeChange={changeMode}
      />

      <GamepadCard
        {connected}
        {controllerName}
        onclick={() => showJoystickConfig = true}
      />
    </div>

    <Logger {logs} />
  </div>

  <Settings
    bind:show={showSettings}
    bind:teamNumber
    bind:alliance
    bind:station
    onSave={updateSettings}
  />

  <JoystickConfig
    bind:show={showJoystickConfig}
    slots={slotStates}
    available={availableGamepads}
    onAssign={(slot, idx) => gm.assignSlot(slot, idx)}
  />
</main>
