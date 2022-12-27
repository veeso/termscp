/**
 * @description resolve copyright year
 */

function resolveCopyright() {
  const year = new Date().getFullYear();
  $("[resolve-copyright]").each(function () {
    $(this).text(year);
  });
}

/**
 * @description resolve video fallback source in case fails. Uses an image instead
 */
function resolveVideoFallback() {
  $("[resolve-video-fallback]").each(function () {
    const fallback = $(this).attr("resolve-video-fallback");
    // Add listener
    $(this).on("error", function () {
      const image = document.createElement("img");
      image.src = fallback;
      image.classList = ["preview"];
      $(this).parent().replaceWith(image);
    });
  });
}

// init
$(function () {
  resolveCopyright();
  resolveVideoFallback();
});
