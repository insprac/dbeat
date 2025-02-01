<script lang="ts">
    import type { Snippet } from "svelte";
    import { Icon, iconComponent } from "./icons";

    const {
        icon,
        tooltip,
        children,
    }: { icon: Icon; tooltip?: string; children: Snippet } = $props();

    const IconComponent = iconComponent(icon);
</script>

<div class="icon-label">
    <span class="icon">
        <IconComponent width="1.4rem" />
    </span>
    <span class="label">
        {@render children()}
    </span>
    {#if tooltip}
        <div class="tooltip">{tooltip}</div>
    {/if}
</div>

<style>
    .icon-label {
        position: relative;
        display: flex;
        align-items: flex-start;
        gap: 0.3rem;
        font-size: 1.1rem;
    }

    .icon {
        width: 1.4rem;
        flex-shrink: 0;
    }

    .tooltip {
        position: absolute;
        top: 0;
        left: 0;
        transform: translateY(calc(-100% - .4rem));
        display: none;
        background-color: #444;
        color: #bbb;
        padding: .3rem .5rem;
        border-radius: .3rem;
        box-shadow: 2rem .2rem .2rem rgba(0, 0, 0, 1);
    }

    .tooltip:after {
        content: "";
        display: block;
        position: absolute;
        bottom: 0;
        left: .47rem;
        width: .5rem;
        height: .5rem;
        transform: translateY(50%) rotate(45deg);
        background-color: #444;
    }

    .icon-label:hover .tooltip {
        display: block;
    }
</style>
