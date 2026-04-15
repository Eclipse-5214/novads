<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  // State variables
  let connected = $state(false);
  let isEnabled = $state(false); // New state for the robot toggle
  let controllerName = $state("No Controller");
  let logs = $state(["System Initialized..."]);
  let battery = $state(12.6);

  function addLog(msg: string) {
    const time = new Date().toLocaleTimeString().split(' ')[0];
    logs = [`[${time}] ${msg}`, ...logs.slice(0, 12)];
  }

  function toggleEnable() {
    isEnabled = !isEnabled;
    const cmd = isEnabled ? "ROBOT_ENABLED" : "ROBOT_DISABLED";
    addLog(cmd);
    invoke("process_robot_command", { message: cmd });
  }

  function checkGamepad() {
    const gp = navigator.getGamepads()[0];
    
    if (gp && !connected) {
      connected = true;
      controllerName = gp.id.split(" (")[0];
      addLog(`Linked: ${controllerName}`);
    } else if (!gp && connected) {
      connected = false;
      isEnabled = false; // Safety: Disable if controller disconnects
      addLog("Link Lost: Controller Disconnected");
    }

    if (gp) {
      // Gamepad Buttons: Start (9) to Enable, Back (8) to Disable (standard Xbox mapping)
      if (gp.buttons[9]?.pressed && !isEnabled) toggleEnable();
      if (gp.buttons[8]?.pressed && isEnabled) toggleEnable();
      
      // Also send periodic heartbeat if enabled
      if (isEnabled) {
          // This is where we would send joystick axes eventually
      }
    }
    
    requestAnimationFrame(checkGamepad);
  }

  onMount(() => {
    checkGamepad();
  });
</script>

<main class="h-screen w-screen bg-neutral-950 text-white p-6 flex flex-col gap-6 overflow-hidden">
  
  <header class="flex justify-between items-center bg-neutral-900/50 p-4 rounded-xl border border-white/5">
    <div class="flex flex-col">
      <h1 class="text-2xl font-black italic tracking-tighter text-purple-500 uppercase">Nova<span class="text-white">DS</span></h1>
      <span class="text-[10px] text-neutral-500 font-bold tracking-[0.3em] uppercase text-purple-400/60">Android Driver Station</span>
    </div>
    
    <div class="flex gap-4">
      <div class="bg-black/40 px-4 py-2 rounded-lg border border-white/5 flex flex-col items-end">
        <span class="text-[10px] text-neutral-500 uppercase font-bold">Battery</span>
        <span class="font-mono text-xl text-green-400">{battery}V</span>
      </div>
    </div>
  </header>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-6 flex-1">
    
    <div class="flex flex-col gap-6">
        <div class="bg-neutral-900 rounded-3xl border border-white/5 p-8 flex flex-col items-center justify-center gap-6 relative overflow-hidden">
            <div class="absolute inset-0 opacity-10" style="background-image: radial-gradient(#a855f7 0.5px, transparent 0.5px); background-size: 16px 16px;"></div>
            
            <div class="z-10 text-center">
                <span class="text-xs font-bold tracking-[0.2em] text-neutral-500 uppercase mb-4 block">Robot State</span>
                <button 
                    onclick={toggleEnable}
                    class="group relative inline-flex h-20 w-48 items-center justify-center overflow-hidden rounded-2xl font-black transition-all duration-300 active:scale-95
                    {isEnabled ? 'bg-red-600 shadow-[0_0_30px_rgba(220,38,38,0.5)]' : 'bg-purple-600 shadow-[0_0_30px_rgba(168,85,247,0.4)]'}"
                >
                    <span class="text-2xl italic tracking-tighter uppercase italic">
                        {isEnabled ? 'Disable' : 'Enable'}
                    </span>
                </button>
            </div>

            <div class="z-10 flex gap-2">
                <div class="px-3 py-1 rounded-full text-[10px] font-bold border {isEnabled ? 'bg-red-500/10 border-red-500/50 text-red-500' : 'bg-neutral-800 border-white/5 text-neutral-500'}">ESTOP</div>
                <div class="px-3 py-1 rounded-full text-[10px] font-bold border {!isEnabled ? 'bg-purple-500/10 border-purple-500/50 text-purple-500' : 'bg-neutral-800 border-white/5 text-neutral-500'}">IDLE</div>
            </div>
        </div>

        <div class="bg-neutral-900/40 rounded-3xl border border-white/5 p-6 flex items-center gap-4">
            <div class="w-12 h-12 rounded-full border border-white/10 flex items-center justify-center {connected ? 'text-purple-500' : 'text-neutral-700'}">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="6" x2="10" y1="12" y2="12"/><line x1="8" x2="8" y1="10" y2="14"/><line x1="15" x2="15.01" y1="13" y2="13"/><line x1="18" x2="18.01" y1="11" y2="11"/><rect width="20" height="12" x="2" y="6" rx="2"/></svg>
            </div>
            <div>
                <p class="text-[10px] font-bold text-neutral-500 uppercase tracking-widest">Link Status</p>
                <p class="text-sm font-bold {connected ? 'text-white' : 'text-neutral-600'}">{connected ? controllerName : 'No Input Detected'}</p>
            </div>
        </div>
    </div>

    <div class="bg-black rounded-3xl border border-white/5 p-6 font-mono text-sm flex flex-col shadow-inner relative">
      <div class="absolute top-0 right-0 p-4 opacity-10">
          <svg width="100" height="100" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1"><path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/></svg>
      </div>
      <div class="flex justify-between items-center mb-4 opacity-50 border-b border-white/10 pb-2">
        <span class="text-xs uppercase font-bold tracking-widest text-purple-400">System Log</span>
        <span class="text-[10px]">v1.0.0-ALPHA</span>
      </div>
      <div class="flex-1 overflow-y-auto space-y-1">
        {#each logs as log}
          <div class="first:text-purple-400 transition-colors duration-500">{log}</div>
        {/each}
      </div>
    </div>

  </div>

</main>