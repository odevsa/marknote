import { writable, derived } from "svelte/store";
import en from "./locales/en.json";
import ptBR from "./locales/pt-BR.json";
import ptPT from "./locales/pt-PT.json";
import es from "./locales/es.json";
import fr from "./locales/fr.json";
import it from "./locales/it.json";
import de from "./locales/de.json";
import ja from "./locales/ja.json";
import zhCN from "./locales/zh-CN.json";
import ru from "./locales/ru.json";
import ko from "./locales/ko.json";
import nl from "./locales/nl.json";

export type Locale =
  | "de"
  | "en"
  | "es"
  | "fr"
  | "it"
  | "nl"
  | "pt-BR"
  | "pt-PT"
  | "ru"
  | "ja"
  | "zh-CN"
  | "ko";

export const locales: { id: Locale; label: string }[] = [
  { id: "de", label: "Deutsch" },
  { id: "en", label: "English" },
  { id: "es", label: "Español" },
  { id: "fr", label: "Français" },
  { id: "it", label: "Italiano" },
  { id: "nl", label: "Nederlands" },
  { id: "pt-BR", label: "Português (BR)" },
  { id: "pt-PT", label: "Português" },
  { id: "ru", label: "Русский" },
  { id: "ja", label: "日本語" },
  { id: "zh-CN", label: "简体中文" },
  { id: "ko", label: "한국어" },
];

const translations: Record<Locale, any> = {
  de: de,
  en: en,
  es: es,
  fr: fr,
  it: it,
  nl: nl,
  "pt-BR": ptBR,
  "pt-PT": ptPT,
  ru: ru,
  ja: ja,
  "zh-CN": zhCN,
  ko: ko,
};

function getInitialLocale(): Locale {
  if (typeof window === "undefined") return "en";

  const saved = (localStorage.getItem("marknote_locale") || localStorage.getItem("memomark_locale")) as Locale;
  if (saved && translations[saved]) return saved;

  const browserLang = navigator.language.toLowerCase();
  if (browserLang.startsWith("pt-pt") || browserLang.startsWith("pt-mz") || browserLang.startsWith("pt-ao")) return "pt-PT";
  if (browserLang.startsWith("pt")) return "pt-BR";
  if (browserLang.startsWith("es")) return "es";
  if (browserLang.startsWith("fr")) return "fr";
  if (browserLang.startsWith("it")) return "it";
  if (browserLang.startsWith("de")) return "de";
  if (browserLang.startsWith("ja")) return "ja";
  if (browserLang.startsWith("zh")) return "zh-CN";
  if (browserLang.startsWith("ru")) return "ru";
  if (browserLang.startsWith("ko")) return "ko";
  if (browserLang.startsWith("nl")) return "nl";

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
