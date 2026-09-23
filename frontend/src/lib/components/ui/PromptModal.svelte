<script lang="ts">
  import Modal from './Modal.svelte';
  import { t } from '$lib/i18n';

  interface Props {
    isOpen: boolean;
    title: string;
    label?: string;
    initialValue?: string;
    placeholder?: string;
    confirmText?: string;
    onConfirm: (val: string) => void;
    onClose: () => void;
  }

  let {
    isOpen,
    title,
    label,
    initialValue = '',
    placeholder = '',
    confirmText,
    onConfirm,
    onClose
  }: Props = $props();

  let inputVal = $state('');

  $effect(() => {
    if (isOpen) {
      inputVal = initialValue;
    }
  });

  function handleSubmit(e: Event) {
    e.preventDefault();
    if (inputVal.trim()) {
      onConfirm(inputVal.trim());
      onClose();
    }
  }
</script>

<Modal {isOpen} {title} {onClose}>
  <form onsubmit={handleSubmit} class="space-y-4">
    {#if label}
      <label for="prompt-input-field" class="block text-sm font-medium text-[var(--text-muted)]">{label}</label>
    {/if}
    <input
      id="prompt-input-field"
      type="text"
      bind:value={inputVal}
      {placeholder}
      class="w-full px-3 py-2 rounded-lg border border-[var(--border-color)] bg-[var(--bg-primary)] text-[var(--text-primary)] placeholder-[var(--text-muted)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)] text-sm"
    />
    <div class="flex justify-end gap-2 pt-2">
      <button
        type="button"
        onclick={onClose}
        class="px-4 py-2 text-sm rounded-lg border border-[var(--border-color)] text-[var(--text-primary)] hover:bg-[var(--bg-tertiary)] transition"
      >
        {$t('common.cancel')}
      </button>
      <button
        type="submit"
        disabled={!inputVal.trim()}
        class="px-4 py-2 text-sm rounded-lg bg-[var(--accent)] text-[var(--accent-text)] hover:bg-[var(--accent-hover)] disabled:opacity-50 transition"
      >
        {confirmText || $t('common.save')}
      </button>
    </div>
  </form>
</Modal>
