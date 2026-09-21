<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { marked } from 'marked';
  import hljs from 'highlight.js';
  import DOMPurify from 'dompurify';
  import { EditorView, lineNumbers, keymap } from '@codemirror/view';
  import { EditorState } from '@codemirror/state';
  import { history, defaultKeymap, historyKeymap, indentWithTab } from '@codemirror/commands';
  import { markdown } from '@codemirror/lang-markdown';
  import { HighlightStyle, syntaxHighlighting } from '@codemirror/language';
  import { tags as t } from '@lezer/highlight';
  import {
    Bold,
    Italic,
    Heading,
    Code,
    Quote,
    List,
    ListOrdered,
    Link,
    Table,
    Columns,
    Edit3,
    Eye,
    Save,
    X
  } from 'lucide-svelte';
  import { activeNote, saveCurrentNote, closeNote, getRouteUrl, saveLocalDraft, clearLocalDraft } from '$lib/stores/notes';
  import { editorViewMode, type ViewMode } from '$lib/stores/ui';
  import { appSettings } from '$lib/stores/settings';

  function changeViewMode(mode: ViewMode) {
    editorViewMode.set(mode);
    if ($activeNote && typeof window !== 'undefined') {
      const targetUrl = getRouteUrl($activeNote.path, mode);
      window.history.pushState({ path: $activeNote.path, mode }, '', targetUrl);
    }
  }
  import { t as i18n } from '$lib/i18n';

  // Configure syntax highlighting for code blocks in preview
  marked.use({
    renderer: {
      code({ text, lang }: { text: string; lang?: string }) {
        const validLang = !!(lang && hljs.getLanguage(lang));
        let highlighted = '';
        try {
          highlighted = validLang
            ? hljs.highlight(text, { language: lang }).value
            : hljs.highlightAuto(text).value;
        } catch (err) {
          highlighted = text;
        }
        return `<pre class="hljs"><code class="${validLang ? 'language-' + lang : ''}">${highlighted}</code></pre>\n`;
      }
    }
  });

  // Define markdown syntax highlighting style using active theme variables
  const markdownHighlightStyle = HighlightStyle.define([
    { tag: t.heading1, fontSize: '1.35em', fontWeight: 'bold', color: 'var(--code-title)' },
    { tag: t.heading2, fontSize: '1.2em', fontWeight: 'bold', color: 'var(--code-title)' },
    { tag: t.heading3, fontSize: '1.1em', fontWeight: 'bold', color: 'var(--code-title)' },
    { tag: t.heading, fontWeight: 'bold', color: 'var(--code-title)' },
    { tag: t.strong, fontWeight: 'bold', color: 'var(--code-keyword)' },
    { tag: t.emphasis, fontStyle: 'italic' },
    { tag: t.strikethrough, textDecoration: 'line-through' },
    { tag: t.keyword, color: 'var(--code-keyword)', fontWeight: '600' },
    { tag: t.string, color: 'var(--code-string)' },
    { tag: t.comment, color: 'var(--code-comment)', fontStyle: 'italic' },
    { tag: t.quote, color: 'var(--code-comment)', fontStyle: 'italic' },
    { tag: t.link, color: 'var(--accent)', textDecoration: 'underline' },
    { tag: t.url, color: 'var(--code-string)' },
    { tag: t.number, color: 'var(--code-number)' },
    { tag: t.variableName, color: 'var(--code-variable)' },
    { tag: t.attributeName, color: 'var(--code-attr)' },
    { tag: t.monospace, color: 'var(--accent)' }
  ]);

  const editorTheme = EditorView.theme({
    '&': {
      height: '100%',
      backgroundColor: 'var(--editor-bg)',
      color: 'var(--text-primary)',
      fontSize: 'var(--code-font-size, 14px)'
    },
    '.cm-scroller': {
      overflow: 'auto',
      fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace',
      lineHeight: '1.65',
      scrollBehavior: 'auto !important'
    },
    '.cm-content': {
      padding: '16px 20px',
      caretColor: 'var(--accent)'
    },
    '.cm-line': {
      padding: '0'
    },
    '&.cm-focused .cm-cursor': {
      borderLeftColor: 'var(--accent)',
      borderLeftWidth: '2px'
    },
    '&.cm-focused .cm-selectionBackground, .cm-selectionBackground, ::selection': {
      backgroundColor: 'var(--accent)',
      opacity: '0.2'
    },
    '.cm-gutters': {
      backgroundColor: 'var(--editor-bg)',
      color: 'var(--text-muted)',
      borderRight: '1px solid var(--border-color)',
      fontSize: '0.85em',
      userSelect: 'none'
    },
    '.cm-activeLineGutter': {
      backgroundColor: 'var(--bg-secondary)',
      color: 'var(--text-primary)'
    },
    '.cm-activeLine': {
      backgroundColor: 'var(--bg-secondary)',
      opacity: '0.4'
    }
  });

  let editorContainerRef: HTMLDivElement | undefined = $state();
  let previewRef: HTMLDivElement | undefined = $state();
  let editorView: EditorView | undefined = $state();

  let activeScrollSource: 'editor' | 'preview' | null = null;
  let scrollTimeout: ReturnType<typeof setTimeout> | null = null;

  function setEditorActive() {
    activeScrollSource = 'editor';
  }

  function setPreviewActive() {
    activeScrollSource = 'preview';
  }

  function clearActiveSourceLater() {
    if (scrollTimeout) clearTimeout(scrollTimeout);
    scrollTimeout = setTimeout(() => {
      activeScrollSource = null;
    }, 150);
  }

  function handleEditorScroll() {
    if ($editorViewMode !== 'split' || !editorView || !previewRef) return;
    if (activeScrollSource === 'preview') return;

    activeScrollSource = 'editor';
    clearActiveSourceLater();

    const scrollDOM = editorView.scrollDOM;
    const maxEditor = scrollDOM.scrollHeight - scrollDOM.clientHeight;
    if (maxEditor <= 0) return;
    const percentage = scrollDOM.scrollTop / maxEditor;
    const maxPreview = previewRef.scrollHeight - previewRef.clientHeight;
    const targetScrollTop = percentage * maxPreview;

    if (Math.abs(previewRef.scrollTop - targetScrollTop) > 1) {
      previewRef.scrollTop = targetScrollTop;
    }
  }

  function handlePreviewScroll() {
    if ($editorViewMode !== 'split' || !editorView || !previewRef) return;
    if (activeScrollSource === 'editor') return;

    activeScrollSource = 'preview';
    clearActiveSourceLater();

    const scrollDOM = editorView.scrollDOM;
    const maxPreview = previewRef.scrollHeight - previewRef.clientHeight;
    if (maxPreview <= 0) return;
    const percentage = previewRef.scrollTop / maxPreview;
    const maxEditor = scrollDOM.scrollHeight - scrollDOM.clientHeight;
    const targetScrollTop = percentage * maxEditor;

    if (Math.abs(scrollDOM.scrollTop - targetScrollTop) > 1) {
      scrollDOM.scrollTop = targetScrollTop;
    }
  }

  // Auto-save timer & Local draft backup
  let autoSaveTimeout: ReturnType<typeof setTimeout> | null = null;
  function handleDocChange(val: string) {
    const current = $activeNote;
    if (!current) return;

    activeNote.update((n) => (n ? { ...n, content: val } : null));

    if (autoSaveTimeout) clearTimeout(autoSaveTimeout);

    if (val === current.initialContent) {
      clearLocalDraft(current.path);
    } else {
      if ($appSettings.auto_save) {
        const delay = $appSettings.auto_save_delay_ms || 1500;
        autoSaveTimeout = setTimeout(() => {
          saveCurrentNote();
        }, delay);
      } else if ($appSettings.enable_draft_recovery) {
        saveLocalDraft(current.path, val);
      }
    }
  }

  let currentNotePath = '';
  $effect(() => {
    if (!editorView) return;
    const note = $activeNote;
    if (!note) return;

    const currentDoc = editorView.state.doc.toString();
    const targetContent = note.content || '';

    if (note.path !== currentNotePath) {
      currentNotePath = note.path;
      if (currentDoc !== targetContent) {
        editorView.dispatch({
          changes: { from: 0, to: currentDoc.length, insert: targetContent }
        });
      }
    } else if (currentDoc !== targetContent && !editorView.hasFocus) {
      editorView.dispatch({
        changes: { from: 0, to: currentDoc.length, insert: targetContent }
      });
    }
  });

  onMount(() => {
    if (!editorContainerRef) return;

    editorView = new EditorView({
      state: EditorState.create({
        doc: $activeNote?.content || '',
        extensions: [
          lineNumbers(),
          history(),
          EditorView.lineWrapping,
          markdown(),
          syntaxHighlighting(markdownHighlightStyle),
          editorTheme,
          keymap.of([
            indentWithTab,
            ...defaultKeymap,
            ...historyKeymap,
            {
              key: 'Mod-s',
              run: () => {
                saveCurrentNote();
                return true;
              }
            }
          ]),
          EditorView.updateListener.of((update) => {
            if (update.docChanged) {
              handleDocChange(update.state.doc.toString());
            }
          })
        ]
      }),
      parent: editorContainerRef
    });

    currentNotePath = $activeNote?.path || '';

    const scrollDOM = editorView.scrollDOM;
    scrollDOM.addEventListener('scroll', handleEditorScroll);
    scrollDOM.addEventListener('mouseenter', setEditorActive);
    scrollDOM.addEventListener('wheel', setEditorActive);
    scrollDOM.addEventListener('touchstart', setEditorActive);
  });

  onDestroy(() => {
    if (editorView) {
      const scrollDOM = editorView.scrollDOM;
      scrollDOM.removeEventListener('scroll', handleEditorScroll);
      scrollDOM.removeEventListener('mouseenter', setEditorActive);
      scrollDOM.removeEventListener('wheel', setEditorActive);
      scrollDOM.removeEventListener('touchstart', setEditorActive);
      editorView.destroy();
    }
  });

  function insertFormatting(prefix: string, suffix = '') {
    if (!editorView) return;
    const state = editorView.state;
    const main = state.selection.main;
    const selected = state.sliceDoc(main.from, main.to);
    const text = selected || 'text';
    const insert = prefix + text + suffix;
    editorView.dispatch({
      changes: { from: main.from, to: main.to, insert },
      selection: {
        anchor: main.from + prefix.length,
        head: main.from + prefix.length + text.length
      }
    });
    editorView.focus();
  }

  function insertTable() {
    const tableTemplate =
      '\n| Header 1 | Header 2 |\n| -------- | -------- |\n| Item 1   | Item 2   |\n';
    insertFormatting(tableTemplate, '');
  }

  let renderedHtml = $derived.by(() => {
    if (!$activeNote?.content) return '';
    try {
      const raw = marked.parse($activeNote.content, { async: false }) as string;
      return DOMPurify.sanitize(raw);
    } catch (e) {
      return '<p class="text-red-500">Error rendering markdown</p>';
    }
  });
