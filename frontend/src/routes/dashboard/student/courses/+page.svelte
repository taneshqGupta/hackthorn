<script lang="ts">
    import { onMount } from "svelte";
    import api from "$lib/api";
    import type { CourseResponse, ApiResponse } from "$lib/types";
    import { goto } from "$app/navigation";

    let courses = $state<CourseResponse[]>([]);
    let loading = $state(true);
    let searchQuery = $state("");

    // Derived filtered list
    let filteredCourses = $derived(
        courses.filter((c) => {
            const query = searchQuery.toLowerCase();
            return (
                c.title.toLowerCase().includes(query) ||
                c.code.toLowerCase().includes(query) ||
                c.department.toLowerCase().includes(query)
            );
        })
    );

    onMount(async () => {
        await loadCourses();
    });

    async function loadCourses() {
        loading = true;
        try {
            const response = await api.get<ApiResponse<CourseResponse[]>>("/api/courses/my-enrollments");
            courses = response.data || [];
        } catch (err) {
            console.error("[COURSES] Error loading:", err);
        } finally {
            loading = false;
        }
    }

    function getBadgeColor(type: string) {
        switch (type) {
            case 'core': return 'bg-[#b31b34] text-white';
            case 'elective': return 'bg-[#c6e1ed] text-[#2b0b0b]';
            default: return 'bg-[#2b0b0b] text-white';
        }
    }
</script>

