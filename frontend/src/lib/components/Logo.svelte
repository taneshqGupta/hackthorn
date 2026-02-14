<script lang="ts">
	import { goto } from '$app/navigation';
</script>

<button on:click={() => goto('/')} class="cursor-pointer bg-transparent border-none p-4">
	<h1 class="logo">AEGIS</h1>
</button>

<style>
	@import url('https://fonts.googleapis.com/css2?family=VT323&display=swap');

	.logo {
		font-family: 'VT323', monospace;
		font-size: 24px;
		font-weight: normal;
		letter-spacing: 2px;
		color: #7a1a1a;
		margin: 0;
		margin-left: 1rem;
	}
</style>
