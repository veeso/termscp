# termscp

<p align="center">
  <img src="/assets/images/termscp.svg" width="256" height="256" />
</p>

<p align="center">~ Un file transfer ricco di funzionalità ~</p>
<p align="center">
  <a href="https://termscp.veeso.dev" target="_blank">Sito</a>
  ·
  <a href="https://termscp.veeso.dev/#get-started" target="_blank">Installazione</a>
  ·
  <a href="https://termscp.veeso.dev/#user-manual" target="_blank">Manuale utente</a>
</p>

<p align="center">
  <a href="https://github.com/veeso/termscp"
    ><img
      height="20"
      src="/assets/images/flags/gb.png"
      alt="English"
  /></a>
  &nbsp;
  <a
    href="https://github.com/veeso/termscp/blob/main/docs/de/README.md"
    ><img
      height="20"
      src="/assets/images/flags/de.png"
      alt="Deutsch"
  /></a>
  &nbsp;
  <a
    href="https://github.com/veeso/termscp/blob/main/docs/es/README.md"
    ><img
      height="20"
      src="/assets/images/flags/es.png"
      alt="Español"
  /></a>
  &nbsp;
  <a
    href="https://github.com/veeso/termscp/blob/main/docs/fr/README.md"
    ><img
      height="20"
      src="/assets/images/flags/fr.png"
      alt="Français"
  /></a>
  &nbsp;
  <a
    href="https://github.com/veeso/termscp/blob/main/docs/it/README.md"
    ><img
      height="20"
      src="/assets/images/flags/it.png"
      alt="Italiano"
  /></a>
  &nbsp;
  <a
    href="https://github.com/veeso/termscp/blob/main/docs/zh-CN/README.md"
    ><img
      height="20"
      src="/assets/images/flags/cn.png"
      alt="简体中文"
  /></a>
</p>

<p align="center">Sviluppato da <a href="https://veeso.dev/" target="_blank">@veeso</a></p>
<p align="center">Versione corrente: 0.13.0 (03/03/2024)</p>

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
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
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
</p>

---

## Riguardo a termscp 🖥

Termscp è un file transfer ed explorer ricco di funzionalità, con supporto a SCP/SFTP/FTP/S3. In pratica è un utility su terminale con una terminal user-interface per connettersi a server remoti per scambiare file ed interagire con il file system sia locale che remoto. È compatibile con **Linux**, **MacOS**, **FreeBSD** e **Windows**.

![Explorer](/assets/images/explorer.gif)

---

## Funzionalità 🎁

- 📁  Diversi protocolli di comunicazione
  - **SFTP**
  - **SCP**
  - **FTP** and **FTPS**
  - **S3**
  - **SMB**
  - **WebDAV**
- 🖥  Esplora e opera sia sul file system locale che su quello remoto con una UI di facile utilizzo.
  - Crea, rimuove, rinomina, cerca, visualizza e modifica file
- ⭐  Connettiti ai tuoi host preferiti tramite la funzionalità integrata dei segnalibri e delle connessioni recenti.
- 📝  Visualizza e modifica i file tramite le tue applicazioni preferite.
- 💁  Autenticazione su server SFTP/SCP tramite chiavi SSH e/o username/password
- 🐧  Compatibile con Windows, Linux, FreeBSD e MacOS
- 🎨  Customizzalo!
  - Temi
  - Formattazione dell'explorer
  - Impostazione del text editor predefinito
  - Imposta l'ordinamento di file e cartelle
  - e tanto altro...
- 📫  Ricevi notifiche desktop quando un file di cospicue dimensioni è stato trasferito
- 🔭  Mantieni sincronizzate le modifiche con l'host remoto
- 🔐  Salva le password degli host remoti nel keyring predefinito dal tuo sistema operativo
- 🦀  Rust-powered
- 👀  Progettato tenendo conto delle performance
- 🦄  Aggiornamenti frequenti con nuove funzionalità

---

## Per iniziare 🚀

Intanto se stai considerando di installare termscp, ti voglio ringraziare 💜 e spero che termscp ti piacerà!  
Se vuoi contribuire al progetto, non dimenticarti di leggere la [contribute guide](../../CONTRIBUTING.md).

Se sei un utente che utilizza Linux, FreeBSD o MacOS, questo shell script installerà termscp sul tuo sistema con un comando secco:

```sh
curl -sSLf http://get-termscp.veeso.dev | sh
```

mentre se sei un utente Windows, puoi installare termscp con [Chocolatey](https://chocolatey.org/):

```sh
choco install termscp
```

Per ulteriori informazioni sui metodi di installazione su altre piattaforme, visita [termscp.veeso.dev](https://termscp.veeso.dev/termscp/#get-started).

⚠️  Se stavi cercando come aggiornare la tua versione di termscp, puoi semplicemente lanciare termscp con questi argomenti: `(sudo) termscp --update` ⚠️

### Requisiti ❗

- **Linux** users:
  - libdbus-1
  - pkg-config
  - libsmbclient
- **FreeBSD** or, **NetBSD** users:
  - dbus
  - pkgconf
  - libsmbclient

### Requisiti opzionali ✔️

Questi requisiti non sono per forza necessari, ma lo sono per sfruttare tutte le sue funzionalità:

- Utenti **Linux/FreeBSD**:
  - Per **aprire** i file con `V` (almeno uno di questi)
    - *xdg-open*
    - *gio*
    - *gnome-open*
    - *kde-open*
- Utenti **Linux**:
  - Un keyring manager: Approfondisci nel [Manuale](man.md#linux-keyring)
- Utenti **WSL**
  - Per **aprire** i file con `V` (almeno uno di questi)
    - [wslu](https://github.com/wslutilities/wslu)

---

## Supporta lo sviluppatore ☕

Se ti piace termscp e ti piacerebbe vedere il progetto crescere e migliorare, considera una piccola donazione 🥳.

Puoi fare una donazione tramite una di queste piattaforme:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## Manuale utente 📚

Il manuale utente lo puoi trovare sul [sito di termscp](https://termscp.veeso.dev/termscp/#user-manual) o su [Github](man.md).

---

## Contributi e issues 🤝🏻

Contributi, report di bug, nuove funzionalità e domande sono i benvenuti! 😉
Se hai qualche domanda o dubbio o vuoi suggerire una nuova funzionalità, sentiti libero di aprire un issue o una PR.

Per favore segui le nostre [contributing guidelines](../../CONTRIBUTING.md)

---

## Changelog ⏳

Visualizza [Qui](../../CHANGELOG.md) il changelog

---

## Un grazie a questi progetti 💪

se termscp esiste, è anche grazie a questi fantastici progetti:

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
- [ratatui](https://github.com/ratatui-org/ratatui)
- [tui-realm](https://github.com/veeso/tui-realm)
- [whoami](https://github.com/libcala/whoami)
- [wildmatch](https://github.com/becheran/wildmatch)

---

## Galleria 🎬

> Termscp Home

![Auth](/assets/images/auth.gif)

> Bookmarks

![Bookmarks](/assets/images/bookmarks.gif)

> Configurazione

![Setup](/assets/images/config.gif)

> Text editor

![TextEditor](/assets/images/text-editor.gif)

---

## Licenza 📃

termscp è fornito sotto licenza MIT.

Puoi leggere l'intero documento di licenza [Qui](../../LICENSE)
