<script lang="ts">
    interface Props {
        id: string;
        username: string;
        title?: string;
        content: string;
        images?: string[];
        upvotes?: number;
        commentsCount?: number;
        date: string;
        status?: string;
        category?: string;
        isLiked?: boolean;
        onclick?: () => void;
        onupvote?: (newStatus: boolean) => void;
    }

    let {
        id,
        username,
        title,
        content,
        images = [],
        upvotes = 0,
        commentsCount = 0,
        date,
        status,
        category,
        isLiked = false,
        onclick,
        onupvote
    }: Props = $props();

    // 1. Initialize State ONCE
    // svelte-ignore state_referenced_locally
        let localUpvotes = $state(upvotes);
    // svelte-ignore state_referenced_locally
        let hasLiked = $state(isLiked);

    // 2. ONLY update from props if the Post ID changes (e.g. scrolling a feed)
    // We intentionally ignore updates to 'upvotes' for the SAME id to prevent the double-counting bug.
    $effect(() => {
        // This line simply "reads" id to track it
        id; 
        // We use untracked to prevent resetting when only upvotes/isLiked change partially
        resetState();
    });

    function resetState() {
        localUpvotes = upvotes;
        hasLiked = isLiked;
    }

    function handleUpvote(e: MouseEvent | KeyboardEvent) {
        e.stopPropagation?.();
        
        // Simple Toggle Logic
        if (hasLiked) {
            localUpvotes -= 1;
            hasLiked = false;
        } else {
            localUpvotes += 1;
            hasLiked = true;
        }

        console.log(`[PostCard] Toggled like for ${id}. New state: ${hasLiked}`);
        if (onupvote) onupvote(hasLiked);
    }

    function formatDate(dateString: string): string {
        const postDate = new Date(dateString);
        const now = new Date();
        const diffMs = now.getTime() - postDate.getTime();
        const diffMins = Math.floor(diffMs / 60000);
        const diffHours = Math.floor(diffMs / 3600000);
        const diffDays = Math.floor(diffMs / 86400000);

        if (diffMins < 60) return `${diffMins}m ago`;
        if (diffHours < 24) return `${diffHours}h ago`;
        if (diffDays < 7) return `${diffDays}d ago`;
        return postDate.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
    }

    const displayImages = $derived(images.slice(0, 5));
</script>

