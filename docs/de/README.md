# termscp

<p align="center">
  <img src="/assets/images/termscp.svg" width="256" height="256" />
</p>

<p align="center">~ Eine funktionsreiche Terminal-Dateiübertragung ~</p>
<p align="center">
  <a href="https://veeso.github.io/termscp/" target="_blank">Webseite</a>
  ·
  <a href="https://veeso.github.io/termscp/#get-started" target="_blank">Installation</a>
  ·
  <a href="https://veeso.github.io/termscp/#user-manual" target="_blank">Benutzerhandbuch</a>
</p>

<p align="center">
  <a href="https://github.com/veeso/termscp"
    ><img
      height="20"
      src="/assets/images/flags/us.png"
      alt="English"
  /></a>
  &nbsp;
  <a
    href="/docs/de/README.md"
    ><img
      height="20"
      src="/assets/images/flags/de.png"
      alt="Deutsch"
  /></a>
  &nbsp;
  <a
    href="/docs/es/README.md"
    ><img
      height="20"
      src="/assets/images/flags/es.png"
      alt="Español"
  /></a>
  &nbsp;
  <a
    href="/docs/fr/README.md"
    ><img
      height="20"
      src="/assets/images/flags/fr.png"
      alt="Français"
  /></a>
  &nbsp;
  <a
    href="/docs/it/README.md"
    ><img
      height="20"
      src="/assets/images/flags/it.png"
      alt="Italiano"
  /></a>
  &nbsp;
  <a
    href="/docs/zh-CN/README.md"
    ><img
      height="20"
      src="/assets/images/flags/cn.png"
      alt="简体中文"
  /></a>
</p>

<p align="center">Entwickelt von <a href="https://veeso.github.io/" target="_blank">@veeso</a></p>
<p align="center">Aktuelle Version: 0.7.0 (12/10/2021)</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso/termscp/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/termscp.svg"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/termscp"
    ><img
      src="https://img.shields.io/crates/d/termscp.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/termscp"
    ><img
      src="https://img.shields.io/crates/v/termscp.svg"
      alt="Latest version"
  /></a>
  <a href="https://www.buymeacoffee.com/veeso"
    ><img
      src="https://img.shields.io/badge/Donate-BuyMeACoffee-yellow.svg"
      alt="Buy me a coffee"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso/termscp/actions"
    ><img
      src="https://github.com/veeso/termscp/workflows/Linux/badge.svg"
      alt="Linux CI"
  /></a>
  <a href="https://github.com/veeso/termscp/actions"
    ><img
      src="https://github.com/veeso/termscp/workflows/MacOS/badge.svg"
      alt="MacOS CI"
  /></a>
  <a href="https://github.com/veeso/termscp/actions"
    ><img
      src="https://github.com/veeso/termscp/workflows/Windows/badge.svg"
      alt="Windows CI"
  /></a>
  <a href="https://github.com/veeso/termscp/actions"
    ><img
      src="https://github.com/veeso/termscp/workflows/FreeBSD/badge.svg"
      alt="FreeBSD CI"
  /></a>
  <a href="https://coveralls.io/github/veeso/termscp"
    ><img
      src="https://coveralls.io/repos/github/veeso/termscp/badge.svg"
      alt="Coveralls"
  /></a>
  <a href="https://docs.rs/termscp"
    ><img
      src="https://docs.rs/termscp/badge.svg"
      alt="Docs"
  /></a>
</p>

---

## Über termscp 🖥

Termscp ist ein funktionsreicher Terminal-Dateitransfer und Explorer mit Unterstützung für SCP/SFTP/FTP/S3. Im Grunde handelt es sich also um ein Terminal-Dienstprogramm mit einer TUI, um eine Verbindung zu einem Remote-Server herzustellen, um Dateien abzurufen und hochzuladen und mit dem lokalen Dateisystem zu interagieren. Es ist **Linux**, **MacOS**, **FreeBSD** und **Windows** kompatibel.

![Explorer](/assets/images/explorer.gif)

---

## Features 🎁

- 📁  Verschiedene Kommunikationsprotokolle
  - **SFTP**
  - **SCP**
  - **FTP** und **FTPS**
  - **Aws S3**
- 🖥  Erkunden und bedienen Sie das Dateisystem der Fernbedienung und des lokalen Computers mit einer praktischen Benutzeroberfläche
  - Erstellen, Entfernen, Umbenennen, Suchen, Anzeigen und Bearbeiten von Dateien
- ⭐  Verbinden Sie sich über integrierte Lesezeichen und aktuelle Verbindungen mit Ihren Lieblingshosts
- 📝  Anzeigen und Bearbeiten von Dateien mit Ihren bevorzugten Anwendungen
- 💁  SFTP/SCP-Authentifizierung mit SSH-Schlüsseln und Benutzername/Passwort
- 🐧  Kompatibel mit Windows, Linux, FreeBSD und MacOS
- 🎨  Mach es zu deinem!
  - Themen
  - Benutzerdefiniertes Datei-Explorer-Format
  - Anpassbarer Texteditor
  - Anpassbare Dateisortierung
  - und viele andere Parameter...
- 📫  Lassen Sie sich benachrichtigen, wenn eine große Datei übertragen wurde
- 🔐  Speichern Sie Ihr Passwort in Ihrem Betriebssystem-Schlüsseltresor
- 🦀  Rust-powered
- 👀  Entwickelt, um die Leistung im Auge zu behalten
- 🦄  Häufige tolle Updates

---

