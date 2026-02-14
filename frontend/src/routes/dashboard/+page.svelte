<script lang="ts">
	import { user } from '$lib/auth';
	import { goto } from '$app/navigation';
	import type { User } from '$lib/types';

	console.log('[DASHBOARD] Page loaded');
	
	let currentUser: User | null = null;
	let authCheckComplete = false;
	
	user.subscribe((value) => {
		console.log('[DASHBOARD] User subscription triggered');
		console.log('[DASHBOARD] User value:', value);
		
		currentUser = value;
		authCheckComplete = true;
		
		if (!value) {
			console.log('[DASHBOARD] No user found after auth check, redirecting to /login');
			setTimeout(() => {
				goto('/login');
			}, 100);
		} else {
			console.log('[DASHBOARD] User authenticated, redirecting to role-based dashboard');
			// Redirect to role-based dashboard
			setTimeout(() => {
				goto(`/dashboard/${value.role}`);
			}, 100);
		}
	});
</script>

<div class="container mx-auto p-8">
	<div class="flex items-center justify-center h-64">
		<div class="text-center">
			<span class="loading loading-spinner loading-lg"></span>
			<p class="mt-4 text-gray-600">Redirecting to your dashboard...</p>
		</div>
	</div>
</div>
