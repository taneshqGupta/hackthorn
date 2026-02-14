<script lang="ts">
    import { page } from '$app/stores';
</script>

<nav class="academic-sub-nav">
    {#each [
        { name: 'COURSES', path: '/dashboard/student/academic/courses' },
        { name: 'ATTENDANCE', path: '/dashboard/student/academic/courses/attendance' },
        { name: 'VAULT', path: '/dashboard/student/academic/vault' },
        { name: 'CALENDAR', path: '/dashboard/student/academic/calendar' }
    ] as link}
        <a 
            href={link.path} 
            class:active={$page.url.pathname === link.path}
        >
            {link.name}
        </a>
    {/each}
</nav>

<style>
    .academic-sub-nav {
        display: flex;
        flex-wrap: wrap;
        gap: 12px;
        margin-bottom: 3rem;
        padding-bottom: 1.5rem;
        border-bottom: 3px solid #000;
    }

    .academic-sub-nav a {
        font-family: monospace;
        font-weight: 950;
        font-size: 0.85rem;
        padding: 6px 12px;
        border: 3px solid #000;
        text-decoration: none;
        color: #000;
        transition: 100ms;
        box-shadow: 4px 4px 0px #000;
        background: transparent; /* Notebook transparency */
    }

    .academic-sub-nav a:hover {
        transform: translate(-2px, -2px);
        box-shadow: 6px 6px 0px #000;
        background: #ffbc42; /* Yellow highlight */
    }

    .academic-sub-nav a.active {
        background: #000;
        color: #fff;
        transform: translate(2px, 2px);
        box-shadow: none;
    }
</style>