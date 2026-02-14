<script lang="ts">
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';
    let { children } = $props();

    let isCollapsed = $state(false);

    // Flat navigation structure - No extra nesting
    const navItems = [
        { label: 'CONCOURSE', path: '/dashboard/student' },
        { label: 'IDENTITY', path: '/dashboard/student/profile' },
        { label: 'GRIEVANCES', path: '/dashboard/student/grievances' },
        { label: 'COURSES', path: '/dashboard/student/courses' },
        { label: 'ATTENDANCE', path: '/dashboard/student/attendance' },
        { label: 'THE_VAULT', path: '/dashboard/student/vault' }
    ];
</script>

<div class="flex min-h-screen bg-transparent">
    <aside class="sidebar {isCollapsed ? 'w-16' : 'w-64'} transition-all duration-300 border-r-2 border-[#2b0b0b]">
        <button 
            class="w-full p-6 text-left font-black border-b-2 border-[#2b0b0b] uppercase text-[10px] tracking-widest" 
            onclick={() => isCollapsed = !isCollapsed}
        >
            {isCollapsed ? '>>' : '<< COLLAPSE_MENU'}
        </button>
        
        <nav class="flex flex-col gap-1 p-2">
            {#each navItems as item}
                <a 
                    href={item.path} 
                    class="nav-link {isCollapsed ? 'justify-center' : ''} {$page.url.pathname === item.path ? 'active' : ''}"
                >
                    {#if !isCollapsed}
                        <span>{item.label}</span>
                    {:else}
                        <span>{item.label.charAt(0)}</span>
                    {/if}
                </a>
            {/each}
        </nav>
    </aside>

    <main class="flex-1 flex justify-center p-8 overflow-y-auto">
        <div class="w-[400px] flex flex-col items-center">
            {@render children()}
        </div>
    </main>
</div>

<style>
    .sidebar {
        height: 100vh;
        position: sticky;
        top: 0;
        z-index: 50;
        background: transparent;
    }

    .nav-link {
        display: flex;
        align-items: center;
        padding: 14px;
        font-weight: 900;
        font-size: 12px;
        text-transform: uppercase;
        color: #2b0b0b;
        text-decoration: none;
        border: 2px solid transparent;
        transition: all 0.2s ease;
    }

    .nav-link:hover {
        background: rgba(179, 27, 52, 0.05);
        border: 2px solid #2b0b0b;
        box-shadow: 4px 4px 0px #2b0b0b;
        transform: translate(-2px, -2px);
    }

    .nav-link.active {
        background: #2b0b0b;
        color: white;
        box-shadow: none;
        transform: none;
    }

    /* Notebook-friendly scrollbar for the sidebar */
    .sidebar::-webkit-scrollbar { width: 4px; }
    .sidebar::-webkit-scrollbar-thumb { background: #2b0b0b; }
</style>