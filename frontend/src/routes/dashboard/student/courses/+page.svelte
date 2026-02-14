<script lang="ts">
    import { api } from "$lib/api";
    import type { Course, ApiResponse } from "$lib/types";
    import CourseCard from "$lib/components/CourseCard.svelte";
    import AcademicStatsCard from "$lib/components/AcademicStatsCard.svelte";
    import AcademicSubNav from "$lib/components/AcademicSubNav.svelte";

    let courses = $state<Course[]>([]);
    let enrolledCourses = $state<Course[]>([]);
    let myEnrollmentIds = $state<Set<string>>(new Set());
    let loading = $state(true);

    // Reactive calculations for the Credit Calculator
    let totalCredits = $derived(
        enrolledCourses.reduce((acc, c) => acc + c.credits, 0),
    );

    let coreCredits = $derived(
        enrolledCourses
            .filter((c) => c.course_type === "core")
            .reduce((acc, c) => acc + c.credits, 0),
    );

    async function loadAcademicData() {
        try {
            const allRes = await api.get<ApiResponse<Course[]>>("/api/courses");
            const myRes = await api.get<ApiResponse<Course[]>>(
                "/api/courses/my-enrollments",
            );

            if (allRes.success) courses = allRes.data;
            if (myRes.success) {
                enrolledCourses = myRes.data;
                myEnrollmentIds = new Set(myRes.data.map((c) => c.id));
            }
        } catch (e) {
            console.error("CITADEL_ACCESS_DENIED:", e);
        } finally {
            loading = false;
        }
    }

    async function handleEnroll(courseId: string) {
        try {
            const res = await api.post<ApiResponse<string>>(
                "/api/courses/enroll",
                {
                    course_id: courseId,
                },
            );
            if (res.success) {
                // Refresh data to update the credit calculator
                await loadAcademicData();
            }
        } catch (e) {
            console.error("ENROLLMENT_ERROR:", e);
        }
    }

    $effect(() => {
        loadAcademicData();
    });
</script>

<div class="destiny-container">
    <div class="flex flex-col items-center gap-4 mb-8 w-full">
        <h1
            class="text-8xl font-bold text-[#2b0b0b] tracking-tighter uppercase text-center w-full"
        >
            Destiny
        </h1>
        <p
            class="text-[10px] font-bold text-[#666] uppercase tracking-widest opacity-70"
        >
            Course & Credit Ledger
        </p>
    </div>

    {#if loading}
        <div class="ink-loading">FETCHING_FROM_SCROLLS...</div>
    {:else}
        <section class="grid-section">
            <h2 class="section-label">Available_Courses</h2>
            <div class="course-grid">
                {#each courses as course}
                    <CourseCard
                        {course}
                        enrolled={myEnrollmentIds.has(course.id)}
                        onenroll={() => handleEnroll(course.id)}
                    />
                {/each}
            </div>
        </section>
    {/if}
</div>

<style>
    .destiny-container {
        padding: 2rem;
        min-height: 100vh;
    }

    .section-label {
        font-weight: 900;
        text-transform: uppercase;
        font-size: 1.2rem;
        background: #000;
        color: #fff;
        display: inline-block;
        padding: 4px 12px;
        margin-bottom: 2rem;
    }

    .course-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
        gap: 2.5rem;
    }

    .ink-loading {
        font-family: monospace;
        font-weight: 900;
        font-size: 1.5rem;
        text-align: center;
        margin-top: 4rem;
    }
</style>
