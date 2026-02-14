<script lang="ts">
	import { user } from '$lib/auth';
	import { goto } from '$app/navigation';
	import type { User } from '$lib/types';

	console.log('[DASHBOARD] Page loaded');
	
	let currentUser: User | null = null;
	
	user.subscribe((value) => {
		console.log('[DASHBOARD] User subscription triggered, user:', value);
		currentUser = value;
		
		if (!value) {
			console.log('[DASHBOARD] No user found, redirecting to /login');
			goto('/login');
		} else {
			console.log('[DASHBOARD] User authenticated:', value.email);
		}
	});
</script>

<div class="container mx-auto p-8">
	{#if currentUser}
		<div class="mb-8">
			<h1 class="text-3xl font-bold mb-2">Welcome to AEGIS Dashboard</h1>
			<p class="text-gray-600">Hello, {currentUser.name}!</p>
		</div>

		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
			<!-- Profile Card -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<div class="flex items-center space-x-4 mb-4">
						<div class="avatar">
							<div class="w-16 rounded-full">
								<img
									src={currentUser.avatar ||
										`https://ui-avatars.com/api/?name=${currentUser.name}&background=random`}
									alt="user avatar"
								/>
							</div>
						</div>
						<div>
							<h2 class="card-title text-lg">{currentUser.name}</h2>
							<p class="text-sm text-gray-600">{currentUser.email}</p>
						</div>
					</div>
					<div class="card-actions justify-end">
						<a href="/profile" class="btn btn-primary btn-sm">View Profile</a>
					</div>
				</div>
			</div>

			<!-- Quick Actions Card -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">Quick Actions</h2>
					<p class="text-sm text-gray-600">Access common features</p>
					<div class="card-actions justify-end mt-4">
						<button class="btn btn-outline btn-sm">Create</button>
						<button class="btn btn-outline btn-sm">Browse</button>
					</div>
				</div>
			</div>

			<!-- Status Card -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title">Status</h2>
					<div class="badge badge-success">Active</div>
					<p class="text-sm text-gray-600 mt-2">All systems operational</p>
				</div>
			</div>
		</div>
	{:else}
		<div class="flex items-center justify-center h-64">
			<span class="loading loading-spinner loading-lg"></span>
		</div>
	{/if}
</div>
