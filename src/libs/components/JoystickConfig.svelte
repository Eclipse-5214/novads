<script lang="ts">
  import type { SlotState, AvailableGamepad } from "../gamepad";

  let {
    show = $bindable(false),
    slots = [] as SlotState[],
    available = [] as AvailableGamepad[],
    onAssign,
    onToggleSplitTriggers,
  }: {
    show: boolean;
    slots: SlotState[];
    available: AvailableGamepad[];
    onAssign: (slot: number, gamepadIndex: number | null) => void;
    onToggleSplitTriggers: (slot: number) => void;
  } = $props();
</script>

{#if show}
  <div class="fixed inset-0 z-50 bg-black/80 backdrop-blur-md flex justify-end" onclick={() => show = false} role="presentation">
    <div class="w-full max-w-sm h-full bg-neutral-900 border-l border-white/10 p-5 shadow-2xl flex flex-col gap-4 overflow-y-auto" onclick={(e) => e.stopPropagation()} role="presentation">

      <div class="flex justify-between items-center shrink-0">
        <h2 class="text-xl font-black italic uppercase tracking-tighter">Joysticks</h2>
        <button onclick={() => show = false} class="p-2 hover:bg-white/5 rounded-full text-neutral-500 text-xl">✕</button>
      </div>

      <div class="flex flex-col gap-3">
        {#each slots as slot, i}
          <div class="bg-neutral-800/60 rounded-xl border border-white/5 p-3 flex flex-col gap-2">

            <div class="flex items-center gap-2">
              <span class="text-[10px] font-black text-purple-400 tracking-widest w-5 shrink-0">#{i}</span>
              <select
                value={slot.gamepadIndex !== null ? String(slot.gamepadIndex) : ""}
                onchange={(e) => {
                  const v = (e.target as HTMLSelectElement).value;
                  onAssign(i, v === "" ? null : parseInt(v));
                }}
                class="flex-1 bg-black border border-white/10 rounded-lg px-2 py-1 text-xs font-mono text-white focus:outline-none focus:border-purple-500 min-w-0"
              >
                <option value="">— Unassigned —</option>
                {#each available as gp}
                  <option value={String(gp.index)}>{gp.name}</option>
                {/each}
              </select>
            </div>

            {#if slot.assigned}
              <button
                onclick={() => onToggleSplitTriggers(i)}
                class="flex items-center gap-2 px-2 py-1 rounded-lg text-[10px] font-mono transition-colors {slot.splitTriggers ? 'bg-purple-600/30 text-purple-300 border border-purple-500/40' : 'bg-neutral-700/50 text-neutral-500 border border-white/5'} hover:border-purple-500/60"
                title="Split combined L/R trigger axis into separate axes (fixes Xbox on macOS)"
              >
                <span class="w-3 h-3 rounded-sm border flex items-center justify-center shrink-0 {slot.splitTriggers ? 'bg-purple-500 border-purple-400' : 'border-neutral-600'}">
                  {#if slot.splitTriggers}<span class="text-[8px] leading-none">✓</span>{/if}
                </span>
                Split triggers
              </button>

              {#if slot.axes.length > 0}
                <div class="flex flex-col gap-1">
                  {#each slot.axes as axis, j}
                    <div class="flex items-center gap-1.5">
                      <span class="text-[8px] text-neutral-600 font-mono w-3 shrink-0">{j}</span>
                      <div class="relative flex-1 h-1.5 bg-neutral-700 rounded-full overflow-hidden">
                        <div class="absolute inset-y-0 w-px bg-neutral-500 left-1/2"></div>
                        {#if axis >= 0}
                          <div class="absolute inset-y-0 bg-purple-500 rounded-full" style="left:50%; width:{axis*50}%"></div>
                        {:else}
                          <div class="absolute inset-y-0 bg-purple-500 rounded-full" style="right:50%; width:{-axis*50}%"></div>
                        {/if}
                      </div>
                      <span class="text-[8px] font-mono text-neutral-500 w-9 text-right shrink-0">{axis.toFixed(2)}</span>
                    </div>
                  {/each}
                </div>
              {/if}

              {#if slot.buttons.length > 0}
                <div class="flex flex-wrap gap-1 pt-0.5">
                  {#each slot.buttons as pressed, j}
                    <div
                      title="Button {j}"
                      class="w-5 h-5 rounded text-[7px] font-black flex items-center justify-center transition-colors {pressed ? 'bg-purple-500 text-white' : 'bg-neutral-700 text-neutral-500'}"
                    >{j}</div>
                  {/each}
                </div>
              {/if}
            {:else}
              <p class="text-[10px] text-neutral-600 italic pl-5">No controller assigned</p>
            {/if}

          </div>
        {/each}
      </div>

      {#if available.length === 0}
        <p class="text-[10px] text-neutral-600 text-center italic mt-auto">No controllers detected</p>
      {/if}

      <p class="text-[10px] text-neutral-600 mt-auto uppercase font-bold text-center italic shrink-0">NovaDS v1.0.0-ALPHA</p>
    </div>
  </div>
{/if}
