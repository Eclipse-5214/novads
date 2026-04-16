<script lang="ts">
  import { Zap } from "@lucide/svelte";

  let { 
    isEnabled = false, 
    hasComms = false, 
    hasRobotCode = false, 
    selectedMode = 0,
    onToggle, 
    onModeChange 
  } = $props();

  let isLocked = $derived(!hasComms || !hasRobotCode);
</script>

<div class="bg-neutral-900 rounded-3xl border border-white/10 p-4 flex flex-col h-[60%] relative overflow-hidden shrink-0">
  <div class="absolute inset-0 opacity-[0.03] pointer-events-none" style="background-image: radial-gradient(#fff 1px, transparent 1px); background-size: 20px 20px;"></div>
  
  <div class="relative z-10 flex flex-col h-full gap-2">
    <div class="flex justify-between items-center">
      <span class="text-[10px] font-black tracking-[0.3em] text-neutral-500 uppercase">Robot Control</span>
    </div>

    <div class="flex-1 flex gap-3 items-stretch min-h-0 py-1">
      <div class="flex-3 flex items-center justify-center">
        <button 
          onclick={onToggle}
          class="w-full h-full max-h-35 rounded-3xl font-black transition-all duration-300 active:scale-[0.95] flex flex-col items-center justify-center gap-1
          {isEnabled 
            ? 'bg-red-600 shadow-[0_10px_30px_rgba(220,38,38,0.2)] border-t border-red-400/30' 
            : isLocked
              ? 'bg-neutral-800 cursor-not-allowed opacity-50 grayscale border-white/5' 
              : 'bg-purple-600 shadow-[0_10px_30px_rgba(168,85,247,0.2)] border-t border-purple-400/30'}"
        >
          <span class="text-3xl italic tracking-tighter uppercase">
            {isEnabled ? 'Disable' : isLocked ? 'Locked' : 'Enable'}
          </span>
          
          <div class="flex items-center gap-2 opacity-50">
            <Zap size={10} fill="currentColor" />
            <span class="text-[8px] uppercase tracking-[0.4em]">
              {isEnabled ? 'Active' : isLocked ? 'No Link' : 'Standby'}
            </span>
          </div>
        </button>
      </div>

      <div class="flex-1 flex flex-col gap-1.5 min-w-20">
        {#each ['Teleop', 'Auto', 'Test'] as mode, i}
          <button onclick={() => onModeChange(i)} 
            class="flex-1 rounded-xl text-[8px] font-black uppercase tracking-widest transition-all border
            {selectedMode === i ? 'bg-purple-600 border-purple-400 text-white' : 'bg-black/40 border-white/5 text-neutral-500'}">
            {mode}
          </button>
        {/each}
      </div>
    </div>
  </div>
</div>