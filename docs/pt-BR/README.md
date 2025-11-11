# termscp

<p align="center">
  <img src="/assets/images/termscp.svg" alt="termscp logo" width="256" height="256" />
</p>

<p align="center">~ Uma transferência de arquivos de terminal rica em recursos ~</p>
<p align="center">
  <a href="https://termscp.veeso.dev" target="_blank">Website</a>
  ·
  <a href="https://termscp.veeso.dev/get-started.html" target="_blank">Instalação</a>
  ·
  <a href="https://termscp.veeso.dev/user-manual.html" target="_blank">Manual do usuário</a>
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

<p align="center">Desenvolvido por <a href="https://veeso.me/" target="_blank">@veeso</a></p>
<p align="center">Versão atual: 0.19.0 11/11/2025</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso/termscp/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/termscp?style=flat"
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
  <a href="https://coveralls.io/github/veeso/termscp"
    ><img
      src="https://coveralls.io/repos/github/veeso/termscp/badge.svg"
      alt="Coveralls"
  /></a>
</p>

---

## Sobre o termscp 🖥

Termscp é um explorador e utilitário de transferência de arquivos com uma interface de terminal, com suporte para SCP/SFTP/FTP/Kube/S3/WebDAV. Basicamente, é uma ferramenta de terminal com uma interface de usuário para conectar-se a um servidor remoto para baixar e enviar arquivos e interagir com o sistema de arquivos local. Ele é compatível com **Linux**, **MacOS**, **FreeBSD**, **NetBSD** e **Windows**.

![Explorer](/assets/images/explorer.gif)

---

## Recursos 🎁

- 📁  Diferentes protocolos de comunicação
  - **SFTP**
  - **SCP**
  - **FTP** e **FTPS**
  - **Kube**
  - **S3**
  - **SMB**
  - **WebDAV**
- 🖥  Explore e opere no sistema de arquivos remoto e local com uma interface amigável
  - Crie, remova, renomeie, pesquise, visualize e edite arquivos
- ⭐  Conecte-se aos seus hosts favoritos por meio de marcadores integrados e conexões recentes
- 📝  Veja e edite arquivos com suas aplicações favoritas
- 💁  Autenticação SFTP/SCP com chaves SSH e nome de usuário/senha
- 🐧  Compatível com Windows, Linux, FreeBSD, NetBSD e MacOS
- 🎨  Personalize do seu jeito!
  - Temas
  - Formato de explorador de arquivos customizável
  - Editor de texto personalizável
  - Ordenação de arquivos customizável
  - e muitos outros parâmetros...
- 📫  Receba notificações no Desktop quando um arquivo grande for transferido
- 🔭  Mantenha as alterações de arquivos sincronizadas com o host remoto
- 🔐  Salve sua senha no cofre de senhas do sistema operacional
- 🦀  Feito em Rust
- 👀  Desenvolvido com foco em desempenho
- 🦄  Atualizações frequentes e incríveis

---

## Como começar 🚀

Se você está pensando em instalar o termscp, eu quero te agradecer 💜 ! Espero que você goste do termscp!  
Se você quiser contribuir para este projeto, não se esqueça de verificar nosso [guia de contribuição](CONTRIBUTING.md).

Se você é um usuário de Linux, FreeBSD ou MacOS, este simples script de shell instalará o termscp no seu sistema com um único comando:

```sh
curl --proto '=https' --tlsv1.2 -sSLf "https://git.io/JBhDb" | sh
```

