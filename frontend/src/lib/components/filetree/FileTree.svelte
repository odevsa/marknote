<script lang="ts">
  import TreeItem from './TreeItem.svelte';
  import PromptModal from '../ui/PromptModal.svelte';
  import ConfirmDialog from '../ui/ConfirmDialog.svelte';
  import { fileTree, createItem, renameItem, deleteItem } from '$lib/stores/notes';
  import { t } from '$lib/i18n';

  let isCreateFileOpen = $state(false);
  let isCreateFolderOpen = $state(false);
  let isRenameOpen = $state(false);
  let isDeleteOpen = $state(false);

  let targetParentPath = $state('');
  let targetItemPath = $state('');
  let currentItemName = $state('');
  let itemToDelete = $state({ path: '', isDir: false });

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
    targetItemPath = path;
    currentItemName = currentName;
    isRenameOpen = true;
  }

  function handleDelete(path: string, isDir: boolean) {
    itemToDelete = { path, isDir };
    isDeleteOpen = true;
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

  async function confirmRename(newName: string) {
    const parent = targetItemPath.includes('/')
      ? targetItemPath.substring(0, targetItemPath.lastIndexOf('/'))
      : '';
    const newPath = parent ? `${parent}/${newName}` : newName;
    await renameItem(targetItemPath, newPath);
  }

  async function confirmDelete() {
    await deleteItem(itemToDelete.path);
  }
</script>

<div class="py-2 space-y-0.5">
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

<PromptModal
  isOpen={isRenameOpen}
  title={$t('common.rename')}
  initialValue={currentItemName}
  placeholder="New name..."
  confirmText={$t('common.save')}
  onConfirm={confirmRename}
  onClose={() => (isRenameOpen = false)}
/>

<ConfirmDialog
  isOpen={isDeleteOpen}
  title={$t('common.delete')}
  message={`${$t('common.confirmDelete')} "${itemToDelete.path}"?`}
  danger={true}
  confirmText={$t('common.delete')}
  onConfirm={confirmDelete}
  onClose={() => (isDeleteOpen = false)}
/>
