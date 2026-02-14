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

<div class="container mx-auto p-8">
	{#if currentUser}
		<div class="mb-8">
			<h1 class="text-4xl font-bold mb-2">⚖️ Authority Dashboard</h1>
			<p class="text-gray-600">Welcome, {currentUser.first_name}!</p>
			<div class="badge badge-warning mt-2">Authority</div>
		</div>

		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
			<!-- Profile Card -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">👤 Profile</h2>
					<p class="text-sm text-gray-600">{currentUser.email}</p>
					<p class="text-sm text-gray-600">{currentUser.department || 'No department'}</p>
					<div class="card-actions justify-end mt-4">
						<a href="/profile" class="btn btn-primary btn-sm">View Profile</a>
					</div>
				</div>
			</div>

			<!-- Grievance Management -->
			<div class="card bg-base-100 shadow-xl hover:shadow-2xl transition-shadow">
				<div class="card-body">
					<h2 class="card-title">📢 Grievance Management</h2>
					<p class="text-sm text-gray-600">Review and resolve issues</p>
					<div class="grid grid-cols-2 gap-2 mt-2">
						<div class="stat bg-base-200 rounded p-2">
							<div class="stat-title text-xs">Pending</div>
							<div class="stat-value text-xl">0</div>
						</div>
						<div class="stat bg-base-200 rounded p-2">
							<div class="stat-title text-xs">Urgent</div>
							<div class="stat-value text-xl text-error">0</div>
						</div>
					</div>
					<div class="card-actions justify-end mt-4">
						<button class="btn btn-error btn-sm">View Urgent</button>
						<button class="btn btn-primary btn-sm">View All</button>
					</div>
				</div>
			</div>

			<!-- Department Grievances -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">🏢 My Department</h2>
					<p class="text-sm text-gray-600">{currentUser.department || 'All Departments'}</p>
					<div class="stats stats-vertical shadow mt-2">
						<div class="stat p-3">
							<div class="stat-title text-xs">Assigned to Me</div>
							<div class="stat-value text-2xl">0</div>
						</div>
					</div>
					<div class="card-actions justify-end mt-4">
						<button class="btn btn-outline btn-sm">Department View</button>
					</div>
				</div>
			</div>

			<!-- Quick Actions -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">⚡ Quick Actions</h2>
					<div class="space-y-2">
						<button class="btn btn-outline btn-block btn-sm">Assign Grievance</button>
						<button class="btn btn-outline btn-block btn-sm">Update Status</button>
						<button class="btn btn-outline btn-block btn-sm">Add Remarks</button>
					</div>
				</div>
			</div>

			<!-- Recent Activity -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">📊 Recent Activity</h2>
					<p class="text-sm text-gray-600">Latest updates</p>
					<div class="text-xs text-gray-500 mt-2">
						<p>No recent activity</p>
					</div>
				</div>
			</div>

			<!-- Statistics -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">📈 Statistics</h2>
					<div class="stats stats-vertical shadow">
						<div class="stat p-3">
							<div class="stat-title text-xs">Resolved This Month</div>
							<div class="stat-value text-2xl text-success">0</div>
						</div>
						<div class="stat p-3">
							<div class="stat-title text-xs">Avg Response Time</div>
							<div class="stat-value text-xl">- hrs</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	{:else}
		<div class="flex items-center justify-center h-64">
			<span class="loading loading-spinner loading-lg"></span>
		</div>
	{/if}
</div>
