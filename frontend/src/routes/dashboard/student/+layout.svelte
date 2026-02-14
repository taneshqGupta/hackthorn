<script lang="ts">
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';

    let { children } = $props(); // Svelte 5 pattern
</script>

<div class="notebook-outer">
    <div class="notebook-locked-container">
        <nav class="brutalist-nav">
            <button 
                onclick={() => goto('/dashboard/student')}
                class:active={$page.url.pathname === '/dashboard/student'}
            >
                HUB
            </button>
            <button 
                onclick={() => goto('/dashboard/student/academic/courses')}
                class:active={$page.url.pathname.includes('/academic')}
            >
                FATE
            </button>
            <button 
                onclick={() => goto('/profile')}
                class:active={$page.url.pathname === '/profile'}
            >
                USER
            </button>
        </nav>

        {#if $page.url.pathname.includes('/academic')}
            <div class="academic-sub-tabs">
                <a href="/dashboard/student/academic/courses" class:on={$page.url.pathname.endsWith('/courses')}>CR</a>
                <a href="/dashboard/student/academic/courses/attendance" class:on={$page.url.pathname.endsWith('/attendance')}>ATT</a>
                <a href="/dashboard/student/academic/vault" class:on={$page.url.pathname.endsWith('/vault')}>VLT</a>
                <a href="/dashboard/student/academic/calendar" class:on={$page.url.pathname.endsWith('/calendar')}>CAL</a>
            </div>
        {/if}

        <main class="content-view">
            {@render children()}
        </main>
    </div>
</div>

<style>
    .notebook-outer {
        min-height: 100vh;
        width: 100%;
        display: flex;
        justify-content: center;
        background-attachment: fixed;
    }

    .notebook-locked-container {
        width: 400px; /* Locked Width as requested */
        padding: 2rem 1rem;
        display: flex;
        flex-direction: column;
    }

    .brutalist-nav {
        display: flex;
        border: 2px solid #2b0b0b;
        margin-bottom: 1rem;
        background: transparent;
    }

    .brutalist-nav button {
        flex: 1;
        padding: 8px;
        font-family: monospace;
        font-weight: 900;
        text-transform: uppercase;
        border-right: 2px solid #2b0b0b;
        transition: 0.2s;
    }

    .brutalist-nav button:last-child { border-right: none; }
    
    .brutalist-nav button.active {
        background: #2b0b0b;
        color: white;
    }

    .academic-sub-tabs {
        display: flex;
        justify-content: center;
        gap: 8px;
        margin-bottom: 2rem;
    }

    .academic-sub-tabs a {
        font-size: 10px;
        font-weight: 900;
        border: 1px solid #2b0b0b;
        padding: 2px 6px;
        text-decoration: none;
        color: #2b0b0b;
    }

    .academic-sub-tabs a.on {
        background: #ffbc42; /* Yellow highlight for active sub-tab */
        box-shadow: 2px 2px 0px #000;
    }

    .content-view {
        width: 100%;
    }
</style>