import { describe, expect, it } from "vitest";
import { manUrl, MAN_REF } from "./fetch-man.mjs";

describe("manUrl", () => {
  it("uses the root man.md for en", () => {
    expect(manUrl("en")).toBe(
      `https://raw.githubusercontent.com/veeso/termscp/${MAN_REF}/docs/man.md`
    );
  });

  it("uses the per-locale path for non-en", () => {
    expect(manUrl("it")).toBe(
      `https://raw.githubusercontent.com/veeso/termscp/${MAN_REF}/docs/it/man.md`
    );
  });

  it("pins to a concrete ref, not a moving branch", () => {
    expect(MAN_REF).not.toBe("main");
    expect(MAN_REF.length).toBeGreaterThan(0);
  });
});
