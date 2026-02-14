<script lang="ts">
    import { api } from "$lib/api";
    import type { AcademicResource, ApiResponse } from "$lib/types";
    import { onMount } from "svelte";

    let resources = $state<AcademicResource[]>([]);
    let loading = $state(true);
    let showUpload = $state(false);

    // --- Upload Form State (Grounded in current backend) ---
    let newResource = $state({
        title: "",
        resource_type: "notes",
        tags_input: "" 
    });
    let selectedFile = $state<File | null>(null);
    let uploading = $state(false);

    let searchQuery = $state("");
    let filterType = $state("");

    let filteredResources = $derived(
        resources.filter((r) => {
            const matchesSearch = !searchQuery || 
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
            const response = await api.get<ApiResponse<AcademicResource[]>>("/api/courses/all/resources");
            resources = response.data || [];
        } catch (err) {
            console.error("[VAULT] Sync failed:", err);
        } finally {
            loading = false;
        }
    }

    async function handleUpload() {
        if (!selectedFile || !newResource.title) return;
        
        uploading = true;
        const formData = new FormData();
        formData.append("file", selectedFile);
        formData.append("title", newResource.title);
        formData.append("resource_type", newResource.resource_type);
        formData.append("tags", newResource.tags_input);

        try {
            await api.post("/api/resources/upload", formData);
            showUpload = false;
            // Reset form
            newResource = { title: "", resource_type: "notes", tags_input: "" };
            selectedFile = null;
            await loadVault();
        } catch (err) {
            console.error("Upload failed:", err);
            alert("UPLOAD_FAILED: CHECK_FILE_SIZE_OR_TYPES");
        } finally {
            uploading = false;
        }
    }
</script>

<div class="vault-container">
    <div class="content-stack">
        <h1 class="text-7xl font-black text-[#2b0b0b] uppercase text-center w-full tracking-tighter">Vault</h1>
        
        <div class="search-block">
            <div class="group-search">
                <svg class="icon-search" aria-hidden="true" viewBox="0 0 24 24">
                    <path d="M21.53 20.47l-3.66-3.66C19.195 15.24 20 13.214 20 11c0-4.97-4.03-9-9-9s-9 4.03-9 9 4.03 9 9 9c2.215 0 4.24-.804 5.808-2.13l3.66 3.66c.147.146.34.22.53.22s.385-.073.53-.22c.295-.293.295-.767.002-1.06zM3.5 11c0-4.135 3.365-7.5 7.5-7.5s7.5 3.365 7.5 7.5-3.365 7.5-7.5 7.5-7.5-3.365-7.5-7.5z"></path>
                </svg>
                <input type="text" bind:value={searchQuery} placeholder="SEARCH_BY_TAGS_OR_TITLES..." class="input-search" />
            </div>
            
            <select bind:value={filterType} class="filter-element mt-2">
                <option value="">ALL_RESOURCE_TYPES</option>
                <option value="pyq">PREVIOUS_YEAR_PAPERS</option>
                <option value="notes">STUDY_NOTES</option>
                <option value="lecture">LECTURE_MATERIALS</option>
                <option value="assignment">ASSIGNMENTS</option>
            </select>
            
            <button onclick={() => showUpload = !showUpload} class="retro-btn contrib mt-2">
                {showUpload ? "CANCEL_UPLOAD" : "CONTRIBUTE_ARTIFACT"}
            </button>
        </div>

        {#if showUpload}
            <div class="brutalist-card upload-form">
                <h3 class="font-black uppercase mb-4 border-b-2 border-[#2b0b0b] text-sm">Upload Artifact</h3>
                <div class="flex flex-col gap-3">
                    <input type="text" bind:value={newResource.title} placeholder="TITLE (E.G. CS201_MIDSEM)" class="form-input" />
                    <input type="text" bind:value={newResource.tags_input} placeholder="TAGS (COMMA_SEPARATED)" class="form-input" />
                    <select bind:value={newResource.resource_type} class="filter-element">
                        <option value="pyq">PYQ</option>
                        <option value="notes">NOTES</option>
                        <option value="lecture">LECTURE</option>
                    </select>
                    <input type="file" onchange={(e) => selectedFile = e.currentTarget.files?.[0] || null} class="file-input" />
                    <button onclick={handleUpload} disabled={uploading} class="retro-btn bg-[#b31b34] text-white">
                        {uploading ? "ARCHIVING..." : "SUBMIT_TO_VAULT"}
                    </button>
                </div>
            </div>
        {/if}

        {#if loading}
            <div class="loading">DECIPHERING_ARCHIVES...</div>
        {:else}
            <div class="flex flex-col gap-4">
                {#each filteredResources as res}
                    <div class="brutalist-card">
                        <div class="flex justify-between items-center mb-2">
                            <span class="type-badge">{res.resource_type}</span>
                            <span class="text-[10px] font-black opacity-50">{res.year ?? ''}</span>
                        </div>
                        <h2 class="font-black text-xl uppercase leading-none mb-2 text-[#2b0b0b]">{res.title}</h2>
                        <div class="flex flex-wrap gap-1 mb-4">
                            {#each res.tags as tag}
                                <span class="text-[9px] font-black opacity-40 uppercase">#{tag}</span>
                            {/each}
                        </div>
                        <a href={res.file_url} target="_blank" class="retro-btn retrieve">RETRIEVE_FILE</a>
                    </div>
                {:else}
                    <div class="text-[#6e0f1c] uppercase text-xs mt-8 opacity-50 text-center">
                        No artifacts found in archives
                    </div>
                {/each}
            </div>
        {/if}
    </div>
</div>

<style>
    .vault-container { display: flex; justify-content: center; width: 100%; padding: 2rem 0; }
    .content-stack { width: 400px; display: flex; flex-direction: column; gap: 1.5rem; }
    
    .brutalist-card { 
        border: 2px solid rgba(198, 225, 237, 0.6); 
        padding: 1.25rem; 
        box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.1);
        background: transparent;
    }

    .form-input {
        width: 100%; padding: 8px; background: transparent; 
        border: 2px solid #2b0b0b; font-family: inherit; font-size: 12px;
        text-transform: uppercase;
    }

    .file-input { font-size: 10px; font-weight: 900; text-transform: uppercase; }

    .retro-btn {
        width: 100%; border: 2px solid #2b0b0b; font-weight: 900;
        text-transform: uppercase; box-shadow: 4px 4px 0px #000;
        padding: 10px; cursor: pointer; transition: 0.1s;
        text-decoration: none; display: block; text-align: center;
    }
    .contrib { background: #2b0b0b; color: white; }
    .retrieve { background: #b31b34; color: white; }
    
    .retro-btn:hover { transform: translate(-1px, -1px); box-shadow: 5px 5px 0px #000; }

    .type-badge { font-size: 9px; font-weight: 900; border: 1px solid #2b0b0b; padding: 1px 4px; text-transform: uppercase; background: white; }
    
    .input-search {
        width: 100%; height: 45px; padding-left: 2.5rem; border: 2px solid rgba(198, 225, 237, 0.6);
        background: transparent; font-family: inherit; text-transform: uppercase; outline: none;
    }
    .group-search { position: relative; width: 100%; display: flex; align-items: center; }
    .icon-search { position: absolute; left: 1rem; fill: rgba(110, 15, 28, 0.5); width: 1rem; height: 1rem; }
    
    .filter-element {
        width: 100%; border: 2px solid rgba(198, 225, 237, 0.6); background: transparent; 
        padding: 0.5rem; text-transform: uppercase; font-family: inherit; cursor: pointer; font-size: 12px;
    }

    .loading { font-family: "Jersey 25", sans-serif; font-size: 1.5rem; text-align: center; color: #2b0b0b; margin-top: 2rem; }
</style>