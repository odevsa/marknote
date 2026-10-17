<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { Keyboard, Search, X, Command } from 'lucide-svelte';
  import { t } from '$lib/i18n';
  import { isShortcutsOpen, closeShortcuts } from '$lib/stores/ui';

  interface ShortcutItem {
    id: string;
    category: 'multicursor' | 'lines' | 'markdown' | 'search' | 'general';
    keys: string[][];
  }

  let searchQuery = $state('');
  let selectedCategory = $state<string>('all');
  let searchInputRef = $state<HTMLInputElement>();
  let isMac = $state(false);

  onMount(() => {
    if (typeof window !== 'undefined') {
      const platform = (navigator as any)?.userAgentData?.platform || navigator.platform || '';
      isMac = /Mac|iPhone|iPod|iPad/i.test(platform);
    }
  });

  $effect(() => {
    if ($isShortcutsOpen) {
      tick().then(() => {
        searchInputRef?.focus();
      });
    } else {
      searchQuery = '';
      selectedCategory = 'all';
    }
  });

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
    if (e.key === 'Escape' && $isShortcutsOpen) {
      closeShortcuts();
    }
  }

  function formatKey(key: string): string {
    if (key === 'Mod') return isMac ? '⌘' : 'Ctrl';
    if (key === 'Alt') return isMac ? '⌥' : 'Alt';
    if (key === 'Shift') return isMac ? '⇧' : 'Shift';
    return key;
  }

  const shortcuts: ShortcutItem[] = [
    // Multi-Cursor & Selection
    { id: 'selectNextOccurrence', category: 'multicursor', keys: [['Mod', 'D']] },
    { id: 'selectAllOccurrences', category: 'multicursor', keys: [['Mod', 'Shift', 'L']] },
    { id: 'addCursorBelow', category: 'multicursor', keys: [['Mod', 'Alt', '↓']] },
    { id: 'addCursorAbove', category: 'multicursor', keys: [['Mod', 'Alt', '↑']] },

    // Lines & Editing
    { id: 'duplicateLineDown', category: 'lines', keys: [['Shift', 'Alt', '↓']] },
    { id: 'duplicateLineUp', category: 'lines', keys: [['Shift', 'Alt', '↑']] },
    { id: 'moveLineDown', category: 'lines', keys: [['Alt', '↓']] },
    { id: 'moveLineUp', category: 'lines', keys: [['Alt', '↑']] },
    { id: 'deleteLine', category: 'lines', keys: [['Mod', 'Shift', 'K']] },
    { id: 'insertLineBelow', category: 'lines', keys: [['Mod', 'Enter']] },
    { id: 'undo', category: 'lines', keys: [['Mod', 'Z']] },
    { id: 'redo', category: 'lines', keys: [['Mod', 'Y'], ['Mod', 'Shift', 'Z']] },
    { id: 'indent', category: 'lines', keys: [['Tab']] },
    { id: 'unindent', category: 'lines', keys: [['Shift', 'Tab']] },

    // Markdown Formatting
    { id: 'bold', category: 'markdown', keys: [['Mod', 'B']] },
    { id: 'italic', category: 'markdown', keys: [['Mod', 'I']] },
    { id: 'inlineCode', category: 'markdown', keys: [['Mod', 'E']] },

    // Search & Navigation
    { id: 'quickSearch', category: 'search', keys: [['Mod', 'K']] },
    { id: 'find', category: 'search', keys: [['Mod', 'F']] },

    // General & System
    { id: 'save', category: 'general', keys: [['Mod', 'S']] },
    { id: 'openSettings', category: 'general', keys: [['Mod', ',']] },
    { id: 'openShortcuts', category: 'general', keys: [['F1'], ['Mod', '/']] },
    { id: 'close', category: 'general', keys: [['Esc']] }
  ];

  const categories = [
    { id: 'all', labelKey: 'shortcuts.all' },
    { id: 'multicursor', labelKey: 'shortcuts.categories.multicursor' },
    { id: 'lines', labelKey: 'shortcuts.categories.lines' },
    { id: 'markdown', labelKey: 'shortcuts.categories.markdown' },
    { id: 'search', labelKey: 'shortcuts.categories.search' },
    { id: 'general', labelKey: 'shortcuts.categories.general' }
  ];

  let filteredShortcuts = $derived.by(() => {
    const q = searchQuery.trim().toLowerCase();
    return shortcuts.filter((item) => {
      if (selectedCategory !== 'all' && item.category !== selectedCategory) {
        return false;
      }
      if (!q) return true;

      const desc = $t(`shortcuts.items.${item.id}`).toLowerCase();
      const catLabel = $t(`shortcuts.categories.${item.category}`).toLowerCase();
      const keysText = item.keys
        .map((kGroup) => kGroup.map(formatKey).join(' '))
        .join(' ')
        .toLowerCase();

      return desc.includes(q) || catLabel.includes(q) || keysText.includes(q);
    });
  });

  let groupedShortcuts = $derived.by(() => {
    const groups: { category: string; items: ShortcutItem[] }[] = [];
    const catOrder = ['multicursor', 'lines', 'markdown', 'search', 'general'];

    for (const cat of catOrder) {
      const items = filteredShortcuts.filter((s) => s.category === cat);
      if (items.length > 0) {
        groups.push({ category: cat, items });
      }
    }
    return groups;
  });
</script>

<svelte:window onkeydown={handleKeydown} />