> ❗ A instalação no MacOS requer [Homebrew](https://brew.sh/), caso contrário, o compilador Rust será instalado.

Se você é um usuário de Windows, pode instalar o termscp com [Chocolatey](https://chocolatey.org/):

```ps
choco install termscp
```

Usuários do NetBSD podem instalar o termscp pelos repositórios oficiais.

```sh
pkgin install termscp
```

Usuários do Arch Linux podem instalar o termscp pelos repositórios oficiais.

```sh
pacman -S termscp
```

Para mais informações ou outras plataformas, visite [termscp.veeso.dev](https://termscp.veeso.dev/get-started.html) para ver todos os métodos de instalação.

⚠️ Se você quer saber como atualizar o termscp, basta executar o termscp a partir do CLI com: `(sudo) termscp --update` ⚠️

### Requisitos ❗

- Para usuários de **Linux**:
  - libdbus-1
  - pkg-config
  - libsmbclient
- Para usuários de **FreeBSD** ou **NetBSD**:
  - dbus
  - pkgconf
  - libsmbclient

### Requisitos Opcionais ✔️

Estes requisitos não são obrigatórios para rodar o termscp, mas para aproveitar todos os seus recursos.

- Para usuários de **Linux/FreeBSD**:
  - Para **abrir** arquivos via `V` (pelo menos um dos seguintes)
    - *xdg-open*
    - *gio*
    - *gnome-open*
    - *kde-open*
- Para usuários de **Linux**:
  - Um gerenciador de chaves: leia mais no [Manual do Usuário](docs/man.md#linux-keyring)
- Para usuários do **WSL**
  - Para **abrir** arquivos via `V` (pelo menos um dos seguintes)
    - [wslu](https://github.com/wslutilities/wslu)

---

## Apoie o desenvolvedor ☕

Se você gosta do termscp e está grato pelo trabalho que fiz, considere uma pequena doação 🥳

Você pode fazer uma doação por meio de uma dessas plataformas:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## Manual do Usuário 📚

O manual do usuário pode ser encontrado no [site do termscp](https://termscp.veeso.dev/user-manual.html) ou no [Github](docs/man.md).

---

## Próximos Recursos 🧪

Para **2023**, haverá duas grandes atualizações durante o ano.

Além de novos recursos, o desenvolvimento do termscp agora está focado em melhorias de UX e desempenho, então, se você tiver alguma sugestão, sinta-se à vontade para abrir um problema.

---

## Contribuições e problemas 🤝🏻

Contribuições, relatos de bugs, novos recursos e perguntas são bem-vindos! 😉
Se você tiver alguma pergunta ou preocupação, ou se quiser sugerir um novo recurso, ou apenas melhorar o termscp, sinta-se à vontade para abrir um problema ou um PR.

Uma contribuição **apreciada** seria a tradução do manual do usuário e do README para **outros idiomas**.

Por favor, siga [nosso guia de contribuição](CONTRIBUTING.md).

---

## Mudanças ⏳

Veja o changelog do termscp [AQUI](CHANGELOG.md).

---

## Impulsionado por 💪

O termscp é impulsionado por esses projetos incríveis:

- [bytesize](https://github.com/hyunsik/bytesize)
- [crossterm](https://github.com/crossterm-rs/crossterm)
- [edit](https://github.com/milkey-mouse/edit)
- [keyring-rs](https://github.com/hwchen/keyring-rs)
- [open-rs](https://github.com/Byron/open-rs)
- [pavao](https://github.com/veeso/pavao)
- [remotefs](https://github.com/veeso/remotefs-rs)
- [rpassword](https://github.com/conradkleinespel/rpassword)
- [self_update](https://github.com/jaemk/self_update)
- [ratatui](https://github.com/ratatui-org/ratatui)
- [tui-realm](https://github.com/veeso/tui-realm)
- [whoami](https://github.com/libcala/whoami)
- [wildmatch](https://github.com/becheran/wildmatch)

---

## Galeria 🎬

> Termscp Home

![Auth](/assets/images/auth.gif)

> Marcadores

![Bookmarks](/assets/images/bookmarks.gif)

> Configuração

![Setup](/assets/images/config.gif)

> Editor de Texto

![TextEditor](/assets/images/text-editor.gif)

---

## Licença 📃

O termscp é licenciado sob a licença MIT.

Você pode ler a licença completa [AQUI](LICENSE).
