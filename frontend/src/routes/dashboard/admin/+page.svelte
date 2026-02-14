<script lang="ts">
	import { user } from '$lib/auth';
	import { api } from '$lib/api';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	let currentUser = $derived($user);
	let stats: any = $state(null);
	let users: any[] = $state([]);
	let auditLogs: any[] = $state([]);
	let loading = $state(true);
	let activeTab = $state('overview');

	$effect(() => {
		if (currentUser && currentUser.role !== 'admin') {
			goto(`/dashboard/${currentUser.role}`);
		} else if (!currentUser) {
			goto('/login');
		}
	});

	async function loadData() {
		if (!currentUser || currentUser.role !== 'admin') return;

		loading = true;
		try {
			const [statsRes, usersRes, logsRes] = await Promise.all([
				api.get('/admin/stats'),
				api.get('/admin/users'),
				api.get('/admin/audit-logs?limit=10')
			]);

			stats = statsRes.data;
			users = usersRes.data || [];
			auditLogs = logsRes.data || [];
		} catch (error) {
			console.error('Failed to load admin data:', error);
		} finally {
			loading = false;
		}
	}

	async function changeUserRole(userId: string, newRole: string) {
		try {
			await api.put(`/admin/users/${userId}/role`, { role: newRole });
			await loadData(); // Reload data
		} catch (error) {
			console.error('Failed to update role:', error);
			alert('Failed to update user role');
		}
	}

	async function changeUserStatus(userId: string, newStatus: string) {
		try {
			await api.put(`/admin/users/${userId}/status`, { status: newStatus });
			await loadData(); // Reload data
		} catch (error) {
			console.error('Failed to update status:', error);
			alert('Failed to update user status');
		}
	}

	onMount(() => {
		loadData();
	});
</script>

