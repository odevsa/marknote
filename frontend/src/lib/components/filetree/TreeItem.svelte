<script lang="ts">
  import {
    Folder,
    FolderOpen,
    FileText,
    ChevronRight,
    ChevronDown,
    MoreVertical,
    FilePlus,
    FolderPlus,
    Edit2,
    Trash2
  } from 'lucide-svelte';
  import type { FileTreeNode } from '$lib/api/client';
  import { activeNote, openNote, isDirty, hasUnsavedDraft } from '$lib/stores/notes';
  import { activeTreeMenuPath } from '$lib/stores/ui';
  import { t } from '$lib/i18n';
  import TreeItem from './TreeItem.svelte';

  interface Props {
    item: FileTreeNode;
    depth?: number;
    onCreateFile: (parentPath: string) => void;
    onCreateFolder: (parentPath: string) => void;
    onRename: (path: string, currentName: string) => void;
    onDelete: (path: string, isDir: boolean) => void;
  }

  let {
    item,
    depth = 0,
    onCreateFile,
    onCreateFolder,
    onRename,
    onDelete
  }: Props = $props();

  let isOpen = $state(false);
  let isMenuOpen = $derived($activeTreeMenuPath === item.path);

  // Auto-expand folder if it contains the currently active note (e.g. on page reload/navigation)
  $effect(() => {
    const activePath = $activeNote?.path;
    if (item.is_dir && activePath && activePath.startsWith(item.path + '/')) {
      isOpen = true;
    }
  });

  let touchTimer: ReturnType<typeof setTimeout> | null = null;
  let isLongPress = false;

  function toggleOpen(e: Event) {
    e.stopPropagation();
    isOpen = !isOpen;
  }

  function handleSelect() {
    if (isLongPress) {
      isLongPress = false;
      return;
    }
    if (!item.is_dir) {
      openNote(item.path, 'preview');
    } else {
      isOpen = !isOpen;
    }
  }

  function toggleMenu(e: Event) {
    e.stopPropagation();
    if ($activeTreeMenuPath === item.path) {
      activeTreeMenuPath.set(null);
    } else {
      activeTreeMenuPath.set(item.path);
    }
  }

  function closeMenu() {
    if ($activeTreeMenuPath === item.path) {
      activeTreeMenuPath.set(null);
    }
  }

  function handleTouchStart() {
    isLongPress = false;
    if (touchTimer) clearTimeout(touchTimer);
    touchTimer = setTimeout(() => {
      isLongPress = true;
      activeTreeMenuPath.set(item.path);
    }, 450);
  }

  function handleTouchEnd() {
    if (touchTimer) {
      clearTimeout(touchTimer);
      touchTimer = null;
    }
  }

  function handleTouchMove() {
    if (touchTimer) {
      clearTimeout(touchTimer);
      touchTimer = null;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    const rowEl = e.currentTarget as HTMLElement;
    const treeContainer = rowEl.closest('.file-tree-container');
    const allVisible = Array.from(treeContainer?.querySelectorAll<HTMLElement>('.tree-item-row') || []);
    const currentIndex = allVisible.indexOf(rowEl);

    if (e.key === 'ArrowDown') {
      e.preventDefault();
      if (currentIndex >= 0 && currentIndex < allVisible.length - 1) {
        const target = allVisible[currentIndex + 1];
        target?.focus();
        target?.scrollIntoView({ block: 'nearest' });
      }
      return;
    }

    if (e.key === 'ArrowUp') {
      e.preventDefault();
      if (currentIndex > 0) {
        const target = allVisible[currentIndex - 1];
        target?.focus();
        target?.scrollIntoView({ block: 'nearest' });
      }
      return;
    }

    if (e.key === 'PageDown') {
      e.preventDefault();
      if (currentIndex >= 0) {
        const nextIdx = Math.min(currentIndex + 8, allVisible.length - 1);
        const target = allVisible[nextIdx];
        target?.focus();
        target?.scrollIntoView({ block: 'nearest' });
      }
      return;
    }

    if (e.key === 'PageUp') {
      e.preventDefault();
      if (currentIndex >= 0) {
        const prevIdx = Math.max(currentIndex - 8, 0);
        const target = allVisible[prevIdx];
        target?.focus();
        target?.scrollIntoView({ block: 'nearest' });
      }
      return;
    }

    if (e.key === 'Home') {
      e.preventDefault();
      const target = allVisible[0];
      target?.focus();
      target?.scrollIntoView({ block: 'nearest' });
      return;
    }

    if (e.key === 'End') {
      e.preventDefault();
      const target = allVisible[allVisible.length - 1];
      target?.focus();
      target?.scrollIntoView({ block: 'nearest' });
      return;
    }

    if (e.key === 'ArrowRight') {
      e.preventDefault();
      if (item.is_dir) {
        if (!isOpen) {
          isOpen = true;
        } else if (currentIndex >= 0 && currentIndex < allVisible.length - 1) {
          const target = allVisible[currentIndex + 1];
          target?.focus();
          target?.scrollIntoView({ block: 'nearest' });
        }
      }
      return;
    }

    if (e.key === 'ArrowLeft') {
      e.preventDefault();
      if (item.is_dir && isOpen) {
        isOpen = false;
      } else {
        const lastSlash = item.path.lastIndexOf('/');
        if (lastSlash > 0) {
          const parentPath = item.path.substring(0, lastSlash);
          const parentEl = treeContainer?.querySelector<HTMLElement>(`.tree-item-row[data-path="${CSS.escape(parentPath)}"]`);
          if (parentEl) {
            parentEl.focus();
            parentEl.scrollIntoView({ block: 'nearest' });
          }
        }
      }
      return;
    }

    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      handleSelect();
      return;
    }

    if (e.key === 'F2') {
      e.preventDefault();
      e.stopPropagation();
      onRename(item.path, item.name);
      return;
    }

    if (e.key === 'Delete') {
      e.preventDefault();
      e.stopPropagation();
      onDelete(item.path, item.is_dir);
      return;
    }

    // Typeahead navigation (search visible item by starting letter)
    if (e.key.length === 1 && !e.ctrlKey && !e.metaKey && !e.altKey && e.key !== ' ') {
      const char = e.key.toLowerCase();
      const nextMatches = allVisible.slice(currentIndex + 1).concat(allVisible.slice(0, currentIndex));
      const match = nextMatches.find((el) => {
        const text = el.querySelector('span')?.textContent?.trim().toLowerCase();
        return text && text.startsWith(char);
      });
      if (match) {
        e.preventDefault();
        match.focus();
        match.scrollIntoView({ block: 'nearest' });
        return;
      }
    }
  }
