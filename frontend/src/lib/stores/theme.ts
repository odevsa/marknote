import { writable } from "svelte/store";

export type Theme =
  | "system"
  | "dark"
  | "light"
  | "paper"
  | "vscode"
  | "dracula"
  | "onedark"
  | "nord"
  | "monokai"
  | "solarized-dark"
  | "solarized-light"
  | "gruvbox"
  | "tokyo-night"
  | "catppuccin"
  | "matrix";

export const themes: { id: Theme; labelKey: string }[] = [
  { id: "system", labelKey: "theme.system" },
  { id: "catppuccin", labelKey: "theme.catppuccin" },
  { id: "dark", labelKey: "theme.dark" },
  { id: "dracula", labelKey: "theme.dracula" },
  { id: "gruvbox", labelKey: "theme.gruvbox" },
  { id: "light", labelKey: "theme.light" },
  { id: "matrix", labelKey: "theme.matrix" },
  { id: "monokai", labelKey: "theme.monokai" },
  { id: "nord", labelKey: "theme.nord" },
  { id: "onedark", labelKey: "theme.onedark" },
  { id: "paper", labelKey: "theme.paper" },
  { id: "solarized-dark", labelKey: "theme.solarizedDark" },
  { id: "solarized-light", labelKey: "theme.solarizedLight" },
  { id: "tokyo-night", labelKey: "theme.tokyoNight" },
  { id: "vscode", labelKey: "theme.vscode" },
];

function getInitialTheme(): Theme {
  if (typeof window === "undefined") return "system";
  return (
    (localStorage.getItem("marknote_theme") as Theme) ||
    (localStorage.getItem("memomark_theme") as Theme) ||
    "system"
  );
}

export const themeStore = writable<Theme>(getInitialTheme());

export function applyTheme(theme: Theme) {
  if (typeof window === "undefined") return;

  const root = document.documentElement;
  if (theme === "system") {
    const prefersDark = window.matchMedia(
      "(prefers-color-scheme: dark)",
    ).matches;
    root.setAttribute("data-theme", prefersDark ? "dark" : "light");
  } else {
    root.setAttribute("data-theme", theme);
  }
}

export function setTheme(theme: Theme) {
  themeStore.set(theme);
  if (typeof window !== "undefined") {
    localStorage.setItem("marknote_theme", theme);
    applyTheme(theme);
  }
}

export function initThemeListener() {
  if (typeof window === "undefined") return;

  const current = getInitialTheme();
  applyTheme(current);

  window
    .matchMedia("(prefers-color-scheme: dark)")
    .addEventListener("change", () => {
      const active =
        (localStorage.getItem("marknote_theme") as Theme) ||
        (localStorage.getItem("memomark_theme") as Theme) ||
        "system";
      if (active === "system") {
        applyTheme("system");
      }
    });
}
