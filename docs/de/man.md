# Benutzerhandbuch 🎓

- [Benutzerhandbuch 🎓](#benutzerhandbuch-)
  - [Verwendung ❓](#verwendung-)
    - [Adressargument 🌎](#adressargument-)
      - [AWS S3 Adressargument](#aws-s3-adressargument)
      - [Kube-Adressargument](#kube-adressargument)
      - [SMB Adressargument](#smb-adressargument)
      - [Wie das Passwort bereitgestellt werden kann 🔐](#wie-das-passwort-bereitgestellt-werden-kann-)
    - [Unterbefehle](#unterbefehle)
      - [Ein Thema importieren](#ein-thema-importieren)
      - [Neueste Version installieren](#neueste-version-installieren)
    - [Unterbefehle](#unterbefehle-1)
      - [Ein Theme importieren](#ein-theme-importieren)
      - [Neueste Version installieren](#neueste-version-installieren-1)
      - [SSH-Hosts importieren](#ssh-hosts-importieren)
  - [S3-Verbindungsparameter](#s3-verbindungsparameter)
    - [S3-Anmeldeinformationen 🦊](#s3-anmeldeinformationen-)
  - [Dateiexplorer 📂](#dateiexplorer-)
    - [Tastenkombinationen ⌨](#tastenkombinationen-)
    - [Mit mehreren Dateien arbeiten 🥷](#mit-mehreren-dateien-arbeiten-)
      - [Beispiel](#beispiel)
    - [Synchronisiertes Durchsuchen ⏲️](#synchronisiertes-durchsuchen-️)
    - [Öffnen und Öffnen mit 🚪](#öffnen-und-öffnen-mit-)
  - [Lesezeichen ⭐](#lesezeichen-)
    - [Sind meine Passwörter sicher 😈](#sind-meine-passwörter-sicher-)
      - [Linux-Schlüsselbund](#linux-schlüsselbund)
        - [KeepassXC-Einrichtung für termscp](#keepassxc-einrichtung-für-termscp)
  - [Konfiguration ⚙️](#konfiguration-️)
    - [SSH-Schlüssel-Speicherung 🔐](#ssh-schlüssel-speicherung-)
    - [Dateiexplorer-Format](#dateiexplorer-format)
  - [Themen 🎨](#themen-)
      - [AWS S3 Adressargument](#aws-s3-adressargument-1)
      - [SMB Adressargument](#smb-adressargument-1)
      - [Wie das Passwort bereitgestellt werden kann 🔐](#wie-das-passwort-bereitgestellt-werden-kann--1)
    - [Unterbefehle](#unterbefehle-2)
      - [Ein Thema importieren](#ein-thema-importieren-1)
      - [Neueste Version installieren](#neueste-version-installieren-2)
  - [S3-Verbindungsparameter](#s3-verbindungsparameter-1)
    - [S3-Anmeldeinformationen 🦊](#s3-anmeldeinformationen--1)
  - [Dateiexplorer 📂](#dateiexplorer--1)
    - [Tastenkombinationen ⌨](#tastenkombinationen--1)
    - [Mit mehreren Dateien arbeiten 🥷](#mit-mehreren-dateien-arbeiten--1)
    - [Synchronisiertes Durchsuchen ⏲️](#synchronisiertes-durchsuchen-️-1)
    - [Öffnen und Öffnen mit 🚪](#öffnen-und-öffnen-mit--1)
  - [Lesezeichen ⭐](#lesezeichen--1)
    - [Sind meine Passwörter sicher 😈](#sind-meine-passwörter-sicher--1)
      - [Linux-Schlüsselbund](#linux-schlüsselbund-1)
        - [KeepassXC-Einrichtung für termscp](#keepassxc-einrichtung-für-termscp-1)
  - [Konfiguration ⚙️](#konfiguration-️-1)
    - [SSH-Schlüssel-Speicherung 🔐](#ssh-schlüssel-speicherung--1)
    - [Dateiexplorer-Format](#dateiexplorer-format-1)
  - [Themen 🎨](#themen--1)
    - [Mein Thema wird nicht geladen 😱](#mein-thema-wird-nicht-geladen-)
    - [Stile 💈](#stile-)
      - [Authentifizierungsseite](#authentifizierungsseite)
      - [Übertragungsseite](#übertragungsseite)
      - [Sonstiges](#sonstiges)
  - [Texteditor ✏](#texteditor-)
  - [Protokollierung 🩺](#protokollierung-)
  - [Benachrichtigungen 📫](#benachrichtigungen-)
  - [Dateiwächter 🔭](#dateiwächter-)

> ❗ Ich benötige Hilfe bei der Übersetzung dieses Handbuchs ins Deutsche. Wenn Sie zur Übersetzung beitragen möchten, öffnen Sie bitte einen PR 🙏

## Verwendung ❓

termscp kann mit den folgenden Optionen gestartet werden:

`termscp [Optionen]... [protokoll://benutzer@adresse:port:arbeitsverzeichnis] [protokoll://benutzer@adresse:port:arbeitsverzeichnis] [lokales-arbeitsverzeichnis]`

ODER

`termscp [Optionen]... -b [Lesezeichen-Name] -b [Lesezeichen-Name] [lokales-arbeitsverzeichnis]`

- `-P, --password <Passwort>` wenn Adresse angegeben wird, ist das Passwort dieses Argument
- `-b, --address-as-bookmark` löst das Adressargument als Lesezeichenname auf
- `-q, --quiet` Protokollierung deaktivieren
- `-v, --version` Versionsinformationen anzeigen
- `-h, --help` Hilfeseite anzeigen

termscp kann in drei verschiedenen Modi gestartet werden. Wenn keine zusätzlichen Argumente angegeben werden, zeigt termscp das Authentifizierungsformular an, in dem der Benutzer die erforderlichen Parameter zum Herstellen einer Verbindung mit dem Remote-Peer angeben kann.

Alternativ kann der Benutzer eine Adresse als Argument angeben, um das Authentifizierungsformular zu überspringen und direkt die Verbindung zum Remote-Server zu starten.

Wenn das Adressargument oder der Lesezeichenname angegeben wird, können Sie auch das Startarbeitsverzeichnis für den lokalen Host angeben.

### Adressargument 🌎

Das Adressargument hat die folgende Syntax:

```txt
[protokoll://][benutzername@]<adresse>[:port][:arbeitsverzeichnis]
```

Sehen wir uns einige Beispiele für diese besondere Syntax an, da sie sehr komfortabel ist und Sie diese wahrscheinlich anstelle der anderen verwenden werden...

- Verbindung mit dem Standardprotokoll herstellen (_in der Konfiguration definiert_) zu 192.168.1.31, Port, wenn nicht angegeben, ist Standard für das ausgewählte Protokoll (in diesem Fall hängt es von Ihrer Konfiguration ab); Benutzername ist der aktuelle Benutzername

```sh
termscp 192.168.1.31
```

- Verbindung mit dem Standardprotokoll herstellen (_in der Konfiguration definiert_) zu 192.168.1.31; Benutzername ist `root`

```sh
termscp root@192.168.1.31
```

- Verbindung mit scp zu 192.168.1.31, Port ist 4022; Benutzername ist `omar`

```sh
termscp scp://omar@192.168.1.31:4022
```

- Verbindung mit scp zu 192.168.1.31, Port ist 4022; Benutzername ist `omar`. Sie starten im Verzeichnis `/tmp`

```sh
termscp scp://omar@192.168.1.31:4022:/tmp
```

#### AWS S3 Adressargument

AWS S3 hat aus offensichtlichen Gründen eine andere Syntax für CLI-Adressargumente, aber ich habe es geschafft, sie so ähnlich wie möglich an das generische Adressargument anzupassen:

```txt
s3://<bucket-name>@<region>[:profile][:/arbeitsverzeichnis]
```

z.B.

```txt
s3://buckethead@eu-central-1:default:/assets
```

#### Kube-Adressargument

Falls Sie eine Verbindung zu Kube herstellen möchten, verwenden Sie die folgende Syntax

```txt
kube://[namespace][@<cluster_url>][$</path>]
```

#### SMB Adressargument

SMB hat eine andere Syntax für CLI-Adressargumente, die je nach System unterschiedlich ist:
**Windows** -Syntax:

```txt
\\[benutzername@]<server-name>\<freigabe>[\pfad\...]
```

**Andere Systeme** -Syntax:

```txt
smb://[benutzername@]<server-name>[:port]/<freigabe>[/pfad/.../]
```

#### Wie das Passwort bereitgestellt werden kann 🔐

Sie haben wahrscheinlich bemerkt, dass beim Bereitstellen der Adresse als Argument keine Möglichkeit besteht, das Passwort anzugeben.
Das Passwort kann im Wesentlichen auf drei Arten bereitgestellt werden, wenn das Adressargument angegeben wird:

- `-P, --password` Option: Verwenden Sie einfach diese CLI-Option und geben Sie das Passwort an. Ich rate dringend von dieser Methode ab, da sie sehr unsicher ist (da Sie das Passwort möglicherweise in der Shell-Historie behalten)

- Über `sshpass`: Sie können das Passwort über `sshpass` bereitstellen, z.B. `sshpass -f ~/.ssh/topsecret.key termscp cvisintin@192.168.1.31`

- Sie werden danach gefragt: Wenn Sie keine der vorherigen Methoden verwenden, werden Sie nach dem Passwort gefragt, wie es bei den klassischen Tools wie `scp`, `ssh` usw. der Fall ist.

### Unterbefehle

#### Ein Thema importieren

Führen Sie termscp als `termscp theme <thema-datei>` aus

#### Neueste Version installieren

Führen Sie termscp als `termscp update` aus

### Unterbefehle

#### Ein Theme importieren

Führen Sie termscp mit `termscp theme <theme-datei>` aus.

#### Neueste Version installieren

Führen Sie termscp mit `termscp update` aus.

#### SSH-Hosts importieren

Führen Sie termscp mit `termscp import-ssh-hosts [ssh-config-datei]` aus.

Importieren Sie alle Hosts aus der angegebenen SSH-Konfigurationsdatei (wenn keine angegeben ist, wird `~/.ssh/config` verwendet) als Lesezeichen in termscp. Identitätsdateien werden ebenfalls als SSH-Schlüssel in termscp importiert.

---

## S3-Verbindungsparameter

Diese Parameter sind erforderlich, um eine Verbindung zu AWS S3 und anderen S3-kompatiblen Servern herzustellen:

- AWS S3:

  - **Bucket-Name**

  - **Region**

  - _Profil_ (wenn nicht angegeben: "default")

  - _Zugriffsschlüssel_ (sofern nicht öffentlich)

  - _Geheimer Zugriffsschlüssel_ (sofern nicht öffentlich)

  - _Sicherheitstoken_ (falls erforderlich)

  - _Sitzungstoken_ (falls erforderlich)

  - neuer Pfadstil: **NEIN**

- Andere S3-Endpunkte:

  - **Bucket-Name**

  - **Endpunkt**

  - _Zugriffsschlüssel_ (sofern nicht öffentlich)

  - _Geheimer Zugriffsschlüssel_ (sofern nicht öffentlich)

  - neuer Pfadstil: **JA**

### S3-Anmeldeinformationen 🦊

Um eine Verbindung zu einem AWS S3-Bucket herzustellen, müssen Sie offensichtlich einige Anmeldeinformationen angeben.
Es gibt im Wesentlichen drei Möglichkeiten, dies zu erreichen.
Dies sind die Möglichkeiten, wie Sie die Anmeldeinformationen für S3 bereitstellen können:

1. Authentifizierungsformular:

1. Sie können den `access_key` (sollte obligatorisch sein), den `secret_access_key` (sollte obligatorisch sein), `security_token` und den `session_token` angeben.

1. Wenn Sie die S3-Verbindung als Lesezeichen speichern, werden diese Anmeldeinformationen als verschlüsselter AES-256/BASE64-String in Ihrer Lesezeichen-Datei gespeichert (außer dem Sicherheitstoken und dem Sitzungstoken, die als temporäre Anmeldeinformationen gedacht sind).

1. Verwenden Sie Ihre Anmeldeinformationsdatei: Konfigurieren Sie einfach die AWS-CLI über `aws configure` und Ihre Anmeldeinformationen sollten bereits unter `~/.aws/credentials` gespeichert sein. Falls Sie ein anderes Profil als `default` verwenden, geben Sie es im Feld "Profil" im Authentifizierungsformular an.

1. **Umgebungsvariablen** : Sie können Ihre Anmeldeinformationen jederzeit als Umgebungsvariablen angeben. Beachten Sie, dass diese Anmeldeinformationen **immer die im Anmeldeinformationsdatei** angegebenen Anmeldeinformationen überschreiben. So konfigurieren Sie die Umgebung:
   Diese sollten immer obligatorisch sein:

- `AWS_ACCESS_KEY_ID`: AWS-Zugriffsschlüssel-ID (beginnt normalerweise mit `AKIA...`)

- `AWS_SECRET_ACCESS_KEY`: der geheime Zugriffsschlüssel

Falls Sie eine stärkere Sicherheit konfiguriert haben, benötigen Sie möglicherweise auch diese:

- `AWS_SECURITY_TOKEN`: Sicherheitstoken

- `AWS_SESSION_TOKEN`: Sitzungstoken
  ⚠️ Ihre Anmeldeinformationen sind sicher: termscp manipuliert diese Werte nicht direkt! Ihre Anmeldeinformationen werden direkt von der **S3** -Bibliothek verwendet.
  Falls Sie Bedenken hinsichtlich der Sicherheit haben, kontaktieren Sie bitte den Bibliotheksautor auf [Github](https://github.com/durch/rust-s3) ⚠️

---

## Dateiexplorer 📂

Wenn wir in termscp von Dateiexplorern sprechen, meinen wir die Panels, die Sie nach dem Herstellen einer Verbindung mit dem Remote-Host sehen können.
Diese Panels sind im Wesentlichen 3 (ja, tatsächlich drei):

- Lokales Explorer-Panel: Es wird links auf Ihrem Bildschirm angezeigt und zeigt die aktuellen Verzeichniseinträge für den lokalen Host.

- Remote-Explorer-Panel: Es wird rechts auf Ihrem Bildschirm angezeigt und zeigt die aktuellen Verzeichniseinträge für den Remote-Host.

- Suchergebnisse-Panel: Abhängig davon, wo Sie nach Dateien suchen (lokal/remote), wird es das lokale oder das Explorer-Panel ersetzen. Dieses Panel zeigt die Einträge an, die der von Ihnen durchgeführten Suchanfrage entsprechen.
  Um das Panel zu wechseln, müssen Sie `<LINKS>` eingeben, um zum Remote-Explorer-Panel zu wechseln, und `<RECHTS>`, um zum lokalen Explorer-Panel zurückzukehren. Wann immer Sie sich im Suchergebnis-Panel befinden, müssen Sie `<ESC>` drücken, um das Panel zu verlassen und zum vorherigen Panel zurückzukehren.

### Tastenkombinationen ⌨

| Taste       | Befehl                                                                 | Erinnerung                                     |
| ----------- | ---------------------------------------------------------------------- | ---------------------------------------------- |
| <ESC>       | Vom Remote-Host trennen; zur Authentifizierungsseite zurückkehren      |                                                |
| <BACKSPACE> | Zum vorherigen Verzeichnis im Stapel wechseln                          |                                                |
| <TAB>       | Explorer-Tab wechseln                                                  |                                                |
| <RECHTS>    | Zum Remote-Explorer-Tab wechseln                                       |                                                |
| <LINKS>     | Zum lokalen Explorer-Tab wechseln                                      |                                                |
| <OBEN>      | Im ausgewählten Eintrag nach oben wechseln                             |                                                |
| <UNTEN>     | Im ausgewählten Eintrag nach unten wechseln                            |                                                |
| <PGUP>      | Im ausgewählten Eintrag um 8 Zeilen nach oben wechseln                 |                                                |
| <PGDOWN>    | Im ausgewählten Eintrag um 8 Zeilen nach unten wechseln                |                                                |
| <ENTER>     | Verzeichnis betreten                                                   |                                                |
| <SPACE>     | Ausgewählte Datei hochladen/herunterladen                              |                                                |
| <BACKTAB>   | Zwischen Protokoll-Tab und Explorer wechseln                           |                                                |
| <A>         | Versteckte Dateien ein-/ausblenden                                     | Alle                                           |
| <B>         | Dateien sortieren nach                                                 | Bubblesort?                                    |
| `<C         | F5>`                                                                   | Datei/Verzeichnis kopieren                     |
| `<D         | F7>`                                                                   | Verzeichnis erstellen                          |
| `<E         | F8                                                                     | DEL>`                                          |
| <F>         | Nach Dateien suchen (Wildcards unterstützt)                            | Finden                                         |
| <G>         | Zum angegebenen Pfad wechseln                                          | Gehe zu                                        |
| `<H         | F1>`                                                                   | Hilfe anzeigen                                 |
| <K>         | Symlink erstellen, der auf den aktuell ausgewählten Eintrag zeigt      | SymlinK                                        |
| <I>         | Informationen über die ausgewählte Datei oder das Verzeichnis anzeigen | Info                                           |
| <L>         | Inhalt des aktuellen Verzeichnisses neu laden / Auswahl löschen        | Liste                                          |
| <M>         | Datei auswählen                                                        | Markieren                                      |
| <N>         | Neue Datei mit angegebenem Namen erstellen                             | Neu                                            |
| `<O         | F4>`                                                                   | Datei bearbeiten; siehe Texteditor             |
| <P>         | Protokoll-Panel öffnen                                                 | Panel                                          |
| `<Q         | F10>`                                                                  | termscp beenden                                |
| `<R         | F6>`                                                                   | Datei umbenennen                               |
| `<S         | F2>`                                                                   | Datei speichern unter...                       |
| <T>         | Änderungen zum ausgewählten Pfad zum Remote-Host synchronisieren       | Track                                          |
| <U>         | Zum übergeordneten Verzeichnis wechseln                                | Übergeordnet                                   |
| `<V         | F3>`                                                                   | Datei mit Standardprogramm für Dateityp öffnen |
| <W>         | Datei mit angegebenem Programm öffnen                                  | Mit                                            |
| <X>         | Befehl ausführen                                                       | Ausführen                                      |
| <Y>         | Synchronisiertes Durchsuchen umschalten                                | sYnc                                           |
| <Z>         | Dateimodus ändern                                                      |                                                |
| <CTRL+A>    | Alle Dateien auswählen                                                 |                                                |
| <ALT+A>     | Alle Dateien abwählen                                                  |                                                |
| <CTRL+C>    | Dateiübertragungsvorgang abbrechen                                     |                                                |
| `<CTRL+S>`  | Gesamte Größe des ausgewählten Pfads abrufen             | Size |
| <CTRL+T>    | Alle synchronisierten Pfade anzeigen                                   | Track                                          |

### Mit mehreren Dateien arbeiten 🥷 

Du kannst mit mehreren Dateien gleichzeitig arbeiten, mit diesen einfachen Tastenkombinationen:

- `<M>`: Datei zur Auswahl markieren
- `<CTRL+A>`: alle Dateien im aktuellen Verzeichnis auswählenas
- `<ALT+A>`: Auswahl aller Dateien aufheben

Markierte Dateien werden **mit hervorgehobenem Hintergrund**  angezeigt.
Bei Auswahlaktionen werden nur die markierten Dateien verarbeitet, das aktuell hervorgehobene Element wird ignoriert.

Auch im Suchergebnis-Panel ist die Mehrfachauswahl möglich.

Alle Aktionen sind bei mehreren Dateien verfügbar, einige funktionieren jedoch leicht anders:

- *Kopieren*: du wirst nach einem Zielnamen gefragt. Bei mehreren Dateien ist das das Zielverzeichnis.
- *Umbenennen*: wie Kopieren, aber verschiebt die Dateien.
- *Speichern unter*: wie Kopieren, aber schreibt die Dateien dorthin.

Wenn du eine Datei in einem Verzeichnis (z. B. `/home`) auswählst und dann das Verzeichnis wechselst, bleibt sie ausgewählt und erscheint in der **Transfer-Warteschlange**  im unteren Panel.
Beim Markieren einer Datei wird das aktuelle *Remote*-Verzeichnis gespeichert; bei einem Transfer wird sie in dieses Verzeichnis übertragen.

#### Beispiel

Wenn wir `/home/a.txt` lokal auswählen und im Remote-Panel in `/tmp` sind, dann zu `/var` wechseln, `/var/b.txt` auswählen und im Remote-Panel in `/home` sind, ergibt der Transfer:

- `/home/a.txt` → `/tmp/a.txt`
- `/var/b.txt` → `/home/b.txt`

### Synchronisiertes Durchsuchen ⏲️

Wenn aktiviert, ermöglicht das synchronisierte Durchsuchen, die Navigation zwischen den beiden Panels zu synchronisieren.
Das bedeutet, dass, wann immer Sie das Arbeitsverzeichnis in einem Panel ändern, dieselbe Aktion im anderen Panel wiederholt wird. Wenn Sie das synchronisierte Durchsuchen aktivieren möchten, drücken Sie einfach `<Y>`; drücken Sie zweimal, um es zu deaktivieren. Während es aktiviert ist, wird der Status des synchronisierten Durchsuchens in der Statusleiste auf `ON` angezeigt.

### Öffnen und Öffnen mit 🚪

Die Befehle Öffnen und Öffnen mit werden von [open-rs](https://docs.rs/crate/open/1.7.0) unterstützt.
Beim Öffnen von Dateien mit dem Befehl Anzeigen (`<V>`) wird die standardmäßige Anwendung für den Dateityp verwendet. Dazu wird der Standarddienst des Betriebssystems verwendet, stellen Sie also sicher, dass mindestens eine dieser Anwendungen auf Ihrem System installiert ist:

- **Windows** -Benutzer: Sie müssen sich keine Sorgen machen, da das Crate den `start`-Befehl verwendet.

- **MacOS** -Benutzer: Sie müssen sich auch keine Sorgen machen, da das Crate `open` verwendet, das bereits auf Ihrem System installiert ist.

- **Linux** -Benutzer: Eines dieser Programme sollte installiert sein

  - _xdg-open_

  - _gio_

  - _gnome-open_

  - _kde-open_

- **WSL** -Benutzer: _wslview_ ist erforderlich, Sie müssen [wslu](https://github.com/wslutilities/wslu) installieren.

> F: Kann ich Remote-Dateien mit dem Befehl Anzeigen bearbeiten?
> A: Nein, zumindest nicht direkt aus dem "Remote-Panel". Sie müssen es zuerst in ein lokales Verzeichnis herunterladen, da beim Öffnen einer Remote-Datei die Datei in ein temporäres Verzeichnis heruntergeladen wird. Es gibt jedoch keine Möglichkeit, einen Wächter für die Datei zu erstellen, um zu überprüfen, wann das Programm, mit dem Sie die Datei geöffnet haben, geschlossen wurde. termscp kann daher nicht wissen, wann Sie mit der Bearbeitung der Datei fertig sind.

---

## Lesezeichen ⭐

In termscp ist es möglich, bevorzugte Hosts zu speichern, die dann schnell aus dem Hauptlayout von termscp geladen werden können.
termscp speichert auch die letzten 16 Hosts, zu denen Sie eine Verbindung hergestellt haben.
Diese Funktion ermöglicht es Ihnen, alle Parameter, die für die Verbindung zu einem bestimmten Remote-Host erforderlich sind, einfach auszuwählen, indem Sie das Lesezeichen im Tab unter dem Authentifizierungsformular auswählen.

Lesezeichen werden, wenn möglich, gespeichert unter:

- `$HOME/.config/termscp/` auf Linux/BSD

- `$HOME/Library/Application Support/termscp` auf MacOS

- `FOLDERID_RoamingAppData\termscp\` auf Windows
  Für Lesezeichen (dies gilt nicht für zuletzt verwendete Hosts) ist es auch möglich, das Passwort zu speichern, das zur Authentifizierung verwendet wird. Das Passwort wird standardmäßig nicht gespeichert und muss beim Speichern eines neuen Lesezeichens über die Eingabeaufforderung angegeben werden.
  Wenn Sie sich Sorgen um die Sicherheit des für Ihre Lesezeichen gespeicherten Passworts machen, lesen Sie bitte das [Kapitel unten 👀](https://chatgpt.com/c/d24fa52b-6f59-4fd7-9b40-d877c758c3de#sind-meine-passwoerter-sicher-) .
  Um ein neues Lesezeichen zu erstellen, folgen Sie einfach diesen Schritten:

1. Geben Sie im Authentifizierungsformular die Parameter ein, um eine Verbindung zu Ihrem Remote-Server herzustellen

2. Drücken Sie `<CTRL+S>`

3. Geben Sie den Namen ein, den Sie dem Lesezeichen geben möchten

4. Wählen Sie, ob das Passwort gespeichert werden soll oder nicht

5. Drücken Sie `<ENTER>`, um zu bestätigen
   Wann immer Sie die zuvor gespeicherte Verbindung verwenden möchten, drücken Sie `<TAB>`, um zur Lesezeichenliste zu navigieren und die Lesezeichenparameter in das Formular zu laden, indem Sie `<ENTER>` drücken.![Lesezeichen](https://github.com/veeso/termscp/blob/main/assets/images/bookmarks.gif?raw=true)

### Sind meine Passwörter sicher 😈

Natürlich 😉.
Wie bereits erwähnt, werden Lesezeichen in Ihrem Konfigurationsverzeichnis zusammen mit Passwörtern gespeichert. Passwörter sind natürlich nicht im Klartext, sie sind mit **AES-128** verschlüsselt. Macht das sie sicher? Absolut! (außer für BSD- und WSL-Benutzer 😢)Unter **Windows** , **Linux** und **MacOS** wird der Schlüssel, der zur Verschlüsselung der Passwörter verwendet wird, falls möglich (aber sollte sein), im _Windows Vault_, im _System-Schlüsselbund_ und im _Schlüsselbund_ gespeichert. Dies ist tatsächlich sehr sicher und wird direkt von Ihrem Betriebssystem verwaltet.❗ Bitte beachten Sie, dass Sie, wenn Sie ein Linux-Benutzer sind, das [Kapitel unten 👀](https://chatgpt.com/c/d24fa52b-6f59-4fd7-9b40-d877c758c3de#linux-schluesselbund) lesen sollten, da der Schlüsselbund auf Ihrem System möglicherweise nicht aktiviert oder unterstützt wird!Auf _BSD_ und _WSL_ hingegen wird der Schlüssel, der zur Verschlüsselung Ihrer Passwörter verwendet wird, auf Ihrer Festplatte gespeichert (unter $HOME/.config/termscp). Es ist daher immer noch möglich, den Schlüssel zum Entschlüsseln von Passwörtern abzurufen. Glücklicherweise garantiert der Speicherort des Schlüssels, dass Ihr Schlüssel nicht von anderen Benutzern gelesen werden kann, aber ja, ich würde das Passwort für einen im Internet exponierten Server trotzdem nicht speichern 😉.

#### Linux-Schlüsselbund

Wir alle lieben Linux aufgrund der Freiheit, die es den Benutzern bietet. Sie können im Wesentlichen alles tun, was Sie als Linux-Benutzer möchten, aber das hat auch einige Nachteile, wie zum Beispiel die Tatsache, dass es oft keine Standardanwendungen über verschiedene Distributionen hinweg gibt. Und das betrifft auch den Schlüsselbund.
Das bedeutet, dass unter Linux möglicherweise kein Schlüsselbund auf Ihrem System installiert ist. Leider erfordert die Bibliothek, die wir verwenden, um mit dem Schlüsselspeicher zu arbeiten, einen Dienst, der `org.freedesktop.secrets` auf D-BUS exponiert, und das Schlimmste daran ist, dass es nur zwei Dienste gibt, die dies tun.

- ❗ Wenn Sie GNOME als Desktop-Umgebung verwenden (z.B. Ubuntu-Benutzer), sollten Sie bereits in Ordnung sein, da der Schlüsselbund bereits von `gnome-keyring` bereitgestellt wird und alles bereits funktionieren sollte.

- ❗ Für Benutzer anderer Desktop-Umgebungen gibt es ein schönes Programm, das Sie verwenden können, um einen Schlüsselbund zu erhalten, nämlich [KeepassXC](https://keepassxc.org/) , das ich auf meiner Manjaro-Installation (mit KDE) verwende und das gut funktioniert. Das einzige Problem ist, dass Sie es einrichten müssen, um es zusammen mit termscp zu verwenden (aber es ist ziemlich einfach). Um mit KeepassXC zu beginnen, lesen Sie mehr [hier](https://chatgpt.com/c/d24fa52b-6f59-4fd7-9b40-d877c758c3de#keepassxc-einrichtung-fuer-termscp) .

- ❗ Was ist, wenn Sie keinen dieser Dienste installieren möchten? Nun, kein Problem! **termscp wird weiterhin wie gewohnt funktionieren** , aber es wird den Schlüssel in einer Datei speichern, wie es normalerweise für BSD und WSL der Fall ist.

##### KeepassXC-Einrichtung für termscp

Befolgen Sie diese Schritte, um KeepassXC für termscp einzurichten:

1. Installieren Sie KeepassXC

2. Gehen Sie in der Symbolleiste zu "Werkzeuge" > "Einstellungen"

3. Wählen Sie "Integration des Geheimdienstes" und aktivieren Sie "KeepassXC freedesktop.org Geheimdienstintegration aktivieren"

4. Erstellen Sie eine Datenbank, falls Sie noch keine haben: In der Symbolleiste "Datenbank" > "Neue Datenbank"

5. In der Symbolleiste: "Datenbank" > "Datenbankeinstellungen"

6. Wählen Sie "Integration des Geheimdienstes" und aktivieren Sie "Einträge unter dieser Gruppe anzeigen"

7. Wählen Sie die Gruppe in der Liste aus, in der das termscp-Geheimnis aufbewahrt werden soll. Denken Sie daran, dass diese Gruppe von jeder anderen Anwendung verwendet werden könnte, um Geheimnisse über DBUS zu speichern.

---

## Konfiguration ⚙️

termscp unterstützt einige benutzerdefinierte Parameter, die in der Konfiguration definiert werden können.
Intern verwendet termscp eine TOML-Datei und einige andere Verzeichnisse, in denen alle Parameter gespeichert werden, aber keine Sorge, Sie werden keine dieser Dateien manuell bearbeiten, da ich es möglich gemacht habe, termscp vollständig über die Benutzeroberfläche zu konfigurieren.

termscp erfordert, wie für Lesezeichen, nur den Zugriff auf diese Pfade:

- `$HOME/.config/termscp/` auf Linux/BSD

- `$HOME/Library/Application Support/termscp` auf MacOS

- `FOLDERID_RoamingAppData\termscp\` auf Windows
  Um auf die Konfiguration zuzugreifen, müssen Sie nur `<CTRL+C>` von der Startseite von termscp drücken.
  Diese Parameter können geändert werden:

- **Texteditor** : Der zu verwendende Texteditor. Standardmäßig findet termscp den Standardeditor für Sie; mit dieser Option können Sie einen Editor zur Verwendung erzwingen (z.B. `vim`). **Auch GUI-Editoren werden unterstützt** , sofern sie sich nicht vom übergeordneten Prozess ablösen (`nohup`). Wenn Sie also fragen: Ja, Sie können `notepad.exe` verwenden, und nein: **Visual Studio Code funktioniert nicht** .

- **Standardprotokoll** : Das Standardprotokoll ist der Standardwert für das in termscp zu verwendende Dateiübertragungsprotokoll. Dies gilt für die Anmeldeseite und für das CLI-Adressargument.

- **Versteckte Dateien anzeigen** : Wählen Sie, ob versteckte Dateien standardmäßig angezeigt werden sollen. Sie können jederzeit zur Laufzeit entscheiden, ob versteckte Dateien angezeigt werden sollen, indem Sie `A` drücken.

- **Auf Updates prüfen** : Wenn auf `Ja` gesetzt, ruft termscp die Github-API ab, um zu überprüfen, ob eine neue Version von termscp verfügbar ist.

- **Aufforderung beim Ersetzen vorhandener Dateien?** : Wenn auf `Ja` gesetzt, fordert termscp Sie zur Bestätigung auf, wenn ein Dateiübertragungsvorgang dazu führt, dass eine vorhandene Datei auf dem Zielhost ersetzt wird.

- **Verzeichnisse gruppieren** : Wählen Sie, ob Verzeichnisse in den Dateiexplorern gruppiert werden sollen. Wenn `Erste anzeigen` ausgewählt ist, werden Verzeichnisse mit der konfigurierten Methode sortiert, aber vor Dateien angezeigt, umgekehrt, wenn `Letzte anzeigen` ausgewählt ist.

- **Remote-Dateiformatierer-Syntax** : Syntax zum Anzeigen von Dateiinformationen für jede Datei im Remote-Explorer. Siehe [Dateiexplorer-Format](https://chatgpt.com/c/d24fa52b-6f59-4fd7-9b40-d877c758c3de#dateiexplorer-format)

- **Lokale Dateiformatierer-Syntax** : Syntax zum Anzeigen von Dateiinformationen für jede Datei im lokalen Explorer. Siehe [Dateiexplorer-Format](https://chatgpt.com/c/d24fa52b-6f59-4fd7-9b40-d877c758c3de#dateiexplorer-format)

- **Benachrichtigungen aktivieren?** : Wenn auf `Ja` gesetzt, werden Benachrichtigungen angezeigt.

- **Benachrichtigungen: Mindestgröße der Übertragung** : Wenn die Übertragungsgröße größer oder gleich dem angegebenen Wert ist, werden Benachrichtigungen für die Übertragung angezeigt. Die akzeptierten Werte sind im Format `{UNSIGNED} B/KB/MB/GB/TB/PB`

- **SSH-Konfigurationspfad** : Legen Sie die SSH-Konfigurationsdatei fest, die beim Herstellen einer Verbindung zu einem SCP/SFTP-Server verwendet werden soll. Wenn nicht festgelegt (leer), wird keine Datei verwendet. Sie können einen Pfad angeben, der mit `~` beginnt, um den Home-Pfad anzugeben (z.B. `~/.ssh/config`). Die von termscp unterstützten Parameter sind [HIER](https://github.com/veeso/ssh2-config#exposed-attributes) angegeben.

### SSH-Schlüssel-Speicherung 🔐

Zusammen mit der Konfiguration bietet termscp auch eine **wesentliche** Funktion für **SFTP/SCP-Clients** : die SSH-Schlüssel-Speicherung.Sie können auf die SSH-Schlüssel-Speicherung zugreifen, indem Sie zur Konfiguration wechseln und zur Registerkarte `SSH-Schlüssel` navigieren. Dort können Sie:

- **Einen neuen Schlüssel hinzufügen** : Drücken Sie einfach `<CTRL+N>` und Sie werden aufgefordert, einen neuen Schlüssel zu erstellen. Geben Sie den Hostnamen/IP-Adresse und den Benutzernamen ein, der mit dem Schlüssel verknüpft ist, und schließlich öffnet sich ein Texteditor: Fügen Sie den **PRIVATEN** SSH-Schlüssel in den Texteditor ein, speichern und beenden Sie.

- **Einen vorhandenen Schlüssel entfernen** : Drücken Sie einfach `<DEL>` oder `<CTRL+E>` auf den Schlüssel, den Sie entfernen möchten, um den Schlüssel dauerhaft aus termscp zu löschen.

- **Einen vorhandenen Schlüssel bearbeiten** : Drücken Sie einfach `<ENTER>` auf den Schlüssel, den Sie bearbeiten möchten, um den privaten Schlüssel zu ändern.

> F: Moment, mein privater Schlüssel ist mit einem Passwort geschützt, kann ich ihn verwenden?
> A: Natürlich können Sie das. Das zur Authentifizierung in termscp angegebene Passwort ist sowohl für die Benutzername/Passwort-Authentifizierung als auch für die RSA-Schlüssel-Authentifizierung gültig.

### Dateiexplorer-Format

Es ist möglich, über die Konfiguration ein benutzerdefiniertes Format für den Dateiexplorer zu definieren. Dies ist sowohl für den lokalen als auch für den Remote-Host möglich, sodass Sie zwei verschiedene Syntaxen verwenden können. Diese Felder mit den Namen `Dateiformatierer-Syntax (lokal)` und `Dateiformatierer-Syntax (remote)` definieren, wie die Dateieinträge im Dateiexplorer angezeigt werden.
Die Syntax für den Formatierer lautet `{SCHLÜSSEL1}... {SCHLÜSSEL2:LÄNGE}... {SCHLÜSSEL3:LÄNGE:EXTRA} {SCHLÜSSELn}...`.
Jeder in Klammern stehende Schlüssel wird durch das zugehörige Attribut ersetzt, während alles außerhalb der Klammern unverändert bleibt.

- Der Schlüsselname ist obligatorisch und muss einer der untenstehenden Schlüssel sein

- Die Länge beschreibt die Länge, die für die Anzeige des Feldes reserviert ist. Statische Attribute unterstützen dies nicht (GRUPPE, PEX, GRÖSSE, BENUTZER)

- Extra wird nur von einigen Parametern unterstützt und ist eine zusätzliche Option. Siehe Schlüssel, um zu überprüfen, ob Extra unterstützt wird.

Dies sind die vom Formatierer unterstützten Schlüssel:

- `ATIME`: Letzte Zugriffszeit (mit Standardsyntax `%b %d %Y %H:%M`); Extra kann als Zeitsyntax angegeben werden (z.B. `{ATIME:8:%H:%M}`)

- `CTIME`: Erstellungszeit (mit Syntax `%b %d %Y %H:%M`); Extra kann als Zeitsyntax angegeben werden (z.B. `{CTIME:8:%H:%M}`)

- `GRUPPE`: Besitzergruppe

- `MTIME`: Letzte Änderungszeit (mit Syntax `%b %d %Y %H:%M`); Extra kann als Zeitsyntax angegeben werden (z.B. `{MTIME:8:%H:%M}`)

- `NAME`: Dateiname (gekürzt, wenn länger als LÄNGE)

- `PFAD`: Absoluter Dateipfad (gekürzt, wenn länger als LÄNGE)

- `PEX`: Dateiberechtigungen (UNIX-Format)

- `GRÖSSE`: Dateigröße (ausgenommen für Verzeichnisse)

- `SYMLINK`: Symlink (falls vorhanden `-> {DATEIPFAD}`)

- `BENUTZER`: Besitzerbenutzer
  Wenn das Feld leer gelassen wird, wird die Standardsyntax des Formatierers verwendet: `{NAME:24} {PEX} {USER} {SIZE} {MTIME:17:%b %d %Y %H:%M}`

---

## Themen 🎨

Termscp bietet Ihnen eine großartige Funktion: die Möglichkeit, die Farben für mehrere Komponenten der Anwendung festzulegen.
Wenn Sie termscp anpassen möchten, gibt es zwei Möglichkeiten, dies zu tun:

- Über das **Konfigurationsmenü**

- Importieren einer **Thema-Datei**
  Um Ihre eigene Anpassung in termscp zu erstellen, müssen Sie nur die Konfiguration von der Authentifizierungsaktivität aus aufrufen, indem Sie `<CTRL+C>` und dann zweimal `<TAB>` drücken. Sie sollten jetzt zum `themen` Panel gewechselt haben.Hier können Sie mit `<OBEN>` und `<UNTEN>` den Stil ändern, den Sie ändern möchten, wie im folgenden GIF gezeigt:![Themen](https://github.com/veeso/termscp/blob/main/assets/images/themes.gif?raw=true)
  termscp unterstützt sowohl die traditionelle explizite Hex-Syntax (`#rrggbb`) als auch die RGB-Syntax `rgb(r, g, b)` zur Angabe von Farben, aber auch \*\*

````markdown
# Benutzerhandbuch 🎓

- [Benutzerhandbuch 🎓](#benutzerhandbuch-)
  - [Verwendung ❓](#verwendung-)
    - [Adressargument 🌎](#adressargument-)
      - [AWS S3 Adressargument](#aws-s3-adressargument)
      - [SMB Adressargument](#smb-adressargument)
      - [Wie das Passwort bereitgestellt werden kann 🔐](#wie-das-passwort-bereitgestellt-werden-kann-)
    - [Unterbefehle](#unterbefehle)
      - [Ein Thema importieren](#ein-thema-importieren)
      - [Neueste Version installieren](#neueste-version-installieren)
  - [S3-Verbindungsparameter](#s3-verbindungsparameter)
    - [S3-Anmeldeinformationen 🦊](#s3-anmeldeinformationen-)
  - [Dateiexplorer 📂](#dateiexplorer-)
    - [Tastenkombinationen ⌨](#tastenkombinationen-)
    - [Mit mehreren Dateien arbeiten 🥷](#mit-mehreren-dateien-arbeiten-)
    - [Synchronisiertes Durchsuchen ⏲️](#synchronisiertes-durchsuchen-️)
    - [Öffnen und Öffnen mit 🚪](#öffnen-und-öffnen-mit-)
  - [Lesezeichen ⭐](#lesezeichen-)
    - [Sind meine Passwörter sicher 😈](#sind-meine-passwoerter-sicher-)
      - [Linux-Schlüsselbund](#linux-schluesselbund)
        - [KeepassXC-Einrichtung für termscp](#keepassxc-einrichtung-fuer-termscp)
  - [Konfiguration ⚙️](#konfiguration-️)
    - [SSH-Schlüssel-Speicherung 🔐](#ssh-schluessel-speicherung-)
    - [Dateiexplorer-Format](#dateiexplorer-format)
  - [Themen 🎨](#themen-)
    - [Mein Thema wird nicht geladen 😱](#mein-thema-wird-nicht-geladen-)
    - [Stile 💈](#stile-)
      - [Authentifizierungsseite](#authentifizierungsseite)
      - [Übertragungsseite](#uebertragungsseite)
      - [Sonstiges](#sonstiges)
  - [Texteditor ✏](#texteditor-)
  - [Protokollierung 🩺](#protokollierung-)
  - [Benachrichtigungen 📫](#benachrichtigungen-)
  - [Dateiwächter 🔭](#dateiwaechter-)

> ❗ Ich benötige Hilfe bei der Übersetzung dieses Handbuchs ins Deutsche. Wenn Sie zur Übersetzung beitragen möchten, öffnen Sie bitte einen PR 🙏

## Verwendung ❓

termscp kann mit den folgenden Optionen gestartet werden:

`termscp [Optionen]... [protokoll://benutzer@adresse:port:arbeitsverzeichnis] [lokales-arbeitsverzeichnis]`

ODER

`termscp [Optionen]... -b [Lesezeichen-Name] [lokales-arbeitsverzeichnis]`

- `-P, --password <Passwort>` wenn Adresse angegeben wird, ist das Passwort dieses Argument
- `-b, --address-as-bookmark` löst das Adressargument als Lesezeichenname auf
- `-q, --quiet` Protokollierung deaktivieren
- `-v, --version` Versionsinformationen anzeigen
- `-h, --help` Hilfeseite anzeigen

termscp kann in drei verschiedenen Modi gestartet werden. Wenn keine zusätzlichen Argumente angegeben werden, zeigt termscp das Authentifizierungsformular an, in dem der Benutzer die erforderlichen Parameter zum Herstellen einer Verbindung mit dem Remote-Peer angeben kann.

Alternativ kann der Benutzer eine Adresse als Argument angeben, um das Authentifizierungsformular zu überspringen und direkt die Verbindung zum Remote-Server zu starten.

Wenn das Adressargument oder der Lesezeichenname angegeben wird, können Sie auch das Startarbeitsverzeichnis für den lokalen Host angeben.

### Adressargument 🌎

Das Adressargument hat die folgende Syntax:

```txt
[protokoll://][benutzername@]<adresse>[:port][:arbeitsverzeichnis]
```
````

Sehen wir uns einige Beispiele für diese besondere Syntax an, da sie sehr komfortabel ist und Sie diese wahrscheinlich anstelle der anderen verwenden werden...

- Verbindung mit dem Standardprotokoll herstellen (_in der Konfiguration definiert_) zu 192.168.1.31, Port, wenn nicht angegeben, ist Standard für das ausgewählte Protokoll (in diesem Fall hängt es von Ihrer Konfiguration ab); Benutzername ist der aktuelle Benutzername

```sh
termscp 192.168.1.31
```

- Verbindung mit dem Standardprotokoll herstellen (_in der Konfiguration definiert_) zu 192.168.1.31; Benutzername ist `root`

```sh
termscp root@192.168.1.31
```

- Verbindung mit scp zu 192.168.1.31, Port ist 4022; Benutzername ist `omar`

```sh
termscp scp://omar@192.168.1.31:4022
```

- Verbindung mit scp zu 192.168.1.31, Port ist 4022; Benutzername ist `omar`. Sie starten im Verzeichnis `/tmp`

```sh
termscp scp://omar@192.168.1.31:4022:/tmp
```

#### AWS S3 Adressargument

AWS S3 hat aus offensichtlichen Gründen eine andere Syntax für CLI-Adressargumente, aber ich habe es geschafft, sie so ähnlich wie möglich an das generische Adressargument anzupassen:

```txt
s3://<bucket-name>@<region>[:profile][:/arbeitsverzeichnis]
```

z.B.

```txt
s3://buckethead@eu-central-1:default:/assets
```

#### SMB Adressargument

SMB hat eine andere Syntax für CLI-Adressargumente, die je nach System unterschiedlich ist:
**Windows** -Syntax:

```txt
\\[benutzername@]<server-name>\<freigabe>[\pfad\...]
```

**Andere Systeme** -Syntax:

```txt
smb://[benutzername@]<server-name>[:port]/<freigabe>[/pfad/.../]
```

#### Wie das Passwort bereitgestellt werden kann 🔐

Sie haben wahrscheinlich bemerkt, dass beim Bereitstellen der Adresse als Argument keine Möglichkeit besteht, das Passwort anzugeben.
Das Passwort kann im Wesentlichen auf drei Arten bereitgestellt werden, wenn das Adressargument angegeben wird:

- `-P, --password` Option: Verwenden Sie einfach diese CLI-Option und geben Sie das Passwort an. Ich rate dringend von dieser Methode ab, da sie sehr unsicher ist (da Sie das Passwort möglicherweise in der Shell-Historie behalten)

- Über `sshpass`: Sie können das Passwort über `sshpass` bereitstellen, z.B. `sshpass -f ~/.ssh/topsecret.key termscp cvisintin@192.168.1.31`

- Sie werden danach gefragt: Wenn Sie keine der vorherigen Methoden verwenden, werden Sie nach dem Passwort gefragt, wie es bei den klassischen Tools wie `scp`, `ssh` usw. der Fall ist.

### Unterbefehle

#### Ein Thema importieren

Führen Sie termscp als `termscp theme <thema-datei>` aus

#### Neueste Version installieren

Führen Sie termscp als `termscp update` aus

---

## S3-Verbindungsparameter

Diese Parameter sind erforderlich, um eine Verbindung zu AWS S3 und anderen S3-kompatiblen Servern herzustellen:

- AWS S3:

  - **Bucket-Name**

  - **Region**

  - _Profil_ (wenn nicht angegeben: "default")

  - _Zugriffsschlüssel_ (sofern nicht öffentlich)

  - _Geheimer Zugriffsschlüssel_ (sofern nicht öffentlich)

  - _Sicherheitstoken_ (falls erforderlich)

  - _Sitzungstoken_ (falls erforderlich)

  - neuer Pfadstil: **NEIN**

- Andere S3-Endpunkte:

  - **Bucket-Name**

  - **Endpunkt**

  - _Zugriffsschlüssel_ (sofern nicht öffentlich)

  - _Geheimer Zugriffsschlüssel_ (sofern nicht öffentlich)

  - neuer Pfadstil: **JA**

### S3-Anmeldeinformationen 🦊

Um eine Verbindung zu einem AWS S3-Bucket herzustellen, müssen Sie offensichtlich einige Anmeldeinformationen angeben.
Es gibt im Wesentlichen drei Möglichkeiten, dies zu erreichen.
Dies sind die Möglichkeiten, wie Sie die Anmeldeinformationen für S3 bereitstellen können:

1. Authentifizierungsformular:

1. Sie können den `access_key` (sollte obligatorisch sein), den `secret_access_key` (sollte obligatorisch sein), `security_token` und den `session_token` angeben.

1. Wenn Sie die S3-Verbindung als Lesezeichen speichern, werden diese Anmeldeinformationen als verschlüsselter AES-256/BASE64-String in Ihrer Lesezeichen-Datei gespeichert (außer dem Sicherheitstoken und dem Sitzungstoken, die als temporäre Anmeldeinformationen gedacht sind).

1. Verwenden Sie Ihre Anmeldeinformationsdatei: Konfigurieren Sie einfach die AWS-CLI über `aws configure` und Ihre Anmeldeinformationen sollten bereits unter `~/.aws/credentials` gespeichert sein. Falls Sie ein anderes Profil als `default` verwenden, geben Sie es im Feld "Profil" im Authentifizierungsformular an.

1. **Umgebungsvariablen** : Sie können Ihre Anmeldeinformationen jederzeit als Umgebungsvariablen angeben. Beachten Sie, dass diese Anmeldeinformationen **immer die im Anmeldeinformationsdatei** angegebenen Anmeldeinformationen überschreiben. So konfigurieren Sie die Umgebung:
   Diese sollten immer obligatorisch sein:

- `AWS_ACCESS_KEY_ID`: AWS-Zugriffsschlüssel-ID (beginnt normalerweise mit `AKIA...`)

- `AWS_SECRET_ACCESS_KEY`: der geheime Zugriffsschlüssel

Falls Sie eine stärkere Sicherheit konfiguriert haben, benötigen Sie möglicherweise auch diese:

- `AWS_SECURITY_TOKEN`: Sicherheitstoken

- `AWS_SESSION_TOKEN`: Sitzungstoken
  ⚠️ Ihre Anmeldeinformationen sind sicher: termscp manipuliert diese Werte nicht direkt! Ihre Anmeldeinformationen werden direkt von der **S3** -Bibliothek verwendet.
  Falls Sie Bedenken hinsichtlich der Sicherheit haben, kontaktieren Sie bitte den Bibliotheksautor auf [Github](https://github.com/durch/rust-s3) ⚠️

---

## Dateiexplorer 📂

Wenn wir in termscp von Dateiexplorern sprechen, meinen wir die Panels, die Sie nach dem Herstellen einer Verbindung mit dem Remote-Host sehen können.
Diese Panels sind im Wesentlichen 3 (ja, tatsächlich drei):

- Lokales Explorer-Panel: Es wird links auf Ihrem Bildschirm angezeigt und zeigt die aktuellen Verzeichniseinträge für den lokalen Host.

- Remote-Explorer-Panel: Es wird rechts auf Ihrem Bildschirm angezeigt und zeigt die aktuellen Verzeichniseinträge für den Remote-Host.

- Suchergebnisse-Panel: Abhängig davon, wo Sie nach Dateien suchen (lokal/remote), wird es das lokale oder das Explorer-Panel ersetzen. Dieses Panel zeigt die Einträge an, die der von Ihnen durchgeführten Suchanfrage entsprechen.
  Um das Panel zu wechseln, müssen Sie `<LINKS>` eingeben, um zum Remote-Explorer-Panel zu wechseln, und `<RECHTS>`, um zum lokalen Explorer-Panel zurückzukehren. Wann immer Sie sich im Suchergebnis-Panel befinden, müssen Sie `<ESC>` drücken, um das Panel zu verlassen und zum vorherigen Panel zurückzukehren.

### Tastenkombinationen ⌨

| Taste       | Befehl                                                                 | Erinnerung                                     |
| ----------- | ---------------------------------------------------------------------- | ---------------------------------------------- |
| <ESC>       | Vom Remote-Host trennen; zur Authentifizierungsseite zurückkehren      |                                                |
| <BACKSPACE> | Zum vorherigen Verzeichnis im Stapel wechseln                          |                                                |
| <TAB>       | Explorer-Tab wechseln                                                  |                                                |
| <RECHTS>    | Zum Remote-Explorer-Tab wechseln                                       |                                                |
| <LINKS>     | Zum lokalen Explorer-Tab wechseln                                      |                                                |
| <OBEN>      | Im ausgewählten Eintrag nach oben wechseln                             |                                                |
| <UNTEN>     | Im ausgewählten Eintrag nach unten wechseln                            |                                                |
| <PGUP>      | Im ausgewählten Eintrag um 8 Zeilen nach oben wechseln                 |                                                |
| <PGDOWN>    | Im ausgewählten Eintrag um 8 Zeilen nach unten wechseln                |                                                |
| <ENTER>     | Verzeichnis betreten                                                   |                                                |
| <SPACE>     | Ausgewählte Datei hochladen/herunterladen                              |                                                |
| <BACKTAB>   | Zwischen Protokoll-Tab und Explorer wechseln                           |                                                |
| <A>         | Versteckte Dateien ein-/ausblenden                                     | Alle                                           |
| <B>         | Dateien sortieren nach                                                 | Bubblesort?                                    |
| `<C         | F5>`                                                                   | Datei/Verzeichnis kopieren                     |
| `<D         | F7>`                                                                   | Verzeichnis erstellen                          |
| `<E         | F8                                                                     | DEL>`                                          |
| <F>         | Nach Dateien suchen (Wildcards unterstützt)                            | Finden                                         |
| <G>         | Zum angegebenen Pfad wechseln                                          | Gehe zu                                        |
| `<H         | F1>`                                                                   | Hilfe anzeigen                                 |
| <K>         | Symlink erstellen, der auf den aktuell ausgewählten Eintrag zeigt      | SymlinK                                        |
| <I>         | Informationen über die ausgewählte Datei oder das Verzeichnis anzeigen | Info                                           |
| <L>         | Inhalt des aktuellen Verzeichnisses neu laden / Auswahl löschen        | Liste                                          |
| <M>         | Datei auswählen                                                        | Markieren                                      |
| <N>         | Neue Datei mit angegebenem Namen erstellen                             | Neu                                            |
| `<O         | F4>`                                                                   | Datei bearbeiten; siehe Texteditor             |
| <P>         | Protokoll-Panel öffnen                                                 | Panel                                          |
| `<Q         | F10>`                                                                  | termscp beenden                                |
| `<R         | F6>`                                                                   | Datei umbenennen                               |
| `<S         | F2>`                                                                   | Datei speichern unter...                       |
| <T>         | Änderungen zum ausgewählten Pfad zum Remote-Host synchronisieren       | Track                                          |
| <U>         | Zum übergeordneten Verzeichnis wechseln                                | Übergeordnet                                   |
| `<V         | F3>`                                                                   | Datei mit Standardprogramm für Dateityp öffnen |
| <W>         | Datei mit angegebenem Programm öffnen                                  | Mit                                            |
| <X>         | Befehl ausführen                                                       | Ausführen                                      |
| <Y>         | Synchronisiertes Durchsuchen umschalten                                | sYnc                                           |
| <Z>         | Dateimodus ändern                                                      |                                                |
| <CTRL+A>    | Alle Dateien auswählen                                                 |                                                |
| <ALT+A>     | Alle Dateien abwählen                                                  |                                                |
| <CTRL+C>    | Dateiübertragungsvorgang abbrechen                                     |                                                |
| <CTRL+T>    | Alle synchronisierten Pfade anzeigen                                   | Track                                          |

### Mit mehreren Dateien arbeiten 🥷

Sie können mit mehreren Dateien arbeiten, indem Sie `<M>` drücken, um die aktuelle Datei auszuwählen, oder `<CTRL+A>`, um alle Dateien im Arbeitsverzeichnis auszuwählen.
Sobald eine Datei zur Auswahl markiert ist, wird sie mit einem `*` auf der linken Seite angezeigt.
Bei der Arbeit mit der Auswahl werden nur die ausgewählten Dateien für Aktionen verarbeitet, während der aktuell hervorgehobene Eintrag ignoriert wird.
Es ist auch möglich, mit mehreren Dateien im Suchergebnis-Panel zu arbeiten.
Alle Aktionen sind verfügbar, wenn Sie mit mehreren Dateien arbeiten, aber beachten Sie, dass einige Aktionen etwas anders funktionieren. Schauen wir uns das genauer an:

- _Kopieren_: Wann immer Sie eine Datei kopieren, werden Sie aufgefordert, den Zielnamen einzugeben. Bei der Arbeit mit mehreren Dateien bezieht sich dieser Name auf das Zielverzeichnis, in dem alle diese Dateien kopiert werden.

- _Umbenennen_: Dasselbe wie Kopieren, aber die Dateien werden dorthin verschoben.

- _Speichern unter_: Dasselbe wie Kopieren, aber die Dateien werden dorthin geschrieben.

### Synchronisiertes Durchsuchen ⏲️

Wenn aktiviert, ermöglicht das synchronisierte Durchsuchen, die Navigation zwischen den beiden Panels zu synchronisieren.
Das bedeutet, dass, wann immer Sie das Arbeitsverzeichnis in einem Panel ändern, dieselbe Aktion im anderen Panel wiederholt wird. Wenn Sie das synchronisierte Durchsuchen aktivieren möchten, drücken Sie einfach `<Y>`; drücken Sie zweimal, um es zu deaktivieren. Während es aktiviert ist, wird der Status des synchronisierten Durchsuchens in der Statusleiste auf `ON` angezeigt.

### Öffnen und Öffnen mit 🚪

Die Befehle Öffnen und Öffnen mit werden von [open-rs](https://docs.rs/crate/open/1.7.0) unterstützt.
Beim Öffnen von Dateien mit dem Befehl Anzeigen (`<V>`) wird die standardmäßige Anwendung für den Dateityp verwendet. Dazu wird der Standarddienst des Betriebssystems verwendet, stellen Sie also sicher, dass mindestens eine dieser Anwendungen auf Ihrem System installiert ist:

- **Windows** -Benutzer: Sie müssen sich keine Sorgen machen, da das Crate den `start`-Befehl verwendet.

- **MacOS** -Benutzer: Sie müssen sich auch keine Sorgen machen, da das Crate `open` verwendet, das bereits auf Ihrem System installiert ist.

- **Linux** -Benutzer: Eines dieser Programme sollte installiert sein

  - _xdg-open_

  - _gio_

  - _gnome-open_

  - _kde-open_

- **WSL** -Benutzer: _wslview_ ist erforderlich, Sie müssen [wslu](https://github.com/wslutilities/wslu) installieren.

> F: Kann ich Remote-Dateien mit dem Befehl Anzeigen bearbeiten?
> A: Nein, zumindest nicht direkt aus dem "Remote-Panel". Sie müssen es zuerst in ein lokales Verzeichnis herunterladen, da beim Öffnen einer Remote-Datei die Datei in ein temporäres Verzeichnis heruntergeladen wird. Es gibt jedoch keine Möglichkeit, einen Wächter für die Datei zu erstellen, um zu überprüfen, wann das Programm, mit dem Sie die Datei geöffnet haben, geschlossen wurde. termscp kann daher nicht wissen, wann Sie mit der Bearbeitung der Datei fertig sind.

---

## Lesezeichen ⭐

In termscp ist es möglich, bevorzugte Hosts zu speichern, die dann schnell aus dem Hauptlayout von termscp geladen werden können.
termscp speichert auch die letzten 16 Hosts, zu denen Sie eine Verbindung hergestellt haben.
Diese Funktion ermöglicht es Ihnen, alle Parameter, die für die Verbindung zu einem bestimmten Remote-Host erforderlich sind, einfach auszuwählen, indem Sie das Lesezeichen im Tab unter dem Authentifizierungsformular auswählen.

Lesezeichen werden, wenn möglich, gespeichert unter:

- `$HOME/.config/termscp/` auf Linux/BSD

- `$HOME/Library/Application Support/termscp` auf MacOS

- `FOLDERID_RoamingAppData\termscp\` auf Windows
  Für Lesezeichen (dies gilt nicht für zuletzt verwendete Hosts) ist es auch möglich, das Passwort zu speichern, das zur Authentifizierung verwendet wird. Das Passwort wird standardmäßig nicht gespeichert und muss beim Speichern eines neuen Lesezeichens über die Eingabeaufforderung angegeben werden.
  Wenn Sie sich Sorgen um die Sicherheit des für Ihre Lesezeichen gespeicherten Passworts machen, lesen Sie bitte das [Kapitel unten 👀](https://chatgpt.com/c/d24fa52b-6f59-4fd7-9b40-d877c758c3de#sind-meine-passwoerter-sicher-) .
  Um ein neues Lesezeichen zu erstellen, folgen Sie einfach diesen Schritten:

1. Geben Sie im Authentifizierungsformular die Parameter ein, um eine Verbindung zu Ihrem Remote-Server herzustellen

2. Drücken Sie `<CTRL+S>`

3. Geben Sie den Namen ein, den Sie dem Lesezeichen geben möchten

4. Wählen Sie, ob das Passwort gespeichert werden soll oder nicht

5. Drücken Sie `<ENTER>`, um zu bestätigen
   Wann immer Sie die zuvor gespeicherte Verbindung verwenden möchten, drücken Sie `<TAB>`, um zur Lesezeichenliste zu navigieren und die Lesezeichenparameter in das Formular zu laden, indem Sie `<ENTER>` drücken.![Lesezeichen](https://github.com/veeso/termscp/blob/main/assets/images/bookmarks.gif?raw=true)

### Sind meine Passwörter sicher 😈

Natürlich 😉.
Wie bereits erwähnt, werden Lesezeichen in Ihrem Konfigurationsverzeichnis zusammen mit Passwörtern gespeichert. Passwörter sind natürlich nicht im Klartext, sie sind mit **AES-128** verschlüsselt. Macht das sie sicher? Absolut! (außer für BSD- und WSL-Benutzer 😢)Unter **Windows** , **Linux** und **MacOS** wird der Schlüssel, der zur Verschlüsselung der Passwörter verwendet wird, falls möglich (aber sollte sein), im _Windows Vault_, im _System-Schlüsselbund_ und im _Schlüsselbund_ gespeichert. Dies ist tatsächlich sehr sicher und wird direkt von Ihrem Betriebssystem verwaltet.❗ Bitte beachten Sie, dass Sie, wenn Sie ein Linux-Benutzer sind, das [Kapitel unten 👀](https://chatgpt.com/c/d24fa52b-6f59-4fd7-9b40-d877c758c3de#linux-schluesselbund) lesen sollten, da der Schlüsselbund auf Ihrem System möglicherweise nicht aktiviert oder unterstützt wird!Auf _BSD_ und _WSL_ hingegen wird der Schlüssel, der zur Verschlüsselung Ihrer Passwörter verwendet wird, auf Ihrer Festplatte gespeichert (unter $HOME/.config/termscp). Es ist daher immer noch möglich, den Schlüssel zum Entschlüsseln von Passwörtern abzurufen. Glücklicherweise garantiert der Speicherort des Schlüssels, dass Ihr Schlüssel nicht von anderen Benutzern gelesen werden kann, aber ja, ich würde das Passwort für einen im Internet exponierten Server trotzdem nicht speichern 😉.

#### Linux-Schlüsselbund

Wir alle lieben Linux aufgrund der Freiheit, die es den Benutzern bietet. Sie können im Wesentlichen alles tun, was Sie als Linux-Benutzer möchten, aber das hat auch einige Nachteile, wie zum Beispiel die Tatsache, dass es oft keine Standardanwendungen über verschiedene Distributionen hinweg gibt. Und das betrifft auch den Schlüsselbund.
Das bedeutet, dass unter Linux möglicherweise kein Schlüsselbund auf Ihrem System installiert ist. Leider erfordert die Bibliothek, die wir verwenden, um mit dem Schlüsselspeicher zu arbeiten, einen Dienst, der `org.freedesktop.secrets` auf D-BUS exponiert, und das Schlimmste daran ist, dass es nur zwei Dienste gibt, die dies tun.

- ❗ Wenn Sie GNOME als Desktop-Umgebung verwenden (z.B. Ubuntu-Benutzer), sollten Sie bereits in Ordnung sein, da der Schlüsselbund bereits von `gnome-keyring` bereitgestellt wird und alles bereits funktionieren sollte.

- ❗ Für Benutzer anderer Desktop-Umgebungen gibt es ein schönes Programm, das Sie verwenden können, um einen Schlüsselbund zu erhalten, nämlich [KeepassXC](https://keepassxc.org/) , das ich auf meiner Manjaro-Installation (mit KDE) verwende und das gut funktioniert. Das einzige Problem ist, dass Sie es einrichten müssen, um es zusammen mit termscp zu verwenden (aber es ist ziemlich einfach). Um mit KeepassXC zu beginnen, lesen Sie mehr [hier](https://chatgpt.com/c/d24fa52b-6f59-4fd7-9b40-d877c758c3de#keepassxc-einrichtung-fuer-termscp) .

- ❗ Was ist, wenn Sie keinen dieser Dienste installieren möchten? Nun, kein Problem! **termscp wird weiterhin wie gewohnt funktionieren** , aber es wird den Schlüssel in einer Datei speichern, wie es normalerweise für BSD und WSL der Fall ist.

##### KeepassXC-Einrichtung für termscp

Befolgen Sie diese Schritte, um KeepassXC für termscp einzurichten:

1. Installieren Sie KeepassXC

2. Gehen Sie in der Symbolleiste zu "Werkzeuge" > "Einstellungen"

3. Wählen Sie "Integration des Geheimdienstes" und aktivieren Sie "KeepassXC freedesktop.org Geheimdienstintegration aktivieren"

4. Erstellen Sie eine Datenbank, falls Sie noch keine haben: In der Symbolleiste "Datenbank" > "Neue Datenbank"

5. In der Symbolleiste: "Datenbank" > "Datenbankeinstellungen"

6. Wählen Sie "Integration des Geheimdienstes" und aktivieren Sie "Einträge unter dieser Gruppe anzeigen"

7. Wählen Sie die Gruppe in der Liste aus, in der das termscp-Geheimnis aufbewahrt werden soll. Denken Sie daran, dass diese Gruppe von jeder anderen Anwendung verwendet werden könnte, um Geheimnisse über DBUS zu speichern.

---

## Konfiguration ⚙️

termscp unterstützt einige benutzerdefinierte Parameter, die in der Konfiguration definiert werden können.
Intern verwendet termscp eine TOML-Datei und einige andere Verzeichnisse, in denen alle Parameter gespeichert werden, aber keine Sorge, Sie werden keine dieser Dateien manuell bearbeiten, da ich es möglich gemacht habe, termscp vollständig über die Benutzeroberfläche zu konfigurieren.

termscp erfordert, wie für Lesezeichen, nur den Zugriff auf diese Pfade:

- `$HOME/.config/termscp/` auf Linux/BSD

- `$HOME/Library/Application Support/termscp` auf MacOS

- `FOLDERID_RoamingAppData\termscp\` auf Windows
  Um auf die Konfiguration zuzugreifen, müssen Sie nur `<CTRL+C>` von der Startseite von termscp drücken.
  Diese Parameter können geändert werden:

- **Texteditor** : Der zu verwendende Texteditor. Standardmäßig findet termscp den Standardeditor für Sie; mit dieser Option können Sie einen Editor zur Verwendung erzwingen (z.B. `vim`). **Auch GUI-Editoren werden unterstützt** , sofern sie sich nicht vom übergeordneten Prozess ablösen (`nohup`). Wenn Sie also fragen: Ja, Sie können `notepad.exe` verwenden, und nein: **Visual Studio Code funktioniert nicht** .

- **Standardprotokoll** : Das Standardprotokoll ist der Standardwert für das in termscp zu verwendende Dateiübertragungsprotokoll. Dies gilt für die Anmeldeseite und für das CLI-Adressargument.

- **Versteckte Dateien anzeigen** : Wählen Sie, ob versteckte Dateien standardmäßig angezeigt werden sollen. Sie können jederzeit zur Laufzeit entscheiden, ob versteckte Dateien angezeigt werden sollen, indem Sie `A` drücken.

- **Auf Updates prüfen** : Wenn auf `Ja` gesetzt, ruft termscp die Github-API ab, um zu überprüfen, ob eine neue Version von termscp verfügbar ist.

- **Aufforderung beim Ersetzen vorhandener Dateien?** : Wenn auf `Ja` gesetzt, fordert termscp Sie zur Bestätigung auf, wenn ein Dateiübertragungsvorgang dazu führt, dass eine vorhandene Datei auf dem Zielhost ersetzt wird.

- **Verzeichnisse gruppieren** : Wählen Sie, ob Verzeichnisse in den Dateiexplorern gruppiert werden sollen. Wenn `Erste anzeigen` ausgewählt ist, werden Verzeichnisse mit der konfigurierten Methode sortiert, aber vor Dateien angezeigt, umgekehrt, wenn `Letzte anzeigen` ausgewählt ist.

- **Remote-Dateiformatierer-Syntax** : Syntax zum Anzeigen von Dateiinformationen für jede Datei im Remote-Explorer. Siehe [Dateiexplorer-Format](https://chatgpt.com/c/d24fa52b-6f59-4fd7-9b40-d877c758c3de#dateiexplorer-format)

- **Lokale Dateiformatierer-Syntax** : Syntax zum Anzeigen von Dateiinformationen für jede Datei im lokalen Explorer. Siehe [Dateiexplorer-Format](https://chatgpt.com/c/d24fa52b-6f59-4fd7-9b40-d877c758c3de#dateiexplorer-format)

- **Benachrichtigungen aktivieren?** : Wenn auf `Ja` gesetzt, werden Benachrichtigungen angezeigt.

- **Benachrichtigungen: Mindestgröße der Übertragung** : Wenn die Übertragungsgröße größer oder gleich dem angegebenen Wert ist, werden Benachrichtigungen für die Übertragung angezeigt. Die akzeptierten Werte sind im Format `{UNSIGNED} B/KB/MB/GB/TB/PB`

- **SSH-Konfigurationspfad** : Legen Sie die SSH-Konfigurationsdatei fest, die beim Herstellen einer Verbindung zu einem SCP/SFTP-Server verwendet werden soll. Wenn nicht festgelegt (leer), wird keine Datei verwendet. Sie können einen Pfad angeben, der mit `~` beginnt, um den Home-Pfad anzugeben (z.B. `~/.ssh/config`). Die von termscp unterstützten Parameter sind [HIER](https://github.com/veeso/ssh2-config#exposed-attributes) angegeben.

### SSH-Schlüssel-Speicherung 🔐

Zusammen mit der Konfiguration bietet termscp auch eine **wesentliche** Funktion für **SFTP/SCP-Clients** : die SSH-Schlüssel-Speicherung.Sie können auf die SSH-Schlüssel-Speicherung zugreifen, indem Sie zur Konfiguration wechseln und zur Registerkarte `SSH-Schlüssel` navigieren. Dort können Sie:

- **Einen neuen Schlüssel hinzufügen** : Drücken Sie einfach `<CTRL+N>` und Sie werden aufgefordert, einen neuen Schlüssel zu erstellen. Geben Sie den Hostnamen/IP-Adresse und den Benutzernamen ein, der mit dem Schlüssel verknüpft ist, und schließlich öffnet sich ein Texteditor: Fügen Sie den **PRIVATEN** SSH-Schlüssel in den Texteditor ein, speichern und beenden Sie.

- **Einen vorhandenen Schlüssel entfernen** : Drücken Sie einfach `<DEL>` oder `<CTRL+E>` auf den Schlüssel, den Sie entfernen möchten, um den Schlüssel dauerhaft aus termscp zu löschen.

- **Einen vorhandenen Schlüssel bearbeiten** : Drücken Sie einfach `<ENTER>` auf den Schlüssel, den Sie bearbeiten möchten, um den privaten Schlüssel zu ändern.

> F: Moment, mein privater Schlüssel ist mit einem Passwort geschützt, kann ich ihn verwenden?
> A: Natürlich können Sie das. Das zur Authentifizierung in termscp angegebene Passwort ist sowohl für die Benutzername/Passwort-Authentifizierung als auch für die RSA-Schlüssel-Authentifizierung gültig.

### Dateiexplorer-Format

Es ist möglich, über die Konfiguration ein benutzerdefiniertes Format für den Dateiexplorer zu definieren. Dies ist sowohl für den lokalen als auch für den Remote-Host möglich, sodass Sie zwei verschiedene Syntaxen verwenden können. Diese Felder mit den Namen `Dateiformatierer-Syntax (lokal)` und `Dateiformatierer-Syntax (remote)` definieren, wie die Dateieinträge im Dateiexplorer angezeigt werden.
Die Syntax für den Formatierer lautet `{SCHLÜSSEL1}... {SCHLÜSSEL2:LÄNGE}... {SCHLÜSSEL3:LÄNGE:EXTRA} {SCHLÜSSELn}...`.
Jeder in Klammern stehende Schlüssel wird durch das zugehörige Attribut ersetzt, während alles außerhalb der Klammern unverändert bleibt.

- Der Schlüsselname ist obligatorisch und muss einer der untenstehenden Schlüssel sein

- Die Länge beschreibt die Länge, die für die Anzeige des Feldes reserviert ist. Statische Attribute unterstützen dies nicht (GRUPPE, PEX, GRÖSSE, BENUTZER)

- Extra wird nur von einigen Parametern unterstützt und ist eine zusätzliche Option. Siehe Schlüssel, um zu überprüfen, ob Extra unterstützt wird.

Dies sind die vom Formatierer unterstützten Schlüssel:

- `ATIME`: Letzte Zugriffszeit (mit Standardsyntax `%b %d %Y %H:%M`); Extra kann als Zeitsyntax angegeben werden (z.B. `{ATIME:8:%H:%M}`)

- `CTIME`: Erstellungszeit (mit Syntax `%b %d %Y %H:%M`); Extra kann als Zeitsyntax angegeben werden (z.B. `{CTIME:8:%H:%M}`)

- `GRUPPE`: Besitzergruppe

- `MTIME`: Letzte Änderungszeit (mit Syntax `%b %d %Y %H:%M`); Extra kann als Zeitsyntax angegeben werden (z.B. `{MTIME:8:%H:%M}`)

- `NAME`: Dateiname (gekürzt, wenn länger als LÄNGE)

- `PFAD`: Absoluter Dateipfad (gekürzt, wenn länger als LÄNGE)

- `PEX`: Dateiberechtigungen (UNIX-Format)

- `GRÖSSE`: Dateigröße (ausgenommen für Verzeichnisse)

- `SYMLINK`: Symlink (falls vorhanden `-> {DATEIPFAD}`)

- `BENUTZER`: Besitzerbenutzer
  Wenn das Feld leer gelassen wird, wird die Standardsyntax des Formatierers verwendet: `{NAME:24} {PEX} {USER} {SIZE} {MTIME:17:%b %d %Y %H:%M}`

---

## Themen 🎨

Termscp bietet Ihnen eine großartige Funktion: die Möglichkeit, die Farben für mehrere Komponenten der Anwendung festzulegen.
Wenn Sie termscp anpassen möchten, gibt es zwei Möglichkeiten, dies zu tun:

- Über das **Konfigurationsmenü**

- Importieren einer **Thema-Datei**
  Um Ihre eigene Anpassung in termscp zu erstellen, müssen Sie nur die Konfiguration von der Authentifizierungsaktivität aus aufrufen, indem Sie `<CTRL+C>` und dann zweimal `<TAB>` drücken. Sie sollten jetzt zum `themen` Panel gewechselt haben.Hier können Sie mit `<OBEN>` und `<UNTEN>` den Stil ändern, den Sie ändern möchten, wie im folgenden GIF gezeigt:![Themen](https://github.com/veeso/termscp/blob/main/assets/images/themes.gif?raw=true)
  termscp unterstützt sowohl die traditionelle explizite Hex-Syntax (`#rrggbb`) als auch die RGB-Syntax `rgb(r, g, b)` zur Angabe von Farben, aber auch [CSS-Farben](https://www.w3schools.com/cssref/css_colors.asp) \*\* (wie `crimson`) werden akzeptiert 😉. Es gibt auch ein spezielles Schlüsselwort, das `Default` lautet. Default bedeutet, dass die verwendete Farbe die Standard-Vordergrund- oder Hintergrundfarbe basierend auf der Situation ist (Vordergrund für Texte und Linien, Hintergrund für, nun, raten Sie mal).Wie bereits erwähnt, können Sie auch Thema-Dateien importieren. Sie können Inspiration aus einem der zusammen mit termscp bereitgestellten Themen ziehen oder diese direkt verwenden, die sich im `themes/` Verzeichnis dieses Repositorys befinden und sie importieren, indem Sie termscp als `termscp -t <thema_datei>` ausführen. Wenn alles in Ordnung war, sollte angezeigt werden, dass das Thema erfolgreich importiert wurde.

### Mein Thema wird nicht geladen 😱

Dies liegt wahrscheinlich an einem kürzlichen Update, das das Thema beschädigt hat. Wann immer ich einen neuen Schlüssel zu den Themen hinzufüge, wird das gespeicherte Thema nicht geladen. Um dieses Problem zu beheben, gibt es zwei schnelle Lösungen:

1. Thema neu laden: Wann immer ich ein Update veröffentliche, werde ich auch die "offiziellen" Themen patchen, sodass Sie es einfach aus dem Repository erneut herunterladen und das Thema erneut über die Option `-t` importieren müssen

```sh
termscp -t <thema.toml>
```

2. Ihr Thema reparieren: Wenn Sie ein benutzerdefiniertes Thema verwenden, können Sie es über `vim` bearbeiten und den fehlenden Schlüssel hinzufügen. Das Thema befindet sich unter `$CONFIG_DIR/termscp/theme.toml`, wobei `$CONFIG_DIR`:

- FreeBSD/GNU-Linux: `$HOME/.config/`

- MacOs: `$HOME/Library/Application Support`

- Windows: `%appdata%`
  ❗ Fehlende Schlüssel werden im CHANGELOG unter `BREAKING CHANGES` für die gerade installierte Version gemeldet.

### Stile 💈

In der folgenden Tabelle finden Sie die Beschreibung für jedes Stilfeld.
Bitte beachten Sie, dass **Stile nicht auf die Konfigurationsseite angewendet werden** , um sicherzustellen, dass sie immer zugänglich bleibt, falls Sie alles durcheinander bringen.

#### Authentifizierungsseite

| Schlüssel      | Beschreibung                                 |
| -------------- | -------------------------------------------- |
| auth_address   | Farbe des Eingabefelds für die IP-Adresse    |
| auth_bookmarks | Farbe des Lesezeichen-Panels                 |
| auth_password  | Farbe des Eingabefelds für das Passwort      |
| auth_port      | Farbe des Eingabefelds für die Portnummer    |
| auth_protocol  | Farbe der Radio-Gruppe für das Protokoll     |
| auth_recents   | Farbe des letzten Panels                     |
| auth_username  | Farbe des Eingabefelds für den Benutzernamen |

#### Übertragungsseite

| Schlüssel                            | Beschreibung                                                                          |
| ------------------------------------ | ------------------------------------------------------------------------------------- |
| transfer_local_explorer_background   | Hintergrundfarbe des lokalen Explorers                                                |
| transfer_local_explorer_foreground   | Vordergrundfarbe des lokalen Explorers                                                |
| transfer_local_explorer_highlighted  | Rahmen- und Hervorhebungsfarbe für den lokalen Explorer                               |
| transfer_remote_explorer_background  | Hintergrundfarbe des Remote-Explorers                                                 |
| transfer_remote_explorer_foreground  | Vordergrundfarbe des Remote-Explorers                                                 |
| transfer_remote_explorer_highlighted | Rahmen- und Hervorhebungsfarbe für den Remote-Explorer                                |
| transfer_log_background              | Hintergrundfarbe für das Protokoll-Panel                                              |
| transfer_log_window                  | Fensterfarbe für das Protokoll-Panel                                                  |
| transfer_progress_bar_partial        | Farbe der teilweisen Fortschrittsanzeige                                              |
| transfer_progress_bar_total          | Farbe der Gesamten Fortschrittsanzeige                                                |
| transfer_status_hidden               | Farbe für den Statusleisten-Eintrag "versteckt"                                       |
| transfer_status_sorting              | Farbe für den Statusleisten-Eintrag "sortieren"; gilt auch für das Dateisortierdialog |
| transfer_status_sync_browsing        | Farbe für den Statusleisten-Eintrag "synchronisiertes Durchsuchen"                    |

#### Sonstiges

Diese Stile gelten für verschiedene Teile der Anwendung.
| Schlüssel | Beschreibung |
| --- | --- |
| misc_error_dialog | Farbe für Fehlermeldungen |
| misc_info_dialog | Farbe für Info-Dialoge |
| misc_input_dialog | Farbe für Eingabe-Dialoge (z.B. Datei kopieren) |
| misc_keys | Farbe des Textes für Tastenkombinationen |
| misc_quit_dialog | Farbe für Beenden-Dialoge |
| misc_save_dialog | Farbe für Speichern-Dialoge |
| misc_warn_dialog | Farbe für Warn-Dialoge |

---

## Texteditor ✏

termscp hat, wie Sie vielleicht bemerkt haben, viele Funktionen, eine davon ist die Möglichkeit, Textdateien anzuzeigen und zu bearbeiten. Es spielt keine Rolle, ob sich die Datei auf dem lokalen Host oder dem Remote-Host befindet, termscp bietet die Möglichkeit, eine Datei in Ihrem bevorzugten Texteditor zu öffnen.
Wenn sich die Datei auf dem Remote-Host befindet, wird die Datei zuerst in Ihr temporäres Verzeichnis heruntergeladen und dann, **nur** wenn Änderungen an der Datei vorgenommen wurden, wieder auf den Remote-Host hochgeladen. termscp überprüft, ob Sie Änderungen an der Datei vorgenommen haben, indem es die letzte Änderungszeit der Datei überprüft.Zur Erinnerung: **Sie können nur Textdateien bearbeiten** ; Binärdateien werden nicht unterstützt.

---

## Protokollierung 🩺

termscp schreibt eine Protokolldatei für jede Sitzung, die unter

- `$HOME/.cache/termscp/termscp.log` auf Linux/BSD

- `$HOME/Library/Caches/termscp/termscp.log` auf MacOS

- `FOLDERID_LocalAppData\termscp\termscp.log` auf Windows
  Die Protokolldatei wird nicht rotiert, sondern nach jedem Start von termscp einfach gekürzt, daher beachten Sie, dass Sie die Protokolldatei an einem sicheren Ort speichern müssen, bevor Sie termscp erneut verwenden, wenn Sie ein Problem melden und Ihre Protokolldatei anhängen möchten.
  Die Protokollierung meldet standardmäßig auf _INFO_-Ebene, sodass sie nicht sehr ausführlich ist.Wenn Sie ein Problem melden möchten, reproduzieren Sie bitte, wenn möglich, das Problem mit auf `TRACE` eingestellter Ebene. Starten Sie dazu termscp mit
  der `-D` CLI-Option.
  Ich weiß, dass Sie möglicherweise einige Fragen zu Protokolldateien haben, daher habe ich eine Art FAQ erstellt:

> Ich möchte keine Protokollierung, kann ich sie ausschalten?Ja, das können Sie. Starten Sie einfach termscp mit der Option `-q oder --quiet`. Sie können termscp aliasen, um es dauerhaft zu machen. Denken Sie daran, dass die Protokollierung verwendet wird, um Probleme zu diagnostizieren, daher könnte es Ihr Weg sein, das Projekt zu unterstützen, indem Sie Protokolldateien behalten 😉. Ich möchte Ihnen kein schlechtes Gewissen einreden, sondern nur sagen.
> Ist die Protokollierung sicher?Wenn Sie sich Sorgen um die Sicherheit machen, enthält die Protokolldatei keine Klartext-Passwörter, also keine Sorge und sie gibt dieselben Informationen weiter, die die Lesezeichendatei `bookmarks` enthält.

## Benachrichtigungen 📫

termscp sendet Desktop-Benachrichtigungen für folgende Ereignisse:

- bei **Übertragung abgeschlossen** : Die Benachrichtigung wird gesendet, sobald eine Übertragung erfolgreich abgeschlossen wurde.

  - ❗ Die Benachrichtigung wird nur angezeigt, wenn die Gesamtgröße der Übertragung mindestens die in der Konfiguration angegebene `Benachrichtigungen: Mindestgröße der Übertragung` beträgt.

- bei **Übertragung fehlgeschlagen** : Die Benachrichtigung wird gesendet, sobald eine Übertragung aufgrund eines Fehlers fehlgeschlagen ist.

  - ❗ Die Benachrichtigung wird nur angezeigt, wenn die Gesamtgröße der Übertragung mindestens die in der Konfiguration angegebene `Benachrichtigungen: Mindestgröße der Übertragung` beträgt.

- bei **Update verfügbar** : Wann immer eine neue Version von termscp verfügbar ist, wird eine Benachrichtigung angezeigt.

- bei **Update installiert** : Wann immer eine neue Version von termscp installiert wurde, wird eine Benachrichtigung angezeigt.

- bei **Update fehlgeschlagen** : Wann immer die Installation des Updates fehlschlägt, wird eine Benachrichtigung angezeigt.
  ❗ Wenn Sie es vorziehen, die Benachrichtigungen ausgeschaltet zu lassen, können Sie einfach das Setup aufrufen und `Benachrichtigungen aktivieren?` auf `Nein` setzen 😉.
  ❗ Wenn Sie die Mindestgröße der Übertragung ändern möchten, um Benachrichtigungen anzuzeigen, können Sie den Wert in der Konfiguration mit dem Schlüssel `Benachrichtigungen: Mindestgröße der Übertragung` ändern und auf das einstellen, was für Sie am besten geeignet ist 🙂.

## Dateiwächter 🔭

Der Dateiwächter ermöglicht es Ihnen, eine Liste von Pfaden einzurichten, die mit den Remote-Hosts synchronisiert werden sollen.
Dies bedeutet, dass wann immer eine Änderung im lokalen Dateisystem im synchronisierten Pfad erkannt wird, die Änderung automatisch innerhalb von 5 Sekunden an den konfigurierten Remote-Host-Pfad gemeldet wird.

Sie können so viele Pfade synchronisieren, wie Sie möchten:

1. Setzen Sie den Cursor im lokalen Explorer auf das Verzeichnis/die Datei, die Sie synchronisieren möchten

2. Gehen Sie zum Verzeichnis, zu dem die Änderungen auf dem Remote-Host gemeldet werden sollen

3. Drücken Sie `<T>`

4. Antworten Sie `<JA>` auf das Radiopopup
   Um die Überwachung zu deaktivieren, drücken Sie einfach `<T>` auf dem lokalen synchronisierten Pfad (oder einem seiner Unterordner)
   ODER Sie können einfach `<CTRL+T>` drücken und `<ENTER>` auf den synchronisierten Pfad, den Sie nicht mehr überwachen möchten.
   Diese Änderungen werden an den Remote-Host gemeldet:

- Neue Dateien, Dateiänderungen

- Datei verschoben/umbenannt

- Datei entfernt/gelöscht

> ❗ Der Wächter arbeitet nur in eine Richtung (lokal > remote). Es ist NICHT möglich, die Änderungen automatisch von remote nach lokal zu synchronisieren.
