<script lang="ts">
	import { goto } from "$app/navigation";
	import { user } from "$lib/auth";
	import { logout } from "$lib/api";
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
			user.set(null);
			goto("/login");
		} catch (error) {
			console.error("Logout failed:", error);
		}
	}

	function navigate(path: string) {
		closeMenu();
		goto(path);
	}
</script>

<div class="menu-wrapper">
	<button
		class="burger"
		class:active={isOpen}
		onclick={toggleMenu}
		aria-label="Menu"
	>
		<span></span>
		<span></span>
		<span></span>
	</button>

	{#if isOpen}
		<div class="backdrop" onclick={closeMenu}></div>

		<div class="dropdown">
			{#if currentUser}
				<button
					class="menu-item profile-item"
					onclick={() => navigate("/dashboard/student/profile")}
				>
					<img
						src={currentUser.profile_picture ||
							`https://ui-avatars.com/api/?name=${currentUser.first_name}&background=random&size=32`}
						alt="avatar"
						class="avatar"
					/>
					<div class="user-info">
						<span class="name">Profile</span>
						<span class="email"
							>{currentUser.email || "My Account"}</span
						>
					</div>
				</button>
			{/if}

			<button class="menu-item" onclick={() => navigate("/terms")}>
				Terms of Service
			</button>

			<button class="menu-item" onclick={() => navigate("/privacy")}>
				Privacy Policy
			</button>

			{#if currentUser}
				<div class="divider"></div>
				<button class="menu-item logout" onclick={handleLogout}>
					Logout
				</button>
			{/if}
		</div>
	{/if}
</div>

<style>
	/* Container keeps the menu anchored to the button */
	.menu-wrapper {
		position: relative;
		display: inline-block;
		z-index: 50;
	}

	/* BURGER BUTTON STYLES */
	.burger {
		width: 24px;
		height: 16px;
		background: transparent;
		border: none;
		cursor: pointer;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		padding: 0;
		margin-right: 1rem;
		z-index: 52; /* Above backdrop */
		position: relative;
	}

	.burger span {
		display: block;
		width: 100%;
		height: 3px;
		background: #000;
		transition: all 0.3s ease;
	}

	/* Animate into X */
	.burger.active span:nth-child(1) {
		transform: rotate(45deg) translate(5px, 6px);
	}
	.burger.active span:nth-child(2) {
		opacity: 0;
	}
	.burger.active span:nth-child(3) {
		transform: rotate(-45deg) translate(5px, -6px);
	}

	/* BACKDROP */
	/* Invisible layer that covers screen to detect outside clicks */
	.backdrop {
		position: fixed;
		inset: 0;
		background: transparent;
		z-index: 50;
		cursor: default;
	}

	/* DROPDOWN CONTAINER */
	.dropdown {
		position: absolute;
		top: 130%; /* Pushes it slightly below the header */
		right: 1rem;
		width: 240px;
		background: #fff;
		border: 2px solid #000;
		/* The hard shadow (Neobrutalist style) */
		box-shadow: 6px 6px 0 #d06065;
		padding: 0.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		z-index: 51; /* Above backdrop, below burger */
	}

	/* MENU ITEMS */
	.menu-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
		padding: 0.75rem;
		border: none;
		background: transparent;
		color: #000;
		font-family: "Oswald", sans-serif; /* Matching your theme */
		font-weight: 600;
		font-size: 1rem;
		text-transform: uppercase;
		cursor: pointer;
		text-align: left;
		transition:
			background 0.2s,
			transform 0.1s;
	}

	.menu-item:hover {
		background: #ffb3ba; /* Light pink hover */
		transform: translateX(2px);
	}

	/* Profile Specifics */
	.profile-item {
		background: #f4f4f5;
		border: 2px solid #000;
		margin-bottom: 0.25rem;
	}

	.profile-item:hover {
		background: #e4e4e7;
	}

	.user-info {
		display: flex;
		flex-direction: column;
		line-height: 1.1;
	}

	.user-info .name {
		font-size: 0.9rem;
	}

	.user-info .email {
		font-size: 0.7rem;
		opacity: 0.6;
		text-transform: none;
	}

	.avatar {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		border: 2px solid #000;
	}

	/* Divider Line */
	.divider {
		height: 2px;
		background: #000;
		margin: 0.25rem 0;
		opacity: 0.1;
	}

	/* Logout Button */
	.logout {
		color: #d06065;
	}

	.logout:hover {
		background: #d06065;
		color: #fff;
	}
</style>
