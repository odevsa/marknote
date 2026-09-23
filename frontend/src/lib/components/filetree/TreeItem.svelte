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
      openNote(item.path);
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
</script>

<svelte:window onclick={() => activeTreeMenuPath.set(null)} />

<div class="select-none relative">
  <div
    onclick={handleSelect}
    ontouchstart={handleTouchStart}
    ontouchend={handleTouchEnd}
    ontouchmove={handleTouchMove}
    class="flex items-center group px-2 py-1.5 rounded-lg text-sm sm:text-xs transition cursor-pointer relative {
      !item.is_dir && $activeNote?.path === item.path
        ? 'bg-[var(--sidebar-active-bg)] text-[var(--sidebar-active-text)] font-semibold'
        : 'text-[var(--text-primary)] hover:bg-[var(--sidebar-hover)]'
    }"
    style="padding-left: {depth * 14 + 8}px;"
    role="button"
    tabindex="0"
    onkeydown={(e) => {
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        handleSelect();
      }
    }}
  >
    <!-- Folder arrow toggle or spacer -->
    {#if item.is_dir}
      <button
        onclick={toggleOpen}
        class="mr-1 p-0.5 rounded hover:bg-black/10 transition shrink-0 cursor-pointer"
        aria-label="Toggle folder"
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
