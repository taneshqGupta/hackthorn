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
		--btn-padding: 20px 30px;
		--btn-hover-bg: rgb(51, 51, 51);
		--btn-transition: .3s;
		--btn-letter-spacing: .1rem;
		--btn-animation-duration: 1.2s;
		--btn-shadow-color: rgba(0, 0, 0, 0.137);
		--btn-shadow: 0 2px 10px 0 var(--btn-shadow-color);
		--hover-btn-color: #FAC921;
		--default-btn-color: #fff;
		--font-size: 20px;
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

	.login-layout {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 3rem;
		padding: 2rem;
	}

	.login-card-wrapper {
		width: 500px;
		max-width: 90vw;
	}

	@media (min-width: 768px) {
		.login-layout {
			flex-direction: row;
			gap: 4rem;
		}
	}

	.wheel-and-hamster {
		--dur: 1s;
		position: relative;
		width: 12em;
		height: 12em;
		font-size: 14px;
	}

	.wheel,
	.hamster,
	.hamster div,
	.spoke {
		position: absolute;
	}

	.wheel,
	.spoke {
		border-radius: 50%;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
	}

	.wheel {
		background: radial-gradient(100% 100% at center,hsla(0,0%,60%,0) 47.8%,hsl(0,0%,60%) 48%);
		z-index: 2;
	}

	.hamster {
		animation: hamster var(--dur) ease-in-out infinite;
		top: 50%;
		left: calc(50% - 3.5em);
		width: 7em;
		height: 3.75em;
		transform: rotate(4deg) translate(-0.8em,1.85em);
		transform-origin: 50% 0;
		z-index: 1;
	}

	.hamster__head {
		animation: hamsterHead var(--dur) ease-in-out infinite;
		background: hsl(30,90%,55%);
		border-radius: 70% 30% 0 100% / 40% 25% 25% 60%;
		box-shadow: 0 -0.25em 0 hsl(30,90%,80%) inset,
			0.75em -1.55em 0 hsl(30,90%,90%) inset;
		top: 0;
		left: -2em;
		width: 2.75em;
		height: 2.5em;
		transform-origin: 100% 50%;
	}

	.hamster__ear {
		animation: hamsterEar var(--dur) ease-in-out infinite;
		background: hsl(0,90%,85%);
		border-radius: 50%;
		box-shadow: -0.25em 0 hsl(30,90%,55%) inset;
		top: -0.25em;
		right: -0.25em;
		width: 0.75em;
		height: 0.75em;
		transform-origin: 50% 75%;
	}

	.hamster__eye {
		animation: hamsterEye var(--dur) linear infinite;
		background-color: hsl(0,0%,0%);
		border-radius: 50%;
		top: 0.375em;
		left: 1.25em;
		width: 0.5em;
		height: 0.5em;
	}

	.hamster__nose {
		background: hsl(0,90%,75%);
		border-radius: 35% 65% 85% 15% / 70% 50% 50% 30%;
		top: 0.75em;
		left: 0;
		width: 0.2em;
		height: 0.25em;
	}

	.hamster__body {
		animation: hamsterBody var(--dur) ease-in-out infinite;
		background: hsl(30,90%,90%);
		border-radius: 50% 30% 50% 30% / 15% 60% 40% 40%;
		box-shadow: 0.1em 0.75em 0 hsl(30,90%,55%) inset,
			0.15em -0.5em 0 hsl(30,90%,80%) inset;
		top: 0.25em;
		left: 2em;
		width: 4.5em;
		height: 3em;
		transform-origin: 17% 50%;
		transform-style: preserve-3d;
	}

	.hamster__limb--fr,
	.hamster__limb--fl {
		clip-path: polygon(0 0,100% 0,70% 80%,60% 100%,0% 100%,40% 80%);
		top: 2em;
		left: 0.5em;
		width: 1em;
		height: 1.5em;
		transform-origin: 50% 0;
	}

	.hamster__limb--fr {
		animation: hamsterFRLimb var(--dur) linear infinite;
		background: linear-gradient(hsl(30,90%,80%) 80%,hsl(0,90%,75%) 80%);
		transform: rotate(15deg) translateZ(-1px);
	}

	.hamster__limb--fl {
		animation: hamsterFLLimb var(--dur) linear infinite;
		background: linear-gradient(hsl(30,90%,90%) 80%,hsl(0,90%,85%) 80%);
		transform: rotate(15deg);
	}

	.hamster__limb--br,
	.hamster__limb--bl {
		border-radius: 0.75em 0.75em 0 0;
		clip-path: polygon(0 0,100% 0,100% 30%,70% 90%,70% 100%,30% 100%,40% 90%,0% 30%);
		top: 1em;
		left: 2.8em;
		width: 1.5em;
		height: 2.5em;
		transform-origin: 50% 30%;
	}

	.hamster__limb--br {
		animation: hamsterBRLimb var(--dur) linear infinite;
		background: linear-gradient(hsl(30,90%,80%) 90%,hsl(0,90%,75%) 90%);
		transform: rotate(-25deg) translateZ(-1px);
	}

	.hamster__limb--bl {
		animation: hamsterBLLimb var(--dur) linear infinite;
		background: linear-gradient(hsl(30,90%,90%) 90%,hsl(0,90%,85%) 90%);
		transform: rotate(-25deg);
	}

	.hamster__tail {
		animation: hamsterTail var(--dur) linear infinite;
		background: hsl(0,90%,85%);
		border-radius: 0.25em 50% 50% 0.25em;
		box-shadow: 0 -0.2em 0 hsl(0,90%,75%) inset;
		top: 1.5em;
		right: -0.5em;
		width: 1em;
		height: 0.5em;
		transform: rotate(30deg) translateZ(-1px);
		transform-origin: 0.25em 0.25em;
	}

	.spoke {
		animation: spoke var(--dur) linear infinite;
		background: radial-gradient(100% 100% at center,hsl(0,0%,60%) 4.8%,hsla(0,0%,60%,0) 5%),
			linear-gradient(hsla(0,0%,55%,0) 46.9%,hsl(0,0%,65%) 47% 52.9%,hsla(0,0%,65%,0) 53%) 50% 50% / 99% 99% no-repeat;
	}

	/* Animations */
	@keyframes hamster {
		from, to {
			transform: rotate(4deg) translate(-0.8em,1.85em);
		}

		50% {
			transform: rotate(0) translate(-0.8em,1.85em);
		}
	}

	@keyframes hamsterHead {
		from, 25%, 50%, 75%, to {
			transform: rotate(0);
		}

		12.5%, 37.5%, 62.5%, 87.5% {
			transform: rotate(8deg);
		}
	}

	@keyframes hamsterEye {
		from, 90%, to {
			transform: scaleY(1);
		}

		95% {
			transform: scaleY(0);
		}
	}

	@keyframes hamsterEar {
		from, 25%, 50%, 75%, to {
			transform: rotate(0);
		}

		12.5%, 37.5%, 62.5%, 87.5% {
			transform: rotate(12deg);
		}
	}

	@keyframes hamsterBody {
		from, 25%, 50%, 75%, to {
			transform: rotate(0);
		}

		12.5%, 37.5%, 62.5%, 87.5% {
			transform: rotate(-2deg);
		}
	}

	@keyframes hamsterFRLimb {
		from, 25%, 50%, 75%, to {
			transform: rotate(50deg) translateZ(-1px);
		}

		12.5%, 37.5%, 62.5%, 87.5% {
			transform: rotate(-30deg) translateZ(-1px);
		}
	}

	@keyframes hamsterFLLimb {
		from, 25%, 50%, 75%, to {
			transform: rotate(-30deg);
		}

		12.5%, 37.5%, 62.5%, 87.5% {
			transform: rotate(50deg);
		}
	}

	@keyframes hamsterBRLimb {
		from, 25%, 50%, 75%, to {
			transform: rotate(-60deg) translateZ(-1px);
		}

		12.5%, 37.5%, 62.5%, 87.5% {
			transform: rotate(20deg) translateZ(-1px);
		}
	}

	@keyframes hamsterBLLimb {
		from, 25%, 50%, 75%, to {
			transform: rotate(20deg);
		}

		12.5%, 37.5%, 62.5%, 87.5% {
			transform: rotate(-60deg);
		}
	}

	@keyframes hamsterTail {
		from, 25%, 50%, 75%, to {
			transform: rotate(30deg) translateZ(-1px);
		}

		12.5%, 37.5%, 62.5%, 87.5% {
			transform: rotate(10deg) translateZ(-1px);
		}
	}

	@keyframes spoke {
		from {
			transform: rotate(0);
		}

		to {
			transform: rotate(-1turn);
		}
	}
