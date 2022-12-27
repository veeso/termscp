const hashBlacklist = ["#menu"];
const converter = new showdown.Converter({ tables: true });

/**
 * @description handle hash change
 */
function onHashChange() {
  const hash = location.hash;
  if (!hashBlacklist.includes(hash) && hash.length > 0) {
    selectMenuEntry(location.hash);
    loadPage(hash);
  } else if (hash.length === 0 || hash === "#") {
    loadPage("#intro");
  }
}

/**
 * @description select menu entry
 * @param {*} hash
 */
function selectMenuEntry(hash) {
  // Remove current entry
  $(".pure-menu-selected").removeClass("pure-menu-selected");
  $('a[href$="' + hash + '"]')
    .parent()
    .addClass("pure-menu-selected");
}

/**
 * @description load page associated to hash
 * @param {string} hash
 */
function loadPage(hash) {
  switch (hash) {
    case "#intro":
      loadHtml("intro.html");
      break;
    case "#get-started":
      loadHtml("get-started.html");
      break;
    case "#user-manual":
      loadUserManual();
      break;
    case "#updates":
      loadHtml("updates.html");
      break;
    case "#changelog":
      loadMarkdown(
        "https://raw.githubusercontent.com/veeso/termscp/main/CHANGELOG.md"
      );
      break;
  }
  window.scrollTo(0, 0);
}

function loadHtml(page) {
  const url = "html/" + page;
  $("#main").load(url, function () {
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

function onMenuBurgerClick() {
  const active = $("#menu").hasClass("active");
  if (active) {
    $("#layout").removeClass("active");
    $("#menu").removeClass("active");
  } else {
    $("#layout").addClass("active");
    $("#menu").addClass("active");
  }
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

function onPageLoaded() {
  reloadTranslations();
}

// Register
window.onhashchange = onHashChange;

// Startup
$(function () {
  onHashChange();
  // Init language
  setSiteLanguage(getNavigatorLanguage());
  // Burger event listener
  $("#menu-burger").on("click", onMenuBurgerClick);
  $(".pure-menu-heading").on("click", function () {
    location.hash = "#";
    onHashChange();
  });
});
