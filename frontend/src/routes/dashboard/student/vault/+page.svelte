<script lang="ts">
    import { api } from "$lib/api";
    import type { AcademicResource, ApiResponse } from "$lib/types";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";

    let resources = $state<AcademicResource[]>([]);
    let loading = $state(true);

    // --- Filter State (Matching your Issues page logic) ---
    let searchQuery = $state("");
    let filterType = $state("");

    // Derived filtered list for the 400px view
    let filteredResources = $derived(
        resources.filter((r) => {
            const matchesSearch =
                !searchQuery ||
                r.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
                r.tags.some(t => t.toLowerCase().includes(searchQuery.toLowerCase()));
            const matchesType = !filterType || r.resource_type === filterType;
            return matchesSearch && matchesType;
        })
    );

    onMount(async () => {
        await loadVault();
    });

    async function loadVault() {
        loading = true;
        try {
            // General fetch for resources
            const response = await api.get<ApiResponse<AcademicResource[]>>("/api/courses/all/resources");
            resources = response.data || [];
        } catch (err) {
            console.error("[VAULT] Load failed:", err);
        } finally {
            loading = false;
        }
    }
</script>

<div class="container">
    <div class="w-[400px] flex flex-col items-center gap-4 mb-8 px-2">
        <h1 class="text-12xl font-bold text-[#2b0b0b] tracking-tighter uppercase text-center w-full">
            Vault
        </h1>
        <p class="text-[10px] font-bold text-[#666] uppercase tracking-widest opacity-70">
            Repository of Knowledge
        </p>

        <div class="flex flex-col gap-2 w-full">
            <div class="group-search">
                <svg class="icon-search" aria-hidden="true" viewBox="0 0 24 24">
                    <path d="M21.53 20.47l-3.66-3.66C19.195 15.24 20 13.214 20 11c0-4.97-4.03-9-9-9s-9 4.03-9 9 4.03 9 9 9c2.215 0 4.24-.804 5.808-2.13l3.66 3.66c.147.146.34.22.53.22s.385-.073.53-.22c.295-.293.295-.767.002-1.06zM3.5 11c0-4.135 3.365-7.5 7.5-7.5s7.5 3.365 7.5 7.5-3.365 7.5-7.5 7.5-7.5-3.365-7.5-7.5z"></path>
                </svg>
                <input
                    type="text"
                    bind:value={searchQuery}
                    placeholder="SEARCH ARTIFACTS..."
                    class="input-search"
                />
            </div>

            <select bind:value={filterType} class="filter-element">
                <option value="">ALL RESOURCE TYPES</option>
                <option value="pyq">PREVIOUS YEAR PAPERS</option>
                <option value="notes">STUDY NOTES</option>
                <option value="lecture">LECTURE MATERIALS</option>
                <option value="assignment">ASSIGNMENTS</option>
            </select>
        </div>
    </div>

    {#if loading}
        <div class="loading">DECIPHERING_ARCHIVES...</div>
    {:else}
        <div class="w-[400px] flex flex-col gap-4">
            {#each filteredResources as res}
                <div class="brutalist-card">
                    <div class="flex justify-between items-center mb-2">
                        <span class="type-tag">{res.resource_type}</span>
                        <span class="text-[11px] font-bold text-[#666]">{res.year ?? 'N/A'}</span>
                    </div>
                    <h3 class="text-xl font-black uppercase text-[#2b0b0b] leading-tight mb-2">
                        {res.title}
                    </h3>
                    <div class="flex flex-wrap gap-2 mb-4">
                        {#each res.tags as tag}
                            <span class="text-[10px] font-bold opacity-50 uppercase">#{tag}</span>
                        {/each}
                    </div>
                    <a href={res.file_url} target="_blank" class="retro-btn text-center block w-full">
                        RETRIEVE_FILE
                    </a>
                </div>
            {:else}
                <div class="text-[#6e0f1c] uppercase text-sm mt-8 opacity-50 text-center">
                    No artifacts found
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    /* Reuse the exact styles from your Admin and Grievance pages */
    .container {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 2rem 1rem;
        min-height: 100vh;
    }

    .brutalist-card {
        border: 2px solid rgba(198, 225, 237, 0.6);
        padding: 1.25rem;
        box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.1);
        background: transparent; /* Transparent for notebook lines */
    }

    .type-tag {
        font-size: 10px;
        font-weight: 900;
        text-transform: uppercase;
        padding: 2px 6px;
        border: 1px solid #2b0b0b;
        background: #fff;
    }

    .retro-btn {
        background: #b31b34;
        color: white;
        padding: 0.5rem;
        border: 2px solid #2b0b0b;
        font-weight: bold;
        text-decoration: none;
        box-shadow: 4px 4px 0px #000;
        transition: 0.2s;
    }

    .retro-btn:hover {
        transform: translate(-2px, -2px);
        box-shadow: 6px 6px 0px #000;
    }

    /* Include the group-search, input-search, and filter-element CSS from your Grievance page */
    .group-search { position: relative; width: 100%; display: flex; align-items: center; }
    .input-search {
        width: 100%; height: 45px; padding-left: 2.5rem; border: 2px solid rgba(198, 225, 237, 0.6);
        background: transparent; font-family: inherit; text-transform: uppercase; outline: none;
    }
    .icon-search { position: absolute; left: 1rem; fill: rgba(110, 15, 28, 0.5); width: 1rem; height: 1rem; }
    .filter-element {
        width: 100%; border: 2px solid rgba(198, 225, 237, 0.6); background: transparent; 
        padding: 0.5rem; text-transform: uppercase; font-family: inherit; cursor: pointer;
    }
    .loading { font-family: "Jersey 25", sans-serif; font-size: 1.5rem; color: #2b0b0b; margin-top: 3rem; text-align: center; }
</style>