function onPageLoaded() {
  reloadTranslations();
  setThemeToggle();
  setTheme(getTheme());
}

function onToggleMenu() {
  const mobileMenu = $("#mobile-menu");
  let wasVisible = false;
  // if not visible set flex and slide in, otherwise slide out
  if (!mobileMenu.is(":visible")) {
    mobileMenu.css("display", "flex");
    mobileMenu.addClass("animate__animated animate__slideInLeft");
  } else {
    mobileMenu.addClass("animate__animated animate__slideOutLeft");
    wasVisible = true;
  }

  // on animation end remove animation, if visible set hidden
  mobileMenu.on("animationend", () => {
    mobileMenu.removeClass(
      "animate__animated animate__slideOutLeft animate__slideInLeft"
    );
    if (wasVisible) {
      mobileMenu.css("display", "none");
    }
    mobileMenu.off("animationend");
  });
}

function getTheme() {
  const theme = localStorage.getItem("theme");

  if (!theme) {
    return window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "theme-dark"
      : "theme-light";
  }

  return theme;
}

function setThemeToggle() {
  if (getTheme() === "theme-dark") {
    $("#theme-toggle-dark-icon").css("display", "block");
    $("#theme-toggle-light-icon").css("display", "none");
  } else {
    $("#theme-toggle-dark-icon").css("display", "none");
    $("#theme-toggle-light-icon").css("display", "block");
  }
}

// function to set a given theme/color-scheme
function setTheme(themeName) {
  localStorage.setItem("theme", themeName);
  if (themeName === "theme-dark") {
    document.documentElement.classList.add("dark");
  } else {
    document.documentElement.classList.remove("dark");
  }
  setThemeToggle();
}

// function to toggle between light and dark theme
function toggleTheme() {
  console.log("theme", getTheme());
  if (getTheme() === "theme-dark") {
    setTheme("theme-light");
  } else {
    setTheme("theme-dark");
  }
}

// Startup
$(function () {
  // Init language
  setSiteLanguage(getNavigatorLanguage());

  // init theme
  setTheme(getTheme());
});
