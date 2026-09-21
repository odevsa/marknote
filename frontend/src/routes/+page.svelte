<script lang="ts">
  import { api, type RecentNote } from '$lib/api/client';
  import MarkdownEditor from '$lib/components/editor/MarkdownEditor.svelte';
  import Header from '$lib/components/layout/Header.svelte';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import StatusBar from '$lib/components/layout/StatusBar.svelte';
  import ConfirmDialog from '$lib/components/ui/ConfirmDialog.svelte';
  import Logo from '$lib/components/ui/Logo.svelte';
  import PromptModal from '$lib/components/ui/PromptModal.svelte';
  import { t } from '$lib/i18n';
  import { authStore } from '$lib/stores/auth';
  import {
      activeNote,
      createItem,
      deleteItem,
      fileTree,
      loadTree,
      openNote,
      parseCurrentRoute
  } from '$lib/stores/notes';
  import { appSettings, loadAppSettings } from '$lib/stores/settings';
  import { renderMarkdown } from '$lib/utils/markdown';
  import { Clock, Edit3, FileText, Plus, Trash2 } from 'lucide-svelte';

  let isQuickCreateOpen = $state(false);
  let isDeleteOpen = $state(false);
  let noteToDeletePath = $state('');

  let recentNotes = $state<RecentNote[]>([]);
  let isLoadingRecent = $state(false);
  let hasInitialLoaded = false;

  $effect(() => {
    if ($authStore.user && !hasInitialLoaded) {
      hasInitialLoaded = true;
      loadTree();
      loadAppSettings();

      // Deep linking / Bookmark support: check if /file/... or ?path=... exists in URL
      const route = parseCurrentRoute();
      if (route.path) {
        openNote(route.path, route.mode);
      }
    }
  });

  function handlePopState() {
    const route = parseCurrentRoute();
    if (route.path) {
      openNote(route.path, route.mode);
    } else {
      activeNote.set(null);
    }
  }

  async function fetchRecentNotes() {
    if (!$appSettings.show_recent_notes) return;
    isLoadingRecent = true;
    try {
      recentNotes = await api.getRecentNotes($appSettings.recent_notes_count);
    } catch (err) {
      console.error('Failed to fetch recent notes:', err);
    } finally {
      isLoadingRecent = false;
    }
  }

  $effect(() => {
    const count = $appSettings.recent_notes_count;
    const show = $appSettings.show_recent_notes;
    const treeLength = $fileTree.length;
    const currentActive = $activeNote;

    if (show && !currentActive && treeLength > 0) {
      fetchRecentNotes();
    }
  });

  async function handleQuickCreate(name: string) {
    let fileName = name.trim();
    if (!fileName.endsWith('.md') && !fileName.endsWith('.txt')) {
      fileName += '.md';
    }
    await createItem(fileName, false);
    fetchRecentNotes();
  }

  function handleCardClick(path: string) {
    // Clicking on card opens in preview mode
    openNote(path, 'preview');
  }

  function handleCardEdit(path: string, e: Event) {
    e.stopPropagation();
    // Mobile: open in code mode ('edit'), Desktop: open in hybrid mode ('split')
    const isMobile = typeof window !== 'undefined' && window.innerWidth < 768;
    openNote(path, isMobile ? 'edit' : 'split');
  }

  function promptDeleteNote(path: string, e: Event) {
    e.stopPropagation();
    noteToDeletePath = path;
    isDeleteOpen = true;
  }

  async function confirmDeleteNote() {
    if (noteToDeletePath) {
      await deleteItem(noteToDeletePath);
      fetchRecentNotes();
    }
  }

  function renderPreview(content: string): string {
    if (!content || !content.trim()) {
      return `<p class="italic opacity-60">${$t('editor.emptyNotePlaceholder')}</p>`;
    }
    return renderMarkdown(content);
  }
</script>

<svelte:window onpopstate={handlePopState} />

