<script lang="ts">
    import { user } from '$lib/auth';
    import { logout } from '$lib/api';
    import { goto } from '$app/navigation';
    import type { User } from '$lib/types';

    let currentUser: User | null = null;
    user.subscribe((value) => {
        currentUser = value;
    });

    // Helper to get full name since the interface splits it
    const getFullName = (u: User) => `${u.first_name} ${u.last_name}`;

    async function handleLogout() {
        try {
            await logout();
            user.set(null);
            goto('/login');
        } catch (error) {
            console.error('Logout failed:', error);
        }
    }
</script>

<div class="p-4">
    {#if currentUser}
        <div class="flex items-center justify-between">
            <div class="flex items-center space-x-4">
                <div class="avatar">
                    <div class="w-24 rounded-full">
                        <img
                            src={currentUser.profile_picture ||
                                `https://ui-avatars.com/api/?name=${getFullName(currentUser)}&background=random`}
                            alt="user avatar"
                        />
                    </div>
                </div>
                <div>
                    <h1 class="text-2xl font-bold">{getFullName(currentUser)}</h1>
                    <p>{currentUser.email}</p>
                    <p class="badge badge-outline uppercase text-xs">{currentUser.role}</p>
                </div>
            </div>
            <button
                on:click={handleLogout}
                class="btn btn-outline btn-error"
            >
                Logout
            </button>
        </div>
    {:else}
        <p>Please log in to view your profile.</p>
    {/if}
</div>