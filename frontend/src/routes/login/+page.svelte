<script lang="ts">
	import { onMount } from 'svelte';
	import { authStore } from '$lib/auth';
	import { goto } from '$app/navigation';
	import { getGoogleLoginUrl } from '$lib/api';

	onMount(() => {
		// If already authenticated, redirect to dashboard
		const unsubscribe = authStore.subscribe(state => {
			if (state.isAuthenticated) {
				goto('/dashboard');
			}
		});

		return unsubscribe;
	});

	function handleGoogleLogin() {
		window.location.href = getGoogleLoginUrl();
	}
</script>

<div>
	<h1>Login to AEGIS</h1>
	<p>IIT Mandi Campus Management Platform</p>
	
	<button on:click={handleGoogleLogin}>
		Login with Google
	</button>
	
	<p>Only @iitmandi.ac.in and @students.iitmandi.ac.in emails allowed</p>
</div>
