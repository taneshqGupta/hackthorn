<script lang="ts">
    // ... Props logic stays exactly the same ...
</script>

<div class="card-wrapper">
    <div class="card" onclick={() => onclick?.()} role="button" tabindex="0" onkeydown={(e) => (e as KeyboardEvent).key === 'Enter' && onclick?.()}>
        <div class="username">{username}</div>

        <div class="body">
            {#if title} <div class="title">{title}</div> {/if}
            <div class="text">{content}</div>

            {#if displayImages.length > 0}
                <div class="images-container">
                    <div class="images" class:single={displayImages.length === 1} class:grid={displayImages.length > 1}>
                        {#each displayImages as image}
                            <img src={image} alt="Evidence" />
                        {/each}
                    </div>
                </div>
            {/if}
        </div>

        <div class="footer">
            <div 
                class="footer-item upvote-btn" 
                class:liked={currentLiked}
                onclick={handleUpvote} 
                role="button" 
                tabindex="0"
                onkeydown={(e) => e.key === 'Enter' && handleUpvote(e)}
            >
                <svg xmlns="http://www.w3.org/2000/svg" fill={currentLiked ? "currentColor" : "none"} viewBox="0 0 24 24" stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z" />
                </svg>
                <span>{currentUpvotes}</span>
            </div>

            <div class="footer-item">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M8.625 12a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H8.25m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12.375M21 12c0 4.556-4.03 8.25-9 8.25a9.764 9.764 0 0 1-2.555-.337A5.972 5.972 0 0 1 5.41 20.97a5.969 5.969 0 0 1-.474-.065 4.48 4.48 0 0 0 .978-2.025c.09-.457-.133-.901-.467-1.226C3.93 16.178 3 14.189 3 12c0-4.556 4.03-8.25 9-8.25s9 3.694 9 8.25Z" />
                </svg>
                <span>{commentsCount}</span>
            </div>

            <div class="footer-item">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
                </svg>
                <span>{formatDate(date)}</span>
            </div>
        </div>

        {#if status || category}
            <div class="badge-row">
                {#if status}<div class="footer-item status-badge"><span>{status}</span></div>{/if}
                {#if category}<div class="footer-item category-tag"><span>#{category}</span></div>{/if}
            </div>
        {/if}
    </div>
</div>

<style>
    .card-wrapper {
        width: 100%;
        display: flex;
        justify-content: center;
        padding: 0.5rem 0;
    }

    .card {
        position: relative;
        background-color: transparent;
        padding: 1.5em;
        z-index: 5;
        box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.5);
        border: 2px solid rgba(198, 225, 237, 0.6);
        cursor: pointer;
        text-align: center; /* Center text globally inside card */
        width: 100%;
        max-width: 400px;
        transition: 200ms ease-in-out;
        display: flex;
        flex-direction: column;
        align-items: center; /* Center children horizontally */
    }

    .username {
        font-weight: 900;
        text-transform: uppercase;
        font-size: 0.75rem;
        margin-bottom: 1rem;
        opacity: 0.7;
    }

    .title {
        font-weight: 950;
        font-size: 1.25rem;
        line-height: 1.1;
        margin-bottom: 0.75rem;
        text-transform: uppercase;
    }

    .text {
        font-size: 1rem;
        line-height: 1.4;
        margin-bottom: 1.5rem;
    }

    .images-container {
        width: 100%;
        display: flex;
        justify-content: center;
        margin-bottom: 1.5rem;
    }

    .images.grid {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 4px;
        width: 100%;
    }

    .images img {
        width: 100%;
        height: auto;
        border: 1px solid rgba(198, 225, 237, 0.6);
    }

    .footer {
        display: flex;
        justify-content: center; /* Center the icons */
        gap: 1.5rem;
        font-size: 0.8rem;
        color: #555;
        width: 100%;
        padding-top: 1rem;
        border-top: 1px dashed rgba(198, 225, 237, 0.3);
    }

    .badge-row {
        display: flex;
        justify-content: center;
        gap: 0.5rem;
        margin-top: 1rem;
    }

    .footer-item {
        display: flex;
        align-items: center;
        gap: 4px;
        font-weight: 700;
    }

    .footer-item svg {
        width: 16px;
        height: 16px;
    }

    .status-badge {
        font-size: 10px;
        text-transform: uppercase;
        background: #000;
        color: #fff;
        padding: 2px 6px;
    }

    .category-tag {
        font-size: 10px;
        text-transform: uppercase;
        opacity: 0.5;
    }

    .card:hover {
        transform: translate(-2px, -2px);
        box-shadow: 6px 6px 0px rgba(0, 0, 0, 0.7);
        border-color: rgba(198, 225, 237, 0.9);
    }

    .upvote-btn.liked {
        color: #e11d48;
    }
    .upvote-btn.liked svg {
        fill: currentColor;
    }
</style>