</style>

<div class="flex items-center justify-center">
	<div class="login-layout">
		<div class="wheel-and-hamster" role="img" aria-label="Loading hamster">
			<div class="wheel"></div>
			<div class="hamster">
				<div class="hamster__body">
					<div class="hamster__head">
						<div class="hamster__ear"></div>
						<div class="hamster__eye"></div>
						<div class="hamster__nose"></div>
					</div>
					<div class="hamster__limb hamster__limb--fr"></div>
					<div class="hamster__limb hamster__limb--fl"></div>
					<div class="hamster__limb hamster__limb--br"></div>
					<div class="hamster__limb hamster__limb--bl"></div>
					<div class="hamster__tail"></div>
				</div>
			</div>
			<div class="spoke"></div>
		</div>

		<div class="login-card-wrapper">
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
							<span>LOGIN WITH GOOGLE</span>
						</a>
					</div>

					<p class="email-restriction">
						Only @iitmandi.ac.in and @students.iitmandi.ac.in emails allowed
					</p>
				</div>
			</Card>
		</div>

		<div class="wheel-and-hamster" role="img" aria-label="Loading hamster">
			<div class="wheel"></div>
			<div class="hamster">
				<div class="hamster__body">
					<div class="hamster__head">
						<div class="hamster__ear"></div>
						<div class="hamster__eye"></div>
						<div class="hamster__nose"></div>
					</div>
					<div class="hamster__limb hamster__limb--fr"></div>
					<div class="hamster__limb hamster__limb--fl"></div>
					<div class="hamster__limb hamster__limb--br"></div>
					<div class="hamster__limb hamster__limb--bl"></div>
					<div class="hamster__tail"></div>
				</div>
			</div>
			<div class="spoke"></div>
		</div>
	</div>
</div>