<div class="card" onclick={() => onclick?.()} role="button" tabindex="0" onkeydown={(e) => (e as KeyboardEvent).key === 'Enter' && onclick?.()}>
    <div class="username">{username}</div>

    <div class="body">
        {#if title}
            <div class="title">{title}</div>
        {/if}
        <div class="text">{content}</div>

        {#if displayImages.length > 0}
            <div class="images" class:single={displayImages.length === 1} class:grid={displayImages.length > 1}>
                {#each displayImages as image}
                    <img src={image} alt="Post attachment" />
                {/each}
            </div>
        {/if}
    </div>

    <div class="footer">
        <div 
            class="footer-item upvote-btn" 
            class:liked={hasLiked}
            onclick={(e) => { (e as MouseEvent).stopPropagation(); handleUpvote(e as MouseEvent); }} 
            role="button" 
            tabindex="0" 
            onkeydown={(e) => { (e as KeyboardEvent).stopPropagation(); if ((e as KeyboardEvent).key === 'Enter') handleUpvote(e as KeyboardEvent); }}
        >
            <svg xmlns="http://www.w3.org/2000/svg" fill={hasLiked ? "currentColor" : "none"} viewBox="0 0 24 24" stroke-width="1.5">
                <path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z" />
            </svg>
            <span>{localUpvotes}</span>
        </div>

        <div class="footer-item">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5">
                <path stroke-linecap="round" stroke-linejoin="round" d="M8.625 12a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H8.25m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12.375M21 12c0 4.556-4.03 8.25-9 8.25a9.764 9.764 0 0 1-2.555-.337A5.972 5.972 0 0 1 5.41 20.97a5.969 5.969 0 0 1-.474-.065 4.48 4.48 0 0 0 .978-2.025c.09-.457-.133-.901-.467-1.226C3.93 16.178 3 14.189 3 12c0-4.556 4.03-8.25 9-8.25s9 3.694 9 8.25Z" />
            </svg>
            <span>{commentsCount}</span>
        </div>

        <div class="footer-item">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5">
                <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 0 1 2.25-2.25h13.5A2.25 2.25 0 0 1 21 7.5v11.25m-18 0A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75m-18 0v-7.5A2.25 2.25 0 0 1 5.25 9h13.5A2.25 2.25 0 0 1 21 11.25v7.5" />
            </svg>
            <span>{formatDate(date)}</span>
        </div>

        {#if status}
            <div class="footer-item">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
                </svg>
                <span>{status}</span>
            </div>
        {/if}

        {#if category}
            <div class="footer-item">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9.568 3H5.25A2.25 2.25 0 0 0 3 5.25v4.318c0 .597.237 1.17.659 1.591l9.581 9.581c.699.699 1.78.872 2.607.33a18.095 18.095 0 0 0 5.223-5.223c.542-.827.369-1.908-.33-2.607L11.16 3.66A2.25 2.25 0 0 0 9.568 3Z" />
                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 6h.008v.008H6V6Z" />
                </svg>
                <span>{category}</span>
            </div>
        {/if}
    </div>
</div>

<style>
    /* ... Keep all your previous styles ... */
    
    .card {
        position: relative;
        background-color: transparent;
        padding: 1em;
        z-index: 5;
        box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.5);
        border-radius: 0px;
        max-width: 300px;
        transition: 200ms ease-in-out;
        border: 2px solid rgba(198, 225, 237, 0.6);
        cursor: pointer;
        text-align: left;
        width: 100%;
        font-family: inherit;
    }

    .card:hover {
        transform: translate(-2px, -2px);
        box-shadow: 6px 6px 0px rgba(0, 0, 0, 0.7);
        border-color: rgba(198, 225, 237, 0.9);
    }

    .username {
        color: #6e0f1c;
        font-size: 0.85em;
        font-weight: 600;
        margin-bottom: 0.5em;
        font-family: inherit;
    }

    .body {
        display: flex;
        flex-direction: column;
    }

    .title {
        color: #2b0b0b;
        font-size: 1.1em;
        font-weight: 600;
        margin-bottom: 0.5em;
        font-family: inherit;
    }

    .body .text {
        margin: 0 10px 0 0;
        white-space: pre-line;
        color: #3b0b12;
        font-weight: 400;
        line-height: 1.5;
        margin-bottom: 4px;
        display: -webkit-box;
        -webkit-line-clamp: 3;
        line-clamp: 3;
        -webkit-box-orient: vertical;
        overflow: hidden;
        font-family: inherit;
    }

    .images {
        margin-top: 10px;
        border-radius: 0px;
        overflow: hidden;
        border: 2px solid rgba(198, 225, 237, 0.6);
    }

    .images.single {
        display: block;
    }

    .images.single img {
        width: 100%;
        height: auto;
        max-height: 200px;
        object-fit: cover;
    }

    .images.grid {
        display: grid;
        gap: 4px;
    }

    .images.grid:has(img:nth-child(2)) {
        grid-template-columns: 1fr 1fr;
    }

    .images.grid:has(img:nth-child(3):last-child) {
        grid-template-columns: 1fr 1fr 1fr;
    }

    .images.grid:has(img:nth-child(4)) {
        grid-template-columns: 1fr 1fr;
    }

    .images.grid:has(img:nth-child(5)) {
        grid-template-columns: 1fr 1fr 1fr;
    }

    .images img {
        width: 100%;
        height: 100px;
        object-fit: cover;
    }

    .footer {
        position: relative;
        width: 100%;
        color: #5a0f1a;
        font-size: 12px;
        display: flex;
        align-items: center;
        flex-wrap: wrap;
        gap: 0.5rem;
        border: none;
        margin-top: 10px;
    }

    .footer-item {
        display: flex;
        align-items: center;
        cursor: pointer;
        transition: color 0.2s;
    }

    .footer-item:hover {
        color: #b31b34;
    }

    .upvote-btn.liked {
        color: #e11d48;
    }

    .upvote-btn.liked svg {
        fill: currentColor;
    }

    .upvote-btn:hover {
        transform: scale(1.1);
    }

    .footer-item svg {
        margin-right: 4px;
        height: 16px;
        width: 16px;
        stroke: currentColor;
    }

    .footer-item span {
        line-height: 1;
    }
</style>