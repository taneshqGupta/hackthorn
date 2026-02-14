<script lang="ts">
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';
    let { children } = $props();

    let isExpanded = $state(true);

    const navItems = [
        { label: 'CONCOURSE', path: '/dashboard/student', icon: 'H' },
        { label: 'IDENTITY', path: '/dashboard/student/profile', icon: 'I' },
        { label: 'GRIEVANCES', path: '/dashboard/student/grievances', icon: 'G' },
        { label: 'COURSES', path: '/dashboard/student/courses', icon: 'C' },
        { label: 'ATTENDANCE', path: '/dashboard/student/attendance', icon: 'A' },
        { label: 'THE_VAULT', path: '/dashboard/student/vault', icon: 'V' }
    ];
</script>

<div class="flex h-screen w-full bg-transparent">
    <aside 
        class="hidden lg:flex flex-col border-r-2 border-[#d06065] transition-all duration-300 {isExpanded ? 'w-64' : 'w-16'}"
        style="background: transparent;"
    >
        <button 
            onclick={() => isExpanded = !isExpanded}
            class="p-4 border-b-2 border-[#d06065] text-[#2b0b0b] font-black hover:bg-[#d06065]/10 text-left"
        >
            {isExpanded ? '<< COLLAPSE' : '>>'}
        </button>

        <nav class="flex-1 flex flex-col gap-1 p-2">
            {#each navItems as item}
                <button 
                    onclick={() => goto(item.path)}
                    class="flex items-center p-3 font-black text-xs transition-colors {$page.url.pathname === item.path ? 'bg-[#2b0b0b] text-white' : 'text-[#2b0b0b] hover:bg-[#d06065]/10'}"
                >
                    <span class="min-w-[20px] text-center">{item.icon}</span>
                    {#if isExpanded}
                        <span class="ml-4 tracking-widest">{item.label}</span>
                    {/if}
                </button>
            {/each}
        </nav>
    </aside>

    <main class="flex-1 overflow-y-auto">
        <header class="lg:hidden flex items-center h-10 border-b-2 border-[#d06065] px-4">
             <span class="font-black text-[10px] tracking-tighter">STUDENT_MENU</span>
        </header>
        
        {@render children()}
    </main>
</div>