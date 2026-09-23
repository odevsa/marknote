<script lang="ts">
  import { FilePlus, FolderPlus, X } from 'lucide-svelte';
  import FileTree from '../filetree/FileTree.svelte';
  import { sidebarOpen } from '$lib/stores/notes';
  import { t } from '$lib/i18n';

  let fileTreeRef: FileTree | undefined = $state();

  function closeSidebar() {
    sidebarOpen.set(false);
  }

  function handleCreateFile() {
    fileTreeRef?.openCreateRootFile();
  }

  function handleCreateFolder() {
    fileTreeRef?.openCreateRootFolder();
  }
</script>

<!-- Mobile backdrop -->
{#if $sidebarOpen}
  <div
    class="fixed inset-0 bg-black/50 z-40 md:hidden backdrop-blur-xs"
    onclick={closeSidebar}
    role="presentation"
  ></div>
{/if}

<aside
  class="fixed md:static inset-y-0 left-0 z-40 w-64 md:w-64 border-r border-[var(--border-color)] bg-[var(--sidebar-bg)] flex flex-col transition-transform duration-200 ease-in-out shrink-0 {
    $sidebarOpen ? 'translate-x-0' : '-translate-x-full md:translate-x-0'
  }"
>
  <!-- Sidebar Header Actions -->
  <div class="h-11 border-b border-[var(--border-color)] px-4 flex items-center justify-between shrink-0">
    <span class="text-xs font-bold uppercase tracking-wider text-[var(--text-muted)]">
      {$t('common.allNotes')}
    </span>

    <div class="flex items-center gap-1">
      <button
        onclick={handleCreateFile}
        class="p-1.5 rounded-md text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--sidebar-hover)] transition cursor-pointer"
        title={$t('common.newNote')}
        aria-label={$t('common.newNote')}
      >
        <FilePlus size={20} class="sm:size-[16px]" />
      </button>

      <button
        onclick={handleCreateFolder}
        class="p-1.5 rounded-md text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--sidebar-hover)] transition cursor-pointer"
        title={$t('common.newFolder')}
        aria-label={$t('common.newFolder')}
      >
        <FolderPlus size={20} class="sm:size-[16px]" />
      </button>

      <!-- Mobile close button -->
      <button
        onclick={closeSidebar}
        class="p-1.5 rounded-md text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--sidebar-hover)] md:hidden transition ml-1 cursor-pointer"
        aria-label="Close sidebar"
      >
        <X size={20} class="sm:size-[16px]" />
      </button>
    </div>
  </div>

  <!-- Scrollable File Tree -->
  <div class="flex-1 overflow-y-auto px-2">
    <FileTree bind:this={fileTreeRef} />
  </div>
</aside>
