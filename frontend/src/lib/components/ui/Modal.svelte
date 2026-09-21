<script lang="ts">
  import { X } from 'lucide-svelte';
  import type { Snippet } from 'svelte';

  interface Props {
    title: string;
    isOpen: boolean;
    onClose: () => void;
    children?: Snippet;
  }

  let { title, isOpen, onClose, children }: Props = $props();

  function portal(node: HTMLElement) {
    document.body.appendChild(node);
    return {
      destroy() {
        if (node.parentNode) {
          node.parentNode.removeChild(node);
        }
      }
    };
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      onClose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
  <div use:portal class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-xs">
    <div
      class="relative w-full max-w-md rounded-xl border border-[var(--border-color)] bg-[var(--card-bg)] shadow-2xl overflow-hidden animate-in fade-in zoom-in-95 duration-150"
      role="dialog"
      aria-modal="true"
    >
      <div class="flex items-center justify-between px-5 py-4 border-b border-[var(--border-color)]">
        <h3 class="text-base font-semibold text-[var(--text-primary)]">{title}</h3>
        <button
          onclick={onClose}
          class="p-1 rounded-lg text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-tertiary)] transition"
          aria-label="Close"
        >
          <X size={18} />
        </button>
      </div>

      <div class="p-5">
        {@render children?.()}
      </div>
    </div>
  </div>
{/if}

