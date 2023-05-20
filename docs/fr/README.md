# termscp

<p align="center">
  <img src="/assets/images/termscp.svg" width="256" height="256" />
</p>

<p align="center">~ Un file transfer de terminal riche en fonctionnalités ~</p>
<p align="center">
  <a href="https://termscp.veeso.dev" target="_blank">Site internet</a>
  ·
  <a href="https://termscp.veeso.dev/#get-started" target="_blank">Installation</a>
  ·
  <a href="https://termscp.veeso.dev/#user-manual" target="_blank">Manuel de l'Utilisateur</a>
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

<p align="center">Développé par <a href="https://veeso.dev/" target="_blank">@veeso</a></p>
<p align="center">Version actuelle: 0.12.0 (16/05/2023)</p>

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

## À propos des termscp 🖥

Termscp est un file transfer et explorateur de fichiers de terminal riche en fonctionnalités, avec support pour SCP/SFTP/FTP/S3. Essentiellement c'est une utilitaire terminal avec une TUI pour se connecter à un serveur distant pour télécharger de fichiers et interagir avec le système de fichiers local. Il est compatible avec **Linux**, **MacOS**, **FreeBSD** et **Windows**.

![Explorer](/assets/images/explorer.gif)

---

## Fonctionnalités 🎁

- 📁  Différents protocoles de communication
  - **SFTP**
  - **SCP**
  - **FTP** et **FTPS**
  - **S3**
  - **SMB**
- 🖥  Explorer et opérer sur le système de fichiers distant et local avec une interface utilisateur pratique.
  - Créer, supprimer, renommer, rechercher, afficher et modifier des fichiers
- ⭐  Connectez-vous à vos hôtes préférés via des signets et des connexions récentes.
- 📝  Affichez et modifiez des fichiers avec vos applications préférées
- 💁  Authentication SFTP/SCP avec des clés SSH et nom/mot de passe
- 🐧  Compatible avec Windows, Linux, FreeBSD et MacOS
- 🎨  Faites en vôtre !
  - thèmes
  - format d'explorateur de fichiers personnalisé
  - éditeur de texte personnalisable
  - tri de fichiers personallisable
  - et bien d'autres paramètres...
- 📫  Recevez une notification quande un gros fichier est télécharger.
- 🔭  Gardez les modifications de fichiers synchronisées avec l'hôte distant
- 🔐  Enregistre tes mots de passe dans le key vault du systeme.
- 🦀  Rust-powered
- 👀  Développé en gardant un œil sur les performances
- 🦄  Mises à jour fréquentes

---

## Pour commencer 🚀

Si tu envisage d'installer termscp, je veux te remercier 💜 ! J'espère que tu vas apprécier termscp !  
Si tu veux contribuer à ce projet, n'oublié pas de consulter notre [guide de contribution](../../CONTRIBUTING.md).

Si tu es un utilisateur Linux, FreeBSD ou MacOS ce simple shell script installera termscp sur te système en un seule commande:

```sh
curl -sSLf http://get-termscp.veeso.dev | sh
```

tandis que si tu es un utilisateur Windows, tu peux installer termscp avec [Chocolatey](https://chocolatey.org/):

```sh
choco install termscp
```

Pour plus d'informations sur les autres méthodes d'installation, veuillez visiter [termscp.veeso.dev](https://termscp.veeso.dev/termscp/#get-started).

⚠️ Si tu cherche comme de mettre à jour termscp, tu dois exécuter cette commande dans le terminal: `(sudo) termscp --update` ⚠️

### Requis ❗

- **Linux** users:
  - libdbus-1
  - pkg-config
  - libsmbclient
- **FreeBSD** or, **NetBSD** users:
  - dbus
  - pkgconf
  - libsmbclient

### Requis facultatives ✔️

Ces requis ne sont pas obligatoires d'exécuter termscp, mais seulement à toutes ses fonctionnalités

- utilisateurs **Linux/FreeBSD**:
  - Pour **ouvrir** les fichiers via `V` (au moins un de ces)
    - *xdg-open*
    - *gio*
    - *gnome-open*
    - *kde-open*
- utilisateurs **Linux**:
  - Un keyring manager: lire plus dans le [manuel d'utilisateur](man.md#linux-keyring)
- utilisateurs **WSL**
  - Pour **ouvrir** les fichiers via `V` (au moins un de ces)
    - [wslu](https://github.com/wslutilities/wslu)

---

## Me soutenir ☕

Si tu aime termscp et que tu aimerais voir le projet grandir et s'améliorer, voudrais considérer un petit don pour me soutenir

Tu peux faire un don avec l'une de ces plateformes:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## Manuel d'utilisateur et Documentation 📚

Le manuel d'utilisateur peut être trouvé sur le [site de termscp](https://termscp.veeso.dev/termscp/#user-manual) ou sur [Github](man.md).

La documentation peut être trouvé sur Rust Docs <https://docs.rs/termscp>

---

## Contribution et enjeux 🤝🏻

Les contributions, les rapports de bugs, les nouvelles fonctionnalités et les questions sont les bienvenus ! 😉
Si tu ai des questions ou des préoccupations, ou si tu souhaite suggérer une nouvelle fonctionnalité, ou si tu souhaite simplement améliorer les conditions de termscp, n'hésite pas à ouvrir un problème ou un PR.

Veuillez suivre [nos directives de contribution](../../CONTRIBUTING.md)

---

## Journal des modifications ⏳

Afficher le journal des modifications [ICI](../../CHANGELOG.md)

---

## Powered by 💪

termscp est soutenu par ces projets impressionnants:

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
- [tui-rs](https://github.com/fdehau/tui-rs)
- [tui-realm](https://github.com/veeso/tui-realm)
- [whoami](https://github.com/libcala/whoami)
- [wildmatch](https://github.com/becheran/wildmatch)

---

## Gallerie 🎬

> Termscp Home

![Auth](/assets/images/auth.gif)

> Bookmarks

![Bookmarks](/assets/images/bookmarks.gif)

> Setup

![Setup](/assets/images/config.gif)

> Text editor

![TextEditor](/assets/images/text-editor.gif)

---

## Licence 📃

termscp est sous licence MIT.

Vous pouvez lire l'intégralité de la licence [ICI](../../LICENSE)
