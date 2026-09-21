import { writable, derived } from "svelte/store";
import en from "./locales/en.json";
import ptBR from "./locales/pt-BR.json";
import es from "./locales/es.json";
import fr from "./locales/fr.json";
import it from "./locales/it.json";

export type Locale = "en" | "pt-BR" | "es" | "fr" | "it";

export const locales: { id: Locale; label: string }[] = [
  { id: "en", label: "English" },
  { id: "pt-BR", label: "Português (BR)" },
  { id: "es", label: "Español" },
  { id: "fr", label: "Français" },
  { id: "it", label: "Italiano" },
];

const translations: Record<Locale, any> = {
  en: en,
  "pt-BR": ptBR,
  es: es,
  fr: fr,
  it: it,
};

function getInitialLocale(): Locale {
  if (typeof window === "undefined") return "en";

  const saved = (localStorage.getItem("marknote_locale") || localStorage.getItem("memomark_locale")) as Locale;
  if (saved && translations[saved]) return saved;

  const browserLang = navigator.language;
  if (browserLang.toLowerCase().startsWith("pt")) return "pt-BR";
  if (browserLang.toLowerCase().startsWith("es")) return "es";
  if (browserLang.toLowerCase().startsWith("fr")) return "fr";
  if (browserLang.toLowerCase().startsWith("it")) return "it";

  return "en";
}

export const currentLocale = writable<Locale>(getInitialLocale());

export function setLocale(locale: Locale) {
  if (translations[locale]) {
    currentLocale.set(locale);
    if (typeof window !== "undefined") {
      localStorage.setItem("marknote_locale", locale);
    }
  }
}

export const t = derived(currentLocale, ($locale) => {
  return (key: string, vars?: Record<string, string | number>): string => {
    const parts = key.split(".");
    let dict = translations[$locale] || translations["en"];
    let val: any = dict;

    for (const part of parts) {
      if (val && typeof val === "object" && part in val) {
        val = val[part];
      } else {
        // Fallback to English
        val = translations["en"];
        for (const p of parts) {
          if (val && typeof val === "object" && p in val) {
            val = val[p];
          } else {
            return key;
          }
        }
        break;
      }
    }

    if (typeof val === "string") {
      if (vars) {
        return Object.entries(vars).reduce((acc, [k, v]) => {
          return acc.replace(new RegExp(`{${k}}`, "g"), String(v));
        }, val);
      }
      return val;
    }

    return key;
  };
});
