<script lang="ts">
	import Card from "$lib/components/Card.svelte";
	import { PUBLIC_BACKEND_URL } from "$env/static/public";
	import { browser } from "$app/environment";
	import { page } from "$app/stores";
	import { user } from "$lib/auth";
	import { goto } from "$app/navigation";

	console.log("[LOGIN] Page loaded");
	console.log("[LOGIN] Backend URL:", PUBLIC_BACKEND_URL);

	let loginUrl = "";
	let error = "";

	// Redirect to dashboard if already logged in
	user.subscribe((currentUser) => {
		console.log("[LOGIN] User subscription update:", currentUser);
		if (currentUser) {
			console.log(
				"[LOGIN] User is already authenticated, redirecting to /dashboard",
			);
			goto("/dashboard");
		} else {
			console.log("[LOGIN] No user, staying on login page");
		}
	});

	if (browser) {
		const origin = window.location.origin;
		console.log("[LOGIN] Current origin:", origin);
		console.log("[LOGIN] Current URL:", window.location.href);
		console.log("[LOGIN] Search params:", window.location.search);

		loginUrl = `${PUBLIC_BACKEND_URL}auth/google?origin=${encodeURIComponent(origin)}`;
		console.log("[LOGIN] Generated login URL:", loginUrl);

		// Get error from URL query parameter
		const urlParams = new URLSearchParams(window.location.search);
		error = urlParams.get("error") || "";

		if (error) {
			console.log("[LOGIN] Error parameter found:", error);
		}
	} else {
		console.log("[LOGIN] Server-side render, using simple login URL");
		loginUrl = `${PUBLIC_BACKEND_URL}auth/google`;
	}
</script>

<style>
	.login-container {
		display: flex;
		flex-direction: column;
		height: 100%;
		justify-content: space-between;
	}

	.center-content {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 20px;
		flex: 1;
	}

	.ui-btn {
		--btn-default-bg: rgb(41, 41, 41);
		--btn-padding: 15px 20px;
		--btn-hover-bg: rgb(51, 51, 51);
		--btn-transition: .3s;
		--btn-letter-spacing: .1rem;
		--btn-animation-duration: 1.2s;
		--btn-shadow-color: rgba(0, 0, 0, 0.137);
		--btn-shadow: 0 2px 10px 0 var(--btn-shadow-color);
		--hover-btn-color: #FAC921;
		--default-btn-color: #fff;
		--font-size: 16px;
		--font-weight: 600;
		--font-family: Menlo,Roboto Mono,monospace;
	}

	.ui-btn {
		box-sizing: border-box;
		padding: var(--btn-padding);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--default-btn-color);
		font: var(--font-weight) var(--font-size) var(--font-family);
		background: var(--btn-default-bg);
		border: none;
		cursor: pointer;
		transition: var(--btn-transition);
		overflow: hidden;
		box-shadow: var(--btn-shadow);
		text-decoration: none;
	}

	.ui-btn span {
		letter-spacing: var(--btn-letter-spacing);
		transition: var(--btn-transition);
		box-sizing: border-box;
		position: relative;
		background: inherit;
	}

	.ui-btn span::before {
		box-sizing: border-box;
		position: absolute;
		content: "";
		background: inherit;
	}

	.ui-btn:hover, .ui-btn:focus {
		background: var(--btn-hover-bg);
	}

	.ui-btn:hover span, .ui-btn:focus span {
		color: var(--hover-btn-color);
	}

	.ui-btn:hover span::before, .ui-btn:focus span::before {
		animation: chitchat linear both var(--btn-animation-duration);
	}

	@keyframes chitchat {
		0% {
			content: "#";
		}

		5% {
			content: ".";
		}

		10% {
			content: "^{";
		}

		15% {
			content: "-!";
		}

		20% {
			content: "#$_";
		}

		25% {
			content: "№:0";
		}

		30% {
			content: "#{+.";
		}

		35% {
			content: "@}-?";
		}

		40% {
			content: "?{4@%";
		}

		45% {
			content: "=.,^!";
		}

		50% {
			content: "?2@%";
		}

		55% {
			content: "\;1}]";
		}

		60% {
			content: "?{%:%";
			right: 0;
		}

		65% {
			content: "|{f[4";
			right: 0;
		}

		70% {
			content: "{4%0%";
			right: 0;
		}

		75% {
			content: "'1_0<";
			right: 0;
		}

		80% {
			content: "{0%";
			right: 0;
		}

		85% {
			content: "]>'";
			right: 0;
		}

		90% {
			content: "4";
			right: 0;
		}

		95% {
			content: "2";
			right: 0;
		}

		100% {
			content: "";
			right: 0;
		}
	}

	.email-restriction {
		text-align: center;
		font-size: 12px;
		margin-top: auto;
		padding-top: 8px;
	}
</style>

<div class="h-screen flex items-center justify-center">
	<Card title="">
		<div class="login-container">
			{#if error}
				<div class="alert alert-error mb-4">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="stroke-current shrink-0 h-6 w-6"
						fill="none"
						viewBox="0 0 24 24"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
						/></svg
					>
					<span>{decodeURIComponent(error)}</span>
				</div>
			{/if}

			<div class="center-content">
				<a href={loginUrl} class="ui-btn">
					<span>LOGIN with GOOGLE</span>
				</a>
			</div>

			<p class="email-restriction">
				Only @iitmandi.ac.in and @students.iitmandi.ac.in emails allowed
			</p>
		</div>
	</Card>
</div>
