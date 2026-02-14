<script lang="ts">
	import { user } from '$lib/auth';
	import { api } from '$lib/api';
	import type { UserRole } from '$lib/types';

	let currentRole: UserRole = 'student';
	let isChanging = false;

	user.subscribe((u) => {
		if (u) currentRole = u.role;
	});

	const roles: { value: UserRole; label: string; color: string }[] = [
		{ value: 'student', label: '🎓 Student', color: 'badge-info' },
		{ value: 'faculty', label: '👨‍🏫 Faculty', color: 'badge-success' },
		{ value: 'authority', label: '⚖️ Authority', color: 'badge-warning' },
		{ value: 'admin', label: '👑 Admin', color: 'badge-error' }
	];

	async function changeRole(newRole: UserRole) {
		if (newRole === currentRole || isChanging) return;

		isChanging = true;
		try {
			await api.put('/user/role', { role: newRole });
			// Refresh user data
			await api.get('/auth/me').then((response) => {
				user.set(response.data);
			});
			window.location.reload(); // Reload to reflect role changes throughout the app
		} catch (error) {
			console.error('Failed to change role:', error);
			alert('Failed to change role. Please try again.');
		} finally {
			isChanging = false;
		}
	}
</script>

<div class="fixed bottom-4 right-4 z-50">
	<div class="dropdown dropdown-top dropdown-end">
		<button
			class="btn btn-sm btn-circle btn-primary shadow-lg"
			title="Dev Mode: Switch Role"
			aria-label="Switch Role"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				class="h-5 w-5"
				fill="none"
				viewBox="0 0 24 24"
				stroke="currentColor"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4"
				/>
			</svg>
		</button>
		<ul
			class="dropdown-content menu p-2 shadow-lg bg-base-100 rounded-box w-52 mb-2 border border-base-300"
		>
			<li class="menu-title">
				<span class="text-xs font-bold">🛠️ DEV MODE: Switch Role</span>
			</li>
			{#each roles as role}
				<li>
					<button
						class="flex items-center justify-between"
						class:active={currentRole === role.value}
						disabled={isChanging || currentRole === role.value}
						onclick={() => changeRole(role.value)}
					>
						<span>{role.label}</span>
						{#if currentRole === role.value}
							<span class="badge {role.color} badge-sm">Active</span>
						{/if}
					</button>
				</li>
			{/each}
			{#if isChanging}
				<li><span class="loading loading-spinner loading-sm"></span></li>
			{/if}
		</ul>
	</div>
</div>

<style>
	.dropdown-content {
		max-height: 300px;
		overflow-y: auto;
	}
</style>
