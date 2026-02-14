<!-- DaisyUI uses label elements for styling, not form association -->
<!-- svelte-ignore a11y_label_has_associated_control -->
<script lang="ts">
	import { user } from '$lib/auth';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import api from '$lib/api';
	import type { Grievance, GrievanceStatusHistory, GrievanceComment, Department } from '$lib/types';

	let currentUser = $derived($user);
	let grievanceId = $derived($page.params.id);
	let grievance: Grievance | null = $state(null);
	let history: GrievanceStatusHistory[] = $state([]);
	let comments: GrievanceComment[] = $state([]);
	let departments: Department[] = $state([]);
	let loading = $state(true);
	let error = $state('');
	let hasUpvoted = $state(false);
	let upvotesCount = $state(0);

	// Comment form
	let newComment = $state('');
	let commentLoading = $state(false);

	// Admin actions
	let showUpdateModal = $state(false);
	let showAssignModal = $state(false);
	let showResolveModal = $state(false);
	let newStatus = $state('');
	let adminRemarks = $state('');
	let assignedUserId = $state('');
	let assignedDepartmentId = $state('');
	let resolutionSummary = $state('');

	const isAdmin = $derived(currentUser?.role === 'admin' || currentUser?.role === 'authority');
	const canManage = $derived(() => {
		if (!grievance) return false;
		return isAdmin || grievance.assigned_to === currentUser?.id;
	});

	onMount(async () => {
		if (!$user) {
			goto('/login');
			return;
		}
		await loadGrievance();
		await loadHistory();
		await loadComments();
		await loadDepartments();
	});

	async function loadGrievance() {
		loading = true;
		error = '';
		try {
			grievance = await api.get(`/grievances/${grievanceId}`);
			upvotesCount = grievance?.upvotes_count || 0;
			// Check if current user has upvoted (would need another endpoint or include in response)
		} catch (err: any) {
			error = err.message || 'Failed to load grievance';
		} finally {
			loading = false;
		}
	}

	async function loadHistory() {
		try {
			history = await api.get(`/grievances/${grievanceId}/history`);
		} catch (err) {
			console.error('Failed to load history:', err);
		}
	}

	async function loadComments() {
		try {
			comments = await api.get(`/grievances/${grievanceId}/comments`);
		} catch (err) {
			console.error('Failed to load comments:', err);
		}
	}

	async function loadDepartments() {
		try {
			departments = await api.get('/grievances/departments');
		} catch (err) {
			console.error('Failed to load departments:', err);
		}
	}

	async function toggleUpvote() {
		try {
			await api.post(`/grievances/${grievanceId}/upvote`, {});
			hasUpvoted = !hasUpvoted;
			upvotesCount += hasUpvoted ? 1 : -1;
		} catch (err: any) {
			error = err.message || 'Failed to toggle upvote';
			setTimeout(() => error = '', 3000);
		}
	}

	async function submitComment() {
		if (!newComment.trim()) return;

		commentLoading = true;
		try {
			await api.post(`/grievances/${grievanceId}/comments`, {
				comment: newComment.trim()
			});
			newComment = '';
			await loadComments();
		} catch (err: any) {
			error = err.message || 'Failed to post comment';
		} finally {
			commentLoading = false;
		}
	}

	async function updateStatus() {
		if (!newStatus) return;

		try {
			await api.put(`/grievances/${grievanceId}/status`, {
				status: newStatus,
				remarks: adminRemarks.trim() || null
			});
			showUpdateModal = false;
			newStatus = '';
			adminRemarks = '';
			await loadGrievance();
			await loadHistory();
		} catch (err: any) {
			error = err.message || 'Failed to update status';
		}
	}

	async function assignGrievance() {
		try {
			await api.put(`/grievances/${grievanceId}/assign`, {
				assigned_to: assignedUserId || null,
				department_id: assignedDepartmentId || null
			});
			showAssignModal = false;
			assignedUserId = '';
			assignedDepartmentId = '';
			await loadGrievance();
			await loadHistory();
		} catch (err: any) {
			error = err.message || 'Failed to assign grievance';
		}
	}

	async function resolveGrievance() {
		if (!resolutionSummary.trim()) {
			error = 'Resolution summary is required';
			return;
		}

		try {
			await api.put(`/grievances/${grievanceId}/resolve`, {
				resolution_summary: resolutionSummary.trim()
			});
			showResolveModal = false;
			resolutionSummary = '';
			await loadGrievance();
			await loadHistory();
		} catch (err: any) {
			error = err.message || 'Failed to resolve grievance';
		}
	}

	async function deleteGrievance() {
		if (!confirm('Are you sure you want to delete this grievance? This action cannot be undone.')) {
			return;
		}

		try {
			await api.delete(`/grievances/${grievanceId}`);
			goto('/grievances');
		} catch (err: any) {
			error = err.message || 'Failed to delete grievance';
		}
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
		return date.toLocaleString('en-US', { 
			year: 'numeric', 
			month: 'short', 
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatCategory(category: string): string {
		return category.charAt(0).toUpperCase() + category.slice(1);
	}

	function formatStatus(status: string): string {
		return status.split('_').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join(' ');
	}
</script>

<div class="min-h-screen bg-base-200">
	<!-- Header -->
	<div class="bg-base-100 shadow-md">
		<div class="container mx-auto px-4 py-6">
			<div class="flex items-center gap-4">
				<button onclick={() => goto('/grievances')} class="btn btn-ghost btn-circle" aria-label="Go back to grievances list">
					<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
					</svg>
				</button>
				<div class="flex-1">
					<h1 class="text-3xl font-bold">Grievance Details</h1>
					<p class="text-gray-600 mt-1">View status, timeline, and updates</p>
				</div>
				{#if isAdmin}
					<button onclick={() => showUpdateModal = true} class="btn btn-primary btn-sm">
						Update Status
					</button>
					<button onclick={() => showAssignModal = true} class="btn btn-secondary btn-sm">
						Assign
					</button>
					{#if grievance?.status !== 'resolved'}
						<button onclick={() => showResolveModal = true} class="btn btn-success btn-sm">
							Mark Resolved
						</button>
					{/if}
				{/if}
			</div>
		</div>
	</div>

	<div class="container mx-auto px-4 py-8 max-w-6xl">
		{#if error}
			<div class="alert alert-error mb-6">
				<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
				</svg>
				<span>{error}</span>
			</div>
		{/if}

		{#if loading}
			<div class="flex justify-center items-center py-12">
				<span class="loading loading-spinner loading-lg"></span>
			</div>
		{:else if !grievance}
			<div class="alert alert-error">
				<span>Grievance not found</span>
			</div>
		{:else}
			<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
				<!-- Main Content -->
				<div class="lg:col-span-2 space-y-6">
					<!-- Grievance Card -->
					<div class="card bg-base-100 shadow-xl">
						<div class="card-body">
							<div class="flex justify-between items-start mb-4">
								<h2 class="card-title text-2xl flex-1">{grievance.title}</h2>
								<div class={`badge badge-lg ${getStatusColor(grievance.status)}`}>
									{formatStatus(grievance.status)}
								</div>
							</div>

							<div class="flex flex-wrap gap-2 mb-4">
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

							<div class="prose max-w-none">
								<p class="text-gray-700 whitespace-pre-wrap">{grievance.description}</p>
							</div>

							{#if grievance.photo_urls && grievance.photo_urls.length > 0}
								<div class="mt-4">
									<h3 class="font-semibold mb-2">Photos:</h3>
									<div class="grid grid-cols-2 md:grid-cols-3 gap-4">
										{#each grievance.photo_urls as photoUrl}
											<a href={photoUrl} target="_blank" class="group">
												<img 
													src={photoUrl} 
													alt="Grievance evidence" 
													class="w-full h-48 object-cover rounded-lg group-hover:opacity-80 transition-opacity"
												/>
											</a>
										{/each}
									</div>
								</div>
							{/if}

							{#if grievance.resolution_summary}
								<div class="alert alert-success mt-4">
									<div>
										<h3 class="font-semibold">Resolution Summary</h3>
										<p class="text-sm mt-1">{grievance.resolution_summary}</p>
									</div>
								</div>
							{/if}

							<div class="divider"></div>

							<div class="flex justify-between items-center">
								<div class="text-sm text-gray-500">
									Submitted {formatDate(grievance.created_at)}
									{#if !grievance.is_anonymous && grievance.submitter_name}
										by {grievance.submitter_name}
									{/if}
								</div>
								<button 
									onclick={toggleUpvote}
									class={`btn btn-sm gap-2 ${hasUpvoted ? 'btn-primary' : 'btn-ghost'}`}
								>
									<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
										<path d="M2 10.5a1.5 1.5 0 113 0v6a1.5 1.5 0 01-3 0v-6zM6 10.333v5.43a2 2 0 001.106 1.79l.05.025A4 4 0 008.943 18h5.416a2 2 0 001.962-1.608l1.2-6A2 2 0 0015.56 8H12V4a2 2 0 00-2-2 1 1 0 00-1 1v.667a4 4 0 01-.8 2.4L6.8 7.933a4 4 0 00-.8 2.4z" />
									</svg>
									{upvotesCount}
								</button>
							</div>
						</div>
					</div>

					<!-- Timeline -->
					<div class="card bg-base-100 shadow-xl">
						<div class="card-body">
							<h2 class="card-title mb-4">Timeline</h2>
							
							{#if history.length === 0}
								<p class="text-gray-500 text-center py-4">No status updates yet</p>
							{:else}
								<ul class="timeline timeline-vertical">
									{#each history as item, index}
										<li>
											{#if index > 0}
												<hr />
											{/if}
											<div class="timeline-start">{formatDate(item.changed_at)}</div>
											<div class="timeline-middle">
												<div class={`badge ${getStatusColor(item.status)}`}></div>
											</div>
											<div class="timeline-end timeline-box">
												<div class="font-semibold">{formatStatus(item.status)}</div>
												{#if item.changed_by_name}
													<p class="text-sm text-gray-600">by {item.changed_by_name}</p>
												{/if}
												{#if item.remarks}
													<p class="text-sm mt-2">{item.remarks}</p>
												{/if}
											</div>
											{#if index < history.length - 1}
												<hr />
											{/if}
										</li>
									{/each}
								</ul>
							{/if}
						</div>
					</div>

					<!-- Comments -->
					<div class="card bg-base-100 shadow-xl">
						<div class="card-body">
							<h2 class="card-title mb-4">Comments & Updates</h2>
							
							<!-- Add Comment Form -->
							<div class="form-control mb-4">
								<textarea 
									bind:value={newComment}
									placeholder="Add a comment or ask for updates..."
									rows="3"
									class="textarea textarea-bordered"
								></textarea>
								<div class="flex justify-end mt-2">
									<button 
										onclick={submitComment}
										disabled={!newComment.trim() || commentLoading}
										class="btn btn-primary btn-sm"
									>
										{#if commentLoading}
											<span class="loading loading-spinner loading-xs"></span>
										{/if}
										Post Comment
									</button>
								</div>
							</div>

							<div class="divider"></div>

							<!-- Comments List -->
							{#if comments.length === 0}
								<p class="text-gray-500 text-center py-4">No comments yet. Be the first to comment!</p>
							{:else}
								<div class="space-y-4">
									{#each comments as comment}
										<div class="border-l-4 border-primary pl-4 py-2">
											<div class="flex justify-between items-start mb-1">
												<span class="font-semibold">{comment.commenter_name || 'Anonymous'}</span>
												<span class="text-xs text-gray-500">{formatDate(comment.created_at)}</span>
											</div>
											<p class="text-gray-700">{comment.comment}</p>
											{#if comment.is_internal_note}
												<div class="badge badge-warning badge-sm mt-1">Internal Note</div>
											{/if}
										</div>
									{/each}
								</div>
							{/if}
						</div>
					</div>
				</div>

				<!-- Sidebar -->
				<div class="space-y-6">
					<!-- Assignment Info -->
					<div class="card bg-base-100 shadow-xl">
						<div class="card-body">
							<h3 class="font-semibold mb-2">Assignment</h3>
							<div class="space-y-2 text-sm">
								<div>
									<span class="text-gray-600">Department:</span>
									<p class="font-medium">{grievance.department_name || 'Not assigned'}</p>
								</div>
								<div>
									<span class="text-gray-600">Assigned To:</span>
									<p class="font-medium">{grievance.assigned_to_name || 'Not assigned'}</p>
								</div>
							</div>
						</div>
					</div>

					<!-- Quick Stats -->
					<div class="stats stats-vertical shadow">
						<div class="stat">
							<div class="stat-title">Created</div>
							<div class="stat-value text-xl">{formatDate(grievance.created_at).split(',')[0]}</div>
						</div>
						<div class="stat">
							<div class="stat-title">Last Updated</div>
							<div class="stat-value text-xl">{formatDate(grievance.updated_at).split(',')[0]}</div>
						</div>
						<div class="stat">
							<div class="stat-title">Upvotes</div>
							<div class="stat-value text-xl">{upvotesCount}</div>
						</div>
					</div>

					<!-- Admin Actions -->
					{#if isAdmin}
						<div class="card bg-base-100 shadow-xl">
							<div class="card-body">
								<h3 class="font-semibold mb-3">Admin Actions</h3>
								<div class="space-y-2">
									<button onclick={() => showUpdateModal = true} class="btn btn-sm btn-block btn-primary">
										Update Status
									</button>
									<button onclick={() => showAssignModal = true} class="btn btn-sm btn-block btn-secondary">
										Reassign
									</button>
									{#if grievance.status !== 'resolved'}
										<button onclick={() => showResolveModal = true} class="btn btn-sm btn-block btn-success">
											Mark Resolved
										</button>
									{/if}
									<button onclick={deleteGrievance} class="btn btn-sm btn-block btn-error">
										Delete
									</button>
								</div>
							</div>
						</div>
					{/if}
				</div>
			</div>
		{/if}
	</div>
</div>

<!-- Update Status Modal -->
{#if showUpdateModal}
	<div class="modal modal-open">
		<div class="modal-box">
			<h3 class="font-bold text-lg mb-4">Update Status</h3>
			<!-- svelte-ignore a11y_label_has_associated_control -->
			<div class="form-control mb-4">
				<label class="label"><span class="label-text">New Status</span></label>
				<select bind:value={newStatus} class="select select-bordered">
					<option value="">Select status...</option>
					<option value="submitted">Submitted</option>
					<option value="under_review">Under Review</option>
					<option value="in_progress">In Progress</option>
					<option value="resolved">Resolved</option>
					<option value="rejected">Rejected</option>
				</select>
			</div>
			<!-- svelte-ignore a11y_label_has_associated_control -->
			<div class="form-control mb-4">
				<label class="label"><span class="label-text">Remarks (Optional)</span></label>
				<textarea bind:value={adminRemarks} class="textarea textarea-bordered" rows="3"></textarea>
			</div>
			<div class="modal-action">
				<button onclick={() => showUpdateModal = false} class="btn">Cancel</button>
				<button onclick={updateStatus} disabled={!newStatus} class="btn btn-primary">Update</button>
			</div>
		</div>
	</div>
{/if}

<!-- Assign Modal -->
{#if showAssignModal}
	<div class="modal modal-open">
		<div class="modal-box">
			<h3 class="font-bold text-lg mb-4">Assign Grievance</h3>
			<!-- svelte-ignore a11y_label_has_associated_control -->
			<div class="form-control mb-4">
				<label class="label"><span class="label-text">Department</span></label>
				<select bind:value={assignedDepartmentId} class="select select-bordered">
					<option value="">Select department...</option>
					{#each departments as dept}
						<option value={dept.id}>{dept.name}</option>
					{/each}
				</select>
			</div>
			<!-- svelte-ignore a11y_label_has_associated_control -->
			<div class="form-control mb-4">
				<label class="label"><span class="label-text">User ID (Optional)</span></label>
				<input type="text" bind:value={assignedUserId} placeholder="Leave empty for department head" class="input input-bordered" />
			</div>
			<div class="modal-action">
				<button onclick={() => showAssignModal = false} class="btn">Cancel</button>
				<button onclick={assignGrievance} class="btn btn-primary">Assign</button>
			</div>
		</div>
	</div>
{/if}

<!-- Resolve Modal -->
{#if showResolveModal}
	<div class="modal modal-open">
		<div class="modal-box">
			<h3 class="font-bold text-lg mb-4">Mark as Resolved</h3>
			<!-- svelte-ignore a11y_label_has_associated_control -->
			<div class="form-control mb-4">
				<label class="label"><span class="label-text">Resolution Summary</span></label>
				<textarea 
					bind:value={resolutionSummary} 
					placeholder="Describe how the issue was resolved..."
					class="textarea textarea-bordered" 
					rows="4"
				></textarea>
			</div>
			<div class="modal-action">
				<button onclick={() => showResolveModal = false} class="btn">Cancel</button>
				<button onclick={resolveGrievance} disabled={!resolutionSummary.trim()} class="btn btn-success">Mark Resolved</button>
			</div>
		</div>
	</div>
{/if}
