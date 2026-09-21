<script lang="ts">
  import Modal from './Modal.svelte';
  import { t } from '$lib/i18n';

  interface Props {
    isOpen: boolean;
    title: string;
    message: string;
    confirmText?: string;
    danger?: boolean;
    onConfirm: () => void;
    onClose: () => void;
  }

  let {
    isOpen,
    title,
    message,
    confirmText,
    danger = false,
    onConfirm,
    onClose
  }: Props = $props();

  function handleConfirm() {
    onConfirm();
    onClose();
  }
</script>

<Modal {isOpen} {title} {onClose}>
  <div class="space-y-4">
    <p class="text-sm text-[var(--text-muted)] leading-relaxed">
      {message}
    </p>
    <div class="flex justify-end gap-2 pt-2">
      <button
        type="button"
        onclick={onClose}
        class="px-4 py-2 text-sm rounded-lg border border-[var(--border-color)] text-[var(--text-primary)] hover:bg-[var(--bg-tertiary)] transition"
      >
        {$t('common.cancel')}
      </button>
      <button
        type="button"
        onclick={handleConfirm}
        class="px-4 py-2 text-sm rounded-lg text-white transition {danger ? 'bg-[var(--danger)] hover:opacity-90' : 'bg-[var(--accent)] hover:bg-[var(--accent-hover)]'}"
      >
        {confirmText || $t('common.delete')}
      </button>
    </div>
  </div>
</Modal>
