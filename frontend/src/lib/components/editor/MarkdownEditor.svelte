<script lang="ts">
  import { t as i18n } from '$lib/i18n';
  import { activeNote, clearLocalDraft, closeNote, getRouteUrl, isDirty, saveCurrentNote, saveLocalDraft } from '$lib/stores/notes';
  import { appSettings } from '$lib/stores/settings';
  import { editorLineWrapping, editorViewMode, type ViewMode } from '$lib/stores/ui';
  import { renderMarkdown } from '$lib/utils/markdown';
  import { defaultKeymap, history, historyKeymap, indentWithTab, redo, undo } from '@codemirror/commands';
  import { markdown } from '@codemirror/lang-markdown';
  import { HighlightStyle, syntaxHighlighting } from '@codemirror/language';
  import { Compartment, EditorState } from '@codemirror/state';
  import { EditorView, highlightActiveLine, highlightActiveLineGutter, keymap, lineNumbers } from '@codemirror/view';
  import { tags as t } from '@lezer/highlight';
  import {
      Bold,
      ChevronDown,
      Code,
      CodeXml,
      Columns,
      Edit3,
      Eye,
      Heading,
      Image,
      Italic,
      Link,
      List,
      ListOrdered,
      ListTodo,
      Minus,
      Quote,
      Redo2,
      Save,
      Strikethrough,
      Table,
      Undo2,
      X
  } from 'lucide-svelte';
  import { onDestroy, onMount } from 'svelte';

  function changeViewMode(mode: ViewMode) {
    editorViewMode.set(mode);
    if ($activeNote && typeof window !== 'undefined') {
      const targetUrl = getRouteUrl($activeNote.path, mode);
      window.history.pushState({ path: $activeNote.path, mode }, '', targetUrl);
    }
  }

  // Define markdown syntax highlighting style using active theme variables
  const markdownHighlightStyle = HighlightStyle.define([
    { tag: t.heading1, fontWeight: 'bold', color: 'var(--code-title)' },
    { tag: t.heading2, fontWeight: 'bold', color: 'var(--code-title)' },
    { tag: t.heading3, fontWeight: 'bold', color: 'var(--code-title)' },
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

  const lineWrappingCompartment = new Compartment();

  const editorTheme = EditorView.theme({
    '&': {
      height: '100%',
      backgroundColor: 'var(--editor-bg)',
      color: 'var(--text-primary)',
      fontSize: 'var(--code-font-size, 14px)',
      fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace',
      lineHeight: 'inherit'
    },
    '.cm-scroller': {
      overflow: 'auto',
      fontFamily: 'inherit',
      lineHeight: 'inherit'
    },
    '.cm-scroller::-webkit-scrollbar-corner': {
      backgroundColor: 'transparent'
    },
    '.cm-content': {
      paddingTop: '8px',
      paddingBottom: '8px',
      caretColor: 'var(--accent)'
    },
    '.cm-line': {
      paddingLeft: '10px',
      paddingRight: '10px',
      lineHeight: 'inherit'
    },
    '&.cm-focused .cm-cursor': {
      borderLeftColor: 'var(--accent)',
      borderLeftWidth: '2px'
    },
    '&.cm-focused .cm-selectionBackground, .cm-selectionBackground, ::selection': {
      backgroundColor: 'var(--selection-bg, rgba(59, 130, 246, 0.35)) !important'
    },
    '.cm-gutters': {
      backgroundColor: 'var(--editor-bg)',
      color: 'var(--text-muted)',
      borderRight: '1px solid var(--border-color)',
      position: 'sticky',
      left: 0,
      zIndex: 3,
      fontFamily: 'inherit',
      fontSize: '1em',
      lineHeight: 'inherit',
      userSelect: 'none'
    },
    '.cm-gutterElement': {
      color: 'var(--text-muted)',
      opacity: 0.5,
      lineHeight: 'inherit',
      fontFamily: 'inherit'
    },
    '.cm-activeLineGutter': {
      backgroundColor: 'var(--active-line-bg, rgba(255, 255, 255, 0.05)) !important',
      color: 'var(--accent) !important',
      opacity: '1 !important',
      fontWeight: 'bold'
    },
    '.cm-activeLine': {
      backgroundColor: 'var(--active-line-bg, rgba(255, 255, 255, 0.05)) !important'
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
    }
  });

  $effect(() => {
    const enabled = $editorLineWrapping;
    if (editorView) {
      editorView.dispatch({
        effects: lineWrappingCompartment.reconfigure(enabled ? EditorView.lineWrapping : [])
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
          highlightActiveLineGutter(),
          highlightActiveLine(),
          history(),
          lineWrappingCompartment.of($editorLineWrapping ? EditorView.lineWrapping : []),
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

  let showHeadingDropdown = $state(false);
  let headingDropdownStyle = $state('');

  function toggleHeadingDropdown(e: MouseEvent) {
    const btn = e.currentTarget as HTMLElement;
    const rect = btn.getBoundingClientRect();
    headingDropdownStyle = `top: ${rect.bottom + 6}px; left: ${Math.max(8, rect.left)}px;`;
    showHeadingDropdown = !showHeadingDropdown;
  }

  function applyUndo() {
    if (editorView) {
      undo(editorView);
      editorView.focus();
    }
  }

  function applyRedo() {
    if (editorView) {
      redo(editorView);
      editorView.focus();
    }
  }

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

  function handleToolbarWheel(e: WheelEvent) {
    if (e.deltaY !== 0) {
      e.preventDefault();
      const container = e.currentTarget as HTMLElement;
      container.scrollLeft += e.deltaY;
    }
  }

  let renderedHtml = $derived.by(() => {
    if (!$activeNote?.content) return '';
    return renderMarkdown($activeNote.content);
  });
</script>

<div class="flex-1 flex flex-col h-full overflow-hidden bg-[var(--editor-bg)]">
  <!-- Toolbar Container -->
  <div class="h-11 border-b border-[var(--border-color)] px-2 sm:px-4 flex items-center justify-between gap-1 select-none shrink-0 w-full overflow-hidden bg-[var(--editor-bg)]">
    <!-- Format tools container with scroll fade indicator -->
    <div class="relative flex-1 flex items-center min-w-0 {$editorViewMode === 'preview' ? 'hidden' : ''}">
      <!-- Scrollable format tools list -->
      <div
        onwheel={handleToolbarWheel}
        class="flex-1 flex items-center gap-0.5 sm:gap-1 overflow-x-auto [scrollbar-width:none] [&::-webkit-scrollbar]:hidden py-1 min-w-0 pr-6"
      >
        <!-- Undo / Redo -->
        <button
          onclick={applyUndo}
          class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer shrink-0"
          title={$i18n('editor.undo')}
          aria-label={$i18n('editor.undo')}
        >
          <Undo2 size={20} class="sm:size-[16px]" />
        </button>

        <button
          onclick={applyRedo}
          class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer shrink-0"
          title={$i18n('editor.redo')}
          aria-label={$i18n('editor.redo')}
        >
          <Redo2 size={20} class="sm:size-[16px]" />
        </button>

        <div class="h-4 w-px bg-[var(--border-color)] mx-0.5 sm:mx-1 shrink-0"></div>

        <!-- Headings Dropdown Selector -->
        <div class="shrink-0">
          <button
            onclick={toggleHeadingDropdown}
            class="p-1.5 px-2 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer flex items-center gap-0.5 text-xs font-semibold"
            title={$i18n('editor.heading')}
            aria-label={$i18n('editor.heading')}
          >
            <Heading size={20} class="sm:size-[16px]" />
            <ChevronDown size={14} class="sm:size-[12px]" />
          </button>

          {#if showHeadingDropdown}
            <div
              class="fixed inset-0 z-40"
              onclick={() => (showHeadingDropdown = false)}
              role="presentation"
            ></div>
            <div
              style={headingDropdownStyle}
              class="fixed z-50 min-w-[140px] bg-[var(--card-bg)] border border-[var(--border-color)] rounded-lg shadow-xl py-1 text-xs"
            >
              <button
                onclick={() => { insertFormatting('# '); showHeadingDropdown = false; }}
                class="w-full text-left px-3 py-1.5 hover:bg-[var(--bg-tertiary)] text-[var(--text-primary)] font-bold text-base flex items-center justify-between cursor-pointer"
              >
                <span>H1</span> <span class="text-xs text-[var(--text-muted)] font-normal"># Título 1</span>
              </button>
              <button
                onclick={() => { insertFormatting('## '); showHeadingDropdown = false; }}
                class="w-full text-left px-3 py-1.5 hover:bg-[var(--bg-tertiary)] text-[var(--text-primary)] font-bold text-sm flex items-center justify-between cursor-pointer"
              >
                <span>H2</span> <span class="text-xs text-[var(--text-muted)] font-normal">## Título 2</span>
              </button>
              <button
                onclick={() => { insertFormatting('### '); showHeadingDropdown = false; }}
                class="w-full text-left px-3 py-1.5 hover:bg-[var(--bg-tertiary)] text-[var(--text-primary)] font-bold text-xs flex items-center justify-between cursor-pointer"
              >
                <span>H3</span> <span class="text-xs text-[var(--text-muted)] font-normal">### Título 3</span>
              </button>
              <button
                onclick={() => { insertFormatting('#### '); showHeadingDropdown = false; }}
                class="w-full text-left px-3 py-1.5 hover:bg-[var(--bg-tertiary)] text-[var(--text-primary)] font-semibold text-xs flex items-center justify-between cursor-pointer"
              >
                <span>H4</span> <span class="text-xs text-[var(--text-muted)] font-normal">#### Título 4</span>
              </button>
            </div>
          {/if}
        </div>

        <div class="h-4 w-px bg-[var(--border-color)] mx-0.5 sm:mx-1 shrink-0"></div>

        <!-- Text formatting: Bold, Italic, Strikethrough -->
        <button
          onclick={() => insertFormatting('**', '**')}
          class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer shrink-0"
          title={$i18n('editor.bold')}
          aria-label={$i18n('editor.bold')}
        >
          <Bold size={20} class="sm:size-[16px]" />
        </button>

        <button
          onclick={() => insertFormatting('*', '*')}
          class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer shrink-0"
          title={$i18n('editor.italic')}
          aria-label={$i18n('editor.italic')}
        >
          <Italic size={20} class="sm:size-[16px]" />
        </button>

        <button
          onclick={() => insertFormatting('~~', '~~')}
          class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer shrink-0"
          title={$i18n('editor.strikethrough')}
          aria-label={$i18n('editor.strikethrough')}
        >
          <Strikethrough size={20} class="sm:size-[16px]" />
        </button>

        <div class="h-4 w-px bg-[var(--border-color)] mx-0.5 sm:mx-1 shrink-0"></div>

        <!-- Lists: Bullet list, Numbered list, Checkbox list -->
        <button
          onclick={() => insertFormatting('- ')}
          class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer shrink-0"
          title={$i18n('editor.list')}
          aria-label={$i18n('editor.list')}
        >
          <List size={20} class="sm:size-[16px]" />
        </button>

        <button
          onclick={() => insertFormatting('1. ')}
          class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer shrink-0"
          title={$i18n('editor.numList')}
          aria-label={$i18n('editor.numList')}
        >
          <ListOrdered size={20} class="sm:size-[16px]" />
        </button>

        <button
          onclick={() => insertFormatting('- [ ] ')}
          class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer shrink-0"
          title={$i18n('editor.todoList')}
          aria-label={$i18n('editor.todoList')}
        >
          <ListTodo size={20} class="sm:size-[16px]" />
        </button>

        <div class="h-4 w-px bg-[var(--border-color)] mx-0.5 sm:mx-1 shrink-0"></div>

        <!-- Inserts: Link, Image, Quote, Inline code, Code block, Table, HR -->
        <button
          onclick={() => insertFormatting('[', '](https://)')}
          class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer shrink-0"
          title={$i18n('editor.link')}
          aria-label={$i18n('editor.link')}
        >
          <Link size={20} class="sm:size-[16px]" />
        </button>

        <button
          onclick={() => insertFormatting('![alt](', ')')}
          class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer shrink-0"
          title={$i18n('editor.image')}
          aria-label={$i18n('editor.image')}
        >
          <Image size={20} class="sm:size-[16px]" />
        </button>

        <button
          onclick={() => insertFormatting('> ')}
          class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer shrink-0"
          title={$i18n('editor.quote')}
          aria-label={$i18n('editor.quote')}
        >
          <Quote size={20} class="sm:size-[16px]" />
        </button>

        <button
          onclick={() => insertFormatting('`', '`')}
          class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer shrink-0"
          title={$i18n('editor.code')}
          aria-label={$i18n('editor.code')}
        >
          <Code size={20} class="sm:size-[16px]" />
        </button>

        <button
          onclick={() => insertFormatting('```\n', '\n```')}
          class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer shrink-0"
          title={$i18n('editor.codeBlock')}
          aria-label={$i18n('editor.codeBlock')}
        >
          <CodeXml size={20} class="sm:size-[16px]" />
        </button>

        <button
          onclick={insertTable}
          class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer shrink-0"
          title={$i18n('editor.table')}
          aria-label={$i18n('editor.table')}
        >
          <Table size={20} class="sm:size-[16px]" />
        </button>

        <button
          onclick={() => insertFormatting('\n---\n', '')}
          class="p-1.5 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition cursor-pointer shrink-0"
          title={$i18n('editor.hr')}
          aria-label={$i18n('editor.hr')}
        >
          <Minus size={20} class="sm:size-[16px]" />
        </button>
      </div>

      <!-- Right fadeout transparency overlay indicating horizontal scrollable content -->
      <div class="pointer-events-none absolute right-0 top-0 bottom-0 w-8 bg-gradient-to-r from-transparent via-[var(--editor-bg)]/80 to-[var(--editor-bg)] z-10"></div>
    </div>

    <!-- Action Buttons (Fixed on the right, larger touch targets on mobile) -->
    <div class="flex items-center gap-1.5 sm:gap-2 shrink-0 ml-auto bg-[var(--editor-bg)] pl-1 z-10">
      <!-- Save Button -->
      <button
        onclick={() => saveCurrentNote()}
        disabled={!$isDirty}
        class="h-9 w-9 sm:h-7 sm:w-auto sm:px-2.5 rounded-lg bg-[var(--accent)] hover:bg-[var(--accent-hover)] text-[var(--accent-text)] text-xs font-semibold flex items-center justify-center gap-1.5 transition cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed shrink-0 shadow-xs"
        title="Ctrl+S"
        aria-label={$i18n('common.save')}
      >
        <Save size={20} class="sm:size-[14px]" />
        <span class="hidden sm:inline">{$i18n('common.save')}</span>
      </button>

      <!-- Desktop view mode switcher (3 buttons) -->
      <div class="hidden sm:flex items-center h-7 p-0.5 rounded-lg border border-[var(--border-color)] bg-[var(--bg-secondary)]">
        <button
          onclick={() => changeViewMode('edit')}
          class="h-full px-2 rounded-md text-xs transition flex items-center justify-center cursor-pointer {$editorViewMode === 'edit' ? 'bg-[var(--card-bg)] text-[var(--text-primary)] shadow-xs font-medium' : 'text-[var(--text-muted)]'}"
          title={$i18n('editor.edit')}
          aria-label={$i18n('editor.edit')}
        >
          <Edit3 size={14} />
        </button>

        <button
          onclick={() => changeViewMode('split')}
          class="h-full px-2 rounded-md text-xs transition flex items-center justify-center cursor-pointer {$editorViewMode === 'split' ? 'bg-[var(--card-bg)] text-[var(--text-primary)] shadow-xs font-medium' : 'text-[var(--text-muted)]'}"
          title={$i18n('editor.split')}
          aria-label={$i18n('editor.split')}
        >
          <Columns size={14} />
        </button>

        <button
          onclick={() => changeViewMode('preview')}
          class="h-full px-2 rounded-md text-xs transition flex items-center justify-center cursor-pointer {$editorViewMode === 'preview' ? 'bg-[var(--card-bg)] text-[var(--text-primary)] shadow-xs font-medium' : 'text-[var(--text-muted)]'}"
          title={$i18n('editor.preview')}
          aria-label={$i18n('editor.preview')}
        >
          <Eye size={14} />
        </button>
      </div>

      <!-- Mobile mode switcher (Single button to toggle between edit and preview) -->
      <div class="flex sm:hidden items-center">
        {#if $editorViewMode === 'preview'}
          <button
            onclick={() => changeViewMode('edit')}
            class="h-9 w-9 rounded-lg border border-[var(--border-color)] bg-[var(--bg-secondary)] text-[var(--text-primary)] hover:bg-[var(--bg-tertiary)] flex items-center justify-center transition cursor-pointer shrink-0 shadow-xs"
            title={$i18n('editor.edit')}
            aria-label={$i18n('editor.edit')}
          >
            <Edit3 size={20} />
          </button>
        {:else}
          <button
            onclick={() => changeViewMode('preview')}
            class="h-9 w-9 rounded-lg border border-[var(--border-color)] bg-[var(--bg-secondary)] text-[var(--text-primary)] hover:bg-[var(--bg-tertiary)] flex items-center justify-center transition cursor-pointer shrink-0 shadow-xs"
            title={$i18n('editor.preview')}
            aria-label={$i18n('editor.preview')}
          >
            <Eye size={20} />
          </button>
        {/if}
      </div>

      <!-- Close Note Button -->
      <button
        onclick={() => closeNote()}
        class="h-9 w-9 sm:h-7 sm:w-7 flex items-center justify-center rounded-lg text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-tertiary)] transition cursor-pointer shrink-0 ml-0.5"
        title={$i18n('common.close')}
        aria-label={$i18n('common.close')}
      >
        <X size={20} class="sm:size-[18px]" />
      </button>
    </div>
  </div>

  <!-- Content Workspace -->
  <div class="flex-1 flex min-w-0 min-h-0 overflow-hidden">
    <!-- CodeMirror Editor with Markdown Syntax Highlighting -->
    <div
      class="flex-1 h-full min-w-0 min-h-0 flex flex-col relative {$editorViewMode === 'preview' ? 'hidden' : ''} {$editorViewMode === 'split' ? 'border-r border-[var(--border-color)]' : ''}"
    >
      <div
        bind:this={editorContainerRef}
        class="w-full h-full min-w-0 min-h-0 overflow-hidden"
      ></div>
    </div>

    <!-- Live Preview -->
    <div
      bind:this={previewRef}
      onscroll={handlePreviewScroll}
      onmouseenter={setPreviewActive}
      ontouchstart={setPreviewActive}
      onwheel={setPreviewActive}
      role="region"
      aria-label="Markdown preview"
      style="scroll-behavior: auto !important;"
      class="flex-1 h-full p-4 sm:p-8 overflow-y-auto bg-[var(--bg-primary)] {$editorViewMode === 'edit' ? 'hidden' : ''}"
    >
      <article class="markdown-body max-w-3xl mx-auto">
        {@html renderedHtml}
      </article>
    </div>
  </div>
</div>
