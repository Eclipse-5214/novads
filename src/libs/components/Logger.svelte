<script lang="ts">
  import { Activity } from "@lucide/svelte";
  let { logs = [] } = $props();
</script>

<div class="bg-black/60 rounded-3xl border border-white/5 p-5 font-mono flex flex-col shadow-2xl relative overflow-hidden min-h-0 h-full">
  <div class="flex justify-between items-center mb-4 border-b border-white/5 pb-3 shrink-0">
    <div class="flex items-center gap-2">
      <Activity size={14} class="text-purple-500" />
      <span class="text-[10px] uppercase font-black tracking-[0.3em] text-purple-400">System Logs</span>
    </div>
    <div class="flex items-center gap-2">
      <div class="w-1.5 h-1.5 rounded-full bg-purple-500 animate-pulse"></div>
      <span class="text-[8px] font-bold text-neutral-700 tracking-widest">LIVE_STREAM</span>
    </div>
  </div>
  
  <div class="flex-1 overflow-y-auto space-y-2 pr-2 custom-scrollbar">
    {#each logs as log}
      <div class="text-[11px] leading-relaxed border-l-2 border-transparent transition-all duration-300 hover:border-purple-500/50 hover:pl-2">
        <span class="text-neutral-700 mr-2 text-[9px] font-bold">{log.split(' ')[0]}</span>
        <span class={log.includes('Enabled') ? 'text-green-400' : log.includes('Disabled') ? 'text-red-400' : 'text-neutral-300'}>
            {log.split(' ').slice(1).join(' ')}
        </span>
      </div>
    {/each}
  </div>
</div>

<style>
  .custom-scrollbar::-webkit-scrollbar {
    width: 2px;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(168, 85, 247, 0.2);
    border-radius: 10px;
  }
</style>