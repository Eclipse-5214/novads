<script lang="ts">
  let { 
    show = $bindable(false), 
    teamNumber = $bindable(9206),
    alliance = $bindable(0),
    station = $bindable(1),
    onSave
  } = $props();

  function close() {
    show = false;
  }

  function handleUpdate() {
    if (onSave) onSave();
  }
</script>

{#if show}
  <div class="fixed inset-0 z-50 bg-black/80 backdrop-blur-md flex justify-end">
    <div class="w-full max-w-xs h-full bg-neutral-900 border-l border-white/10 p-6 shadow-2xl flex flex-col gap-6">
      <div class="flex justify-between items-center">
        <h2 class="text-xl font-black italic uppercase tracking-tighter">Settings</h2>
        <button onclick={close} class="p-2 hover:bg-white/5 rounded-full text-neutral-500 text-xl">✕</button>
      </div>

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
      <p class="text-[10px] text-neutral-600 mt-auto uppercase font-bold text-center italic">NovaDS v1.0.0-ALPHA</p>
    </div>
  </div>
{/if}