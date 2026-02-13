<script lang="ts">
	import '../app.css';
	let { children } = $props();
</script>

<div class="h-screen flex flex-col overflow-hidden container">
	<header
		class="flex-none w-full border-b-2 flex items-center justify-center"
		style="border-color: #d06065; height: 40px;"
	>
	</header>

	<main class="flex-1 overflow-auto">
		{@render children()}
	</main>

	<footer
		class="flex-none w-full border-t-2 flex items-center justify-center"
		style="border-color: #d06065; background-color: rgba(255, 179, 186, 0.2); height: 40px;"
	>
	</footer>
</div>