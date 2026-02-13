<script lang="ts">
	import Card from '$lib/components/Card.svelte';
	import { PUBLIC_BACKEND_URL } from '$env/static/public';
	import { browser } from '$app/environment';
	import { page } from '$app/stores';

	let loginUrl = '';
	let error = '';
	
	if (browser) {
		const origin = window.location.origin;
		loginUrl = `${PUBLIC_BACKEND_URL}auth/google?origin=${encodeURIComponent(origin)}`;
		
		// Get error from URL query parameter
		const urlParams = new URLSearchParams(window.location.search);
		error = urlParams.get('error') || '';
	} else {
		loginUrl = `${PUBLIC_BACKEND_URL}auth/google`;
	}
</script>

<div class="h-screen flex items-center justify-center">
	<Card>
		<h1 class="text-2xl font-bold text-center mb-4">Login to AEGIS</h1>
		
		{#if error}
			<div class="alert alert-error mb-4">
				<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
				<span>{decodeURIComponent(error)}</span>
			</div>
		{/if}
		
		<a href={loginUrl} class="btn btn-primary w-full">
			Login with Google
		</a>
		<p class="text-center mt-2">Only @iitmandi.ac.in and @students.iitmandi.ac.in emails allowed</p>
	</Card>
</div>
