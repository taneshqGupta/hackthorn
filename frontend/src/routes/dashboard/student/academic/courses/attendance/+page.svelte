<script lang="ts">
    import { api } from "$lib/api";
    import type { AttendanceSummary, ApiResponse } from "$lib/types";
    import AcademicStatsCard from "$lib/components/AcademicStatsCard.svelte";
    import AcademicSubNav from "$lib/components/AcademicSubNav.svelte";

    let summaries = $state<AttendanceSummary[]>([]);
    let loading = $state(true);

    async function loadAttendance() {
        try {
            // We'll fetch the summary for all enrolled courses
            // You might need a specific endpoint or loop through enrollments
            const res = await api.get<ApiResponse<AttendanceSummary[]>>(
                "/api/attendance/all",
            );
            if (res.success) summaries = res.data;
        } catch (e) {
            console.error("Attendance retrieval failed:", e);
        } finally {
            loading = false;
        }
    }

    $effect(() => {
        loadAttendance();
    });
</script>

<AcademicSubNav />

<div class="attendance-page">
    <div class="page-header">
        <h2 class="title">Attendance_Log</h2>
        <p class="subtitle">Self-track your presence in the citadel.</p>
    </div>

    {#if loading}
        <div class="loading">SYNCING_STATS...</div>
    {:else}
        <div class="stats-row">
            {#each summaries as summary}
                <div class="course-section">
                    <div class="course-header">
                        <h3>COURSE_ID: {summary.course_id}</h3>
                    </div>

                    <div class="cards-grid">
                        <AcademicStatsCard
                            label="Presence"
                            value="{summary.percentage}%"
                            subtext="{summary.present_count} / {summary.total_classes} Classes"
                            color="transparent"
                        />
                    </div>

                    <div class="log-table">
                        <div class="table-header">DATE | STATUS | REMARKS</div>
                        {#each summary.logs as log}
                            <div class="table-row">
                                <span>{log.date}</span>
                                <span class="status {log.status}"
                                    >{log.status}</span
                                >
                                <span class="remarks">{log.remarks || "-"}</span
                                >
                            </div>
                        {/each}
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .attendance-page {
        padding: 2rem;
    }
    .title {
        font-size: 3rem;
        font-weight: 950;
        text-transform: uppercase;
        margin: 0;
    }
    .subtitle {
        font-weight: 700;
        text-transform: uppercase;
        opacity: 0.7;
        margin-bottom: 2rem;
    }

    .course-section {
        border: 4px solid #000;
        margin-bottom: 3rem;
        padding: 1.5rem;
        box-shadow: 8px 8px 0px #000;
    }

    .course-header h3 {
        font-weight: 900;
        margin-bottom: 1.5rem;
        text-decoration: underline;
    }

    .log-table {
        margin-top: 2rem;
        font-family: monospace;
        border-top: 2px solid #000;
    }

    .table-header {
        font-weight: 900;
        padding: 0.5rem 0;
        border-bottom: 2px solid #000;
    }
    .table-row {
        display: grid;
        grid-template-columns: 1fr 1fr 2fr;
        padding: 0.5rem 0;
        border-bottom: 1px dashed #000;
    }

    .status.present {
        color: #059669;
        font-weight: 900;
    }
    .status.absent {
        color: #dc2626;
        font-weight: 900;
    }
</style>
