<script lang="ts">
  import TreeItem from './TreeItem.svelte';
  import PromptModal from '../ui/PromptModal.svelte';
  import { fileTree, createItem, promptRename, promptDelete } from '$lib/stores/notes';
  import { t } from '$lib/i18n';

  let isCreateFileOpen = $state(false);
  let isCreateFolderOpen = $state(false);

  let targetParentPath = $state('');

  export function openCreateRootFile() {
    targetParentPath = '';
    isCreateFileOpen = true;
  }

  export function openCreateRootFolder() {
    targetParentPath = '';
    isCreateFolderOpen = true;
  }

  function handleCreateFile(parentPath: string) {
    targetParentPath = parentPath;
    isCreateFileOpen = true;
  }

  function handleCreateFolder(parentPath: string) {
    targetParentPath = parentPath;
    isCreateFolderOpen = true;
  }

  function handleRename(path: string, currentName: string) {
    promptRename(path, currentName);
  }

  function handleDelete(path: string, isDir: boolean) {
    promptDelete(path, isDir);
  }

  async function confirmCreateFile(name: string) {
    let fileName = name.trim();
    if (!fileName.endsWith('.md') && !fileName.endsWith('.txt')) {
      fileName += '.md';
    }
    const fullPath = targetParentPath ? `${targetParentPath}/${fileName}` : fileName;
    await createItem(fullPath, false);
  }

  async function confirmCreateFolder(name: string) {
    const folderName = name.trim();
    const fullPath = targetParentPath ? `${targetParentPath}/${folderName}` : folderName;
    await createItem(fullPath, true);
  }
</script>

<div class="file-tree-container py-2 space-y-0.5" role="tree" tabindex="-1">
  {#if $fileTree.length === 0}
    <div class="px-3 py-6 text-center text-xs text-[var(--text-muted)]">
      {$t('common.selectNoteToView')}
    </div>
  {:else}
    {#each $fileTree as item (item.path)}
      <TreeItem
        {item}
        onCreateFile={handleCreateFile}
        onCreateFolder={handleCreateFolder}
        onRename={handleRename}
        onDelete={handleDelete}
      />
    {/each}
  {/if}
</div>

<!-- Modals -->
<PromptModal
  isOpen={isCreateFileOpen}
  title={$t('common.newNote')}
  label={$t('common.namePlaceholder')}
  placeholder="my-note.md"
  confirmText={$t('common.create')}
  onConfirm={confirmCreateFile}
  onClose={() => (isCreateFileOpen = false)}
/>

<PromptModal
  isOpen={isCreateFolderOpen}
  title={$t('common.newFolder')}
  label={$t('common.namePlaceholder')}
  placeholder="Folder Name"
  confirmText={$t('common.create')}
  onConfirm={confirmCreateFolder}
  onClose={() => (isCreateFolderOpen = false)}
/>
