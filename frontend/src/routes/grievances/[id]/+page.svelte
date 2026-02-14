<script lang="ts">
    import { page } from "$app/stores";
    import { user } from "$lib/auth";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import type { UserResponse } from "$lib/types";
    import api from "$lib/api";
    import type {
        Grievance,
        GrievanceComment,
        GrievanceStatusHistory,
        ApiResponse,
    } from "$lib/types";

    // --- State (Svelte 5 Runes) ---
    let grievance = $state<Grievance | null>(null);
    let comments = $state<GrievanceComment[]>([]);
    let history = $state<GrievanceStatusHistory[]>([]);

    let loading = $state(true);
    let error = $state("");

    let showAdminModal = $state(false);
    let eligibleAssignees = $state<UserResponse[]>([]);
    let selectedStatus = $state("");
    let selectedAssignee = $state("");
    let adminRemarks = $state("");
    let isSaving = $state(false);

    // Optimistic UI State
    let isUpvoted = $state(false);
    let localUpvoteCount = $state(0);

    // Comment Input State
    let newComment = $state("");
    let submittingComment = $state(false);

    // Derived Values
    const grievanceId = $derived($page.params.id);
    const currentUser = $derived($user);
    const isAdmin = $derived(
        currentUser?.role === "admin" || currentUser?.role === "authority",
    );

    onMount(async () => {
        if (!currentUser) {
            goto("/login");
            return;
        }
        await loadAllData();
    });

    async function loadAllData() {
        loading = true;
        try {
            // Fetch everything in parallel
            const [gRes, cRes, hRes] = await Promise.all([
                api.get<ApiResponse<Grievance>>(
                    `/api/grievances/${grievanceId}`,
                ),
                api.get<ApiResponse<GrievanceComment[]>>(
                    `/api/grievances/${grievanceId}/comments`,
                ),
                api.get<ApiResponse<GrievanceStatusHistory[]>>(
                    `/api/grievances/${grievanceId}/history`,
                ),
            ]);

            // Set Grievance Data
            if (gRes.data) {
                grievance = gRes.data;
                // Sync local state for optimistic UI
                isUpvoted = grievance.user_has_upvoted;
                localUpvoteCount = grievance.upvote_count;
            }

            // Set Comments & History
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

        // 1. Optimistic Update (Update UI immediately)
        const previousState = isUpvoted;
        isUpvoted = !isUpvoted;
        localUpvoteCount += isUpvoted ? 1 : -1;

        try {
            // 2. Send request in background
            await api.post(`/api/grievances/${grievanceId}/upvote`);
        } catch (e) {
            // 3. Revert on failure
            console.error("Upvote failed", e);
            isUpvoted = previousState;
            localUpvoteCount += isUpvoted ? 1 : -1;
        }
    }

    async function handlePostComment() {
        if (!newComment.trim()) return;

        submittingComment = true;
        try {
            const res = await api.post<ApiResponse<GrievanceComment>>(
                `/api/grievances/${grievanceId}/comments`,
                {
                    comment: newComment,
                    is_internal: false,
                },
            );

            if (res.data) {
                comments = [...comments, res.data]; // Append new comment
                newComment = "";
            }
        } catch (e: any) {
            alert(e.message || "Failed to post comment");
        } finally {
            submittingComment = false;
        }
    }

    async function openAdminModal() {
        if (!isAdmin) return;
        selectedStatus = grievance?.status || "submitted";
        selectedAssignee = grievance?.assigned_to?.id || "";
        showAdminModal = true;

        // Fetch eligible users (Faculty & Authority)
        if (eligibleAssignees.length === 0) {
            try {
                // We fetch all users and filter client-side for simplicity,
                // or you can add ?role=faculty to your backend later
                const res = await api.get("/api/admin/users?limit=100");
                if (res.data) {
                    eligibleAssignees = res.data.filter(
                        (u: UserResponse) =>
                            u.role === "faculty" ||
                            u.role === "authority" ||
                            u.role === "admin",
                    );
                }
            } catch (e) {
                console.error("Failed to load assignees", e);
            }
        }
    }

    async function saveAdminChanges() {
        if (!grievance) return;
        isSaving = true;
        try {
            // 1. Update Status
            if (selectedStatus !== grievance.status) {
                await api.put(`/api/grievances/${grievanceId}/status`, {
                    status: selectedStatus,
                    // FIX: Ensure empty strings become strict null for Rust
                    remarks:
                        adminRemarks.trim() === "" ? null : adminRemarks.trim(),
                });
            }

            // 2. Update Assignment (if changed)
            const currentAssigneeId = grievance.assigned_to?.id || "";
            if (selectedAssignee !== currentAssigneeId) {
                await api.put(`/api/grievances/${grievanceId}/assign`, {
                    // FIX: Ensure empty strings become strict null for Rust
                    assigned_to:
                        selectedAssignee === "" ? null : selectedAssignee,
                    assigned_department: "General",
                });
            }

            // Reload page data
            await loadAllData();
            showAdminModal = false;
        } catch (e) {
            alert("Failed to update grievance");
            console.error(e);
        } finally {
            isSaving = false;
        }
    }

    // Helper functions
    function formatName(
        u: { first_name: string; last_name: string } | null,
    ): string {
        if (!u) return "Unknown User";
        return `${u.first_name} ${u.last_name}`;
    }

    function formatDate(d: string) {
        return new Date(d).toLocaleDateString("en-US", {
            month: "short",
            day: "numeric",
            hour: "2-digit",
            minute: "2-digit",
        });
    }
</script>

<div class="page-container">
    {#if loading}
        <div class="loading-state">Loading details...</div>
    {:else if error}
        <div class="error-state">{error}</div>
    {:else if grievance}
        <div class="header">
            <button onclick={() => goto("/grievances")} class="back-btn"
                >← Back to Feed</button
            >
            <div class="status-badge {grievance.status}">
                {grievance.status.replace("_", " ")}
            </div>
        </div>

        <div class="grid-layout">
            <div class="main-card">
                <div class="content-header">
                    <h1 class="title">{grievance.title}</h1>
                    <div class="meta-row">
                        <span class="category-tag"
                            >[{grievance.category.toUpperCase()}]</span
                        >
                        <span class="dot">•</span>
                        <span class="user-name">
                            {grievance.is_anonymous
                                ? "Anonymous Student"
                                : formatName(grievance.submitter)}
                        </span>
                        <span class="dot">•</span>
                        <span class="date"
                            >{formatDate(grievance.created_at)}</span
                        >
                    </div>
                </div>

                <p class="description">{grievance.description}</p>

                {#if grievance.location_details || grievance.assigned_department}
                    <div class="info-box">
                        {#if grievance.location_details}
                            <div class="info-item">
                                <span class="label">LOCATION:</span>
                                {grievance.location_details}
                            </div>
                        {/if}
                        {#if grievance.assigned_department}
                            <div class="info-item">
                                <span class="label">ASSIGNED DEPT:</span>
                                {grievance.assigned_department}
                            </div>
                        {/if}
                    </div>
                {/if}

                {#if grievance.photo_urls && grievance.photo_urls.length > 0}
                    <div class="photos-grid">
                        {#each grievance.photo_urls as photo}
                            <a href={photo} target="_blank" rel="noreferrer">
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
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill={isUpvoted ? "currentColor" : "none"}
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z"
                            />
                        </svg>
                        <span>{localUpvoteCount}</span>
                    </button>

                    {#if isAdmin}
                        <div class="admin-controls">
                            <button class="btn-admin" onclick={openAdminModal}
                                >Manage Status</button
                            >
                        </div>
                    {/if}
                </div>
            </div>

            <div class="sidebar">
                {#if grievance.status === "resolved"}
                    <div class="resolution-banner">
                        <h3>✓ Resolved</h3>
                        <p>
                            {grievance.resolution_notes ||
                                "No resolution notes provided."}
                        </p>
                        <small
                            >Resolved by {formatName(
                                grievance.assigned_to,
                            )}</small
                        >
                    </div>
                {/if}

                <div class="section-card">
                    <h3>Discussion ({comments.length})</h3>

                    <div class="comment-input-area">
                        <textarea
                            bind:value={newComment}
                            placeholder="Add a comment..."
                            rows="2"
                        ></textarea>
                        <button
                            disabled={submittingComment || !newComment.trim()}
                            onclick={handlePostComment}
                        >
                            {submittingComment ? "Posting..." : "Post"}
                        </button>
                    </div>

                    <div class="comments-list">
                        {#each comments as comment}
                            <div
                                class="comment-item {comment.is_internal
                                    ? 'internal'
                                    : ''}"
                            >
                                <div class="comment-header">
                                    <span class="c-name"
                                        >{formatName(comment.user)}</span
                                    >
                                    {#if comment.is_internal}<span
                                            class="badge-internal"
                                            >INTERNAL</span
                                        >{/if}
                                    <span class="c-time"
                                        >{formatDate(comment.created_at)}</span
                                    >
                                </div>
                                <div class="comment-body">
                                    {comment.comment}
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>

                <div class="section-card">
                    <h3>Timeline</h3>
                    <div class="timeline">
                        {#each history as event}
                            <div class="timeline-item">
                                <div class="t-dot"></div>
                                <div class="t-content">
                                    <div class="t-status">
                                        Changed to <b
                                            >{event.new_status.replace(
                                                "_",
                                                " ",
                                            )}</b
                                        >
                                    </div>
                                    <div class="t-meta">
                                        by {formatName(event.updated_by)} • {formatDate(
                                            event.created_at,
                                        )}
                                    </div>
                                    {#if event.remarks}
                                        <div class="t-remarks">
                                            "{event.remarks}"
                                        </div>
                                    {/if}
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>
            </div>
        </div>
    {/if}
    {#if showAdminModal}
        <div class="modal-overlay">
            <div class="brutalist-card modal-box">
                <h3>ADMIN COMMAND</h3>

                <div class="field">
                    <!-- svelte-ignore a11y_label_has_associated_control -->
                    <label>STATUS</label>
                    <select bind:value={selectedStatus}>
                        <option value="submitted">SUBMITTED</option>
                        <option value="under_review">UNDER REVIEW</option>
                        <option value="in_progress">IN PROGRESS</option>
                        <option value="resolved">RESOLVED</option>
                        <option value="closed">CLOSED</option>
                    </select>
                </div>

                <div class="field">
                    <!-- svelte-ignore a11y_label_has_associated_control -->
                    <label>ASSIGN TO</label>
                    <select bind:value={selectedAssignee}>
                        <option value="">-- UNASSIGNED --</option>
                        {#each eligibleAssignees as user}
                            <option value={user.id}>
                                [{user.role.toUpperCase()}] {user.first_name}
                                {user.last_name}
                            </option>
                        {/each}
                    </select>
                </div>

                <div class="field">
                    <label>REMARKS</label>
                    <textarea
                        bind:value={adminRemarks}
                        rows="3"
                        placeholder="Add a note about this status change..."
                    ></textarea>
                </div>

                <div class="modal-actions">
                    <button
                        class="retro-btn"
                        onclick={() => (showAdminModal = false)}>CANCEL</button
                    >
                    <button
                        class="retro-btn admin"
                        onclick={saveAdminChanges}
                        disabled={isSaving}
                    >
                        {isSaving ? "SAVING..." : "CONFIRM"}
                    </button>
                </div>
            </div>
        </div>
    {/if}
</div>

<style>
    :global(body) {
        background-color: transparent; /* Changed to transparent for notebook bg */
    }

    .page-container {
        max-width: 1000px;
        margin: 0 auto;
        padding: 20px;
    }

    /* Header */
    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 20px;
    }
    .back-btn {
        background: none;
        border: none;
        color: #666;
        cursor: pointer;
        font-weight: 600;
        font-size: 14px;
    }
    .back-btn:hover {
        text-decoration: underline;
    }

    .status-badge {
        padding: 6px 12px;
        border-radius: 20px;
        font-size: 12px;
        font-weight: 700;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }
    /* Status Colors */
    .submitted {
        background: #e0f2fe;
        color: #0284c7;
    }
    .under_review {
        background: #ffedd5;
        color: #c2410c;
    }
    .in_progress {
        background: #dcfce7;
        color: #15803d;
    }
    .resolved {
        background: #d1fae5;
        color: #047857;
        border: 1px solid #10b981;
    }
    .rejected {
        background: #fee2e2;
        color: #b91c1c;
    }

    /* Layout */
    .grid-layout {
        display: grid;
        grid-template-columns: 1fr 340px;
        gap: 20px;
    }
    @media (max-width: 800px) {
        .grid-layout {
            grid-template-columns: 1fr;
        }
    }

    /* Main Card */
    .main-card {
        background: transparent;
        border: 2px solid rgba(198, 225, 237, 0.6);
        box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.1);
        padding: 24px;
    }
    .title {
        margin: 0 0 10px 0;
        color: #2b0b0b;
        font-size: 22px;
    }
    .meta-row {
        font-size: 12px;
        color: #666;
        display: flex;
        align-items: center;
        gap: 8px;
    }
    .category-tag {
        color: #b31b34;
        font-weight: 700;
    }
    .description {
        margin-top: 20px;
        line-height: 1.6;
        color: #333;
        white-space: pre-wrap;
    }

    .info-box {
        margin-top: 20px;
        background: transparent; /* Changed to transparent */
        border: 2px solid rgba(198, 225, 237, 0.6); /* Match main card border */
        padding: 12px;
        border-radius: 4px;
        font-size: 13px;
    }
    .info-item .label {
        font-weight: 700;
        color: #64748b;
        margin-right: 5px;
    }

    .photos-grid {
        margin-top: 20px;
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
        gap: 8px;
    }
    .photos-grid img {
        width: 100%;
        height: 100px;
        object-fit: cover;
        border-radius: 4px;
        border: 1px solid #eee;
    }

    .divider {
        height: 1px;
        background: #eee;
        margin: 20px 0;
    }

    .card-footer {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .upvote-btn {
        display: flex;
        align-items: center;
        gap: 6px;
        background: transparent;
        border: none;
        cursor: pointer;
        font-size: 14px;
        font-weight: 600;
        color: #666;
        transition: transform 0.1s;
    }
    .upvote-btn:hover {
        color: #b31b34;
        transform: scale(1.05);
    }
    .upvote-btn.active {
        color: #e11d48;
    }
    .upvote-btn svg {
        width: 20px;
        height: 20px;
    }

    .btn-admin {
        background: #1e293b;
        color: white;
        border: none;
        padding: 6px 12px;
        border-radius: 4px;
        font-size: 12px;
        cursor: pointer;
    }

    /* Sidebar */
    .sidebar {
        display: flex;
        flex-direction: column;
        gap: 20px;
    }

    .section-card {
        background: transparent; /* Changed to transparent */
        border: 2px solid rgba(198, 225, 237, 0.6); /* Match main card border */
        box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.1); /* Match main card shadow */
        padding: 16px;
        border-radius: 4px;
    }
    h3 {
        font-size: 14px;
        text-transform: uppercase;
        color: #64748b;
        margin: 0 0 12px 0;
    }

    .resolution-banner {
        background: transparent; /* Changed to transparent */
        border: 2px solid #10b981; /* Keep green border */
        box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.1); /* Match shadow */
        padding: 16px;
        border-radius: 4px;
    }
    .resolution-banner h3 {
        color: #047857;
        margin-bottom: 5px;
    }
    .resolution-banner p {
        margin: 0 0 8px 0;
        font-size: 13px;
        color: #065f46;
    }
    .resolution-banner small {
        font-size: 11px;
        color: #059669;
    }

    /* Comments */
    .comment-input-area textarea {
        width: 100%;
        background: transparent; /* Changed to transparent */
        border: 2px solid rgba(198, 225, 237, 0.6); /* Match border style */
        border-radius: 4px;
        padding: 8px;
        font-family: inherit;
        resize: vertical;
        margin-bottom: 5px;
        box-sizing: border-box; /* FIX: Ensures padding doesn't push width over 100% */
    }
    .comment-input-area button {
        background: #b31b34;
        color: white; /* FIX: Changed from transparent to white */
        border: none;
        padding: 4px 12px;
        border-radius: 2px;
        font-size: 12px;
        cursor: pointer;
        float: right;
    }

    .comments-list {
        margin-top: 40px;
        display: flex;
        flex-direction: column;
        gap: 12px;
    }
    .comment-item {
        font-size: 13px;
        border-bottom: 1px solid #f1f5f9;
        padding-bottom: 8px;
    }
    .comment-item:last-child {
        border-bottom: none;
    }
    .comment-header {
        display: flex;
        align-items: center;
        gap: 6px;
        margin-bottom: 2px;
    }
    .c-name {
        font-weight: 700;
        color: #334155;
    }
    .c-time {
        font-size: 11px;
        color: #94a3b8;
    }
    .badge-internal {
        background: #fef3c7;
        color: #b45309;
        font-size: 9px;
        padding: 1px 4px;
        border-radius: 2px;
    }

    /* Timeline */
    .timeline {
        position: relative;
        padding-left: 8px;
    }
    .timeline-item {
        position: relative;
        padding-left: 20px;
        padding-bottom: 20px;
        border-left: 2px solid #e2e8f0;
    }
    .timeline-item:last-child {
        border-left: none;
        padding-bottom: 0;
    }
    .t-dot {
        position: absolute;
        left: -5px;
        top: 0;
        width: 8px;
        height: 8px;
        background: #cbd5e1;
        border-radius: 50%;
    }
    .t-status {
        font-size: 12px;
        color: #334155;
    }
    .t-meta {
        font-size: 11px;
        color: #94a3b8;
        margin-top: 2px;
    }
    .t-remarks {
        margin-top: 4px;
        font-style: italic;
        color: #64748b;
        font-size: 12px;
        background: transparent;
        padding: 4px;
    }

    .loading-state,
    .error-state {
        text-align: center;
        margin-top: 50px;
        font-size: 16px;
        color: #666;
    }
    .error-state {
        color: #dc2626;
    }

    /* Modal Styles */
    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.6);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
        backdrop-filter: blur(2px);
    }
    .modal-box {
        background: #f0f2f5; /* Opaque background for readability */
        width: 90%;
        max-width: 500px;
        color: #2b0b0b;
    }
    .field {
        margin-bottom: 16px;
    }
    .field label {
        display: block;
        font-weight: 700;
        margin-bottom: 6px;
        font-size: 0.9rem;
    }
    .field select,
    .field textarea {
        width: 100%;
        padding: 8px;
        background: transparent;
        border: 2px solid #2b0b0b;
        font-family: inherit;
        font-size: 1rem;
    }
    .modal-actions {
        display: flex;
        justify-content: flex-end;
        gap: 10px;
        margin-top: 20px;
    }
</style>
