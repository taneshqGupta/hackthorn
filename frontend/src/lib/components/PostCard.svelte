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
		onclick?: () => void;
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
		onclick
	}: Props = $props();

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

<button class="card" onclick={onclick}>
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
		<div class="footer-item">
			<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5">
				<path stroke-linecap="round" stroke-linejoin="round" d="M6.633 10.25c.806 0 1.533-.446 2.031-1.08a9.041 9.041 0 0 1 2.861-2.4c.723-.384 1.35-.956 1.653-1.715a4.498 4.498 0 0 0 .322-1.672V2.75a.75.75 0 0 1 .75-.75 2.25 2.25 0 0 1 2.25 2.25c0 1.152-.26 2.243-.723 3.218-.266.558.107 1.282.725 1.282m0 0h3.126c1.026 0 1.945.694 2.054 1.715.045.422.068.85.068 1.285a11.95 11.95 0 0 1-2.649 7.521c-.388.482-.987.729-1.605.729H13.48c-.483 0-.964-.078-1.423-.23l-3.114-1.04a4.501 4.501 0 0 0-1.423-.23H5.904m10.598-9.75H14.25M5.904 18.5c.083.205.173.405.27.602.197.4-.078.898-.523.898h-.908c-.889 0-1.713-.518-1.972-1.368a12 12 0 0 1-.521-3.507c0-1.553.295-3.036.831-4.398C3.387 9.953 4.167 9.5 5 9.5h1.053c.472 0 .745.556.5.96a8.958 8.958 0 0 0-1.302 4.665c0 1.194.232 2.333.654 3.375Z" />
			</svg>
			<span>{upvotes}</span>
		</div>

		<div class="footer-item">
			<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5">
				<path stroke-linecap="round" stroke-linejoin="round" d="M8.625 12a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H8.25m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0h-.375M21 12c0 4.556-4.03 8.25-9 8.25a9.764 9.764 0 0 1-2.555-.337A5.972 5.972 0 0 1 5.41 20.97a5.969 5.969 0 0 1-.474-.065 4.48 4.48 0 0 0 .978-2.025c.09-.457-.133-.901-.467-1.226C3.93 16.178 3 14.189 3 12c0-4.556 4.03-8.25 9-8.25s9 3.694 9 8.25Z" />
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
</button>

<style>
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
		color: #c6e1ed;
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
		color: #e0e3f5;
		font-size: 1.1em;
		font-weight: 600;
		margin-bottom: 0.5em;
		font-family: inherit;
	}

	.body .text {
		margin: 0 10px 0 0;
		white-space: pre-line;
		color: #c0c3d7;
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
		color: #9fa4aa;
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
		color: #c6e1ed;
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
