<script lang="ts">
    import { onMount } from 'svelte';
    import api from '$lib/api';
    import type { ApiResponse, UserResponse } from '$lib/types';
    import { goto } from '$app/navigation';

    let users = $state<UserResponse[]>([]);
    let loading = $state(true);
    let search = $state('');

    // Load users on mount
    async function loadUsers() {
        loading = true;
        try {
            // Hits the existing get_all_users in admin.rs
            const res = await api.get<ApiResponse<UserResponse[]>>(`/api/admin/users?limit=100&search=${search}`);
            users = res.data || [];
        } catch (e) {
            console.error("Failed to load user roster", e);
        } finally {
            loading = false;
        }
    }

    // Hits the existing update_user_role in admin.rs
    async function handleRoleChange(userId: string, newRole: string) {
        if (!confirm(`CONFIRM ROLE CHANGE TO: ${newRole.toUpperCase()}?`)) return;
        
        try {
            await api.put(`/api/admin/users/${userId}/role`, { role: newRole });
            await loadUsers(); // Refresh the list
        } catch (e) {
            alert("COMMAND REJECTED: Role update failed.");
        }
    }

    onMount(loadUsers);
</script>

<div class="roster-container">
    <div class="header">
        <button onclick={() => goto('/dashboard/admin')} class="back-btn">← BACK TO COMMAND</button>
        <h1 class="text-6xl font-bold tracking-tighter uppercase">User Roster</h1>
    </div>

    <div class="search-bar">
        <input 
            type="text" 
            bind:value={search} 
            oninput={loadUsers}
            placeholder="SEARCH BY NAME OR EMAIL..." 
            class="roster-input"
        />
    </div>

    {#if loading}
        <div class="status-msg">SCANNING CITIZEN RECORDS...</div>
    {:else}
        <div class="table-wrapper">
            <table class="brutalist-table">
                <thead>
                    <tr>
                        <th>NAME</th>
                        <th>EMAIL</th>
                        <th>ROLE</th>
                        <th>ACTION</th>
                    </tr>
                </thead>
                <tbody>
                    {#each users as user}
                        <tr>
                            <td>{user.first_name} {user.last_name}</td>
                            <td class="email-cell">{user.email}</td>
                            <td><span class="role-badge {user.role}">{user.role}</span></td>
                            <td>
                                <select 
                                    class="role-select"
                                    value={user.role}
                                    onchange={(e) => handleRoleChange(user.id, (e.target as HTMLSelectElement).value)}
                                >
                                    <option value="student">STUDENT</option>
                                    <option value="faculty">FACULTY</option>
                                    <option value="authority">AUTHORITY</option>
                                    <option value="admin">ADMIN</option>
                                </select>
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    {/if}
</div>

<style>
    .roster-container { max-width: 900px; margin: 0 auto; padding: 2rem 1rem; }
    
    .header { margin-bottom: 2rem; }
    .back-btn { font-family: inherit; font-size: 0.8rem; text-decoration: underline; cursor: pointer; color: #666; margin-bottom: 1rem; }

    .roster-input {
        width: 100%; padding: 0.75rem; font-family: inherit; font-size: 16px;
        background: transparent; border: 2px solid rgba(198, 225, 237, 0.6);
        text-transform: uppercase; margin-bottom: 1.5rem;
    }

    .table-wrapper { overflow-x: auto; border: 2px solid #2b0b0b; box-shadow: 6px 6px 0px rgba(0,0,0,0.1); }
    
    .brutalist-table { width: 100%; border-collapse: collapse; background: transparent; }
    th, td { padding: 1rem; border: 1px solid rgba(198, 225, 237, 0.4); text-align: left; font-size: 14px; }
    th { background: rgba(198, 225, 237, 0.2); font-weight: 900; letter-spacing: 1px; }

    .role-badge { padding: 2px 6px; border: 1px solid #2b0b0b; font-size: 10px; font-weight: bold; text-transform: uppercase; }
    .role-badge.admin { background: #b31b34; color: white; }
    .role-badge.faculty { background: #e0f2fe; color: #0369a1; }

    .role-select {
        background: transparent; border: 1px solid #2b0b0b; font-family: inherit;
        font-size: 12px; padding: 4px; cursor: pointer;
    }

    .status-msg { text-align: center; margin-top: 3rem; font-family: 'Jersey 25', sans-serif; font-size: 1.5rem; }
    
    @media (max-width: 600px) {
        .email-cell { display: none; } /* Hide email on small screens to fit */
    }
</style>