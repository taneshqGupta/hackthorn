<script lang="ts">
    import { api } from "$lib/api";
    import type { AcademicResource, ApiResponse } from "$lib/types";
    import { onMount } from "svelte";

    // --- State ---
    let resources = $state<AcademicResource[]>([]);
    let enrollments = $state<any[]>([]);
    let selectedCourseId = $state("");

    let loading = $state(true);
    let showUpload = $state(false);
    let uploading = $state(false);

    // --- Upload Form State ---
    let newResource = $state({
        title: "",
        resource_type: "notes",
        tags_input: "",
    });
    let selectedFile = $state<File | null>(null);

    let searchQuery = $state("");
    let filterType = $state("");

    // --- Derived Data ---
    let filteredResources = $derived(
        resources.filter((r) => {
            const matchesSearch =
                !searchQuery ||
                r.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
                r.tags.some((t) =>
                    t.toLowerCase().includes(searchQuery.toLowerCase()),
                );
            const matchesType = !filterType || r.resource_type === filterType;
            return matchesSearch && matchesType;
        }),
    );

    onMount(async () => {
        await loadInitialData();
    });

    async function loadInitialData() {
        loading = true;
        try {
            // 1. Fetch student's courses first to get valid IDs
            const courseRes = await api.get<ApiResponse<any[]>>(
                "/api/courses/my-enrollments",
            );
            enrollments = courseRes.data || [];

            if (enrollments.length > 0) {
                selectedCourseId = enrollments[0].course_id;
                await loadVault();
            }
        } catch (err) {
            console.error("[VAULT] Initialization failed:", err);
        } finally {
            loading = false;
        }
    }

    async function loadVault() {
        if (!selectedCourseId) return;
        loading = true;
        try {
            // 2. Fetch resources for the specific course ID
            const response = await api.get<ApiResponse<AcademicResource[]>>(
                `/api/courses/${selectedCourseId}/resources`,
            );
            resources = response.data || [];
        } catch (err) {
            console.error("[VAULT] Fetch failed:", err);
            resources = [];
        } finally {
            loading = false;
        }
    }

    async function handleUpload() {
        if (!selectedFile || !newResource.title || !selectedCourseId) return;

        // Frontend Guard: Prevent silent backend rejections
        if (selectedFile.size > 10 * 1024 * 1024) {
            alert("FILE_TOO_LARGE: MAX 10MB");
            return;
        }

        uploading = true;
        const formData = new FormData();
        formData.append("file", selectedFile);
        formData.append("title", newResource.title);
        formData.append("resource_type", newResource.resource_type);
        formData.append("tags", newResource.tags_input);

        try {
            // 3. POST to the course-specific endpoint to avoid 404
            await api.post(
                `/api/courses/${selectedCourseId}/resources`,
                formData,
            );
            showUpload = false;
            newResource = { title: "", resource_type: "notes", tags_input: "" };
            selectedFile = null;
            await loadVault();
        } catch (err) {
            console.error("Upload failed:", err);
            alert("UPLOAD_REJECTED: Check file type (PDF/ZIP) or size.");
        } finally {
            uploading = false;
        }
    }
</script>

