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
    <div class="capybaraloader">
        <div class="capybara">
            <div class="capyhead">
                <div class="capyear"><div class="capyear2"></div></div>
                <div class="capyear"><div class="capyear2"></div></div>
                <div class="capyeye"></div>
                <div class="capyeye"></div>
                <div class="capymouth">
                    <div class="capylips"></div>
                    <div class="capylips"></div>
                </div>
            </div>
            <div class="capy"></div>
            <div class="capyleg"></div>
            <div class="capyleg2"></div>
            <div class="capyleg2"></div>
        </div>
        <div class="loader">
            <div class="loaderline"></div>
        </div>
    </div>
    <div class="w-100 flex flex-col items-center gap-4 mb-8 px-2">
        <h1
            class="text-[12rem] font-bold text-[#2b0b0b] tracking-tighter uppercase text-center w-full"
        >
            Issues
        </h1>

        <div class="flex flex-col gap-2 w-full">
            <div class="group-search">
                <svg class="icon-search" aria-hidden="true" viewBox="0 0 24 24">
                    <g
                        ><path
                            d="M21.53 20.47l-3.66-3.66C19.195 15.24 20 13.214 20 11c0-4.97-4.03-9-9-9s-9 4.03-9 9 4.03 9 9 9c2.215 0 4.24-.804 5.808-2.13l3.66 3.66c.147.146.34.22.53.22s.385-.073.53-.22c.295-.293.295-.767.002-1.06zM3.5 11c0-4.135 3.365-7.5 7.5-7.5s7.5 3.365 7.5 7.5-3.365 7.5-7.5 7.5-7.5-3.365-7.5-7.5z"
                        ></path></g
                    >
                </svg>
                <input
                    type="text"
                    bind:value={searchQuery}
                    placeholder="SEARCH ISSUES..."
                    class="input-search"
                />
            </div>

            <div class="grid grid-cols-2 gap-2 w-full">
                <select bind:value={filterCategory} class="filter-element">
                    <option value="">ALL CATEGORIES</option>
                    <option value="infrastructure">INFRASTRUCTURE</option>
                    <option value="academics">ACADEMICS</option>
                    <option value="hostel">HOSTEL</option>
                    <option value="food">FOOD</option>
                    <option value="other">OTHER</option>
                </select>

                <select bind:value={filterStatus} class="filter-element">
                    <option value="">ALL STATUSES</option>
                    <option value="submitted">SUBMITTED</option>
                    <option value="under_review">UNDER REVIEW</option>
                    <option value="in_progress">IN PROGRESS</option>
                    <option value="resolved">RESOLVED</option>
                    <option value="closed">CLOSED</option>
                </select>

                <select
                    bind:value={filterPriority}
                    class="filter-element col-span-2"
                >
                    <option value="">ALL PRIORITIES</option>
                    <option value="low">LOW</option>
                    <option value="medium">MEDIUM</option>
                    <option value="high">HIGH</option>
                    <option value="urgent">URGENT</option>
                </select>
            </div>
        </div>
    </div>
    <button
        class="submit-btn"
        onclick={() => goto("/dashboard/student/grievances/submit")}
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
                onclick={() =>
                    goto(`/dashboard/student/grievances/${grievance.id}`)}
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
    .capybaraloader {
        width: 14em;
        height: 10em;
        position: relative;
        z-index: 1;
        --color: rgb(204, 125, 45);
        --color2: rgb(83, 56, 28);
        transform: scale(0.75);
    }
    .capybara {
        width: 100%;
        height: 7.5em;
        position: relative;
        z-index: 1;
    }
    .loader {
        width: 100%;
        height: 2.5em;
        position: relative;
        z-index: 1;
        overflow: hidden;
    }
    .capy {
        width: 85%;
        height: 100%;
        background: linear-gradient(var(--color), 90%, var(--color2));
        border-radius: 45%;
        position: relative;
        z-index: 1;
        animation: movebody 1s linear infinite;
    }
    .capyhead {
        width: 7.5em;
        height: 7em;
        bottom: 0em;
        right: 0em;
        position: absolute;
        background-color: var(--color);
        z-index: 3;
        border-radius: 3.5em;
        box-shadow: -1em 0em var(--color2);
        animation: movebody 1s linear infinite;
    }
    .capyear {
        width: 2em;
        height: 2em;
        background: linear-gradient(-45deg, var(--color), 90%, var(--color2));
        top: 0em;
        left: 0em;
        border-radius: 100%;
        position: absolute;
        overflow: hidden;
        z-index: 3;
    }
    .capyear:nth-child(2) {
        left: 5em;
        background: linear-gradient(25deg, var(--color), 90%, var(--color2));
    }
    .capyear2 {
        width: 100%;
        height: 1em;
        background-color: var(--color2);
        bottom: 0em;
        left: 0.5em;
        border-radius: 100%;
        position: absolute;
        transform: rotate(-45deg);
    }
    .capymouth {
        width: 3.5em;
        height: 2em;
        background-color: var(--color2);
        position: absolute;
        bottom: 0em;
        left: 2.5em;
        border-radius: 50%;
        display: flex;
        justify-content: space-around;
        align-items: center;
        padding: 0.5em;
    }
    .capylips {
        width: 0.25em;
        height: 0.75em;
        border-radius: 100%;
        transform: rotate(-45deg);
        background-color: var(--color);
    }
    .capylips:nth-child(2) {
        transform: rotate(45deg);
    }
    .capyeye {
        width: 2em;
        height: 0.5em;
        background-color: var(--color2);
        position: absolute;
        bottom: 3.5em;
        left: 1.5em;
        border-radius: 5em;
        transform: rotate(45deg);
    }
    .capyeye:nth-child(4) {
        transform: rotate(-45deg);
        left: 5.5em;
        width: 1.75em;
    }
    .capyleg {
        width: 6em;
        height: 5em;
        bottom: 0em;
        left: 0em;
        position: absolute;
        background: linear-gradient(var(--color), 95%, var(--color2));
        z-index: 2;
        border-radius: 2em;
        animation: movebody 1s linear infinite;
    }
    .capyleg2 {
        width: 1.75em;
        height: 3em;
        bottom: 0em;
        left: 3.25em;
        position: absolute;
        background: linear-gradient(var(--color), 80%, var(--color2));
        z-index: 2;
        border-radius: 0.75em;
        box-shadow: inset 0em -0.5em var(--color2);
        animation: moveleg 1s linear infinite;
    }
    .capyleg2:nth-child(3) {
        width: 1.25em;
        left: 0.5em;
        height: 2em;
        animation: moveleg2 1s linear infinite 0.075s;
    }

    @keyframes moveleg {
        0% {
            transform: rotate(-45deg) translateX(-5%);
        }
        50% {
            transform: rotate(45deg) translateX(5%);
        }
        100% {
            transform: rotate(-45deg) translateX(-5%);
        }
    }

    @keyframes moveleg2 {
        0% {
            transform: rotate(45deg);
        }
        50% {
            transform: rotate(-45deg);
        }
        100% {
            transform: rotate(45deg);
        }
    }

    @keyframes movebody {
        0% {
            transform: translateX(0%);
        }
        50% {
            transform: translateX(2%);
        }
        100% {
            transform: translateX(0%);
        }
    }

    .loaderline {
        width: 50em;
        height: 0.5em;
        border-top: 0.5em dashed var(--color2);
        animation: moveline 10s linear infinite;
    }

    @keyframes moveline {
        0% {
            transform: translateX(0%);
            opacity: 0%;
        }
        5% {
            opacity: 100%;
        }
        95% {
            opacity: 100%;
        }
        100% {
            opacity: 0%;
            transform: translateX(-70%);
        }
    }

    .container {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 4rem;
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

    .group-search {
        display: flex;
        line-height: 28px;
        align-items: center;
        position: relative;
        width: 100%; /* Overriding the 190px to fit your 400px layout */
    }

    .input-search {
        width: 100%;
        height: 45px;
        line-height: 28px;
        padding: 0 1rem;
        padding-left: 2.5rem;
        border: 2px solid rgba(198, 225, 237, 0.6);
        border-radius: 0px; /* Removed radius to match brutalist theme */
        outline: none;
        background-color: transparent; /* Made transparent per your preference */
        color: #2b0b0b;
        transition: 0.3s ease;
        font-family: inherit; /* Inherit Jersey 25 */
        font-size: 16px;
        text-transform: uppercase;
    }

    .input-search::placeholder {
        color: rgba(110, 15, 28, 0.5);
        font-family: inherit;
    }

    .input-search:focus,
    .input-search:hover {
        outline: none;
        border-color: #b31b34;
        background-color: rgba(255, 255, 255, 0.1);
        box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.1);
    }

    .icon-search {
        position: absolute;
        left: 1rem;
        fill: rgba(110, 15, 28, 0.5);
        width: 1rem;
        height: 1rem;
        pointer-events: none;
    }

    /* Shared style for the dropdowns below the search bar */
    .filter-element {
        font-family: inherit;
        font-size: 16px;
        background-color: transparent;
        border: 2px solid rgba(198, 225, 237, 0.6);
        padding: 0.5rem;
        text-transform: uppercase;
        width: 100%;
        color: #2b0b0b;
        cursor: pointer;
        transition: 0.3s ease;
    }

    .filter-element:hover,
    .filter-element:focus {
        outline: none;
        border-color: #b31b34;
        box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.1);
    }

    option {
        background-color: #f0f2f5;
        color: #2b0b0b;
        font-family: inherit;
    }
</style>
