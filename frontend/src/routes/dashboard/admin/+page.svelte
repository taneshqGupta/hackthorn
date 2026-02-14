<script lang="ts">
	import { user } from "$lib/auth";
	import { goto } from "$app/navigation";

	let currentUser = $derived($user);

	$effect(() => {
		if (currentUser && currentUser.role !== "admin") {
			goto(`/dashboard/${currentUser.role}`);
		} else if (!currentUser) {
			goto("/login");
		}
	});

	import { onMount } from "svelte";
	import api from "$lib/api";
	import type { ApiResponse } from "$lib/types";

	// State for analytics
	let stats = $state<any>(null);
	let logs = $state<any[]>([]);
	let loading = $state(true);

	onMount(async () => {
		try {
			// Parallel fetch for stats and logs
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
	<h1 class="text-6xl font-bold mb-8 text-[#2b0b0b] tracking-tighter">
		ADMIN PANEL
	</h1>

	{#if loading}
		<div class="loading">SYNCING DATA...</div>
	{:else}
		<div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
			<div class="stat-box">
				<span class="label">ACTIVE USERS</span>
				<span class="value">{stats?.active_users ?? 0}</span>
			</div>
			<div class="stat-box">
				<span class="label">TOTAL ISSUES</span>
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

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
			<div class="brutalist-card">
				<h3 class="mb-4 text-2xl border-b-2 border-black">
					CORE MANAGEMENT
				</h3>
				<div class="flex flex-col gap-4">
					<button
						onclick={() => goto("/admin/users")}
						class="retro-btn"
					>
						MANAGE USER ROSTERS
					</button>
					<button
						onclick={() => goto("/grievances")}
						class="retro-btn"
					>
						COMMAND GRIEVANCE FEED
					</button>
				</div>
			</div>

			<div class="brutalist-card">
				<h3 class="mb-4 text-2xl border-b-2 border-black">
					RECENT AUDIT LOGS
				</h3>
				<div class="log-list">
					{#each logs as log}
						<div class="log-item">
							<span class="time"
								>{new Date(
									log.created_at,
								).toLocaleTimeString()}</span
							>
							<span class="action">{log.action}</span>
							<span class="user"
								>by {log.user?.first_name ?? "SYSTEM"}</span
							>
						</div>
					{/each}
				</div>
				<button
					onclick={() => goto("/admin/logs")}
					class="mt-4 text-xs underline">VIEW ALL LOGS</button
				>
			</div>
		</div>
	{/if}
</div>

<style>
	.dashboard-container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 2rem;
	}
	.stat-box {
		border: 2px solid #2b0b0b;
		padding: 1rem;
		display: flex;
		flex-direction: column;
		background: transparent;
		box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.1);
	}
	.stat-box .label {
		font-size: 0.75rem;
		color: #666;
		font-weight: bold;
	}
	.stat-box .value {
		font-size: 2.5rem;
		font-family: "Jersey 25", sans-serif;
		line-height: 1;
	}
	.stat-box.alert .value {
		color: #b31b34;
	}
	.stat-box.success .value {
		color: #059669;
	}

	.brutalist-card {
		border: 2px solid #2b0b0b;
		padding: 1.5rem;
		box-shadow: 6px 6px 0px rgba(0, 0, 0, 0.1);
	}

	.retro-btn {
		background: #b31b34;
		color: white;
		padding: 1rem;
		border: 2px solid #2b0b0b;
		font-weight: bold;
		cursor: pointer;
		box-shadow: 4px 4px 0px #000;
	}
	.retro-btn:hover {
		transform: translate(-2px, -2px);
		box-shadow: 6px 6px 0px #000;
	}

	.log-item {
		display: flex;
		gap: 10px;
		font-size: 0.8rem;
		padding: 4px 0;
		border-bottom: 1px dashed #ccc;
	}
	.log-item .time {
		color: #888;
	}
	.log-item .action {
		font-weight: bold;
		color: #b31b34;
	}
</style>
