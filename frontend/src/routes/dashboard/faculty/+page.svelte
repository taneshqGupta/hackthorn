<script lang="ts">
    import { user } from '$lib/auth';
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';
    import api from '$lib/api';
    import type { Grievance, ApiResponse } from '$lib/types';

    let currentUser = $derived($user);
    let assignments = $state<Grievance[]>([]);
    let loading = $state(true);

    // Auth Guard
    $effect(() => {
        if (currentUser && currentUser.role !== 'faculty') {
            goto(`/dashboard/${currentUser.role}`);
        } else if (!currentUser) {
            goto('/login');
        }
    });

    onMount(async () => {
        if (currentUser) {
            await loadAssignments();
        }
    });

    async function loadAssignments() {
        loading = true;
        try {
            // Fetch grievances where assigned_to matches current user ID
            const res = await api.get<ApiResponse<Grievance[]>>(`/api/grievances?assigned_to=${currentUser?.id}`);
            assignments = res.data || [];
        } catch (e) {
            console.error("Failed to load assignments", e);
        } finally {
            loading = false;
        }
    }

    function formatDate(d: string) {
        return new Date(d).toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
    }
</script>

<div class="dashboard-container">
    <div class="header">
        <h1>FACULTY CONSOLE</h1>
        <p class="subtitle">WELCOME, PROFESSOR {currentUser?.first_name?.toUpperCase() || 'FACULTY'}.</p>
    </div>

    {#if loading}
        <div class="state-msg">RETRIEVING ASSIGNMENTS...</div>
    {:else if assignments.length === 0}
        <div class="empty-state">
            <div class="box">
                <span class="check">✓</span>
                <p>NO PENDING GRIEVANCES</p>
                <small>You're all caught up.</small>
            </div>
        </div>
    {:else}
        <div class="task-grid">
            {#each assignments as task}
                <div class="task-card">
                    <div class="card-top">
                        <span class="status {task.status}">{task.status.replace('_', ' ')}</span>
                        <span class="date">{formatDate(task.created_at)}</span>
                    </div>
                    
                    <h3 class="task-title">{task.title}</h3>
                    <p class="task-desc">{task.description.slice(0, 120)}...</p>
                    
                    <div class="card-actions">
                        <button class="retro-btn" onclick={() => goto(`/grievances/${task.id}`)}>
                            REVIEW CASE
                        </button>
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    :global(body) { background-color: transparent; }

    .dashboard-container {
        max-width: 1000px;
        margin: 0 auto;
        padding: 40px 20px;
    }

    .header {
        text-align: center;
        margin-bottom: 40px;
    }

    h1 {
        font-size: 3rem;
        margin: 0 0 10px 0;
        color: #2b0b0b;
        line-height: 1;
    }

    .subtitle {
        font-size: 1.2rem;
        color: #666;
        font-family: inherit;
    }

    /* Grid Layout */
    .task-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 20px;
    }

    /* Brutalist Transparent Card */
    .task-card {
        background: transparent;
        border: 2px solid #2b0b0b;
        padding: 20px;
        box-shadow: 4px 4px 0px rgba(0,0,0,0.1);
        display: flex;
        flex-direction: column;
        transition: transform 0.1s;
    }

    .task-card:hover {
        transform: translate(-2px, -2px);
        box-shadow: 6px 6px 0px rgba(0,0,0,0.2);
    }

    .card-top {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 15px;
    }

    .status {
        font-size: 0.8rem;
        font-weight: 700;
        text-transform: uppercase;
        padding: 2px 8px;
        border: 1px solid #2b0b0b;
        background: #fff;
    }
    /* Status Colors */
    .submitted { background: #e0f2fe; }
    .under_review { background: #ffedd5; }
    .in_progress { background: #dcfce7; }
    .resolved { background: #d1fae5; border-color: #059669; color: #047857; }

    .date {
        font-size: 0.9rem;
        color: #666;
    }

    .task-title {
        margin: 0 0 10px 0;
        font-size: 1.4rem;
        color: #2b0b0b;
        line-height: 1.2;
    }

    .task-desc {
        color: #444;
        font-size: 1rem;
        line-height: 1.5;
        margin-bottom: 20px;
        flex-grow: 1;
    }

    .card-actions {
        margin-top: auto;
    }

    .retro-btn {
        width: 100%;
        background: #b31b34;
        color: #fff;
        border: 2px solid #2b0b0b;
        padding: 8px;
        font-family: inherit;
        font-weight: 600;
        font-size: 1rem;
        cursor: pointer;
        transition: background 0.2s;
    }
    .retro-btn:hover {
        background: #2b0b0b;
    }

    /* Empty State */
    .empty-state {
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 300px;
    }
    .box {
        text-align: center;
        border: 2px dashed #ccc;
        padding: 40px;
        color: #888;
    }
    .check {
        font-size: 3rem;
        display: block;
        margin-bottom: 10px;
        color: #10b981;
    }

    .state-msg {
        text-align: center;
        margin-top: 50px;
        font-size: 1.5rem;
        color: #666;
    }
</style>