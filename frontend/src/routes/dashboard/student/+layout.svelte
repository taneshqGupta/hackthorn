<script lang="ts">
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';
    let { children } = $props();

    // Drawer state
    let isDesktopOpen = $state(true);
    let isMobileOpen = $state(false);

    const navItems = [
        { label: 'CONCOURSE', path: '/dashboard/student', tip: 'Home', icon: 'M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8M3 10a2 2 0 0 1 .709-1.528l7-5.999a2 2 0 0 1 2.582 0l7 5.999A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z' },
        { label: 'IDENTITY', path: '/dashboard/student/profile', tip: 'Profile', icon: 'M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0M12 10m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0' },
        { label: 'GRIEVANCES', path: '/dashboard/student/grievances', tip: 'Issues', icon: 'M3 21h18M3 7v1a3 3 0 0 0 6 0V7m0 1a3 3 0 0 0 6 0V7m0 1a3 3 0 0 0 6 0V7M4 21V4a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v17' },
        { label: 'COURSES', path: '/dashboard/student/courses', tip: 'Courses', icon: 'M3 19a9 9 0 0 1 9 0a9 9 0 0 1 9 0M3 6a9 9 0 0 1 9 0a9 9 0 0 1 9 0M3 6v13M12 6v13M21 6v13' },
        { label: 'ATTENDANCE', path: '/dashboard/student/attendance', tip: 'Logs', icon: 'M9 5H7a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-2M9 5a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2' },
        { label: 'THE_VAULT', path: '/dashboard/student/vault', tip: 'Vault', icon: 'M12 3L4 9v11a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1V9l-8-6z' }
    ];
</script>

<div class="flex h-screen w-full overflow-hidden bg-transparent">
    <aside 
        class="hidden lg:flex flex-col border-r-2 border-[#d06065] transition-all duration-300 {isDesktopOpen ? 'w-64' : 'w-16'}"
        style="background: transparent;"
    >
        <div class="p-4 border-b-2 border-[#d06065]">
            <!-- svelte-ignore a11y_consider_explicit_label -->
            <button onclick={() => isDesktopOpen = !isDesktopOpen} class="hover:text-[#d06065]">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" stroke-width="2" fill="none" stroke="currentColor" class="size-5">
                    <path d="M4 4m0 2a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2z"></path>
                    <path d="M9 4v16"></path>
                    <path d="M14 10l2 2l-2 2"></path>
                </svg>
            </button>
        </div>

        <nav class="flex-1 p-2 flex flex-col gap-2">
            {#each navItems as item}
                <button 
                    onclick={() => goto(item.path)}
                    class="group relative flex items-center p-2 border-2 border-transparent hover:border-[#2b0b0b] transition-all {$page.url.pathname === item.path ? 'bg-[#2b0b0b] text-white' : 'text-[#2b0b0b]'}"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" stroke-width="2" fill="none" stroke="currentColor" class="size-5 min-w-5">
                        <path d={item.icon}></path>
                    </svg>
                    {#if isDesktopOpen}
                        <span class="ml-4 font-black uppercase text-[10px] tracking-widest">{item.label}</span>
                    {:else}
                        <span class="absolute left-14 bg-[#2b0b0b] text-white text-[10px] px-2 py-1 opacity-0 group-hover:opacity-100 pointer-events-none transition-opacity whitespace-nowrap z-50">
                            {item.tip}
                        </span>
                    {/if}
                </button>
            {/each}
        </nav>
    </aside>

    <div class="flex flex-col flex-1 min-w-0">
        <header class="lg:hidden h-10 border-b-2 border-[#d06065] flex items-center px-4 justify-between">
            <!-- svelte-ignore a11y_consider_explicit_label -->
            <button onclick={() => isMobileOpen = !isMobileOpen} class="text-[#2b0b0b]">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="size-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                </svg>
            </button>
            <span class="font-black text-[10px] uppercase">Student_Concourse</span>
        </header>

        <main class="flex-1 overflow-y-auto">
            {@render children()}
        </main>
    </div>

    {#if isMobileOpen}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="fixed inset-0 z-50 lg:hidden" onclick={() => isMobileOpen = false}>
            <div class="absolute inset-y-0 left-0 w-64 bg-white border-r-4 border-[#2b0b0b] p-4 flex flex-col gap-4">
                <span class="font-black border-b-2 border-[#2b0b0b] pb-2 text-xs">MENU</span>
                {#each navItems as item}
                    <button 
                        onclick={() => { goto(item.path); isMobileOpen = false; }}
                        class="text-left font-black uppercase text-xs p-2 {$page.url.pathname === item.path ? 'bg-[#2b0b0b] text-white' : ''}"
                    >
                        {item.label}
                    </button>
                {/each}
            </div>
        </div>
    {/if}
</div>