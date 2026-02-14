<script lang="ts">
	interface Props {
		title?: string;
		children?: any;
	}

	let { title = 'Window', children }: Props = $props();
</script>

<div class="card">
	<div class="head">{title}</div>
	<div class="content">
		{@render children()}
	</div>
</div>

<style>
	.card {
		width: 300px;
		min-height: 250px;
		translate: -6px -6px;
		background: #ff66a3;
		border: 3px solid #000000;
		box-shadow: 12px 12px 0 #000000;
		overflow: hidden;
		transition: all 0.3s ease;
	}

	.head {
		font-size: 14px;
		font-weight: 900;
		width: 100%;
		height: 24px;
		background: #ffffff;
		padding: 2px 12px;
		color: #000000;
		border-bottom: 3px solid #000000;
	}

	.content {
		padding: 8px 12px;
		font-size: 14px;
		font-weight: 600;
		color: #000000;
	}

	.card:hover {
		translate: -6px;
	}
</style>
