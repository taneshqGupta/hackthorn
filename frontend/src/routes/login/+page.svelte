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

<div class="h-screen flex items-center justify-center">
	<Card>
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

		<button class="btn bg-white text-black border-[#e5e5e5]">
			<svg
				aria-label="Google logo"
				width="16"
				height="16"
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 512 512"
				><g
					><path d="m0 0H512V512H0" fill="#fff"></path><path
						fill="#34a853"
						d="M153 292c30 82 118 95 171 60h62v48A192 192 0 0190 341"
					></path><path
						fill="#4285f4"
						d="m386 400a140 175 0 0053-179H260v74h102q-7 37-38 57"
					></path><path
						fill="#fbbc02"
						d="m90 341a208 200 0 010-171l63 49q-12 37 0 73"
					></path><path
						fill="#ea4335"
						d="m153 219c22-69 116-109 179-50l55-54c-78-75-230-72-297 55"
					></path></g
				></svg
			>
			Login with Google
		</button>
		<p class="text-center mt-2">
			Only @iitmandi.ac.in and @students.iitmandi.ac.in emails allowed
		</p>
	</Card>
</div>
