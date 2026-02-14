<script lang="ts">
	import { goto } from '$app/navigation';
</script>

<div on:click={() => goto('/')} class="cursor-pointer">
	<h1 class="logo">AEGIS</h1>
</div>

<style>
	@import url('https://fonts.googleapis.com/css2?family=VT323&display=swap');

	.logo {
		font-family: 'VT323', monospace;
		font-size: 24px;
		font-weight: normal;
		letter-spacing: 2px;
		color: #d06065;
	}
</style>
