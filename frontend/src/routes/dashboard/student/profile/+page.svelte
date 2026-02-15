<script lang="ts">
    import { user } from "$lib/auth";
    import { logout } from "$lib/api";
    import { goto } from "$app/navigation";
    import type { User } from "$lib/types";

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
            goto("/login");
        } catch (error) {
            console.error("Logout failed:", error);
        }
    }

    function goToDashboard() {
        goto("/dashboard/student");
    }
</script>

<div class="page-container">
    {#if currentUser}
        <div class="neo-card">
            <div class="card-header">
                <div class="avatar-frame">
                    <img
                        src={currentUser.profile_picture ||
                            `https://ui-avatars.com/api/?name=${getFullName(currentUser)}&background=random&bold=true`}
                        alt="user avatar"
                    />
                </div>
                <div class="user-info">
                    <h1 class="user-name">{getFullName(currentUser)}</h1>
                    <div class="badges">
                        <span class="role-badge">{currentUser.role}</span>
                    </div>
                </div>
            </div>

            <div class="details-list">
                <div class="detail-item">
                    <span class="label">EMAIL</span>
                    <span class="value">{currentUser.email}</span>
                </div>
                {#if currentUser.department}
                    <div class="detail-item">
                        <span class="label">DEPT</span>
                        <span class="value">{currentUser.department}</span>
                    </div>
                {/if}
            </div>

            <div class="actions">
                <button class="neo-btn dashboard" onclick={goToDashboard}>
                    Open Dashboard
                </button>
                <button class="neo-btn logout" onclick={handleLogout}>
                    Logout
                </button>
            </div>
        </div>
    {:else}
        <div class="neo-card simple">
            <p class="text-xl font-bold uppercase mb-4">Access Denied</p>
            <button class="neo-btn" onclick={() => goto("/login")}
                >Go to Login</button
            >
        </div>
    {/if}
</div>

<style>
    /* Full Page Center Alignment */
    .page-container {
        min-height: 100vh;
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 1rem;
    }

    /* NEOBRUTALIST CARD */
    .neo-card {
        width: 100%;
        max-width: 450px;

        /* UPDATED: Much higher transparency (0.15 alpha) */
        background-color: rgba(255, 255, 255, 0.15);
        backdrop-filter: blur(
            4px
        ); /* Blur helps text read over notebook lines */

        /* Hard Borders & Shadows */
        border: 3px solid #000;
        box-shadow: 8px 8px 0 rgba(0, 0, 0, 1);

        padding: 2.5rem;
        display: flex;
        flex-direction: column;
        gap: 2rem;

        /* Hover Reactivity */
        transition: all 0.3s cubic-bezier(0.25, 0.8, 0.25, 1);
        transform: translate(0, 0);
    }

    .neo-card.simple {
        align-items: center;
        text-align: center;
    }

    /* HOVER STATE: Pops up and becomes semi-solid white */
    .neo-card:hover {
        background-color: rgba(
            255,
            255,
            255,
            0.85
        ); /* Increases opacity on hover */
        transform: translate(-4px, -4px);
        box-shadow: 12px 12px 0 rgba(0, 0, 0, 1);
        border-color: #000;
    }

    /* --- Header --- */
    .card-header {
        display: flex;
        flex-direction: column;
        align-items: center;
        text-align: center;
        gap: 1rem;
    }

    .avatar-frame {
        width: 100px;
        height: 100px;
        border: 3px solid #000;
        border-radius: 50%;
        overflow: hidden;
        background: #fff;
    }

    .avatar-frame img {
        width: 100%;
        height: 100%;
        object-fit: cover;
    }

    .user-name {
        font-family: "Oswald", sans-serif;
        font-size: 2rem;
        font-weight: 800;
        text-transform: uppercase;
        line-height: 1;
        margin: 0;
        color: #000;
    }

    .role-badge {
        display: inline-block;
        background: #000;
        color: #fff;
        padding: 0.2rem 0.6rem;
        font-size: 0.8rem;
        font-weight: 700;
        text-transform: uppercase;
        border: 2px solid #000;
        margin-top: 0.5rem;
    }

    /* --- Details --- */
    .details-list {
        display: flex;
        flex-direction: column;
        gap: 0.8rem;
        border-top: 2px dashed rgba(0, 0, 0, 0.5); /* Darker dash for contrast on transparent bg */
        border-bottom: 2px dashed rgba(0, 0, 0, 0.5);
        padding: 1.5rem 0;
    }

    .detail-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        font-size: 1rem;
    }

    .label {
        font-weight: 800;
        opacity: 0.8; /* Increased opacity for readability */
        font-size: 0.85rem;
        text-transform: uppercase;
        color: #000;
    }

    .value {
        font-weight: 600;
        text-align: right;
        color: #000;
    }

    /* --- Buttons --- */
    .actions {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .neo-btn {
        width: 100%;
        padding: 1rem;
        font-weight: 700;
        font-size: 1rem;
        text-transform: uppercase;
        border: 3px solid #000;
        cursor: pointer;
        transition: all 0.2s ease;
        font-family: inherit;
    }

    .neo-btn.dashboard {
        background: #2b0b0b;
        color: #fff;
        box-shadow: 4px 4px 0 rgba(0, 0, 0, 0.3);
    }

    .neo-btn.dashboard:hover {
        background: #4a1a1a;
        transform: translate(-2px, -2px);
        box-shadow: 6px 6px 0 rgba(0, 0, 0, 0.4);
    }

    .neo-btn.logout {
        background: rgba(255, 255, 255, 0.5); /* Semi-transparent button bg */
        color: #d06065;
        border-color: #d06065;
    }

    .neo-btn.logout:hover {
        background: #d06065;
        color: #fff;
    }
</style>
