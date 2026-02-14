<script lang="ts">
	import api from "$lib/api";
	import { user } from "$lib/auth";

	async function setRole(newRole: string) {
		try {
			// This hits the update_own_role function in admin.rs
			await api.put("/api/user/role", { role: newRole });
			window.location.reload(); // Reload to refresh session/permissions
		} catch (e) {
			console.error("Failed to switch role", e);
			alert("Failed to switch role");
		}
	}

	async function seedDB() {
		if (!confirm("Create dummy Faculty & Authority users?")) return;
		try {
			await api.post("/api/dev/seed");
			alert(
				"Users created! Reload the page to see them in the assignment list.",
			);
		} catch (e) {
			alert("Seeding failed");
		}
	}
</script>

<div class="dev-dock">
	<span class="label">DEV MODE:</span>
	<div class="buttons">
		<button
			onclick={() => setRole("student")}
			class:active={$user?.role === "student"}>STU</button
		>
		<button
			onclick={() => setRole("faculty")}
			class:active={$user?.role === "faculty"}>FAC</button
		>
		<button
			onclick={() => setRole("authority")}
			class:active={$user?.role === "authority"}>AUT</button
		>
		<button
			onclick={() => setRole("admin")}
			class:active={$user?.role === "admin"}>ADM</button
		>
		<button onclick={seedDB} style="color: cyan; border-color: cyan;"
			>SEED USERS</button
		>
	</div>
</div>

<style>
	.dev-dock {
		position: fixed;
		bottom: 20px;
		left: 20px;
		background: #000;
		border: 2px solid #0f0; /* Hacker green for dev mode */
		padding: 8px;
		z-index: 9999;
		display: flex;
		gap: 12px;
		align-items: center;
		font-family: monospace;
		box-shadow: 4px 4px 0px rgba(0, 0, 0, 0.5);
	}
	.label {
		color: #0f0;
		font-weight: bold;
	}
	.buttons {
		display: flex;
		gap: 4px;
	}
	button {
		background: #222;
		color: #fff;
		border: 1px solid #555;
		cursor: pointer;
		padding: 4px 8px;
		font-family: monospace;
	}
	button:hover {
		background: #444;
	}
	button.active {
		background: #0f0;
		color: #000;
		border-color: #0f0;
		font-weight: bold;
	}
</style>
