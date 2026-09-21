<script lang="ts">
  import {
    Settings,
    X,
    Palette,
    Globe,
    Type,
    ChevronDown,
    Save,
    LayoutGrid
  } from 'lucide-svelte';
  import { t, currentLocale, setLocale, locales, type Locale } from '$lib/i18n';
  import { themeStore, setTheme, themes, type Theme } from '$lib/stores/theme';
  import { isSettingsOpen, closeSettings, editorFontSize, setEditorFontSize, fontSizes } from '$lib/stores/ui';
  import { appSettings, updateAppSettings } from '$lib/stores/settings';
  import { APP_VERSION } from '$lib/version';
  import Logo from '$lib/components/ui/Logo.svelte';

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && $isSettingsOpen) {
      closeSettings();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if $isSettingsOpen}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 z-50 bg-black/50 backdrop-blur-xs transition-opacity"
    onclick={closeSettings}
    role="presentation"
  ></div>

  <!-- Right Drawer -->
  <aside
    class="fixed inset-y-0 right-0 z-50 w-80 sm:w-96 border-l border-[var(--border-color)] bg-[var(--card-bg)] shadow-2xl flex flex-col animate-in slide-in-from-right duration-200 select-none overflow-hidden"
    role="dialog"
    aria-modal="true"
    aria-labelledby="settings-heading"
  >
    <!-- Header -->
    <div class="h-14 px-5 border-b border-[var(--border-color)] flex items-center justify-between shrink-0 bg-[var(--bg-primary)]">
      <div class="flex items-center gap-2.5">
        <Settings size={20} class="text-[var(--accent)]" />
        <h2 id="settings-heading" class="text-base font-semibold text-[var(--text-primary)]">
          {$t('settings.title')}
        </h2>
      </div>

      <button
        onclick={closeSettings}
        class="p-1.5 rounded-lg text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-tertiary)] transition cursor-pointer"
        aria-label="Close settings"
      >
        <X size={18} />
      </button>
    </div>

    <!-- Scrollable Content -->
    <div class="flex-1 overflow-y-auto p-5 space-y-6">
      <!-- Section: Themes (Selectbox) -->
      <div class="space-y-2">
        <label for="theme-select" class="flex items-center gap-2 text-xs font-bold uppercase tracking-wider text-[var(--text-muted)]">
          <Palette size={16} class="text-[var(--accent)]" />
          <span>{$t('settings.theme')}</span>
        </label>

        <div class="relative">
          <select
            id="theme-select"
            value={$themeStore}
            onchange={(e) => setTheme(e.currentTarget.value as Theme)}
            class="w-full px-3.5 py-2.5 rounded-xl border border-[var(--border-color)] bg-[var(--bg-secondary)] text-[var(--text-primary)] text-sm font-medium focus:outline-none focus:ring-2 focus:ring-[var(--accent)] transition cursor-pointer appearance-none"
          >
            {#each themes as th}
              <option value={th.id} class="bg-[var(--card-bg)] text-[var(--text-primary)]">
                {$t(th.labelKey)}
              </option>
            {/each}
          </select>
          <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-3.5 text-[var(--text-muted)]">
            <ChevronDown size={16} />
          </div>
        </div>
      </div>

      <!-- Section: Languages (Selectbox) -->
      <div class="space-y-2">
        <label for="lang-select" class="flex items-center gap-2 text-xs font-bold uppercase tracking-wider text-[var(--text-muted)]">
          <Globe size={16} class="text-[var(--accent)]" />
          <span>{$t('settings.language')}</span>
        </label>

        <div class="relative">
          <select
            id="lang-select"
            value={$currentLocale}
            onchange={(e) => setLocale(e.currentTarget.value as Locale)}
            class="w-full px-3.5 py-2.5 rounded-xl border border-[var(--border-color)] bg-[var(--bg-secondary)] text-[var(--text-primary)] text-sm font-medium focus:outline-none focus:ring-2 focus:ring-[var(--accent)] transition cursor-pointer appearance-none"
          >
            {#each locales as loc}
              <option value={loc.id} class="bg-[var(--card-bg)] text-[var(--text-primary)]">
                {loc.label}
              </option>
            {/each}
          </select>
          <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-3.5 text-[var(--text-muted)]">
            <ChevronDown size={16} />
          </div>
        </div>
      </div>

      <!-- Section: Code Font Size (Selectbox) -->
      <div class="space-y-2">
        <label for="font-size-select" class="flex items-center gap-2 text-xs font-bold uppercase tracking-wider text-[var(--text-muted)]">
          <Type size={16} class="text-[var(--accent)]" />
          <span>{$t('settings.fontSize')}</span>
        </label>

        <div class="relative">
          <select
            id="font-size-select"
            value={$editorFontSize}
            onchange={(e) => setEditorFontSize(e.currentTarget.value)}
            class="w-full px-3.5 py-2.5 rounded-xl border border-[var(--border-color)] bg-[var(--bg-secondary)] text-[var(--text-primary)] text-sm font-medium focus:outline-none focus:ring-2 focus:ring-[var(--accent)] transition cursor-pointer appearance-none"
          >
            {#each fontSizes as fs}
              <option value={fs.value} class="bg-[var(--card-bg)] text-[var(--text-primary)]">
                {fs.label}
              </option>
            {/each}
          </select>
          <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-3.5 text-[var(--text-muted)]">
            <ChevronDown size={16} />
          </div>
        </div>
      </div>

      <!-- Section: Auto Save & Draft Settings -->
      <div class="space-y-4 pt-4 border-t border-[var(--border-color)]">
        <div class="flex items-center justify-between">
          <label for="auto-save-toggle" class="flex flex-col gap-0.5 cursor-pointer">
            <span class="text-xs font-bold uppercase tracking-wider text-[var(--text-muted)] flex items-center gap-2">
              <Save size={16} class="text-[var(--accent)]" />
              <span>{$t('settings.autoSave')}</span>
            </span>
            <span class="text-[11px] text-[var(--text-muted)]">
              {$t('settings.autoSaveHelp')}
            </span>
          </label>
          <input
            id="auto-save-toggle"
            type="checkbox"
            checked={$appSettings.auto_save}
            onchange={(e) => updateAppSettings({ auto_save: e.currentTarget.checked })}
            class="w-5 h-5 rounded border-[var(--border-color)] text-[var(--accent)] focus:ring-[var(--accent)] cursor-pointer"
          />
        </div>

        {#if $appSettings.auto_save}
          <div class="space-y-2 pl-6">
            <label for="auto-save-delay-select" class="text-xs font-bold uppercase tracking-wider text-[var(--text-muted)] block">
              <span>{$t('settings.autoSaveDelay')}</span>
            </label>
            <div class="relative">
              <select
                id="auto-save-delay-select"
                value={$appSettings.auto_save_delay_ms}
                onchange={(e) => updateAppSettings({ auto_save_delay_ms: parseInt(e.currentTarget.value, 10) || 1500 })}
                class="w-full px-3.5 py-2 rounded-xl border border-[var(--border-color)] bg-[var(--bg-secondary)] text-[var(--text-primary)] text-xs font-medium focus:outline-none focus:ring-2 focus:ring-[var(--accent)] transition cursor-pointer appearance-none"
              >
                <option value={500} class="bg-[var(--card-bg)] text-[var(--text-primary)]">0.5s</option>
                <option value={1000} class="bg-[var(--card-bg)] text-[var(--text-primary)]">1.0s</option>
                <option value={1500} class="bg-[var(--card-bg)] text-[var(--text-primary)]">1.5s</option>
                <option value={2000} class="bg-[var(--card-bg)] text-[var(--text-primary)]">2.0s</option>
                <option value={3000} class="bg-[var(--card-bg)] text-[var(--text-primary)]">3.0s</option>
                <option value={5000} class="bg-[var(--card-bg)] text-[var(--text-primary)]">5.0s</option>
              </select>
              <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-3.5 text-[var(--text-muted)]">
                <ChevronDown size={14} />
              </div>
            </div>
          </div>
        {:else}
          <div class="flex items-center justify-between pt-2 border-t border-[var(--border-color)]/50">
            <label for="draft-recovery-toggle" class="flex flex-col gap-0.5 cursor-pointer">
              <span class="text-xs font-bold uppercase tracking-wider text-[var(--text-muted)] flex items-center gap-2">
                <span>{$t('settings.enableDraftRecovery')}</span>
              </span>
              <span class="text-[11px] text-[var(--text-muted)]">
                {$t('settings.enableDraftRecoveryHelp')}
              </span>
            </label>
            <input
              id="draft-recovery-toggle"
              type="checkbox"
              checked={$appSettings.enable_draft_recovery}
              onchange={(e) => updateAppSettings({ enable_draft_recovery: e.currentTarget.checked })}
              class="w-5 h-5 rounded border-[var(--border-color)] text-[var(--accent)] focus:ring-[var(--accent)] cursor-pointer"
            />
          </div>
        {/if}
      </div>

      <!-- Section: Recent Notes Settings -->
      <div class="space-y-4 pt-4 border-t border-[var(--border-color)]">
        <div class="flex items-center justify-between">
          <label for="show-recent-toggle" class="flex flex-col gap-0.5 cursor-pointer">
            <span class="text-xs font-bold uppercase tracking-wider text-[var(--text-muted)] flex items-center gap-2">
              <LayoutGrid size={16} class="text-[var(--accent)]" />
              <span>{$t('settings.showRecentNotes')}</span>
            </span>
          </label>
          <input
            id="show-recent-toggle"
            type="checkbox"
            checked={$appSettings.show_recent_notes}
            onchange={(e) => updateAppSettings({ show_recent_notes: e.currentTarget.checked })}
            class="w-5 h-5 rounded border-[var(--border-color)] text-[var(--accent)] focus:ring-[var(--accent)] cursor-pointer"
          />
        </div>

        {#if $appSettings.show_recent_notes}
          <div class="space-y-2">
            <label for="recent-count-select" class="text-xs font-bold uppercase tracking-wider text-[var(--text-muted)] block">
              <span>{$t('settings.recentNotesCount')}</span>
            </label>
            <div class="relative">
              <select
                id="recent-count-select"
                value={$appSettings.recent_notes_count}
                onchange={(e) => updateAppSettings({ recent_notes_count: parseInt(e.currentTarget.value, 10) || 5 })}
                class="w-full px-3.5 py-2.5 rounded-xl border border-[var(--border-color)] bg-[var(--bg-secondary)] text-[var(--text-primary)] text-sm font-medium focus:outline-none focus:ring-2 focus:ring-[var(--accent)] transition cursor-pointer appearance-none"
              >
                {#each [3, 5, 8, 10, 15, 20] as num}
                  <option value={num} class="bg-[var(--card-bg)] text-[var(--text-primary)]">
                    {num}
                  </option>
                {/each}
              </select>
              <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-3.5 text-[var(--text-muted)]">
                <ChevronDown size={16} />
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Section: About -->
      <div class="pt-2 border-t border-[var(--border-color)]">
        <div class="flex items-center gap-3 p-3.5 rounded-xl bg-[var(--bg-secondary)] border border-[var(--border-color)]">
          <Logo size={48} />
          <div class="flex-1">
            <h4 class="text-sm font-bold text-[var(--text-primary)]">MarkNote</h4>
            <p class="text-xs text-[var(--text-muted)] mt-0.5">
              {$t('settings.version')} {APP_VERSION}
            </p>
            <p class="text-xs mt-0.5">
              <a href="https://github.com/odevsa/marknote" target="_blank" rel="noopener noreferrer">https://github.com/odevsa/marknote</a>
            </p>
          </div>
        </div>
      </div>
    </div>
  </aside>
{/if}
