<script lang="ts">
    import { user } from "$lib/auth";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import api from "$lib/api";
    import type { Grievance, ApiResponse } from "$lib/types";
    import PostCard from "$lib/components/PostCard.svelte";

    let grievances = $state<Grievance[]>([]);
    let loading = $state(true);

    // --- Filter State ---
    let searchQuery = $state("");
    let filterStatus = $state("");
    let filterCategory = $state("");
    let filterPriority = $state("");

    // Derived filtered list
    let filteredGrievances = $derived(
        grievances.filter((g) => {
            const matchesSearch =
                !searchQuery ||
                g.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
                g.description.toLowerCase().includes(searchQuery.toLowerCase());
            const matchesStatus = !filterStatus || g.status === filterStatus;
            const matchesCategory =
                !filterCategory || g.category === filterCategory;
            const matchesPriority =
                !filterPriority || g.priority === filterPriority;
            return (
                matchesSearch &&
                matchesStatus &&
                matchesCategory &&
                matchesPriority
            );
        }),
    );

    onMount(async () => {
        if (!$user) {
            goto("/login");
            return;
        }
        await loadGrievances();
    });

    async function loadGrievances() {
        loading = true;
        try {
            const response =
                await api.get<ApiResponse<Grievance[]>>("/api/grievances?");
            grievances = response.data || [];
        } catch (err: any) {
            console.error("[GRIEVANCES] Error loading:", err);
            grievances = [];
        } finally {
            loading = false;
        }
    }

    async function handleUpvote(grievanceId: string) {
        grievances = grievances.map((g) => {
            if (g.id === grievanceId) {
                const newLikedState = !g.user_has_upvoted;
                return {
                    ...g,
                    user_has_upvoted: newLikedState,
                    upvote_count: g.upvote_count + (newLikedState ? 1 : -1),
                };
            }
            return g;
        });

        try {
            await api.post(`/api/grievances/${grievanceId}/upvote`);
        } catch (err) {
            console.error("[GRIEVANCES] Upvote failed, reverting:", err);
            await loadGrievances();
        }
    }

    function getSubmitterName(g: Grievance): string {
        if (g.is_anonymous) return "Anonymous";
        return g.submitter
            ? `${g.submitter.first_name} ${g.submitter.last_name}`
            : "Unknown";
    }

    function formatStatus(status: string): string {
        return status
            .split("_")
            .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
            .join(" ");
    }

    function formatCategory(category: string): string {
        return category.charAt(0).toUpperCase() + category.slice(1);
    }
</script>

<div class="container">
    <div class="w-full max-w-[400px] flex flex-col gap-4 mb-4 px-2">
        <h1
            class="text-4xl font-bold text-[#2b0b0b] self-start tracking-tighter uppercase"
        >
            Issues
        </h1>

        <div class="grid grid-cols-2 gap-2 w-full">
            <input
                type="text"
                bind:value={searchQuery}
                placeholder="SEARCH..."
                class="col-span-2 bg-transparent border-2 border-[rgba(198,225,237,0.6)] p-2 text-xs uppercase focus:outline-none focus:border-[#b31b34] placeholder:text-[#6e0f1c]/50"
            />

            <select
                bind:value={filterCategory}
                class="bg-transparent border-2 border-[rgba(198,225,237,0.6)] p-2 text-[10px] uppercase focus:outline-none appearance-none"
            >
                <option value="">ALL CATEGORIES</option>
                <option value="infrastructure">INFRASTRUCTURE</option>
                <option value="academics">ACADEMICS</option>
                <option value="hostel">HOSTEL</option>
                <option value="food">FOOD</option>
                <option value="other">OTHER</option>
            </select>

            <select
                bind:value={filterStatus}
                class="bg-transparent border-2 border-[rgba(198,225,237,0.6)] p-2 text-[10px] uppercase focus:outline-none appearance-none"
            >
                <option value="">ALL STATUSES</option>
                <option value="submitted">SUBMITTED</option>
                <option value="under_review">UNDER REVIEW</option>
                <option value="in_progress">IN PROGRESS</option>
                <option value="resolved">RESOLVED</option>
                <option value="closed">CLOSED</option>
            </select>

            <select
                bind:value={filterPriority}
                class="bg-transparent border-2 border-[rgba(198,225,237,0.6)] p-2 text-[10px] uppercase focus:outline-none appearance-none col-span-2"
            >
                <option value="">ALL PRIORITIES</option>
                <option value="low">LOW</option>
                <option value="medium">MEDIUM</option>
                <option value="high">HIGH</option>
                <option value="urgent">URGENT</option>
            </select>
        </div>
    </div>

    <button
        class="submit-btn"
        onclick={() => goto("/grievances/submit")}
        aria-label="Submit new grievance"
    >
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="2"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M12 4.5v15m7.5-7.5h-15"
            />
        </svg>
    </button>

    {#if loading}
        <div class="loading">Loading...</div>
    {:else}
        {#each filteredGrievances as grievance}
            <PostCard
                id={grievance.id}
                username={getSubmitterName(grievance)}
                title={grievance.title}
                content={grievance.description}
                images={grievance.photo_urls || []}
                upvotes={grievance.upvote_count}
                isLiked={grievance.user_has_upvoted}
                commentsCount={0}
                date={grievance.created_at}
                status={formatStatus(grievance.status)}
                category={formatCategory(grievance.category)}
                onclick={() => goto(`/grievances/${grievance.id}`)}
                onupvote={() => handleUpvote(grievance.id)}
            />
        {/each}

        {#if filteredGrievances.length === 0}
            <div class="text-[#6e0f1c] uppercase text-sm mt-8 opacity-50">
                No issues found
            </div>
        {/if}
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

    /* Ensure select options are readable against your theme background */
    option {
        background-color: #f0f2f5;
        color: #2b0b0b;
    }
</style>
