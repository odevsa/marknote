<script lang="ts">
  import { Check, Loader2, Save, FileText } from 'lucide-svelte';
  import { activeNote, isDirty, isSaving, saveCurrentNote, fileTree } from '$lib/stores/notes';
  import { t } from '$lib/i18n';
  import Logo from '$lib/components/ui/Logo.svelte';

  let wordCount = $derived.by(() => {
    if (!$activeNote?.content) return 0;
    const clean = $activeNote.content.trim();
    if (!clean) return 0;
    return clean.split(/\s+/).length;
  });

  let charCount = $derived.by(() => {
    return $activeNote?.content?.length || 0;
  });
</script>

<footer class="h-8 border-t border-[var(--statusbar-border)] bg-[var(--statusbar-bg)] px-3 sm:px-4 flex items-center justify-between text-xs sm:text-[11px] text-[var(--statusbar-text)] select-none shrink-0 z-30 transition-colors">
  <!-- Left: File Path or App Title -->
  <div class="flex items-center gap-2 truncate max-w-xs sm:max-w-sm">
    {#if $activeNote}
      <span class="font-mono text-xs sm:text-[11px] truncate text-[var(--statusbar-text)] opacity-95">
        {$activeNote.path}
      </span>
    {:else}
      <div class="flex items-center gap-1.5 text-[var(--statusbar-text)] font-medium text-xs sm:text-[11px]">
        <Logo size={14} class="text-[var(--statusbar-text)]" />
        <span>MarkNote</span>
      </div>
    {/if}
  </div>

  <!-- Right: Words, Chars, Save status or General status -->
  <div class="flex items-center gap-3 sm:gap-4">
    {#if $activeNote}
      <div class="hidden sm:flex items-center gap-2 text-[var(--statusbar-text-muted)] text-[11px]">
        <span>{wordCount} {$t('editor.words')}</span>
        <span>•</span>
        <span>{charCount} {$t('editor.chars')}</span>
      </div>

      <div class="flex items-center gap-1.5 text-xs font-medium">
        {#if $isSaving}
          <div class="flex items-center gap-1 text-[var(--statusbar-text)]">
            <Loader2 size={13} class="animate-spin" />
            <span>{$t('common.saving')}</span>
          </div>
        {:else if $isDirty}
          <button
            onclick={saveCurrentNote}
            class="flex items-center gap-1 text-amber-300 hover:text-amber-200 font-medium transition cursor-pointer"
            title="Ctrl+S"
            aria-label={$t('common.save')}
          >
            <span class="w-2 h-2 rounded-full bg-amber-400"></span>
            <span>{$t('common.unsaved')}</span>
            <Save size={13} class="ml-0.5" />
          </button>
        {:else}
          <div class="flex items-center gap-1 text-emerald-300 font-medium">
            <Check size={14} />
            <span>{$t('common.saved')}</span>
          </div>
        {/if}
      </div>
    {:else}
      <div class="flex items-center gap-1.5 text-[var(--statusbar-text-muted)] text-xs sm:text-[11px]">
        <FileText size={13} />
        <span>{$fileTree.length} {$t('common.allNotes').toLowerCase()}</span>
      </div>
    {/if}
  </div>
</footer>
