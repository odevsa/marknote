import { writable } from "svelte/store";
import { api, type User } from "../api/client";
import { goto } from "$app/navigation";

export interface AuthState {
  user: User | null;
  initialized: boolean | null;
  loading: boolean;
  error: string | null;
}

export const authStore = writable<AuthState>({
  user: null,
  initialized: null,
  loading: true,
  error: null,
});

export async function checkAuth(): Promise<void> {
  authStore.update((s) => ({ ...s, loading: true, error: null }));
  try {
    const status = await api.getAuthStatus();
    authStore.update((s) => ({ ...s, initialized: status.initialized }));

    if (!status.initialized) {
      authStore.update((s) => ({ ...s, user: null, loading: false }));
      return;
    }

    const token =
      typeof window !== "undefined"
        ? localStorage.getItem("marknote_token") ||
          localStorage.getItem("memomark_token")
        : null;

    if (!token) {
      authStore.update((s) => ({ ...s, user: null, loading: false }));
      return;
    }

    // Try fetching me
    try {
      const user = await api.getMe();
      authStore.update((s) => ({ ...s, user, loading: false }));
    } catch (_) {
      authStore.update((s) => ({ ...s, user: null, loading: false }));
    }
  } catch (err: any) {
    authStore.update((s) => ({
      ...s,
      loading: false,
      error: err.message || "Failed to connect to server",
    }));
  }
}

export async function loginUser(payload: any): Promise<boolean> {
  authStore.update((s) => ({ ...s, loading: true, error: null }));
  try {
    const res = await api.login(payload);
    if (typeof window !== "undefined") {
      localStorage.setItem("marknote_token", res.token);
    }
    authStore.update((s) => ({
      ...s,
      user: res.user,
      loading: false,
    }));
    let redirectTarget = "/";
    if (typeof window !== "undefined") {
      const searchParams = new URLSearchParams(window.location.search);
      const param = searchParams.get("redirect");
      if (param && param.startsWith("/") && !param.startsWith("//")) {
        redirectTarget = param;
      }
    }
    await goto(redirectTarget);
    return true;
  } catch (err: any) {
    authStore.update((s) => ({
      ...s,
      loading: false,
      error: err.message || "Login failed",
    }));
    return false;
  }
}

export async function setupUser(payload: any): Promise<boolean> {
  authStore.update((s) => ({ ...s, loading: true, error: null }));
  try {
    const res = await api.setup(payload);
    if (typeof window !== "undefined") {
      localStorage.setItem("marknote_token", res.token);
    }
    authStore.update((s) => ({
      ...s,
      user: res.user,
      initialized: true,
      loading: false,
    }));
    await goto("/");
    return true;
  } catch (err: any) {
    authStore.update((s) => ({
      ...s,
      loading: false,
      error: err.message || "Setup failed",
    }));
    return false;
  }
}

export async function logoutUser(): Promise<void> {
  try {
    await api.logout();
  } catch (_) {}
  if (typeof window !== "undefined") {
    localStorage.removeItem("marknote_token");
    localStorage.removeItem("memomark_token");
  }
  authStore.update((s) => ({ ...s, user: null }));
  await goto("/login");
}
