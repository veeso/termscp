# Manuale utente 🎓

- [Manuale utente 🎓](#manuale-utente-)
  - [Argomenti da linea di comando ❓](#argomenti-da-linea-di-comando-)
    - [Argomento indirizzo 🌎](#argomento-indirizzo-)
      - [Argomento indirizzo per AWS S3](#argomento-indirizzo-per-aws-s3)
      - [Indirizzo SMB](#indirizzo-smb)
      - [Come fornire la password 🔐](#come-fornire-la-password-)
  - [Parametri di connessione S3](#parametri-di-connessione-s3)
    - [Credenziali S3 🦊](#credenziali-s3-)
  - [File explorer 📂](#file-explorer-)
    - [Abbinamento tasti ⌨](#abbinamento-tasti-)
    - [Lavora su più file 🥷](#lavora-su-più-file-)
    - [Synchronized browsing ⏲️](#synchronized-browsing-️)
    - [Apri e apri con 🚪](#apri-e-apri-con-)
  - [Segnalibri ⭐](#segnalibri-)
    - [Le mie password sono al sicuro 😈](#le-mie-password-sono-al-sicuro-)
      - [Linux Keyring](#linux-keyring)
        - [KeepassXC setup per termscp](#keepassxc-setup-per-termscp)
  - [Configurazione ⚙️](#configurazione-️)
    - [SSH Key Storage 🔐](#ssh-key-storage-)
    - [File Explorer Format](#file-explorer-format)
  - [Temi 🎨](#temi-)
    - [Il tema non carica 😱](#il-tema-non-carica-)
    - [Stili 💈](#stili-)
      - [Pagina autenticazione](#pagina-autenticazione)
      - [Pagina explorer e trasferimento](#pagina-explorer-e-trasferimento)
      - [Misc](#misc)
  - [Editor di testo ✏](#editor-di-testo-)
  - [Logging 🩺](#logging-)
  - [Notifiche 📫](#notifiche-)
  - [File watcher 🔭](#file-watcher-)

## Argomenti da linea di comando ❓

termscp può essere lanciato con questi argomenti:

`termscp [options]... [protocol://user@address:port:wrkdir] [local-wrkdir]`

O

`termscp [options]... -b [bookmark-name] [local-wrkdir]`

- `-P, --password <password>` Se viene fornito l'argomento indirizzo, questa sarà la password utilizzata per autenticarsi
- `-b, --address-as-bookmark` risolve l'argomento indirizzo come nome di un segnalibro
- `-q, --quiet` Disabilita i log
- `-v, --version` Mostra a video le informazioni sulla versione attualmente installata
- `-h, --help` Mostra la pagina di aiuto.

termscp può venire lanciato in due modalità diverse. Se nessun argomento posizionale viene fornito, termscp mostrerà il form di autenticazione, dove l'utente potrà fornire i parametri di connessione necessari.
Alternativamente, l'utente può fornire l'argomento posizionale "indirizzo" per connettersi direttamente all'host fornito.
Se viene fornito anche il secondo argomento posizionale, ovvero la directory locale, termscp avvierà l'explorer locale sul percorso fornito.

### Argomento indirizzo 🌎

L'argomento indirizzo ha la sintassi seguente:

```txt
[protocollo://][username@]<indirizzo>[:porta][:wrkdir]
```

Vediamo qualche esempio per questa sintassi, dal momento che risulta molto comodo connettersi tramite questa modalità:

- Connessione utilizzando il protocollo di default (definito in configurazione) a 192.168.1.31, la porta sarà quella di default per il protocollo di default. Il nome utente è quello attualmente attivo sulla propria macchina:

    ```sh
    termscp 192.168.1.31
    ```

- Connessione con protocollo di default a 192.168.1.31, utente è `root`:

    ```sh
    termscp root@192.168.1.31
    ```

- Connessione usando `scp`, la porta è 4022, l'utente è `omar`:

    ```sh
    termscp scp://omar@192.168.1.31:4022
    ```

- Connessione via `scp`, porta 4022, utente `omar`, l'explorer si avvierà in `/tmp`:

    ```sh
    termscp scp://omar@192.168.1.31:4022:/tmp
    ```

#### Argomento indirizzo per AWS S3

Aws S3 ha una sintassi differente dal classico argomento indirizzo, per ovvie ragioni, in quanto S3 non ha la porta o l'host o l'utente. Ho deciso però di mantenere una sintassi il più simile possibile a quella "tradizionale":

```txt
s3://<bucket-name>@<region>[:profile][:/wrkdir]
```

e.g.

```txt
s3://buckethead@eu-central-1:default:/assets
```

#### Indirizzo SMB

SMB ha una sintassi differente rispetto agli altri protocolli e cambia in base al sistema operativo:

**Windows**:

```txt
\\[username@]<server-name>\<share>[\path\...]
```

**Altri sistemi**:

```txt
smb://[username@]<server-name>[:port]/<share>[/path/.../]
```


#### Come fornire la password 🔐

Quando si usa l'argomento indirizzo non è possibile fornire la password direttamente nell'argomento, esistono però altri metodi per farlo:

- Argomento `-P, --password <password>`: Passa direttamente la password nell'argomento. Non lo consiglio particolarmente questo metodo, in quanto la password rimarrebbe nella history della shell in chiaro.
- Tramite `sshpass`: puoi fornire la password tramite l'applicazione GNU/Linux sshpass `sshpass -f ~/.ssh/topsecret.key termscp cvisintin@192.168.1.31`
- Forniscila quando richiesta: se non la fornisci tramite nessun metodo precedente, alla connessione ti verrà richiesto di fornirla in un prompt che la oscurerà (come avviene con sudo tipo).

---

## Parametri di connessione S3

Questi parametri sono necessari per connettersi ad un bucket Aws s3 o ad uno storage compatibile:

- AWS S3:
  - **bucket name**
  - **region**
  - *profile* (se non fornito: "default")
  - *access key* (a meno che non sia pubblico)
  - *secret access key* (a meno che non sia pubblico)
  - *security token* (se necessario)
  - *session token* (se necessario)
  - new path style: **NO**
- Other S3 endpoints:
  - **bucket name**
  - **endpoint**
  - *access key* (a meno che non sia pubblico)
  - *secret access key* (a meno che non sia pubblico)
  - new path style: **YES**

### Credenziali S3 🦊

Per connettersi ad un bucket S3 devi come già saprai fornire le credenziali fornite da AWS.
Ci sono tre modi per passare queste credenziali a termscp.
Questi sono quindi i tre modi per passare le chiavi:

1. Form di autenticazione:
   1. Puoi fornire la `access_key` (dovrebbe essere obbligatoria), la `secret_access_key` (dovrebbe essere obbligatoria), il `security_token` ed il `session_token`
   2. Se salvi la connessione s3 come segnalibro e decidi di salvare la password, questi parametri verranno salvati nel file dei segnalibri criptati con AES-256/BASE64; ad eccezion fatta per i due token, che dovrebbero essere credenziali temporanee, quindi inutili da salvare.
2. Utilizza il file delle credenziali s3: configurando aws via `aws configure` le tue credenziali dovrebbero già venir salvate in  `~/.aws/credentials`. Nel caso tu debba usare un profile diverso da `default`, puoi fornire un profilo diverso nell'authentication form.
3. **Variabili d'ambiente**: nel caso il primo metodo non sia utilizzabile, puoi comunque fornirle come variabili d'ambiente. Considera però che queste variabili sovrascriveranno sempre le credenziali situate nel file credentials. Vediamo come impostarle:

    Queste sono sempre obbligatorie:

    - `AWS_ACCESS_KEY_ID`: aws access key ID (di solito inizia per `AKIA...`)
    - `AWS_SECRET_ACCESS_KEY`: la secret access key

    nel caso tu abbia impostato un maggiore livello di sicurezza, potrebbero servirti anche queste:

    - `AWS_SECURITY_TOKEN`: security token
    - `AWS_SESSION_TOKEN`: session token

⚠️ le tue credenziali sono al sicuro: termscp non manipola direttamente questi dati! Le credenziali sono direttamente lette dal crate di **s3**. Nel caso tu abbia dei dubbi sulla sicurezza, puoi contattare l'autore della libreria su [Github](https://github.com/durch/rust-s3) ⚠️

---

## File explorer 📂

Quando ci riferiamo al file explorer in termscp, intendiamo i pannelli che puoi vedere quando stabilisci una connessione con il server remoto.
Questi pannelli sono 3 (e non 2 come sembra):

- Pannello locale: viene visualizzato sulla sinistra del tuo schermo e mostra la cartella sul file system locale.
- Pannello remoto: viene visualizzato sulla destra del tuo schermo e mostra la cartella sul file system remoto.
- Pannello di ricerca: viene visualizzato a destra o a sinistra in base a dove stai cercando dei file. Questo pannello mostra i file che matchano al pattern cercato sull'host.

Per cambiare pannello ti puoi muovere con le frecce, `<LEFT>` per andare sul pannello locale e `<RIGHT>` per andare su quello remoto. Attenzione che quando è attivo il pannello ricerca non puoi spostarti sugli altri pannelli e devi prima chiuderlo con `<ESC>`.

### Abbinamento tasti ⌨

| Key           | Command                                               | Reminder    |
|---------------|-------------------------------------------------------|-------------|
| `<ESC>`       | Disconnettiti; chiudi popup                           |             |
| `<BACKSPACE>` | Vai alla directory precedente                         |             |
| `<TAB>`       | Cambia pannello remoto                                |             |
| `<RIGHT>`     | Vai al pannello remoto                                |             |
| `<LEFT>`      | Vai al pannello locale                                |             |
| `<UP>`        | Muovi il cursore verso l'alto                         |             |
| `<DOWN>`      | Muovi il cursore verso il basso                       |             |
| `<PGUP>`      | Muovi il cursore verso l'alto di 8                    |             |
| `<PGDOWN>`    | Muovi il cursore verso il basso di 8                  |             |
| `<ENTER>`     | Entra nella directory                                 |             |
| `<SPACE>`     | Upload / download file selezionato/i                  |             |
| `<BACKTAB>`   | Cambia tra explorer e pannello di log                 |             |
| `<A>`         | Mostra/nascondi file nascosti                         | All         |
| `<B>`         | Ordina file per                                       | Bubblesort? |
| `<C|F5>`      | Copia file/directory                                  | Copy        |
| `<D|F7>`      | Crea directory                                        | Directory   |
| `<E|F8|DEL>`  | Elimina file                                          | Erase       |
| `<F>`         | Cerca file (wild match supportato)                    | Find        |
| `<G>`         | Vai al percorso indicato                              | Go to       |
| `<H|F1>`      | Mostra help                                           | Help        |
| `<I>`         | Mostra informazioni per il file selezionato           | Info        |
| `<K>`         | Crea un link simbolico che punta al file selezionato  | symlinK     |
| `<L>`         | Ricarica posizione corrente / pulisci selezione file  | List        |
| `<M>`         | Seleziona file                                        | Mark        |
| `<N>`         | Crea nuovo file con il nome fornito                   | New         |
| `<O|F4>`      | Modifica file; Vedi text editor                       | Open        |
| `<P>`         | Apri pannello log                                     | Panel       |
| `<Q|F10>`     | Termina termscp                                       | Quit        |
| `<R|F6>`      | Rinomina file                                         | Rename      |
| `<S|F2>`      | Salva file con nome                                   | Save        |
| `<T>`         | Sincronizza il percorso locale con l'host remoto      | Track       |
| `<U>`         | Vai alla directory padre                              | Upper       |
| `<V|F3>`      | Apri il file con il programma definito dal sistema    | View        |
| `<W>`         | Apri il file con il programma specificato             | With        |
| `<X>`         | Esegui comando shell                                  | eXecute     |
| `<Y>`         | Abilita/disabilita Sync-Browsing                      | sYnc        |
| `<Z>`         | Modifica permessi file                                |             |
| `<CTRL+A>`    | Seleziona tutti i file                                |             |
| `<CTRL+C>`    | Annulla trasferimento file                            |             |
| `<CTRL+T>`    | Visualizza tutti i percorsi sincronizzati             | Track       |

### Lavora su più file 🥷

Puoi lavorare su una selezione di file, marcandoli come selezionati tramite `<M>`, per selezionare il file corrente o con `<CTRL+A` per selezionarli tutti.
Una volta che un file è marcato, sarà visualizzato con un `*` prima del nome.
Quando lavori con una selezioni, solo i file selezionati saranno presi in considerazione (l'eventuale file evidenziato sarà ignorato).
È possibile operare su più file anche nel pannello di ricerca.
Tutte le azioni sono disponibili quando si lavora sulle selezioni, ma occhio, che alcune azioni si comporteranno in maniera leggermente differente. Vediamo quali e come:

- *Copia*: Se copi un file, ti verrà richiesto di inserire il nome delle destinazione, ma quando lavori con la selezione, il nome si riferisce alla directory di destinazione, mentre il nome del file rimarrà inviariato.
- *Rinomina*: Come il copia, ma li sposterà.
- *Salva con nome*: Come il copia, ma li trasferirà.

### Synchronized browsing ⏲️

Quando abilitato, ti permetterà di sincronizzare la navigazione tra i due pannelli.
Ciò comporta che quando cambierai directory in uno dei due pannelli, lo stesso verrà fatto nell'altro. Per abilitare la modalità è sufficiente premere `<Y>`; fai lo stesso per disabilitarlo. Mentre abilitato, sull'interfaccia dovrebbe essere visualizzato `Sync Browsing: ON` nella barra di stato.

### Apri e apri con 🚪

I comandi "apri" e "apri con" sono forniti da [open-rs](https://docs.rs/crate/open/2.1.0).
Quando apri un file (`<V>`), l'applicazione predefinita di sistema sarà utilizzata per aprire il file. Per fare ciò, sul tuo sistema dovrà essere usato il servizio di default del sistema.

- **Windows**: non devi installare niente, è già presente sul sistema.
- **MacOS**: non devi installare niente, è già presente sul sistema.
- **Linux**: uno di questi dev'essere presente (potrebbe già esserlo):
  - *xdg-open*
  - *gio*
  - *gnome-open*
  - *kde-open*
- **WSL**: *wslview* è richiesto, lo puoi installare tramite questa suite [wslu](https://github.com/wslutilities/wslu).

> Q: Posso modificare i file su remoto tramite la funzionalità "apri" / "apri con"?  
> A: No, almeno non direttamente dal pannello remoto. Devi prima scaricarlo in locale, modificarlo e poi ricaricarlo. Questo perché il file remoto viene scaricato come file temporaneo in locale, ma non esiste poi un modo per sapere quando è stato modificato e quando l'utente ha effettivamente finito di lavorarci.

---

## Segnalibri ⭐

In termscp è possibile salvare i tuoi host preferiti tramite i segnalibri al fine di connettersi velocemente ad essi.
Termscp salverà anche gli ultimi 16 host ai quali ti sei connesso.
Questa funzionalità ti permette di caricare tutti i parametri necessari per connettersi ad un certo host, semplicemente selezioandolo dal tab dei preferiti nel form di autenticazione.

I preferiti saranno salvati se possibile presso:

- `$HOME/.config/termscp/` su Linux/BSD
- `$HOME/Library/Application Support/termscp` su MacOs
- `FOLDERID_RoamingAppData\termscp\` su Windows

Per i segnalibri (ma non per le connessioni recenti), è anche possibile salvare la password. La password non viene salvata di default e deve essere specificato tramite apposita opzione, al momento della creazione del segnalibro stesso.
Se sei preoccupato riguardo alla sicurezza della password per i segnalibri, dai un'occhiata al capitolo qui sotto 👀.

Per creare un segnalibro, segui questa procedura:

1. Inserisci i parametri per connetterti all'host che vuoi inserire come segnalibro nell'authentication form.
2. Premi `<CTRL+S>`
3. Inserisci il nome che vuoi dare al bookmark
4. Seleziona nel radio button se salvare la password
5. Premi `<ENTER>` per salvare

Quando vuoi caricare un segnalibro, premi `<TAB>` e naviga nella lista dei segnalibri fino al segnalibro che vuoi caricare, quindi premi `<ENTER>`.

![Bookmarks](https://github.com/veeso/termscp/blob/main/assets/images/bookmarks.gif?raw=true)

### Le mie password sono al sicuro 😈

Certo 😉.
Come detto in precedenza, i segnalibri sono salvati nella cartella delle configurazioni insieme alle password. Le password però non sono in chiaro, ma bensì sono criptate con **AES-128**. Questo le rende sicure? Sì! Does this make them safe? (salvo che per gli utenti di FreeBSD e WSL 😢)

In **Windows**, **Linux** and **MacOS** la chiave per criptare le password è salvata, se possibile, rispettivamente nel *Windows Vault*, nel *system keyring* e nel *Keychain*. Questo sistema è super-sicuro, in quanto garantito direttamente dal tuo sistema operativo.

❗ Attenzione che se sei un utente Linux, dovresti leggere il capitolo qui sotto riguardante il linux keyring 👀, questo perché il keyring potrebbe non essere ancora presente sul tuo sistema.

Su *FreeBSD* e *WSL*, d'altro canto, la chiave utilizzata per criptare le password è salvata su file presso (at $HOME/.config/termscp). È quindi possibile per un malintenzionato ottenere la chiave. Per fortuna essendo sotto la tua home, non dovrebbe essere possibile accedere al file, se non con il tuo utente, ma comunque per sicurezza ti consiglio di non salvare dati sensibili 😉.

#### Linux Keyring

Tutti gli amanti di Linux lo preferiscono per la libertà che questo dà agli utenti nella personalizzazione. Allo stesso tempo però questo spesso comporta degli effetti collaterali, tra cui la mancanza spesso di un'imposizione da parte dei creatori delle distro di standard e applicazioni e questo fatto coinvolge anche la questione del keyring.
Su alcuni sistemi di default, non c'è nessun provider di keyring, perché la distro dà all'utente la possibilità di sceglierne uno.
termscp richiede un servizio D-BUS che fornisce `org.freedesktop.secrets` e purtroppo ci sono ad oggi solo due servizi mantenuti che lo supportano.

- ❗ Se usi GNOME come Desktop environment (come gli utenti Ubuntu), dovresti già averne uno installato sul sistema, chiamato `gnome-keyring` e quindi dovrebbe già funzionare tutto.
- ❗ Se invece usi un altro DE, dovresti installare [KeepassXC](https://keepassxc.org/), che io per esempio utilizzo sul mio Manjaro Linux (con KDE) e funziona piuttosto bene. L'unico problema è che dovrai fare il setup per farlo funzionare. Per farlo puoi leggere il tutorial [qui](#keepassxc-setup-per-termscp)
- ❗ Se non volessi installare uno di questi servizi, termscp funzionerà come sempre, l'unica differenza sarà che salverà la chiave di crittazione su un file, come fa per FreeBSD e WSL.

##### KeepassXC setup per termscp

Questo tutorial spiega come impostare KeepassXC per termscp.

1. Installa KeepassXC dal sito ufficiale <https://keepassxc.org/>
2. Una volta avviato, vai su "strumenti" > "impostazioni" nella toolbar
3. Seleziona "Secret service integration" e abilita "Enable KeepassXC freedesktop.org secret service integration"
4. Crea un database se non ne hai già uno: dalla toolbar "Database" > "Nuovo database"
5. Dalla toolbar: "Database" > "Impostazioni database"
6. Seleziona "Secret service integration" e abilita "Expose entries under this group"
7. Seleziona il gruppo in cui vuoi salvare le chiavi di termscp. Attenzione che questo gruppo sarà utilizzato da tutte le altre eventuali applicazioni che salvano le password via D-BUS.

---

## Configurazione ⚙️

termscp supporta diversi parametri definiti dall'utente, che possono essere impostati nella configurazione.
termscp usa un file TOML e altre directory per salvare tutti i parametri, ma non preoccuparti, tutto può essere comodamente configurato da interfaccia grafica.

Per la configurazione, termscp richiede che i seguenti percorsi siano accessibili (termscp proverà a crearli per te):

- `$HOME/.config/termscp/` su Linux/BSD
- `$HOME/Library/Application Support/termscp` su MacOs
- `FOLDERID_RoamingAppData\termscp\` su Windows

Per accedere alla configurazione è sufficiente premere `<CTRL+C>` dall'authentication form.

Questi parametri possono essere impostati:

- **Text Editor**: l'editor di testo da utilizzare per aprire i file. Di default termscp userà quello definito nella variabile `EDITOR` od il primo che troverà installato tra quelli più popolari. Puoi tuttavia definire quello che vuoi (ad esempio `vim`). **Anche gli editor GUI sono supportati**, a meno che loro non partano in `nohup` dal processo padre.
- **Default Protocol**: il protocollo di default da visualizzare come prima opzione nell'authentication form. Questa opzione sarà anche utilizzata quando si usa l'argomento indirizzo da CLI e non si specifica un protocollo.
- **Show Hidden Files**: seleziona se mostrare di default i file nascosti. A runtime potrai comunque scegliere se visualizzarli o meno premendo `<A>`.
- **Check for updates**: se impostato a `YES` all'avvio termscp controllerà l'eventuale presenza di aggiornamenti. Per farlo utilizzerà una chiamata GET all'API di Github.
- **Prompt when replacing existing files?**: se impostato a `yes`, termscp ti chiederà una conferma prima di sovrascrivere un file a seguito di un download/upload.
- **Group Dirs**: seleziona se e come raggruppare le cartelle negli explorer. Se `Display first` è impostato, le directory verranno ordinate secondo quanto stabilito nel `sort by`, ma verranno messe prima dei file, viceversa se `Display last` è utilizzato. Se invece metti `no`, le cartelle verrano messe in ordine assieme ad i file.
- **Remote File formatter syntax**: La formattazione da usare per formattare i file sull'explorer remoto. Vedi [File explorer format](#file-explorer-format)
- **Local File formatter syntax**: La formattazione da usare per formattare i file sull'explorer locale. Vedi [File explorer format](#file-explorer-format)
- **Enable notifications?**: Se impostato a `yes`, le notifiche desktop saranno abilitate.
- **Notifications: minimum transfer size**: se la dimensione di un trasferimento supera o è uguale al valore impostato, al termine del trasferimento riceverai una notifica desktop (se queste sono abilitate). Il formato del valore dev'essere `{UNSIGNED} B/KB/MB/GB/TB/PB`
- **SSH configuration path**: Imposta il percorso del file di configurazione per SSH, per quando ci si connette ad un server SFTP/SCP. Se lasciato vuoto, nessun file verrà usato. Il percorso può anche iniziare con `~` per indicare il percorso della home dell'utente attuale (e.s. `~/.ssh/config`). I parametri supportati dalla configurazioni sono descritti [QUI](https://github.com/veeso/ssh2-config#exposed-attributes).

### SSH Key Storage 🔐

Assieme alla configurazione termscp supporta anche una feature essenziale per i client **SFTP/SCP**: lo storage di chiavi SSH.

Puoi accedere allo storage muovendoti nel tab delle chiavi SSH tramite `<TAB>` dalla configurazione.

- **Aggiungere chiavi**: premi `<CTRL+N>` e ti verrà chiesto di creare una nuova chiave. Inserisci l'hostname/indirizzo ed il nome utente, infine una volta che premerai invio, ti si aprirà l'editor di testo: incolla la chiave SSH **PRIVATA**, salva ed esci.
- **Rimuovi una chiave esistente**: premi `<DEL>` o `<CTRL+E>` selezionando la chiave da rimuovere.
- **Aggiorna una chiave esistente**: premi `<ENTER>` sulla chiave che vuoi modificare.

> Q: Se la mia chiave è protetta da password, posso comunque usarla?  
> A: Sì, certo. In questo caso dovrai fornire la password come faresti per autenticarti con utente/password, ma in questo caso la password sarà usata per decrittare la chiave.

### File Explorer Format

È possibile dalla configurazione impostare la formattazione dei file sull'explorer. È possibile sia farlo per il pannello locale, che per quello remoto; quindi puoi avere due sintassi diverse. Questi campi, con nome `File formatter syntax (local)` and `File formatter syntax (remote)` definiranno come i file devono essere formattati sull'explorer.
La sintassi è la seguente `{KEY1}... {KEY2:LENGTH}... {KEY3:LENGTH:EXTRA} {KEYn}...`.
Ogni chiave sarà rimpiazzata dal formatter con il relativo attributo, mentre tutto ciò che è fuori dalle parentesi graffe rimarrà inviariato (quindi puoi metterci del testo arbitratio).

- Il nome della chiave è obbligatorio e dev'essere uno di quelli sotto.
- La lunghezza descrive quanto spazio in caratteri riservare al campo. Attributi con dimensione statico (GROUP, PEX, SIZE, USER) non supportano la lunghezza.
- L'extra serve a definire degli attributi in più. Solo alcuni lo supportano.

These are the keys supported by the formatter:

- `ATIME`: Last access time (con sintassi di default `%b %d %Y %H:%M`); Extra definisce il formato data (e.g. `{ATIME:8:%H:%M}`)
- `CTIME`: Creation time (con sintassi di default `%b %d %Y %H:%M`); Extra definisce il formato data (e.g. `{CTIME:8:%H:%M}`)
- `GROUP`: Owner group
- `MTIME`: Last change time (con sintassi di default `%b %d %Y %H:%M`); Extra definisce il formato data (e.g. `{MTIME:8:%H:%M}`)
- `NAME`: Nome file (Le cartelle comprese tra la root ed il genitore del file sono omessi se la lunghezza è maggiore di LENGTH)
- `PATH`: Percorso assoluto del file (Le cartelle comprese tra la root ed il genitore del file sono omessi se la lunghezza è maggiore di LENGHT)
- `PEX`: Permessi utente (formato UNIX)
- `SIZE`: Dimensione file (omesso per le directory)
- `SYMLINK`: Link simbolico (se presente `-> {FILE_PATH}`)
- `USER`: Owner user

Se lasciata vuota, la sintassi di default sarà utilizzata: `{NAME:24} {PEX} {USER} {SIZE} {MTIME:17:%b %d %Y %H:%M}`

---

## Temi 🎨

termscp fornisce anche una funzionalità strafiga: la possibilità di impostare i colori per tutta l'interfaccia.
Se vuoi impostare i colori, ci sono due modi per farlo:

- dal **menù di configurazione**
- importando un **tema** da file

Per personalizzare i colori dovrai andare nella configurazione temi, partendo dal menù di autenticazione, premendo `<CTRL+C>` e premendo due volte `<TAB>`. Dovresti essere quindi in configurazione nel tab `themes`.

Da qui puoi spostarti con le frecce per cambiare lo stile che vuoi, come mostrato nella GIF qua sotto:

![Themes](https://github.com/veeso/termscp/blob/main/assets/images/themes.gif?raw=true)

termscp supporta diverse sintassi per i colori, sia il formato hex (`#rrggbb`) che rgb `rgb(r, g, b)`, ma anche i **[colori CSS](https://www.w3schools.com/cssref/css_colors.asp)** (tipo `crimson`) 😉. C'è anche una chiave speciale `Default`. Default significa che per il colore verrà usato il default in base al tipo di elemento (foreground per i testi e linee, background per gli sfondi e i riempimenti).

Come detto già in precedenza, puoi anche importare i temi da file. Volendo puoi anche creare un tema prendendo ispirazione da quelli situati nella cartella `themes/` del repository ed importarli su termscp con `termscp -t <theme_file>`. Se l'operazione va a buon fine dovrebbe dirti che l'ha importato con successo.

### Il tema non carica 😱

Probabilmente è dovuto ad un aggiornamento che ha rotto il tema. Se viene aggiunta una nuova chiave nel tema (ma questo accade molto raramente), il tema non verrà più caricato. Ci sono diverse soluzioni veloci per questo problema.

1. Ricarica il tema: se stai usando un tema "ufficiale" fornito nel repository, basterà ricaricarlo, perché li aggiorno sempre quando modifico i temi:

    ```sh
    termscp -t <theme.toml>
    ```

2. Sistema il tuo tema a mano: puoi modificare il tuo tema con un editor di testo tipo `vim` e aggiungere la chiave mancante. Il il tema si trova in `$CONFIG_DIR/termscp/theme.toml` dove `$CONFIG_DIR` è:

    - FreeBSD/GNU-Linux: `$HOME/.config/`
    - MacOs: `$HOME/Library/Application Support`
    - Windows: `%appdata%`

    ❗ Le chiavi mancanti vengono riportate nel CHANGELOG sotto `BREAKING CHANGES` per la versione installata.

### Stili 💈

Puoi trovare qui sotto la definizione per ogni chiave.
Attenzione che gli stili **non coinvolgono la pagina di configurazione**, per renderla sempre accessibile nel caso gli stili siano inutilizzabili.

#### Pagina autenticazione

| Key            | Description                        |
|----------------|------------------------------------|
| auth_address   | Colore del campo indirizzo IP      |
| auth_bookmarks | Colore del pannello segnalibri     |
| auth_password  | Colore del campo password          |
| auth_port      | Colore del campo numero porta      |
| auth_protocol  | Colore del selettore di protocollo |
| auth_recents   | Colore del pannello recenti        |
| auth_username  | Colore del campo nome utente       |

#### Pagina explorer e trasferimento

| Key                                  | Description                                                               |
|--------------------------------------|---------------------------------------------------------------------------|
| transfer_local_explorer_background   | Sfondo explorer locale                                                    |
| transfer_local_explorer_foreground   | Foreground explorer locale                                                |
| transfer_local_explorer_highlighted  | Colore bordo e file selezionato explorer locale                           |
| transfer_remote_explorer_background  | Sfondo explorer remoto                                                    |
| transfer_remote_explorer_foreground  | Foreground explorer remoto                                                |
| transfer_remote_explorer_highlighted | Colore bordo e file selezionato explorer remoto                           |
| transfer_log_background              | Sfondo pannello di log                                                    |
| transfer_log_window                  | Colore bordi e testo log                                                  |
| transfer_progress_bar_partial        | Colore barra progresso parziale                                           |
| transfer_progress_bar_total          | Colore barra progresso totale                                             |
| transfer_status_hidden               | Colore status bar file nascosti                                           |
| transfer_status_sorting              | Colore status bar ordinamento file; si applica anche al popup ordinamento |
| transfer_status_sync_browsing        | Colore status bar per sync browsing                                       |

#### Misc

Questi stili si applicano a varie componenti dell'applicazione.

| Key               | Description                                 |
|-------------------|---------------------------------------------|
| misc_error_dialog | Colore dialoghi errore                      |
| misc_info_dialog  | Colore per dialoghi informazioni            |
| misc_input_dialog | Colore per dialoghi input (tipo copia file) |
| misc_keys         | Colore per abbinamento tasti                |
| misc_quit_dialog  | Colore per dialogo quit                     |
| misc_save_dialog  | Colore per dialogo salva                    |
| misc_warn_dialog  | Colore per dialoghi avvertimento            |

---

## Editor di testo ✏

Con termscp puoi anche modificare i file di testo direttamente da terminale, utilizzando il tuo editor preferito.
Non importa se il file si trova in locale od in remoto, termscp ti consente di modificare e sincronizzare le modifiche per entrambi.
Nel caso il file si trovi su host remoto, il file verrà prima scaricato temporaneamente in locale, modificato e poi nel caso ci siano state modifiche, reinviato in remoto.

> ❗ Ricorda: **puoi modificare solo i file testuali**; non puoi modificare i file binari.

---

## Logging 🩺

termscp scrive un file di log per ogni sessione, nel percorso seguente:

- `$HOME/.cache/termscp/termscp.log` su Linux/BSD
- `$HOME/Library/Caches/termscp/termscp.log` su MacOs
- `FOLDERID_LocalAppData\termscp\termscp.log` su Windows

Il log non viene ruotato, ma viene troncato ad ogni lancio di termscp, quindi se devi riportare un issue, non avviare termscp fino a che non avrai salvato il file di log.
I log sono di default riportati a livello *INFO*, quindi non sono particolarmente parlanti.

Se vuoi riportare un problema, se riesci, riproduci l'errore lanciando termscp in modalità di debug, in modo da fornire un log più dettagliato.
Per farlo, lancia termscp con l'opzione `-D`.

Ho scritto questo FAQ sui log, visto che potresti avere qualche dubbio:

> Non voglio il log, posso disabilitarlo?

Sì, puoi. Basta lanciare termscp con `-q or --quiet` come opzione. Puoi mantenerlo persistente salvandolo come alias nella tua shell. Ricorda che i log vengono usati per diagnosticare problemi e considerando che questo è un progetto open-source è anche un modo per contribuire al progetto 😉. Non voglio far sentire in colpa nessuno, ma tanto per dire.

> Il log è sicuro?

Se ti chiedi se il log espone dati sensibili, il log non espone nessuna password o dato sensibile.

## Notifiche 📫

termscp invierà notifiche destkop per i seguenti eventi:

- a **Transferimento completato**: La notifica verrà inviata a seguito di un trasferimento completato.
  - ❗ La notifica verrà mostrata solo se la dimensione totale del trasferimento è uguale o maggiore al parametro `Notifications: minimum transfer size` definito in configurazione.
- a **Transferimento fallito**: La notifica verrà inviata a seguito di un trasferimento fallito.
  - ❗ La notifica verrà mostrata solo se la dimensione totale del trasferimento è uguale o maggiore al parametro `Notifications: minimum transfer size` definito in configurazione.
- ad **Aggiornamento disponibile**: Ogni volta che una nuova versione di termscp è disponibile, verrà mostrata una notifica.
- ad **Aggiornamento installato**: Al termine dell'installazione di un aggiornamento, verrà mostrata una notifica.
- ad **Aggiornamento fallito**: Al fallimento dell'installazione di un aggiornamento, verrà mostrata una notifica.

❗ Se vuoi disabilitare le notifiche, è sufficiente andare in configurazione ed impostare `Enable notifications?` a `No` 😉.  
❗ Se vuoi modificare la soglia minima per le notifiche dei trasferimenti, puoi impostare il valore di `Notifications: minimum transfer size` in configurazione 🙂.

## File watcher 🔭

Il file watcher ti permette di impostare una lista di percorsi da sincronizzare con l'host remoto.
Ciò implica che ogni volta che una modifica verrà rilevata al percorso sincronizzato, la modifica verrà automaticamente sincronizzata con l'host remoto, entro 5 secondi.

Puoi impostare quanti percorsi preferisci da sincronizzare:

1. Porta il cursore dell'explorer sulla cartella/file che vuoi sincronizzare
2. Vai alla directory sull'explorer remoto dove vuoi riportare le modifiche
3. Premi `<T>`
4. Rispondi `<YES>` alla domanda se vuoi sincronizzare il percorso

Per terminare la sincronizzazione, premi `<T>`, al percorso locale sincronizzato (od in qualsiasi sua sottocartella)
OPPURE, puoi semplicemente premere `<CTRL+T>` e premi `<ENTER>` sul percorso che vuoi desincronizzare.

Queste modifiche verranno applicate sull'host remoto:

- Nuovi file, modifiche
- File spostati o rinominati
- File rimossi

> ❗ Il watcher funziona solo in maniera unidirezionale locale > remoto. NON è possibile tracciare le modifiche da remoto a locale.
