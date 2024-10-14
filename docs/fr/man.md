# User manual 🎓

- [User manual 🎓](#user-manual-)
  - [Usage ❓](#usage-)
    - [Argument d'adresse 🌎](#argument-dadresse-)
      - [Argument d'adresse AWS S3](#argument-dadresse-aws-s3)
      - [Argument d'adresse Kube](#argument-dadresse-kube)
      - [Argument d'adresse WebDAV](#argument-dadresse-webdav)
      - [Argument d'adresse SMB](#argument-dadresse-smb)
      - [Comment le mot de passe peut être fourni 🔐](#comment-le-mot-de-passe-peut-être-fourni-)
  - [S3 paramètres de connexion](#s3-paramètres-de-connexion)
    - [Identifiants S3 🦊](#identifiants-s3-)
  - [Explorateur de fichiers 📂](#explorateur-de-fichiers-)
    - [Raccourcis clavier ⌨](#raccourcis-clavier-)
    - [Travailler sur plusieurs fichiers 🥷](#travailler-sur-plusieurs-fichiers-)
    - [Navigation synchronisée ⏲️](#navigation-synchronisée-️)
    - [Ouvrir et ouvrir avec 🚪](#ouvrir-et-ouvrir-avec-)
  - [Signets ⭐](#signets-)
    - [Mes mots de passe sont-ils sûrs 😈](#mes-mots-de-passe-sont-ils-sûrs-)
      - [Linux Keyring](#linux-keyring)
        - [Configuration de KeepassXC pour termscp](#configuration-de-keepassxc-pour-termscp)
  - [Configuration ⚙️](#configuration-️)
    - [SSH Key Storage 🔐](#ssh-key-storage-)
    - [Format de l'explorateur de fichiers](#format-de-lexplorateur-de-fichiers)
  - [Thèmes 🎨](#thèmes-)
    - [Mon thème ne se charge pas 😱](#mon-thème-ne-se-charge-pas-)
    - [Modes 💈](#modes-)
      - [Authentication page](#authentication-page)
      - [Transfer page](#transfer-page)
      - [Misc](#misc)
  - [Éditeur de texte ✏](#éditeur-de-texte-)
  - [Fichier Journal 🩺](#fichier-journal-)
  - [Notifications 📫](#notifications-)
  - [Observateur de fichiers 🔭](#observateur-de-fichiers-)

## Usage ❓

termscp peut être démarré avec les options suivantes :

`termscp [options]... [protocol://user@address:port:wrkdir] [protocol://user@address:port:wrkdir] [local-wrkdir]`

ou

`termscp [options]... -b [bookmark-name] -b [bookmark-name] [local-wrkdir]`

- `-P, --password <password>` si l'adresse est fournie, le mot de passe sera cet argument
- `-b, --address-as-bookmark` résoudre l'argument d'adresse en tant que nom de signet
- `-q, --quiet` Désactiver la journalisation
- `-v, --version` Imprimer les informations sur la version
- `-h, --help` Imprimer la page d'aide

termscp peut être démarré dans deux modes différents, si aucun argument supplémentaire n'est fourni, termscp affichera le formulaire d'authentification, où l'utilisateur pourra fournir les paramètres requis pour se connecter au pair distant.

Alternativement, l'utilisateur peut fournir une adresse comme argument pour ignorer le formulaire d'authentification et démarrer directement la connexion au serveur distant.

Si l'argument d'adresse est fourni, vous pouvez également fournir le répertoire de démarrage de l'hôte local

### Argument d'adresse 🌎

L'argument adresse a la syntaxe suivante :

```txt
[protocole://][nom-utilisateur@]<adresse>[:port][:wrkdir]
```

Voyons un exemple de cette syntaxe particulière, car elle est très confortable et vous allez probablement l'utiliser à la place de l'autre...

- Se connecter en utilisant le protocole par défaut (*défini dans la configuration*) à 192.168.1.31, le port s'il n'est pas fourni est par défaut pour le protocole sélectionné (dans ce cas dépend de votre configuration) ; nom d'utilisateur est le nom de l'utilisateur actuel

    ```sh
    termscp 192.168.1.31
    ```

- Se connecter en utilisant le protocole par défaut (*défini dans la configuration*) à 192.168.1.31 ; le nom d'utilisateur est "root"

    ```sh
    termscp root@192.168.1.31
    ```

- Se connecter en utilisant scp à 192.168.1.31, le port est 4022 ; le nom d'utilisateur est "omar"

    ```sh
    termscp scp://omar@192.168.1.31:4022
    ```

- Se connecter en utilisant scp à 192.168.1.31, le port est 4022 ; le nom d'utilisateur est "omar". Vous commencerez dans le répertoire `/tmp`

    ```sh
    termscp scp://omar@192.168.1.31:4022:/tmp
    ```

#### Argument d'adresse AWS S3

Aws S3 a une syntaxe différente pour l'argument d'adresse CLI, pour des raisons évidentes, mais j'ai réussi à le garder le plus similaire possible à l'argument d'adresse générique :

```txt
s3://<bucket-name>@<region>[:profile][:/wrkdir]
```

e.g.

```txt
s3://buckethead@eu-central-1:default:/assets
```

#### Argument d'adresse Kube

Si vous souhaitez vous connecter à Kube, utilisez la syntaxe suivante

```txt
kube://[namespace][@<cluster_url>][$</path>]
```

#### Argument d'adresse WebDAV

Dans le cas où vous souhaitez vous connecter à WebDAV, utilisez la syntaxe suivante

```txt
http://<username>:<password>@<url></path>
```

ou dans le cas où vous souhaitez utiliser https

```txt
https://<username>:<password>@<url></path>
```

#### Argument d'adresse SMB

SMB a une syntaxe différente pour l'argument d'adresse CLI, qui est différente que vous soyez sur Windows ou sur d'autres systèmes :

syntaxe **Windows**:

```txt
\\[username@]<server-name>\<share>[\path\...]
```

syntaxe **Other systems**:

```txt
smb://[username@]<server-name>[:port]/<share>[/path/.../]
```


#### Comment le mot de passe peut être fourni 🔐

Vous avez probablement remarqué que, lorsque vous fournissez l'adresse comme argument, il n'y a aucun moyen de fournir le mot de passe.
Le mot de passe peut être fourni de 3 manières lorsque l'argument d'adresse est fourni :

- `-P, --password` option : utilisez simplement cette option CLI en fournissant le mot de passe. Je déconseille fortement cette méthode, car elle n'est pas sécurisée (puisque vous pouvez conserver le mot de passe dans l'historique du shell)
- Avec `sshpass`: vous pouvez fournir un mot de passe via `sshpass`, par ex. `sshpass -f ~/.ssh/topsecret.key termscp cvisintin@192.168.1.31`
- Il vous sera demandé : si vous n'utilisez aucune des méthodes précédentes, le mot de passe vous sera demandé, comme c'est le cas avec les outils plus classiques tels que `scp`, `ssh`, etc.

---

## S3 paramètres de connexion

Ces paramètres sont requis pour se connecter à aws s3 et à d'autres serveurs compatibles s3 :

- AWS S3:
  - **bucket name**
  - **region**
  - *profile* (si non fourni : "par défaut")
  - *access key* (sauf si public)
  - *secret access key* (sauf si public)
  - *security token* (si nécessaire)
  - *session token* (si nécessaire)
  - new path style: **NO**
- Autres points de terminaison S3:
  - **bucket name**
  - **endpoint**
  - *access key* (sauf si public)
  - *secret access key* (sauf si public)
  - new path style: **YES**

### Identifiants S3 🦊

Afin de vous connecter à un compartiment Aws S3, vous devez évidemment fournir des informations d'identification.
Il existe essentiellement trois manières d'y parvenir.
Voici donc les moyens de fournir les informations d'identification pour s3 :

1. Authentication form:
   1. Vous pouvez fournir le `access_key` (devrait être obligatoire), le `secret_access_key` (devrait être obligatoire), `security_token` et le `session_token`
   2. Si vous enregistrez la connexion s3 en tant que signet, ces informations d'identification seront enregistrées en tant que chaîne AES-256/BASE64 cryptée dans votre fichier de signets (à l'exception du jeton de sécurité et du jeton de session qui sont censés être des informations d'identification temporaires).
2. Utilisez votre fichier d'informations d'identification : configurez simplement l'AWS cli via `aws configure` et vos informations d'identification doivent déjà se trouver dans `~/.aws/credentials`. Si vous utilisez un profil différent de "default", fournissez-le simplement dans le champ profile du formulaire d'authentification.
3. **Variables d'environnement** : vous pouvez toujours fournir vos informations d'identification en tant que variables d'environnement. Gardez à l'esprit que ces informations d'identification **remplaceront toujours** les informations d'identification situées dans le fichier « credentials ». Voir comment configurer l'environnement ci-dessous :

    Ceux-ci devraient toujours être obligatoires:

    - `AWS_ACCESS_KEY_ID`: aws access key ID (commence généralement par `AKIA...`)
    - `AWS_SECRET_ACCESS_KEY`: la secret access key

    Au cas où vous auriez configuré une sécurité renforcée, vous *pourriez* également en avoir besoin :

    - `AWS_SECURITY_TOKEN`: security token
    - `AWS_SESSION_TOKEN`: session token

⚠️ Vos identifiants sont en sécurité : les termscp ne manipuleront pas ces valeurs directement ! Vos identifiants sont directement consommés par la caisse **s3**.
Si vous avez des inquiétudes concernant la sécurité, veuillez contacter l'auteur de la bibliothèque sur [Github](https://github.com/durch/rust-s3) ⚠️

---

## Explorateur de fichiers 📂

Lorsque nous nous référons aux explorateurs de fichiers en termscp, nous nous référons aux panneaux que vous pouvez voir après avoir établi une connexion avec la télécommande.
Ces panneaux sont essentiellement 3 (oui, trois en fait):

- Panneau de l'explorateur local : il s'affiche sur la gauche de votre écran et affiche les entrées du répertoire en cours pour localhost
- Panneau de l'explorateur distant : il s'affiche à droite de votre écran et affiche les entrées du répertoire en cours pour l'hôte distant.
- Panneau de résultats de recherche : selon l'endroit où vous recherchez des fichiers (local/distant), il remplacera le panneau local ou l'explorateur. Ce panneau affiche les entrées correspondant à la requête de recherche que vous avez effectuée.

Pour changer de panneau, vous devez taper `<LEFT>` pour déplacer le panneau de l'explorateur distant et `<RIGHT>` pour revenir au panneau de l'explorateur local. Chaque fois que vous êtes dans le panneau des résultats de recherche, vous devez appuyer sur `<ESC>` pour quitter le panneau et revenir au panneau précédent.

### Raccourcis clavier ⌨

| Key           | Command                                                                   | Reminder    |
|---------------|---------------------------------------------------------------------------|-------------|
| `<ESC>`       | Se Déconnecter de le serveur; retour à la page d'authentification         |             |
| `<BACKSPACE>` | Aller au répertoire précédent dans la pile                                |             |
| `<TAB>`       | Changer d'onglet explorateur                                              |             |
| `<RIGHT>`     | Déplacer vers l'onglet explorateur distant                                |             |
| `<LEFT>`      | Déplacer vers l'onglet explorateur local                                  |             |
| `<UP>`        | Remonter dans la liste sélectionnée                                       |             |
| `<DOWN>`      | Descendre dans la liste sélectionnée                                      |             |
| `<PGUP>`      | Remonter dans la liste sélectionnée de 8 lignes                           |             |
| `<PGDOWN>`    | Descendre dans la liste sélectionnée de 8 lignes                          |             |
| `<ENTER>`     | Entrer dans le directoire                                                 |             |
| `<SPACE>`     | Télécharger le fichier sélectionné                                        |             |
| `<BACKTAB>`   | Basculer entre l'onglet journal et l'explorateur                          |             |
| `<A>`         | Basculer les fichiers cachés                                              | All         |
| `<B>`         | Trier les fichiers par                                                    | Bubblesort? |
| `<C|F5>`      | Copier le fichier/répertoire                                              | Copy        |
| `<D|F7>`      | Créer un dossier                                                          | Directory   |
| `<E|F8|DEL>`  | Supprimer le fichier (Identique à `DEL`)                                  | Erase       |
| `<F>`         | Rechercher des fichiers                                                   | Find        |
| `<G>`         | Aller au chemin fourni                                                    | Go to       |
| `<H|F1>`      | Afficher l'aide                                                           | Help        |
| `<I>`         | Afficher les informations sur le fichier ou le dossier sélectionné        | Info        |
| `<K>`         | Créer un lien symbolique pointant vers l'entrée actuellement sélectionnée | symlinK     |
| `<L>`         | Recharger le contenu du répertoire actuel / Effacer la sélection          | List        |
| `<M>`         | Sélectionner un fichier                                                   | Mark        |
| `<N>`         | Créer un nouveau fichier avec le nom fourni                               | New         |
| `<O|F4>`      | Modifier le fichier                                                       | Open        |
| `<P>`         | Ouvre le panel de journals                                                | Panel       |
| `<Q|F10>`     | Quitter termscp                                                           | Quit        |
| `<R|F6>`      | Renommer le fichier                                                       | Rename      |
| `<S|F2>`      | Enregistrer le fichier sous...                                            | Save        |
| `<T>`         | Synchroniser les modifications apportées au chemin sélectionné            | Track       |
| `<U>`         | Aller dans le répertoire parent                                           | Upper       |
| `<V|F3>`      | Ouvrir le fichier avec le programme défaut pour le type de fichier        | View        |
| `<W>`         | Ouvrir le fichier avec le programme spécifié                              | With        |
| `<X>`         | Exécuter une commande                                                     | eXecute     |
| `<Y>`         | Basculer la navigation synchronisée                                       | sYnc        |
| `<Z>`         | Changer permissions de fichier                                            |             |
| `</>`         | Filtrer les fichiers (les expressions régulières et les correspondances génériques sont prises en charge)    |             |
| `<CTRL+A>`    | Sélectionner tous les fichiers                                            |             |
| `<ALT+A>`    | Desélectionner tous les fichiers                                            |             |
| `<CTRL+C>`    | Abandonner le processus de transfert de fichiers                          |             |
| `<CTRL+T>`    | Afficher tous les chemins synchronisés                                    | Track       |

### Travailler sur plusieurs fichiers 🥷

Vous pouvez choisir de travailler sur plusieurs fichiers, en les sélectionnant en appuyant sur `<M>`, afin de sélectionner le fichier actuel, ou en appuyant sur `<CTRL+A>`, ce qui sélectionnera tous les fichiers dans le répertoire de travail.
Une fois qu'un fichier est marqué pour la sélection, il sera affiché avec un `*` sur la gauche.
Lorsque vous travaillez sur la sélection, seul le fichier sélectionné sera traité pour les actions, tandis que l'élément en surbrillance actuel sera ignoré.
Il est également possible de travailler sur plusieurs fichiers dans le panneau des résultats de recherche.
Toutes les actions sont disponibles lorsque vous travaillez avec plusieurs fichiers, mais sachez que certaines actions fonctionnent de manière légèrement différente. Plongeons dans:

- *Copy*: chaque fois que vous copiez un fichier, vous serez invité à insérer le nom de destination. Lorsque vous travaillez avec plusieurs fichiers, ce nom fait référence au répertoire de destination où tous ces fichiers seront copiés.
- *Rename*: identique à la copie, mais y déplacera les fichiers.
- *Save as*: identique à la copie, mais les y écrira.

### Navigation synchronisée ⏲️

Lorsqu'elle est activée, la navigation synchronisée vous permettra de synchroniser la navigation entre les deux panneaux.
Cela signifie que chaque fois que vous changerez de répertoire de travail sur un panneau, la même action sera reproduite sur l'autre panneau. Si vous souhaitez activer la navigation synchronisée, appuyez simplement sur `<Y>` ; appuyez deux fois pour désactiver. Lorsqu'il est activé, l'état de navigation synchronisé sera signalé dans la barre d'état sur `ON`

### Ouvrir et ouvrir avec 🚪

Lors de l'ouverture de fichiers avec la commande Afficher (`<V>`), l'application par défaut du système pour le type de fichier sera utilisée. Pour ce faire, le service du système d'exploitation par défaut sera utilisé, alors assurez-vous d'avoir au moins l'un de ceux-ci installé sur votre système :

- Utilisateurs **Windows** : vous n'avez pas à vous en soucier, puisque la caisse utilisera la commande `start`.
- Utilisateurs **MacOS** : vous n'avez pas à vous inquiéter non plus, puisque le crate utilisera `open`, qui est déjà installé sur votre système.
- Utilisateurs **Linux** : l'un d'eux doit être installé
  - *xdg-open*
  - *gio*
  - *gnome-open*
  - *kde-open*
- Utilisateurs **WSL** : *wslview* est requis, vous devez installer [wslu](https://github.com/wslutilities/wslu).

> Q: Puis-je modifier des fichiers distants à l'aide de la commande view ?  
> A: Non, du moins pas directement depuis le "panneau distant". Vous devez d'abord le télécharger dans un répertoire local, cela est dû au fait que lorsque vous ouvrez un fichier distant, le fichier est téléchargé dans un répertoire temporaire, mais il n'y a aucun moyen de créer un observateur pour que le fichier vérifie quand le programme que vous utilisé pour l'ouvrir était fermé, donc termscp n'est pas en mesure de savoir quand vous avez fini de modifier le fichier.

---

## Signets ⭐

Dans termscp, il est possible de sauvegarder les hôtes favoris, qui peuvent ensuite être chargés rapidement à partir de la mise en page principale de termscp.
termscp enregistrera également les 16 derniers hôtes auxquels vous vous êtes connecté.
Cette fonctionnalité vous permet de charger tous les paramètres nécessaires pour vous connecter à une certaine télécommande, en sélectionnant simplement le signet dans l'onglet sous le formulaire d'authentification.

Les signets seront enregistrés, si possible à l'adresse :

- `$HOME/.config/termscp/` sous Linux/BSD
- `$HOME/Library/Application Support/termscp` sous MacOs
- `FOLDERID_RoamingAppData\termscp\` sous Windows

Pour les signets uniquement (cela ne s'appliquera pas aux hôtes récents), il est également possible de sauvegarder le mot de passe utilisé pour s'authentifier. Le mot de passe n'est pas enregistré par défaut et doit être spécifié via l'invite lors de l'enregistrement d'un nouveau signet.
Si vous êtes préoccupé par la sécurité du mot de passe enregistré pour vos favoris, veuillez lire le [chapitre ci-dessous 👀](#mes-mots-de-passe-sont-ils-sûrs-).

Pour créer un nouveau signet, suivez simplement ces étapes :

1. Tapez dans le formulaire d'authentification les paramètres pour vous connecter à votre serveur distant
2. Appuyez sur `<CTRL+S>`
3. Tapez le nom que vous souhaitez donner au signet
4. Choisissez de rappeler ou non le mot de passe
5. Appuyez sur `<ENTER>` pour soumettre

chaque fois que vous souhaitez utiliser la connexion précédemment enregistrée, appuyez simplement sur `<TAB>` pour accéder à la liste des signets et chargez les paramètres des signets dans le formulaire en appuyant sur `<ENTER>`.

![Bookmarks](https://github.com/veeso/termscp/blob/main/assets/images/bookmarks.gif?raw=true)

### Mes mots de passe sont-ils sûrs 😈

Bien sûr 😉.
Comme dit précédemment, les signets sont enregistrés dans votre répertoire de configuration avec les mots de passe. Les mots de passe ne sont évidemment pas en texte brut, ils sont cryptés avec **AES-128**. Est-ce que cela les sécurise ? Absolument! (sauf pour les utilisateurs BSD et WSL 😢)

Sous **Windows**, **Linux** et **MacOS**, la clé utilisée pour crypter les mots de passe est stockée, si possible (mais devrait l'être), respectivement dans le *Windows Vault*, dans le *porte-clés système* et dans le *Porte-clés*. Ceci est en fait super sûr et est directement géré par votre système d'exploitation.

❗ Veuillez noter que si vous êtes un utilisateur Linux, vous feriez mieux de lire le [chapitre ci-dessous 👀](#linux-keyring), car le trousseau peut ne pas être activé ou pris en charge sur votre système !

Sur *BSD* et *WSL*, en revanche, la clé utilisée pour crypter vos mots de passe est stockée sur votre disque (dans $HOME/.config/termscp). Il est alors, toujours possible de récupérer la clé pour déchiffrer les mots de passe. Heureusement, l'emplacement de la clé garantit que votre clé ne peut pas être lue par des utilisateurs différents du vôtre, mais oui, je n'enregistrerais toujours pas le mot de passe pour un serveur exposé sur Internet 😉.

#### Linux Keyring

Nous aimons tous Linux grâce à la liberté qu'il donne aux utilisateurs. En tant qu'utilisateur Linux, vous pouvez essentiellement faire tout ce que vous voulez, mais cela présente également des inconvénients, tels que le fait qu'il n'y a souvent pas d'applications standard dans différentes distributions. Et cela implique aussi un porte-clés.
Cela signifie que sous Linux, aucun trousseau de clés n'est peut-être installé sur votre système. Malheureusement, la bibliothèque que nous utilisons pour travailler avec le stockage des clés nécessite un service qui expose `org.freedesktop.secrets` sur D-BUS et le pire est qu'il n'y a que deux services qui l'exposent.

- ❗ Si vous utilisez GNOME comme environnement de bureau (par exemple, les utilisateurs d'ubuntu), ça devrait déjà aller, car le trousseau de clés est déjà fourni par `gnome-keyring` et tout devrait déjà fonctionner.
- ❗ Pour les autres utilisateurs d'environnement de bureau, il existe un programme sympa que vous pouvez utiliser pour obtenir un trousseau de clés qui est [KeepassXC](https://keepassxc.org/), que j'utilise sur mon installation Manjaro (avec KDE) et qui fonctionne bien. Le seul problème est que vous devez le configurer pour qu'il soit utilisé avec termscp (mais c'est assez simple). Pour commencer avec KeepassXC, lisez la suite [ici](#configuration-de-keepassxc-pour-termscp).
- ❗ Et si vous ne souhaitez installer aucun de ces services ? Eh bien, il n'y a pas de problème ! **termscp continuera à fonctionner comme d'habitude**, mais il enregistrera la clé dans un fichier, comme il le fait habituellement pour BSD et WSL.

##### Configuration de KeepassXC pour termscp

Suivez ces étapes afin de configurer keepassXC pour termscp :

1. Installer KeepassXC
2. Allez dans "outils" > "paramètres" dans la barre d'outils
3. Selectioner "Secret service integration" et basculer "Enable KeepassXC freedesktop.org secret service integration"
4. Creer une base de données, si vous n'en avez pas encore : à partir de la barre d'outils "Database" > "New database"
5. De la barre d'outils: "Database" > "Database settings"
6. Selectioner "Secret service integration" et basculer "Expose entries under this group"
7. Sélectionnez le groupe dans la liste où vous souhaitez conserver le secret du termscp. N'oubliez pas que ce groupe peut être utilisé par toute autre application pour stocker des secrets via DBUS.

---

## Configuration ⚙️

termscp prend en charge certains paramètres définis par l'utilisateur, qui peuvent être définis dans la configuration.
Underhood termscp a un fichier TOML et quelques autres répertoires où tous les paramètres seront enregistrés, mais ne vous inquiétez pas, vous ne toucherez à aucun de ces fichiers manuellement, car j'ai rendu possible la configuration complète de termscp à partir de son interface utilisateur.

termscp, comme pour les signets, nécessite juste d'avoir ces chemins accessibles :

- `$HOME/.config/termscp/` sous Linux/BSD
- `$HOME/Library/Application Support/termscp` sous MacOs
- `FOLDERID_RoamingAppData\termscp\` sous Windows

Pour accéder à la configuration, il vous suffit d'appuyer sur `<CTRL+C>` depuis l'accueil de termscp.

Ces paramètres peuvent être modifiés :

- **Text Editor**: l'éditeur de texte à utiliser. Par défaut, termscp trouvera l'éditeur par défaut pour vous ; avec cette option, vous pouvez forcer l'utilisation d'un éditeur (par exemple `vim`). **Les éditeurs d'interface graphique sont également pris en charge**, à moins qu'ils ne soient `nohup` à partir du processus parent.
- **Default Protocol**: le protocole par défaut est la valeur par défaut du protocole de transfert de fichiers à utiliser dans termscp. Cela s'applique à la page de connexion et à l'argument de l'adresse CLI.
- **Show Hidden Files**: sélectionnez si les fichiers cachés doivent être affichés par défaut. Vous pourrez décider d'afficher ou non les fichiers cachés au moment de l'exécution en appuyant sur `A` de toute façon.
- **Check for updates**: s'il est défini sur `yes`, Termscp récupère l'API Github pour vérifier si une nouvelle version de Termscp est disponible.
- **Prompt when replacing existing files?**: S'il est défini sur `yes`, Termscp vous demandera une confirmation chaque fois qu'un transfert de fichier entraînera le remplacement d'un fichier existant sur l'hôte cible.
- **Group Dirs**: sélectionnez si les répertoires doivent être regroupés ou non dans les explorateurs de fichiers. Si `Display first` est sélectionné, les répertoires seront triés en utilisant la méthode configurée mais affichés avant les fichiers, vice-versa si `Display last` est sélectionné.
- **Remote File formatter syntax**: syntaxe pour afficher les informations de fichier pour chaque fichier dans l'explorateur distant. Voir [File explorer format](#format-de-lexplorateur-de-fichiers)
- **Local File formatter syntax**: syntaxe pour afficher les informations de fichier pour chaque fichier dans l'explorateur local. Voir [File explorer format](#format-de-lexplorateur-de-fichiers)
- **Enable notifications?**: S'il est défini sur `Yes`, les notifications seront affichées.
- **Notifications: minimum transfer size**: si la taille du transfert est supérieure ou égale à la valeur spécifiée, les notifications de transfert seront affichées. Les valeurs acceptées sont au format `{UNSIGNED} B/KB/MB/GB/TB/PB`
- **SSH configuration path** : définissez le fichier de configuration SSH à utiliser lors de la connexion à un serveur SCP/SFTP. S'il n'est pas défini (vide), aucun fichier ne sera utilisé. Vous pouvez spécifier un chemin commençant par `~` pour indiquer le chemin d'accueil (par exemple `~/.ssh/config`). Les paramétrages disponibles pour la configuration sont listées [ICI](https://github.com/veeso/ssh2-config#exposed-attributes).

### SSH Key Storage 🔐

n plus de la configuration, termscp fournit également une fonctionnalité **essentielle** pour les **clients SFTP/SCP** : le stockage de clés SSH.

Vous pouvez accéder au stockage des clés SSH, de la configuration à l'onglet « Clés SSH », une fois là-bas, vous pouvez :

- **Ajouter une neuf clé SSH**: appuyez simplement sur `<CTRL+N>` et vous serez invité à créer une nouvelle clé. Fournissez le nom d'hôte/l'adresse IP et le nom d'utilisateur associé à la clé et enfin un éditeur de texte s'ouvrira : collez la clé ssh **PRIVÉE** dans l'éditeur de texte, enregistrez et quittez.
- **Supprimer une clé existante**: appuyez simplement sur `<DEL>` ou `<CTRL+E>` sur la clé que vous souhaitez supprimer, pour supprimer de manière persistante la clé de termscp.
- **Modifier une clé existante**: appuyez simplement sur `<ENTER>` sur la clé que vous souhaitez modifier, pour changer la clé privée.

> Q: Ma clé privée est protégée par mot de passe, puis-je l'utiliser ?
> A: Bien sûr vous pouvez. Le mot de passe fourni pour l'authentification dans termscp est valide à la fois pour l'authentification par nom d'utilisateur/mot de passe et pour l'authentification par clé RSA.

### Format de l'explorateur de fichiers

Il est possible via la configuration de définir un format personnalisé pour l'explorateur de fichiers. Ceci est possible à la fois pour l'hôte local et distant, vous pouvez donc utiliser deux syntaxes différentes. Ces champs, nommés `File formatter syntax (local)` et `File formatter syntax (remote)` définiront comment les entrées de fichier seront affichées dans l'explorateur de fichiers.
La syntaxe du formateur est la suivante `{KEY1}... {KEY2:LENGTH}... {KEY3:LENGTH:EXTRA} {KEYn}...`.
Chaque clé entre crochets sera remplacée par l'attribut associé, tandis que tout ce qui se trouve en dehors des crochets restera inchangé.

- Le nom de la clé est obligatoire et doit être l'une des clés ci-dessous
- La longueur décrit la longueur réservée pour afficher le champ. Les attributs statiques ne prennent pas en charge cela (GROUP, PEX, SIZE, USER)
- Extra n'est pris en charge que par certains paramètres et constitue une option supplémentaire. Voir les touches pour vérifier si les extras sont pris en charge.

Voici les clés prises en charge par le formateur :

- `ATIME`: Heure du dernier accès (avec la syntaxe par défaut `%b %d %Y %H:%M`) ; Un supplément peut être fourni comme syntaxe de l'heure (par exemple, `{ATIME:8:%H:%M}`)
- `CTIME`: Heure de création (avec la syntaxe `%b %d %Y %H:%M`); Un supplément peut être fourni comme syntaxe de l'heure (par exemple, `{CTIME:8:%H:%M}`)
- `GROUP`: Groupe de propriétaires
- `MTIME`: Heure du dernier changement (avec la syntaxe `%b %d %Y %H:%M`); Un supplément peut être fourni comme syntaxe de l'heure (par exemple, `{MTIME:8:%H:%M}`)
- `NAME`: Nom du fichier (élidé si plus long que LENGTH)
- `PATH`: Chemin absolu du fichier (les dossiers entre la racine et les premiers ancêtres sont éludés s'ils sont plus longs que LENGTH)
- `PEX`: Autorisations de fichiers (format UNIX)
- `SIZE`: Taille du fichier (omis pour les répertoires)
- `SYMLINK`: Lien symbolique (le cas échéant `-> {FILE_PATH}`)
- `USER`: Utilisateur propriétaire

Si elle est laissée vide, la syntaxe par défaut du formateur sera utilisée : `{NAME:24} {PEX} {USER} {SIZE} {MTIME:17:%b %d %Y %H:%M}`

---

## Thèmes 🎨

Termscp vous offre une fonctionnalité géniale : la possibilité de définir les couleurs de plusieurs composants de l'application.
Si vous souhaitez personnaliser termscp, il existe deux manières de le faire :

- Depuis le **menu de configuration**
- Importation d'un **fichier de thème**

Afin de créer votre propre personnalisation à partir de termscp, il vous suffit de saisir la configuration à partir de l'activité d'authentification, en appuyant sur `<CTRL+C>` puis sur `<TAB>` deux fois. Vous devriez être maintenant passé au panneau `thèmes`.

Ici, vous pouvez vous déplacer avec `<UP>` et `<DOWN>` pour changer le style que vous souhaitez modifier, comme indiqué dans le gif ci-dessous :

![Themes](https://github.com/veeso/termscp/blob/main/assets/images/themes.gif?raw=true)

termscp prend en charge à la fois la syntaxe hexadécimale explicite traditionnelle (`#rrggbb`) et rgb `rgb(r, g, b)` pour fournir des couleurs, mais aussi **[couleurs css](https://www.w3schools.com/cssref/css_colors.asp)** (comme `crimson`) sont acceptés 😉. Il y a aussi un keywork spécial qui est `Default`. Par défaut signifie que la couleur utilisée sera la couleur de premier plan ou d'arrière-plan par défaut en fonction de la situation (premier plan pour les textes et les lignes, arrière-plan pour bien, devinez quoi)

Comme dit précédemment, vous pouvez également importer des fichiers de thème. Vous pouvez vous inspirer de ou utiliser directement l'un des thèmes fournis avec termscp, situé dans le répertoire `themes/` de ce référentiel et les importer en exécutant termscp en tant que `termscp -t <theme_file>`. Si tout allait bien, cela devrait vous dire que le thème a été importé avec succès.

### Mon thème ne se charge pas 😱

Cela est probablement dû à une mise à jour récente qui a cassé le thème. Chaque fois que j'ajoute une nouvelle clé aux thèmes, le thème enregistré ne se charge pas. Pour résoudre ces problèmes, il existe deux solutions vraiment rapides :

1. Recharger le thème : chaque fois que je publie une mise à jour, je corrige également les thèmes "officiels", il vous suffit donc de le télécharger à nouveau depuis le référentiel et de réimporter le thème via l'option `-t`

    ```sh
    termscp -t <theme.toml>
    ```

2. Corrigez votre thème : si vous utilisez un thème personnalisé, vous pouvez le modifier via `vim` et ajouter la clé manquante. Le thème est situé dans `$CONFIG_DIR/termscp/theme.toml` où `$CONFIG_DIR` est :

    - FreeBSD/GNU-Linux: `$HOME/.config/`
    - MacOs: `$HOME/Library/Application Support`
    - Windows: `%appdata%`

    ❗ Les clés manquantes sont signalées dans le CHANGELOG sous `BREAKING CHANGES` pour la version que vous venez d'installer.

### Modes 💈

Vous pouvez trouver dans le tableau ci-dessous, la description de chaque champ de style.
Veuillez noter que **les styles ne s'appliqueront pas à la page de configuration**, afin de la rendre toujours accessible au cas où vous gâcheriez tout

#### Authentication page

| Key            | Description                              |
|----------------|------------------------------------------|
| auth_address   | Couleur du champ pour adresse IP         |
| auth_bookmarks | Couleur du panneau des signets           |
| auth_password  | Couleur du champ pour mot de passe       |
| auth_port      | Couleur du champ pour nombre de port     |
| auth_protocol  | Couleur du groupe radio pour protocole   |
| auth_recents   | Couleur du panneau récent                |
| auth_username  | Couleur du champ pour nom d'utilisateur  |

#### Transfer page

| Key                                  | Description                                                               |
|--------------------------------------|---------------------------------------------------------------------------|
| transfer_local_explorer_background   | Couleur d'arrière-plan de l'explorateur localhost                         |
| transfer_local_explorer_foreground   | Couleur de premier plan de l'explorateur localhost                        |
| transfer_local_explorer_highlighted  | Bordure et couleur surlignée pour l'explorateur localhost                 |
| transfer_remote_explorer_background  | Couleur d'arrière-plan de l'explorateur distant                           |
| transfer_remote_explorer_foreground  | Couleur de premier plan de l'explorateur distant                          |
| transfer_remote_explorer_highlighted | Bordure et couleur en surbrillance pour l'explorateur distant             |
| transfer_log_background              | Couleur d'arrière-plan du panneau de journal                              |
| transfer_log_window                  | Couleur de la fenêtre du panneau de journal                               |
| transfer_progress_bar_partial        | Couleur de la barre de progression partielle                              |
| transfer_progress_bar_total          | Couleur de la barre de progression totale                                 |
| transfer_status_hidden               | Couleur de l'étiquette "hidden" de la barre d'état                        |
| transfer_status_sorting              | Couleur de l'étiquette "sorting" de la barre d'état                       |
| transfer_status_sync_browsing        | Couleur de l'étiquette "sync browsing" de la barre d'état                 |

#### Misc

These styles applie to different part of the application.

| Key               | Description                                 |
|-------------------|---------------------------------------------|
| misc_error_dialog | Couleur des messages d'erreur               |
| misc_info_dialog  | Couleur des messages d'info                 |
| misc_input_dialog | Couleur des messages de input               |
| misc_keys         | Couleur du texte pour les frappes de touches|
| misc_quit_dialog  | Couleur des messages de quit                |
| misc_save_dialog  | Couleur des messages d'enregistrement       |
| misc_warn_dialog  | Couleur des messages de attention           |

---

## Éditeur de texte ✏

termscp a, comme vous l'avez peut-être remarqué, de nombreuses fonctionnalités, l'une d'entre elles est la possibilité de visualiser et de modifier un fichier texte. Peu importe que le fichier se trouve sur l'hôte local ou sur l'hôte distant, termscp offre la possibilité d'ouvrir un fichier dans votre éditeur de texte préféré.
Si le fichier se trouve sur l'hôte distant, le fichier sera d'abord téléchargé dans votre répertoire de fichiers temporaires, puis **uniquement** si des modifications ont été apportées au fichier, rechargé sur l'hôte distant. termscp vérifie si vous avez apporté des modifications au fichier en vérifiant l'heure de la dernière modification du fichier.

> ❗ Juste un rappel : **vous ne pouvez éditer que des fichiers texte** ; les fichiers binaires ne sont pas pris en charge.

---

## Fichier Journal 🩺

termscp écrit un fichier journal pour chaque session, qui est écrit à

- `$HOME/.cache/termscp/termscp.log` sous Linux/BSD
- `$HOME/Library/Caches/termscp/termscp.log` sous MacOs
- `FOLDERID_LocalAppData\termscp\termscp.log` sous Windows

le journal ne sera pas tourné, mais sera simplement tronqué après chaque lancement de termscp, donc si vous souhaitez signaler un problème et que vous souhaitez joindre votre fichier journal, n'oubliez pas de sauvegarder le fichier journal dans un endroit sûr avant de l'utiliser termescp à nouveau.

La journalisation par défaut se rapporte au niveau *INFO*, elle n'est donc pas très détaillée.

Si vous souhaitez soumettre un problème, veuillez, si vous le pouvez, reproduire le problème avec le niveau défini sur `TRACE`, pour ce faire, lancez termscp avec
l'option CLI `-D`.

Je sais que vous pourriez avoir des questions concernant les fichiers journaux, alors j'ai fait une sorte de Q/R :

> Je ne veux pas me connecter, puis-je le désactiver ?

Oui, vous pouvez. Démarrez simplement termscp avec l'option `-q ou --quiet`. Vous pouvez créer un alias termcp pour le rendre persistant. N'oubliez pas que la journalisation est utilisée pour diagnostiquer les problèmes, donc puisque derrière chaque projet open source, il devrait toujours y avoir ce genre d'aide mutuelle, la conservation des fichiers journaux peut être votre moyen de soutenir le projet 😉. Je ne veux pas que tu te sentes coupable, mais juste pour dire.

> La journalisation est-elle sûre ?

Si vous êtes préoccupé par la sécurité, le fichier journal ne contient aucun mot de passe simple, alors ne vous inquiétez pas et expose les mêmes informations que le fichier frère "signets".

## Notifications 📫

Termscp enverra des notifications de bureau pour ce type d'événements :

- sur **Transfert terminé** : La notification sera envoyée une fois le transfert terminé avec succès.
  - ❗ La notification ne s'affichera que si la taille totale du transfert est au moins la `Notifications: minimum transfer size` spécifiée dans la configuration.
- sur **Transfert échoué** : La notification sera envoyée une fois qu'un transfert a échoué en raison d'une erreur.
  - ❗ La notification ne s'affichera que si la taille totale du transfert est au moins la `Notifications: minimum transfer size` spécifiée dans la configuration.
- sur **Mise à jour disponible** : chaque fois qu'une nouvelle version de Termscp est disponible, une notification s'affiche.
- sur **Mise à jour installée** : chaque fois qu'une nouvelle version de Termscp est installée, une notification s'affiche.
- sur **Échec de la mise à jour** : chaque fois que l'installation de la mise à jour échoue, une notification s'affiche.

❗ Si vous préférez désactiver les notifications, vous pouvez simplement accéder à la configuration et définir `Enable notifications?` sur `No` 😉.  
❗ Si vous souhaitez modifier la taille de transfert minimale pour afficher les notifications, vous pouvez modifier la valeur dans la configuration avec la touche `Notifications: minimum transfer size` et la définir sur ce qui vous convient le mieux 🙂.

## Observateur de fichiers 🔭

L'observateur de fichiers vous permet de configurer une liste de chemins à synchroniser avec les hôtes distants.
Cela signifie que chaque fois qu'un changement sur le système de fichiers local sera détecté sur le chemin synchronisé, le changement sera automatiquement signalé au chemin de l'hôte distant configuré, dans les 5 secondes.

Vous pouvez définir autant de chemins à synchroniser que vous préférez :

1. Placez le curseur de l'explorateur local sur le répertoire/fichier que vous souhaitez conserver synchronisé
2. Accédez au répertoire dans lequel vous souhaitez que les modifications soient signalées sur l'hôte distant
3. Appuyez sur `<T>`
4. Répondez `<YES>` à la fenêtre contextuelle de la radio

Pour annuler la surveillance, appuyez simplement sur `<T>` sur le chemin synchronisé local (ou sur l'un de ses sous-dossiers)
OU vous pouvez simplement appuyer sur `<CTRL + T>` et appuyer sur `<ENTER>` jusqu'au chemin synchronisé que vous souhaitez désactiver.

Ces modifications seront signalées à l'hôte distant :

- Nouveaux fichiers, modifications de fichiers
- Fichier déplacé / renommé
- Fichier supprimé / dissocié

> ❗ Le watcher ne fonctionne que dans un sens (local > distant). Il n'est PAS possible de synchroniser automatiquement les changements de distant à local.
