<script lang="ts">
	import { user } from '$lib/auth';
	import { goto } from '$app/navigation';

	let currentUser = $derived($user);

	// Redirect if not a student
	$effect(() => {
		if (currentUser && currentUser.role !== 'student') {
			goto(`/dashboard/${currentUser.role}`);
		} else if (!currentUser) {
			goto('/login');
		}
	});
</script>

<div class="container mx-auto p-8">
	{#if currentUser}
		<div class="mb-8">
			<h1 class="text-4xl font-bold mb-2">🎓 Student Dashboard</h1>
			<p class="text-gray-600">Welcome, {currentUser.first_name}!</p>
			<div class="badge badge-info mt-2">Student</div>
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

			<!-- Grievances Card -->
			<div class="card bg-base-100 shadow-xl hover:shadow-2xl transition-shadow">
				<div class="card-body">
					<h2 class="card-title">📢 My Grievances</h2>
					<p class="text-sm text-gray-600">Submit and track your issues</p>
					<div class="stats stats-vertical shadow mt-2">
						<div class="stat p-3">
							<div class="stat-title text-xs">Submitted</div>
							<div class="stat-value text-2xl">0</div>
						</div>
					</div>
					<div class="card-actions justify-end mt-4">
						<button class="btn btn-primary btn-sm">Submit Grievance</button>
						<button class="btn btn-outline btn-sm">View All</button>
					</div>
				</div>
			</div>

			<!-- Academic Resources Card -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">📚 Academic</h2>
					<p class="text-sm text-gray-600">Course materials & papers</p>
					<div class="card-actions justify-end mt-4">
						<button class="btn btn-outline btn-sm">Browse</button>
					</div>
				</div>
			</div>

			<!-- Opportunities Card -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">💼 Opportunities</h2>
					<p class="text-sm text-gray-600">Internships & research</p>
					<div class="card-actions justify-end mt-4">
						<button class="btn btn-outline btn-sm">Explore</button>
					</div>
				</div>
			</div>

			<!-- Tasks Card -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">✅ My Tasks</h2>
					<p class="text-sm text-gray-600">Personal task manager</p>
					<div class="card-actions justify-end mt-4">
						<button class="btn btn-outline btn-sm">Manage</button>
					</div>
				</div>
			</div>

			<!-- Quick Links Card -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">🔗 Quick Links</h2>
					<ul class="text-sm space-y-2">
						<li><button class="link link-primary text-left">Campus Map</button></li>
						<li><button class="link link-primary text-left">Lost & Found</button></li>
						<li><button class="link link-primary text-left">Ride Sharing</button></li>
					</ul>
				</div>
			</div>
		</div>
	{:else}
		<div class="flex items-center justify-center h-64">
			<span class="loading loading-spinner loading-lg"></span>
		</div>
	{/if}
</div>
