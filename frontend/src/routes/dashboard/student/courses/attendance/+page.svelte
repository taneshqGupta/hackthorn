<script lang="ts">
    import { onMount } from "svelte";
    import { page } from "$app/stores";
    import api from "$lib/api";
    import type {
        CourseResponse,
        AttendanceSummary,
        ApiResponse,
    } from "$lib/types";

    // Data State
    let courses = $state<CourseResponse[]>([]);
    let attendanceData = $state<Record<string, AttendanceSummary>>({});
    let loading = $state(true);
    let expandedCourseId = $state<string | null>(null);

    // Initial load logic
    onMount(async () => {
        // 1. Check if URL has a specific ID to auto-expand
        const queryId = $page.url.searchParams.get("id");
        if (queryId) expandedCourseId = queryId;

        await loadAllData();
    });

    async function loadAllData() {
        loading = true;
        try {
            // A. Get Enrollments first
            const coursesRes = await api.get<ApiResponse<CourseResponse[]>>(
                "/api/courses/my-enrollments",
            );
            courses = coursesRes.data || [];

            // B. Parallel fetch attendance for all courses
            const promises = courses.map((c) =>
                api
                    .get<ApiResponse<AttendanceSummary>>(
                        `/api/attendance/${c.id}`,
                    )
                    .then((res) => ({ id: c.id, data: res.data }))
                    .catch(() => ({ id: c.id, data: null })),
            );

            const results = await Promise.all(promises);

            // Map results to state
            results.forEach((r) => {
                if (r.data) attendanceData[r.id] = r.data;
            });
        } catch (err) {
            console.error("Error loading attendance data", err);
        } finally {
            loading = false;
        }
    }

    function toggleExpand(id: string) {
        if (expandedCourseId === id) {
            expandedCourseId = null;
        } else {
            expandedCourseId = id;
        }
    }

    function getStatusColor(status: string) {
        switch (status) {
            case "present":
                return "text-green-700 font-bold";
            case "absent":
                return "text-[#b31b34] font-bold";
            default:
                return "text-gray-500 italic";
        }
    }

    function getPercentageColor(pct: number) {
        if (pct >= 75) return "text-[#2b0b0b]"; // Standard
        if (pct >= 60) return "text-orange-600";
        return "text-[#b31b34]"; // Danger
    }
</script>

<div class="container">
    <h1 class="page-title">ATTENDANCE</h1>

    {#if loading}
        <div class="loading">CALCULATING...</div>
    {:else}
        <div class="w-full flex flex-col gap-4">
            {#each courses as course}
                {@const stats = attendanceData[course.id]}

                <button
                    class="stats-card w-full text-left"
                    class:expanded={expandedCourseId === course.id}
                    onclick={() => toggleExpand(course.id)}
                >
                    <div class="flex justify-between items-center mb-1">
                        <span
                            class="text-xl font-bold bg-[#2b0b0b] text-white px-2 py-1"
                        >
                            {course.code}
                        </span>
                        {#if stats}
                            <span
                                class={`text-3xl font-black ${getPercentageColor(stats.percentage)}`}
                            >
                                {stats.percentage.toFixed(0)}%
                            </span>
                        {:else}
                            <span class="text-gray-400">--</span>
                        {/if}
                    </div>

                    <div
                        class="text-sm uppercase font-bold opacity-80 mb-2 truncate"
                    >
                        {course.title}
                    </div>

                    {#if stats}
                        <div
                            class="w-full h-4 border-2 border-[#2b0b0b] p-[2px] relative"
                        >
                            <div
                                class="h-full bg-[#b31b34]"
                                style="width: {stats.percentage}%"
                            ></div>
                            <div
                                class="absolute top-0 bottom-0 left-[75%] w-[2px] bg-black opacity-30 border-l border-dashed border-white"
                            ></div>
                        </div>
                        <div
                            class="flex justify-between text-xs mt-1 uppercase font-bold opacity-60"
                        >
                            <span
                                >{stats.present_count} / {stats.total_classes} Classes</span
                            >
                            <span>Target: 75%</span>
                        </div>
                    {/if}
                </button>

                {#if expandedCourseId === course.id && stats}
                    <div class="logs-container">
                        <table class="w-full text-sm uppercase">
                            <thead>
                                <tr
                                    class="border-b-2 border-dashed border-[#2b0b0b]"
                                >
                                    <th class="text-left py-2">Date</th>
                                    <th class="text-right py-2">Status</th>
                                </tr>
                            </thead>
                            <tbody>
                                {#each stats.logs as log}
                                    <tr class="border-b border-gray-200">
                                        <td class="py-2">{log.date}</td>
                                        <td
                                            class={`text-right py-2 ${getStatusColor(log.status)}`}
                                        >
                                            {log.status}
                                        </td>
                                    </tr>
                                {/each}
                                {#if stats.logs.length === 0}
                                    <tr
                                        ><td
                                            colspan="2"
                                            class="text-center py-4 opacity-50"
                                            >No logs yet</td
                                        ></tr
                                    >
                                {/if}
                            </tbody>
                        </table>
                    </div>
                {/if}
            {/each}
        </div>
    {/if}
</div>

<style>
    .container {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1.5rem;
        padding: 2rem 1rem;
        min-height: 100vh;
        max-width: 400px;
        margin: 0 auto;
        color: #2b0b0b;
    }

    .page-title {
        font-size: 3rem;
        font-weight: 700;
        color: #2b0b0b;
        letter-spacing: -0.05em;
        text-transform: uppercase;
        width: 100%;
        text-align: center;
    }

    .loading {
        color: #c0c3d7;
        font-size: 1.2rem;
        margin-top: 3rem;
        font-family: inherit;
        text-transform: uppercase;
    }

    /* Stats Card */
    .stats-card {
        background: white;
        border: 2px solid rgba(198, 225, 237, 0.6);
        padding: 1rem;
        box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.8);
        transition: 0.2s ease;
        cursor: pointer;
    }

    .stats-card:hover {
        border-color: #b31b34;
        transform: translate(-2px, -2px);
        box-shadow: 6px 6px 0px #b31b34;
    }

    .stats-card.expanded {
        border-color: #2b0b0b;
        background-color: #f0f0f0;
        box-shadow: 2px 2px 0px #2b0b0b;
        transform: translate(2px, 2px);
    }

    /* Logs Dropdown */
    .logs-container {
        background: white;
        border: 2px solid #2b0b0b;
        border-top: none;
        margin-top: -1rem; /* Connect to card */
        margin-bottom: 1rem;
        padding: 1rem;
        width: 95%; /* Slightly narrower than card */
        align-self: center;
        box-shadow: inset 0px 4px 4px rgba(0, 0, 0, 0.05);
    }
</style>
