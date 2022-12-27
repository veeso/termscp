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
