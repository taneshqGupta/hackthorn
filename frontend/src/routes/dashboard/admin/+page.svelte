<script lang="ts">
	import { user } from "$lib/auth";
	import { goto } from "$app/navigation";
	import { onMount } from "svelte";
	import api from "$lib/api";
	import type { ApiResponse } from "$lib/types";

	let currentUser = $derived($user);

	$effect(() => {
		if (currentUser && currentUser.role !== "admin") {
			goto(`/dashboard/${currentUser.role}`);
		} else if (!currentUser) {
			goto("/login");
		}
	});

	// State for analytics
	let stats = $state<any>(null);
	let logs = $state<any[]>([]);
	let loading = $state(true);

	onMount(async () => {
		try {
			// Hits get_system_stats and get_audit_logs in admin.rs
			const [statsRes, logsRes] = await Promise.all([
				api.get<ApiResponse<any>>("/api/admin/stats"),
				api.get<ApiResponse<any[]>>("/api/admin/audit-logs?limit=5"),
			]);
			stats = statsRes.data;
			logs = logsRes.data || [];
		} catch (e) {
			console.error("Dashboard load failed", e);
		} finally {
			loading = false;
		}
	});
</script>

<div class="dashboard-container">
	<nav
		class="flex gap-4 mb-4 border-b-2 border-black/10 pb-2 w-full justify-center"
	>
		<button onclick={() => goto("/dashboard/admin")} class="nav-link"
			>PANEL</button
		>
		<button onclick={() => goto("/admin/users")} class="nav-link"
			>ROSTER</button
		>
		<button onclick={() => goto("/admin/logs")} class="nav-link"
			>TRAIL</button
		>
	</nav>
	<h1
		class="text-8xl font-bold mb-6 text-[#2b0b0b] tracking-tighter uppercase text-center w-full"
	>
		Admin
	</h1>

	{#if loading}
		<div class="loading text-center uppercase tracking-widest opacity-50">
			Syncing Citadel...
		</div>
	{:else}
		<div class="grid grid-cols-2 gap-2 mb-6 w-full">
			<div class="stat-box">
				<span class="label">ACTIVE</span>
				<span class="value">{stats?.active_users ?? 0}</span>
			</div>
			<div class="stat-box">
				<span class="label">TOTAL</span>
				<span class="value">{stats?.total_grievances ?? 0}</span>
			</div>
			<div class="stat-box alert">
				<span class="label">PENDING</span>
				<span class="value">{stats?.pending_grievances ?? 0}</span>
			</div>
			<div class="stat-box success">
				<span class="label">RESOLVED</span>
				<span class="value">{stats?.resolved_grievances ?? 0}</span>
			</div>
		</div>

		<div class="flex flex-col gap-6 w-full">
			<div class="brutalist-card">
				<h3
					class="mb-4 text-xl border-b-2 border-[#2b0b0b] uppercase font-black"
				>
					Command
				</h3>
				<div class="flex flex-col gap-2">
					<button
						onclick={() => goto("/admin/users")}
						class="retro-btn"
					>
						USER ROSTERS
					</button>
					<button
						onclick={() => goto("/grievances")}
						class="retro-btn"
					>
						GRIEVANCE FEED
					</button>
				</div>
			</div>

			<div class="brutalist-card">
				<h3
					class="mb-4 text-xl border-b-2 border-[#2b0b0b] uppercase font-black"
				>
					Audit Trail
				</h3>
				<div class="log-list">
					{#each logs as log}
						<div class="log-item">
							<span class="action"
								>{log.action.replace("_", " ")}</span
							>
							<span class="user"
								>BY {log.user?.first_name ?? "SYSTEM"}</span
							>
						</div>
					{/each}
				</div>
				<button
					onclick={() => goto("/admin/logs")}
					class="mt-4 text-[10px] underline uppercase font-bold tracking-tighter hover:text-[#b31b34]"
				>
					Open Full Logs →
				</button>
			</div>
		</div>
	{/if}
</div>

<style>
	.dashboard-container {
		width: 400px; /* Locked width to match search/filters */
		margin: 0 auto;
		padding: 2rem 1rem;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.stat-box {
		border: 2px solid rgba(198, 225, 237, 0.6);
		padding: 0.75rem;
		display: flex;
		flex-direction: column;
		background: transparent;
		box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.1);
	}

	.stat-box .label {
		font-size: 10px;
		color: #666;
		font-weight: 900;
		letter-spacing: 1px;
	}

	.stat-box .value {
		font-size: 2.5rem;
		font-family: "Jersey 25", sans-serif;
		line-height: 1;
		color: #2b0b0b;
	}

	.stat-box.alert .value {
		color: #b31b34;
	}
	.stat-box.success .value {
		color: #059669;
	}

	.brutalist-card {
		border: 2px solid rgba(198, 225, 237, 0.6);
		padding: 1.25rem;
		box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.1);
		background: transparent;
	}

	.retro-btn {
		font-family: inherit;
		background: #b31b34;
		color: white;
		padding: 0.75rem;
		border: 2px solid #2b0b0b;
		font-weight: bold;
		font-size: 16px; /* Matched to filter font size */
		cursor: pointer;
		box-shadow: 4px 4px 0px #000;
		text-transform: uppercase;
		transition: 0.2s ease;
	}

	.retro-btn:hover {
		transform: translate(-2px, -2px);
		box-shadow: 6px 6px 0px #000;
	}

	.log-list {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.log-item {
		display: flex;
		justify-content: space-between;
		font-size: 11px;
		padding: 4px 0;
		border-bottom: 1px dashed rgba(198, 225, 237, 0.6);
		text-transform: uppercase;
	}

	.log-item .action {
		font-weight: 900;
		color: #b31b34;
	}

	.log-item .user {
		color: #666;
	}

	.loading {
		font-family: "Jersey 25", sans-serif;
		font-size: 1.5rem;
		color: #2b0b0b;
		margin-top: 3rem;
	}
</style>
