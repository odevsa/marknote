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
  import { initEditorFontSize, initEditorLineWrapping } from '$lib/stores/ui';
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

  function handleKeydown(e: KeyboardEvent) {
    // Global Ctrl+K / Cmd+K search shortcut
    if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
      e.preventDefault();
      isSearchOpen.set(true);
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