</script>

<div class="flex-1 flex flex-col h-full overflow-hidden bg-[var(--editor-bg)]">
  <!-- Toolbar -->
  <div class="h-11 border-b border-[var(--border-color)] px-4 flex items-center justify-between gap-2 select-none shrink-0 overflow-x-auto">
    <!-- Format tools -->
    <div class="flex items-center gap-0.5 sm:gap-1">
      <button
        onclick={() => insertFormatting('**', '**')}
        class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer"
        title={$i18n('editor.bold')}
      >
        <Bold size={16} />
      </button>

      <button
        onclick={() => insertFormatting('*', '*')}
        class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer"
        title={$i18n('editor.italic')}
      >
        <Italic size={16} />
      </button>

      <button
        onclick={() => insertFormatting('### ')}
        class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer"
        title={$i18n('editor.heading')}
      >
        <Heading size={16} />
      </button>

      <div class="h-4 w-px bg-[var(--border-color)] mx-1"></div>

      <button
        onclick={() => insertFormatting('```\n', '\n```')}
        class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer"
        title={$i18n('editor.code')}
      >
        <Code size={16} />
      </button>

      <button
        onclick={() => insertFormatting('> ')}
        class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer"
        title={$i18n('editor.quote')}
      >
        <Quote size={16} />
      </button>

      <button
        onclick={() => insertFormatting('- ')}
        class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer"
        title={$i18n('editor.list')}
      >
        <List size={16} />
      </button>

      <button
        onclick={() => insertFormatting('1. ')}
        class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer"
        title={$i18n('editor.numList')}
      >
        <ListOrdered size={16} />
      </button>

      <button
        onclick={() => insertFormatting('[', '](https://)')}
        class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer"
        title={$i18n('editor.link')}
      >
        <Link size={16} />
      </button>

      <button
        onclick={insertTable}
        class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer"
        title={$i18n('editor.table')}
      >
        <Table size={16} />
      </button>
    </div>

    <!-- View mode switcher & Save button -->
    <div class="flex items-center gap-1.5">
      <button
        onclick={() => saveCurrentNote()}
        class="h-7 px-2.5 rounded-lg bg-[var(--accent)] hover:bg-[var(--accent-hover)] text-white text-xs font-semibold flex items-center justify-center gap-1.5 transition cursor-pointer shrink-0"
        title="Ctrl+S"
      >
        <Save size={14} />
        <span class="hidden sm:inline">{$i18n('common.save')}</span>
      </button>

      <!-- Desktop view mode switcher (3 buttons) -->
      <div class="hidden sm:flex items-center h-7 p-0.5 rounded-lg border border-[var(--border-color)] bg-[var(--bg-secondary)]">
        <button
          onclick={() => changeViewMode('edit')}
          class="h-full px-2 rounded-md text-xs transition flex items-center justify-center cursor-pointer {$editorViewMode === 'edit' ? 'bg-[var(--card-bg)] text-[var(--text-primary)] shadow-xs font-medium' : 'text-[var(--text-muted)]'}"
          title={$i18n('editor.edit')}
        >
          <Edit3 size={14} />
        </button>

        <button
          onclick={() => changeViewMode('split')}
          class="h-full px-2 rounded-md text-xs transition flex items-center justify-center cursor-pointer {$editorViewMode === 'split' ? 'bg-[var(--card-bg)] text-[var(--text-primary)] shadow-xs font-medium' : 'text-[var(--text-muted)]'}"
          title={$i18n('editor.split')}
        >
          <Columns size={14} />
        </button>

        <button
          onclick={() => changeViewMode('preview')}
          class="h-full px-2 rounded-md text-xs transition flex items-center justify-center cursor-pointer {$editorViewMode === 'preview' ? 'bg-[var(--card-bg)] text-[var(--text-primary)] shadow-xs font-medium' : 'text-[var(--text-muted)]'}"
          title={$i18n('editor.preview')}
        >
          <Eye size={14} />
        </button>
      </div>

      <!-- Mobile mode switcher (Single button to toggle between edit and preview) -->
      <div class="flex sm:hidden items-center">
        {#if $editorViewMode === 'preview'}
          <button
            onclick={() => changeViewMode('edit')}
            class="h-7 w-7 rounded-lg border border-[var(--border-color)] bg-[var(--bg-secondary)] text-[var(--text-primary)] hover:bg-[var(--bg-tertiary)] flex items-center justify-center transition cursor-pointer"
            title={$i18n('editor.edit')}
          >
            <Edit3 size={14} />
          </button>
        {:else}
          <button
            onclick={() => changeViewMode('preview')}
            class="h-7 w-7 rounded-lg border border-[var(--border-color)] bg-[var(--bg-secondary)] text-[var(--text-primary)] hover:bg-[var(--bg-tertiary)] flex items-center justify-center transition cursor-pointer"
            title={$i18n('editor.preview')}
          >
            <Eye size={14} />
          </button>
        {/if}
      </div>

      <!-- Close Note Button -->
      <button
        onclick={() => closeNote()}
        class="h-7 w-7 flex items-center justify-center rounded-lg text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-tertiary)] transition cursor-pointer ml-0.5"
        title={$i18n('common.close')}
        aria-label={$i18n('common.close')}
      >
        <X size={15} />
      </button>
    </div>
  </div>

  <!-- Content Workspace -->
  <div class="flex-1 flex overflow-hidden">
    <!-- CodeMirror Editor with Markdown Syntax Highlighting -->
    <div
      class="flex-1 h-full flex flex-col relative {$editorViewMode === 'preview' ? 'hidden' : ''} {$editorViewMode === 'split' ? 'border-r border-[var(--border-color)]' : ''}"
    >
      <div
        bind:this={editorContainerRef}
        class="w-full h-full overflow-hidden"
      ></div>
    </div>

    <!-- Live Preview -->
    <div
      bind:this={previewRef}
      onscroll={handlePreviewScroll}
      onmouseenter={setPreviewActive}
      ontouchstart={setPreviewActive}
      onwheel={setPreviewActive}
      style="scroll-behavior: auto !important;"
      class="flex-1 h-full p-4 sm:p-8 overflow-y-auto bg-[var(--bg-primary)] {$editorViewMode === 'edit' ? 'hidden' : ''}"
    >
      <article class="markdown-body max-w-3xl mx-auto">
        {@html renderedHtml}
      </article>
    </div>
  </div>
</div>
