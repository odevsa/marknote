<script lang="ts">
  import { FileText, ArrowRight, Loader2 } from 'lucide-svelte';
  import Logo from '$lib/components/ui/Logo.svelte';
  import { authStore, setupUser } from '$lib/stores/auth';
  import { t } from '$lib/i18n';

  let username = $state('');
  let password = $state('');
  let confirmPassword = $state('');
  let validationError = $state('');

  async function handleSubmit(e: Event) {
    e.preventDefault();
    validationError = '';

    if (!username.trim()) {
      validationError = 'Username is required';
      return;
    }

    if (password.length < 6) {
      validationError = $t('auth.minPasswordLength');
      return;
    }

    if (password !== confirmPassword) {
      validationError = $t('auth.passwordMismatch');
      return;
    }

    await setupUser({
      username: username.trim(),
      password,
      confirm_password: confirmPassword
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
        {$t('auth.setupSubtitle')}
      </p>
    </div>

    <!-- Error message -->
    {#if validationError || $authStore.error}
      <div class="mb-4 p-3 rounded-lg bg-red-500/10 border border-red-500/20 text-xs text-red-500 font-medium">
        {validationError || $authStore.error}
      </div>
    {/if}

    <!-- Setup Form -->
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

      <div>
        <label for="confirm_password" class="block text-xs font-semibold text-[var(--text-muted)] uppercase mb-1">
          {$t('auth.confirmPassword')}
        </label>
        <input
          id="confirm_password"
          type="password"
          bind:value={confirmPassword}
          required
          class="w-full px-3.5 py-2.5 rounded-lg border border-[var(--border-color)] bg-[var(--bg-primary)] text-[var(--text-primary)] text-sm focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
        />
      </div>

      <button
        type="submit"
        disabled={$authStore.loading}
        class="w-full mt-2 py-2.5 px-4 rounded-lg bg-[var(--accent)] hover:bg-[var(--accent-hover)] text-white text-sm font-semibold flex items-center justify-center gap-2 transition disabled:opacity-50 cursor-pointer"
      >
        {#if $authStore.loading}
          <Loader2 size={16} class="animate-spin" />
          <span>{$t('common.loading')}</span>
        {:else}
          <span>{$t('auth.submitSetup')}</span>
          <ArrowRight size={16} />
        {/if}
      </button>
    </form>
  </div>
</div>

