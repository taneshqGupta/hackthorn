<script lang="ts">
    import { api } from "$lib/api";
    import type { AcademicResource, ApiResponse } from "$lib/types";
    import { onMount } from "svelte";

    // --- Core Data State ---
    let courses = $state<any[]>([]); // List of all available courses (The Directory)
    let resources = $state<AcademicResource[]>([]); // Resources for selected course
    let selectedCourse = $state<any | null>(null); // Currently open cabinet
    
    // --- UI State ---
    let loadingCourses = $state(true);
    let loadingResources = $state(false);
    let viewMode = $state<"browse" | "upload">("browse"); // Toggle between viewing and contributing
    
    // --- Upload State ---
    let uploadState = $state({
        title: "",
        type: "notes",
        year: new Date().getFullYear().toString(),
        professor: "",
        tags: ""
    });
    let selectedFile = $state<File | null>(null);
    let isUploading = $state(false);

    // --- Search & Filter State ---
    let searchQuery = $state("");
    let typeFilter = $state("all");

    // --- Derived Resources (Client-Side Search) ---
    let visibleResources = $derived(
        resources.filter(r => {
            const matchesSearch = !searchQuery || 
                r.title.toLowerCase().includes(searchQuery.toLowerCase()) || 
                r.tags.some(t => t.toLowerCase().includes(searchQuery.toLowerCase()));
            const matchesType = typeFilter === "all" || r.resource_type === typeFilter;
            return matchesSearch && matchesType;
        })
    );

    onMount(async () => {
        await fetchDirectory();
    });

    // 1. Fetch All Courses (The Directory)
    async function fetchDirectory() {
        try {
            // We fetch ALL courses so you can access/upload for any subject, not just enrolled ones
            const res = await api.get<ApiResponse<any[]>>("/api/courses"); 
            courses = res.data || [];
            if (courses.length > 0) selectCourse(courses[0]);
        } catch (e) {
            console.error("Directory sync failed", e);
        } finally {
            loadingCourses = false;
        }
    }

    // 2. Select a Course & Load Its Resources
    async function selectCourse(course: any) {
        selectedCourse = course;
        loadingResources = true;
        viewMode = "browse"; // Reset to browse view
        try {
            const res = await api.get<ApiResponse<AcademicResource[]>>(`/api/courses/${course.id}/resources`);
            resources = res.data || [];
        } catch (e) {
            console.error("Cabinet locked (Fetch failed)", e);
            resources = [];
        } finally {
            loadingResources = false;
        }
    }

    // 3. Handle The Upload (Strictly Typed)
    async function handleUpload() {
        if (!selectedCourse || !selectedFile || !uploadState.title) return;

        // Validation Guard
        if (selectedFile.size > 10 * 1024 * 1024) {
            alert("ARTIFACT_REJECTED: File size exceeds 10MB limit.");
            return;
        }

        isUploading = true;
        const fd = new FormData();
        fd.append("file", selectedFile);
        
        // Formatting title to be searchable (e.g. "CS201_2024_Midsem_Notes")
        const finalTitle = `${uploadState.title}`; 
        fd.append("title", finalTitle);
        fd.append("resource_type", uploadState.type);
        
        // Combining metadata into tags for the backend
        const metaTags = `${uploadState.year},${uploadState.professor},${uploadState.tags}`;
        fd.append("tags", metaTags);

        try {
            // Hitting the CORRECT route defined in main.rs
            await api.post(`/api/courses/${selectedCourse.id}/resources`, fd);
            
            // Success Reset
            alert("ARTIFACT_ARCHIVED_SUCCESSFULLY");
            viewMode = "browse";
            uploadState = { title: "", type: "notes", year: new Date().getFullYear().toString(), professor: "", tags: "" };
            selectedFile = null;
            await selectCourse(selectedCourse); // Refresh list
        } catch (e) {
            console.error(e);
            alert("ARCHIVAL_FAILED: Server rejected the artifact data.");
        } finally {
            isUploading = false;
        }
    }
</script>

