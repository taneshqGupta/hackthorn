<script lang="ts">
	import { goto } from '$app/navigation';
</script>

<button on:click={() => goto('/')} class="cursor-pointer bg-transparent border-none p-2">
	<h1 class="logo">AEGIS</h1>
</button>

<style>
	@import url('https://fonts.googleapis.com/css2?family=VT323&display=swap');

	.logo {
		font-family: 'VT323', monospace;
		font-size: 24px;
		font-weight: normal;
		letter-spacing: 2px;
		color: #d06065;
		margin: 0;
	}
</style>
