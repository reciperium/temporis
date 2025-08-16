/// <reference types="user-agent-data-types" />
import hljs from "highlight.js";
import "highlight.js/styles/github-dark.css";

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

function insertAppArmorConfig() {
  const apparmorscript = `
mkdir ~/Applications

cat << EOF | sudo tee "/etc/apparmor.d/applications"
abi <abi/4.0>,
include <tunables/global>

profile apps /home/woile/Applications/*.AppImage flags=(unconfined) {
  userns,
}
EOF

sudo apparmor_parser -r /etc/apparmor.d/applications`.trim();
  //get document by id apparmor-config and insert apparmorscript
  const apparmorConfig = document.getElementById("apparmor-config");
  if (apparmorConfig) {
    // apparmorConfig.innerHTML = apparmorscript;
    apparmorConfig.appendChild(document.createTextNode(apparmorscript));
  }
}

document.addEventListener("DOMContentLoaded", () => {
  // Existing anchor link and theme toggle logic...

  // Auto-select Mac download tab if needed
  selectMacDownloadTabIfNeeded();

  insertAppArmorConfig();

  hljs.highlightAll();
});
