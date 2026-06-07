import en from "./en.json";
import zhCN from "./zh-CN.json";
import it from "./it.json";
import fr from "./fr.json";
import es from "./es.json";

export const locales = ["en", "zh-CN", "it", "fr", "es"] as const;
export type Locale = (typeof locales)[number];
export const defaultLocale: Locale = "en";

const dictionaries: Record<Locale, Record<string, string>> = {
  en: en as Record<string, string>,
  "zh-CN": zhCN as Record<string, string>,
  it: it as Record<string, string>,
  fr: fr as Record<string, string>,
  es: es as Record<string, string>,
};

export function isLocale(value: string): value is Locale {
  return (locales as readonly string[]).includes(value);
}

/** Resolve a translator for `locale`, falling back to en, then to the key. */
export function useTranslations(locale: Locale) {
  const dict = dictionaries[locale] ?? dictionaries[defaultLocale];
  return (key: string, vars?: Record<string, string>): string => {
    let value = dict[key] ?? dictionaries[defaultLocale][key] ?? key;
    if (vars) {
      for (const [k, v] of Object.entries(vars)) {
        value = value.replaceAll(`{${k}}`, v);
      }
    }
    return value;
  };
}
