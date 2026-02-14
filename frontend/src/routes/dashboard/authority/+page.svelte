<script lang="ts">
    import { user } from '$lib/auth';
    import { goto } from '$app/navigation';

    let currentUser = $derived($user);

    $effect(() => {
        if (currentUser && currentUser.role !== 'authority') {
            goto(`/dashboard/${currentUser.role}`);
        } else if (!currentUser) {
            goto('/login');
        }
    });
</script>

<div class="dashboard-container">
    <div class="content-stack">
        <h1 class="text-7xl font-black text-[#2b0b0b] tracking-tighter uppercase text-center w-full">
            Command
        </h1>
        
        <p class="text-[10px] font-bold text-[#666] uppercase tracking-widest text-center opacity-70 -mt-4">
            Authority Control Center
        </p>

        <div class="brutalist-card">
            <div class="flex flex-col items-center text-center gap-4">
                <h2 class="text-xl font-black uppercase text-[#2b0b0b] leading-tight">
                    Welcome, {currentUser?.first_name || 'Officer'}
                </h2>
                <p class="text-sm opacity-80 leading-tight">
                    Monitor, assign, and resolve active campus grievances.
                </p>
                
                <button 
                    onclick={() => goto('/dashboard/student/grievances')} 
                    class="retro-btn w-full"
                >
                    ACCESS_GRIEVANCE_ARCHIVES
                </button>
            </div>
        </div>
        
        <div class="grid grid-cols-2 gap-2">
            <div class="stat-box">
                <span class="label">ROLE</span>
                <span class="value text-xs uppercase">{currentUser?.role || '???'}</span>
            </div>
            <div class="stat-box">
                <span class="label">DEPT</span>
                <span class="value text-xs uppercase">General</span>
            </div>
        </div>
    </div>
</div>

<style>
    /* Absolute transparency for notebook background visibility */
    .dashboard-container {
        display: flex;
        justify-content: center;
        width: 100%;
        padding: 4rem 0;
    }

    .content-stack {
        width: 400px;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .brutalist-card {
        border: 2px solid rgba(198, 225, 237, 0.6);
        padding: 2rem 1.25rem;
        box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.1);
        background: transparent;
    }

    .retro-btn {
        background: #b31b34;
        color: white;
        padding: 12px;
        border: 2px solid #2b0b0b;
        font-weight: 900;
        text-transform: uppercase;
        box-shadow: 4px 4px 0px #000;
        cursor: pointer;
        transition: 0.1s;
    }

    .retro-btn:hover {
        transform: translate(-1px, -1px);
        box-shadow: 5px 5px 0px #000;
    }

    /* Matching the Admin Stat Box style for symmetry */
    .stat-box {
        border: 2px solid rgba(198, 225, 237, 0.6);
        padding: 0.75rem;
        display: flex;
        flex-direction: column;
        background: transparent;
        box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.1);
    }

    .stat-box .label {
        font-size: 10px;
        color: #666;
        font-weight: 900;
        letter-spacing: 1px;
    }

    .stat-box .value {
        font-family: "Jersey 25", sans-serif;
        color: #2b0b0b;
    }
</style>