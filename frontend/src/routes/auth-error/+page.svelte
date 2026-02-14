<script lang="ts">
	import Card from '$lib/components/Card.svelte';
	import { browser } from '$app/environment';

	console.log('[AUTH-ERROR] Page loaded');
	
	let errorMessage = 'An authentication error occurred';
	let errorDetails = '';

	if (browser) {
		console.log('[AUTH-ERROR] Browser environment detected');
		console.log('[AUTH-ERROR] Current URL:', window.location.href);
		console.log('[AUTH-ERROR] Search params:', window.location.search);
		
		const urlParams = new URLSearchParams(window.location.search);
		const error = urlParams.get('error');
		const details = urlParams.get('details');
		
		console.log('[AUTH-ERROR] error param:', error);
		console.log('[AUTH-ERROR] details param:', details);
		
		if (error) {
			errorMessage = decodeURIComponent(error);
			console.log('[AUTH-ERROR] Decoded error message:', errorMessage);
		}
		if (details) {
			errorDetails = decodeURIComponent(details);
			console.log('[AUTH-ERROR] Decoded error details:', errorDetails);
		}
	}
</script>

<svelte:head>
	<title>Authentication Error - AEGIS</title>
</svelte:head>

<div class="container">
	<div class="h-screen flex items-center justify-center">
		<div class="loader-wrapper">
			<div class="loader"></div>
		</div>
		<Card>
			<div class="text-center content-wrapper">
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
</div>

<style>
	:global(body) {
		overflow: hidden;
	}

	.container {
		--color: #c09bd8;
		background-color: #9f84bd;
		width: 100%;
		height: 100%;
		position: relative;
	}

	.container::after {
		content: "";
		position: absolute;
		inset: 0;
		opacity: 0.8;
		background: radial-gradient(circle, var(--color) 15%, transparent 15%),
			radial-gradient(circle, var(--color) 15%, transparent 15%) 6px -6px,
			radial-gradient(circle, var(--color) 15%, transparent 15%) 6px -1px,
			radial-gradient(circle, var(--color) 15%, transparent 15%) 10px 2px,
			radial-gradient(circle, var(--color) 15%, transparent 15%) 14px 5px,
			radial-gradient(circle, var(--color) 15%, transparent 15%) 18px 8px,
			radial-gradient(circle, var(--color) 15%, transparent 15%) 18px 12px,
			radial-gradient(circle, var(--color) 15%, transparent 15%) 23px 6px;
		background-size: 50px 50px;
		animation: moveBones 3s linear infinite;
	}

	@keyframes moveBones {
		0% {
			background-position: 0 0, 6px -6px, 6px -1px, 10px 2px, 14px 5px, 18px 8px, 18px 12px, 23px 6px;
		}
		100% {
			background-position: 50px 50px, 56px 44px, 56px 49px, 60px 52px, 64px 55px, 68px 58px, 68px 62px, 73px 56px;
		}
	}

	.loader-wrapper {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		z-index: 1;
	}

	.loader {
		width: 160px;
		height: 185px;
		position: relative;
		background: #fff;
		border-radius: 100px 100px 0 0;
	}

	.loader:after {
		content: "";
		position: absolute;
		width: 100px;
		height: 125px;
		left: 50%;
		top: 25px;
		transform: translateX(-50%);
		background-image: radial-gradient(circle, #000 48%, transparent 55%),
			radial-gradient(circle, #000 48%, transparent 55%),
			radial-gradient(circle, #fff 30%, transparent 45%),
			radial-gradient(circle, #000 48%, transparent 51%),
			linear-gradient(#000 20px, transparent 0),
			linear-gradient(#cfecf9 60px, transparent 0),
			radial-gradient(circle, #cfecf9 50%, transparent 51%),
			radial-gradient(circle, #cfecf9 50%, transparent 51%);
		background-repeat: no-repeat;
		background-size: 16px 16px, 16px 16px, 10px 10px, 42px 42px, 12px 3px,
			50px 25px, 70px 70px, 70px 70px;
		background-position: 25px 10px, 55px 10px, 36px 44px, 50% 30px, 50% 85px,
			50% 50px, 50% 22px, 50% 45px;
		animation: faceLift 3s linear infinite alternate;
	}

	.loader:before {
		content: "";
		position: absolute;
		width: 140%;
		height: 125px;
		left: -20%;
		top: 0;
		background-image: radial-gradient(circle, #fff 48%, transparent 50%),
			radial-gradient(circle, #fff 48%, transparent 50%);
		background-repeat: no-repeat;
		background-size: 65px 65px;
		background-position: 0px 12px, 145px 12px;
		animation: earLift 3s linear infinite alternate;
	}

	@keyframes faceLift {
		0% {
			transform: translateX(-60%);
		}

		100% {
			transform: translateX(-30%);
		}
	}

	@keyframes earLift {
		0% {
			transform: translateX(10px);
		}

		100% {
			transform: translateX(0px);
		}
	}

	.content-wrapper {
		position: relative;
		z-index: 2;
	}
</style>