<div class="citadel-container">
    <aside class="directory-pane">
        <div class="pane-header">
            <h2>COURSE_INDEX</h2>
            <div class="directory-stats">{courses.length} CABINETS FOUND</div>
        </div>
        
        <div class="course-list">
            {#if loadingCourses}
                <div class="loading-text">SCANNING_INDEX...</div>
            {:else}
                {#each courses as course}
                    <button 
                        class="course-btn {selectedCourse?.id === course.id ? 'active' : ''}" 
                        onclick={() => selectCourse(course)}
                    >
                        <span class="code">{course.code}</span>
                        <span class="name">{course.name}</span>
                    </button>
                {/each}
            {/if}
        </div>
    </aside>

    <main class="vault-pane">
        {#if selectedCourse}
            <div class="vault-header">
                <div class="header-info">
                    <h1 class="cabinet-title">{selectedCourse.code}: {selectedCourse.name}</h1>
                    <div class="cabinet-meta">
                        ID: {selectedCourse.id.split('-')[0]}... • {selectedCourse.credits || '3'} CREDITS
                    </div>
                </div>
                
                <div class="header-actions">
                    <button 
                        class="tab-btn {viewMode === 'browse' ? 'active' : ''}" 
                        onclick={() => viewMode = 'browse'}
                    >
                        BROWSE_ARTIFACTS
                    </button>
                    <button 
                        class="tab-btn {viewMode === 'upload' ? 'active' : ''}" 
                        onclick={() => viewMode = 'upload'}
                    >
                        CONTRIBUTE_DATA
                    </button>
                </div>
            </div>

            {#if viewMode === 'browse'}
                <div class="toolbar">
                    <div class="search-box">
                        <span class="icon">🔍</span>
                        <input type="text" bind:value={searchQuery} placeholder="SEARCH PYQS, NOTES, TAGS..." />
                    </div>
                    <select bind:value={typeFilter} class="filter-select">
                        <option value="all">ALL_TYPES</option>
                        <option value="pyq">PREVIOUS_YEAR_QP</option>
                        <option value="notes">STUDY_NOTES</option>
                        <option value="assignment">ASSIGNMENTS</option>
                    </select>
                </div>

                <div class="artifacts-grid">
                    {#if loadingResources}
                        <div class="loading-text">DECRYPTING_CABINET...</div>
                    {:else if visibleResources.length === 0}
                        <div class="empty-state">
                            <h3>CABINET_EMPTY</h3>
                            <p>No artifacts found. Be the first to contribute.</p>
                        </div>
                    {:else}
                        {#each visibleResources as res}
                            <div class="artifact-card">
                                <div class="card-top">
                                    <span class="res-type {res.resource_type}">{res.resource_type}</span>
                                    <span class="res-date">{new Date(res.created_at).toLocaleDateString()}</span>
                                </div>
                                <h4>{res.title}</h4>
                                <div class="tags-row">
                                    {#each res.tags as tag}
                                        <span class="tag">#{tag.trim()}</span>
                                    {/each}
                                </div>
                                <a href={res.file_url} target="_blank" class="download-btn">RETRIEVE_FILE</a>
                            </div>
                        {/each}
                    {/if}
                </div>
            
            {:else if viewMode === 'upload'}
                <div class="contribution-form">
                    <div class="form-header">
                        <h3>ARCHIVE_NEW_ARTIFACT</h3>
                        <p>Ensure files are legible and correctly tagged for future generations.</p>
                    </div>

                    <div class="form-grid">
                        <div class="field">
                            <label>ARTIFACT_TITLE</label>
                            <input type="text" bind:value={uploadState.title} placeholder="e.g. Endsem_Solution_2025" />
                        </div>
                        
                        <div class="field-row">
                            <div class="field">
                                <label>TYPE</label>
                                <select bind:value={uploadState.type}>
                                    <option value="notes">NOTES</option>
                                    <option value="pyq">PYQ (Question Paper)</option>
                                    <option value="assignment">ASSIGNMENT</option>
                                    <option value="book">REFERENCE_BOOK</option>
                                </select>
                            </div>
                            <div class="field">
                                <label>YEAR</label>
                                <input type="number" bind:value={uploadState.year} />
                            </div>
                        </div>

                        <div class="field">
                            <label>PROFESSOR (Optional)</label>
                            <input type="text" bind:value={uploadState.professor} placeholder="Who taught this?" />
                        </div>

                        <div class="field">
                            <label>EXTRA_TAGS (Comma Separated)</label>
                            <input type="text" bind:value={uploadState.tags} placeholder="midsem, difficult, graph-theory" />
                        </div>

                        <div class="field upload-zone">
                            <label>FILE_PAYLOAD (PDF/IMG/ZIP)</label>
                            <input 
                                type="file" 
                                accept=".pdf,.png,.jpg,.jpeg,.zip,.docx" 
                                onchange={e => selectedFile = e.currentTarget.files[0]} 
                            />
                        </div>
                    </div>

                    <div class="form-actions">
                        <button class="cancel-btn" onclick={() => viewMode = 'browse'}>CANCEL</button>
                        <button class="submit-btn" onclick={handleUpload} disabled={isUploading}>
                            {isUploading ? "TRANSMITTING..." : "INITIATE_UPLOAD"}
                        </button>
                    </div>
                </div>
            {/if}

        {:else}
            <div class="no-selection">
                <h1>THE_VAULT</h1>
                <p>SELECT_A_COURSE_CABINET_FROM_THE_DIRECTORY_TO_BEGIN.</p>
            </div>
        {/if}
    </main>
</div>

<style>
    /* --- LAYOUT --- */
    .citadel-container {
        display: grid;
        grid-template-columns: 280px 1fr;
        height: calc(100vh - 80px); /* Fill screen minus header */
        max-width: 1400px;
        margin: 0 auto;
        gap: 20px;
        padding: 20px;
        font-family: 'Inter', sans-serif; /* Or your preferred font */
    }

    /* --- LEFT PANE: DIRECTORY --- */
    .directory-pane {
        border: 4px solid #2b0b0b;
        background: rgba(255, 255, 255, 0.05);
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .pane-header {
        background: #2b0b0b;
        color: white;
        padding: 15px;
        text-align: center;
    }
    .pane-header h2 { margin: 0; font-weight: 900; font-size: 1.2rem; letter-spacing: 1px; }
    .directory-stats { font-size: 0.7rem; opacity: 0.8; margin-top: 5px; font-family: monospace; }

    .course-list {
        overflow-y: auto;
        flex: 1;
        padding: 10px;
    }

    .course-btn {
        display: block;
        width: 100%;
        text-align: left;
        background: transparent;
        border: none;
        border-bottom: 2px solid rgba(43, 11, 11, 0.1);
        padding: 12px 10px;
        cursor: pointer;
        transition: 0.1s;
    }
    .course-btn:hover { background: rgba(43, 11, 11, 0.05); padding-left: 15px; }
    .course-btn.active { background: #2b0b0b; color: white; }
    
    .course-btn .code { display: block; font-weight: 900; font-size: 0.9rem; }
    .course-btn .name { display: block; font-size: 0.8rem; opacity: 0.8; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }

    /* --- RIGHT PANE: VAULT --- */
    .vault-pane {
        border: 4px solid #2b0b0b;
        background: rgba(255, 255, 255, 0.2);
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .vault-header {
        border-bottom: 4px solid #2b0b0b;
        padding: 20px;
        background: #fff;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .cabinet-title { margin: 0; font-weight: 950; font-size: 1.8rem; text-transform: uppercase; color: #2b0b0b; }
    .cabinet-meta { font-size: 0.8rem; font-family: monospace; color: #666; margin-top: 5px; }

    .header-actions { display: flex; gap: 10px; }
    .tab-btn {
        background: transparent; border: 2px solid #2b0b0b;
        padding: 8px 16px; font-weight: 800; cursor: pointer;
        text-transform: uppercase; font-size: 0.8rem;
    }
    .tab-btn.active { background: #2b0b0b; color: white; box-shadow: 3px 3px 0px rgba(0,0,0,0.2); }

    /* --- BROWSE VIEW --- */
    .toolbar {
        padding: 15px;
        border-bottom: 2px solid #2b0b0b;
        display: flex;
        gap: 10px;
        background: rgba(43, 11, 11, 0.02);
    }
    .search-box {
        flex: 1;
        display: flex;
        align-items: center;
        border: 2px solid #2b0b0b;
        background: white;
        padding: 0 10px;
    }
    .search-box input {
        border: none; outline: none; width: 100%; padding: 10px;
        font-weight: 600; text-transform: uppercase;
    }
    .filter-select {
        border: 2px solid #2b0b0b;
        padding: 0 15px;
        font-weight: 700;
        text-transform: uppercase;
        background: white;
    }

    .artifacts-grid {
        padding: 20px;
        overflow-y: auto;
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 20px;
    }

    /* --- ARTIFACT CARD --- */
    .artifact-card {
        border: 3px solid #2b0b0b;
        background: white;
        padding: 15px;
        box-shadow: 6px 6px 0px rgba(0,0,0,1);
        transition: 0.1s;
    }
    .artifact-card:hover { transform: translate(-2px, -2px); box-shadow: 8px 8px 0px #b31b34; }

    .card-top { display: flex; justify-content: space-between; margin-bottom: 10px; }
    .res-type { font-size: 0.7rem; font-weight: 900; text-transform: uppercase; padding: 2px 6px; border: 1px solid black; }
    .res-type.pyq { background: #fef08a; }
    .res-type.notes { background: #bfdbfe; }
    .res-date { font-size: 0.7rem; font-family: monospace; }

    .artifact-card h4 { margin: 0 0 10px 0; font-weight: 800; font-size: 1.1rem; line-height: 1.2; }
    
    .tags-row { display: flex; flex-wrap: wrap; gap: 5px; margin-bottom: 15px; }
    .tag { font-size: 0.7rem; background: #eee; padding: 2px 6px; border-radius: 4px; font-weight: 600; }

    .download-btn {
        display: block; width: 100%; text-align: center;
        background: #2b0b0b; color: white; padding: 8px;
        text-decoration: none; font-weight: 700; font-size: 0.8rem;
    }
    .download-btn:hover { background: #b31b34; }

    /* --- UPLOAD FORM VIEW --- */
    .contribution-form {
        max-width: 600px;
        margin: 40px auto;
        border: 4px solid #2b0b0b;
        background: white;
        padding: 30px;
        box-shadow: 10px 10px 0px #2b0b0b;
    }
    .form-header { text-align: center; margin-bottom: 20px; border-bottom: 2px dashed #ccc; padding-bottom: 15px; }
    .form-header h3 { margin: 0; font-size: 1.5rem; font-weight: 900; }

    .form-grid { display: flex; flex-direction: column; gap: 15px; }
    .field label { display: block; font-size: 0.8rem; font-weight: 800; margin-bottom: 5px; }
    .field input, .field select {
        width: 100%; padding: 10px; border: 2px solid #2b0b0b;
        font-family: inherit; box-sizing: border-box;
    }
    .field-row { display: grid; grid-template-columns: 1fr 1fr; gap: 15px; }
    
    .upload-zone input { border: 2px dashed #2b0b0b; padding: 20px; background: #f9f9f9; cursor: pointer; }

    .form-actions { display: grid; grid-template-columns: 1fr 2fr; gap: 15px; margin-top: 20px; }
    .cancel-btn { background: transparent; border: 2px solid #2b0b0b; font-weight: 800; cursor: pointer; }
    .submit-btn { background: #b31b34; color: white; border: 2px solid #2b0b0b; font-weight: 800; box-shadow: 4px 4px 0px #000; cursor: pointer; }

    /* --- MOBILE RESPONSIVENESS --- */
    @media (max-width: 900px) {
        .citadel-container { grid-template-columns: 1fr; }
        .directory-pane { height: 200px; } /* Directory becomes a top strip on mobile */
    }
</style>