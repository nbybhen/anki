<!--
Copyright: Ankitects Pty Ltd and contributors
License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html
-->
<script lang="ts">
    import IconButton from "$lib/components/IconButton.svelte";

    import { mdiClose } from "./icons";

    export let type: "success" | "error" = "success";
    export let message;
    export let showToast = false;
    const closeToast = () => {
        showToast = false;
    };
</script>

{#if showToast}
    <div class="toast-container desktop-only">
        <div class="toast {type === 'success' ? 'success' : 'error'}">
            {message}
            <IconButton iconSize={96} on:click={closeToast} class="toast-icon">
                {@html mdiClose}
            </IconButton>
        </div>
    </div>
{/if}

<style>
    .toast-container {
        position: fixed;
        bottom: 3rem;
        z-index: 100;
        width: 100%;
        text-align: center;
        display: flex;
        justify-content: center;
    }
    .toast {
        display: flex;
        align-items: center;
        padding: 1rem;
        background-color: #fff;
        border-radius: 0.5rem;
        box-shadow: 0 0.5rem 1rem rgba(0, 0, 0, 0.1);
        width: 60%;
        justify-content: space-between;
    }
    .success {
        background: #66bb6a;
        color: white;
    }
    .error {
        background: #ef5350;
        color: white;
    }
    :global(.toast-icon) {
        background: unset !important;
        color: white !important;
        border: none !important;
    }
</style>
