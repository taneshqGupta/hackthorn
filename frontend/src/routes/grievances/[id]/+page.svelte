<script lang="ts">
    import { page } from '$app/stores';
    import { user } from '$lib/auth';
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';
    import api from '$lib/api';
    import type { Grievance, GrievanceComment, GrievanceStatusHistory, ApiResponse } from '$lib/types';

    // --- State (Svelte 5 Runes) ---
    let grievance = $state<Grievance | null>(null);
    let comments = $state<GrievanceComment[]>([]);
    let history = $state<GrievanceStatusHistory[]>([]);
    
    let loading = $state(true);
    let error = $state('');

    // Optimistic UI State
    let isUpvoted = $state(false);
    let localUpvoteCount = $state(0);
    
    // Comment Input State
    let newComment = $state('');
    let submittingComment = $state(false);

    // Derived Values
    const grievanceId = $derived($page.params.id);
    const currentUser = $derived($user);
    const isAdmin = $derived(currentUser?.role === 'admin' || currentUser?.role === 'authority');

    onMount(async () => {
        if (!currentUser) { 
            goto('/login'); 
            return; 
        }
        await loadAllData();
    });

    async function loadAllData() {
        loading = true;
        try {
            const [gRes, cRes, hRes] = await Promise.all([
                api.get<ApiResponse<Grievance>>(`/api/grievances/${grievanceId}`),
                api.get<ApiResponse<GrievanceComment[]>>(`/api/grievances/${grievanceId}/comments`),
                api.get<ApiResponse<GrievanceStatusHistory[]>>(`/api/grievances/${grievanceId}/history`)
            ]);

            if (gRes.data) {
                grievance = gRes.data;
                isUpvoted = grievance.user_has_upvoted;
                localUpvoteCount = grievance.upvote_count;
            }

            if (cRes.data) comments = cRes.data;
            if (hRes.data) history = hRes.data;

        } catch (e: any) {
            console.error("Load failed:", e);
            error = e.message || "Failed to load grievance details.";
        } finally {
            loading = false;
        }
    }

    async function handleUpvote() {
        if (!grievance) return;
        
        const previousState = isUpvoted;
        isUpvoted = !isUpvoted;
        localUpvoteCount += isUpvoted ? 1 : -1;

        try {
            await api.post(`/api/grievances/${grievanceId}/upvote`);
        } catch (e) {
            console.error("Upvote failed", e);
            isUpvoted = previousState;
            localUpvoteCount += isUpvoted ? 1 : -1;
        }
    }

    async function handlePostComment() {
        if (!newComment.trim()) return;
        
        submittingComment = true;
        try {
            const res = await api.post<ApiResponse<GrievanceComment>>(`/api/grievances/${grievanceId}/comments`, {
                comment: newComment,
                is_internal: false 
            });
            
            if (res.data) {
                comments = [...comments, res.data];
                newComment = '';
            }
        } catch (e: any) {
            alert(e.message || "Failed to post comment");
        } finally {
            submittingComment = false;
        }
    }

    // Helper functions
    function formatName(u: { first_name: string, last_name: string } | null): string {
        if (!u) return 'Unknown User';
        return `${u.first_name} ${u.last_name}`;
    }

    function formatDate(d: string) {
        return new Date(d).toLocaleDateString('en-US', { 
            month: 'short', day: 'numeric', hour: '2-digit', minute:'2-digit'
        });
    }
</script>

