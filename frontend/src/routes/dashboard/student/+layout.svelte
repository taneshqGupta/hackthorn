<script lang="ts">
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';
    let { children } = $props();

    const navItems = [
        { label: 'Concourse', path: '/dashboard/student', tip: 'Home', icon: 'M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8M3 10a2 2 0 0 1 .709-1.528l7-5.999a2 2 0 0 1 2.582 0l7 5.999A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z' },
        { label: 'Identity', path: '/dashboard/student/profile', tip: 'Profile', icon: 'M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0M12 10m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0M6.168 18.849a4 4 0 0 1 3.832 -2.849h4a4 4 0 0 1 3.834 2.855' },
        { label: 'Grievances', path: '/dashboard/student/grievances', tip: 'Issues', icon: 'M12 9v4M12 17h.01M3.3 7H21M7 7V5a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v2M9 21h6a2 2 0 0 0 2-2v-5H7v5a2 2 0 0 0 2 2Z' },
        { label: 'Courses', path: '/dashboard/student/courses', tip: 'Enrollment', icon: 'M3 19a9 9 0 0 1 9 0a9 9 0 0 1 9 0M3 6a9 9 0 0 1 9 0a9 9 0 0 1 9 0M3 6v13M12 6v13M21 6v13' },
        { label: 'Attendance', path: '/dashboard/student/attendance', tip: 'Logs', icon: 'M4 7h16M4 12h16M4 17h16' },
        { label: 'The Vault', path: '/dashboard/student/vault', tip: 'PYQs', icon: 'M12 3L4 9v11a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1V9l-8-6zM12 8v4M12 16h.01' }
    ];
</script>

<div class="drawer lg:drawer-open min-h-screen">
    <input id="student-drawer" type="checkbox" class="drawer-toggle" />
    
    <div class="drawer-content flex flex-col bg-transparent">
        <nav class="navbar w-full bg-transparent border-b-2 border-[#d06065] lg:hidden">
            <div class="flex-none">
                <label for="student-drawer" aria-label="open sidebar" class="btn btn-square btn-ghost">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-6 h-6 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
                </label>
            </div>
            <div class="flex-1 px-2 mx-2 font-black uppercase text-xs tracking-tighter">Student Concourse</div>
        </nav>

        <main class="flex-1">
            {@render children()}
        </main>
    </div>

    <div class="drawer-side z-40 is-drawer-close:overflow-visible">
        <label for="student-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
        
        <div class="flex min-h-full flex-col items-start bg-transparent border-r-2 border-[#d06065] backdrop-blur-sm is-drawer-close:w-14 is-drawer-open:w-64 transition-all duration-300">
            <div class="hidden lg:flex w-full p-4 justify-start">
                <label for="student-drawer" class="cursor-pointer hover:opacity-70">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor" class="size-5"><path d="M4 4m0 2a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2z"></path><path d="M9 4v16"></path><path d="M14 10l2 2l-2 2"></path></svg>
                </label>
            </div>

            <ul class="menu w-full grow p-2 gap-2">
                {#each navItems as item}
                    <li>
                        <button 
                            onclick={() => goto(item.path)}
                            class="flex items-center gap-4 py-3 {$page.url.pathname === item.path ? 'bg-[#2b0b0b] text-white' : ''} is-drawer-close:tooltip is-drawer-close:tooltip-right" 
                            data-tip={item.tip}
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor" class="size-5 min-w-[20px]">
                                <path d={item.icon}></path>
                            </svg>
                            <span class="is-drawer-close:hidden font-black uppercase text-xs tracking-widest">{item.label}</span>
                        </button>
                    </li>
                {/each}
            </ul>
        </div>
    </div>
</div>

<style>
    /* Ensure the notebook lines are visible through the drawer */
    :global(.drawer-side > div) {
        background-color: rgba(255, 255, 255, 0.1) !important;
    }

    .menu li > button {
        border-radius: 0;
        border: 2px solid transparent;
        transition: all 0.2s;
    }

    .menu li > button:hover {
        border-color: #d06065;
        background: rgba(208, 96, 101, 0.1);
    }
</style>