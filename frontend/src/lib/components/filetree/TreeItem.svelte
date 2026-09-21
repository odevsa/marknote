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

  let isOpen = $state(true);
  let showMenu = $state(false);

  function toggleOpen(e: Event) {
    e.stopPropagation();
    isOpen = !isOpen;
  }

  function handleSelect() {
    if (!item.is_dir) {
      openNote(item.path);
    } else {
      isOpen = !isOpen;
    }
  }

  function toggleMenu(e: Event) {
    e.stopPropagation();
    showMenu = !showMenu;
  }

  function closeMenu() {
    showMenu = false;
  }
</script>

<svelte:window onclick={closeMenu} />

<div class="select-none">
  <div
    onclick={handleSelect}
    class="flex items-center group px-2 py-1.5 rounded-lg text-sm transition cursor-pointer relative {
      !item.is_dir && $activeNote?.path === item.path
        ? 'bg-[var(--sidebar-active-bg)] text-[var(--sidebar-active-text)] font-semibold'
        : 'text-[var(--text-primary)] hover:bg-[var(--sidebar-hover)]'
    }"
    style="padding-left: {depth * 14 + 8}px;"
    role="button"
    tabindex="0"
    onkeydown={(e) => e.key === 'Enter' && handleSelect()}
  >
    <!-- Folder arrow toggle or spacer -->
    {#if item.is_dir}
      <button
        onclick={toggleOpen}
        class="mr-1 p-0.5 rounded hover:bg-black/10 transition shrink-0"
        aria-label="Toggle folder"
      >
        {#if isOpen}
          <ChevronDown size={14} class="opacity-70" />
        {:else}
          <ChevronRight size={14} class="opacity-70" />
        {/if}
      </button>
      {#if isOpen}
        <FolderOpen size={16} class="mr-2 text-[var(--accent)] shrink-0" />
      {:else}
        <Folder size={16} class="mr-2 text-[var(--accent)] shrink-0" />
      {/if}
    {:else}
      <span class="w-4 mr-1 shrink-0"></span>
      <FileText
        size={15}
        class="mr-2 shrink-0 {!item.is_dir && $activeNote?.path === item.path ? 'text-[var(--sidebar-active-text)]' : 'text-[var(--text-muted)]'}"
      />
    {/if}

    <!-- Name -->
    <span class="truncate flex-1 text-xs sm:text-sm">{item.name}</span>

    <!-- Unsaved indicator dot -->
    {#if !item.is_dir && (($activeNote?.path === item.path && $isDirty) || ($activeNote?.path !== item.path && hasUnsavedDraft(item.path)))}
      <span class="w-2 h-2 rounded-full bg-[var(--accent)] shrink-0 ml-1.5 shadow-xs" title="Unsaved changes"></span>
    {/if}

    <!-- Item Actions Trigger -->
    <div class="opacity-0 group-hover:opacity-100 transition ml-1 shrink-0">
      <button
        onclick={toggleMenu}
        class="p-1 rounded hover:bg-black/10 transition text-inherit"
        title="Options"
      >
        <MoreVertical size={14} />
      </button>
    </div>

    <!-- Dropdown Menu -->
    {#if showMenu}
      <div
        class="absolute right-2 top-8 z-50 w-44 py-1 rounded-lg border border-[var(--border-color)] bg-[var(--card-bg)] shadow-xl text-[var(--text-primary)] text-xs"
        onclick={(e) => e.stopPropagation()}
        role="menu"
      >
        {#if item.is_dir}
          <button
            onclick={() => { closeMenu(); onCreateFile(item.path); }}
            class="w-full flex items-center gap-2 px-3 py-1.5 hover:bg-[var(--bg-tertiary)] text-left"
          >
            <FilePlus size={14} class="text-[var(--accent)]" />
            {$t('common.newNote')}
          </button>
          <button
            onclick={() => { closeMenu(); onCreateFolder(item.path); }}
            class="w-full flex items-center gap-2 px-3 py-1.5 hover:bg-[var(--bg-tertiary)] text-left"
          >
            <FolderPlus size={14} class="text-[var(--accent)]" />
            {$t('common.newFolder')}
          </button>
          <div class="my-1 border-t border-[var(--border-color)]"></div>
        {/if}
        <button
          onclick={() => { closeMenu(); onRename(item.path, item.name); }}
          class="w-full flex items-center gap-2 px-3 py-1.5 hover:bg-[var(--bg-tertiary)] text-left"
        >
          <Edit2 size={14} />
          {$t('common.rename')}
        </button>
        <button
          onclick={() => { closeMenu(); onDelete(item.path, item.is_dir); }}
          class="w-full flex items-center gap-2 px-3 py-1.5 hover:bg-[var(--bg-tertiary)] text-[var(--danger)] text-left"
        >
          <Trash2 size={14} />
          {$t('common.delete')}
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
