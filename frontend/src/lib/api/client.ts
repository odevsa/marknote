export interface AuthStatus {
  initialized: boolean;
}

export interface User {
  username: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}

export interface FileTreeNode {
  name: string;
  path: string;
  is_dir: boolean;
  children?: FileTreeNode[];
}

export interface NoteContent {
  path: string;
  content: string;
}

export interface SearchResult {
  path: string;
  title: string;
  snippet: string;
}

export interface AppSettings {
  show_recent_notes: boolean;
  recent_notes_count: number;
  auto_save: boolean;
  auto_save_delay_ms: number;
  enable_draft_recovery: boolean;
}

export interface RecentNote {
  path: string;
  title: string;
  content: string;
  modified_at: string;
}

class ApiError extends Error {
  constructor(
    public status: number,
    message: string,
  ) {
    super(message);
    this.name = "ApiError";
  }
}

async function request<T>(url: string, options: RequestInit = {}): Promise<T> {
  const headers = new Headers(options.headers || {});
  if (
    !headers.has("Content-Type") &&
    options.body &&
    typeof options.body === "string"
  ) {
    headers.set("Content-Type", "application/json");
  }

  const token =
    typeof window !== "undefined"
      ? localStorage.getItem("marknote_token") ||
        localStorage.getItem("memomark_token")
      : null;
  if (token && !headers.has("Authorization")) {
    headers.set("Authorization", `Bearer ${token}`);
  }

  const response = await fetch(url, {
    ...options,
    headers,
    credentials: "include",
  });

  if (!response.ok) {
    let errorMsg = `HTTP Error ${response.status}`;
    try {
      const data = await response.json();
      if (data && data.error) {
        errorMsg = data.error;
      }
    } catch (_) {}
    throw new ApiError(response.status, errorMsg);
  }

  // Handle 204 or empty response
  if (response.status === 204) {
    return {} as T;
  }

  return response.json();
}

export const api = {
  // Auth
  getAuthStatus: () => request<AuthStatus>("/api/auth/status"),
  setup: (body: any) =>
    request<AuthResponse>("/api/auth/setup", {
      method: "POST",
      body: JSON.stringify(body),
    }),
  login: (body: any) =>
    request<AuthResponse>("/api/auth/login", {
      method: "POST",
      body: JSON.stringify(body),
    }),
  logout: () =>
    request<{ success: boolean }>("/api/auth/logout", { method: "POST" }),
  getMe: () => request<User>("/api/auth/me"),

  // Notes
  getTree: () => request<FileTreeNode[]>("/api/notes/tree"),
  getNote: (path: string) =>
    request<NoteContent>(`/api/notes/content?path=${encodeURIComponent(path)}`),
  getRecentNotes: (limit?: number) =>
    request<RecentNote[]>(`/api/notes/recent${limit ? `?limit=${limit}` : ""}`),
  saveNote: (path: string, content: string) =>
    request<{ success: boolean }>("/api/notes/content", {
      method: "POST",
      body: JSON.stringify({ path, content }),
    }),
  createItem: (path: string, is_dir: boolean) =>
    request<{ success: boolean }>("/api/notes/create", {
      method: "POST",
      body: JSON.stringify({ path, is_dir }),
    }),
  renameItem: (old_path: string, new_path: string) =>
    request<{ success: boolean }>("/api/notes/rename", {
      method: "POST",
      body: JSON.stringify({ old_path, new_path }),
    }),
  deleteItem: (path: string) =>
    request<{ success: boolean }>("/api/notes", {
      method: "DELETE",
      body: JSON.stringify({ path }),
    }),
  search: (q: string) =>
    request<SearchResult[]>(`/api/notes/search?q=${encodeURIComponent(q)}`),

  // Settings
  getSettings: () => request<AppSettings>("/api/settings"),
  updateSettings: (settings: AppSettings) =>
    request<AppSettings>("/api/settings", {
      method: "POST",
      body: JSON.stringify(settings),
    }),
};
