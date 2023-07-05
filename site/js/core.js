/**
 * @description return navigator language. If language is not supported default will be returned
 * @returns {string}
 */

function getNavigatorLanguage() {
  let lang = navigator.language;
  // Complete lang
  if (languageSupported(lang)) {
    return lang;
  }
  // Reduced lang
  lang = lang.split(/[-_]/)[0] || "en";
  if (!languageSupported(lang)) {
    return "en";
  }
  return lang;
}

/**
 * @description check whether provided language is supported by the website
 * @param {string} lang
 * @returns {boolean}
 */
function languageSupported(lang) {
  return ["en", "zh-CN", "it", "fr", "es"].includes(lang);
}

/**
 * @description update website language
 * @param {string} lang
 */
function setSiteLanguage(lang) {
  setLanguage(lang);
}

const converter = new showdown.Converter({ tables: true });

/**
 * @description load page associated to hash
 * @param {string} hash
 */
function loadPage(path) {
  switch (path) {
    case "/":
    case "/index.html":
      loadHtml("home.html");
      break;
    case "/get-started.html":
      loadHtml("get-started.html");
      break;
    case "/user-manual.html":
      loadUserManual();
      break;
    case "/updates.html":
      loadHtml("updates.html");
      break;
    case "/changelog.html":
      loadMarkdown(
        "https://raw.githubusercontent.com/veeso/termscp/main/CHANGELOG.md"
      );
      break;
  }
}

function loadHtml(page) {
  const url = "html/" + page;
  $("#main").load(url, function () {
    onPageLoaded();
  });
}

function loadMenu() {
  $("#menu").load("html/components/menu.html", function () {
    onPageLoaded();
  });
}

function loadFooter() {
  $("#footer").load("html/components/footer.html", function () {
    onPageLoaded();
  });
}

function loadMarkdown(page) {
  getMarkdown(page, function (md) {
    const div = jQuery("<div/>", {
      id: page,
      class: "container markdown",
    });
    div.html(converter.makeHtml(md));
    $("#main").empty();
    $("#main").append(div);
    onPageLoaded();
  });
}

/**
 * @description get markdown and pass result to onLoaded
 * @param {string} url
 * @param {function} onLoaded
 */
function getMarkdown(url, onLoaded) {
  $.ajax({
    url,
    type: "GET",
    dataType: "text",
    success: onLoaded,
  });
}

function loadUserManual() {
  // Load language
  const lang = getNavigatorLanguage();
  if (lang === "en") {
    loadMarkdown(
      `https://raw.githubusercontent.com/veeso/termscp/main/docs/man.md`
    );
  } else {
    loadMarkdown(
      `https://raw.githubusercontent.com/veeso/termscp/main/docs/${lang}/man.md`
    );
  }
}

// startup
$(function () {
  loadPage(window.location.pathname);
  loadMenu();
  loadFooter();
});
