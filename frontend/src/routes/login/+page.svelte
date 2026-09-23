<script lang="ts">
  import { FileText, ArrowRight, Loader2 } from 'lucide-svelte';
  import Logo from '$lib/components/ui/Logo.svelte';
  import { authStore, loginUser } from '$lib/stores/auth';
  import { t } from '$lib/i18n';

  let username = $state('');
  let password = $state('');

  async function handleSubmit(e: Event) {
    e.preventDefault();
    await loginUser({
      username: username.trim(),
      password
    });
  }
</script>

<div class="flex-1 flex items-center justify-center p-4 bg-[var(--bg-secondary)]">
  <div class="w-full max-w-md p-6 sm:p-8 rounded-2xl border border-[var(--border-color)] bg-[var(--card-bg)] shadow-xl animate-in fade-in zoom-in-95 duration-200">
    <!-- Header -->
    <div class="flex flex-col items-center text-center mb-6">
      <Logo size={48} class="mb-3" />
      <h1 class="text-xl sm:text-2xl font-bold text-[var(--text-primary)]">
        {$t('common.appTitle')}
      </h1>
      <p class="text-xs sm:text-sm text-[var(--text-muted)] mt-1">
        {$t('auth.loginSubtitle')}
      </p>
    </div>

    <!-- Error message -->
    {#if $authStore.error}
      <div class="mb-4 p-3 rounded-lg bg-red-500/10 border border-red-500/20 text-xs text-red-500 font-medium">
        {$authStore.error}
      </div>
    {/if}

    <!-- Login Form -->
    <form onsubmit={handleSubmit} class="space-y-4">
      <div>
        <label for="username" class="block text-xs font-semibold text-[var(--text-muted)] uppercase mb-1">
          {$t('auth.username')}
        </label>
        <input
          id="username"
          type="text"
          bind:value={username}
          required
          autofocus
          class="w-full px-3.5 py-2.5 rounded-lg border border-[var(--border-color)] bg-[var(--bg-primary)] text-[var(--text-primary)] text-sm focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
        />
      </div>

      <div>
        <label for="password" class="block text-xs font-semibold text-[var(--text-muted)] uppercase mb-1">
          {$t('auth.password')}
        </label>
        <input
          id="password"
          type="password"
          bind:value={password}
          required
          class="w-full px-3.5 py-2.5 rounded-lg border border-[var(--border-color)] bg-[var(--bg-primary)] text-[var(--text-primary)] text-sm focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
        />
      </div>

      <button
        type="submit"
        disabled={$authStore.loading}
        class="w-full mt-2 py-2.5 px-4 rounded-lg bg-[var(--accent)] hover:bg-[var(--accent-hover)] text-[var(--accent-text)] text-sm font-semibold flex items-center justify-center gap-2 transition disabled:opacity-50 cursor-pointer"
      >
        {#if $authStore.loading}
          <Loader2 size={16} class="animate-spin" />
          <span>{$t('common.loading')}</span>
        {:else}
          <span>{$t('auth.submitLogin')}</span>
          <ArrowRight size={16} />
        {/if}
      </button>
    </form>
  </div>
</div>

