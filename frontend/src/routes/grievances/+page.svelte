<script lang="ts">
	import { user } from '$lib/auth';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import api from '$lib/api';
	import type { Grievance, Department } from '$lib/types';

	let currentUser = $derived($user);
	let grievances: Grievance[] = $state([]);
	let departments: Department[] = $state([]);
	let loading = $state(true);
	let error = $state('');

	// Filters
	let selectedCategory = $state('all');
	let selectedStatus = $state('all');
	let selectedPriority = $state('all');
	let searchQuery = $state('');
	let showOnlyMine = $state(false);

	const categories = ['infrastructure', 'academics', 'hostel', 'food', 'other'];
	const statuses = ['submitted', 'under_review', 'in_progress', 'resolved', 'rejected'];
	const priorities = ['low', 'medium', 'high', 'urgent'];

	onMount(async () => {
		if (!$user) {
			goto('/login');
			return;
		}
		await loadGrievances();
		await loadDepartments();
	});

	async function loadGrievances() {
		loading = true;
		error = '';
		try {
			const params = new URLSearchParams();
			if (selectedCategory !== 'all') params.append('category', selectedCategory);
			if (selectedStatus !== 'all') params.append('status', selectedStatus);
			if (selectedPriority !== 'all') params.append('priority', selectedPriority);
			if (searchQuery) params.append('search', searchQuery);
			if (showOnlyMine) params.append('submitter_id', currentUser?.id || '');

			const response = await api.get(`/grievances?${params.toString()}`);
			grievances = response || [];
		} catch (err: any) {
			error = err.message || 'Failed to load grievances';
			grievances = [];
		} finally {
			loading = false;
		}
	}

	async function loadDepartments() {
		try {
			departments = await api.get('/grievances/departments');
		} catch (err) {
			console.error('Failed to load departments:', err);
		}
	}

	function handleFilterChange() {
		loadGrievances();
	}

	function getStatusColor(status: string): string {
		const colors: Record<string, string> = {
			submitted: 'badge-info',
			under_review: 'badge-warning',
			in_progress: 'badge-primary',
			resolved: 'badge-success',
			rejected: 'badge-error'
		};
		return colors[status] || 'badge-ghost';
	}

	function getPriorityColor(priority: string): string {
		const colors: Record<string, string> = {
			low: 'text-gray-500',
			medium: 'text-blue-500',
			high: 'text-orange-500',
			urgent: 'text-red-500'
		};
		return colors[priority] || 'text-gray-500';
	}

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' });
	}

	function formatCategory(category: string): string {
		return category.charAt(0).toUpperCase() + category.slice(1);
	}

	function formatStatus(status: string): string {
		return status.split('_').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join(' ');
	}

	$effect(() => {
		if (currentUser) {
			handleFilterChange();
		}
	});
</script>

