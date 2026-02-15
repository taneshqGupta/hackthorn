<script lang="ts">
    import { user } from "$lib/auth";
    import { goto } from "$app/navigation";

    let currentUser = $derived($user);

    $effect(() => {
        if (currentUser && currentUser.role !== "student") {
            goto(`/dashboard/${currentUser.role}`);
        } else if (!currentUser) {
            goto("/login");
        }
    });
</script>

<div class="hub-container">
    <header class="header">
        <h1>THE_STUDENT_CONCOURSE</h1>
        <p>
            GREETINGS, {currentUser?.first_name?.toUpperCase()}. SELECT YOUR
            PATH.
        </p>
    </header>

    <div class="gateway-grid">
        <button
            onclick={() => goto("/dashboard/student/grievances")}
            class="gateway-card red"
        >
            <div class="card-inner">
                <span class="pillar-no">PILLAR_II</span>
                <h2>THE_SILENT_SCROLL</h2>
                <p>SUBMIT GRIEVANCES & TRACK RESOLUTIONS.</p>
            </div>
        </button>

        <button
            onclick={() => goto("/dashboard/student/courses")}
            class="gateway-card yellow"
        >
            <div class="card-inner">
                <span class="pillar-no">PILLAR_III</span>
                <h2>THE_DESTINY_MANAGER</h2>
                <p>ENROLLMENTS, CREDITS, & ATTENDANCE LOGS.</p>
            </div>
        </button>

        <button
            onclick={() => goto("/dashboard/student/vault")}
            class="gateway-card pink"
        >
            <div class="card-inner">
                <span class="pillar-no">PILLAR_III_EXTRA</span>
                <h2>THE_VAULT</h2>
                <p>RECORDS, PYQS, & ACADEMIC ARTIFACTS.</p>
            </div>
        </button>
    </div>
</div>

<style>
    .hub-container {
        width: 100%;
        max-width: 1000px;
        margin: 0 auto;
        padding: 20px;
        box-sizing: border-box; /* Ensures padding doesn't add to width */
    }

    /* ... Header styles remain the same ... */

    .gateway-grid {
        display: grid;
        /* Changed from 300px to 100% for mobile, then 300px for desktop */
        grid-template-columns: 1fr;
        gap: 20px;
        width: 100%;
    }

    .gateway-card {
        background: transparent;
        border: 4px solid #2b0b0b;
        padding: 0;
        cursor: pointer;
        text-align: left;
        box-shadow: 6px 6px 0px #2b0b0b;
        transition: 150ms ease-in-out;
        /* Force the card to never exceed its parent */
        max-width: 100%;
        box-sizing: border-box;
    }

    .card-inner {
        padding: 20px;
        word-wrap: break-word; /* Prevents long text from pushing the box out */
    }

    /* Desktop and Tablet Adjustments */
    @media (min-width: 768px) {
        .gateway-grid {
            /* Now we allow them to sit side-by-side */
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
            gap: 30px;
        }

        .gateway-card {
            box-shadow: 8px 8px 0px #2b0b0b;
        }

        .card-inner {
            padding: 25px;
        }
    }
</style>
