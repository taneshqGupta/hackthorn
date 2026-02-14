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

	function formatStatus(status: string): string {
		return status.split('_').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join(' ');
	}

	function formatCategory(category: string): string {
		return category.charAt(0).toUpperCase() + category.slice(1);
	}
</script>

<div class="container">
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
</style>