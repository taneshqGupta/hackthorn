<script lang="ts">
	import { user } from '$lib/auth';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import api from '$lib/api';
	import type { Department } from '$lib/types';
	import PDA from '$lib/components/PDA.svelte';

	let currentUser = $derived($user);
	let loading = $state(false);
	let error = $state('');
	let success = $state(false);
	let departments: Department[] = $state([]);
	let currentPage = $state('details');

	// Form fields
	let title = $state('');
	let description = $state('');
	let category = $state('infrastructure');
	let priority = $state('medium');
	let location = $state('');
	let isAnonymous = $state(false);
	let departmentId = $state('');
	let selectedFiles: File[] = $state([]);
	let filePreviews: string[] = $state([]);

	const categories = [
		{ value: 'infrastructure', label: 'Infrastructure', icon: '[INF]' },
		{ value: 'academics', label: 'Academics', icon: '[ACA]' },
		{ value: 'hostel', label: 'Hostel', icon: '[HST]' },
		{ value: 'food', label: 'Food', icon: '[FOD]' },
		{ value: 'other', label: 'Other', icon: '[OTH]' }
	];

	const priorities = [
		{ value: 'low', label: 'Low', symbol: '◯' },
		{ value: 'medium', label: 'Medium', symbol: '◐' },
		{ value: 'high', label: 'High', symbol: '◕' },
		{ value: 'urgent', label: 'Urgent', symbol: '●' }
	];

	onMount(async () => {
		if (!$user) {
			goto('/login');
			return;
		}
		await loadDepartments();
	});

	async function loadDepartments() {
		try {
			departments = await api.get('/api/departments');
		} catch (err) {
			console.error('Failed to load departments:', err);
		}
	}

	function handleFileSelect(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input.files) {
			const newFiles = Array.from(input.files);
			if (selectedFiles.length + newFiles.length > 5) {
				error = 'Maximum 5 photos allowed';
				return;
			}

			selectedFiles = [...selectedFiles, ...newFiles];
			
			newFiles.forEach(file => {
				const reader = new FileReader();
				reader.onload = (e) => {
					filePreviews = [...filePreviews, e.target?.result as string];
				};
				reader.readAsDataURL(file);
			});
		}
	}

	function removeFile(index: number) {
		selectedFiles = selectedFiles.filter((_, i) => i !== index);
		filePreviews = filePreviews.filter((_, i) => i !== index);
	}

	async function handleSubmit() {
		console.log('[SUBMIT] handleSubmit called');
		console.log('[SUBMIT] Current user:', currentUser);
		
		if (!title.trim()) {
			console.log('[SUBMIT] Validation failed: Title is empty');
			error = 'Title is required';
			return;
		}
		if (!description.trim()) {
			console.log('[SUBMIT] Validation failed: Description is empty');
			error = 'Description is required';
			return;
		}
		if (title.length < 10) {
			console.log('[SUBMIT] Validation failed: Title too short:', title.length);
			error = 'Title must be at least 10 characters';
			return;
		}
		if (description.length < 20) {
			console.log('[SUBMIT] Validation failed: Description too short:', description.length);
			error = 'Description must be at least 20 characters';
			return;
		}

		console.log('[SUBMIT] Validation passed, starting submission');
		loading = true;
		error = '';

		try {
			const grievanceData = {
				title: title.trim(),
				description: description.trim(),
				category,
				priority,
				location_type: departmentId || null,
				location_details: location.trim() || null,
				is_anonymous: isAnonymous
			};
			
			console.log('[SUBMIT] Grievance data:', grievanceData);
			console.log('[SUBMIT] Posting to /grievances endpoint');

			const response = await api.post('/grievances', grievanceData);
			const grievanceId = response.id;
			
			console.log('[SUBMIT] Grievance created successfully, ID:', grievanceId);

			if (selectedFiles.length > 0) {
				console.log('[SUBMIT] Uploading', selectedFiles.length, 'photos');
				const formData = new FormData();
				selectedFiles.forEach(file => {
					console.log('[SUBMIT] Adding photo:', file.name, file.size, 'bytes');
					formData.append('photos', file);
				});

				console.log('[SUBMIT] Posting photos to /grievances/' + grievanceId + '/photos');
				await api.post(`/grievances/${grievanceId}/photos`, formData);
				console.log('[SUBMIT] Photos uploaded successfully');
			} else {
				console.log('[SUBMIT] No photos to upload');
			}

			console.log('[SUBMIT] Submission complete, redirecting to grievance page');
			success = true;
			setTimeout(() => {
				goto(`/grievances/${grievanceId}`);
			}, 1500);
		} catch (err: any) {
			console.error('[SUBMIT] Submission failed:', err);
			console.error('[SUBMIT] Error message:', err.message);
			error = err.message || 'Failed to submit grievance';
			loading = false;
		}
	}

	function nextPage() {
		const pageIndex = ['details', 'category', 'upload', 'review'].indexOf(currentPage);
		if (pageIndex < 3) {
			currentPage = ['details', 'category', 'upload', 'review'][pageIndex + 1];
		}
	}

	function prevPage() {
		const pageIndex = ['details', 'category', 'upload', 'review'].indexOf(currentPage);
		if (pageIndex > 0) {
			currentPage = ['details', 'category', 'upload', 'review'][pageIndex - 1];
		}
	}
