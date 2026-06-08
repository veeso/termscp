// Injects a language toggle (EN / 中文) into the mdBook menu bar.
// Swaps the leading /en-US/ <-> /zh-CN/ path segment, preserving the
// rest of the path; falls back to the language root on 404 navigation.
(function () {
  const LANGS = [
    { code: "en-US", label: "EN" },
    { code: "zh-CN", label: "中文" },
  ];

  function currentLang() {
    const m = window.location.pathname.match(/\/(en-US|zh-CN)\//);
    return m ? m[1] : "en-US";
  }

  function swapTo(code) {
    const path = window.location.pathname;
    const cur = currentLang();
    if (path.includes(`/${cur}/`)) {
      return path.replace(`/${cur}/`, `/${code}/`);
    }
    return `/${code}/`;
  }

  function build() {
    const right = document.querySelector(".right-buttons");
    if (!right) return;
    const cur = currentLang();
    const wrap = document.createElement("div");
    wrap.className = "lang-switcher";
    wrap.style.display = "inline-flex";
    wrap.style.gap = "0.5rem";
    wrap.style.marginInlineStart = "0.5rem";
    LANGS.forEach((l) => {
      const a = document.createElement("a");
      a.textContent = l.label;
      a.href = swapTo(l.code);
      a.title = l.code;
      a.setAttribute("aria-current", l.code === cur ? "true" : "false");
      if (l.code === cur) a.style.fontWeight = "bold";
      wrap.appendChild(a);
    });
    right.appendChild(wrap);
  }

  if (document.readyState !== "loading") build();
  else document.addEventListener("DOMContentLoaded", build);
})();
