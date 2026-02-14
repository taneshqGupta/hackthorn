<script lang="ts">
    import type { Course } from '$lib/types';
    let { course, enrolled = false, onenroll }: { course: Course, enrolled?: boolean, onenroll?: () => void } = $props();
</script>

<div class="course-card">
    <div class="card-header">
        <span class="code-tag">{course.code}</span>
        <span class="credits-tag">{course.credits} CR</span>
    </div>
    
    <div class="card-body">
        <h3 class="course-title">{course.title}</h3>
        <p class="meta-text">INST: {course.instructor?.last_name || 'TBA'}</p>
        <p class="meta-text">DEPT: {course.department} | {course.course_type}</p>
    </div>

    <div class="card-footer">
        {#if !enrolled}
            <button class="enroll-btn" onclick={onenroll}>
                ENROLL_IN_COURSE
            </button>
        {:else}
            <div class="enrolled-badge">STAMP: ENROLLED</div>
        {/if}
    </div>
</div>

<style>
    .course-card {
        background-color: transparent; /* Transparent to show the notebook background */
        border: 4px solid #000;
        padding: 1.2rem;
        box-shadow: 8px 8px 0px rgba(0, 0, 0, 1);
        display: flex;
        flex-direction: column;
        gap: 1rem;
        transition: transform 0.1s;
    }

    .course-card:hover {
        transform: translate(-2px, -2px);
        box-shadow: 10px 10px 0px rgba(0, 0, 0, 1);
    }

    .card-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .code-tag {
        background: #000;
        color: #fff;
        padding: 2px 8px;
        font-family: monospace;
        font-weight: 900;
        font-size: 0.9rem;
    }

    .credits-tag {
        font-weight: 900;
        text-transform: uppercase;
        font-size: 0.8rem;
        border: 2px solid #000;
        padding: 2px 6px;
    }

    .course-title {
        font-weight: 950;
        font-size: 1.4rem;
        text-transform: uppercase;
        line-height: 1.1;
        margin: 0;
    }

    .meta-text {
        font-weight: 700;
        font-size: 0.85rem;
        margin: 0.2rem 0;
        text-transform: uppercase;
        opacity: 0.8;
    }

    .enroll-btn {
        width: 100%;
        background-color: #ff4d4d; /* Orangish-red */
        border: 3px solid #000;
        padding: 0.6rem;
        font-weight: 900;
        text-transform: uppercase;
        cursor: pointer;
        box-shadow: 4px 4px 0px #000;
    }

    .enroll-btn:hover {
        background-color: #ff8fa3;
    }

    .enroll-btn:active {
        transform: translate(2px, 2px);
        box-shadow: 2px 2px 0px #000;
    }

    .enrolled-badge {
        width: 100%;
        text-align: center;
        border: 3px dashed #000;
        padding: 0.6rem;
        font-weight: 900;
        text-transform: uppercase;
        color: #000;
    }
</style>