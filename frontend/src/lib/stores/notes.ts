import { writable, derived, get } from "svelte/store";
import { api, type FileTreeNode, type SearchResult } from "../api/client";

export interface ActiveNote {
  path: string;
  content: string;
  initialContent: string;
}

export const fileTree = writable<FileTreeNode[]>([]);
export const activeNote = writable<ActiveNote | null>(null);
export const isSaving = writable<boolean>(false);
export const lastSavedAt = writable<Date | null>(null);
export const searchQuery = writable<string>("");
export const searchResults = writable<SearchResult[]>([]);
export const isSearching = writable<boolean>(false);
export const isSearchOpen = writable<boolean>(false);
export const sidebarOpen = writable<boolean>(false);

// Draft & Unsaved Confirmation Modals
export const showUnsavedConfirmModal = writable<boolean>(false);
export const pendingTargetRoute = writable<{ path: string | null; mode?: ViewMode } | null>(null);

export const showDraftRecoveryModal = writable<boolean>(false);
export const pendingDraftData = writable<{ path: string; mode?: ViewMode; draftContent: string; serverContent: string } | null>(null);

export const isDirty = derived(activeNote, ($note) => {
  if (!$note) return false;
  return $note.content !== $note.initialContent;
});

// Local Storage Draft Helpers
export function getDraftKey(path: string): string {
  return `marknote_draft_${path}`;
}

export function getLocalDraft(path: string): string | null {
  if (typeof window === "undefined") return null;
  return localStorage.getItem(getDraftKey(path));
}

export function saveLocalDraft(path: string, content: string): void {
  if (typeof window === "undefined") return;
  localStorage.setItem(getDraftKey(path), content);
}

export function clearLocalDraft(path: string): void {
  if (typeof window === "undefined") return;
  localStorage.removeItem(getDraftKey(path));
}

export function hasUnsavedDraft(path: string): boolean {
  const current = get(activeNote);
  if (current && current.path === path) {
    if (current.content === current.initialContent) {
      const draft = getLocalDraft(path);
      if (draft === current.initialContent) {
        clearLocalDraft(path);
      }
    }
    return get(isDirty);
  }
  const draft = getLocalDraft(path);
  return draft !== null;
}

export async function loadTree(): Promise<void> {
  try {
    const tree = await api.getTree();
    fileTree.set(tree);
  } catch (err) {
    console.error("Failed to load file tree:", err);
  }
}

import { editorViewMode, type ViewMode } from "./ui";

export interface ParsedRoute {
  path: string | null;
  mode: ViewMode;
}

export function parseCurrentRoute(): ParsedRoute {
  if (typeof window === "undefined") {
    return { path: null, mode: "split" };
  }

  const pathname = window.location.pathname;

  if (pathname.startsWith("/file/")) {
    let raw = pathname.substring(6); // remove '/file/'
    let mode: ViewMode = "preview";

    if (raw.endsWith("/edit")) {
      mode = "edit";
      raw = raw.substring(0, raw.length - 5);
    } else if (raw.endsWith("/split")) {
      mode = "split";
      raw = raw.substring(0, raw.length - 6);
    }

    const decodedPath = decodeURIComponent(raw);
    return { path: decodedPath, mode };
  }

  // Fallback for legacy search params
  const searchParams = new URLSearchParams(window.location.search);
  const paramPath = searchParams.get("path") || searchParams.get("note");
  if (paramPath) {
    return { path: paramPath, mode: "preview" };
  }

  return { path: null, mode: "split" };
}

export function getRouteUrl(path: string, mode: ViewMode = "preview"): string {
  const encoded = path
    .split("/")
    .map((seg) => encodeURIComponent(seg))
    .join("/");
  if (mode === "edit") {
    return `/file/${encoded}/edit`;
  } else if (mode === "split") {
    return `/file/${encoded}/split`;
  } else {
    return `/file/${encoded}`;
  }
}

import { appSettings } from "./settings";

