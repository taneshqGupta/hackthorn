<script lang="ts">
	import '../app.css';
	import Logo from '$lib/components/Logo.svelte';
	import BurgerMenu from '$lib/components/BurgerMenu.svelte';
	import DevRoleSwitcher from '$lib/components/DevRoleSwitcher.svelte';
	import { user } from '$lib/auth';
	import type { User } from '$lib/types';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';

	let { children } = $props();
	
	// Use $derived for reactive values from stores in Svelte 5
	let currentUser = $derived($user);
	
	// Show dev role switcher in development mode
	const isDev = browser && (import.meta.env.DEV || import.meta.env.MODE === 'development');

	// PWA Service Worker Registration
	onMount(async () => {
		if ('serviceWorker' in navigator) {
			try {
				const registration = await navigator.serviceWorker.register('/service-worker.js', {
					scope: '/'
				});
				console.log('[PWA] Service Worker registered:', registration);
				
				// Check for updates
				registration.addEventListener('updatefound', () => {
					const newWorker = registration.installing;
					if (newWorker) {
						newWorker.addEventListener('statechange', () => {
							if (newWorker.state === 'installed' && navigator.serviceWorker.controller) {
								console.log('[PWA] New content available, please refresh');
							}
						});
					}
				});
			} catch (error) {
				console.log('[PWA] Service Worker registration failed:', error);
			}
		}
	});
</script>

<div class="h-screen flex flex-col overflow-hidden container">
	<header
		class="flex-none w-full border-b-2 flex items-center justify-between px-4"
		style="border-color: #d06065; height: 40px;"
	>
		<Logo />
		<BurgerMenu />
	</header>

	<main class="flex-1 overflow-auto">
		{@render children()}
	</main>

	<footer
		class="flex-none w-full border-t-2 flex items-center justify-center"
		style="border-color: #d06065; background-color: rgba(255, 179, 186, 0.2); height: 40px;"
	>
	</footer>
	
	<!-- Dev Mode Role Switcher -->
	{#if isDev && currentUser}
		<DevRoleSwitcher />
	{/if}
</div>