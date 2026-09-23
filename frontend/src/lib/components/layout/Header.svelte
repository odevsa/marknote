<script lang="ts">
  import {
    Menu,
    Search,
    Settings,
    LogOut,
    User
  } from 'lucide-svelte';
  import { t } from '$lib/i18n';
  import { authStore, logoutUser } from '$lib/stores/auth';
  import { isSearchOpen, sidebarOpen, closeNote } from '$lib/stores/notes';
  import { openSettings } from '$lib/stores/ui';
  import Logo from '$lib/components/ui/Logo.svelte';

  function toggleSidebar() {
    sidebarOpen.update((v) => !v);
  }

  function openSearch() {
    isSearchOpen.set(true);
  }
</script>

<header class="h-14 border-b border-[var(--border-color)] bg-[var(--bg-primary)] px-3 sm:px-4 flex items-center justify-between select-none z-30 shrink-0">
  <!-- Left section -->
  <div class="flex items-center gap-2 sm:gap-3">
    <button
      onclick={toggleSidebar}
      class="p-1.5 rounded-lg text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-tertiary)] md:hidden transition cursor-pointer"
      aria-label="Toggle navigation menu"
    >
      <Menu size={22} class="sm:size-[20px]" />
    </button>

    <div
      onclick={() => closeNote()}
      class="flex items-center gap-2 font-bold tracking-tight text-[17px] sm:text-lg text-[var(--text-primary)] cursor-pointer hover:opacity-80 transition"
      role="button"
      tabindex="0"
      onkeydown={(e) => e.key === 'Enter' && closeNote()}
    >
      <Logo size={26} />
      <span class="hidden sm:inline">MarkNote</span>
    </div>
  </div>

  <!-- Center Search button -->
  <div class="flex-1 max-w-md mx-2 sm:mx-4">
    <button
      onclick={openSearch}
      class="w-full flex items-center justify-between px-3 py-1.5 rounded-lg border border-[var(--border-color)] bg-[var(--bg-secondary)] hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] text-sm sm:text-xs transition group cursor-pointer"
    >
      <div class="flex items-center gap-2 truncate">
        <Search size={17} class="group-hover:text-[var(--text-primary)] transition sm:size-[15px]" />
        <span class="truncate">{$t('common.search')}</span>
      </div>
      <kbd class="hidden sm:inline-block px-1.5 py-0.5 text-[10px] font-mono rounded border border-[var(--border-color)] bg-[var(--bg-primary)] text-[var(--text-muted)]">
        Ctrl+K
      </kbd>
    </button>
  </div>

  <!-- Right Actions: Settings, Profile -->
  <div class="flex items-center gap-1 sm:gap-2">
    <!-- Settings Drawer Button -->
    <button
      onclick={openSettings}
      class="p-1.5 rounded-lg text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-tertiary)] transition cursor-pointer"
      title={$t('settings.title')}
      aria-label={$t('settings.title')}
    >
      <Settings size={20} class="sm:size-[18px]" />
    </button>

    <!-- User / Logout -->
    {#if $authStore.user}
      <div class="flex items-center gap-1 pl-1.5 sm:pl-2 border-l border-[var(--border-color)]">
        <div class="hidden lg:flex items-center gap-1.5 text-xs font-medium text-[var(--text-primary)]">
          <User size={15} class="text-[var(--text-muted)]" />
          <span class="max-w-[100px] truncate">{$authStore.user.username}</span>
        </div>
        <button
          onclick={logoutUser}
          class="p-1.5 rounded-lg text-[var(--text-muted)] hover:text-[var(--danger)] hover:bg-[var(--bg-tertiary)] transition cursor-pointer"
          title={$t('auth.logout')}
          aria-label={$t('auth.logout')}
        >
          <LogOut size={20} class="sm:size-[18px]" />
        </button>
      </div>
    {/if}
  </div>
</header>