{#if $isShortcutsOpen}
  <div
    use:portal
    class="fixed inset-0 z-50 flex items-center justify-center p-3 sm:p-6 bg-black/60 backdrop-blur-xs animate-in fade-in duration-150"
    onclick={(e) => {
      if (e.target === e.currentTarget) closeShortcuts();
    }}
    role="presentation"
  >
    <div
      class="relative w-full max-w-2xl max-h-[88vh] rounded-2xl border border-[var(--border-color)] bg-[var(--card-bg)] shadow-2xl flex flex-col overflow-hidden animate-in zoom-in-95 duration-150"
      role="dialog"
      aria-modal="true"
      aria-labelledby="shortcuts-dialog-title"
    >
      <!-- Modal Header -->
      <div class="px-5 py-4 border-b border-[var(--border-color)] flex items-center justify-between bg-[var(--bg-primary)] shrink-0">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-xl bg-[var(--accent)]/10 text-[var(--accent)]">
            <Keyboard size={20} />
          </div>
          <div>
            <h2 id="shortcuts-dialog-title" class="text-base font-bold text-[var(--text-primary)]">
              {$t('shortcuts.title')}
            </h2>
            <p class="text-xs text-[var(--text-muted)] mt-0.5">
              {$t('shortcuts.subtitle')}
            </p>
          </div>
        </div>

        <button
          onclick={closeShortcuts}
          class="p-1.5 rounded-lg text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-tertiary)] transition cursor-pointer"
          aria-label={$t('common.close')}
        >
          <X size={18} />
        </button>
      </div>

      <!-- Search & Category Filters -->
      <div class="p-4 border-b border-[var(--border-color)] bg-[var(--bg-secondary)]/50 space-y-3 shrink-0">
        <!-- Search bar -->
        <div class="relative">
          <Search size={16} class="absolute left-3.5 top-1/2 -translate-y-1/2 text-[var(--text-muted)] pointer-events-none" />
          <input
            bind:this={searchInputRef}
            type="text"
            bind:value={searchQuery}
            placeholder={$t('shortcuts.searchPlaceholder')}
            class="w-full pl-9 pr-8 py-2 rounded-xl border border-[var(--border-color)] bg-[var(--bg-primary)] text-[var(--text-primary)] text-sm placeholder-[var(--text-muted)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)] transition"
          />
          {#if searchQuery}
            <button
              onclick={() => (searchQuery = '')}
              class="absolute right-2.5 top-1/2 -translate-y-1/2 text-[var(--text-muted)] hover:text-[var(--text-primary)] p-0.5 rounded cursor-pointer"
            >
              <X size={14} />
            </button>
          {/if}
        </div>

        <!-- Category chips -->
        <div class="flex items-center gap-1.5 overflow-x-auto [scrollbar-width:none] [&::-webkit-scrollbar]:hidden pb-0.5">
          {#each categories as cat}
            <button
              onclick={() => (selectedCategory = cat.id)}
              class="px-2.5 py-1 rounded-lg text-xs font-medium whitespace-nowrap transition cursor-pointer {selectedCategory === cat.id
                ? 'bg-[var(--accent)] text-[var(--accent-text)] shadow-xs'
                : 'bg-[var(--bg-secondary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-tertiary)] border border-[var(--border-color)]/60'}"
            >
              {$t(cat.labelKey)}
            </button>
          {/each}
        </div>
      </div>

      <!-- Shortcuts List (Scrollable) -->
      <div class="flex-1 overflow-y-auto p-4 sm:p-5 space-y-6">
        {#if groupedShortcuts.length === 0}
          <div class="py-12 text-center text-sm text-[var(--text-muted)]">
            {$t('shortcuts.noResults')} "{searchQuery}"
          </div>
        {:else}
          {#each groupedShortcuts as group}
            <div class="space-y-2">
              <h3 class="text-xs font-bold uppercase tracking-wider text-[var(--accent)] px-1">
                {$t(`shortcuts.categories.${group.category}`)}
              </h3>

              <div class="rounded-xl border border-[var(--border-color)] bg-[var(--bg-secondary)]/30 divide-y divide-[var(--border-color)] overflow-hidden">
                {#each group.items as item}
                  <div class="px-3.5 py-2.5 flex items-center justify-between gap-4 hover:bg-[var(--bg-tertiary)]/50 transition">
                    <span class="text-sm text-[var(--text-primary)] font-normal">
                      {$t(`shortcuts.items.${item.id}`)}
                    </span>

                    <div class="flex items-center gap-1.5 shrink-0">
                      {#each item.keys as keyCombo, comboIdx}
                        {#if comboIdx > 0}
                          <span class="text-xs text-[var(--text-muted)] px-0.5">/</span>
                        {/if}
                        <div class="flex items-center gap-1">
                          {#each keyCombo as k}
                            <kbd
                              class="inline-flex items-center justify-center min-w-[24px] h-6 px-1.5 text-[11px] font-mono font-semibold rounded-md border border-[var(--border-color)] bg-[var(--bg-primary)] text-[var(--text-primary)] shadow-xs select-none"
                            >
                              {formatKey(k)}
                            </kbd>
                          {/each}
                        </div>
                      {/each}
                    </div>
                  </div>
                {/each}
              </div>
            </div>
          {/each}
        {/if}
      </div>

      <!-- Footer -->
      <div class="px-5 py-3 border-t border-[var(--border-color)] bg-[var(--bg-primary)] flex items-center justify-between text-xs text-[var(--text-muted)] shrink-0">
        <span class="flex items-center gap-1.5">
          <kbd class="px-1.5 py-0.5 text-[10px] font-mono rounded border border-[var(--border-color)] bg-[var(--bg-secondary)]">
            Esc
          </kbd>
          {$t('common.close')}
        </span>
        <span>MarkNote Editor</span>
      </div>
    </div>
  </div>
{/if}
