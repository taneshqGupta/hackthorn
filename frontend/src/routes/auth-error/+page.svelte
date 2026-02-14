<script lang="ts">
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
		<div class="content-wrapper">
			<h1 class="text-4xl font-bold text-white mb-6 text-center">Authentication Failed</h1>
			
			<div class="error-box">
				<h3 class="font-bold text-xl mb-2">{errorMessage}</h3>
				{#if errorDetails}
					<div class="text-base">{errorDetails}</div>
				{/if}
			</div>

			<div class="domains-box">
				<h3 class="font-semibold text-lg mb-3">Allowed Email Domains:</h3>
				<ul class="space-y-2 text-base">
					<li>• @iitmandi.ac.in (Faculty/Staff)</li>
					<li>• @students.iitmandi.ac.in (Students)</li>
				</ul>
			</div>

			<div class="flex flex-col sm:flex-row gap-4 justify-center mt-8">
				<a href="/login" class="btn btn-primary">
					Try Again with Different Account
				</a>
				<a href="/" class="btn btn-outline">Return Home</a>
			</div>
		</div>
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
		opacity: 1;
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
		bottom: 40px;
		left: 50%;
		transform: translateX(-50%);
		z-index: 1;
	}

	.loader {
		width: 420px;
		height: 487px;
		position: relative;
		background: #fff;
		border-radius: 262px 262px 0 0;
	}

	.loader:after {
		content: "";
		position: absolute;
		width: 262px;
		height: 328px;
		left: 50%;
		top: 66px;
		transform: translateX(-50%);
		background-image: radial-gradient(circle, #000 48%, transparent 55%),
			radial-gradient(circle, #000 48%, transparent 55%),
			radial-gradient(circle, #fff 30%, transparent 45%),
			radial-gradient(circle, #000 48%, transparent 51%),
			linear-gradient(#000 52px, transparent 0),
			linear-gradient(#cfecf9 157px, transparent 0),
			radial-gradient(circle, #cfecf9 50%, transparent 51%),
			radial-gradient(circle, #cfecf9 50%, transparent 51%);
		background-repeat: no-repeat;
		background-size: 42px 42px, 42px 42px, 26px 26px, 110px 110px, 31px 8px,
			131px 66px, 184px 184px, 184px 184px;
		background-position: 66px 26px, 144px 26px, 94px 115px, 50% 79px, 50% 223px,
			50% 131px, 50% 58px, 50% 118px;
		animation: faceLift 3s linear infinite alternate;
	}

	.loader:before {
		content: "";
		position: absolute;
		width: 140%;
		height: 328px;
		left: -20%;
		top: 0;
		background-image: radial-gradient(circle, #fff 48%, transparent 50%),
			radial-gradient(circle, #fff 48%, transparent 50%);
		background-repeat: no-repeat;
		background-size: 171px 171px;
		background-position: 0px 31px, 381px 31px;
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
		position: absolute;
		top: 0px;
		left: 50%;
		transform: translateX(-50%);
		z-index: 2;
		max-width: 600px;
		padding: 2rem;
		color: white;
		text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);
	}

	.error-box {
		background: rgba(255, 255, 255, 0.15);
		backdrop-filter: blur(10px);
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-radius: 12px;
		padding: 1.5rem;
		margin-bottom: 2rem;
		color: white;
	}

	.domains-box {
		background: rgba(255, 255, 255, 0.15);
		backdrop-filter: blur(10px);
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-radius: 12px;
		padding: 1.5rem;
		color: white;
	}
</style>
