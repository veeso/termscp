# User manual 🎓

- [User manual 🎓](#user-manual-)
  - [Uso ❓](#uso-)
    - [Argumento dirección 🌎](#argumento-dirección-)
      - [Argumento dirección por AWS S3](#argumento-dirección-por-aws-s3)
      - [Cómo se puede proporcionar la contraseña 🔐](#cómo-se-puede-proporcionar-la-contraseña-)
  - [Credenciales de AWS S3 🦊](#credenciales-de-aws-s3-)
  - [Explorador de archivos 📂](#explorador-de-archivos-)
    - [Keybindings ⌨](#keybindings-)
    - [Trabaja en varios archivos 🥷](#trabaja-en-varios-archivos-)
    - [Navegación sincronizada ⏲️](#navegación-sincronizada-️)
    - [Abierta y abierta con 🚪](#abierta-y-abierta-con-)
  - [Marcadores ⭐](#marcadores-)
    - [¿Son seguras mis contraseñas? 😈](#son-seguras-mis-contraseñas-)
      - [Linux Keyring](#linux-keyring)
        - [KeepassXC setup por termscp](#keepassxc-setup-por-termscp)
  - [Configuración  ⚙️](#configuración--️)
    - [SSH Key Storage 🔐](#ssh-key-storage-)
    - [Formato del explorador de archivos](#formato-del-explorador-de-archivos)
  - [Temas 🎨](#temas-)
    - [Mi tema no se carga 😱](#mi-tema-no-se-carga-)
    - [Estilos 💈](#estilos-)
      - [Authentication page](#authentication-page)
      - [Transfer page](#transfer-page)
      - [Misc](#misc)
  - [Text Editor ✏](#text-editor-)
  - [Logging 🩺](#logging-)
  - [Notificaciones 📫](#notificaciones-)

> ❗ Este documento ha sido traducido con Google Translator (y luego lo he revisado a grandes rasgos, pero no puedo hablar el idioma muy bien). Si habla l'idioma, abra un [issue](https://github.com/veeso/termscp/issues/new/choose) utilizando la label COPY o abra un PR 🙏

## Uso ❓

termscp se puede iniciar con las siguientes opciones:

`termscp [options]... [protocol://user@address:port:wrkdir] [local-wrkdir]`

- `-P, --password <password>` si se proporciona la dirección, la contraseña será este argumento
- `-c, --config` Abrir termscp comenzando desde la página de configuración
- `-q, --quiet` Deshabilitar el registro
- `-t, --theme <path>` Importar tema especificado
- `-u, --update` Actualizar termscp a la última versión
- `-v, --version` Imprimir información de la versión
- `-h, --help` Imprimir página de ayuda

termscp se puede iniciar en dos modos diferentes, si no se proporcionan argumentos adicionales, termscp mostrará el formulario de autenticación, donde el usuario podrá proporcionar los parámetros necesarios para conectarse al par remoto.

Alternativamente, el usuario puede proporcionar una dirección como argumento para omitir el formulario de autenticación e iniciar directamente la conexión al servidor remoto.

Si se proporciona un argumento de dirección, también puede proporcionar el directorio de inicio de trabajo para el host local

### Argumento dirección 🌎

El argumento dirección tiene la siguiente sintaxis:

```txt
[protocol://][username@]<address>[:port][:wrkdir]
```

Veamos algún ejemplo de esta sintaxis en particular, ya que es muy cómoda y probablemente usarás esta en lugar de la otra ...

- Conéctese usando el protocolo predeterminado (*definido en la configuración*) a 192.168.1.31, el puerto, si no se proporciona, es el predeterminado para el protocolo seleccionado (en este caso, depende de su configuración); nombre de usuario es el nombre del usuario actual

    ```sh
    termscp 192.168.1.31
    ```

- Conéctese usando el protocolo predeterminado (*definido en la configuración*) a 192.168.1.31; el nombre de usuario es `root`

    ```sh
    termscp root@192.168.1.31
    ```

- Conéctese usando scp a 192.168.1.31, el puerto es 4022; nombre de usuario es `omar`

    ```sh
    termscp scp://omar@192.168.1.31:4022
    ```

- Conéctese usando scp a 192.168.1.31, el puerto es 4022; El nombre de usuario es `omar`. Comenzará en el directorio `/ tmp`

    ```sh
    termscp scp://omar@192.168.1.31:4022:/tmp
    ```

#### Argumento dirección por AWS S3

Aws S3 tiene una sintaxis diferente para el argumento de la dirección CLI, por razones obvias, pero logré mantenerlo lo más similar posible al argumento de la dirección genérica:

```txt
s3://<bucket-name>@<region>[:profile][:/wrkdir]
```

por ejemplo

```txt
s3://buckethead@eu-central-1:default:/assets
```

#### Cómo se puede proporcionar la contraseña 🔐

Probablemente haya notado que, al proporcionar la dirección como argumento, no hay forma de proporcionar la contraseña.
La contraseña se puede proporcionar básicamente a través de 3 formas cuando se proporciona un argumento de dirección:

- `-P, --password` opción: simplemente use esta opción CLI proporcionando la contraseña. No recomiendo este método, ya que es muy inseguro (ya que puede mantener la contraseña en el historial de shell)
- Con `sshpass`: puede proporcionar la contraseña a través de `sshpass`, p. ej. `sshpass -f ~/.ssh/topsecret.key termscp cvisintin@192.168.1.31`
- Se te pedirá que ingreses: si no utilizas ninguno de los métodos anteriores, se te pedirá la contraseña, como ocurre con las herramientas más clásicas como `scp`, `ssh`, etc.

---

## Credenciales de AWS S3 🦊

Para conectarse a un bucket de Aws S3, obviamente debe proporcionar algunas credenciales.
Básicamente, hay dos formas de lograr esto, y como probablemente ya hayas notado, **no puedes** hacerlo a través del formulario de autenticación.
Entonces, estas son las formas en que puede proporcionar las credenciales para s3:

1. Use su archivo de credenciales: simplemente configure la cli de AWS a través de `aws configure` y sus credenciales ya deberían estar ubicadas en`~/.aws/credentials`. En caso de que esté usando un perfil diferente al "predeterminado", simplemente proporciónelo en el campo de perfil en el formulario de autenticación.
2. **Variables de entorno**: siempre puede proporcionar sus credenciales como variables de entorno. Tenga en cuenta que estas credenciales **siempre anularán** las credenciales ubicadas en el archivo `credentials`. Vea cómo configurar el entorno a continuación:

    Estos siempre deben ser obligatorios:

    - `AWS_ACCESS_KEY_ID`: aws access key ID (generalmente comienza con `AKIA...`)
    - `AWS_SECRET_ACCESS_KEY`: la secret access key

    En caso de que haya configurado una seguridad más fuerte, *puede* requerir estos también:

    - `AWS_SECURITY_TOKEN`: security token
    - `AWS_SESSION_TOKEN`: session token

⚠️ Sus credenciales están seguras: ¡termscp no manipulará estos valores directamente! Sus credenciales son consumidas directamente por la caja **s3**.
En caso de que tenga alguna inquietud con respecto a la seguridad, comuníquese con el autor de la biblioteca en [Github](https://github.com/durch/rust-s3) ⚠️

---

## Explorador de archivos 📂

Cuando nos referimos a exploradores de archivos en termscp, nos referimos a los paneles que puede ver después de establecer una conexión con el control remoto.
Estos paneles son básicamente 3:

- Panel del explorador local: se muestra a la izquierda de la pantalla y muestra las entradas del directorio actual para localhost
- Panel del explorador remoto: se muestra a la derecha de la pantalla y muestra las entradas del directorio actual para el host remoto.
- Panel de resultados de búsqueda: dependiendo de dónde esté buscando archivos (local / remoto), reemplazará el panel local o del explorador. Este panel muestra las entradas que coinciden con la consulta de búsqueda que realizó.

Para cambiar de panel, debe escribir `<LEFT>` para mover el panel del explorador remoto y `<RIGHT>` para volver al panel del explorador local. Siempre que se encuentre en el panel de resultados de búsqueda, debe presionar `<ESC>` para salir del panel y volver al panel anterior.

### Keybindings ⌨

| Key           | Command                                                                    | Reminder    |
|---------------|----------------------------------------------------------------------------|-------------|
| `<ESC>`       | Desconecte; volver a la página de autenticación                            |             |
| `<BACKSPACE>` | Ir al directorio anterior en la pila                                       |             |
| `<TAB>`       | Cambiar pestaña del explorador                                             |             |
| `<RIGHT>`     | Mover a la pestaña del explorador remoto                                   |             |
| `<LEFT>`      | Mover a la pestaña del explorador local                                    |             |
| `<UP>`        | Subir en la lista seleccionada                                             |             |
| `<DOWN>`      | Bajar en la lista seleccionada                                             |             |
| `<PGUP>`      | Subir 8 filas en la lista seleccionada                                     |             |
| `<PGDOWN>`    | Bajar 8 filas en la lista seleccionada                                     |             |
| `<ENTER>`     | Entrar directorio                                                          |             |
| `<SPACE>`     | Cargar / descargar el archivo seleccionado                                 |             |
| `<BACKTAB>`   | Cambiar entre la pestaña de registro y el explorador                       |             |
| `<A>`         | Alternar archivos ocultos                                                  | All         |
| `<B>`         | Ordenar archivos por                                                       | Bubblesort? |
| `<C|F5>`      | Copiar archivo / directorio                                                | Copy        |
| `<D|F7>`      | Hacer directorio                                                           | Directory   |
| `<E|F8|DEL>`  | Eliminar archivo                                                           | Erase       |
| `<F>`         | Búsqueda de archivos                                                       | Find        |
| `<G>`         | Ir a la ruta proporcionada                                                 | Go to       |
| `<H|F1>`      | Mostrar ayuda                                                              | Help        |
| `<I>`         | Mostrar información sobre el archivo                                       | Info        |
| `<K>`         | Crear un enlace simbólico que apunte a la entrada seleccionada actualmente | symlinK     |
| `<L>`         | Recargar contenido del directorio / Borrar selección                       | List        |
| `<M>`         | Seleccione un archivo                                                      | Mark        |
| `<N>`         | Crear un nuevo archivo con el nombre proporcionado                         | New         |
| `<O|F4>`      | Editar archivo                                                             | Open        |
| `<Q|F10>`     | Salir de termscp                                                           | Quit        |
| `<R|F6>`      | Renombrar archivo                                                          | Rename      |
| `<S|F2>`      | Guardar archivo como...                                                    | Save        |
| `<U>`         | Ir al directorio principal                                                 | Upper       |
| `<V|F3>`      | Abrir archivo con el programa predeterminado                               | View        |
| `<W>`         | Abrir archivo con el programa proporcionado                                | With        |
| `<X>`         | Ejecutar un comando                                                        | eXecute     |
| `<Y>`         | Alternar navegación sincronizada                                           | sYnc        |
| `<CTRL+A>`    | Seleccionar todos los archivos                                             |             |
| `<CTRL+C>`    | Abortar el proceso de transferencia de archivos                            |             |

### Trabaja en varios archivos 🥷

Puede optar por trabajar en varios archivos, seleccionándolos presionando `<M>`, para seleccionar el archivo actual, o presionando `<CTRL + A>`, que seleccionará todos los archivos en el directorio de trabajo.
Una vez que un archivo está marcado para su selección, se mostrará con un `*` a la izquierda.
Al trabajar en la selección, solo se procesará el archivo seleccionado para las acciones, mientras que el elemento resaltado actual se ignorará.
También es posible trabajar en varios archivos desde el panel de resultados de búsqueda.
Todas las acciones están disponibles cuando se trabaja con varios archivos, pero tenga en cuenta que algunas acciones funcionan de forma ligeramente diferente. Vamos a sumergirnos en:

- *Copy*: cada vez que copie un archivo, se le pedirá que inserte el nombre de destino. Cuando se trabaja con varios archivos, este nombre se refiere al directorio de destino donde se copiarán todos estos archivos.
- *Rename*: igual que copiar, pero moverá archivos allí.
- *Save as*: igual que copiar, pero los escribirá allí.

### Navegación sincronizada ⏲️

Cuando está habilitada, la navegación sincronizada le permitirá sincronizar la navegación entre los dos paneles.
Esto significa que siempre que cambie el directorio de trabajo en un panel, la misma acción se reproducirá en el otro panel. Si desea habilitar la navegación sincronizada, simplemente presione `<Y>`; presione dos veces para deshabilitar. Mientras está habilitado, el estado de navegación sincronizada se informará en la barra de estado en "ON".

### Abierta y abierta con 🚪

Al abrir archivos con el comando Ver (`<V>`), se utilizará la aplicación predeterminada del sistema para el tipo de archivo. Para hacerlo, se utilizará el servicio del sistema operativo predeterminado, así que asegúrese de tener al menos uno de estos instalado en su sistema:

- Usuarios **Windows**: no tiene que preocuparse por eso, ya que la caja usará el comando `start`.
- Usuarios **MacOS**: tampoco tiene que preocuparse, ya que la caja usará `open`, que ya está instalado en su sistema.
- Usuarios **Linux**: uno de estos debe estar instalado
  - *xdg-open*
  - *gio*
  - *gnome-open*
  - *kde-open*
- Usuarios **WSL**: *wslview* es obligatorio, debe instalar [wslu](https://github.com/wslutilities/wslu).

> Q: ¿Puedo editar archivos remotos usando el comando de vista?  
> A: No, al menos no directamente desde el "panel remoto". Primero debe descargarlo en un directorio local, eso se debe al hecho de que cuando abre un archivo remoto, el archivo se descarga en un directorio temporal, pero no hay forma de crear un observador para que el archivo verifique cuándo el programa utilizado para abrirlo estaba cerrado, por lo que termscp no puede saber cuándo ha terminado de editar el archivo.

---

## Marcadores ⭐

En termscp es posible guardar hosts favoritos, que luego se pueden cargar rápidamente desde el diseño principal de termscp.
termscp también guardará los últimos 16 hosts a los que se conectó.
Esta función le permite cargar todos los parámetros necesarios para conectarse a un determinado control remoto, simplemente seleccionando el marcador en la pestaña debajo del formulario de autenticación.

Los marcadores se guardarán, si es posible, en:

- `$HOME/.config/termscp/` en Linux/BSD
- `$HOME/Library/Application Support/termscp` en MacOs
- `FOLDERID_RoamingAppData\termscp\` en Windows

Solo para marcadores (esto no se aplicará a hosts recientes) también es posible guardar la contraseña utilizada para autenticarse. La contraseña no se guarda de forma predeterminada y debe especificarse a través del indicador al guardar un nuevo marcador.
Si le preocupa la seguridad de la contraseña guardada para sus marcadores, lea el [capítulo siguiente 👀](#are-my-passwords-safe-).

Para crear un nuevo marcador, simplemente siga estos pasos:

1. Escriba en el formulario de autenticación los parámetros para conectarse a su servidor remoto
2. Presionar `<CTRL + S>`
3. Escriba el nombre que desea darle al marcador
4. Elija si recordar la contraseña o no
5. Presionar `<ENTER>`

siempre que desee utilizar la conexión previamente guardada, simplemente presione `<TAB>` para navegar a la lista de marcadores y cargue los parámetros del marcador en el formulario presionando `<ENTER>`.

![Bookmarks](https://github.com/veeso/termscp/blob/main/assets/images/bookmarks.gif?raw=true)

### ¿Son seguras mis contraseñas? 😈

Seguro 😉.
Como se dijo antes, los marcadores se guardan en su directorio de configuración junto con las contraseñas. Las contraseñas obviamente no son texto sin formato, están encriptadas con **AES-128**. ¿Esto los hace seguros? ¡Absolutamente! (excepto para usuarios de BSD y WSL 😢)

En **Windows**, **Linux** y **MacOS**, la clave utilizada para cifrar las contraseñas se almacena, si es posible (pero debería estar), respectivamente, en *Windows Vault*, en el *anillo de claves del sistema* y en el *Llavero*. Esto es realmente muy seguro y lo administra directamente su sistema operativo.

❗ Por favor, tenga en cuenta que si es un usuario de Linux, es mejor que lea el [capítulo siguiente 👀](#linux-keyring), ¡porque es posible que el llavero no esté habilitado o no sea compatible con su sistema!

En *BSD* y *WSL*, por otro lado, la clave utilizada para cifrar sus contraseñas se almacena en su disco (en `$HOME/.config/ termscp`). Entonces, todavía es posible recuperar la clave para descifrar las contraseñas. Afortunadamente, la ubicación de la clave garantiza que su clave no pueda ser leída por usuarios diferentes al suyo, pero sí, todavía no guardaría la contraseña para un servidor expuesto en Internet 😉.

#### Linux Keyring

A todos nos encanta Linux gracias a la libertad que ofrece a los usuarios. Básicamente, puede hacer lo que quiera como usuario de Linux, pero esto también tiene algunas desventajas, como el hecho de que a menudo no hay aplicaciones estándar en las diferentes distribuciones. Y esto también implica el llavero.
Esto significa que en Linux puede que no haya un llavero instalado en su sistema. Desafortunadamente, la biblioteca que usamos para trabajar con el almacenamiento de claves requiere un servicio que exponga `org.freedesktop.secrets` en D-BUS y el peor hecho es que solo hay dos servicios que lo exponen.

- ❗ Si usa GNOME como entorno de escritorio (por ejemplo, usuarios de ubuntu), ya debería estar bien, ya que `gnome-keyring` ya proporciona el llavero y todo debería estar funcionando.
- ❗ Para otros usuarios de entornos de escritorio, hay un buen programa que pueden usar para obtener un llavero que es [KeepassXC](https://keepassxc.org/), que utilizo en mi instalación de Manjaro (con KDE) y funciona bien. El único problema es que debe configurarlo para que se use junto con termscp (pero es bastante simple). Para comenzar con KeepassXC, lea más [aquí](#keepassxc-setup-por-termscp).
- ❗ ¿Qué pasa si no desea instalar ninguno de estos servicios? Bueno, ¡no hay problema! **termscp seguirá funcionando como de costumbre**, pero guardará la clave en un archivo, como suele hacer para BSD y WSL.

##### KeepassXC setup por termscp

Siga estos pasos para configurar keepassXC para termscp:

1. Instalar KeepassXC
2. Vaya a "tools" > "settings" en la barra de herramientas
3. Seleccione "Secret service integration" y abilita "Enable KeepassXC freedesktop.org secret service integration"
4. Cree una base de datos, si aún no tiene una: desde la barra de herramientas "Database" > "New database"
5. Desde la barra de herramientas: "Database" > "Database settings"
6. Seleccione "Secret service integration" y abilita "Expose entries under this group"
7. Seleccione el grupo de la lista donde desea que se mantenga el secreto de termscp. Recuerde que este grupo puede ser utilizado por cualquier otra aplicación para almacenar secretos a través de DBUS.

---

## Configuración  ⚙️

termscp admite algunos parámetros definidos por el usuario, que se pueden definir en la configuración.
Underhood termscp tiene un archivo TOML y algunos otros directorios donde se guardarán todos los parámetros, pero no se preocupe, no tocará ninguno de estos archivos manualmente, ya que hice posible configurar termscp desde su interfaz de usuario por completo.

termscp, al igual que para los marcadores, solo requiere tener estas rutas accesibles:

- `$HOME/.config/termscp/` en Linux/BSD
- `$HOME/Library/Application Support/termscp` en MacOs
- `FOLDERID_RoamingAppData\termscp\` en Windows

Para acceder a la configuración, solo tiene que presionar `<CTRL + C>` desde el inicio de termscp.

Estos parámetros se pueden cambiar:

- **Text Editor**: l editor de texto a utilizar. Por defecto, termscp encontrará el editor predeterminado para usted; con esta opción puede forzar el uso de un editor (por ejemplo, `vim`). **También se admiten los editores de GUI**, a menos que hagan "nohup" del proceso principal.
- **Default Protocol**: el protocolo predeterminado es el valor predeterminado para el protocolo de transferencia de archivos que se utilizará en termscp. Esto se aplica a la página de inicio de sesión y al argumento de la CLI de la dirección.
- **Show Hidden Files**: seleccione si los archivos ocultos se mostrarán de forma predeterminada. Podrás decidir si mostrar o no archivos ocultos en tiempo de ejecución presionando "A" de todos modos.
- **Check for updates**: si se establece en `yes`, termscp buscará la API de Github para comprobar si hay una nueva versión de termscp disponible.
- **Prompt when replacing existing files?**: Si se establece en "sí", termscp le pedirá confirmación cada vez que una transferencia de archivo provoque la sustitución de un archivo existente en el host de destino.
- **Group Dirs**: seleccione si los directorios deben agruparse o no en los exploradores de archivos. Si se selecciona `Display first`, los directorios se ordenarán usando el método configurado pero se mostrarán antes de los archivos, y viceversa si se selecciona`Display last`.
- **Remote File formatter syntax**: sintaxis para mostrar información de archivo para cada archivo en el explorador remoto. Consulte [Formato del explorador de archivos](#formato-del-explorador-de-archivos).
- **Local File formatter syntax**: sintaxis para mostrar información de archivo para cada archivo en el explorador local. Consulte [Formato del explorador de archivos](#formato-del-explorador-de-archivos).
- **Enable notifications?**: Si se establece en "Sí", se mostrarán las notificaciones.
- **Notifications: minimum transfer size**: si el tamaño de la transferencia es mayor o igual que el valor especificado, se mostrarán notificaciones de transferencia. Los valores aceptados están en formato `{UNSIGNED} B/KB/MB/GB/TB/PB`
- **SSH configuration path**: Configure el archivo de configuración SSH para usar al conectarse a un servidor SCP / SFTP. Si no se configura (está vacío), no se utilizará ningún archivo. Puede especificar una ruta que comience con `~` para indicar la ruta de inicio (por ejemplo, `~/.ssh/config`)

### SSH Key Storage 🔐

Junto con la configuración, termscp también proporciona una característica **esencial** para **clientes SFTP / SCP**: el almacenamiento de claves SSH.

Puede acceder al almacenamiento de claves SSH, desde la configuración pasando a la pestaña `Claves SSH`, una vez allí puede:

- **Agregar una nueva clave**: simplemente presione `<CTRL + N>` y se le pedirá que cree una nueva clave. Proporcione el nombre de host / dirección IP y el nombre de usuario asociado a la clave y finalmente se abrirá un editor de texto: pegue la clave ssh **PRIVATE** en el editor de texto, guarde y salga.
- **Eliminar una clave existente**: simplemente presione `<DEL>` o `<CTRL + E>` en la clave que desea eliminar, para eliminar persistentemente la clave de termscp.
- **Editar una clave existente**: simplemente presione `<ENTER>` en la clave que desea editar, para cambiar la clave privada.

> Q: Mi clave privada está protegida con contraseña, ¿puedo usarla?
> A: Por supuesto que puede. La contraseña proporcionada para la autenticación en termscp es válida tanto para la autenticación de nombre de usuario / contraseña como para la autenticación de clave RSA.

### Formato del explorador de archivos

Es posible a través de la configuración definir un formato personalizado para el explorador de archivos. Esto es posible tanto para el host local como para el remoto, por lo que puede tener dos sintaxis diferentes en uso. Estos campos, con el nombre `File formatter syntax (local)` y `File formatter syntax (remote)` definirán cómo se mostrarán las entradas del archivo en el explorador de archivos.
La sintaxis del formateador es la siguiente `{KEY1} ... {KEY2:LENGTH} ... {KEY3:LENGTH:EXTRA} {KEYn} ...`.
Cada clave entre corchetes se reemplazará con el atributo relacionado, mientras que todo lo que esté fuera de los corchetes se dejará sin cambios.

- El nombre de la clave es obligatorio y debe ser una de las claves siguientes
- La longitud describe la longitud reservada para mostrar el campo. Los atributos estáticos no admiten esto (GROUP, PEX, SIZE, USER)
- Extra es compatible solo con algunos parámetros y es una opción adicional. Consulte las claves para comprobar si se admite extra.

Estas son las claves admitidas por el formateador:

- `ATIME`: Hora del último acceso (con la sintaxis predeterminada`%b %d %Y %H:%M`); Se puede proporcionar un extra como la sintaxis de tiempo (por ejemplo, "{ATIME: 8:% H:% M}")
- `CTIME`: Hora de creación (con sintaxis`%b %d %Y %H:%M`); Se puede proporcionar un extra como sintaxis de tiempo (p. Ej., `{CTIME:8:%H:%M}`)
- `GROUP`: Grupo propietario
- `MTIME`: Hora del último cambio (con sintaxis`%b %d %Y %H:%M`); Se puede proporcionar extra como sintaxis de tiempo (p. Ej., `{MTIME: 8:% H:% M}`)
- `NAME`: nombre de archivo (Las carpetas entre la raíz y los primeros antepasados ​​se eliminan si es más largo que LENGTH)
- `PATH`: Percorso completo de archivo (Las carpetas entre la raíz y los primeros antepasados ​​se eliminan si es màs largo que LENGHT)
- `PEX`: permisos de archivo (formato UNIX)
- `SIZE`: Tamaño del archivo (se omite para directorios)
- `SYMLINK`: Symlink (si existe` -> {FILE_PATH} `)
- `USER`: Usuario propietario

Si se deja vacío, se utilizará la sintaxis del formateador predeterminada: `{NAME:24} {PEX} {USER} {SIZE} {MTIME:17:%b %d %Y %H:%M}`

---

## Temas 🎨

Termscp le ofrece una característica asombrosa: la posibilidad de configurar los colores para varios componentes de la aplicación.
Si desea personalizar termscp, hay dos formas disponibles de hacerlo:

- Desde el **menú de configuración**
- Importando un **archivo de tema**

Para crear su propia personalización desde termscp, todo lo que tiene que hacer es ingresar a la configuración desde la actividad de autenticación, presionando `<CTRL + C>` y luego `<TAB>` dos veces. Deberías haberte movido ahora al panel de `temas`.

Aquí puede moverse con `<UP>` y `<DOWN>` para cambiar el estilo que desea cambiar, como se muestra en el siguiente gif:

![Themes](https://github.com/veeso/termscp/blob/main/assets/images/themes.gif?raw=true)

termscp admite la sintaxis tradicional hexadecimal explícita (`#rrggbb`) y rgb `rgb(r, g, b)` para proporcionar colores, pero se aceptan también **[colores css](https://www.w3schools.com/cssref/css_colors.asp)** (como `crimson`) 😉. También hay un teclado especial que es `Default`. Predeterminado significa que el color utilizado será el color de primer plano o de fondo predeterminado según la situación (primer plano para textos y líneas, fondo para bien, adivinen qué).

Como se dijo antes, también puede importar archivos de temas. Puede inspirarse o utilizar directamente uno de los temas proporcionados junto con termscp, ubicado en el directorio `themes/` de este repositorio e importarlos ejecutando termscp como `termscp -t <theme_file>`. Si todo estuvo bien, debería decirle que el tema se ha importado correctamente.

### Mi tema no se carga 😱

Esto probablemente se deba a una actualización reciente que ha roto el tema. Siempre que agrego una nueva clave a los temas, el tema guardado no se carga. Para solucionar este problema, existen dos soluciones realmente rápidas:

1. Recargar tema: cada vez que publique una actualización, también parchearé los temas "oficiales", por lo que solo tiene que descargarlo del repositorio nuevamente y volver a importar el tema a través de la opción `-t`

    ```sh
    termscp -t <theme.toml>
    ```

2. Corrija su tema: si está utilizando un tema personalizado, puede editarlo a través de `vim` y agregar la clave que falta. El tema se encuentra en `$CONFIG_DIR/termscp/theme.toml` donde `$CONFIG_DIR` es:

    - FreeBSD/GNU-Linux: `$HOME/.config/`
    - MacOs: `$HOME/Library/Application Support`
    - Windows: `%appdata%`

    ❗ Las claves que faltan se informan en el CAMBIO en `BREAKING CHANGES` para la versión que acaba de instalar.

### Estilos 💈

Puede encontrar en la tabla siguiente la descripción de cada campo de estilo.
Tenga en cuenta que **los estilos no se aplicarán a la página de configuración**, para que sea siempre accesible en caso de que lo estropee todo

#### Authentication page

| Key            | Description                              |
|----------------|------------------------------------------|
| auth_address   | Color of the input field for IP address  |
| auth_bookmarks | Color of the bookmarks panel             |
| auth_password  | Color of the input field for password    |
| auth_port      | Color of the input field for port number |
| auth_protocol  | Color of the radio group for protocol    |
| auth_recents   | Color of the recents panel               |
| auth_username  | Color of the input field for username    |

#### Transfer page

| Key                                  | Description                                                               |
|--------------------------------------|---------------------------------------------------------------------------|
| transfer_local_explorer_background   | Background color of localhost explorer                                    |
| transfer_local_explorer_foreground   | Foreground coloor of localhost explorer                                   |
| transfer_local_explorer_highlighted  | Border and highlighted color for localhost explorer                       |
| transfer_remote_explorer_background  | Background color of remote explorer                                       |
| transfer_remote_explorer_foreground  | Foreground coloor of remote explorer                                      |
| transfer_remote_explorer_highlighted | Border and highlighted color for remote explorer                          |
| transfer_log_background              | Background color for log panel                                            |
| transfer_log_window                  | Window color for log panel                                                |
| transfer_progress_bar_partial        | Partial progress bar color                                                |
| transfer_progress_bar_total          | Total progress bar color                                                  |
| transfer_status_hidden               | Color for status bar "hidden" label                                       |
| transfer_status_sorting              | Color for status bar "sorting" label; applies also to file sorting dialog |
| transfer_status_sync_browsing        | Color for status bar "sync browsing" label                                |

#### Misc

These styles applie to different part of the application.

| Key               | Description                                 |
|-------------------|---------------------------------------------|
| misc_error_dialog | Color for error messages                    |
| misc_info_dialog  | Color for info dialogs                      |
| misc_input_dialog | Color for input dialogs (such as copy file) |
| misc_keys         | Color of text for key strokes               |
| misc_quit_dialog  | Color for quit dialogs                      |
| misc_save_dialog  | Color for save dialogs                      |
| misc_warn_dialog  | Color for warn dialogs                      |

---

## Text Editor ✏

termscp tiene, como habrás notado, muchas características, una de ellas es la posibilidad de ver y editar archivos de texto. No importa si el archivo está ubicado en el host local o en el host remoto, termscp brinda la posibilidad de abrir un archivo en su editor de texto favorito.
En caso de que el archivo esté ubicado en un host remoto, el archivo se descargará primero en su directorio de archivos temporales y luego, **solo** si se realizaron cambios en el archivo, se volverá a cargar en el host remoto. termscp comprueba si realizó cambios en el archivo verificando la última hora de modificación del archivo.

> ❗ Just a reminder: **you can edit only textual file**; binary files are not supported.

---

## Logging 🩺

termscp escribe un archivo de registro para cada sesión, que se escribe en

- `$HOME/.config/termscp/termscp.log` en Linux/BSD
- `$HOME/Library/Application Support/termscp/termscp.log` en MacOs
- `FOLDERID_RoamingAppData\termscp\termscp.log` en Windows

el registro no se rotará, sino que se truncará después de cada lanzamiento de termscp, por lo que si desea informar un problema y desea adjuntar su archivo de registro, recuerde guardar el archivo de registro en un lugar seguro antes de usar termscp de nuevo.
El registro por defecto informa en el nivel *INFO*, por lo que no es muy detallado.

Si desea enviar un problema, por favor, si puede, reproduzca el problema con el nivel establecido en "TRACE", para hacerlo, inicie termscp con
la opción CLI `-D`.

Sé que es posible que tenga algunas preguntas sobre los archivos de registro, así que hice una especie de Q/A:

> No quiero el registro, ¿puedo apagarlo?

Sí tu puedes. Simplemente inicie termscp con la opción `-q o --quiet`. Puede alias termscp para que sea persistente. Recuerde que el registro se usa para diagnosticar problemas, por lo que, dado que detrás de cada proyecto de código abierto, siempre debe haber este tipo de ayuda mutua, mantener los archivos de registro puede ser su forma de respaldar el proyecto 😉.

> ¿Es seguro el registro?

Si le preocupa la seguridad, el archivo de registro no contiene ninguna contraseña simple, así que no se preocupe y expone la misma información que informa el archivo hermano `marcadores`.

## Notificaciones 📫

Termscp enviará notificaciones de escritorio para este tipo de eventos:

- en **Transferencia completada**: la notificación se enviará una vez que la transferencia se haya completado con éxito.
  - ❗ La notificación se mostrará solo si el tamaño total de la transferencia es al menos el `Notifications: minimum transfer size` especificado en la configuración.
- en **Transferencia fallida**: la notificación se enviará una vez que la transferencia haya fallado debido a un error.
  - ❗ La notificación se mostrará solo si el tamaño total de la transferencia es al menos el `Notifications: minimum transfer size` especificado en la configuración.
- en **Actualización disponible**: siempre que haya una nueva versión de termscp disponible, se mostrará una notificación.
- en **Actualización instalada**: siempre que se haya instalado una nueva versión de termscp, se mostrará una notificación.
- en **Actualización fallida**: siempre que falle la instalación de la actualización, se mostrará una notificación.

❗ Si prefiere mantener las notificaciones desactivadas, puede simplemente ingresar a la configuración y configurar `Enable notifications?` En `No` 😉.  
❗ Si desea cambiar el tamaño mínimo de transferencia para mostrar notificaciones, puede cambiar el valor en la configuración con la tecla `Notifications: minimum transfer size` y configurarlo como mejor le convenga 🙂.
