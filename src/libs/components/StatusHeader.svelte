<script lang="ts">
  import { Settings } from "@lucide/svelte";

  let { 
    hasComms = false, 
    hasRobotCode = false, 
    hasJoysticks = false, 
    battery = 0.0,
    onOpenSettings
  } = $props();
</script>

<header class="flex justify-between items-center bg-neutral-900/50 p-3 rounded-2xl border border-white/5 shrink-0">
    <div class="flex flex-col">
        <h1 class="text-xl font-black italic tracking-tighter text-purple-500 uppercase leading-none">Nova<span class="text-white">DS</span></h1>
        <span class="text-[8px] text-neutral-500 font-bold tracking-[0.2em] uppercase">Android Driver Station</span>
    </div>
    
    <div class="flex gap-3 items-center">
      <div class="flex gap-4 bg-black/40 px-4 py-1.5 rounded-xl border border-white/5 items-center">
        <div class="flex gap-3 pr-4 border-r border-white/10">
            {#each [{l: 'COMMS', v: hasComms}, {l: 'CODE', v: hasRobotCode}, {l: 'JOY', v: hasJoysticks}] as item}
                <div class="flex flex-col items-center">
                    <div class="w-1.5 h-1.5 rounded-full shadow-[0_0_8px] {item.v ? 'bg-green-500 shadow-green-500/50' : 'bg-red-500 shadow-red-500/50'}"></div>
                    <span class="text-[6px] uppercase font-black text-neutral-600 mt-1">{item.l}</span>
                </div>
            {/each}
        </div>
        <div class="flex flex-col items-end leading-none">
          <span class="text-[7px] text-neutral-500 uppercase font-black mb-0.5">Battery</span>
          <span class="font-mono text-lg text-green-400 font-bold">{battery}V</span>
        </div>
      </div>

      <button onclick={onOpenSettings} class="p-2.5 bg-neutral-800/50 rounded-xl hover:text-purple-400 transition-all border border-white/5 active:scale-95">
        <Settings size={18} />
      </button>
    </div>
</header>