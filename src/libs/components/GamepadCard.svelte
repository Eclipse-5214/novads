<script lang="ts">
  import { Gamepad2 } from "@lucide/svelte";
  let { connected = false, controllerName = "No Controller", onclick } = $props();
</script>

<button
  {onclick}
  class="w-full bg-neutral-900/40 rounded-2xl border border-white/5 p-4 flex items-center gap-4 shrink-0 h-20 hover:border-white/10 hover:bg-neutral-800/40 transition-colors text-left cursor-pointer"
>
  <div class="w-10 h-10 rounded-xl border border-white/10 flex items-center justify-center {connected ? 'bg-purple-600/20 text-purple-400' : 'bg-neutral-800 text-neutral-600'}">
    <Gamepad2 size={20} />
  </div>
  <div class="flex flex-col min-w-0">
    <p class="text-[8px] font-black text-neutral-500 uppercase tracking-widest">Input Interface</p>
    <p class="text-sm font-bold truncate {connected ? 'text-white' : 'text-neutral-700 italic'}">
      {connected ? controllerName : 'No Controller Linked'}
    </p>
  </div>
</button>
