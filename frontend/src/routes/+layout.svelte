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

	// PWA Install prompt
	let deferredPrompt: any = $state(null);
	let showInstallButton = $state(false);

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

		// Listen for the beforeinstallprompt event
		window.addEventListener('beforeinstallprompt', (e) => {
			e.preventDefault();
			deferredPrompt = e;
			showInstallButton = true;
			console.log('[PWA] Install prompt captured');
		});

		// Listen for successful installation
		window.addEventListener('appinstalled', () => {
			showInstallButton = false;
			deferredPrompt = null;
			console.log('[PWA] App installed successfully');
		});
	});

	async function handleInstallClick() {
		if (!deferredPrompt) {
			console.log('[PWA] No install prompt available');
			return;
		}

		deferredPrompt.prompt();
		const { outcome } = await deferredPrompt.userChoice;
		console.log('[PWA] User choice:', outcome);
		
		if (outcome === 'accepted') {
			showInstallButton = false;
		}
		deferredPrompt = null;
}
</script>

<style>
	.install-btn {
		font-size: 18px;
		color: #000;
		font-family: 'Roboto Condensed', sans-serif;
		font-weight: 800;
		cursor: pointer;
		position: relative;
		border: none;
		background: none;
		text-transform: uppercase;
		transition-timing-function: cubic-bezier(0.25, 0.8, 0.25, 1);
		transition-duration: 400ms;
		transition-property: color;
		padding: 0;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.install-btn:focus,
	.install-btn:hover {
		color: #000;
	}

	.install-btn:focus:after,
	.install-btn:hover:after {
		width: 100%;
		left: 0%;
	}

	.install-btn:after {
		content: "";
		pointer-events: none;
		bottom: -2px;
		left: 50%;
		position: absolute;
		width: 0%;
		height: 2px;
		background-color: #000;
		transition-timing-function: cubic-bezier(0.25, 0.8, 0.25, 1);
		transition-duration: 400ms;
		transition-property: width, left;
	}
</style>

<div class="h-screen flex flex-col overflow-hidden container">
	<header
		class="flex-none w-full border-b-2 flex items-center justify-between px-4"
		style="border-color: #d06065; height: 40px;"
	>
		<Logo />
		<div class="flex items-center gap-16">
			{#if showInstallButton}
				<button onclick={handleInstallClick} class="install-btn" aria-label="Install App">
					Install App
				</button>
			{/if}
			<BurgerMenu />
		</div>
	</header>

	<main class="flex-1 flex items-center justify-center">
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