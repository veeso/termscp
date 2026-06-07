import { describe, expect, it } from "vitest";
import { useTranslations, locales, defaultLocale } from "./ui";

describe("i18n", () => {
  it("exposes the five supported locales with en default", () => {
    expect(locales).toEqual(["en", "zh-CN", "it", "fr", "es"]);
    expect(defaultLocale).toBe("en");
  });

  it("resolves a key in the requested locale", () => {
    const t = useTranslations("it");
    expect(t("nav.install")).toBe("Installa");
  });

  it("falls back to en when a key is missing in the locale", () => {
    const t = useTranslations("fr");
    expect(t("nav.manual")).toBe("User manual");
  });

  it("returns the key itself when missing everywhere", () => {
    const t = useTranslations("en");
    expect(t("does.not.exist")).toBe("does.not.exist");
  });

  it("interpolates {vars}", () => {
    const t = useTranslations("en");
    expect(t("hero.tabs", { host: "1.2.3.4" })).toContain("1.2.3.4");
  });

  it("interpolates the full string exactly", () => {
    const t = useTranslations("en");
    expect(t("hero.tabs", { host: "10.0.0.1" })).toBe(
      "termscp — 10.0.0.1 (sftp) connected",
    );
  });
});
