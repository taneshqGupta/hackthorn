<script lang="ts">
    import { user } from '$lib/auth';
    import { goto } from '$app/navigation';

    let currentUser = $derived($user);

    $effect(() => {
        if (currentUser && currentUser.role !== 'student') {
            goto(`/dashboard/${currentUser.role}`);
        } else if (!currentUser) {
            goto('/login');
        }
    });
</script>

<div class="hub-container">
    <header class="header">
        <h1>THE_STUDENT_CONCOURSE</h1>
        <p>GREETINGS, {currentUser?.first_name?.toUpperCase()}. SELECT YOUR PATH.</p>
    </header>

    <div class="gateway-grid">
        <button onclick={() => goto('/dashboard/student/grievances')} class="gateway-card red">
            <div class="card-inner">
                <span class="pillar-no">PILLAR_II</span>
                <h2>THE_SILENT_SCROLL</h2>
                <p>SUBMIT GRIEVANCES & TRACK RESOLUTIONS.</p>
            </div>
        </button>

        <button onclick={() => goto('/dashboard/student/academic/courses')} class="gateway-card yellow">
            <div class="card-inner">
                <span class="pillar-no">PILLAR_III</span>
                <h2>THE_DESTINY_MANAGER</h2>
                <p>ENROLLMENTS, CREDITS, & ATTENDANCE LOGS.</p>
            </div>
        </button>

        <button onclick={() => goto('/dashboard/student/academic/vault')} class="gateway-card pink">
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
        max-width: 1000px;
        margin: 0 auto;
        padding: 40px 20px;
    }

    .header { text-align: center; margin-bottom: 50px; }
    h1 { font-size: 3.5rem; font-weight: 950; margin: 0; color: #2b0b0b; line-height: 1; }
    p { font-weight: 700; opacity: 0.7; }

    .gateway-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        gap: 30px;
    }

    .gateway-card {
        background: transparent;
        border: 4px solid #2b0b0b;
        padding: 0;
        cursor: pointer;
        text-align: left;
        box-shadow: 8px 8px 0px #2b0b0b;
        transition: 150ms ease-in-out;
    }

    .card-inner { padding: 25px; }

    .gateway-card:hover { transform: translate(-4px, -4px); box-shadow: 12px 12px 0px #2b0b0b; }
    .gateway-card:active { transform: translate(2px, 2px); box-shadow: 4px 4px 0px #2b0b0b; }

    .pillar-no { font-family: monospace; font-weight: 900; background: #2b0b0b; color: #fff; padding: 2px 6px; font-size: 0.8rem; }
    h2 { font-size: 1.8rem; font-weight: 950; margin: 15px 0 10px 0; }
    p { font-weight: 600; font-size: 0.9rem; }

    .red:hover { background: #ff4d4d; }
    .yellow:hover { background: #ffbc42; }
    .pink:hover { background: #ff8fa3; }
</style>