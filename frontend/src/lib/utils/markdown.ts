import DOMPurify from 'dompurify';
import hljs from 'highlight.js';
import { marked } from 'marked';

// Configure syntax highlighting for code blocks in marked
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

/**
 * Renders markdown string to sanitized HTML with syntax highlighting
 */
export function renderMarkdown(content: string): string {
  if (!content || !content.trim()) {
    return '';
  }
  try {
    const raw = marked.parse(content, { async: false }) as string;
    return DOMPurify.sanitize(raw);
  } catch (err) {
    console.error('Error rendering markdown:', err);
    return content;
  }
}