<div class="container w-100 flex flex-col items-center gap-4 mb-8 px-2">
    <h1 class="page-title">ACADEMICS</h1>

    <div class="group-search">
        <svg class="icon-search" aria-hidden="true" viewBox="0 0 24 24">
            <g><path d="M21.53 20.47l-3.66-3.66C19.195 15.24 20 13.214 20 11c0-4.97-4.03-9-9-9s-9 4.03-9 9 4.03 9 9 9c2.215 0 4.24-.804 5.808-2.13l3.66 3.66c.147.146.34.22.53.22s.385-.073.53-.22c.295-.293.295-.767.002-1.06zM3.5 11c0-4.135 3.365-7.5 7.5-7.5s7.5 3.365 7.5 7.5-3.365 7.5-7.5 7.5-7.5-3.365-7.5-7.5z"></path></g>
        </svg>
        <input
            type="text"
            bind:value={searchQuery}
            placeholder="SEARCH COURSES..."
            class="input-search"
        />
    </div>

    {#if loading}
        <div class="loading">LOADING ACADEMICS...</div>
    {:else}
        <div class="w-full flex flex-col gap-4">
            {#each filteredCourses as course}
                <div class="course-card">
                    <div class="flex justify-between items-start mb-2">
                        <span class="font-bold text-2xl tracking-tighter">{course.code}</span>
                        <span class={`px-2 py-0.5 text-xs font-bold uppercase border border-black ${getBadgeColor(course.course_type)}`}>
                            {course.course_type}
                        </span>
                    </div>
                    
                    <h3 class="text-xl leading-none mb-4 font-bold uppercase text-[#b31b34]">
                        {course.title}
                    </h3>

                    <div class="grid grid-cols-2 gap-2 text-sm uppercase font-medium opacity-80 mb-4">
                        <div>
                            <span class="block text-xs opacity-50">CREDITS</span>
                            {course.credits}
                        </div>
                        <div class="text-right">
                            <span class="block text-xs opacity-50">SEM</span>
                            {course.semester}
                        </div>
                    </div>

                    {#if course.instructor}
                        <div class="border-t-2 border-dashed border-[#2b0b0b] opacity-30 my-2"></div>
                        <div class="text-xs uppercase mt-2">
                            <span class="opacity-50">PROF. </span>
                            {course.instructor.first_name} {course.instructor.last_name}
                        </div>
                    {/if}

                    <div class="flex gap-2 mt-4">
                        <button 
                            class="action-btn"
                            onclick={() => goto(`/dashboard/student/courses/attendance?id=${course.id}`)}
                        >
                            Attendance
                        </button>
                     </div>
                </div>
            {/each}

            {#if filteredCourses.length === 0}
                <div class="text-[#6e0f1c] uppercase text-sm mt-8 opacity-50 text-center">
                    No enrollments found
                </div>
            {/if}
        </div>
    {/if}

    <button
        class="submit-btn"
        onclick={() => goto("/dashboard/student/courses/enroll")}
        aria-label="Enroll in course"
    >
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
        </svg>
    </button>
</div>

<style>
    /* Inheriting layout from Grievances */
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
        font-size: 3.5rem; /* 7xl equivalent roughly */
        font-weight: 700;
        color: #2b0b0b;
        letter-spacing: -0.05em;
        text-transform: uppercase;
        text-align: center;
        width: 100%;
        line-height: 0.9;
    }

    .loading {
        color: #c0c3d7;
        font-size: 1.2rem;
        margin-top: 3rem;
        font-family: inherit;
        text-transform: uppercase;
        animation: pulse 2s infinite;
    }
    @keyframes pulse { 0% { opacity: 0.5; } 50% { opacity: 1; } 100% { opacity: 0.5; } }

    /* Search Input Style */
    .group-search {
        display: flex;
        align-items: center;
        position: relative;
        width: 100%;
    }

    .input-search {
        width: 100%;
        height: 50px;
        padding: 0 1rem 0 2.5rem;
        border: 2px solid rgba(198, 225, 237, 0.6);
        background-color: transparent;
        color: #2b0b0b;
        font-family: inherit;
        font-size: 18px;
        text-transform: uppercase;
        transition: 0.2s ease;
    }
    .input-search:focus {
        outline: none;
        border-color: #b31b34;
        background-color: rgba(255, 255, 255, 0.5);
        box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.1);
    }
    .icon-search {
        position: absolute;
        left: 0.8rem;
        fill: rgba(110, 15, 28, 0.5);
        width: 1.2rem;
        height: 1.2rem;
        pointer-events: none;
    }

    /* Neobrutalist Course Card */
    .course-card {
        background: white;
        border: 2px solid rgba(198, 225, 237, 0.6);
        padding: 1.25rem;
        box-shadow: 4px 4px 0px rgba(0,0,0,0.8);
        transition: transform 0.1s ease, box-shadow 0.1s ease;
        display: flex;
        flex-direction: column;
    }
    
    .course-card:hover {
        transform: translate(-2px, -2px);
        box-shadow: 6px 6px 0px rgba(0,0,0,1);
        border-color: #b31b34;
    }

    /* Action Buttons inside Card */
    .action-btn {
        flex: 1;
        padding: 0.5rem;
        border: 2px solid #2b0b0b;
        background: #2b0b0b;
        color: white;
        text-transform: uppercase;
        font-weight: bold;
        font-size: 0.8rem;
        cursor: pointer;
        transition: 0.2s;
    }
    .action-btn:hover {
        background: #b31b34;
        border-color: #b31b34;
    }
    .action-btn.secondary {
        background: transparent;
        color: #2b0b0b;
    }
    .action-btn.secondary:hover {
        background: rgba(198, 225, 237, 0.3);
        border-color: #b31b34;
        color: #b31b34;
    }

    /* FAB */
    .submit-btn {
        position: fixed;
        bottom: 2rem;
        right: 2rem;
        width: 60px;
        height: 60px;
        background-color: rgba(179, 27, 52, 0.9);
        border: 2px solid rgba(198, 225, 237, 0.6);
        box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.5);
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: 200ms ease-in-out;
        z-index: 100;
    }
    .submit-btn:hover {
        transform: translate(-2px, -2px);
        box-shadow: 6px 6px 0px rgba(0, 0, 0, 0.7);
        background-color: rgba(179, 27, 52, 1);
    }
    .submit-btn svg {
        width: 28px;
        height: 28px;
        stroke: #fff;
    }
</style>