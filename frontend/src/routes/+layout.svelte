<script lang="ts">
	import '../app.css';
	import Logo from '$lib/components/Logo.svelte';
	import BurgerMenu from '$lib/components/BurgerMenu.svelte';
	import { user } from '$lib/auth';
	import type { User } from '$lib/types';
	import { goto } from '$app/navigation';

	let { children } = $props();
	
	// Use $derived for reactive values from stores in Svelte 5
	let currentUser = $derived($user);
</script>

<div class="h-screen flex flex-col overflow-hidden container">
	<header
		class="flex-none w-full border-b-2 flex items-center justify-between px-4"
		style="border-color: #d06065; height: 40px;"
	>
		<Logo />
		<div class="flex items-center">
			{#if currentUser}
				<div class="mr-4">
					<button on:click={() => goto('/profile')} class="btn btn-ghost btn-circle">
						<div class="avatar">
							<div class="w-10 rounded-full">
								<img
									src={currentUser.avatar ||
										`https://ui-avatars.com/api/?name=${currentUser.name}&background=random`}
									alt="user avatar"
								/>
							</div>
						</div>
					</button>
				</div>
			{/if}
			<BurgerMenu isOpen={false} ontoggle={() => {}} />
		</div>
	</header>

	<main class="flex-1 overflow-auto">
		{@render children()}
	</main>

	<footer
		class="flex-none w-full border-t-2 flex items-center justify-center"
		style="border-color: #d06065; background-color: rgba(255, 179, 186, 0.2); height: 40px;"
	>
	</footer>
</div>