<script lang="ts">
  import { robotApi } from "../api";

  let {
    show = $bindable(false),
    teamNumber = $bindable(9206),
    alliance = $bindable(0),
    station = $bindable(1),
    onSave
  } = $props();

  let simMode = $state(false);

  function close() {
    show = false;
  }

  function handleUpdate() {
    if (onSave) onSave();
  }

  async function toggleSimMode() {
    simMode = !simMode;
    await robotApi.setRobotAddress(simMode ? "127.0.0.1" : "");
  }
</script>

{#if show}
  <div class="fixed inset-0 z-50 bg-black/80 backdrop-blur-md flex justify-end">
    <div class="w-full max-w-xs h-full bg-neutral-900 border-l border-white/10 shadow-2xl flex flex-col">
      <div class="flex justify-between items-center p-6 pb-0 shrink-0">
        <h2 class="text-xl font-black italic uppercase tracking-tighter">Settings</h2>
        <button onclick={close} class="p-2 hover:bg-white/5 rounded-full text-neutral-500 text-xl">✕</button>
      </div>

      <div class="flex-1 overflow-y-auto px-6 py-6 flex flex-col gap-6 min-h-0">
        <div class="space-y-6">
          <label class="block">
            <span class="text-[10px] font-bold text-neutral-500 uppercase tracking-widest block mb-2">Team Number</span>
            <input type="number" bind:value={teamNumber} onchange={handleUpdate}
              class="w-full bg-black border border-white/10 rounded-xl p-3 font-mono text-purple-400 focus:outline-none focus:border-purple-500" />
          </label>

          <div>
            <span class="text-[10px] font-bold text-neutral-500 uppercase tracking-widest block mb-2">Alliance Selection</span>
            <div class="grid grid-cols-2 gap-2 mb-4">
              <button onclick={() => {alliance = 0; handleUpdate()}} class="p-3 rounded-xl border font-bold transition-all {alliance === 0 ? 'bg-red-600 border-red-500 text-white shadow-lg shadow-red-600/20' : 'bg-black border-white/5 text-neutral-600'}">RED</button>
              <button onclick={() => {alliance = 1; handleUpdate()}} class="p-3 rounded-xl border font-bold transition-all {alliance === 1 ? 'bg-blue-600 border-blue-500 text-white shadow-lg shadow-blue-600/20' : 'bg-black border-white/5 text-neutral-600'}">BLUE</button>
            </div>

            <span class="text-[10px] font-bold text-neutral-500 uppercase tracking-widest block mb-2">Station Position</span>
            <div class="grid grid-cols-3 gap-2">
              {#each [1, 2, 3] as s}
                <button onclick={() => {station = s; handleUpdate()}}
                  class="p-3 rounded-xl border font-bold transition-all {station === s ? 'bg-purple-600 border-purple-500 text-white' : 'bg-black border-white/5 text-neutral-600'}">{s}</button>
              {/each}
            </div>
          </div>
        </div>

        <div>
          <span class="text-[10px] font-bold text-neutral-500 uppercase tracking-widest block mb-2">Simulation</span>
          <button onclick={toggleSimMode}
            class="w-full p-3 rounded-xl border font-bold transition-all {simMode ? 'bg-yellow-500/20 border-yellow-500/50 text-yellow-400' : 'bg-black border-white/5 text-neutral-600'}">
            {simMode ? 'SIM MODE ON — 127.0.0.1' : 'Enable Sim Mode'}
          </button>
          {#if simMode}
            <p class="text-[10px] text-yellow-600 mt-2 text-center">Connecting to local WPILib simulation</p>
          {/if}
        </div>

        <p class="text-[10px] text-neutral-600 uppercase font-bold text-center italic">NovaDS v1.0.0-ALPHA</p>
      </div>
    </div>
  </div>
{/if}