<div class="h-full flex flex-col overflow-hidden">
  <Header />

  <div class="flex-1 flex overflow-hidden relative">
    <Sidebar />

    <main class="flex-1 flex flex-col overflow-hidden bg-[var(--bg-primary)]">
      {#if $activeNote}
        <MarkdownEditor />
      {:else if $appSettings.show_recent_notes && $fileTree.length > 0 && recentNotes.length > 0}
        <!-- Recent Notes Grid View -->
        <div class="flex-1 overflow-y-auto p-6 sm:p-8 max-w-6xl mx-auto w-full">
          <div class="flex items-center justify-between mb-6 pb-3 border-b border-[var(--border-color)]">
            <div class="flex items-center gap-2.5">
              <Clock size={20} class="text-[var(--accent)]" />
              <h2 class="text-base sm:text-lg font-bold text-[var(--text-primary)]">
                {$t('common.recentNotes')}
              </h2>
            </div>
            <button
              onclick={() => (isQuickCreateOpen = true)}
              class="px-3.5 py-1.5 rounded-lg bg-[var(--accent)] hover:bg-[var(--accent-hover)] text-white text-xs sm:text-sm font-medium flex items-center gap-2 shadow-xs transition cursor-pointer"
            >
              <Plus size={16} />
              <span>{$t('common.newNote')}</span>
            </button>
          </div>

          {#if isLoadingRecent}
            <div class="py-12 text-center text-sm text-[var(--text-muted)]">
              {$t('common.loading')}
            </div>
          {:else}
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
              {#each recentNotes as note (note.path)}
                <div
                  onclick={() => handleCardClick(note.path)}
                  class="group relative rounded-xl border border-[var(--border-color)] bg-[var(--card-bg)] hover:border-[var(--accent)] hover:shadow-lg transition-all duration-200 flex flex-col max-h-256 lg:h-64 overflow-hidden cursor-pointer"
                  role="button"
                  tabindex="0"
                  onkeydown={(e) => e.key === 'Enter' && handleCardClick(note.path)}
                >
                  <!-- Card Header -->
                  <div class="px-4 py-3 border-b border-[var(--border-color)] flex items-center justify-between shrink-0 bg-[var(--bg-secondary)]/50">
                    <div class="flex items-center gap-2 min-w-0 flex-1 pr-2">
                      <FileText size={16} class="text-[var(--accent)] shrink-0" />
                      <span class="font-semibold text-xs sm:text-sm text-[var(--text-primary)] truncate" title={note.title}>
                        {note.title}
                      </span>
                    </div>

                    <!-- Actions -->
                    <div class="flex items-center gap-1 shrink-0 opacity-80 group-hover:opacity-100 transition">
                      <button
                        onclick={(e) => handleCardEdit(note.path, e)}
                        class="p-1.5 rounded-md text-[var(--text-muted)] hover:text-[var(--accent)] hover:bg-[var(--bg-tertiary)] transition cursor-pointer"
                        title={$t('common.edit')}
                      >
                        <Edit3 size={14} />
                      </button>
                      <button
                        onclick={(e) => promptDeleteNote(note.path, e)}
                        class="p-1.5 rounded-md text-[var(--text-muted)] hover:text-[var(--danger)] hover:bg-[var(--bg-tertiary)] transition cursor-pointer"
                        title={$t('common.delete')}
                      >
                        <Trash2 size={14} />
                      </button>
                    </div>
                  </div>

                  <!-- Card Body / Markdown Preview -->
                  <div class="p-4 flex-1 overflow-hidden relative">
                    <div class="markdown-body card-markdown-preview opacity-90">
                      {@html renderPreview(note.content)}
                    </div>
                    <div class="absolute inset-x-0 bottom-0 h-10 bg-gradient-to-t from-[var(--card-bg)] to-transparent pointer-events-none"></div>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      {:else}
        <!-- Empty workspace placeholder when 0 notes exist or show_recent_notes is disabled -->
        <div class="flex-1 flex flex-col items-center justify-center p-6 text-center select-none">
          <Logo size={56} class="mb-4" />
          <h2 class="text-lg font-semibold text-[var(--text-primary)] mb-1">
            {$t('common.appTitle')}
          </h2>
          <p class="text-xs sm:text-sm text-[var(--text-muted)] max-w-sm mb-6 leading-relaxed">
            {$t('common.selectNoteToView')}
          </p>
          <button
            onclick={() => (isQuickCreateOpen = true)}
            class="px-4 py-2 rounded-lg bg-[var(--accent)] hover:bg-[var(--accent-hover)] text-white text-xs sm:text-sm font-medium flex items-center gap-2 shadow-xs transition cursor-pointer"
          >
            <Plus size={16} />
            <span>{$t('common.newNote')}</span>
          </button>
        </div>
      {/if}
    </main>
  </div>

  <StatusBar />
</div>

<!-- Quick Create Modal -->
<PromptModal
  isOpen={isQuickCreateOpen}
  title={$t('common.newNote')}
  label={$t('common.namePlaceholder')}
  placeholder="welcome.md"
  confirmText={$t('common.create')}
  onConfirm={handleQuickCreate}
  onClose={() => (isQuickCreateOpen = false)}
/>

<!-- Delete Confirm Modal -->
<ConfirmDialog
  isOpen={isDeleteOpen}
  title={$t('common.delete')}
  message={`${$t('common.confirmDelete')} "${noteToDeletePath}"?`}
  danger={true}
  confirmText={$t('common.delete')}
  onConfirm={confirmDeleteNote}
  onClose={() => (isDeleteOpen = false)}
/>