</script>

<svelte:window onclick={() => activeTreeMenuPath.set(null)} />

<div class="select-none relative">
  <div
    onclick={handleSelect}
    ontouchstart={handleTouchStart}
    ontouchend={handleTouchEnd}
    ontouchmove={handleTouchMove}
    class="tree-item-row flex items-center group px-2 py-1.5 rounded-lg text-sm sm:text-xs transition cursor-pointer relative outline-none focus-visible:ring-1.5 focus-visible:ring-[var(--accent)] {
      !item.is_dir && $activeNote?.path === item.path
        ? 'bg-[var(--sidebar-active-bg)] text-[var(--sidebar-active-text)] font-semibold'
        : 'text-[var(--text-primary)] hover:bg-[var(--sidebar-hover)] focus:bg-[var(--sidebar-hover)]'
    }"
    style="padding-left: {depth * 14 + 8}px;"
    role="treeitem"
    aria-expanded={item.is_dir ? isOpen : undefined}
    aria-selected={!item.is_dir && $activeNote?.path === item.path}
    data-path={item.path}
    tabindex="0"
    onkeydown={handleKeydown}
  >
    <!-- Folder arrow toggle or spacer -->
    {#if item.is_dir}
      <button
        onclick={toggleOpen}
        class="mr-1 p-0.5 rounded hover:bg-black/10 transition shrink-0 cursor-pointer"
        aria-label="Toggle folder"
        tabindex="-1"
      >
        {#if isOpen}
          <ChevronDown size={16} class="opacity-70 sm:size-[14px]" />
        {:else}
          <ChevronRight size={16} class="opacity-70 sm:size-[14px]" />
        {/if}
      </button>
      {#if isOpen}
        <FolderOpen size={18} class="mr-2 text-[var(--accent)] shrink-0 sm:size-[16px]" />
      {:else}
        <Folder size={18} class="mr-2 text-[var(--accent)] shrink-0 sm:size-[16px]" />
      {/if}
    {:else}
      <span class="w-4 mr-1 shrink-0"></span>
      <FileText
        size={17}
        class="mr-2 shrink-0 sm:size-[15px] {!item.is_dir && $activeNote?.path === item.path ? 'text-[var(--sidebar-active-text)]' : 'text-[var(--text-muted)]'}"
      />
    {/if}

    <!-- Name -->
    <span class="truncate flex-1 text-sm sm:text-xs">{item.name}</span>

    <!-- Unsaved indicator dot -->
    {#if !item.is_dir && (($activeNote?.path === item.path && $isDirty) || ($activeNote?.path !== item.path && hasUnsavedDraft(item.path)))}
      <span class="w-2 h-2 rounded-full bg-[var(--accent)] shrink-0 ml-1.5 shadow-xs" title="Unsaved changes"></span>
    {/if}

    <!-- Item Actions Trigger (Visible on mobile, hover on desktop) -->
    <div class="opacity-100 md:opacity-0 md:group-hover:opacity-100 transition ml-1 shrink-0">
      <button
        onclick={toggleMenu}
        class="p-1 rounded hover:bg-black/10 transition text-inherit cursor-pointer"
        title="Options"
        aria-label="Options"
        tabindex="-1"
      >
        <MoreVertical size={18} class="sm:size-[14px]" />
      </button>
    </div>

    <!-- Actions Dropdown (Positioned right below 3-dots) -->
    {#if isMenuOpen}
      <div
        class="absolute right-2 top-7 z-50 w-44 py-1 rounded-lg border border-[var(--border-color)] bg-[var(--card-bg)] shadow-xl text-[var(--text-primary)] text-sm sm:text-xs"
        onclick={(e) => e.stopPropagation()}
        onkeydown={(e) => {
          if (e.key === 'Escape') closeMenu();
        }}
        role="menu"
        tabindex="0"
      >
        {#if item.is_dir}
          <button
            onclick={() => { closeMenu(); onCreateFile(item.path); }}
            class="w-full flex items-center gap-2.5 px-3 py-2 sm:py-1.5 hover:bg-[var(--bg-tertiary)] text-left cursor-pointer"
            role="menuitem"
          >
            <FilePlus size={16} class="text-[var(--accent)] shrink-0 sm:size-[14px]" />
            <span>{$t('common.newNote')}</span>
          </button>
          <button
            onclick={() => { closeMenu(); onCreateFolder(item.path); }}
            class="w-full flex items-center gap-2.5 px-3 py-2 sm:py-1.5 hover:bg-[var(--bg-tertiary)] text-left cursor-pointer"
            role="menuitem"
          >
            <FolderPlus size={16} class="text-[var(--accent)] shrink-0 sm:size-[14px]" />
            <span>{$t('common.newFolder')}</span>
          </button>
          <div class="my-1 border-t border-[var(--border-color)]"></div>
        {/if}
        <button
          onclick={() => { closeMenu(); onRename(item.path, item.name); }}
          class="w-full flex items-center gap-2.5 px-3 py-2 sm:py-1.5 hover:bg-[var(--bg-tertiary)] text-left cursor-pointer"
          role="menuitem"
        >
          <Edit2 size={16} class="shrink-0 sm:size-[14px]" />
          <span>{$t('common.rename')}</span>
        </button>
        <button
          onclick={() => { closeMenu(); onDelete(item.path, item.is_dir); }}
          class="w-full flex items-center gap-2.5 px-3 py-2 sm:py-1.5 hover:bg-[var(--bg-tertiary)] text-[var(--danger)] text-left cursor-pointer"
          role="menuitem"
        >
          <Trash2 size={16} class="shrink-0 sm:size-[14px]" />
          <span>{$t('common.delete')}</span>
        </button>
      </div>
    {/if}
  </div>

  <!-- Children if directory and open -->
  {#if item.is_dir && isOpen && item.children && item.children.length > 0}
    <div>
      {#each item.children as child (child.path)}
        <TreeItem
          item={child}
          depth={depth + 1}
          {onCreateFile}
          {onCreateFolder}
          {onRename}
          {onDelete}
        />
      {/each}
    </div>
  {/if}
</div>
