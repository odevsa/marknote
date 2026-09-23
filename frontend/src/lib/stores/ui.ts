import { writable } from "svelte/store";

export const isSettingsOpen = writable<boolean>(false);

export type ViewMode = "split" | "edit" | "preview";
export const editorViewMode = writable<ViewMode>("split");

export const activeTreeMenuPath = writable<string | null>(null);

export function openSettings() {
  isSettingsOpen.set(true);
}

export function closeSettings() {
  isSettingsOpen.set(false);
}

export function toggleSettings() {
  isSettingsOpen.update((v) => !v);
}

export interface FontSizeOption {
  value: string;
  label: string;
}

export const fontSizes: FontSizeOption[] = [
  { value: "12px", label: "12px" },
  { value: "13px", label: "13px" },
  { value: "14px", label: "14px" },
  { value: "15px", label: "15px" },
  { value: "16px", label: "16px" },
  { value: "18px", label: "18px" },
  { value: "20px", label: "20px" },
];

const DEFAULT_FONT_SIZE = "12px";

export const editorFontSize = writable<string>(DEFAULT_FONT_SIZE);

export function initEditorFontSize() {
  if (typeof window !== "undefined") {
    const saved = localStorage.getItem("marknote_code_font_size");
    const size = saved || DEFAULT_FONT_SIZE;
    editorFontSize.set(size);
    document.documentElement.style.setProperty("--code-font-size", size);
  }
}

export function setEditorFontSize(size: string) {
  editorFontSize.set(size);
  if (typeof window !== "undefined") {
    localStorage.setItem("marknote_code_font_size", size);
    document.documentElement.style.setProperty("--code-font-size", size);
  }
}

export const editorLineWrapping = writable<boolean>(false);

export function initEditorLineWrapping() {
  if (typeof window !== "undefined") {
    const saved = localStorage.getItem("marknote_line_wrapping");
    const enabled = saved === null ? true : saved === "true";
    editorLineWrapping.set(enabled);
  }
}

export function setEditorLineWrapping(enabled: boolean) {
  editorLineWrapping.set(enabled);
  if (typeof window !== "undefined") {
    localStorage.setItem("marknote_line_wrapping", String(enabled));
  }
}
