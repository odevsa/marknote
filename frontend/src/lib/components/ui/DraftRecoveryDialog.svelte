<script lang="ts">
  import Modal from './Modal.svelte';
  import { t } from '$lib/i18n';
  import {
    showDraftRecoveryModal,
    restoreLocalDraft,
    discardLocalDraft
  } from '$lib/stores/notes';
</script>

<Modal
  isOpen={$showDraftRecoveryModal}
  title={$t('settings.draftFoundTitle')}
  onClose={discardLocalDraft}
>
  <div class="space-y-4">
    <p class="text-sm text-[var(--text-muted)] leading-relaxed">
      {$t('settings.draftFoundMessage')}
    </p>

    <div class="flex justify-end gap-2 pt-2">
      <button
        type="button"
        onclick={discardLocalDraft}
        class="px-3.5 py-2 text-xs font-semibold rounded-lg border border-[var(--border-color)] text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-tertiary)] transition cursor-pointer"
      >
        {$t('settings.discardDraft')}
      </button>
      <button
        type="button"
        onclick={restoreLocalDraft}
        class="px-4 py-2 text-xs font-semibold rounded-lg bg-[var(--accent)] hover:bg-[var(--accent-hover)] text-[var(--accent-text)] transition cursor-pointer shadow-xs"
      >
        {$t('settings.restoreDraft')}
      </button>
    </div>
  </div>
</Modal>
