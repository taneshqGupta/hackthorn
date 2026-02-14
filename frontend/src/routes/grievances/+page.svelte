<script lang="ts">
	import { user } from '$lib/auth';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import api from '$lib/api';
	import type { Grievance } from '$lib/types';
	import PostCard from '$lib/components/PostCard.svelte';

	let grievances: Grievance[] = $state([]);
	let loading = $state(true);

	onMount(async () => {
		if (!$user) {
			goto('/login');
			return;
		}
		await loadGrievances();
	});

	async function loadGrievances() {
		loading = true;
		try {
			const response = await api.get('/api/grievances?');
			grievances = response.data || [];
		} catch (err: any) {
			console.error('[GRIEVANCES] Error loading:', err);
			grievances = [];
		} finally {
			loading = false;
		}
	}

	async function handleUpvote(grievanceId: string) {
		try {
			await api.post(`/api/grievances/${grievanceId}/upvote`);
			// Update local count
			grievances = grievances.map(g => 
				g.id === grievanceId ? { ...g, upvotes_count: g.upvotes_count + 1 } : g
			);
		} catch (err: any) {
			console.error('[GRIEVANCES] Upvote failed:', err);
		}
	}

	function formatStatus(status: string): string {
		return status.split('_').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join(' ');
	}

	function formatCategory(category: string): string {
		return category.charAt(0).toUpperCase() + category.slice(1);
	}
</script>

<div class="container">
	<button class="submit-btn" onclick={() => goto('/grievances/submit')} aria-label="Submit new grievance">
		<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2">
			<path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
		</svg>
	</button>

	{#if loading}
		<div class="loading">Loading...</div>
	{:else}
		{#each grievances as grievance}
		<PostCard
			id={grievance.id}
			username={grievance.is_anonymous ? 'Anonymous' : (grievance.submitter_name || 'Unknown')}
			title={grievance.title}
			content={grievance.description}
			images={grievance.photo_urls || []}
			upvotes={grievance.upvotes_count || 0}
			commentsCount={0}
			date={grievance.created_at}
			status={formatStatus(grievance.status)}
			category={formatCategory(grievance.category)}
			onclick={() => goto(`/grievances/${grievance.id}`)}
			onupvote={() => handleUpvote(grievance.id)}
		/>
		{/each}
	{/if}
</div>

<style>
	.container {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1.5rem;
		padding: 2rem 1rem;
		min-height: 100vh;
	}

	.loading {
		color: #c0c3d7;
		font-size: 1.2rem;
		margin-top: 3rem;
	}

	.submit-btn {
		position: fixed;
		bottom: 2rem;
		right: 2rem;
		width: 60px;
		height: 60px;
		border-radius: 0;
		background-color: rgba(179, 27, 52, 0.9);
		border: 2px solid rgba(198, 225, 237, 0.6);
		box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.5);
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: 200ms ease-in-out;
		z-index: 100;
	}

	.submit-btn:hover {
		transform: translate(-2px, -2px);
		box-shadow: 6px 6px 0px rgba(0, 0, 0, 0.7);
		background-color: rgba(179, 27, 52, 1);
	}

	.submit-btn svg {
		width: 28px;
		height: 28px;
		stroke: #fff;
	}
</style>