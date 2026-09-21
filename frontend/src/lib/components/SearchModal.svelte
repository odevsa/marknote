<script lang="ts">
  import { tick } from 'svelte';
  import { Search, FileText, X } from 'lucide-svelte';
  import { t } from '$lib/i18n';
  import {
    searchQuery,
    searchResults,
    isSearching,
    isSearchOpen,
    performSearch,
    openNote
  } from '$lib/stores/notes';

  let inputRef = $state<HTMLInputElement>();

  $effect(() => {
    if ($isSearchOpen) {
      tick().then(() => {
        inputRef?.focus();
        inputRef?.select();
      });
    }
  });

  function focusOnMount(node: HTMLElement) {
    requestAnimationFrame(() => {
      node.focus();
    });
  }

  function handleClose() {
    isSearchOpen.set(false);
    performSearch('');
  }

  function handleSelect(path: string) {
    openNote(path);
    handleClose();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      handleClose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if $isSearchOpen}
  <div class="fixed inset-0 z-50 flex items-start justify-center p-4 pt-16 bg-black/60 backdrop-blur-xs">
    <div
      class="relative w-full max-w-xl rounded-xl border border-[var(--border-color)] bg-[var(--card-bg)] shadow-2xl overflow-hidden animate-in fade-in zoom-in-95 duration-150"
      role="dialog"
      aria-modal="true"
    >
      <!-- Search Input Bar -->
      <div class="flex items-center px-4 py-3 border-b border-[var(--border-color)] gap-3 bg-[var(--bg-primary)]">
        <Search size={18} class="text-[var(--text-muted)] shrink-0" />
        <input
          bind:this={inputRef}
          use:focusOnMount
          type="text"
          value={$searchQuery}
          oninput={(e) => performSearch((e.target as HTMLInputElement).value)}
          placeholder={$t('common.searchPlaceholder')}
          class="w-full bg-transparent text-[var(--text-primary)] placeholder-[var(--text-muted)] text-sm focus:outline-none"
        />
        {#if $isSearching}
          <div class="w-4 h-4 border-2 border-[var(--accent)] border-t-transparent rounded-full animate-spin shrink-0"></div>
        {/if}
        <button
          onclick={handleClose}
          class="p-1 rounded-lg text-[var(--text-muted)] hover:text-[var(--text-primary)] transition"
        >
          <X size={18} />
        </button>
      </div>

      <!-- Search Results List -->
      <div class="max-h-96 overflow-y-auto p-2">
        {#if $searchQuery.trim() && $searchResults.length === 0 && !$isSearching}
          <div class="py-8 text-center text-sm text-[var(--text-muted)]">
            {$t('common.noResults')}
          </div>
        {:else}
          {#each $searchResults as result}
            <button
              onclick={() => handleSelect(result.path)}
              class="w-full text-left p-3 rounded-lg hover:bg-[var(--bg-tertiary)] transition group mb-1"
            >
              <div class="flex items-center gap-2 mb-1">
                <FileText size={15} class="text-[var(--accent)] shrink-0" />
                <span class="text-sm font-semibold text-[var(--text-primary)] group-hover:text-[var(--accent)] transition">
                  {result.title}
                </span>
                <span class="text-xs text-[var(--text-muted)] font-mono ml-auto">
                  {result.path}
                </span>
              </div>
              {#if result.snippet}
                <div class="text-xs text-[var(--text-muted)] line-clamp-2 pl-6 leading-relaxed">
                  {@html result.snippet}
                </div>
              {/if}
            </button>
          {/each}
        {/if}
      </div>
    </div>
  </div>
{/if}
