/// <reference types="user-agent-data-types" />

// Detect the user's OS from the browser
function detectOS(): string {
  const userAgent = window.navigator.userAgent,
    platform =
      window.navigator?.userAgentData?.platform || window.navigator.platform,
    macosPlatforms = ["macOS", "Macintosh", "MacIntel", "MacPPC", "Mac68K"],
    windowsPlatforms = ["Win32", "Win64", "Windows", "WinCE"],
    iosPlatforms = ["iPhone", "iPad", "iPod"];
  let os = "Unknown";

  if (macosPlatforms.indexOf(platform) !== -1) {
    os = "Mac OS";
  } else if (iosPlatforms.indexOf(platform) !== -1) {
    os = "iOS";
  } else if (windowsPlatforms.indexOf(platform) !== -1) {
    os = "Windows";
  } else if (/Android/.test(userAgent)) {
    os = "Android";
  } else if (/Linux/.test(platform)) {
    os = "Linux";
  }

  return os;
}

// Update the radio input for Mac OS users
function selectMacDownloadTabIfNeeded() {
  if (detectOS() === "Mac OS") {
    const macRadio = document.getElementById(
      "tab-dmg",
    ) as HTMLInputElement | null;
    if (macRadio) {
      macRadio.checked = true;
    }
  }
}

document.addEventListener("DOMContentLoaded", () => {
  // Existing anchor link and theme toggle logic...

  // Auto-select Mac download tab if needed
  selectMacDownloadTabIfNeeded();

  // (You can keep your anchor link and theme toggle code here)
});
