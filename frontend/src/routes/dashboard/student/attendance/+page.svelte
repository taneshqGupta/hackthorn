<script lang="ts">
    import { api } from '$lib/api';
    import type { AttendanceSummary, ApiResponse } from '$lib/types';

    let summaries = $state<AttendanceSummary[]>([]);
    let loading = $state(true);

    async function loadAttendance() {
        try {
            const res = await api.get<ApiResponse<AttendanceSummary[]>>('/api/attendance/summary');
            if (res.success) summaries = res.data;
        } catch (e) {
            console.error("LOG_RETRIEVAL_FAILED", e);
        } finally {
            loading = false;
        }
    }

    $effect(() => { loadAttendance(); });
</script>

<div class="flex flex-col items-center gap-4 mb-8 w-full">
    <h1 class="text-12xl font-bold text-[#2b0b0b] tracking-tighter uppercase text-center w-full">
        Logs
    </h1>
    <p class="text-[10px] font-bold text-[#666] uppercase tracking-widest opacity-70">
        Attendance Self-Tracking
    </p>
</div>

{#if loading}
    <div class="text-center font-black uppercase opacity-50">Syncing_Records...</div>
{:else}
    <div class="flex flex-col gap-6 w-full">
        {#each summaries as item}
            <div class="border-2 border-[#2b0b0b] p-4 shadow-[4px_4px_0px_rgba(0,0,0,0.1)]">
                <div class="flex justify-between items-center border-b-2 border-[#2b0b0b] pb-2 mb-4">
                    <span class="font-black text-xl text-[#2b0b0b]">{item.course_id}</span>
                    <span class="text-2xl font-black {item.percentage < 75 ? 'text-[#b31b34]' : 'text-green-700'}">
                        {item.percentage}%
                    </span>
                </div>
                
                <div class="flex flex-col gap-2">
                    <div class="flex justify-between text-[10px] font-black uppercase">
                        <span>Present</span>
                        <span>{item.present_count} / {item.total_classes}</span>
                    </div>
                    <div class="w-full h-4 border-2 border-[#2b0b0b] bg-transparent overflow-hidden">
                        <div 
                            class="h-full bg-[#2b0b0b]" 
                            style="width: {item.percentage}%"
                        ></div>
                    </div>
                </div>

                <button 
                    class="mt-4 w-full border-2 border-[#2b0b0b] py-2 font-black uppercase text-xs hover:bg-[#2b0b0b] hover:text-white transition-colors"
                    onclick={() => {/* Open detail view or modal */}}
                >
                    View_Full_History
                </button>
            </div>
        {/each}
    </div>
{/if}