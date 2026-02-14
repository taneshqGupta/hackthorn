<!-- DaisyUI uses label elements for styling, not form association -->
<!-- svelte-ignore a11y_label_has_associated_control -->
<script lang="ts">
	import { user } from '$lib/auth';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import api from '$lib/api';
	import type { Department } from '$lib/types';

	let currentUser = $derived($user);
	let loading = $state(false);
	let error = $state('');
	let success = $state(false);
	let departments: Department[] = $state([]);

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
		{ value: 'infrastructure', label: 'Infrastructure', icon: '🏗️', description: 'Buildings, roads, facilities' },
		{ value: 'academics', label: 'Academics', icon: '📚', description: 'Courses, exams, academics' },
		{ value: 'hostel', label: 'Hostel', icon: '🏠', description: 'Hostel facilities & services' },
		{ value: 'food', label: 'Food', icon: '🍽️', description: 'Mess, canteen, food quality' },
		{ value: 'other', label: 'Other', icon: '📋', description: 'Other campus issues' }
	];

	const priorities = [
		{ value: 'low', label: 'Low', color: 'text-gray-500', description: 'Can be addressed later' },
		{ value: 'medium', label: 'Medium', color: 'text-blue-500', description: 'Needs attention soon' },
		{ value: 'high', label: 'High', color: 'text-orange-500', description: 'Requires prompt action' },
		{ value: 'urgent', label: 'Urgent', color: 'text-red-500', description: 'Critical issue, immediate action needed' }
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
			departments = await api.get('/grievances/departments');
		} catch (err) {
			console.error('Failed to load departments:', err);
		}
	}

	function handleFileSelect(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input.files) {
			const newFiles = Array.from(input.files);
			
			// Validate file types (images only)
			const validFiles = newFiles.filter(file => file.type.startsWith('image/'));
			if (validFiles.length !== newFiles.length) {
				error = 'Only image files are allowed';
				setTimeout(() => error = '', 3000);
			}

			// Limit to 5 files
			if (selectedFiles.length + validFiles.length > 5) {
				error = 'Maximum 5 photos allowed';
				setTimeout(() => error = '', 3000);
				return;
			}

			selectedFiles = [...selectedFiles, ...validFiles];
			
			// Create previews
			validFiles.forEach(file => {
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
		// Validation
		if (!title.trim()) {
			error = 'Title is required';
			return;
		}
		if (!description.trim()) {
			error = 'Description is required';
			return;
		}
		if (title.length < 10) {
			error = 'Title must be at least 10 characters';
			return;
		}
		if (description.length < 20) {
			error = 'Description must be at least 20 characters';
			return;
		}

		loading = true;
		error = '';

		try {
			// Create grievance
			const grievanceData = {
				title: title.trim(),
				description: description.trim(),
				category,
				priority,
				location: location.trim() || null,
				is_anonymous: isAnonymous,
				department_id: departmentId || null
			};

			const response = await api.post('/grievances', grievanceData);
			const grievanceId = response.id;

			// Upload photos if any
			if (selectedFiles.length > 0) {
				const formData = new FormData();
				selectedFiles.forEach(file => {
					formData.append('photos', file);
				});

				await api.post(`/grievances/${grievanceId}/photos`, formData);
			}

			success = true;
			setTimeout(() => {
				goto(`/grievances/${grievanceId}`);
			}, 1500);
		} catch (err: any) {
			error = err.message || 'Failed to submit grievance';
			loading = false;
		}
	}

	function resetForm() {
		title = '';
		description = '';
		category = 'infrastructure';
		priority = 'medium';
		location = '';
		isAnonymous = false;
		departmentId = '';
		selectedFiles = [];
		filePreviews = [];
		error = '';
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
				<div>
					<h1 class="text-3xl font-bold">Submit Grievance</h1>
					<p class="text-gray-600 mt-1">Report an issue and help improve campus life</p>
				</div>
			</div>
		</div>
	</div>

	<div class="container mx-auto px-4 py-8 max-w-4xl">
		{#if success}
			<!-- Success Message -->
			<div class="alert alert-success mb-6">
				<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
				</svg>
				<span>Grievance submitted successfully! Redirecting...</span>
			</div>
		{/if}

		{#if error}
			<div class="alert alert-error mb-6">
				<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
				</svg>
				<span>{error}</span>
			</div>
		{/if}

		<form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-6">
			<!-- Title -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title mb-4">Issue Details</h2>
					
					<div class="form-control">
						<label class="label">
							<span class="label-text font-semibold">Title <span class="text-error">*</span></span>
							<span class="label-text-alt">{title.length} / 200</span>
						</label>
						<input 
							type="text" 
							bind:value={title}
							placeholder="Brief description of the issue (e.g., 'Broken light in Hostel A corridor')"
							maxlength="200"
							class="input input-bordered"
							required
						/>
						<label class="label">
							<span class="label-text-alt text-gray-500">Minimum 10 characters</span>
						</label>
					</div>

					<div class="form-control">
						<label class="label">
							<span class="label-text font-semibold">Description <span class="text-error">*</span></span>
							<span class="label-text-alt">{description.length} / 1000</span>
						</label>
						<textarea 
							bind:value={description}
							placeholder="Provide detailed information about the issue, including when it started, how it affects you, and any other relevant details..."
							maxlength="1000"
							rows="6"
							class="textarea textarea-bordered"
							required
						></textarea>
						<label class="label">
							<span class="label-text-alt text-gray-500">Minimum 20 characters. Be specific and provide relevant details.</span>
						</label>
					</div>
				</div>
			</div>

			<!-- Category -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title mb-4">Categorization</h2>
					
					<div class="form-control">
						<label class="label">
							<span class="label-text font-semibold">Category <span class="text-error">*</span></span>
						</label>
						<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
							{#each categories as cat}
								<label class="cursor-pointer">
									<input 
										type="radio" 
										name="category" 
										bind:group={category}
										value={cat.value}
										class="hidden"
									/>
									<div class={`card border-2 transition-all ${category === cat.value ? 'border-primary bg-primary/10' : 'border-base-300 hover:border-primary/50'}`}>
										<div class="card-body p-4">
											<div class="text-3xl mb-1">{cat.icon}</div>
											<h3 class="font-semibold">{cat.label}</h3>
											<p class="text-xs text-gray-600">{cat.description}</p>
										</div>
									</div>
								</label>
							{/each}
						</div>
					</div>

					<div class="form-control mt-4">
						<label class="label">
							<span class="label-text font-semibold">Priority Level <span class="text-error">*</span></span>
						</label>
						<div class="grid grid-cols-2 md:grid-cols-4 gap-3">
							{#each priorities as prio}
								<label class="cursor-pointer">
									<input 
										type="radio" 
										name="priority" 
										bind:group={priority}
										value={prio.value}
										class="hidden"
									/>
									<div class={`card border-2 transition-all ${priority === prio.value ? 'border-primary bg-primary/10' : 'border-base-300 hover:border-primary/50'}`}>
										<div class="card-body p-4 text-center">
											<h3 class={`font-semibold ${prio.color}`}>{prio.label}</h3>
											<p class="text-xs text-gray-600">{prio.description}</p>
										</div>
									</div>
								</label>
							{/each}
						</div>
					</div>
				</div>
			</div>

			<!-- Location & Department -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title mb-4">Additional Information</h2>
					
					<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
						<div class="form-control">
							<label class="label">
								<span class="label-text font-semibold">Location</span>
							</label>
							<input 
								type="text" 
								bind:value={location}
								placeholder="e.g., Hostel A, Room 201, Academic Block 3"
								class="input input-bordered"
							/>
							<label class="label">
								<span class="label-text-alt text-gray-500">Specific location helps faster resolution</span>
							</label>
						</div>

						<div class="form-control">
							<label class="label">
								<span class="label-text font-semibold">Department (Optional)</span>
							</label>
							<select bind:value={departmentId} class="select select-bordered">
								<option value="">Auto-assign based on category</option>
								{#each departments as dept}
									<option value={dept.id}>{dept.name}</option>
								{/each}
							</select>
							<label class="label">
								<span class="label-text-alt text-gray-500">Leave empty for automatic assignment</span>
							</label>
						</div>
					</div>
				</div>
			</div>

			<!-- Photo Upload -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<h2 class="card-title mb-4">Visual Evidence (Optional)</h2>
					
					<div class="form-control">
						<label class="label">
							<span class="label-text font-semibold">Upload Photos</span>
							<span class="label-text-alt">{selectedFiles.length} / 5 photos</span>
						</label>
						
						<input 
							type="file" 
							accept="image/*"
							multiple
							onchange={handleFileSelect}
							class="file-input file-input-bordered"
							disabled={selectedFiles.length >= 5}
						/>
						
						<label class="label">
							<span class="label-text-alt text-gray-500">Photos help authorities understand the issue better. Max 5 images.</span>
						</label>

						{#if filePreviews.length > 0}
							<div class="grid grid-cols-2 md:grid-cols-3 gap-4 mt-4">
								{#each filePreviews as preview, index}
									<div class="relative group">
										<img src={preview} alt="Preview {index + 1}" class="w-full h-32 object-cover rounded-lg" />
										<button 
											type="button"
											onclick={() => removeFile(index)}
											aria-label="Remove photo {index + 1}"
											class="absolute top-2 right-2 btn btn-circle btn-sm btn-error opacity-0 group-hover:opacity-100 transition-opacity"
										>
											<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
												<path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
											</svg>
										</button>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				</div>
			</div>

			<!-- Anonymous Submission -->
			<div class="card bg-base-100 shadow-xl">
				<div class="card-body">
					<div class="form-control">
						<label class="label cursor-pointer justify-start gap-4">
							<input type="checkbox" bind:checked={isAnonymous} class="checkbox checkbox-primary" />
							<div>
								<span class="label-text font-semibold">Submit Anonymously</span>
								<p class="text-sm text-gray-600 mt-1">Your identity will be hidden from public view, but authorities can see who submitted this grievance.</p>
							</div>
						</label>
					</div>
				</div>
			</div>

			<!-- Submit Button -->
			<div class="flex gap-4">
				<button type="button" onclick={resetForm} class="btn btn-ghost flex-1">
					Reset Form
				</button>
				<button type="submit" disabled={loading} class="btn btn-primary flex-1">
					{#if loading}
						<span class="loading loading-spinner"></span>
						Submitting...
					{:else}
						Submit Grievance
					{/if}
				</button>
			</div>
		</form>
	</div>
</div>
