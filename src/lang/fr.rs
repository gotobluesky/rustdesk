lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Statut"),
        ("Your Desktop", "Votre bureau"),
        ("desk_tip", "Votre bureau est accessible via l'identifiant et le mot de passe ci-dessous."),
        ("Password", "Mot de passe"),
        ("Ready", "Prêt"),
        ("connecting_status", "Connexion au réseau RustDesk..."),
        ("Enable Service", "Autoriser le service"),
        ("Start Service", "Démarrer le service"),
        ("Service is not running", "Le service ne fonctionne pas"),
        ("not_ready_status", "Pas prêt, veuillez vérifier la connexion réseau"),
        ("Control Remote Desktop", "Contrôler le bureau à distance"),
        ("Transfer File", "Transférer le fichier"),
        ("Connect", "Connecter"),
        ("Recent Sessions", "Sessions récentes"),
        ("Address Book", "Carnet d'adresses"),
        ("Confirmation", "Confirmation"),
        ("TCP Tunneling", "Tunneling TCP"),
        ("Remove", "Supprimer"),
        ("Refresh random password", "Actualiser le mot de passe aléatoire"),
        ("Set your own password", "Définir votre propre mot de passe"),
        ("Enable Keyboard/Mouse", "Activer le contrôle clavier/souris"),
        ("Enable Clipboard", "Activer la synchronisation du presse-papiers"),
        ("Enable File Transfer", "Activer le transfert de fichiers"),
        ("Enable TCP Tunneling", "Activer le tunneling TCP"),
        ("IP Whitelisting", "Liste blanche IP"),
        ("ID/Relay Server", "ID/Serveur Relais"),
        ("Stop service", "Arrêter service"),
        ("Change ID", "Changer d'ID"),
        ("Website", "Site Web"),
        ("About", "Sur"),
        ("Mute", "Muet"),
        ("Audio Input", "Entrée audio"),
        ("ID Server", "Serveur ID"),
        ("Relay Server", "Serveur Relais"),
        ("API Server", "Serveur API"),
        ("invalid_http", "Doit commencer par http:// ou https://"),
        ("Invalid IP", "IP invalide"),
        ("id_change_tip", "Seules les lettres a-z, A-Z, 0-9, _ (trait de soulignement) peuvent être utilisées. La première lettre doit être a-z, A-Z. La longueur est comprise entre 6 et 16."),
        ("Invalid format", "Format invalide"),
        ("This function is turned off by the server", "Cette fonction est désactivée par le serveur"),
        ("Not available", "Indisponible"),
        ("Too frequent", "Modifier trop fréquemment, veuillez réessayer plus tard"),
        ("Cancel", "Annuler"),
        ("Skip", "Ignorer"),
        ("Close", "Fermer"),
        ("Retry", "Réessayer"),
        ("OK", "Confirmer"),
        ("Password Required", "Mot de passe requis"),
        ("Please enter your password", "Veuillez saisir votre mot de passe"),
        ("Remember password", "Mémoriser le mot de passe"),
        ("Wrong Password", "Mauvais mot de passe"),
        ("Do you want to enter again?", "Voulez-vous participer à nouveau ?"),
        ("Connection Error", "Erreur de connexion"),
        ("Error", "Erreur"),
        ("Reset by the peer", "La connexion a été fermée par le pair"),
        ("Connecting...", "Connexion..."),
        ("Connection in progress. Please wait.", "Connexion en cours. Veuillez patienter."),
        ("Please try 1 minute later", "Réessayez dans une minute"),
        ("Login Error", "Erreur de connexion"),
        ("Successful", "Succès"),
        ("Connected, waiting for image...", "Connecté, en attente de transmission d'image..."),
        ("Name", "Nom du fichier"),
        ("Modified", "Modifié"),
        ("Size", "Taille"),
        ("Show Hidden Files", "Afficher les fichiers cachés"),
        ("Receive", "Accepter"),
        ("Send", "Envoyer"),
        ("Remote Computer", "Ordinateur distant"),
        ("Local Computer", "Ordinateur local"),
        ("Confirm Delete", "Confirmer la suppression"),
        ("Are you sure you want to delete this file?", "Voulez-vous vraiment supprimer ce fichier ?"),
        ("Do this for all conflicts", "Appliquer à d'autres conflits"),
        ("Deleting", "Suppression"),
        ("files", "fichier"),
        ("Waiting", "En attente en attente..."),
        ("Finished", "Terminé"),
        ("Custom Image Quality", "Définir la qualité d'image"),
        ("Privacy mode", "Mode privé"),
        ("Adjust Window", "Ajuster la fenêtre"),
        ("Original", "Ratio d'origine"),
        ("Shrink", "Rétréci"),
        ("Stretch", "Étirer"),
        ("Good image quality", "Bonne qualité d'image"),
        ("Balanced", "Qualité d'image normale"),
        ("Optimize reaction time", "Optimiser le temps de réaction"),
        ("Custom", "Qualité d'image personnalisée"),
        ("Show remote cursor", "Afficher le curseur distant"),
        ("Disable clipboard", "Désactiver le presse-papiers"),
        ("Lock after session end", "Verrouiller l'ordinateur distant après la déconnexion"),
        ("Insert", "Insérer"),
        ("Insert Lock", "Verrouiller l'ordinateur distant"),
        ("Refresh", "Rafraîchir l'écran"),
        ("ID does not exist", "L'ID n'existe pas"),
        ("Failed to connect to rendezvous server", "Échec de la connexion au serveur de rendez-vous"),
        ("Please try later", "Veuillez essayer plus tard"),
        ("Remote desktop is offline", "Le bureau à distance est hors ligne"),
        ("Key mismatch", "Discordance de clé"),
        ("Timeout", "Connexion expirée"),
        ("Failed to connect to relay server", "Échec de la connexion au serveur relais"),
        ("Failed to connect via rendezvous server", "Échec de l'établissement d'une connexion via le serveur de rendez-vous"),
        ("Failed to connect via relay server", "Impossible d'établir une connexion via le serveur relais"),
        ("Failed to make direct connection to remote desktop", "Impossible d'établir une connexion directe"),
        ("Set Password", "Définir le mot de passe"),
        ("OS Password", "Mot de passe du système d'exploitation"),
        ("install_tip", "Vous utilisez une version désinstallée. En raison des restrictions UAC, en tant que terminal contrôlé, dans certains cas, il ne sera pas en mesure de contrôler la souris et le clavier ou d'enregistrer l'écran. Veuillez cliquer sur le bouton ci-dessous pour installer RustDesk au système pour éviter la question ci-dessus."),
        ("Click to upgrade", "Cliquez pour mettre à niveau"),
        ("Configuration Permissions", "Autorisations de configuration"),
        ("Configure", "Configurer"),
        ("config_acc", "Afin de pouvoir contrôler votre bureau à distance, veuillez donner l'autorisation\"accessibilité\" à RustDesk."),
        ("config_screen", "Afin de pouvoir accéder à votre bureau à distance, veuillez donner l'autorisation à RustDesk\"enregistrement d'écran\"."),
        ("Installing ...", "Installation ..."),
        ("Install", "Installer"),
        ("Installation", "Installation"),
        ("Installation Path", "Chemin d'installation"),
        ("Create start menu shortcuts", "Créer des raccourcis dans le menu démarrer"),
        ("Create desktop icon", "Créer une icône sur le bureau"),
        ("agreement_tip", "Démarrer l'installation signifie accepter le contrat de licence."),
        ("Accept and Install", "Accepter et installer"),
        ("End-user license agreement", "Contrat d'utilisateur"),
        ("Generating ...", "Génération ..."),
        ("Your installation is lower version.", "La version que vous avez installée est inférieure à la version en cours d'exécution."),
        ("not_close_tcp_tip", "Veuillez ne pas fermer cette fenêtre lors de l'utilisation du tunnel"),
        ("Listening ...", "En attente de connexion tunnel..."),
        ("Remote Host", "Hôte distant"),
        ("Remote Port", "Port distant"),
        ("Action", "Action"),
        ("Add", "Ajouter"),
        ("Local Port", "Port local"),
        ("setup_server_tip", "Si vous avez besoin d'une vitesse de connexion plus rapide, vous pouvez choisir de créer votre propre serveur"),
        ("Too short, at least 6 characters.", "Trop court, au moins 6 caractères."),
        ("The confirmation is not identical.", "Les deux entrées ne correspondent pas"),
        ("Permissions", "Autorisations"),
        ("Accept", "Accepter"),
        ("Dismiss", "Rejeter"),
        ("Disconnect", "Déconnecter"),
        ("Allow using keyboard and mouse", "Autoriser l'utilisation du clavier et de la souris"),
        ("Allow using clipboard", "Autoriser l'utilisation du presse-papiers"),
        ("Allow hearing sound", "Autoriser l'audition du son"),
        ("Connected", "Connecté"),
        ("Direct and encrypted connection", "Connexion directe cryptée"),
        ("Relayed and encrypted connection", "Connexion relais cryptée"),
        ("Direct and unencrypted connection", "Connexion directe non cryptée"),
        ("Relayed and unencrypted connection", "Connexion relais non cryptée"),
        ("Enter Remote ID", "Entrez l'ID à distance"),
        ("Enter your password", "Entrez votre mot de passe"),
        ("Logging in...", "Se connecter..."),
        ("Enable RDP session sharing", "Activer le partage de session RDP"),
        ("Auto Login", "Connexion automatique (le verrouillage ne sera effectif qu'après la déconnexion du paramètre)"),
        ("Enable Direct IP Access", "Autoriser l'accès direct IP"),
        ("Rename", "Renommer"),
        ("Space", "Espace"),
        ("Create Desktop Shortcut", "Créer un raccourci sur le bureau"),
        ("Change Path", "Changer de chemin"),
        ("Create Folder", "Créer un dossier"),
        ("Please enter the folder name", "Veuillez saisir le nom du dossier"),
        ("Fix it", "Réparez-le"),
        ("Warning", "Avertissement"),
        ("Login screen using Wayland is not supported", "L'écran de connexion utilisant Wayland n'est pas pris en charge"),
        ("Reboot required", "Redémarrage pour prendre effet"),
        ("Unsupported display server ", "Le serveur d'affichage actuel n'est pas pris en charge"),
        ("x11 expected", "Veuillez passer à x11"),
        ("Port", "Port"),
        ("Settings", "Paramètres"),
        ("Username", " Nom d'utilisateur"),
        ("Invalid port", "Port invalide"),
        ("Closed manually by the peer", "Fermé manuellement par le pair"),
        ("Enable remote configuration modification", "Autoriser la modification de la configuration à distance"),
        ("Run without install", "Exécuter sans installer"),
        ("Always connected via relay", "Forcer la connexion relais"),
        ("Always connect via relay", "Forcer la connexion relais"),
        ("whitelist_tip", "Seul l'ip dans la liste blanche peut m'accéder"),
        ("Login", "Connexion"),
        ("Logout", "Déconnexion"),
        ("Tags", "Étiqueter"),
        ("Search ID", "Identifiant de recherche"),
        ("Current Wayland display server is not supported", "Le serveur d'affichage Wayland n'est pas pris en charge"),
        ("whitelist_sep", "Vous pouvez utiliser une virgule, un point-virgule, un espace ou une nouvelle ligne comme séparateur"),
        ("Add ID", "Ajouter ID"),
        ("Add Tag", "Ajouter une balise"),
        ("Unselect all tags", "Désélectionner toutes les balises"),
        ("Network error", "Erreur réseau"),
        ("Username missed", "Nom d'utilisateur manqué"),
        ("Password missed", "Mot de passe manqué"),
        ("Wrong credentials", "Identifiant ou mot de passe erroné"),
        ("Edit Tag", "Modifier la balise"),
        ("Invalid folder name", "Nom de dossier invalide"),
        ("Hostname", "nom d'hôte"),
    ].iter().cloned().collect();
    }