</script>

{#snippet detailsPage()}
	<div class="pda-content">
		<div class="pda-field">
			<div class="pda-label">TITLE</div>
			<input type="text" bind:value={title} maxlength="200" class="pda-input" placeholder="Brief summary..." />
			<div class="pda-counter">{title.length}/200</div>
		</div>

		<div class="pda-field">
			<div class="pda-label">DESCRIPTION</div>
			<textarea bind:value={description} maxlength="2000" class="pda-textarea" rows="6" placeholder="Detailed description of the issue..."></textarea>
			<div class="pda-counter">{description.length}/2000</div>
		</div>

		<div class="pda-field">
			<div class="pda-label">LOCATION</div>
			<input type="text" bind:value={location} maxlength="100" class="pda-input" placeholder="Building, room, area..." />
		</div>

		<div class="pda-field">
			<div class="pda-label">DEPARTMENT</div>
			<select bind:value={departmentId} class="pda-select">
				<option value="">-- Select Department --</option>
				{#each departments as dept}
					<option value={dept.id}>{dept.name}</option>
				{/each}
			</select>
		</div>

		<div class="pda-check">
			<input type="checkbox" id="anon" bind:checked={isAnonymous} />
			<label for="anon">SUBMIT ANONYMOUSLY</label>
		</div>

		<button onclick={nextPage} class="pda-btn">NEXT →</button>
	</div>
{/snippet}

{#snippet categoryPage()}
	<div class="pda-content">
		<div class="pda-label">SELECT CATEGORY</div>
		<div class="pda-grid">
			{#each categories as cat}
				<button 
					class="pda-card {category === cat.value ? 'pda-card--active' : ''}" 
					onclick={() => category = cat.value}
				>
					<div class="pda-card-icon">{cat.icon}</div>
					<div class="pda-card-label">{cat.label}</div>
				</button>
			{/each}
		</div>

		<div class="pda-label" style="margin-top: 16px;">SELECT PRIORITY</div>
		<div class="pda-grid">
			{#each priorities as pri}
				<button 
					class="pda-card pda-card--small {priority === pri.value ? 'pda-card--active' : ''}" 
					onclick={() => priority = pri.value}
				>
					<div class="pda-card-icon">{pri.symbol}</div>
					<div class="pda-card-label">{pri.label}</div>
				</button>
			{/each}
		</div>

		<div class="pda-nav">
			<button onclick={prevPage} class="pda-btn">← BACK</button>
			<button onclick={nextPage} class="pda-btn">NEXT →</button>
		</div>
	</div>
{/snippet}

{#snippet uploadPage()}
	<div class="pda-content">
		<div class="pda-label">ATTACH PHOTOS (MAX 5)</div>
		
		<div class="pda-upload">
			<input type="file" id="photos" accept="image/*" multiple onchange={handleFileSelect} hidden />
			<label for="photos" class="pda-upload-btn">
				<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M17 8l-5-5-5 5M12 3v12"/>
				</svg>
				SELECT FILES
			</label>
			<div class="pda-hint">{selectedFiles.length}/5 photos selected</div>
		</div>

		{#if filePreviews.length > 0}
			<div class="pda-previews">
				{#each filePreviews as preview, i}
					<div class="pda-preview">
						<img src={preview} alt="Preview {i + 1}" />
						<button onclick={() => removeFile(i)} class="pda-preview-remove">×</button>
					</div>
				{/each}
			</div>
		{/if}

		<div class="pda-nav">
			<button onclick={prevPage} class="pda-btn">← BACK</button>
			<button onclick={nextPage} class="pda-btn">NEXT →</button>
		</div>
	</div>
{/snippet}

{#snippet reviewPage()}
	<div class="pda-content">
		<div class="pda-label">REVIEW SUBMISSION</div>

		<div class="pda-review">
			<div class="pda-kv">
				<div class="pda-k">CATEGORY:</div>
				<div class="pda-v">{categories.find(c => c.value === category)?.label || category}</div>
			</div>
			<div class="pda-kv">
				<div class="pda-k">PRIORITY:</div>
				<div class="pda-v">{priorities.find(p => p.value === priority)?.label || priority}</div>
			</div>
			<div class="pda-kv">
				<div class="pda-k">LOCATION:</div>
				<div class="pda-v">{location || 'Not specified'}</div>
			</div>
			<div class="pda-kv">
				<div class="pda-k">DEPARTMENT:</div>
				<div class="pda-v">{departments.find(d => d.id === departmentId)?.name || 'Not specified'}</div>
			</div>
			<div class="pda-kv">
				<div class="pda-k">ANONYMOUS:</div>
				<div class="pda-v">{isAnonymous ? 'YES' : 'NO'}</div>
			</div>
			<div class="pda-kv">
				<div class="pda-k">PHOTOS:</div>
				<div class="pda-v">{selectedFiles.length}</div>
			</div>

			<div class="pda-box" style="margin-top: 12px;">
				<div class="pda-label">TITLE</div>
				<div style="font-weight: 900; font-size: 10px; margin-top: 4px;">{title || '[No title]'}</div>
			</div>

			<div class="pda-box" style="margin-top: 8px;">
				<div class="pda-label">DESCRIPTION</div>
				<div style="font-weight: 900; font-size: 10px; margin-top: 4px; line-height: 1.4;">{description || '[No description]'}</div>
			</div>
		</div>

		{#if error}
			<div class="pda-error">{error}</div>
		{/if}

		{#if success}
			<div class="pda-success">✓ SUBMITTED SUCCESSFULLY</div>
		{/if}

		<div class="pda-nav">
			<button onclick={prevPage} class="pda-btn" disabled={loading}>← BACK</button>
			<button onclick={handleSubmit} class="pda-btn pda-btn--primary" disabled={loading || success}>
				{#if loading}
					SUBMITTING...
				{:else if success}
					SUBMITTED ✓
				{:else}
					SUBMIT
				{/if}
			</button>
		</div>
	</div>
{/snippet}

<div class="min-h-screen bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900 flex items-center justify-center p-4">
	<PDA 
		pages={[
			{ id: 'details', title: 'DETAILS', icon: '<svg class="pda2__ico" viewBox="0 0 24 24"><path class="pda2__st" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4" stroke="var(--lcd-ink)" stroke-width="1.7" fill="none"/></svg>', content: detailsPage },
			{ id: 'category', title: 'CATEGORY', icon: '<svg class="pda2__ico" viewBox="0 0 24 24"><path class="pda2__st" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A2 2 0 013 12V7a4 4 0 014-4z" stroke="var(--lcd-ink)" stroke-width="1.7" fill="none"/></svg>', content: categoryPage },
			{ id: 'upload', title: 'UPLOAD', icon: '<svg class="pda2__ico" viewBox="0 0 24 24"><path class="pda2__st" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" stroke="var(--lcd-ink)" stroke-width="1.7" fill="none"/></svg>', content: uploadPage },
			{ id: 'review', title: 'REVIEW', icon: '<svg class="pda2__ico" viewBox="0 0 24 24"><path class="pda2__st" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" stroke="var(--lcd-ink)" stroke-width="1.7" fill="none"/></svg>', content: reviewPage }
		]}
		width="600px" 
		height="750px" 
		bind:currentPage 
		showBootSequence={false}
	/>
</div>

<style>
.pda-content {
	padding: 6px;
	font-size: 11px;
	font-weight: 900;
}

.pda-field {
	margin-bottom: 8px;
}

.pda-label {
	font-weight: 900;
	font-size: 10px;
	color: var(--lcd-dim);
	margin-bottom: 4px;
	text-transform: uppercase;
}

.pda-input,
.pda-textarea,
.pda-select {
	width: 100%;
	background: rgba(255, 255, 255, 0.3);
	border: 1px solid rgba(0, 0, 0, 0.25);
	padding: 6px;
	font-family: inherit;
	font-size: 10px;
	font-weight: 900;
	color: var(--lcd-ink);
	border-radius: 2px;
	box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.5), inset 0 -1px 0 rgba(0, 0, 0, 0.08);
}

.pda-textarea {
	resize: none;
	line-height: 1.4;
}

.pda-counter {
	text-align: right;
	font-size: 9px;
	color: var(--lcd-dim);
	margin-top: 2px;
}

.pda-check {
	display: flex;
	align-items: center;
	gap: 6px;
	margin: 12px 0;
}

.pda-check input[type="checkbox"] {
	width: 12px;
	height: 12px;
}

.pda-check label {
	font-size: 10px;
	font-weight: 900;
	cursor: pointer;
}

.pda-btn {
	width: 100%;
	padding: 8px;
	background: rgba(0, 0, 0, 0.15);
	border: 1px solid rgba(0, 0, 0, 0.3);
	font-family: inherit;
	font-size: 10px;
	font-weight: 900;
	color: var(--lcd-ink);
	cursor: pointer;
	border-radius: 2px;
	box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.3), inset 0 -1px 0 rgba(0, 0, 0, 0.1);
	transition: all 0.15s;
}

.pda-btn:hover:not(:disabled) {
	background: rgba(0, 0, 0, 0.2);
	transform: translateY(-1px);
}

.pda-btn:active:not(:disabled) {
	transform: translateY(0);
	box-shadow: inset 0 2px 3px rgba(0, 0, 0, 0.2);
}

.pda-btn:disabled {
	opacity: 0.5;
	cursor: not-allowed;
}

.pda-btn--primary {
	background: rgba(0, 0, 0, 0.25);
	border-color: rgba(0, 0, 0, 0.4);
}

.pda-grid {
	display: grid;
	grid-template-columns: repeat(2, 1fr);
	gap: 6px;
	margin-top: 8px;
}

.pda-card {
	padding: 12px 8px;
	background: rgba(255, 255, 255, 0.2);
	border: 1px solid rgba(0, 0, 0, 0.2);
	border-radius: 2px;
	cursor: pointer;
	transition: all 0.15s;
	display: flex;
	flex-direction: column;
	align-items: center;
	gap: 4px;
}

.pda-card:hover {
	background: rgba(255, 255, 255, 0.3);
}

.pda-card--active {
	background: rgba(0, 0, 0, 0.2);
	border-color: rgba(0, 0, 0, 0.4);
	box-shadow: inset 0 2px 3px rgba(0, 0, 0, 0.15);
}

.pda-card--small {
	padding: 8px 6px;
}

.pda-card-icon {
	font-size: 20px;
}

.pda-card-label {
	font-size: 9px;
	font-weight: 900;
	text-transform: uppercase;
}

.pda-nav {
	display: grid;
	grid-template-columns: 1fr 1fr;
	gap: 6px;
	margin-top: 12px;
}

.pda-upload {
	margin: 12px 0;
}

.pda-upload-btn {
	display: flex;
	flex-direction: column;
	align-items: center;
	gap: 6px;
	padding: 16px;
	background: rgba(255, 255, 255, 0.2);
	border: 2px dashed rgba(0, 0, 0, 0.3);
	border-radius: 2px;
	cursor: pointer;
	font-size: 10px;
	font-weight: 900;
	transition: all 0.15s;
}

.pda-upload-btn:hover {
	background: rgba(255, 255, 255, 0.3);
}

.pda-hint {
	text-align: center;
	font-size: 9px;
	color: var(--lcd-dim);
	margin-top: 6px;
}

.pda-previews {
	display: grid;
	grid-template-columns: repeat(3, 1fr);
	gap: 6px;
	margin-top: 12px;
}

.pda-preview {
	position: relative;
	aspect-ratio: 1;
	background: rgba(0, 0, 0, 0.1);
	border: 1px solid rgba(0, 0, 0, 0.2);
	border-radius: 2px;
	overflow: hidden;
}

.pda-preview img {
	width: 100%;
	height: 100%;
	object-fit: cover;
	filter: sepia(0.3) contrast(1.1);
}

.pda-preview-remove {
	position: absolute;
	top: 2px;
	right: 2px;
	width: 16px;
	height: 16px;
	background: rgba(0, 0, 0, 0.7);
	color: var(--lcd-bg);
	border: none;
	border-radius: 2px;
	cursor: pointer;
	font-size: 14px;
	line-height: 1;
	display: grid;
	place-items: center;
}

.pda-review {
	margin-top: 8px;
}

.pda-kv {
	display: grid;
	grid-template-columns: 90px 1fr;
	gap: 6px;
	font-size: 10px;
	margin-bottom: 4px;
}

.pda-k {
	color: var(--lcd-dim);
}

.pda-v {
	color: var(--lcd-ink);
}

.pda-box {
	border: 1px solid rgba(0, 0, 0, 0.25);
	background: linear-gradient(180deg, rgba(255, 255, 255, 0.28), rgba(255, 255, 255, 0.12));
	padding: 6px;
	border-radius: 2px;
	box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.55), inset 0 -1px 0 rgba(0, 0, 0, 0.08);
}

.pda-error {
	background: rgba(0, 0, 0, 0.2);
	border: 1px dashed var(--lcd-ink);
	padding: 8px;
	margin: 12px 0;
	font-size: 10px;
	font-weight: 900;
	text-align: center;
	animation: errflash 0.6s steps(2, end) 2;
}

.pda-success {
	background: rgba(0, 0, 0, 0.15);
	border: 1px solid rgba(0, 0, 0, 0.3);
	padding: 8px;
	margin: 12px 0;
	font-size: 10px;
	font-weight: 900;
	text-align: center;
}

@keyframes errflash {
	50% { filter: invert(1); }
}
</style>
