<script lang="ts">
    interface Props {
        label: string;
        value: string | number;
        subtext?: string;
        color?: string; // Default changed to transparent
    }

    let { label, value, subtext, color = "transparent" }: Props = $props();
</script>

<div class="stats-card" style:background-color={color}>
    <div class="label">{label}</div>
    <div class="value">{value}</div>
    {#if subtext}
        <div class="subtext">{subtext}</div>
    {/if}
</div>

<style>
    .stats-card {
        border: 4px solid #000; /* Slightly thicker for brutalist pop */
        padding: 1.5rem;
        box-shadow: 6px 6px 0px #000;
        display: flex;
        flex-direction: column; /* Fixed typo from flex-col */
        align-items: center;
        justify-content: center;
        text-align: center;
        min-width: 150px;
        /* Ensures the card doesn't hide the notebook texture */
        background-color: transparent; 
    }

    .label {
        font-family: monospace;
        font-weight: 900;
        text-transform: uppercase;
        font-size: 0.8rem;
        margin-bottom: 0.5rem;
        background: #000; /* Inverted label for neobrutalist look */
        color: #fff;
        padding: 0 4px;
    }

    .value {
        font-size: 3.5rem;
        font-weight: 950;
        line-height: 1;
        letter-spacing: -2px;
    }

    .subtext {
        font-weight: 700;
        font-size: 0.75rem;
        margin-top: 0.5rem;
        text-transform: uppercase;
        border-top: 2px dashed #000; /* Hand-drawn look */
        padding-top: 4px;
        width: 100%;
    }
</style>