<script lang="ts">
  import logoSvg from '../../../../static/logo.svg?raw';

  let { size = 28, class: className = '' }: { size?: number; class?: string } = $props();
  let colorClass = $derived(className.includes('text-') ? className : `text-[var(--accent)] ${className}`);
</script>

<span
  class="shrink-0 transition-colors inline-flex items-center justify-center {colorClass}"
  style="width: {size}px; height: {size}px;"
  aria-label="MarkNote Logo"
  role="img"
>
  {@html logoSvg}
</span>
