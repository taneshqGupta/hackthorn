<script lang="ts">
	import { user } from '$lib/auth';
	import { goto } from '$app/navigation';

	let currentUser = $derived($user);

	$effect(() => {
		if (currentUser && currentUser.role !== 'student') {
			goto(`/dashboard/${currentUser.role}`);
		} else if (!currentUser) {
			goto('/login');
		}
	});
</script>

<div class="min-h-screen bg-base-200 p-8">
	<div class="max-w-4xl mx-auto">
		<div class="text-center mb-8">
			<h1 class="text-4xl font-bold mb-4">Welcome, {currentUser?.first_name || 'Student'}!</h1>
			<p class="text-lg text-gray-600">Your voice matters. Report and track campus issues.</p>
		</div>

		<div class="card bg-base-100 shadow-xl">
			<div class="card-body items-center text-center">
				<h2 class="card-title text-2xl mb-4">Grievance System</h2>
				<p class="mb-6">Submit grievances, track their progress, and help improve campus life</p>
				<button onclick={() => goto('/grievances')} class="btn btn-primary btn-lg">
					Go to Grievance System
				</button>
			</div>
		</div>
	</div>
</div>