<div class="container mx-auto p-8">
	{#if currentUser}
		<div class="mb-8">
			<h1 class="text-4xl font-bold mb-2">👑 Admin Dashboard</h1>
			<p class="text-gray-600">High Command Control Panel</p>
			<div class="badge badge-error mt-2">Admin</div>
		</div>

		<!-- Tabs -->
		<div class="tabs tabs-boxed mb-6">
			<button
				class="tab"
				class:tab-active={activeTab === 'overview'}
				onclick={() => (activeTab = 'overview')}
			>
				📊 Overview
			</button>
			<button
				class="tab"
				class:tab-active={activeTab === 'users'}
				onclick={() => (activeTab = 'users')}
			>
				👥 Users
			</button>
			<button
				class="tab"
				class:tab-active={activeTab === 'grievances'}
				onclick={() => (activeTab = 'grievances')}
			>
				📢 Grievances
			</button>
			<button
				class="tab"
				class:tab-active={activeTab === 'audit'}
				onclick={() => (activeTab = 'audit')}
			>
				📜 Audit Logs
			</button>
		</div>

		{#if loading}
			<div class="flex items-center justify-center h-64">
				<span class="loading loading-spinner loading-lg"></span>
			</div>
		{:else if activeTab === 'overview' && stats}
			<!-- System Stats -->
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-6">
				<div class="stat bg-base-100 shadow rounded-lg">
					<div class="stat-title">Total Users</div>
					<div class="stat-value">{stats.total_users}</div>
					<div class="stat-desc">Active: {stats.active_users}</div>
				</div>
				<div class="stat bg-base-100 shadow rounded-lg">
					<div class="stat-title">Total Grievances</div>
					<div class="stat-value">{stats.total_grievances}</div>
				</div>
				<div class="stat bg-base-100 shadow rounded-lg">
					<div class="stat-title">Pending</div>
					<div class="stat-value text-warning">{stats.pending_grievances}</div>
				</div>
				<div class="stat bg-base-100 shadow rounded-lg">
					<div class="stat-title">Resolved</div>
					<div class="stat-value text-success">{stats.resolved_grievances}</div>
				</div>
			</div>

			<!-- Users by Role -->
			<div class="card bg-base-100 shadow-xl mb-6">
				<div class="card-body">
					<h2 class="card-title">Users by Role</h2>
					<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
						{#each Object.entries(stats.users_by_role) as [role, count]}
							<div class="stat bg-base-200 rounded">
								<div class="stat-title capitalize">{role}</div>
								<div class="stat-value text-2xl">{count}</div>
							</div>
						{/each}
					</div>
				</div>
			</div>

			<!-- Quick Actions -->
			<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
				<div class="card bg-base-100 shadow-xl">
					<div class="card-body">
						<h2 class="card-title">🔧 System Health</h2>
						<div class="badge badge-success">All Systems Operational</div>
						<p class="text-sm text-gray-600 mt-2">Database, Auth, and Services running</p>
					</div>
				</div>
				<div class="card bg-base-100 shadow-xl">
					<div class="card-body">
						<h2 class="card-title">⚡ Quick Actions</h2>
						<div class="space-y-2">
							<button
								class="btn btn-outline btn-block btn-sm"
								onclick={() => (activeTab = 'users')}
							>
								Manage Users
							</button>
							<button
								class="btn btn-outline btn-block btn-sm"
								onclick={() => (activeTab = 'grievances')}
							>
								View Grievances
							</button>
						</div>
					</div>
				</div>
				<div class="card bg-base-100 shadow-xl">
					<div class="card-body">
						<h2 class="card-title">📈 Analytics</h2>
						<p class="text-sm text-gray-600">Response times, trends, patterns</p>
						<button class="btn btn-outline btn-sm mt-2">View Reports</button>
					</div>
				</div>
			</div>
		{:else if activeTab === 'users'}
			<!-- User Management -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">User Management</h2>
					<div class="overflow-x-auto">
						<table class="table table-zebra">
							<thead>
								<tr>
									<th>Name</th>
									<th>Email</th>
									<th>Role</th>
									<th>Status</th>
									<th>Actions</th>
								</tr>
							</thead>
							<tbody>
								{#each users as u}
									<tr>
										<td>{u.first_name} {u.last_name}</td>
										<td class="text-sm">{u.email}</td>
										<td>
											<select
												class="select select-bordered select-sm"
												value={u.role}
											onchange={(e) => changeUserRole(u.id, e.currentTarget.value)}
											disabled={u.id === currentUser.id}
										>
											<option value="student">Student</option>
											<option value="faculty">Faculty</option>
											<option value="authority">Authority</option>
											<option value="admin">Admin</option>
										</select>
									</td>
									<td>
										<select
											class="select select-bordered select-sm"
											value={u.status}
											onchange={(e) => changeUserStatus(u.id, e.currentTarget.value)}
												disabled={u.id === currentUser.id}
											>
												<option value="active">Active</option>
												<option value="inactive">Inactive</option>
												<option value="suspended">Suspended</option>
											</select>
										</td>
										<td>
											{#if u.id === currentUser.id}
												<span class="badge badge-sm">You</span>
											{/if}
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</div>
			</div>
		{:else if activeTab === 'grievances'}
			<!-- Grievances Management -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">Grievance Management</h2>
					<p>Coming soon: View and manage all grievances</p>
				</div>
			</div>
		{:else if activeTab === 'audit'}
			<!-- Audit Logs -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">Audit Logs</h2>
					<div class="overflow-x-auto">
						<table class="table table-sm">
							<thead>
								<tr>
									<th>Time</th>
									<th>User</th>
									<th>Action</th>
									<th>Details</th>
								</tr>
							</thead>
							<tbody>
								{#each auditLogs as log}
									<tr>
										<td class="text-xs">{new Date(log.created_at).toLocaleString()}</td>
										<td class="text-sm">{log.user?.first_name || 'System'}</td>
										<td class="text-sm font-mono">{log.action}</td>
										<td class="text-xs">{JSON.stringify(log.metadata)}</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</div>
			</div>
		{/if}
	{:else}
		<div class="flex items-center justify-center h-64">
			<span class="loading loading-spinner loading-lg"></span>
		</div>
	{/if}
</div>
