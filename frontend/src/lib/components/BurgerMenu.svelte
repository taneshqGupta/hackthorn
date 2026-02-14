<script lang="ts">
	import { goto } from '$app/navigation';
	import { logout } from '$lib/api';
	import { user } from '$lib/auth';

	let isOpen = $state(false);
	let currentUser = $derived($user);

	function toggleMenu() {
		isOpen = !isOpen;
	}

	function closeMenu() {
		isOpen = false;
	}

	async function handleLogout() {
		try {
			await logout();
			user.set(null); // Clear the user store
			closeMenu();
			goto('/login');
		} catch (err) {
			console.error('Logout failed:', err);
		}
	}

	function navigate(path: string) {
		goto(path);
		closeMenu();
	}
</script>

<div class="menu-container">
	<button class="burger" onclick={toggleMenu} aria-label="Menu">
		<span class:open={isOpen}></span>
		<span class:open={isOpen}></span>
		<span class:open={isOpen}></span>
	</button>

	{#if isOpen}
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="dropdown" onclick={closeMenu}>
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div class="dropdown-content" onclick={(e) => e.stopPropagation()}>
				{#if currentUser}
					<button class="menu-item" onclick={() => navigate('/profile')}>
						<img
							src={currentUser.avatar ||
								`https://ui-avatars.com/api/?name=${currentUser.name}&background=random&size=32`}
							alt="avatar"
							class="avatar-small"
						/>
						<span>My Profile</span>
					</button>
				{/if}
				<button class="menu-item" onclick={() => navigate('/about')}>About</button>
				<button class="menu-item" onclick={() => navigate('/terms')}>Terms of Service</button>
				<button class="menu-item" onclick={() => navigate('/privacy')}>Privacy Policy</button>
				{#if currentUser}
					<button class="menu-item logout" onclick={handleLogout}>Logout</button>
				{/if}
			</div>
		</div>
	{/if}
</div>

<style>
	.menu-container {
		position: relative;
	}

	.burger {
		position: relative;
		width: 26px;
		height: 20px;
		background: transparent;
		cursor: pointer;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		padding: 0;
		border: none;
		margin-right: 8px;
	}

	.burger span {
		display: block;
		height: 3px;
		width: 100%;
		background: #d06065;
		border-radius: 2px;
		transition: all 0.3s ease;
	}

	.burger span.open:nth-child(1) {
		transform: rotate(45deg) translate(6px, 6px);
	}

	.burger span.open:nth-child(2) {
		opacity: 0;
	}

	.burger span.open:nth-child(3) {
		transform: rotate(-45deg) translate(6px, -6px);
	}

	.dropdown {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.3);
		z-index: 50;
		display: flex;
		justify-content: flex-end;
		align-items: flex-start;
		padding: 60px 16px 16px;
	}

	.dropdown-content {
		background: #fff;
		border: 4px solid #000;
		box-shadow: 8px 8px 0 #d06065;
		padding: 8px;
		min-width: 220px;
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.menu-item {
		padding: 12px 16px;
		background: #ffb3ba;
		border: 3px solid #000;
		color: #000;
		font-weight: 700;
		font-size: 14px;
		text-align: left;
		cursor: pointer;
		transition: all 0.15s;
		display: flex;
		align-items: center;
		gap: 12px;
		text-transform: uppercase;
	}

	.menu-item:hover {
		background: #d06065;
		color: #fff;
		transform: translate(2px, 2px);
	}

	.menu-item:active {
		transform: translate(4px, 4px);
	}

	.menu-item.logout {
		background: #ffa5ab;
		margin-top: 8px;
	}

	.menu-item.logout:hover {
		background: #ff6b7a;
	}

	.avatar-small {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		object-fit: cover;
		border: 2px solid #000;
	}
</style>
