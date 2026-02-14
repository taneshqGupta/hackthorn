<script lang="ts">
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    let { children } = $props();

    const navItems = [
        { label: "CONCOURSE", path: "/dashboard/student" },
        { label: "IDENTITY", path: "/dashboard/student/profile" },
        { label: "GRIEVANCES", path: "/dashboard/student/grievances" },
        { label: "COURSES", path: "/dashboard/student/courses" },
        { label: "ATTENDANCE", path: "/dashboard/student/attendance" },
        { label: "THE_VAULT", path: "/dashboard/student/vault" },
    ];

    let isMobile = $state(false);

    onMount(() => {
        const checkMobile = () => (isMobile = window.innerWidth < 1024);
        checkMobile();
        window.addEventListener("resize", checkMobile);
        return () => window.removeEventListener("resize", checkMobile);
    });
</script>

<div class="flex w-full h-full">
    {#if !isMobile}
        <aside
            class="w-64 flex-none border-r-2 border-[#d06065] flex flex-col bg-transparent sticky top-0 h-screen"
        >
            <div
                class="p-4 border-b-2 border-[#d06065] bg-[#2b0b0b] text-white font-black text-xs tracking-widest"
            >
                STUDENT_CORE
            </div>
            <nav class="flex-1 overflow-y-auto p-2 flex flex-col gap-1">
                {#each navItems as item}
                    <a
                        href={item.path}
                        class="nav-link {$page.url.pathname === item.path
                            ? 'active'
                            : ''}"
                    >
                        {item.label}
                    </a>
                {/each}
            </nav>
        </aside>
    {/if}

    <main class="flex-1">
        {@render children()}
    </main>
</div>

<style>
    .nav-link {
        padding: 12px;
        font-weight: 900;
        font-size: 11px;
        text-transform: uppercase;
        color: #2b0b0b;
        text-decoration: none;
        border: 2px solid transparent;
        transition: 0.2s;
    }

    .nav-link:hover {
        border-color: #d06065;
        background: rgba(208, 96, 101, 0.05);
    }

    .nav-link.active {
        background: #2b0b0b;
        color: white;
        border-color: #2b0b0b;
    }
</style>