<div class="page-container">
    {#if loading}
        <div class="loading-state">LOADING DATA...</div>
    {:else if error}
        <div class="error-state">{error}</div>
    {:else if grievance}
        
        <div class="header">
            <button onclick={() => goto('/grievances')} class="back-btn">← BACK TO FEED</button>
            <div class="status-badge {grievance.status}">
                {grievance.status.replace('_', ' ')}
            </div>
        </div>

        <div class="grid-layout">
            <div class="brutalist-card main-card">
                <div class="content-header">
                    <h1 class="title">{grievance.title}</h1>
                    <div class="meta-row">
                        <span class="category-tag">[{grievance.category.toUpperCase()}]</span>
                        <span class="dot">•</span>
                        <span class="user-name">
                            {grievance.is_anonymous ? 'ANONYMOUS' : formatName(grievance.submitter).toUpperCase()}
                        </span>
                        <span class="dot">•</span>
                        <span class="date">{formatDate(grievance.created_at)}</span>
                    </div>
                </div>

                <p class="description">{grievance.description}</p>

                {#if grievance.location_details || grievance.assigned_department}
                    <div class="info-box">
                        {#if grievance.location_details}
                            <div class="info-item">
                                <span class="label">LOCATION:</span> {grievance.location_details}
                            </div>
                        {/if}
                        {#if grievance.assigned_department}
                            <div class="info-item">
                                <span class="label">DEPT:</span> {grievance.assigned_department}
                            </div>
                        {/if}
                    </div>
                {/if}

                {#if grievance.photo_urls && grievance.photo_urls.length > 0}
                    <div class="photos-grid">
                        {#each grievance.photo_urls as photo}
                            <a href={photo} target="_blank" rel="noreferrer" class="photo-frame">
                                <img src={photo} alt="Evidence" />
                            </a>
                        {/each}
                    </div>
                {/if}

                <div class="divider"></div>

                <div class="card-footer">
                    <button 
                        class="upvote-btn" 
                        class:active={isUpvoted}
                        onclick={handleUpvote}
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" fill={isUpvoted ? "currentColor" : "none"} viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z" />
                        </svg>
                        <span>{localUpvoteCount}</span>
                    </button>
                    
                    {#if isAdmin}
                        <div class="admin-controls">
                            <button class="retro-btn admin">MANAGE STATUS</button>
                        </div>
                    {/if}
                </div>
            </div>

            <div class="sidebar">
                
                {#if grievance.status === 'resolved'}
                    <div class="brutalist-card resolution-banner">
                        <h3>✓ RESOLVED</h3>
                        <p>{grievance.resolution_notes || 'No resolution notes provided.'}</p>
                        <small>BY {formatName(grievance.assigned_to).toUpperCase()}</small>
                    </div>
                {/if}

                <div class="brutalist-card">
                    <h3>DISCUSSION ({comments.length})</h3>
                    
                    <div class="comment-input-area">
                        <textarea 
                            bind:value={newComment}
                            placeholder="WRITE A COMMENT..." 
                            rows="2"
                        ></textarea>
                        <button 
                            class="retro-btn"
                            disabled={submittingComment || !newComment.trim()} 
                            onclick={handlePostComment}
                        >
                            {submittingComment ? 'POSTING...' : 'POST'}
                        </button>
                    </div>

                    <div class="comments-list">
                        {#each comments as comment}
                            <div class="comment-item {comment.is_internal ? 'internal' : ''}">
                                <div class="comment-header">
                                    <span class="c-name">{formatName(comment.user).toUpperCase()}</span>
                                    {#if comment.is_internal}<span class="badge-internal">INTERNAL</span>{/if}
                                    <span class="c-time">{formatDate(comment.created_at)}</span>
                                </div>
                                <div class="comment-body">{comment.comment}</div>
                            </div>
                        {/each}
                    </div>
                </div>

                <div class="brutalist-card">
                    <h3>TIMELINE</h3>
                    <div class="timeline">
                        {#each history as event}
                            <div class="timeline-item">
                                <div class="t-dot"></div>
                                <div class="t-content">
                                    <div class="t-status">CHANGED TO <b>{event.new_status.replace('_', ' ').toUpperCase()}</b></div>
                                    <div class="t-meta">
                                        BY {formatName(event.updated_by).toUpperCase()} • {formatDate(event.created_at)}
                                    </div>
                                    {#if event.remarks}
                                        <div class="t-remarks">"{event.remarks}"</div>
                                    {/if}
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>

            </div>
        </div>
    {/if}
</div>

<style>
    /* Reset & Base */
    :global(body) { background-color: #f0f2f5; font-family: inherit; }

    .page-container {
        max-width: 1100px;
        margin: 0 auto;
        padding: 30px 20px;
    }

    /* --- Retro Components --- */
    
    /* The Core Card Style */
    .brutalist-card {
        background: transparent;
        border: 2px solid rgba(198, 225, 237, 0.8);
        box-shadow: 6px 6px 0px rgba(0, 0, 0, 0.5); /* Hard shadow */
        padding: 24px;
        margin-bottom: 24px;
        border-radius: 0px; /* Sharp corners */
    }

    .retro-btn {
        background: #b31b34;
        color: #fff;
        border: 2px solid #2b0b0b;
        padding: 6px 16px;
        font-family: inherit;
        font-weight: 600;
        font-size: 14px;
        cursor: pointer;
        box-shadow: 3px 3px 0px rgba(0,0,0,1);
        border-radius: 0px;
        transition: transform 0.1s, box-shadow 0.1s;
    }
    .retro-btn:active {
        transform: translate(2px, 2px);
        box-shadow: 1px 1px 0px rgba(0,0,0,1);
    }
    .retro-btn:disabled {
        background: #888;
        cursor: not-allowed;
    }
    .retro-btn.admin {
        background: #2b0b0b;
        color: #fff;
    }

    /* Header */
    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 30px;
    }
    .back-btn {
        background: transparent;
        border: 2px solid transparent;
        color: #2b0b0b;
        cursor: pointer;
        font-weight: 700;
        font-size: 16px;
        font-family: inherit;
        padding: 4px 8px;
    }
    .back-btn:hover { 
        text-decoration: underline; 
        color: #b31b34;
    }

    .status-badge {
        padding: 6px 14px;
        border: 2px solid #2b0b0b;
        background: #fff;
        font-size: 14px;
        font-weight: 700;
        text-transform: uppercase;
        box-shadow: 3px 3px 0px rgba(0,0,0,0.5);
    }
    .submitted { background: #e0f2fe; color: #0284c7; }
    .under_review { background: #ffedd5; color: #c2410c; }
    .in_progress { background: #dcfce7; color: #15803d; }
    .resolved { background: #d1fae5; color: #047857; }
    .rejected { background: #fee2e2; color: #b91c1c; }

    /* Layout */
    .grid-layout {
        display: grid;
        grid-template-columns: 1fr 380px;
        gap: 30px;
    }
    @media (max-width: 800px) {
        .grid-layout { grid-template-columns: 1fr; }
    }

    /* Content Typography */
    .title {
        margin: 0 0 12px 0;
        color: #2b0b0b;
        font-size: 2.5rem; /* Larger for Jersey 25 */
        line-height: 1;
        font-weight: 400;
        text-transform: uppercase;
    }
    .meta-row {
        font-size: 14px;
        color: #555;
        display: flex;
        align-items: center;
        gap: 8px;
        font-family: 'Oswald', sans-serif; /* Readable font for meta */
        letter-spacing: 0.5px;
    }
    .category-tag { color: #b31b34; font-weight: 700; }
    .description {
        margin-top: 24px;
        line-height: 1.5;
        color: #2b0b0b;
        white-space: pre-wrap;
        font-size: 1.2rem;
    }

    /* Info Box */
    .info-box {
        margin-top: 24px;
        background: rgba(255, 255, 255, 0.4); /* Slight tint for legibility */
        border: 2px solid #2b0b0b;
        padding: 12px;
        display: flex;
        gap: 20px;
        flex-wrap: wrap;
        box-shadow: 2px 2px 0px rgba(0,0,0,0.2);
    }
    .info-item .label { font-weight: 700; color: #b31b34; margin-right: 5px; }

    /* Photos */
    .photos-grid {
        margin-top: 24px;
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
        gap: 12px;
    }
    .photo-frame {
        border: 2px solid #2b0b0b;
        box-shadow: 3px 3px 0px rgba(0,0,0,0.3);
        transition: transform 0.2s;
        display: block;
        height: 120px;
        background: #fff;
    }
    .photo-frame:hover { transform: scale(1.05); z-index: 2; }
    .photo-frame img {
        width: 100%;
        height: 100%;
        object-fit: cover;
    }

    .divider { height: 2px; background: rgba(198, 225, 237, 0.8); margin: 24px 0; }

    /* Footer Actions */
    .card-footer {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    
    .upvote-btn {
        display: flex;
        align-items: center;
        gap: 8px;
        background: transparent;
        border: 2px solid transparent;
        cursor: pointer;
        font-size: 1.5rem;
        color: #555;
        padding: 4px 8px;
        transition: all 0.2s;
    }
    .upvote-btn:hover { color: #b31b34; transform: scale(1.1); }
    .upvote-btn.active { color: #e11d48; }
    .upvote-btn svg { width: 32px; height: 32px; }

    /* Sidebar Components */
    h3 { 
        font-size: 1.5rem; 
        color: #2b0b0b; 
        margin: 0 0 16px 0; 
        border-bottom: 2px solid #2b0b0b; 
        padding-bottom: 4px;
        display: inline-block;
    }

    .resolution-banner {
        background: rgba(209, 250, 229, 0.8);
        border-color: #059669;
    }
    .resolution-banner h3 { border-bottom: none; color: #047857; }

    /* Comments */
    .comment-input-area textarea {
        width: 100%;
        background: rgba(255, 255, 255, 0.5);
        border: 2px solid #2b0b0b;
        padding: 10px;
        font-family: inherit;
        font-size: 1rem;
        resize: vertical;
        margin-bottom: 10px;
        border-radius: 0;
    }
    .comment-input-area { margin-bottom: 20px; display: flex; flex-direction: column; align-items: flex-end;}

    .comments-list { display: flex; flex-direction: column; gap: 16px; }
    .comment-item { 
        border-bottom: 1px dashed #2b0b0b; 
        padding-bottom: 12px; 
    }
    .comment-item:last-child { border-bottom: none; }
    .comment-header { display: flex; align-items: center; gap: 8px; margin-bottom: 4px; flex-wrap: wrap;}
    .c-name { font-weight: 700; color: #b31b34; }
    .c-time { font-size: 0.9rem; color: #666; }
    .badge-internal { background: #fef3c7; border: 1px solid #000; padding: 2px 4px; font-size: 10px; }
    .comment-body { font-family: 'Oswald', sans-serif; font-size: 1rem; line-height: 1.4; }

    /* Timeline */
    .timeline { position: relative; padding-left: 10px; margin-top: 10px; }
    .timeline-item { 
        position: relative; 
        padding-left: 25px; 
        padding-bottom: 25px; 
        border-left: 2px solid #2b0b0b; /* Hard timeline line */
    }
    .timeline-item:last-child { border-left: 2px solid transparent; padding-bottom: 0; }
    
    .t-dot { 
        position: absolute; 
        left: -6px; 
        top: 2px; 
        width: 10px; 
        height: 10px; 
        background: #b31b34; 
        border: 2px solid #2b0b0b;
        border-radius: 0; /* Square dots for brutalism */
    }
    .t-status { font-size: 1.1rem; color: #2b0b0b; }
    .t-meta { font-size: 0.9rem; color: #666; margin-top: 2px; font-family: 'Oswald', sans-serif; }
    .t-remarks { 
        margin-top: 6px; 
        font-style: italic; 
        background: rgba(255, 255, 255, 0.5); 
        padding: 6px; 
        border: 1px dashed #2b0b0b;
    }
    
    .loading-state, .error-state {
        text-align: center; margin-top: 100px; font-size: 2rem; color: #2b0b0b;
    }
    .error-state { color: #b31b34; }
</style>