export async function openNote(path: string, mode?: ViewMode, force = false): Promise<void> {
  const current = get(activeNote);
  const settings = get(appSettings);

  // Check if current note has unsaved changes when auto-save is disabled
  if (!force && current && current.path !== path && !settings.auto_save && get(isDirty)) {
    pendingTargetRoute.set({ path, mode });
    showUnsavedConfirmModal.set(true);
    return;
  }

  try {
    const res = await api.getNote(path);
    activeNote.set({
      path: res.path,
      content: res.content,
      initialContent: res.content,
    });

    const targetMode = mode || get(editorViewMode) || "preview";
    editorViewMode.set(targetMode);

    // On mobile, close sidebar when a note is opened
    sidebarOpen.set(false);

    if (typeof window !== "undefined") {
      const targetUrl = getRouteUrl(res.path, targetMode);
      if (window.location.pathname !== targetUrl) {
        window.history.pushState(
          { path: res.path, mode: targetMode },
          "",
          targetUrl,
        );
      }
    }

    // Check for local draft if draft recovery is enabled
    const draft = getLocalDraft(res.path);
    if (draft) {
      if (draft === res.content) {
        clearLocalDraft(res.path);
      } else if (settings.enable_draft_recovery && !settings.auto_save) {
        pendingDraftData.set({
          path: res.path,
          mode: targetMode,
          draftContent: draft,
          serverContent: res.content,
        });
        showDraftRecoveryModal.set(true);
      }
    }
  } catch (err) {
    console.error(`Failed to open note ${path}:`, err);
  }
}

export function closeNote(force = false): void {
  const current = get(activeNote);
  const settings = get(appSettings);

  if (!force && current && !settings.auto_save && get(isDirty)) {
    pendingTargetRoute.set({ path: null });
    showUnsavedConfirmModal.set(true);
    return;
  }

  activeNote.set(null);
  if (typeof window !== "undefined") {
    if (window.location.pathname !== "/") {
      window.history.pushState({}, "", "/");
    }
  }
}

export function confirmDiscardAndNavigate(): void {
  const current = get(activeNote);
  if (current) {
    clearLocalDraft(current.path);
  }

  const target = get(pendingTargetRoute);
  showUnsavedConfirmModal.set(false);
  pendingTargetRoute.set(null);

  if (target) {
    if (target.path) {
      openNote(target.path, target.mode, true);
    } else {
      closeNote(true);
    }
  }
}

export function restoreLocalDraft(): void {
  const data = get(pendingDraftData);
  if (data) {
    activeNote.update((n) => (n ? { ...n, content: data.draftContent } : null));
  }
  showDraftRecoveryModal.set(false);
  pendingDraftData.set(null);
}

export function discardLocalDraft(): void {
  const data = get(pendingDraftData);
  if (data) {
    clearLocalDraft(data.path);
  }
  showDraftRecoveryModal.set(false);
  pendingDraftData.set(null);
}

export async function saveCurrentNote(): Promise<boolean> {
  const current = get(activeNote);
  if (!current) return false;

  isSaving.set(true);
  try {
    await api.saveNote(current.path, current.content);
    clearLocalDraft(current.path);
    activeNote.update((n) => {
      if (!n) return null;
      return { ...n, initialContent: n.content };
    });
    lastSavedAt.set(new Date());
    isSaving.set(false);
    return true;
  } catch (err) {
    console.error("Failed to save note:", err);
    isSaving.set(false);
    return false;
  }
}

export async function createItem(
  path: string,
  isDir: boolean,
): Promise<boolean> {
  try {
    await api.createItem(path, isDir);
    await loadTree();
    if (!isDir) {
      await openNote(path);
    }
    return true;
  } catch (err) {
    console.error("Failed to create item:", err);
    return false;
  }
}

export async function renameItem(
  oldPath: string,
  newPath: string,
): Promise<boolean> {
  try {
    await api.renameItem(oldPath, newPath);
    await loadTree();

    const current = get(activeNote);
    if (current && current.path === oldPath) {
      activeNote.update((n) => (n ? { ...n, path: newPath } : null));
    }
    return true;
  } catch (err) {
    console.error("Failed to rename item:", err);
    return false;
  }
}

export async function deleteItem(path: string): Promise<boolean> {
  try {
    await api.deleteItem(path);
    await loadTree();

    const current = get(activeNote);
    if (
      current &&
      (current.path === path || current.path.startsWith(`${path}/`))
    ) {
      activeNote.set(null);
    }
    return true;
  } catch (err) {
    console.error("Failed to delete item:", err);
    return false;
  }
}

let searchDebounce: any = null;
export function performSearch(query: string) {
  searchQuery.set(query);
  if (searchDebounce) clearTimeout(searchDebounce);

  if (!query.trim()) {
    searchResults.set([]);
    isSearching.set(false);
    return;
  }

  isSearching.set(true);
  searchDebounce = setTimeout(async () => {
    try {
      const results = await api.search(query);
      searchResults.set(results);
    } catch (err) {
      console.error("Search error:", err);
      searchResults.set([]);
    } finally {
      isSearching.set(false);
    }
  }, 250);
}
