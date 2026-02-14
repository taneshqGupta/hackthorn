<script lang="ts">
	import { user } from '$lib/auth';
	import { goto } from '$app/navigation';

	let currentUser = $derived($user);

	$effect(() => {
		if (currentUser && currentUser.role !== 'faculty') {
			goto(`/dashboard/${currentUser.role}`);
		} else if (!currentUser) {
			goto('/login');
		}
	});
</script>

<div class="container mx-auto p-8">
	{#if currentUser}
		<div class="mb-8">
			<h1 class="text-4xl font-bold mb-2">👨‍🏫 Faculty Dashboard</h1>
			<p class="text-gray-600">Welcome, Dr. {currentUser.last_name}!</p>
			<div class="badge badge-success mt-2">Faculty</div>
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

			<!-- Assigned Grievances -->
			<div class="card bg-base-100 shadow-xl hover:shadow-2xl transition-shadow">
				<div class="card-body">
					<h2 class="card-title">📋 Assigned Grievances</h2>
					<p class="text-sm text-gray-600">Issues assigned to you</p>
					<div class="stats stats-vertical shadow mt-2">
						<div class="stat p-3">
							<div class="stat-title text-xs">Pending</div>
							<div class="stat-value text-2xl">0</div>
						</div>
					</div>
					<div class="card-actions justify-end mt-4">
						<button class="btn btn-primary btn-sm">View Assigned</button>
					</div>
				</div>
			</div>

			<!-- Post Opportunities -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">💼 Research Opportunities</h2>
					<p class="text-sm text-gray-600">Post internships & research</p>
					<div class="card-actions justify-end mt-4">
						<button class="btn btn-primary btn-sm">Post New</button>
						<button class="btn btn-outline btn-sm">View All</button>
					</div>
				</div>
			</div>

			<!-- Applications -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">📬 Applications</h2>
					<p class="text-sm text-gray-600">Student applications</p>
					<div class="card-actions justify-end mt-4">
						<button class="btn btn-outline btn-sm">Review</button>
					</div>
				</div>
			</div>

			<!-- Courses -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">📚 My Courses</h2>
					<p class="text-sm text-gray-600">Course management</p>
					<div class="card-actions justify-end mt-4">
						<button class="btn btn-outline btn-sm">Manage</button>
					</div>
				</div>
			</div>

			<!-- Resources -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">📄 Resources</h2>
					<p class="text-sm text-gray-600">Upload study materials</p>
					<div class="card-actions justify-end mt-4">
						<button class="btn btn-outline btn-sm">Upload</button>
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
