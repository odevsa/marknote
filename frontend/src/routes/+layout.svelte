<script lang="ts">
  import '../styles/app.css';
  import { onMount } from 'svelte';
  import { page } from '$app/state';
  import { goto } from '$app/navigation';
  import { authStore, checkAuth } from '$lib/stores/auth';
  import { initThemeListener } from '$lib/stores/theme';
  import {
    isSearchOpen,
    showUnsavedConfirmModal,
    confirmDiscardAndNavigate,
    pendingTargetRoute
  } from '$lib/stores/notes';
  import SearchModal from '$lib/components/SearchModal.svelte';
  import SettingsDrawer from '$lib/components/layout/SettingsDrawer.svelte';
  import ConfirmDialog from '$lib/components/ui/ConfirmDialog.svelte';
  import DraftRecoveryDialog from '$lib/components/ui/DraftRecoveryDialog.svelte';
  import { initEditorFontSize } from '$lib/stores/ui';
  import { t } from '$lib/i18n';

  let { children } = $props();

  onMount(() => {
    initThemeListener();
    initEditorFontSize();
    checkAuth();
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

  function handleKeydown(e: KeyboardEvent) {
    // Global Ctrl+K / Cmd+K search shortcut
    if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
      e.preventDefault();
      isSearchOpen.set(true);
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="h-screen w-screen flex flex-col overflow-hidden bg-[var(--bg-primary)] text-[var(--text-primary)]">
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

