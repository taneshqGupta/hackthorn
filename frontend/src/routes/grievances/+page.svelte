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

			const response = await api.get(`/api/grievances?${params.toString()}`);
		console.log('[GRIEVANCES] Full response:', response);
		console.log('[GRIEVANCES] response.data:', response.data);
		console.log('[GRIEVANCES] Type of response.data:', Array.isArray(response.data) ? 'array' : typeof response.data);
		grievances = response.data || [];
		console.log('[GRIEVANCES] Set grievances to:', grievances);
		console.log('[GRIEVANCES] grievances.length:', grievances.length);
	} catch (err: any) {
		console.error('[GRIEVANCES] Error loading:', err);
		error = err.message || 'Failed to load grievances';
		grievances = [];
	} finally {
		loading = false;
		console.log('[GRIEVANCES] Loading complete. Final state:', { loading, grievances: grievances.length, error });
	}
}

async function loadDepartments() {
		try {
			const response = await api.get('/api/departments');
			departments = response.data || [];
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

<div></div>