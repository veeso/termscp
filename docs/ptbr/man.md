# Manual do Usuário 🎓

- [Manual do Usuário 🎓](#manual-do-usuário-)
  - [Uso ❓](#uso-)
    - [Argumento de Endereço 🌎](#argumento-de-endereço-)
      - [Argumento de Endereço do AWS S3](#argumento-de-endereço-do-aws-s3)
      - [Argumento de Endereço do WebDAV](#argumento-de-endereço-do-webdav)
      - [Argumento de Endereço do SMB](#argumento-de-endereço-do-smb)
      - [Como a Senha Pode Ser Fornecida 🔐](#como-a-senha-pode-ser-fornecida-)
    - [Subcomandos](#subcomandos)
      - [Importar um Tema](#importar-um-tema)
      - [Instalar a Última Versão](#instalar-a-última-versão)
  - [Parâmetros de Conexão do S3](#parâmetros-de-conexão-do-s3)
    - [Credenciais do S3 🦊](#credenciais-do-s3-)
  - [Explorador de Arquivos 📂](#explorador-de-arquivos-)
    - [Atalhos de Teclado ⌨](#atalhos-de-teclado-)
    - [Trabalhar com Vários Arquivos 🥷](#trabalhar-com-vários-arquivos-)
    - [Navegação Sincronizada ⏲️](#navegação-sincronizada-️)
    - [Abrir e Abrir Com 🚪](#abrir-e-abrir-com-)
  - [Favoritos ⭐](#favoritos-)
    - [Minhas Senhas São Seguras? 😈](#minhas-senhas-são-seguras-)
      - [Keyring do Linux](#keyring-do-linux)
        - [Configuração do KeepassXC para o termscp](#configuração-do-keepassxc-para-o-termscp)
  - [Configuração ⚙️](#configuração-️)
    - [Armazenamento de Chave SSH 🔐](#armazenamento-de-chave-ssh-)
    - [Formato do Explorador de Arquivos](#formato-do-explorador-de-arquivos)
  - [Temas 🎨](#temas-)
    - [Meu Tema Não Carrega 😱](#meu-tema-não-carrega-)
    - [Estilos 💈](#estilos-)
      - [Página de Autenticação](#página-de-autenticação)
      - [Página de Transferência](#página-de-transferência)
      - [Diversos](#diversos)
  - [Editor de Texto ✏](#editor-de-texto-)
  - [Registro de Logs 🩺](#registro-de-logs-)
  - [Notificações 📫](#notificações-)
  - [Observador de Arquivos 🔭](#observador-de-arquivos-)

## Uso ❓

O termscp pode ser iniciado com as seguintes opções:

`termscp [opções]... [protocol://usuário@endereço:porta:diretório-trabalho] [diretório-trabalho-local]`

OU

`termscp [opções]... -b [nome-do-favorito] [diretório-trabalho-local]`

- `-P, --password <senha>` se o endereço for fornecido, a senha será este argumento
- `-b, --address-as-bookmark` resolve o argumento do endereço como um nome de favorito
- `-q, --quiet` Desabilita o registro de logs
- `-v, --version` Exibe informações da versão
- `-h, --help` Exibe a página de ajuda

O termscp pode ser iniciado em três modos diferentes, se nenhum argumento adicional for fornecido, ele exibirá o formulário de autenticação, onde o usuário poderá fornecer os parâmetros necessários para se conectar ao peer remoto.

Alternativamente, o usuário pode fornecer um endereço como argumento para pular o formulário de autenticação e iniciar diretamente a conexão com o servidor remoto.

Se um argumento de endereço ou nome de favorito for fornecido, você também pode definir o diretório de trabalho para o host local.

### Argumento de Endereço 🌎

O argumento de endereço tem a seguinte sintaxe:

```txt
[protocol://][username@]<address>[:port][:wrkdir]
```

Vamos ver alguns exemplos dessa sintaxe particular, pois ela é bem conveniente e você provavelmente a usará com mais frequência do que a outra...

- Conectar usando o protocolo padrão (*definido na configuração*) a 192.168.1.31; a porta, se não for fornecida, será a padrão para o protocolo selecionado (dependerá da sua configuração); o nome de usuário será o do usuário atual

    ```sh
    termscp 192.168.1.31
    ```

- Conectar usando o protocolo padrão (*definido na configuração*) a 192.168.1.31; o nome de usuário é `root`

    ```sh
    termscp root@192.168.1.31
    ```

- Conectar usando scp a 192.168.1.31, a porta é 4022; o nome de usuário é `omar`

    ```sh
    termscp scp://omar@192.168.1.31:4022
    ```

- Conectar usando scp a 192.168.1.31, a porta é 4022; o nome de usuário é `omar`. Você começará no diretório `/tmp`

    ```sh
    termscp scp://omar@192.168.1.31:4022:/tmp
    ```

#### Argumento de Endereço do AWS S3

O AWS S3 tem uma sintaxe diferente para o argumento de endereço CLI, por razões óbvias, mas tentei mantê-la o mais próxima possível do argumento de endereço genérico:

```txt
s3://<bucket-name>@<region>[:profile][:/wrkdir]
```

Exemplo:

```txt
s3://buckethead@eu-central-1:default:/assets
```

#### Argumento de Endereço do WebDAV

Caso você queira se conectar ao WebDAV, use a seguinte sintaxe:

```txt
http://<username>:<password>@<url></path>
```

ou, se preferir usar https:

```txt
https://<username>:<password>@<url></path>
```

#### Argumento de Endereço do SMB

O SMB tem uma sintaxe diferente para o argumento de endereço CLI, que varia se você estiver no Windows ou em outros sistemas:

**Sintaxe do Windows:**

```txt
\\[username@]<server-name>\<share>[\path\...]
```

**Sintaxe de outros sistemas:**

```txt
smb://[username@]<server-name>[:port]/<share>[/path/.../]
```

#### Como a Senha Pode Ser Fornecida 🔐

Você provavelmente notou que, ao fornecer o argumento de endereço, não há como fornecer a senha.
A senha pode ser fornecida basicamente de três maneiras quando o argumento de endereço é fornecido:

- Opção `-P, --password`: apenas use essa opção CLI fornecendo a senha. Eu desaconselho fortemente esse método, pois é muito inseguro (você pode manter a senha no histórico do shell).
- Via `sshpass`: você pode fornecer a senha via `sshpass`, por exemplo, `sshpass -f ~/.ssh/topsecret.key termscp cvisintin@192.168.1.31`.
- Você será solicitado a fornecer a senha: se você não usar nenhum dos métodos anteriores, será solicitado a fornecer a senha, como acontece com ferramentas mais clássicas como `scp`, `ssh`, etc.

### Subcomandos

#### Importar um Tema

Execute o termscp como `termscp theme <theme-file>`

#### Instalar a Última Versão

Execute o termscp como `termscp update`

---

## Parâmetros de Conexão do S3

Esses parâmetros são necessários para se conectar ao AWS S3 e a outros servidores compatíveis com S3:

- AWS S3:
  - **Nome do balde**
  - **Região**
  - *Perfil* (se não fornecido: "default")
  - *Chave de acesso* (a menos que seja público)
  - *Chave de acesso secreta* (a menos que seja público)
  - *Token de segurança* (se necessário)
  - *Token de sessão* (se necessário)
  - Novo estilo de caminho: **NÃO**
- Outros endpoints S3:
  - **Nome do balde**
  - **Endpoint**
  - *Chave de acesso* (a menos que seja público)
  - *Chave de acesso secreta* (a menos que seja público)
  - Novo estilo de caminho: **SIM**

### Credenciais do S3 🦊

Para se conectar a um balde do AWS S3, você obviamente precisa fornecer algumas credenciais.
Existem basicamente três maneiras de fazer isso:
Estes são os métodos para fornecer credenciais para o S3:

1. Formulário de autenticação:


   1. Você pode fornecer a `access_key` (deve ser obrigatória), a `secret_access_key` (deve ser obrigatória), o `security_token` e o `session_token`.
   2. Se você salvar a conexão S3 como um favorito, essas credenciais serão salvas como uma string criptografada AES-256/BASE64 no seu arquivo de favoritos (exceto para o token de segurança e o token de sessão, que são credenciais temporárias).
2. Use seu arquivo de credenciais: basta configurar a CLI da AWS via `aws configure` e suas credenciais já devem estar localizadas em `~/.aws/credentials`. Caso você esteja usando um perfil diferente de `default`, apenas forneça-o no campo de perfil no formulário de autenticação.
3. **Variáveis de ambiente**: você sempre pode fornecer suas credenciais como variáveis de ambiente. Lembre-se de que essas credenciais **sempre substituirão** as credenciais localizadas no arquivo de `credentials`. Veja como configurar o ambiente abaixo:

    Estas devem sempre ser obrigatórias:

    - `AWS_ACCESS_KEY_ID`: ID da chave de acesso da AWS (geralmente começa com `AKIA...`)
    - `AWS_SECRET_ACCESS_KEY`: a chave de acesso secreta

    Caso você tenha configurado uma segurança mais rigorosa, você *pode* precisar destes também:

    - `AWS_SECURITY_TOKEN`: token de segurança
    - `AWS_SESSION_TOKEN`: token de sessão

⚠️ Suas credenciais estão seguras: o termscp não manipula esses valores diretamente! Suas credenciais são consumidas diretamente pelo crate **s3**.
Se você tiver alguma preocupação com a segurança, entre em contato com o autor da biblioteca no [Github](https://github.com/durch/rust-s3) ⚠️

---

## Explorador de Arquivos 📂

Quando nos referimos a exploradores de arquivos no termscp, estamos falando dos painéis que você pode ver após estabelecer uma conexão com o remoto.
Esses painéis são basicamente três (sim, três na verdade):

- Painel do explorador local: ele é exibido à esquerda da sua tela e mostra as entradas do diretório atual do localhost.
- Painel do explorador remoto: ele é exibido à direita da sua tela e mostra as entradas do diretório atual do host remoto.
- Painel de resultados de busca: dependendo de onde você está buscando arquivos (local/remoto), ele substituirá o painel local ou o painel do explorador. Este painel mostra as entradas que correspondem à consulta de busca que você realizou.

Para trocar de painel, você precisa pressionar `<LEFT>` para mover para o painel do explorador remoto e `<RIGHT>` para voltar para o painel do explorador local. Sempre que estiver no painel de resultados da busca, você precisa pressionar `<ESC>` para sair do painel e voltar ao painel anterior.

### Atalhos de Teclado ⌨

| Tecla          | Comando                                                 | Lembrete  |
|----------------|----------------------------------------------------------|-------------|
| `<ESC>`         | Desconectar do remoto; retornar à página de autenticação |           |
| `<BACKSPACE>`   | Voltar ao diretório anterior na pilha                     |           |
| `<TAB>`         | Alternar aba do explorador                                 |           |
| `<RIGHT>`     | Mover para a aba do explorador remoto                      |           |
| `<LEFT>`    | Mover para a aba do explorador local                        |           |
| `<UP>`       | Mover para cima na lista selecionada                         |           |
| `<DOWN>`      | Mover para baixo na lista selecionada                       |           |
| `<PGUP>`       | Mover para cima na lista selecionada por 8 linhas           |           |
| `<PGDOWN>`    | Mover para baixo na lista selecionada por 8 linhas          |           |
| `<ENTER>`     | Entrar no diretório                                         |           |
| `<ESPAÇO>`    | Fazer upload/download do arquivo selecionado                |           |
| `<BACKTAB>`   | Alternar entre aba de logs e explorador                      |           |
| `<A>`           | Alternar arquivos ocultos                                  | Todos     |
| `<B>`           | Ordenar arquivos por                                      | Bubblesort?|
| `<C\|F5>`     | Copiar arquivo/diretório                                  | Copiar     |
| `<D\|F7>`     | Criar diretório                                             | Diretório   |
| `<E\|F8\|DEL>`| Deletar arquivo                                           | Apagar     |
| `<F>`          | Buscar arquivos (suporta pesquisa com coringas)              | Buscar      |
| `<G>`           | Ir para caminho especificado                               | Ir para      |
| `<H\|F1>`     | Mostrar ajuda                                             | Ajuda       |
| `<I>`           | Mostrar informações sobre arquivo ou diretório selecionado | Informação  |
| `<K>`           | Criar link simbólico apontando para a entrada selecionada  | Symlink      |
| `<L>`          | Recarregar conteúdo do diretório atual / Limpar seleção    | Lista        |
| `<M>`           | Selecionar um arquivo                                     | Marcar        |
| `<N>`          | Criar novo arquivo com o nome fornecido                     | Novo         |
| `<O\|F4>`     | Editar arquivo; veja Editor de Texto                          | Abrir          |
| `<P>`           | Abrir painel de logs                                      | Painel       |
| `<Q\|F10>`    | Sair do termscp                                           | Sair          |
| `<R\|F6>`     | Renomear arquivo                                         | Renomear     |
| `<S\|F2>`     | Salvar arquivo como...                                    | Salvar       |
| `<T>`          | Sincronizar alterações para caminho selecionado para remoto | Track        |
| `<U>`           | Ir para o diretório pai                                  | Subir          |
| `<V\|F3>`     | Abrir arquivo com o programa padrão para o tipo de arquivo  | Visualizar   |
| `<W>`           | Abrir arquivo com o programa fornecido                      | Com             |
| `<X>`           | Executar um comando                                        | Executar     |
| `<Y>`           | Alternar navegação sincronizada                             | Sincronizar   |
| `<Z>`           | Alterar modo de arquivo                                   |                   |
| `</>`         | Filtrar arquivos (suporte tanto para regex quanto para coringa)    |             |
| `<CTRL+A>`    | Selecionar todos os arquivos                               |                   |
| `<ALT+A>`    | Deselecionar todos os arquivos                               |                   |
| `<CTRL+C>`    | Abortir processo de transferência de arquivo                  |                   |
| `<CTRL+T>`   | Mostrar todos os caminhos sincronizados                         | Track          |

### Trabalhar com Vários Arquivos 🥷

Você pode optar por trabalhar com vários arquivos, selecionando-os pressionando `<M>`, para selecionar o arquivo atual, ou pressionando `<CTRL+A>`, que selecionará todos os arquivos no diretório de trabalho.
Uma vez que um arquivo esteja marcado para seleção, ele será exibido com um `*` à esquerda.
Ao trabalhar com seleção, apenas o arquivo selecionado será processado para ações, enquanto o item destacado atual será ignorado.
É possível trabalhar com vários arquivos também quando estiver no painel de resultados da busca.
Todas as ações estão disponíveis ao trabalhar com vários arquivos, mas tenha em mente que algumas ações funcionam de forma ligeiramente diferente. Vamos explicar algumas delas:

- *Copiar*: sempre que você copiar um arquivo, você será solicitado a inserir o nome de destino. Ao trabalhar com vários arquivos, esse nome refere-se ao diretório de destino onde todos esses arquivos serão copiados.
- *Renomear*: igual ao copiar, mas moverá os arquivos para lá.
- *Salvar como*: igual ao copiar, mas gravará lá.

### Navegação Sincronizada ⏲️

Quando ativada, a navegação sincronizada permitirá sincronizar a navegação entre os dois painéis.
Isso significa que, sempre que você mudar o diretório de trabalho em um painel, a mesma ação será reproduzida no outro painel. Se quiser ativar a navegação sincronizada, basta pressionar `<Y>`; pressione duas vezes para desativar. Enquanto estiver ativada, o estado da navegação sincronizada será exibido na barra de status como `ON` (Ligado).

### Abrir e Abrir Com 🚪

Os comandos para abrir e abrir com são alimentados pelo [open-rs](https://docs.rs/crate/open/1.7.0).
Ao abrir arquivos com o comando Visualizar (`<V>`), será usado o aplicativo padrão do sistema para o tipo de arquivo. Para isso, será usado o serviço padrão do sistema operacional, então certifique-se de ter pelo menos um destes instalados no seu sistema:

- Para usuários do **Windows**: você não precisa se preocupar, pois o crate usará o comando `start`.
- Para usuários do **MacOS**: também não é necessário se preocupar, pois o crate usará `open`, que já está instalado no seu sistema.
- Para usuários do **Linux**: um dos seguintes deve estar instalado:
  - *xdg-open*
  - *gio*
  - *gnome-open*
  - *kde-open*
- Para usuários do **WSL**: *wslview* é necessário, você deve instalar [wslu](https://github.com/wslutilities/wslu).

> Pergunta: Posso editar arquivos remotos usando o comando de visualização?  
> Resposta: Não, pelo menos não diretamente do "painel remoto". Você deve baixá-lo para um diretório local primeiro, porque quando você abre um arquivo remoto, ele é baixado para um diretório temporário, mas não há como criar um observador para o arquivo para verificar quando o programa que você usou para abri-lo foi fechado, então o termscp não pode saber quando você terminou de editar o arquivo.

---

## Favoritos ⭐

No termscp é possível salvar hosts favoritos, que podem ser carregados rapidamente a partir do layout principal do termscp.
O termscp também salvará os últimos 16 hosts aos quais você se conectou.
Esse recurso permite que você carregue todos os parâmetros necessários para se conectar a um determinado host remoto, simplesmente selecionando o favorito na aba abaixo do formulário de autenticação.

Os favoritos serão salvos, se possível, em:

- `$HOME/.config/termscp/` no Linux/BSD
- `$HOME/Library/Application Support/termscp` no MacOS
- `FOLDERID_RoamingAppData\termscp\` no Windows

Para os favoritos apenas (isso não se aplica aos hosts recentes), também é possível salvar a senha usada para autenticar. A senha não é salva por padrão e deve ser especificada no prompt ao salvar um novo favorito.
Se você estiver preocupado com a segurança da senha salva para seus favoritos, por favor, leia o [capítulo abaixo 👀](#minhas-senhas-são-seguras-).

Para criar um novo favorito, siga estas etapas:

1. Digite no formulário de autenticação os parâmetros para se conectar ao seu servidor remoto
2. Pressione `<CTRL+S>`
3. Digite o nome que deseja dar ao favorito
4. Escolha se deseja lembrar da senha ou não
5. Pressione `<ENTER>` para enviar

Sempre que quiser usar a conexão salva anteriormente, basta pressionar `<TAB>` para navegar para a lista de favoritos e carregar os parâmetros do favorito no formulário pressionando `<ENTER>`.

![Favoritos](https://github.com/veeso/termscp/blob/main/assets/images/bookmarks.gif?raw=true)

### Minhas Senhas São Seguras? 😈

Claro 😉.
Como já mencionado, os favoritos são salvos no diretório de configuração juntamente com as senhas. As senhas, obviamente, não são texto simples, elas são criptografadas com **AES-128**. Isso as torna seguras? Absolutamente! (exceto para usuários de BSD e WSL 😢)

No **Windows**, **Linux** e **MacOS**, a chave usada para criptografar senhas é armazenada, se possível (e deve ser), respectivamente no *Windows Vault*, no *sistema keyring* e no *Keychain*. Isso é realmente super seguro e é gerenciado diretamente pelo seu sistema operacional.

❗ Por favor, note que se você é um usuário de Linux, seria melhor ler o [capítulo abaixo 👀](#keyring-do-linux), porque o keyring pode não estar habilitado ou suportado no seu sistema!

Por outro lado, no *BSD* e no *WSL*, a chave usada para criptografar suas senhas é armazenada em seu disco (em `$HOME/.config/termscp`). Ainda é possível recuperar a chave para descriptografar as senhas. Felizmente, a localização da chave garante que ela não possa ser lida por outros usuários diferentes de você, mas sim, eu ainda não salvaria a senha para um servidor exposto na internet 😉.

#### Keyring do Linux

Todos nós amamos o Linux por causa da liberdade que ele oferece aos usuários. Você pode basicamente fazer o que quiser como usuário de Linux, mas isso também tem alguns contras, como o fato de muitas vezes não haver aplicativos padrão em diferentes distribuições. E isso também envolve o keyring.
Isso significa que no Linux pode não haver um keyring instalado no seu sistema. Infelizmente, a biblioteca que usamos para trabalhar com o armazenamento de chaves requer um serviço que expõe `org.freedesktop.secrets` no D-BUS, e o pior é que há apenas dois serviços que o expõem.

- ❗ Se você usa GNOME como ambiente de desktop (por exemplo, usuários do Ubuntu), já deve estar bem, pois o keyring já é fornecido pelo `gnome-keyring` e tudo deve estar funcionando.
- ❗ Para usuários de outros ambientes de desktop, há um programa legal que você pode usar para obter um keyring, que é o [KeepassXC](https://keepassxc.org/), que eu uso na minha instalação Manjaro (com KDE) e funciona bem. O único problema é que você precisa configurá-lo para ser usado junto com o termscp (mas é bastante simples). Para começar com KeepassXC, leia mais [aqui](#configuração-do-keepassxc-para-o-termscp).
- ❗ E se você não quiser instalar nenhum desses serviços? Bem, não tem problema! **termscp continuará funcionando normalmente**, mas salvará a chave em um arquivo, como normalmente faz para BSD e WSL.

##### Configuração do KeepassXC para o termscp

Siga estas etapas para configurar o KeepassXC para o termscp:

1. Instale o KeepassXC
2. Vá para "ferramentas" > "configurações" na barra de ferramentas
3. Selecione "Integração do Serviço Secreto" e ative "Habilitar Integração do Serviço Secreto do KeepassXC"
4. Crie um banco de dados, se você ainda não tiver um: na barra de ferramentas "Banco de dados" > "Novo banco de dados"
5. Na barra de ferramentas: "Banco de dados" > "Configurações do banco de dados"
6. Selecione "Integração do Serviço Secreto" e ative "Expor entradas sob este grupo"
7. Selecione o grupo na lista onde deseja que o segredo do termscp seja mantido. Lembre-se de que esse grupo pode ser usado por qualquer outro aplicativo para armazenar segredos via DBUS.

---

## Configuração ⚙️

O termscp suporta alguns parâmetros definidos pelo usuário, que podem ser definidos na configuração.
Por baixo dos panos, o termscp tem um arquivo TOML e alguns outros diretórios onde todos os parâmetros serão salvos, mas não se preocupe, você não precisará tocar em nenhum desses arquivos manualmente, pois fiz com que fosse possível configurar o termscp completamente a partir de sua interface de usuário.

Assim como para os favoritos, o termscp só requer que esses caminhos estejam acessíveis:

- `$HOME/.config/termscp/` no Linux/BSD
- `$HOME/Library/Application Support/termscp` no MacOs
- `FOLDERID_RoamingAppData\termscp\` no Windows

Para acessar a configuração, basta pressionar `<CTRL+C>` a partir da tela inicial do termscp.

Estes parâmetros podem ser alterados:

- **Editor de Texto**: o editor de texto a ser usado. Por padrão, o termscp encontrará o editor padrão para você; com essa opção, você pode forçar um editor a ser usado (por exemplo, `vim`). **Também são suportados editores GUI**, a menos que eles `nohup` do processo pai.
- **Protocolo Padrão**: o protocolo padrão é o valor padrão para o protocolo de transferência de arquivos a ser usado no termscp. Isso se aplica à página de login e ao argumento CLI do endereço.
- **Exibir Arquivos Ocultos**: selecione se os arquivos ocultos devem ser exibidos por padrão. Você ainda poderá decidir se deseja exibir ou não arquivos ocultos em tempo de execução pressionando `A`.
- **Verificar atualizações**: se definido como `yes`, o termscp buscará a API do Github para verificar se há uma nova versão do termscp disponível.
- **Prompt ao substituir arquivos existentes?**: Se definido como `yes`, o termscp pedirá confirmação sempre que uma transferência de arquivos causaria a substituição de um arquivo existente no host de destino.
- **Agrupar Diretórios**: selecione se os diretórios devem ser agrupados ou não nos exploradores de arquivos. Se `Display first` for selecionado, os diretórios serão ordenados usando o método configurado, mas exibidos antes dos arquivos; se `Display last` for selecionado, eles serão exibidos depois.
- **Sintaxe do formatador de arquivos remotos**: sintaxe para exibir informações de arquivo para cada arquivo no explorador remoto. Veja [Formato do Explorador de Arquivos](#formato-do-explorador-de-arquivos)
- **Sintaxe do formatador de arquivos locais**: sintaxe para exibir informações de arquivo para cada arquivo no explorador local. Veja [Formato do Explorador de Arquivos](#formato-do-explorador-de-arquivos)
- **Habilitar notificações?**: Se definido como `Yes`, as notificações serão exibidas.
- **Notificações: tamanho mínimo para transferência**: se o tamanho da transferência for maior ou igual ao valor especificado, as notificações para a transferência serão exibidas. Os valores aceitos estão no formato `{UNSIGNED} B/KB/MB/GB/TB/PB`.
- **Caminho da configuração SSH**: define o arquivo de configuração SSH a ser usado ao se conectar a um servidor SCP/SFTP. Se não definido (vazio), nenhum arquivo será usado. Você pode especificar um caminho começando com `~` para indicar o caminho inicial (por exemplo, `~/.ssh/config`). Os parâmetros suportados pelo termscp estão especificados [AQUI](https://github.com/veeso/ssh2-config#exposed-attributes).

### Armazenamento de Chave SSH 🔐

Além da configuração, o termscp também oferece um recurso **essencial** para **clientes SFTP/SCP**: o armazenamento de chave SSH.

Você pode acessar o armazenamento de chaves SSH na configuração, indo para a aba `Chaves SSH`. Uma vez lá, você pode:

- **Adicionar uma nova chave**: basta pressionar `<CTRL+N>` e você será solicitado a criar uma nova chave. Forneça o nome do host/endereço IP e o nome de usuário associado à chave e, finalmente, um editor de texto será aberto: cole a **chave SSH PRIVADA** no editor de texto, salve e saia.
- **Remover uma chave existente**: apenas pressione `<DEL>` ou `<CTRL+E>` na chave que você deseja remover para deletar a chave do termscp permanentemente.
- **Editar uma chave existente**: basta pressionar `<ENTER>` na chave que você deseja editar para alterar a chave privada.

> Pergunta: Espere, minha chave privada está protegida com senha, posso usá-la?  
> Resposta: Claro que sim. A senha fornecida para autenticação no termscp é válida tanto para autenticação por nome de usuário/senha quanto para autenticação por chave RSA.

### Formato do Explorador de Arquivos

É possível, através da configuração, definir um formato personalizado para o explorador de arquivos. Isso é possível tanto para o host local quanto para o remoto, para que você possa ter duas sintaxes diferentes em uso. Esses campos, com nome `File formatter syntax (local)` e `File formatter syntax (remote)`, definirão como as entradas de arquivos serão exibidas no explorador de arquivos.
A sintaxe para o formatador é a seguinte `{KEY1}... {KEY2:LENGTH}... {KEY3:LENGTH:EXTRA} {KEYn}...`.
Cada chave entre colchetes será substituída pelo atributo relacionado, enquanto tudo fora dos colchetes permanecerá inalterado.

- O nome da chave é obrigatório e deve ser uma das chaves abaixo.
- O comprimento descreve o espaço reservado para exibir o campo. Atributos estáticos não suportam esse recurso (GRUPO, PEX, TAMANHO, USUÁRIO).
- O Extra é suportado apenas por alguns parâmetros e é uma opção adicional. Veja as chaves para verificar se o extra é suportado.

Estas são as chaves suportadas pelo formatador:

- `ATIME`: Última vez de acesso (com sintaxe padrão `%b %d %Y %H:%M`); O Extra pode ser fornecido como a sintaxe de tempo (por exemplo, `{ATIME:8:%H:%M}`).
- `CTIME`: Tempo de criação (com sintaxe `%b %d %Y %H:%M`); O Extra pode ser fornecido como a sintaxe de tempo (por exemplo, `{CTIME:8:%H:%M}`).
- `GROUP`: Grupo do proprietário.
- `MTIME`: Última modificação (com sintaxe `%b %d %Y %H:%M`); O Extra pode ser fornecido como a sintaxe de tempo (por exemplo, `{MTIME:8:%H:%M}`).
- `NAME`: Nome do arquivo (pastas entre a raiz e os primeiros ancestrais são omitidas se forem maiores que o comprimento).
- `PATH`: Caminho absoluto do arquivo (pastas entre a raiz e os primeiros ancestrais são omitidas se forem maiores que o comprimento).
- `PEX`: Permissões do arquivo (formato UNIX).
- `SIZE`: Tamanho do arquivo (omitido para diretórios).
- `SYMLINK`: Link simbólico (se houver `-> {FILE_PATH}`).
- `USER`: Nome do proprietário.

Se deixado vazio, será usada a sintaxe padrão do formatador: `{NAME:24} {PEX} {USER} {SIZE} {MTIME:17:%b %d %Y %H:%M}`.

---

## Temas 🎨

O termscp oferece a você um recurso incrível: a possibilidade de definir as cores para vários componentes no aplicativo.
Se você deseja personalizar o termscp, há duas maneiras disponíveis para fazer isso:

- A partir do **menu de configuração**
- Importando um **arquivo de tema**

Para criar sua própria personalização no termscp, tudo o que você precisa fazer é entrar na configuração a partir da atividade de autenticação, pressionar `<CTRL+C>` e depois `<TAB>` duas vezes. Agora você deve ter se movido para o painel de `themes`.

Aqui você pode se mover com `<UP>` e `<DOWN>` para alterar o estilo que deseja alterar, como mostrado no gif abaixo:

![Temas](https://github.com/veeso/termscp/blob/main/assets/images/themes.gif?raw=true)

O termscp suporta tanto a sintaxe tradicional de hexadecimal explícito (`#rrggbb`) quanto rgb `rgb(r, g, b)` para fornecer cores, mas também **[cores CSS](https://www.w3schools.com/cssref/css_colors.asp)** (como `crimson`) são aceitas 😉. Há também uma palavra-chave especial, que é `Default`. Default significa que a cor usada será a cor padrão de primeiro plano ou plano de fundo, dependendo da situação (primeiro plano para textos e linhas, plano de fundo para, bem, adivinhe).

Como mencionado antes, você também pode importar arquivos de temas. Você pode se inspirar ou usar diretamente um dos temas fornecidos junto com o termscp, localizado no diretório `themes/` deste repositório, e importá-los executando o termscp como `termscp -t <arquivo-do-tema>`. Se tudo correu bem, ele deve informar que o tema foi importado com sucesso.

### Meu Tema Não Carrega 😱

Isso provavelmente se deve a uma atualização recente que quebrou o tema. Sempre que eu adiciono uma nova chave aos temas, o tema salvo não será carregado. Para corrigir esse problema, existem duas soluções rápidas:

1. Recarregar o tema: sempre que eu lançar uma atualização, também corrigirei os "temas oficiais", então você só precisará baixá-lo novamente do repositório e reimportar o tema usando a opção `-t`.

    ```sh
    termscp -t <theme.toml>
    ```

2. Corrigir seu tema: se você estiver usando um tema personalizado, você pode editá-lo via `vim` e adicionar a chave que está faltando. O tema está localizado em `$CONFIG_DIR/termscp/theme.toml`, onde `$CONFIG_DIR` é:

    - FreeBSD/GNU-Linux: `$HOME/.config/`
    - MacOs: `$HOME/Library/Application Support`
    - Windows: `%appdata%`

    ❗ As chaves que faltam são relatadas no CHANGELOG sob `BREAKING CHANGES` para a versão que você acabou de instalar.

### Estilos 💈

Você pode encontrar na tabela abaixo a descrição para cada campo de estilo.
Por favor, note que **estilos não se aplicam à página de configuração**, para torná-la sempre acessível no caso de você bagunçar tudo.

#### Página de Autenticação

| Chave          | Descrição                                  |
|-----------------|----------------------------------------------|
| auth_address     | Cor do campo de entrada para endereço IP       |
| auth_bookmarks   | Cor do painel de favoritos                      |
| auth_password   | Cor do campo de entrada para senha             |
| auth_port            | Cor do campo de entrada para número da porta   |
| auth_protocol      | Cor do grupo de rádio para protocolo            |
| auth_recents        | Cor do painel de recentes                        |
| auth_username   | Cor do campo de entrada para nome de usuário    |

#### Página de Transferência

| Chave                                  | Descrição                                                                       |
|--------------------------------------|---------------------------------------------------------------------------------|
| transfer_local_explorer_background   | Cor de fundo do explorador do localhost                                         |
| transfer_local_explorer_foreground   | Cor de primeiro plano do explorador do localhost                                |
| transfer_local_explorer_highlighted  | Cor da borda e realce do explorador do localhost                                |
| transfer_remote_explorer_background  | Cor de fundo do explorador remoto                                               |
| transfer_remote_explorer_foreground  | Cor de primeiro plano do explorador remoto                                       |
| transfer_remote_explorer_highlighted | Cor da borda e realce do explorador remoto                                       |
| transfer_log_background                | Cor de fundo do painel de logs                                                   |
| transfer_log_window                       | Cor da janela para o painel de logs                                               |
| transfer_progress_bar_partial           | Cor parcial da barra de progresso                                                 |
| transfer_progress_bar_total              | Cor total da barra de progresso                                                   |
| transfer_status_hidden                       | Cor para a etiqueta "oculto" na barra de status                                    |
| transfer_status_sorting                      | Cor para a etiqueta "ordenando" na barra de status; aplica-se também ao diálogo de ordenação de arquivos |
| transfer_status_sync_browsing             | Cor para a etiqueta "navegação sincronizada" na barra de status                 |

#### Diversos

Estes estilos se aplicam a diferentes partes do aplicativo.

| Chave                     | Descrição                                      |
|-----------------------------|------------------------------------------------|
| misc_error_dialog      | Cor para mensagens de erro                     |
| misc_info_dialog          | Cor para diálogos de informações                  |
| misc_input_dialog        | Cor para diálogos de entrada (como copiar arquivo) |
| misc_keys                    | Cor do texto para teclas de atalho                    |
| misc_quit_dialog         | Cor para diálogos de saída                       |
| misc_save_dialog        | Cor para diálogos de salvar                         |
| misc_warn_dialog       | Cor para diálogos de aviso                          |

---

## Editor de Texto ✏

O termscp possui, como você deve ter notado, muitos recursos, um deles é a possibilidade de visualizar e editar arquivos de texto. Não importa se o arquivo está localizado no host local ou no host remoto, o termscp oferece a possibilidade de abrir um arquivo no seu editor de texto favorito.
Caso o arquivo esteja localizado no host remoto, ele será primeiro baixado para o seu diretório temporário e, **somente** se alterações forem feitas no arquivo, ele será re-enviado para o host remoto. O termscp verifica se você fez alterações no arquivo verificando o último tempo de modificação do arquivo.

> ❗ Apenas um lembrete: **você só pode editar arquivos de texto**; arquivos binários não são suportados.

---

## Registro de Logs 🩺

O termscp escreve um arquivo de log para cada sessão, que é gravado em:

- `$HOME/.cache/termscp/termscp.log` no Linux/BSD
- `$HOME/Library/Caches/termscp/termscp.log` no MacOs
- `FOLDERID_LocalAppData\termscp\termscp.log` no Windows

o log não será rotacionado, mas será truncado após cada execução do termscp, então se você quiser relatar um problema e anexar seu arquivo de log, lembre-se de salvar o arquivo de log em um local seguro antes de usar o termscp novamente. O registro por padrão é feito no nível *INFO*, então não é muito detalhado.

Se você quiser enviar um problema, por favor, se puder, reproduza o problema com o nível definido como `TRACE`, para isso, inicie o termscp com a opção CLI `-D`.

Sei que você pode ter algumas perguntas sobre arquivos de log, então fiz um tipo de perguntas e respostas:

> Não quero registros, posso desativá-los?

Sim, você pode. Basta iniciar o termscp com a opção `-q ou --quiet`. Você pode aliasar o termscp para tornar isso persistente. Lembre-se de que os registros são usados para diagnosticar problemas, então, como atrás de todo projeto de código aberto deve sempre haver esse tipo de ajuda mútua, manter os arquivos de log pode ser sua maneira de apoiar o projeto 😉. Não quero que você se sinta culpado, mas só estou dizendo.

> O registro é seguro?

Se você estiver preocupado com a segurança, o arquivo de log não contém nenhuma senha em texto simples, então não se preocupe e expõe as mesmas informações que o arquivo irmão `bookmarks` relata.

## Notificações 📫

O termscp enviará notificações da área de trabalho para estes tipos de eventos:

- Em **Transferência concluída**: A notificação será enviada quando uma transferência for concluída com sucesso.
  - ❗ A notificação será exibida apenas se o tamanho total da transferência for pelo menos o especificado em `Notifications: minimum transfer size` na configuração.
- Em **Transferência falhou**: A notificação será enviada quando uma transferência falhar devido a um erro.
  - ❗ A notificação será exibida apenas se o tamanho total da transferência for pelo menos o especificado em `Notifications: minimum transfer size` na configuração.
- Em **Atualização disponível**: Sempre que uma nova versão do termscp estiver disponível, uma notificação será exibida.
- Em **Atualização instalada**: Sempre que uma nova versão do termscp for instalada, uma notificação será exibida.
- Em **Falha na atualização**: Sempre que a instalação da atualização falhar, uma notificação será exibida.

❗ Se você prefere manter as notificações desativadas, basta entrar na configuração e definir `Enable notifications?` para `No` 😉.  
❗ Se quiser alterar o tamanho mínimo para exibir notificações, você pode mudar o valor na configuração com a chave `Notifications: minimum transfer size` e ajustá-lo ao que for melhor para você 🙂.

---

## Observador de Arquivos 🔭

O observador de arquivos permite que você configure uma lista de caminhos para sincronizar com os hosts remotos.
Isso significa que, sempre que uma alteração no sistema de arquivos local for detectada no caminho sincronizado, a alteração será automaticamente relatada para o caminho do host remoto configurado, dentro de 5 segundos.

Você pode definir quantos caminhos desejar para sincronizar:

1. Coloque o cursor no explorador local no diretório/arquivo que deseja manter sincronizado.
2. Vá para o diretório para o qual deseja que as alterações sejam relatadas no host remoto.
3. Pressione `<T>`.
4. Responda `<YES>` na janela pop-up.

Para desfazer a observação, basta pressionar `<T>` no caminho local sincronizado (ou em qualquer um de seus subdiretórios)
OU você pode simplesmente pressionar `<CTRL+T>` e pressionar `<ENTER>` no caminho sincronizado que deseja desfazer a observação.

Estas alterações serão relatadas para o host remoto:

- Novos arquivos, alterações em arquivos.
- Arquivo movido/renomeado.
- Arquivo removido/desvinculado.

> ❗ O observador funciona apenas em uma direção (local > remoto). Não é possível sincronizar automaticamente as alterações do host remoto para o local.
