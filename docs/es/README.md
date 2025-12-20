# termscp

<p align="center">
  <img src="/assets/images/termscp.svg" alt="logo" width="256" height="256" />
</p>

<p align="center">~ Una transferencia de archivos de terminal rica en funciones ~</p>
<p align="center">
  <a href="https://termscp.veeso.dev" target="_blank">Sitio Web</a>
  ·
  <a href="https://termscp.veeso.dev/get-started.html" target="_blank">Instalación</a>
  ·
  <a href="https://termscp.veeso.dev/user-manual.html" target="_blank">Manual de usuario</a>
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
    href="https://github.com/veeso/termscp/blob/main/docs/pt-BR/README.md"
    ><img
      height="20"
      src="/assets/images/flags/br.png"
      alt="Brazilian Portuguese"
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

<p align="center">Desarrollado por <a href="https://veeso.me/" target="_blank">@veeso</a></p>
<p align="center">Versión actual: 0.19.1 11/11/2025</p>

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

## Sobre termscp 🖥

Termscp es un explorador y transferencia de archivos de terminal rico en funciones, con apoyo para SCP/SFTP/FTP/Kube/S3/WebDAV. Básicamente, es una utilidad de terminal con una TUI para conectarse a un servidor remoto para recuperar y cargar archivos e interactuar con el sistema de archivos local. Es compatible con **Linux**, **MacOS**, **FreeBSD** y **Windows**.

![Explorer](/assets/images/explorer.gif)

---

## Características 🎁

- 📁  Diferentes protocolos de comunicación
  - **SFTP**
  - **SCP**
  - **FTP** y **FTPS**
  - **Kube**
  - **S3**
  - **SMB**
  - **WebDAV**
- 🖥  Explore y opere en el sistema de archivos de la máquina local y remota con una interfaz de usuario práctica
  - Cree, elimine, cambie el nombre, busque, vea y edite archivos
- ⭐  Conéctese a sus hosts favoritos y conexiones recientes
- 📝  Ver y editar archivos con sus aplicaciones favoritas
- 💁  Autenticación SFTP / SCP con claves SSH y nombre de usuario / contraseña
- 🐧  compatible con Linux, MacOS, FreeBSD y Windows
- 🎨  Haz lo tuyo!
  - Temas
  - Formato de explorador de archivos personalizado
  - Editor de texto personalizable
  - Clasificación de archivos personalizable
  - y muchos otros parámetros ...
- 📫  Reciba una notificación cuando se haya transferido un archivo grande
- 🔭  Mantenga los cambios de archivos sincronizados con el host remoto
- 🔐  Guarde su contraseña en el almacén de claves de su sistema operativo
- 🦀  Rust-powered
- 👀  Desarrollado sin perder de vista el rendimiento
- 🦄  Actualizaciones frecuentes

---

## Para iniciar 🚀

Si estás considerando instalar termscp, ¡quiero darte las gracias 💜! ¡Espero que disfrutes de termscp!
Si desea contribuir a este proyecto, no olvide consultar nuestra [guía de contribución](../../CONTRIBUTING.md).

Si tu eres un usuario de Linux, FreeBSD o MacOS, este sencillo script de shell instalará termscp en tu sistema con un solo comando:

```sh
curl -sSLf http://get-termscp.veeso.dev | sh
```

mientras que si eres un usuario de Windows, puedes instalar termscp con [Chocolatey](https://chocolatey.org/):

```sh
choco install termscp
```

Para obtener más información u otras plataformas, visite [termscp.veeso.dev](https://termscp.veeso.dev/termscp/get-started.html) para ver todos los métodos de instalación.

⚠️ Si estás buscando cómo actualizar termscp, simplemente ejecute termscp desde CLI con:: `(sudo) termscp --update` ⚠️

### Requisitos ❗

- **Linux** users:
  - libdbus-1
  - pkg-config
  - libsmbclient
- **FreeBSD** or, **NetBSD** users:
  - dbus
  - pkgconf
  - libsmbclient

### Requisitos opcionales ✔️

These requirements are not forced required to run termscp, but to enjoy all of its features

- Usuarios **Linux/FreeBSD**:
  - Para **abrir** archivos con `V` (al menos uno de estos)
    - *xdg-open*
    - *gio*
    - *gnome-open*
    - *kde-open*
- Usuarios **Linux**:
  - Un keyring manager: leer más en el [manual de usuario](man.md#linux-keyring)
- Usuarios **WSL**
  - Para **abrir** archivos con `V` (al menos uno de estos)
    - [wslu](https://github.com/wslutilities/wslu)

---

## Apoyame ☕

Si te gusta termscp y te encantaría que el proyecto crezca y mejore, considera una pequeña donación para apoyarme 🥳

Puedes hacer una donación con una de estas plataformas:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## Manual de usuario y documentación 📚

El manual del usuario se puede encontrar en el [sitio web de termscp](https://termscp.veeso.dev/termscp/user-manual.html) o en [Github](man.md).

---

## Contribuir y problemas 🤝🏻

¡Las contribuciones, los informes de errores, las nuevas funciones y las preguntas son bienvenidas! 😉
Si tiene alguna pregunta o inquietud, o si desea sugerir una nueva función, o simplemente desea mejorar termscp, no dude en abrir un problema o un PR.

Sigue [nuestras pautas de contribución](../../CONTRIBUTING.md)

---

## Changelog ⏳

Ver registro de cambios de termscp [AQUÍ](../../CHANGELOG.md)

---

## Powered by 💪

termscp funciona con estos increíbles proyectos:

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

## Galería 🎬

> Termscp Home

![Auth](/assets/images/auth.gif)

> Bookmarks

![Bookmarks](/assets/images/bookmarks.gif)

> Setup

![Setup](/assets/images/config.gif)

> Text editor

![TextEditor](/assets/images/text-editor.gif)

---

## Licencia 📃

termscp tiene la licencia MIT.

Puede leer la licencia completa [AQUÍ](../../LICENSE)