## Loslegen 🚀

Wenn Sie überlegen, termscp zu installieren, möchte ich Ihnen danken 💜 ! Ich hoffe, Sie werden Termscp genießen!  
Wenn Sie zu diesem Projekt beitragen möchten, vergessen Sie nicht, unseren [Beitragsleitfaden](../../CONTRIBUTING.md) zu lesen.

Wenn Sie ein Linux-, FreeBSD- oder MacOS-Benutzer sind, installiert dieses einfache Shell-Skript termscp mit einem einzigen Befehl auf Ihrem System:

```sh
curl --proto '=https' --tlsv1.2 -sSLf "https://git.io/JBhDb" | sh
```

Wenn Sie ein Windows-Benutzer sind, können Sie termscp mit [Chocolatey](https://chocolatey.org/) installieren:

```sh
choco install termscp
```

Für weitere Informationen oder andere Plattformen besuchen Sie bitte [veeso.github.io](https://veeso.github.io/termscp/#get-started), um alle Installationsmethoden anzuzeigen.

⚠️ Wenn Sie wissen möchten, wie Sie termscp aktualisieren können, führen Sie einfach termscp über die CLI aus mit: `(sudo) termscp --update` ⚠️

### Softwareanforderungen ❗

- **Linux** Benutzer:
  - libssh
  - libdbus-1
  - pkg-config
- **FreeBSD** Benutzer:
  - libssh
  - dbus
  - pkgconf

### Optionale Softwareanforderungen ✔️

Diese Anforderungen sind nicht zwingend erforderlich, um termscp auszuführen, sondern um alle Funktionen nutzen zu können

- **Linux/FreeBSD** Benutzer:
  - Um Dateien mit `V` zu **öffnen** (mindestens eines davon)
    - *xdg-open*
    - *gio*
    - *gnome-open*
    - *kde-open*
- **Linux** Benutzer:
  - Ein Keyring-manager: Lesen Sie mehr in der [Bedienungsanleitung](man.md#linux-keyring)
- **WSL** Benutzer
  - Um Dateien mit `V` zu **öffnen** (mindestens eines davon)
    - [wslu](https://github.com/wslutilities/wslu)

---

## Unterstütze mich ☕

Wenn Ihnen termscp gefällt und Sie für die Arbeit, die ich geleistet habe, dankbar sind, denken Sie bitte über eine kleine Spende nach 🥳

Sie können mit einer dieser Plattformen spenden:

[![Buy-me-a-coffee](https://img.shields.io/badge/Donate-BuyMeACoffee-yellow.svg)](https://www.buymeacoffee.com/veeso)
[![PayPal](https://img.shields.io/badge/Donate-PayPal-blue.svg)](https://www.paypal.me/chrisintin)

---

## User manual and Documentation 📚

Das Benutzerhandbuch finden Sie auf der [termscp-Website](https://veeso.github.io/termscp/#user-manual) oder auf [Github](man.md).

Die Entwicklerdokumentation finden Sie in Rust Docs unter <https://docs.rs/termscp>

---

## Known issues 🧻

- `NoSuchFileOrDirectory` auf verbinden (WSL1): Ich kenne dieses Problem und es ist ein Fehler von WSL, denke ich. Machen Sie sich keine Sorgen, verschieben Sie einfach die ausführbare Datei von termcp an einen anderen PATH-Speicherort, z. B. `/usr/bin`, oder installieren Sie sie über das entsprechende Paketformat (z. B. deb).

---

## Contributing and issues 🤝🏻

Beiträge, Fehlerberichte, neue Funktionen und Fragen sind willkommen! 😉
Wenn Sie Fragen oder Bedenken haben, eine neue Funktion vorschlagen oder einfach nur die Bedingungen verbessern möchten, können Sie ein Problem oder eine PR erstellen.

Bitte befolgen Sie [unsere Beitragsrichtlinien](../../CONTRIBUTING.md)

---

## Changelog ⏳

Änderungsprotokoll von termscp ansehen [HIER](../../CHANGELOG.md)

---

## Powered by 💪

termscp wird von diesen großartigen Projekten unterstützt:

- [bytesize](https://github.com/hyunsik/bytesize)
- [crossterm](https://github.com/crossterm-rs/crossterm)
- [edit](https://github.com/milkey-mouse/edit)
- [keyring-rs](https://github.com/hwchen/keyring-rs)
- [open-rs](https://github.com/Byron/open-rs)
- [rpassword](https://github.com/conradkleinespel/rpassword)
- [rust-s3](https://github.com/durch/rust-s3)
- [self_update](https://github.com/jaemk/self_update)
- [ssh2-rs](https://github.com/alexcrichton/ssh2-rs)
- [suppaftp](https://github.com/veeso/suppaftp)
- [textwrap](https://github.com/mgeisler/textwrap)
- [tui-rs](https://github.com/fdehau/tui-rs)
- [tui-realm](https://github.com/veeso/tui-realm)
- [whoami](https://github.com/libcala/whoami)
- [wildmatch](https://github.com/becheran/wildmatch)

---

## Galerie 🎬

> Termscp Home

![Auth](/assets/images/auth.gif)

> Bookmarks

![Bookmarks](/assets/images/bookmarks.gif)

> Setup

![Setup](/assets/images/config.gif)

> Text editor

![TextEditor](/assets/images/text-editor.gif)

---

## License 📃

termscp ist unter der MIT-Lizenz lizenziert.

Du kannst die gesamte Lizenz [HIER](../../LICENSE) lesen
