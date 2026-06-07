import en from "./en.json";
import zhCN from "./zh-CN.json";
import it from "./it.json";
import fr from "./fr.json";
import es from "./es.json";

export const locales = ["en", "zh-CN", "it", "fr", "es"] as const;
export type Locale = (typeof locales)[number];
export const defaultLocale: Locale = "en";

/** Authoritative set of translation keys, derived from the en dictionary. */
export type TranslationKey = keyof typeof en;

// `en` is left uncast so it remains the source of truth for `TranslationKey`.
// Other locales may omit keys, so they are typed as partial dictionaries.
const dictionaries = {
  en,
  "zh-CN": zhCN as Partial<Record<TranslationKey, string>>,
  it: it as Partial<Record<TranslationKey, string>>,
  fr: fr as Partial<Record<TranslationKey, string>>,
  es: es as Partial<Record<TranslationKey, string>>,
} satisfies Record<Locale, Partial<Record<TranslationKey, string>>>;

export function isLocale(value: string): value is Locale {
  return (locales as readonly string[]).includes(value);
}

/** Resolve a translator for `locale`, falling back to en, then to the key. */
export function useTranslations(locale: Locale) {
  const dict = dictionaries[locale] ?? dictionaries[defaultLocale];
  // `TranslationKey | (string & {})` keeps autocomplete for known keys while
  // still accepting arbitrary strings, preserving the "return key if missing"
  // contract at runtime.
  return (
    key: TranslationKey | (string & {}),
    vars?: Record<string, string>,
  ): string => {
    let value =
      dict[key as TranslationKey] ??
      dictionaries[defaultLocale][key as TranslationKey] ??
      key;
    if (vars) {
      for (const [k, v] of Object.entries(vars)) {
        value = value.replaceAll(`{${k}}`, v);
      }
    }
    return value;
  };
}