<div class="vault-container">
    <div class="content-stack">
        <h1
            class="text-7xl font-black text-[#2b0b0b] uppercase text-center w-full tracking-tighter"
        >
            Vault
        </h1>

        <div class="search-block">
            <select
                bind:value={selectedCourseId}
                onchange={loadVault}
                class="filter-element mb-2 bg-[#2b0b0b] text-white"
            >
                <option value="">SELECT_KNOWLEDGE_CABINET...</option>
                {#each enrollments as course}
                    <option value={course.course_id}
                        >{course.course_code}</option
                    >
                {/each}
            </select>

            <div class="group-search">
                <input
                    type="text"
                    bind:value={searchQuery}
                    placeholder="SEARCH_BY_TAGS_OR_TITLES..."
                    class="input-search"
                />
            </div>

            <button
                onclick={() => (showUpload = !showUpload)}
                class="retro-btn contrib mt-2"
            >
                {showUpload ? "CANCEL_UPLOAD" : "CONTRIBUTE_ARTIFACT"}
            </button>
        </div>

        {#if showUpload}
            <div class="brutalist-card upload-form">
                <h3
                    class="font-black uppercase mb-4 border-b-2 border-[#2b0b0b] text-sm"
                >
                    Upload to {enrollments.find(
                        (c) => c.course_id === selectedCourseId,
                    )?.course_code || "Archive"}
                </h3>
                <div class="flex flex-col gap-3">
                    <input
                        type="text"
                        bind:value={newResource.title}
                        placeholder="TITLE (E.G. CS201_MIDSEM)"
                        class="form-input"
                    />
                    <input
                        type="text"
                        bind:value={newResource.tags_input}
                        placeholder="TAGS (COMMA_SEPARATED)"
                        class="form-input"
                    />
                    <input
                        type="file"
                        onchange={(e) =>
                            (selectedFile = e.currentTarget.files?.[0] || null)}
                        class="file-input"
                    />
                    <button
                        onclick={handleUpload}
                        disabled={uploading}
                        class="retro-btn bg-[#b31b34] text-white"
                    >
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
                        </div>
                        <h2
                            class="font-black text-xl uppercase leading-none mb-2 text-[#2b0b0b]"
                        >
                            {res.title}
                        </h2>
                        <div class="flex flex-wrap gap-1 mb-4">
                            {#each res.tags as tag}
                                <span
                                    class="text-[9px] font-black opacity-40 uppercase"
                                    >#{tag}</span
                                >
                            {/each}
                        </div>
                        <a
                            href={res.file_url}
                            target="_blank"
                            class="retro-btn retrieve">RETRIEVE_FILE</a
                        >
                    </div>
                {:else}
                    <div
                        class="text-[#6e0f1c] uppercase text-xs mt-8 opacity-50 text-center"
                    >
                        No artifacts found in this cabinet
                    </div>
                {/each}
            </div>
        {/if}
    </div>
</div>

<style>
    .vault-container {
        display: flex;
        justify-content: center;
        width: 100%;
        padding: 2rem 0;
    }
    .content-stack {
        width: 400px;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .brutalist-card {
        border: 2px solid rgba(198, 225, 237, 0.6);
        padding: 1.25rem;
        box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.1);
        background: transparent;
    }

    .form-input {
        width: 100%;
        padding: 8px;
        background: transparent;
        border: 2px solid #2b0b0b;
        font-family: inherit;
        font-size: 12px;
        text-transform: uppercase;
    }

    .retro-btn {
        width: 100%;
        border: 2px solid #2b0b0b;
        font-weight: 900;
        text-transform: uppercase;
        box-shadow: 4px 4px 0px #000;
        padding: 10px;
        cursor: pointer;
        transition: 0.1s;
        text-decoration: none;
        display: block;
        text-align: center;
    }
    .contrib {
        background: #2b0b0b;
        color: white;
    }
    .retrieve {
        background: #b31b34;
        color: white;
    }

    .type-badge {
        font-size: 9px;
        font-weight: 900;
        border: 1px solid #2b0b0b;
        padding: 1px 4px;
        text-transform: uppercase;
        background: white;
    }

    .input-search {
        width: 100%;
        height: 45px;
        padding-left: 1rem;
        border: 2px solid rgba(198, 225, 237, 0.6);
        background: transparent;
        font-family: inherit;
        text-transform: uppercase;
        outline: none;
    }

    .filter-element {
        width: 100%;
        border: 2px solid rgba(198, 225, 237, 0.6);
        background: transparent;
        padding: 0.5rem;
        text-transform: uppercase;
        font-family: inherit;
        cursor: pointer;
        font-size: 12px;
    }

    .loading {
        font-family: "Jersey 25", sans-serif;
        font-size: 1.5rem;
        text-align: center;
        color: #2b0b0b;
        margin-top: 2rem;
    }
</style>