<div class="min-h-screen bg-base-200">
	<!-- Header -->
	<div class="bg-base-100 shadow-md">
		<div class="container mx-auto px-4 py-6">
			<div class="flex justify-between items-center">
				<div>
					<h1 class="text-3xl font-bold">Grievance System</h1>
					<p class="text-gray-600 mt-1">Transparent issue tracking and resolution</p>
				</div>
				<button onclick={() => goto('/grievances/submit')} class="btn btn-primary">
					<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
						<path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
					</svg>
					Submit Grievance
				</button>
			</div>
		</div>
	</div>

	<div class="container mx-auto px-4 py-8">
		<!-- Filters -->
		<div class="card bg-base-100 shadow-xl mb-6">
			<div class="card-body">
				<h2 class="card-title mb-4">Filter & Search</h2>
				
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-4">
					<!-- Category Filter -->
					<div class="form-control">
						<label class="label"><span class="label-text">Category</span></label>
						<select bind:value={selectedCategory} onchange={handleFilterChange} class="select select-bordered">
							<option value="all">All Categories</option>
							{#each categories as category}
								<option value={category}>{formatCategory(category)}</option>
							{/each}
						</select>
					</div>

					<!-- Status Filter -->
					<div class="form-control">
						<label class="label"><span class="label-text">Status</span></label>
						<select bind:value={selectedStatus} onchange={handleFilterChange} class="select select-bordered">
							<option value="all">All Statuses</option>
							{#each statuses as status}
								<option value={status}>{formatStatus(status)}</option>
							{/each}
						</select>
					</div>

					<!-- Priority Filter -->
					<div class="form-control">
						<label class="label"><span class="label-text">Priority</span></label>
						<select bind:value={selectedPriority} onchange={handleFilterChange} class="select select-bordered">
							<option value="all">All Priorities</option>
							{#each priorities as priority}
								<option value={priority}>{formatCategory(priority)}</option>
							{/each}
						</select>
					</div>

					<!-- Search -->
					<div class="form-control">
						<label class="label"><span class="label-text">Search</span></label>
						<input 
							type="text" 
							bind:value={searchQuery} 
							onchange={handleFilterChange}
							placeholder="Search grievances..." 
							class="input input-bordered" 
						/>
					</div>
				</div>

				<div class="flex items-center gap-4">
					<label class="label cursor-pointer gap-2">
						<input type="checkbox" bind:checked={showOnlyMine} onchange={handleFilterChange} class="checkbox checkbox-primary" />
						<span class="label-text">Show only my grievances</span>
					</label>
					
					<button onclick={handleFilterChange} class="btn btn-sm btn-ghost">
						<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" viewBox="0 0 20 20" fill="currentColor">
							<path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd" />
						</svg>
						Refresh
					</button>
				</div>
			</div>
		</div>

		<!-- Loading State -->
		{#if loading}
			<div class="flex justify-center items-center py-12">
				<span class="loading loading-spinner loading-lg"></span>
			</div>
		{:else if error}
			<!-- Error State -->
			<div class="alert alert-error">
				<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
				</svg>
				<span>{error}</span>
			</div>
		{:else if grievances.length === 0}
			<!-- Empty State -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body items-center text-center py-12">
					<svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 text-gray-400 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
					</svg>
					<h3 class="text-xl font-bold mb-2">No Grievances Found</h3>
					<p class="text-gray-600 mb-4">Be the first to submit a grievance or adjust your filters</p>
					<button onclick={() => goto('/grievances/submit')} class="btn btn-primary">Submit Grievance</button>
				</div>
			</div>
		{:else}
			<!-- Grievances List -->
			<div class="space-y-4">
				{#each grievances as grievance}
					<button class="card bg-base-100 shadow-xl hover:shadow-2xl transition-shadow cursor-pointer text-left w-full" onclick={() => goto(`/grievances/${grievance.id}`)}>
						<div class="card-body">
							<div class="flex justify-between items-start">
								<div class="flex-1">
									<div class="flex items-center gap-2 mb-2">
										<h3 class="card-title">{grievance.title}</h3>
										<div class={`badge ${getStatusColor(grievance.status)}`}>
											{formatStatus(grievance.status)}
										</div>
									</div>
									<p class="text-gray-600 mb-3 line-clamp-2">{grievance.description}</p>
									
									<div class="flex flex-wrap gap-2 text-sm">
										<div class="badge badge-outline">{formatCategory(grievance.category)}</div>
										<div class={`badge badge-outline ${getPriorityColor(grievance.priority)}`}>
											{formatCategory(grievance.priority)} Priority
										</div>
										{#if grievance.location}
											<div class="badge badge-ghost">
												<svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 mr-1" viewBox="0 0 20 20" fill="currentColor">
													<path fill-rule="evenodd" d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z" clip-rule="evenodd" />
												</svg>
												{grievance.location}
											</div>
										{/if}
										{#if grievance.is_anonymous}
											<div class="badge badge-ghost">Anonymous</div>
										{/if}
									</div>
								</div>

								<div class="text-right">
									<div class="flex items-center gap-2 text-sm text-gray-500">
										<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
											<path d="M2 10.5a1.5 1.5 0 113 0v6a1.5 1.5 0 01-3 0v-6zM6 10.333v5.43a2 2 0 001.106 1.79l.05.025A4 4 0 008.943 18h5.416a2 2 0 001.962-1.608l1.2-6A2 2 0 0015.56 8H12V4a2 2 0 00-2-2 1 1 0 00-1 1v.667a4 4 0 01-.8 2.4L6.8 7.933a4 4 0 00-.8 2.4z" />
										</svg>
										<span>{grievance.upvotes_count || 0}</span>
									</div>
									<div class="text-xs text-gray-500 mt-1">{formatDate(grievance.created_at)}</div>
								</div>
							</div>
						</div>
					</button>
				{/each}
			</div>
		{/if}
	</div>
</div>
