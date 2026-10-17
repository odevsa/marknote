<script lang="ts">
  import '../styles/app.css';
  import { onMount } from 'svelte';
  import { page } from '$app/state';
  import { goto } from '$app/navigation';
  import { authStore, checkAuth } from '$lib/stores/auth';
  import { initThemeListener } from '$lib/stores/theme';
  import {
    activeNote,
    isSearchOpen,
    showUnsavedConfirmModal,
    confirmDiscardAndNavigate,
    pendingTargetRoute,
    renameModalState,
    deleteModalState,
    promptRename,
    promptDelete,
    closeRenameModal,
    closeDeleteModal,
    renameItem,
    deleteItem
  } from '$lib/stores/notes';
  import SearchModal from '$lib/components/SearchModal.svelte';
  import SettingsDrawer from '$lib/components/layout/SettingsDrawer.svelte';
  import ConfirmDialog from '$lib/components/ui/ConfirmDialog.svelte';
  import PromptModal from '$lib/components/ui/PromptModal.svelte';
  import DraftRecoveryDialog from '$lib/components/ui/DraftRecoveryDialog.svelte';
  import ShortcutsModal from '$lib/components/ui/ShortcutsModal.svelte';
  import {
    initEditorFontSize,
    initEditorLineWrapping,
    openSettings,
    openShortcuts,
    isSettingsOpen,
    isShortcutsOpen
  } from '$lib/stores/ui';
  import { t } from '$lib/i18n';

  let { children } = $props();

  onMount(() => {
    initThemeListener();
    initEditorFontSize();
    initEditorLineWrapping();
    checkAuth();

    // Prevent virtual keyboard from scrolling window and displacing header
    if (typeof window !== 'undefined') {
      const updateViewport = () => {
        if (window.visualViewport) {
          const vh = window.visualViewport.height;
          document.documentElement.style.setProperty(
            '--visual-viewport-height',
            `${vh}px`
          );
        }
        if (window.scrollY !== 0 || window.scrollX !== 0) {
          window.scrollTo(0, 0);
        }
      };

      if (window.visualViewport) {
        window.visualViewport.addEventListener('resize', updateViewport);
        window.visualViewport.addEventListener('scroll', updateViewport);
      }
      window.addEventListener('scroll', updateViewport);

      const handleFocusIn = () => {
        requestAnimationFrame(updateViewport);
        setTimeout(updateViewport, 50);
        setTimeout(updateViewport, 250);
      };
      document.addEventListener('focusin', handleFocusIn);

      updateViewport();

      return () => {
        if (window.visualViewport) {
          window.visualViewport.removeEventListener('resize', updateViewport);
          window.visualViewport.removeEventListener('scroll', updateViewport);
        }
        window.removeEventListener('scroll', updateViewport);
        document.removeEventListener('focusin', handleFocusIn);
      };
    }
  });

  // Handle route guards and setup redirects
  $effect(() => {
    const currentPath = page.url.pathname;
    const { initialized, user, loading } = $authStore;

    if (loading) return;

    if (initialized === false) {
      if (currentPath !== '/setup') {
        goto('/setup');
      }
    } else if (initialized === true) {
      if (!user) {
        if (currentPath !== '/login') {
          const redirect = encodeURIComponent(currentPath + page.url.search);
          goto(`/login?redirect=${redirect}`);
        }
      } else {
        if (currentPath === '/login' || currentPath === '/setup') {
          const rawTarget = page.url.searchParams.get('redirect');
          const target = rawTarget && rawTarget.startsWith('/') && !rawTarget.startsWith('//')
            ? rawTarget
            : '/';
          goto(target);
        }
      }
    }
  });

  function isEditableElement(target: EventTarget | null): boolean {
    if (!target || !(target instanceof HTMLElement)) return false;
    const tag = target.tagName.toLowerCase();
    if (tag === 'input' || tag === 'textarea' || tag === 'select') return true;
    if (target.isContentEditable) return true;
    if (target.closest('.cm-editor') || target.closest('[contenteditable="true"]')) return true;
    return false;
  }

  function handleKeydown(e: KeyboardEvent) {
    // Global Ctrl+K / Cmd+K search shortcut (must NOT trigger on Ctrl+Shift+K or Alt)
    if ((e.ctrlKey || e.metaKey) && !e.shiftKey && !e.altKey && e.key.toLowerCase() === 'k') {
      e.preventDefault();
      isSearchOpen.set(true);
      return;
    }

    // F1 or Ctrl+/ / Cmd+/ opens Keyboard Shortcuts modal
    if (e.key === 'F1' || ((e.ctrlKey || e.metaKey) && !e.altKey && e.key === '/')) {
      e.preventDefault();
      openShortcuts();
      return;
    }

    // Ctrl+, / Cmd+, opens Settings
    if ((e.ctrlKey || e.metaKey) && !e.shiftKey && !e.altKey && e.key === ',') {
      e.preventDefault();
      openSettings();
      return;
    }

    // F2: rename active note
    if (e.key === 'F2') {
      const active = $activeNote;
      if (
        active &&
        !$renameModalState.isOpen &&
        !$deleteModalState.isOpen &&
        !$isSearchOpen &&
        !$isSettingsOpen &&
        !$isShortcutsOpen &&
        !$showUnsavedConfirmModal
      ) {
        e.preventDefault();
        promptRename(active.path);
        return;
      }
    }

    // Delete: delete active note (only when NOT typing in an editable field)
    if (e.key === 'Delete' && !isEditableElement(e.target)) {
      const active = $activeNote;
      if (
        active &&
        !$renameModalState.isOpen &&
        !$deleteModalState.isOpen &&
        !$isSearchOpen &&
        !$isSettingsOpen &&
        !$isShortcutsOpen &&
        !$showUnsavedConfirmModal
      ) {
        e.preventDefault();
        promptDelete(active.path, false);
        return;
      }
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div
  class="fixed inset-0 flex flex-col overflow-hidden bg-[var(--bg-primary)] text-[var(--text-primary)]"
  style="height: var(--visual-viewport-height, 100%); max-height: var(--visual-viewport-height, 100%);"
>
  {#if $authStore.loading}
    <div class="flex-1 flex items-center justify-center">
      <div class="w-8 h-8 border-3 border-[var(--accent)] border-t-transparent rounded-full animate-spin"></div>
    </div>
  {:else}
    {@render children?.()}
  {/if}
</div>

<SearchModal />
<SettingsDrawer />
<ShortcutsModal />
<DraftRecoveryDialog />

<ConfirmDialog
  isOpen={$showUnsavedConfirmModal}
  title={$t('settings.unsavedChangesTitle')}
  message={$t('settings.unsavedChangesMessage')}
  confirmText={$t('settings.discard')}
  danger={true}
  onConfirm={confirmDiscardAndNavigate}
  onClose={() => {
    showUnsavedConfirmModal.set(false);
    pendingTargetRoute.set(null);
  }}
/>

<PromptModal
  isOpen={$renameModalState.isOpen}
  title={$t('common.rename')}
  initialValue={$renameModalState.currentName}
  placeholder="New name..."
  confirmText={$t('common.save')}
  onConfirm={async (newName) => {
    const targetPath = $renameModalState.path;
    const parent = targetPath.includes('/')
      ? targetPath.substring(0, targetPath.lastIndexOf('/'))
      : '';
    const newPath = parent ? `${parent}/${newName}` : newName;
    await renameItem(targetPath, newPath);
  }}
  onClose={closeRenameModal}
/>

<ConfirmDialog
  isOpen={$deleteModalState.isOpen}
  title={$t('common.delete')}
  message={`${$t('common.confirmDelete')} "${$deleteModalState.path}"?`}
  danger={true}
  confirmText={$t('common.delete')}
  onConfirm={async () => {
    await deleteItem($deleteModalState.path);
  }}
  onClose={closeDeleteModal}
/>

