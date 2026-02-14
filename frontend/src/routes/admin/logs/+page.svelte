<script lang="ts">
    import { onMount } from 'svelte';
    import api from '$lib/api';
    import type { ApiResponse, AuditLogResponse } from '$lib/types';
    import { goto } from '$app/navigation';

    let logs = $state<AuditLogResponse[]>([]);
    let loading = $state(true);
    let offset = $state(0);
    const limit = 50;

    async function loadLogs() {
        loading = true;
        try {
            // Hits the existing get_audit_logs in admin.rs
            const res = await api.get<ApiResponse<AuditLogResponse[]>>(
                `/api/admin/audit-logs?limit=${limit}&offset=${offset}`
            );
            logs = res.data || [];
        } catch (e) {
            console.error("Failed to fetch logs", e);
        } finally {
            loading = false;
        }
    }

    function formatMetadata(metadata: any) {
        if (!metadata) return '';
        return JSON.stringify(metadata).replace(/[{}"]/g, ' ').slice(0, 50) + '...';
    }

    onMount(loadLogs);
</script>

<div class="logs-container">
    <div class="header">
        <button onclick={() => goto('/dashboard/admin')} class="back-btn">← BACK TO COMMAND</button>
        <h1 class="text-6xl font-bold tracking-tighter uppercase">Audit Trail</h1>
    </div>

    {#if loading}
        <div class="status-msg">EXTRACTING SYSTEM LOGS...</div>
    {:else}
        <div class="table-wrapper">
            <table class="brutalist-table">
                <thead>
                    <tr>
                        <th>TIMESTAMP</th>
                        <th>ACTION</th>
                        <th>AGENT</th>
                        <th class="meta-cell">METADATA</th>
                    </tr>
                </thead>
                <tbody>
                    {#each logs as log}
                        <tr>
                            <td class="time-cell">{new Date(log.created_at).toLocaleString()}</td>
                            <td class="action-cell">
                                <span class="action-tag">{log.action.replace('_', ' ')}</span>
                            </td>
                            <td>{log.user ? `${log.user.first_name} ${log.user.last_name}` : 'SYSTEM'}</td>
                            <td class="meta-cell font-mono text-[10px] opacity-60">
                                {formatMetadata(log.metadata)}
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>

        <div class="pagination mt-4 flex gap-4">
            <button 
                class="retro-btn-sm" 
                onclick={() => { offset = Math.max(0, offset - limit); loadLogs(); }}
                disabled={offset === 0}
            >PREV</button>
            <button 
                class="retro-btn-sm" 
                onclick={() => { offset += limit; loadLogs(); }}
            >NEXT</button>
        </div>
    {/if}
</div>

<style>
    .logs-container { max-width: 1000px; margin: 0 auto; padding: 2rem 1rem; }
    .header { margin-bottom: 2rem; }
    .back-btn { font-family: inherit; font-size: 0.8rem; text-decoration: underline; cursor: pointer; color: #666; }

    .table-wrapper { border: 2px solid #2b0b0b; box-shadow: 6px 6px 0px rgba(0,0,0,0.1); overflow-x: auto; }
    .brutalist-table { width: 100%; border-collapse: collapse; background: transparent; }
    
    th, td { padding: 0.75rem 1rem; border: 1px solid rgba(198, 225, 237, 0.4); text-align: left; font-size: 12px; }
    th { background: #2b0b0b; color: white; text-transform: uppercase; letter-spacing: 1px; }

    .action-tag { font-weight: 900; color: #b31b34; text-transform: uppercase; }
    .time-cell { color: #666; white-space: nowrap; }

    .retro-btn-sm {
        background: transparent; border: 2px solid #2b0b0b; padding: 4px 12px;
        font-family: inherit; font-weight: bold; cursor: pointer;
    }
    .retro-btn-sm:disabled { opacity: 0.3; cursor: not-allowed; }

    .status-msg { text-align: center; margin-top: 3rem; font-family: 'Jersey 25', sans-serif; font-size: 1.5rem; }

    @media (max-width: 800px) {
        .meta-cell { display: none; }
    }
</style>