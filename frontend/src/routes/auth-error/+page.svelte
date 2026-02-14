<script lang="ts">
	import Card from '$lib/components/Card.svelte';
	import { browser } from '$app/environment';

	let errorMessage = 'An authentication error occurred';
	let errorDetails = '';

	if (browser) {
		const urlParams = new URLSearchParams(window.location.search);
		const error = urlParams.get('error');
		const details = urlParams.get('details');
		
		if (error) {
			errorMessage = decodeURIComponent(error);
		}
		if (details) {
			errorDetails = decodeURIComponent(details);
		}
	}
</script>

<svelte:head>
	<title>Authentication Error - AEGIS</title>
</svelte:head>

<div class="h-screen flex items-center justify-center bg-base-200">
	<Card>
		<div class="text-center">
			<!-- Error Icon -->
			<div class="flex justify-center mb-4">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-24 w-24 text-error"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
					/>
				</svg>
			</div>

			<h1 class="text-3xl font-bold text-error mb-4">Authentication Failed</h1>
			
			<div class="alert alert-error shadow-lg mb-6">
				<div>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="stroke-current flex-shrink-0 h-6 w-6"
						fill="none"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
						/>
					</svg>
					<div class="text-left">
						<h3 class="font-bold">{errorMessage}</h3>
						{#if errorDetails}
							<div class="text-sm mt-1">{errorDetails}</div>
						{/if}
					</div>
				</div>
			</div>

			<div class="bg-base-300 p-4 rounded-lg mb-6 text-left">
				<h3 class="font-semibold mb-2">Allowed Email Domains:</h3>
				<ul class="list-disc list-inside space-y-1 text-sm">
					<li>@iitmandi.ac.in (Faculty/Staff)</li>
					<li>@students.iitmandi.ac.in (Students)</li>
				</ul>
			</div>

			<div class="flex gap-4 justify-center">
				<a href="/login" class="btn btn-primary">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-5 w-5 mr-2"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1"
						/>
					</svg>
					Try Again with Different Account
				</a>
				<a href="/" class="btn btn-outline">Return Home</a>
			</div>
		</div>
	</Card>
</div>

<style>
	:global(body) {
		overflow: hidden;
	}
</style>
