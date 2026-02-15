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

    let grievance = $state<Grievance | null>(null);
    let comments = $state<GrievanceComment[]>([]);
    let history = $state<GrievanceStatusHistory[]>([]);
    let loading = $state(true);
    let error = $state("");

    // --- State for the Inline Form (Replaces Modal) ---
    let showAdminPanel = $state(false);
    let eligibleAssignees = $state<UserResponse[]>([]);
    let selectedStatus = $state("");
    let selectedAssignee = $state("");
    let adminRemarks = $state("");
    let isSaving = $state(false);

    let isUpvoted = $state(false);
    let localUpvoteCount = $state(0);
    let newComment = $state("");
    let submittingComment = $state(false);

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
            if (gRes.data) {
                grievance = gRes.data;
                isUpvoted = grievance.user_has_upvoted;
                localUpvoteCount = grievance.upvote_count;
            }
            if (cRes.data) comments = cRes.data;
            if (hRes.data) history = hRes.data;
        } catch (e: any) {
            error = e.message || "Failed to load grievance details.";
        } finally {
            loading = false;
        }
    }

    async function toggleAdminPanel() {
        if (!showAdminPanel) {
            selectedStatus = grievance?.status || "submitted";
            selectedAssignee = grievance?.assigned_to?.id || "";
            if (eligibleAssignees.length === 0) {
                const res = await api.get("/api/admin/users?limit=100");
                eligibleAssignees = res.data.filter(
                    (u: UserResponse) =>
                        u.role === "faculty" ||
                        u.role === "authority" ||
                        u.role === "admin",
                );
            }
        }
        showAdminPanel = !showAdminPanel;
    }

    async function saveAdminChanges() {
        if (!grievance) return;
        isSaving = true;
        try {
            if (selectedStatus !== grievance.status) {
                await api.put(`/api/grievances/${grievanceId}/status`, {
                    status: selectedStatus,
                    remarks:
                        adminRemarks.trim() === "" ? null : adminRemarks.trim(),
                });
            }
            const currentAssigneeId = grievance.assigned_to?.id || "";
            if (selectedAssignee !== currentAssigneeId) {
                await api.put(`/api/grievances/${grievanceId}/assign`, {
                    assigned_to:
                        selectedAssignee === "" ? null : selectedAssignee,
                    assigned_department: "General",
                });
            }
            await loadAllData();
            showAdminPanel = false;
        } catch (e) {
            alert("Failed to update");
        } finally {
            isSaving = false;
        }
    }

    // Existing helper functions
    async function handleUpvote() {
        if (!grievance) return;
        const previousState = isUpvoted;
        isUpvoted = !isUpvoted;
        localUpvoteCount += isUpvoted ? 1 : -1;
        try {
            await api.post(`/api/grievances/${grievanceId}/upvote`);
        } catch (e) {
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
                comments = [...comments, res.data];
                newComment = "";
            }
        } catch (e: any) {
            alert(e.message || "Failed to post comment");
        } finally {
            submittingComment = false;
        }
    }

    function formatName(
        u: { first_name: string; last_name: string } | null,
    ): string {
        return u ? `${u.first_name} ${u.last_name}` : "Unknown User";
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
            <button
                onclick={() => goto("/dashboard/student/grievances")}
                class="back-btn">← Back to Feed</button
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
                        <span
                            >• {grievance.is_anonymous
                                ? "Anonymous"
                                : formatName(grievance.submitter)} • {formatDate(
                                grievance.created_at,
                            )}</span
                        >
                    </div>
                </div>
                <p class="description">{grievance.description}</p>
                {#if grievance.location_details || grievance.assigned_department}
                    <div class="info-box">
                        {#if grievance.location_details}<div>
                                <b>LOCATION:</b>
                                {grievance.location_details}
                            </div>{/if}
                        {#if grievance.assigned_department}<div>
                                <b>DEPT:</b>
                                {grievance.assigned_department}
                            </div>{/if}
                    </div>
                {/if}
                {#if grievance.photo_urls && grievance.photo_urls.length > 0}
                    <div class="photos-grid">
                        {#each grievance.photo_urls as photo}
                            <a href={photo} target="_blank" rel="noreferrer"
                                ><img src={photo} alt="Evidence" /></a
                            >
                        {/each}
                    </div>
                {/if}
                <div class="divider"></div>
                <div class="card-footer">
                    <button
                        class="upvote-btn {isUpvoted ? 'active' : ''}"
                        onclick={handleUpvote}
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill={isUpvoted ? "currentColor" : "none"}
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                            class="w-5 h-5"
                            ><path
                                d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z"
                            /></svg
                        >
                        <span>{localUpvoteCount}</span>
                    </button>
                </div>
            </div>

            <div class="sidebar">
                {#if isAdmin}
                    <div class="section-card admin-inline-card">
                        <div class="sidebar-header">
                            <h3>Admin Control</h3>
                            <button
                                class="toggle-link"
                                onclick={toggleAdminPanel}
                            >
                                {showAdminPanel ? "[ CLOSE ]" : "[ OPEN ]"}
                            </button>
                        </div>
                        {#if showAdminPanel}
                            <div class="admin-form">
                                <div class="field">
                                    <label>Status</label>
                                    <select bind:value={selectedStatus}>
                                        <option value="submitted"
                                            >SUBMITTED</option
                                        >
                                        <option value="under_review"
                                            >UNDER REVIEW</option
                                        >
                                        <option value="in_progress"
                                            >IN PROGRESS</option
                                        >
                                        <option value="resolved"
                                            >RESOLVED</option
                                        >
                                        <option value="closed">CLOSED</option>
                                    </select>
                                </div>
                                <div class="field">
                                    <label>Assign To</label>
                                    <select bind:value={selectedAssignee}>
                                        <option value=""
                                            >-- UNASSIGNED --</option
                                        >
                                        {#each eligibleAssignees as user}
                                            <option value={user.id}
                                                >[{user.role.toUpperCase()}] {user.first_name}
                                                {user.last_name}</option
                                            >
                                        {/each}
                                    </select>
                                </div>
                                <div class="field">
                                    <label>Remarks</label>
                                    <textarea
                                        bind:value={adminRemarks}
                                        rows="3"
                                        placeholder="Notes..."
                                    ></textarea>
                                </div>
                                <button
                                    class="retro-btn execute"
                                    onclick={saveAdminChanges}
                                    disabled={isSaving}
                                >
                                    {isSaving ? "SAVING..." : "CONFIRM_CHANGES"}
                                </button>
                            </div>
                        {/if}
                    </div>
                {/if}

                {#if grievance.status === "resolved"}
                    <div class="resolution-banner">
                        <h3>✓ Resolved</h3>
                        <p>{grievance.resolution_notes || "Fixed."}</p>
                    </div>
                {/if}
                <div class="section-card">
                    <h3>Discussion ({comments.length})</h3>
                    <div class="comment-input-area">
                        <textarea
                            bind:value={newComment}
                            placeholder="Add a comment..."
                        ></textarea>
                        <button
                            disabled={submittingComment || !newComment.trim()}
                            onclick={handlePostComment}>Post</button
                        >
                    </div>
                    <div class="comments-list mt-10">
                        {#each comments as comment}
                            <div class="comment-item">
                                <div class="text-[11px] font-bold uppercase">
                                    {formatName(comment.user)} • {formatDate(
                                        comment.created_at,
                                    )}
                                </div>
                                <div class="text-sm">{comment.comment}</div>
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
                                <div class="text-xs">
                                    <b>{event.new_status.replace("_", " ")}</b>
                                    by {formatName(event.updated_by)}
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
    :global(body) {
        background-color: transparent;
    }
    .page-container {
        max-width: 1000px;
        margin: 0 auto;
        padding: 20px;
    }
    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 20px;
    }
    .status-badge {
        padding: 4px 12px;
        border-radius: 20px;
        font-size: 10px;
        font-weight: 900;
        text-transform: uppercase;
        border: 1px solid currentColor;
    }
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
    }

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

    .main-card,
    .section-card,
    .resolution-banner {
        border: 2px solid rgba(198, 225, 237, 0.6);
        box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.1);
        padding: 20px;
        background: transparent;
    }

    /* Admin Inline Form Styles */
    .admin-inline-card {
        border: 2px solid #2b0b0b !important;
        box-shadow: 4px 4px 0px #000 !important;
    }
    .sidebar-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .toggle-link {
        background: none;
        border: none;
        font-weight: 900;
        color: #b31b34;
        cursor: pointer;
        font-size: 10px;
    }
    .admin-form {
        display: flex;
        flex-direction: column;
        gap: 12px;
        margin-top: 15px;
        border-top: 2px dashed rgba(0, 0, 0, 0.1);
        padding-top: 15px;
    }
    .field label {
        display: block;
        font-size: 9px;
        font-weight: 900;
        text-transform: uppercase;
        margin-bottom: 4px;
    }
    .field select,
    .field textarea {
        width: 100%;
        border: 2px solid #2b0b0b;
        background: transparent;
        padding: 8px;
        font-family: inherit;
        font-size: 12px;
    }
    .retro-btn.execute {
        background: #b31b34;
        color: white;
        border: 2px solid #2b0b0b;
        padding: 10px;
        font-weight: 900;
        box-shadow: 4px 4px 0px #000;
        cursor: pointer;
    }

    .title {
        color: #2b0b0b;
        font-size: 24px;
        font-weight: 900;
        text-transform: uppercase;
        line-height: 1;
        margin-bottom: 8px;
    }
    .meta-row {
        font-size: 11px;
        font-weight: 700;
        color: #666;
        text-transform: uppercase;
    }
    .category-tag {
        color: #b31b34;
    }
    .description {
        margin-top: 20px;
        line-height: 1.5;
        white-space: pre-wrap;
        font-size: 14px;
    }
    .info-box {
        margin-top: 15px;
        border-left: 4px solid #b31b34;
        padding-left: 10px;
        font-size: 12px;
    }
    .photos-grid {
        margin-top: 20px;
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
        gap: 10px;
    }
    .photos-grid img {
        width: 100%;
        height: 100px;
        object-fit: cover;
        border: 2px solid #2b0b0b;
    }
    .divider {
        height: 2px;
        background: #2b0b0b;
        margin: 20px 0;
        opacity: 0.1;
    }
    .card-footer {
        display: flex;
        justify-content: space-between;
    }
    .upvote-btn {
        display: flex;
        align-items: center;
        gap: 4px;
        font-weight: 900;
        color: #666;
        background: none;
        border: none;
    }
    .upvote-btn.active {
        color: #e11d48;
    }

    .sidebar {
        display: flex;
        flex-direction: column;
        gap: 20px;
    }
    .comment-input-area textarea {
        width: 100%;
        border: 2px solid #2b0b0b;
        background: transparent;
        padding: 10px;
        font-family: inherit;
        font-size: 13px;
        box-sizing: border-box;
    }
    .comment-input-area button {
        background: #b31b34;
        color: white;
        padding: 4px 12px;
        font-weight: 900;
        text-transform: uppercase;
        border: 2px solid #2b0b0b;
        margin-top: 5px;
        float: right;
        box-shadow: 3px 3px 0px #000;
    }
    .timeline-item {
        position: relative;
        padding-left: 20px;
        padding-bottom: 15px;
        border-left: 2px solid #2b0b0b;
        margin-left: 5px;
    }
    .t-dot {
        position: absolute;
        left: -6px;
        top: 0;
        width: 10px;
        height: 10px;
        background: #2b0b0b;
        border-radius: 50%;
    }
</style>
