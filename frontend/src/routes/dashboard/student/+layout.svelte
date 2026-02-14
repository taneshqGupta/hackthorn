<script lang="ts">
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';
    let { children } = $props();

    const navItems = [
        { label: 'Homepage', path: '/dashboard/student', tip: 'Homepage', icon: 'M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8M3 10a2 2 0 0 1 .709-1.528l7-5.999a2 2 0 0 1 2.582 0l7 5.999A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z' },
        { label: 'Identity', path: '/dashboard/student/profile', tip: 'Identity', icon: 'M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0M12 10m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0M6.168 18.849a4 4 0 0 1 3.832 -2.849h4a4 4 0 0 1 3.834 2.855' },
        { label: 'Grievances', path: '/dashboard/student/grievances', tip: 'Grievances', icon: 'M3 21h18M3 7v1a3 3 0 0 0 6 0V7m0 1a3 3 0 0 0 6 0V7m0 1a3 3 0 0 0 6 0V7M4 21V4a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v17' },
        { label: 'Courses', path: '/dashboard/student/courses', tip: 'Courses', icon: 'M3 19a9 9 0 0 1 9 0a9 9 0 0 1 9 0M3 6a9 9 0 0 1 9 0a9 9 0 0 1 9 0M3 6v13M12 6v13M21 6v13' },
        { label: 'Attendance', path: '/dashboard/student/attendance', tip: 'Attendance', icon: 'M9 5H7a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-2M9 5a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2M9 5a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2' },
        { label: 'Vault', path: '/dashboard/student/vault', tip: 'Vault', icon: 'M12 3L4 9v11a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1V9l-8-6z' }
    ];
</script>

<div class="drawer lg:drawer-open">
  <input id="my-drawer-4" type="checkbox" class="drawer-toggle" />
  <div class="drawer-content flex flex-col min-h-screen">
    <nav class="navbar w-full bg-base-300">
      <label for="my-drawer-4" aria-label="open sidebar" class="btn btn-square btn-ghost">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor" class="my-1.5 inline-block size-4">
            <path d="M4 4m0 2a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2z"></path>
            <path d="M9 4v16"></path>
            <path d="M14 10l2 2l-2 2"></path>
        </svg>
      </label>
      <div class="px-4 font-bold uppercase tracking-widest text-xs">Student Concourse</div>
    </nav>
    
    <main class="flex-1">
        {@render children()}
    </main>
  </div>

  <div class="drawer-side is-drawer-close:overflow-visible">
    <label for="my-drawer-4" aria-label="close sidebar" class="drawer-overlay"></label>
    <div class="flex min-h-full flex-col items-start bg-base-200 is-drawer-close:w-14 is-drawer-open:w-64">
      <ul class="menu w-full grow">
        {#each navItems as item}
            <li>
                <button 
                    onclick={() => goto(item.path)}
                    class="is-drawer-close:tooltip is-drawer-close:tooltip-right {$page.url.pathname === item.path ? 'active' : ''}" 
                    data-tip={item.tip}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor" class="my-1.5 inline-block size-4">
                        <path d={item.icon}></path>
                    </svg>
                    <span class="is-drawer-close:hidden">{item.label}</span>
                </button>
            </li>
        {/each}
      </ul>
    </div>
  </div>
</div>