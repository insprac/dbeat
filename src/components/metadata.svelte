<script lang="ts">
    import type { Snippet } from "svelte";
    import { Icon, iconComponent } from "./icons";

    const { icon, children }: { icon: Icon; children: Snippet } = $props();

    const IconComponent = iconComponent(icon);
</script>

<div class="metadata">
    <span class="icon">
        <IconComponent width="1.4rem" />
    </span>
    <span class="value">
        {@render children()}
    </span>
</div>

<style>
    .metadata {
        color: #777;
        background-color: #222;
        display: flex;
        align-items: stretch;
        font-size: 1.1rem;
        border-radius: 0.5rem;
    }

    .icon {
        width: 3.5rem;
        flex-shrink: 0;
        background-color: #333;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 0.5rem;
    }

    .value {
        padding: .6rem 1rem;
        flex-grow: 1;
        display: flex;
        align-items: center;
        font-weight: 500;
    }
</style>
