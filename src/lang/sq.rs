lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Statusi"),
        ("Your Desktop", "Desktopi juaj"),
        ("desk_tip", "Desktopi juaj mund të aksesohet me këtë ID dhe fjalëkalim."),
        ("Password", "fjalëkalimi"),
        ("Ready", "Gati"),
        ("Established", "I themeluar"),
        ("connecting_status", "statusi_i_lidhjes"),
        ("Enable Service", "Aktivizo Shërbimin"),
        ("Start Service", "Nis Shërbimin"),
        ("Service is running", "Shërbimi është duke funksionuar"),
        ("Service is not running", "Shërbimi nuk është duke funksionuar"),
        ("not_ready_status", "Jo gati.Ju lutem kontolloni lidhjen tuaj."),
        ("Control Remote Desktop", "Kontrolli i desktopit në distancë"),
        ("Transfer File", "Transfero dosje"),
        ("Connect", "Lidh"),
        ("Recent Sessions", "Sessioni i fundit"),
        ("Address Book", "Libër adresash"),
        ("Confirmation", "Konfirmimi"),
        ("TCP Tunneling", "TCP tunel"),
        ("Remove", "Hiqni"),
        ("Refresh random password", "Rifreskoni fjalëkalimin e rastësishëm"),
        ("Set your own password", "Vendosni fjalëkalimin tuaj"),
        ("Enable Keyboard/Mouse", "Aktivizoni Tastierën/Mousin"),
        ("Enable Clipboard", "Aktivizo"),
        ("Enable File Transfer", "Aktivizoni transferimin e skedarëve"),
        ("Enable TCP Tunneling", "Aktivizoni TCP Tunneling"),
        ("IP Whitelisting", ""),
        ("ID/Relay Server", "ID/server rele"),
        ("Import Server Config", "Konfigurimi i severit të importit"),
        ("Export Server Config", "Konfigurimi i severit të eksportit"),
        ("Import server configuration successfully", "Konfigurimi i severit të importit i suksesshëm"),
        ("Export server configuration successfully", "Konfigurimi i severit të eksprotit i suksesshëm"),
        ("Invalid server configuration", "Konfigurim i pavlefshëm i serverit"),
        ("Clipboard is empty", "Clipboard është bosh"),
        ("Stop service", "Ndaloni shërbimin"),
        ("Change ID", "Ndryshoni ID"),
        ("Your new ID", ""),
        ("length %min% to %max%", ""),
        ("starts with a letter", ""),
        ("allowed characters", ""),
        ("id_change_tip", "Lejohen Vetëm karkteret a-z,A-Z,0-9 dhe _(nënvizimet).Shkronja e parë duhet të jetë a-z, A-Z. Gjatesia midis 6 dhe 16."),
        ("Website", "Faqe ëebi"),
        ("About", "Rreth"),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "Pa zë"),
        ("Build Date", ""),
        ("Version", ""),
        ("Home", ""),
        ("Audio Input", "Inputi zërit"),
        ("Enhancements", "Përmirësimet"),
        ("Hardware Codec", "Kodeku Harduerik"),
        ("Adaptive Bitrate", "Shpejtësia adaptive e biteve"),
        ("ID Server", "ID e serverit"),
        ("Relay Server", "Serveri rele"),
        ("API Server", "Serveri API"),
        ("invalid_http", "Duhet të fillojë me http:// ose https://"),
        ("Invalid IP", "IP e pavlefshme"),
        ("Invalid format", "Format i pavlefshëm"),
        ("server_not_support", "Nuk suportohet akoma nga severi"),
        ("Not available", "I padisponueshëm"),
        ("Too frequent", "Shumë i përdorur"),
        ("Cancel", "Anullo"),
        ("Skip", "Kalo"),
        ("Close", "Mbyll"),
        ("Retry", "Riprovo"),
        ("OK", "OK"),
        ("Password Required", "Fjalëkalimi i detyrueshëm"),
        ("Please enter your password", "Ju lutem vendosni fjalëkalimin tuaj"),
        ("Remember password", "Mbani mend fjalëkalimin"),
        ("Wrong Password", "Fjalëkalim i gabuar"),
        ("Do you want to enter again?", "Dëshironi të vendosni përsëri"),
        ("Connection Error", "Gabim në lidhje"),
        ("Error", "Gabim"),
        ("Reset by the peer", "Riseto nga peer"),
        ("Connecting...", "Duke u lidhur"),
        ("Connection in progress. Please wait.", "Lidhja në progres. Ju lutem prisni"),
        ("Please try 1 minute later", "Ju lutemi provoni 1 minut më vonë"),
        ("Login Error", "Gabim në login"),
        ("Successful", "E suksesshme"),
        ("Connected, waiting for image...", "E lidhur , prisni për imazhin..."),
        ("Name", "Emri"),
        ("Type", "Shkruaj"),
        ("Modified", "E modifikuar"),
        ("Size", "Madhesia"),
        ("Show Hidden Files", "Shfaq skedarët e fshehur"),
        ("Receive", "Merr"),
        ("Send", "Dërgo"),
        ("Refresh File", "Rifreskoni skedarët"),
        ("Local", "Lokal"),
        ("Remote", "Në distancë"),
        ("Remote Computer", "Kompjuter në distancë"),
        ("Local Computer", "Kompjuter Lokal"),
        ("Confirm Delete", "Konfirmoni fshirjen"),
        ("Delete", "Fshij"),
        ("Properties", "Karakteristikat"),
        ("Multi Select", "Shumë përzgjedhje"),
        ("Select All", "Selektoni të gjitha"),
        ("Unselect All", "Ç'selektoni të gjitha"),
        ("Empty Directory", "Direktori boshe"),
        ("Not an empty directory", "Jo një direktori boshe"),
        ("Are you sure you want to delete this file?", "Jeni të sigurtë që doni të fshini këtë skedarë"),
        ("Are you sure you want to delete this empty directory?", "Jeni të sigurtë që dëshironi të fshini këtë direktori boshe"),
        ("Are you sure you want to delete the file of this directory?", "Jeni të sigurtë që dëshironi te fshini skedarin e kësaj direktorie"),
        ("Do this for all conflicts", "Bëjeni këtë për të gjitha konfliktet"),
        ("This is irreversible!", "Kjo është e pakthyeshme"),
        ("Deleting", "Duke i fshirë"),
        ("files", "Skedarë"),
        ("Waiting", "Në pritje"),
        ("Finished", "Përfunduar"),
        ("Speed", "Shpejtësia"),
        ("Custom Image Quality", "Cilësi e personalizuar imazhi"),
        ("Privacy mode", "Modaliteti i Privatësisë"),
        ("Block user input", "Blloko inputin e përdorusesit"),
        ("Unblock user input", "Zhblloko inputin e përdorusesit"),
        ("Adjust Window", "Rregulloni dritaren"),
        ("Original", "Origjinal"),
        ("Shrink", "Shkurtim"),
        ("Stretch", "Shtrirje"),
        ("Scrollbar", "Shiriti i lëvizjes"),
        ("ScrollAuto", "Levizje automatikisht"),
        ("Good image quality", "Cilësi e mirë imazhi"),
        ("Balanced", "E balancuar"),
        ("Optimize reaction time", "Optimizo kohën e reagimit"),
        ("Custom", "Personalizuar"),
        ("Show remote cursor", "Shfaq kursorin në distancë"),
        ("Show quality monitor", "Shaq cilësinë e monitorit"),
        ("Disable clipboard", "Ç'aktivizo clipboard"),
        ("Lock after session end", "Kyç pasi sesioni të përfundoj"),
        ("Insert", "Fut"),
        ("Insert Lock", "Fut bllokimin"),
        ("Refresh", "Rifresko"),
        ("ID does not exist", "ID nuk ekziston"),
        ("Failed to connect to rendezvous server", "Dështoj të lidhet me serverin e takimit"),
        ("Please try later", "Ju lutemi provoni më vonë"),
        ("Remote desktop is offline", "Desktopi në distancë nuk është në linjë"),
        ("Key mismatch", "Mospërputhje kryesore"),
        ("Timeout", "Koha mbaroi"),
        ("Failed to connect to relay server", "Lidhja me serverin transmetues dështoi"),
        ("Failed to connect via rendezvous server", "Lidhja nëpërmjet serverit të takimit dështoi"),
        ("Failed to connect via relay server", "Lidhja nëpërmjet serverit të transmetimit dështoi"),
        ("Failed to make direct connection to remote desktop", "Lidhja direkte me desktopin në distancë dështoi"),
        ("Set Password", "Vendosni fjalëkalimin"),
        ("OS Password", "OS fjalëkalim"),
        ("install_tip", "Për shkak të UAC, Rustdesk nuk mund të punoj sic duhet si nje remote në distancë në disa raste. Për të shamngur UAC, ju lutem klikoni butonin më poshtë për të instaluar RustDesk në sistem."),
        ("Click to upgrade", "Klikoni për përmirësim"),
        ("Click to download", "Klikoni për tu shkarkuar"),
        ("Click to update", "Klikoni për përditësim"),
        ("Configure", "Koniguro"),
        ("config_acc", "Për të kontrolluar Desktopin tuaj nga distanca, duhet të jepni leje RustDesk \"Aksesueshmëri\"."),
        ("config_screen", "Për të aksesuar Desktopin tuaj nga distanca, duhet ti jepni lejet RustDesk \"Regjistrimin e ekranit\"."),
        ("Installing ...", "Duke u instaluar"),
        ("Install", "Instalo"),
        ("Installation", "Instalimi"),
        ("Installation Path", "Rruga instalimit"),
        ("Create start menu shortcuts", "Krijoni shortcuts për menunë e fillimit"),
        ("Create desktop icon", "Krijoni ikonën e desktopit"),
        ("agreement_tip", "Duke filluar instalimin, ju pranoni marrëveshjen e licencës"),
        ("Accept and Install", "Pranoni dhe instaloni"),
        ("End-user license agreement", "Marrëeveshja e licencës së perdoruesit fundor"),
        ("Generating ...", "Duke gjeneruar"),
        ("Your installation is lower version.", "Instalimi juaj është version i ulët"),
        ("not_close_tcp_tip", "Mos e mbyll këtë dritare ndërsa jeni duke  përdorur tunelin"),
        ("Listening ...", "Duke dëgjuar"),
        ("Remote Host", "Host në distancë"),
        ("Remote Port", "Port në distancë"),
        ("Action", "Veprim"),
        ("Add", "Shto"),
        ("Local Port", "Portë Lokale"),
        ("Local Address", "Adresë Lokale"),
        ("Change Local Port", "Ndryshoni portën lokale"),
        ("setup_server_tip", "Për lidhje më të shpejtë, ju lutemi konfiguroni serverin tuaj"),
        ("Too short, at least 6 characters.", "Shumë e shkurtër , nevojiten të paktën 6 karaktere"),
        ("The confirmation is not identical.", "Konfirmimi nuk është identik"),
        ("Permissions", "Leje"),
        ("Accept", "Prano"),
        ("Dismiss", "Hiq"),
        ("Disconnect", "Shkëput"),
        ("Allow using keyboard and mouse", "Lejoni përdorimin e Tastierës dhe Mousit"),
        ("Allow using clipboard", "Lejoni përdorimin e clipboard"),
        ("Allow hearing sound", "Lejoni dëgjimin e zërit"),
        ("Allow file copy and paste", "Lejoni kopjimin dhe pastimin e skedarëve"),
        ("Connected", "I lidhur"),
        ("Direct and encrypted connection", "Lidhje direkte dhe enkriptuar"),
        ("Relayed and encrypted connection", "Lidhje transmetuese dhe e enkriptuar"),
        ("Direct and unencrypted connection", "Lidhje direkte dhe jo e enkriptuar"),
        ("Relayed and unencrypted connection", "Lidhje transmetuese dhe jo e enkriptuar"),
        ("Enter Remote ID", "Vendosni ID në distancë"),
        ("Enter your password", "Vendosni fjalëkalimin tuaj"),
        ("Logging in...", "Duke u loguar"),
        ("Enable RDP session sharing", "Aktivizoni shpërndarjen e sesionit RDP"),
        ("Auto Login", "Hyrje automatike"),
        ("Enable Direct IP Access", "Aktivizoni aksesimin e IP direkte"),
        ("Rename", "Riemërto"),
        ("Space", "Hapërsirë"),
        ("Create Desktop Shortcut", "Krijoni shortcut desktop"),
        ("Change Path", "Ndrysho rrugëzimin"),
        ("Create Folder", "Krijoni një folder"),
        ("Please enter the folder name", "Ju lutem vendosni emrin e folderit"),
        ("Fix it", "Rregulloni ate"),
        ("Warning", "Dicka po shkon keq"),
        ("Login screen using Wayland is not supported", "Hyrja në ekran duke përdorur Wayland muk suportohet"),
        ("Reboot required", "Kërkohet rinisja"),
        ("Unsupported display server", "Nuk supurtohet severi ekranit"),
        ("x11 expected", "Pritet x11"),
        ("Port", "Port"),
        ("Settings", "Cilësimet"),
        ("Username", "Emri i përdoruesit"),
        ("Invalid port", "Port e pavlefshme"),
        ("Closed manually by the peer", "E mbyllur manualisht nga peer"),
        ("Enable remote configuration modification", "Aktivizoni modifikimin e konfigurimit në distancë"),
        ("Run without install", "Ekzekuto pa instaluar"),
        ("Connect via relay", ""),
        ("Always connect via relay", "Gjithmonë lidheni me transmetues"),
        ("whitelist_tip", "Vetëm IP e listës së bardhë mund të më aksesoj."),
        ("Login", "Hyrje"),
        ("Verify", ""),
        ("Remember me", ""),
        ("Trust this device", ""),
        ("Verification code", ""),
        ("verification_tip", ""),
        ("Logout", "Dalje"),
        ("Tags", "Tage"),
        ("Search ID", "Kerko ID"),
        ("whitelist_sep", "Të ndara me presje, pikëpresje, hapësira ose rresht të ri"),
        ("Add ID", "Shto ID"),
        ("Add Tag", "Shto Tag"),
        ("Unselect all tags", "Hiq selektimin e te gjithë tageve"),
        ("Network error", "Gabim në rrjet"),
        ("Username missed", "Mungon përdorusesi"),
        ("Password missed", "Mungon fjalëkalimi"),
        ("Wrong credentials", "Kredinciale të gabuara"),
        ("The verification code is incorrect or has expired", ""),
        ("Edit Tag", "Edito tagun"),
        ("Unremember Password", "Fjalëkalim jo i kujtueshëm"),
        ("Favorites", "Te preferuarat"),
        ("Add to Favorites", "Shto te të preferuarat"),
        ("Remove from Favorites", "Hiq nga të preferuarat"),
        ("Empty", "Bosh"),
        ("Invalid folder name", "Emri i dosjes i pavlefshëm"),
        ("Socks5 Proxy", "Socks5 Proxy"),
        ("Hostname", "Emri Hostit"),
        ("Discovered", "I pambuluar"),
        ("install_daemon_tip", "Për të nisur në boot, duhet të instaloni shërbimin e sistemit"),
        ("Remote ID", "ID në distancë"),
        ("Paste", "Ngjit"),
        ("Paste here?", "Ngjit këtu"),
        ("Are you sure to close the connection?", "Jeni të sigurtë të mbyllni lidhjen"),
        ("Download new version", "Shkarko versionin e ri"),
        ("Touch mode", "Metoda me prekje"),
        ("Mouse mode", "Modaliteti mausit"),
        ("One-Finger Tap", "Prekja Një gisht"),
        ("Left Mouse", "Mausi majt"),
        ("One-Long Tap", "Prekja nje-gjate"),
        ("Two-Finger Tap", "Prekja dy-gishta"),
        ("Right Mouse", "Mausi i djathtë"),
        ("One-Finger Move", "Lëvizja një-gisht"),
        ("Double Tap & Move", "Prekja dhe lëvizja e dyfishtë"),
        ("Mouse Drag", "Zhvendosja e mausit"),
        ("Three-Finger vertically", "Tre-Gishta vertikalisht"),
        ("Mouse Wheel", "Rrota mausit"),
        ("Two-Finger Move", "Lëvizja Dy-Gishta"),
        ("Canvas Move", "Lëvizja Canvas"),
        ("Pinch to Zoom", "Prekni për të zmadhuar"),
        ("Canvas Zoom", "Zmadhimi Canavas"),
        ("Reset canvas", "Riseto canvas"),
        ("No permission of file transfer", "Nuk ka leje për transferimin e dosjesve"),
        ("Note", "Shënime"),
        ("Connection", "Lidhja"),
        ("Share Screen", "Ndaj ekranin"),
        ("Chat", "Biseda"),
        ("Total", "Total"),
        ("items", "artikuj"),
        ("Selected", "E zgjedhur"),
        ("Screen Capture", "Kapja e ekranit"),
        ("Input Control", "Kontrollo inputin"),
        ("Audio Capture", "Kapja e zërit"),
        ("File Connection", "Lidhja e skedarëve"),
        ("Screen Connection", "Lidhja e ekranit"),
        ("Do you accept?", "E pranoni"),
        ("Open System Setting", "Hapni cilësimet e sistemit"),
        ("How to get Android input permission?", "Si të merrni leje e inputit të Android"),
        ("android_input_permission_tip1", "Në mënyrë që një pajisje në distancë të kontrollojë pajisjen tuaj Android nëpërmjet mausit ose prekjes, duhet të lejoni RustDesk të përdorë shërbimin."),
        ("android_input_permission_tip2", "Ju lutemi shkoni në faqen tjetër të cilësimeve të sistemit, gjeni dhe shtypni [Shërbimet e Instaluara], aktivizoni shërbimin [RustDesk Input]"),
        ("android_new_connection_tip", "Është marrë një kërkesë e re kontrolli, e cila dëshiron të kontrollojë pajisjen tuaj aktuale."),
        ("android_service_will_start_tip", "Aktivizimi i \"Regjistrimi i ekranit\" do të nisë automatikisht shërbimin, duke lejuar pajisjet e tjera të kërkojnë një lidhje me pajisjen tuaj."),
        ("android_stop_service_tip", "Mbyllja e shërbimit do të mbyllë automatikisht të gjitha lidhjet e vendosura."),
        ("android_version_audio_tip", "Versioni aktual i Android nuk mbështet regjistrimin e audios, ju lutemi përmirësoni në Android 10 ose më të lartë."),
        ("android_start_service_tip", ""),
        ("android_permission_may_not_change_tip", ""),
        ("Account", "Llogaria"),
        ("Overwrite", "Përshkruaj"),
        ("This file exists, skip or overwrite this file?", "Ky skedar ekziston , tejkalo ose përshkruaj këtë skedarë"),
        ("Quit", "Hiq"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "Ndihmë"),
        ("Failed", "Deshtoi"),
        ("Succeeded", "Sukses"),
        ("Someone turns on privacy mode, exit", "Dikush ka ndezur menyrën e privatësisë , largohu"),
        ("Unsupported", "Nuk mbështetet"),
        ("Peer denied", "Peer mohohet"),
        ("Please install plugins", "Ju lutemi instaloni shtojcat"),
        ("Peer exit", "Dalje peer"),
        ("Failed to turn off", "Dështoi të fiket"),
        ("Turned off", "I fikur"),
        ("In privacy mode", "Në modalitetin e privatësisë"),
        ("Out privacy mode", "Jashtë modaliteti i privatësisë"),
        ("Language", "Gjuha"),
        ("Keep RustDesk background service", "Mbaje shërbimin e sfondit të RustDesk"),
        ("Ignore Battery Optimizations", "Injoro optimizimet e baterisë"),
        ("android_open_battery_optimizations_tip", "Nëse dëshironi ta çaktivizoni këtë veçori, ju lutemi shkoni te faqja tjetër e cilësimeve të aplikacionit RustDesk, gjeni dhe shtypni [Batteri], hiqni zgjedhjen [Te pakufizuara]"),
        ("Start on Boot", ""),
        ("Start the screen sharing service on boot, requires special permissions", ""),
        ("Connection not allowed", "Lidhja nuk lejohet"),
        ("Legacy mode", "Modaliteti i trashëgimisë"),
        ("Map mode", "Modaliteti i hartës"),
        ("Translate mode", "Modaliteti i përkthimit"),
        ("Use permanent password", "Përdor fjalëkalim të përhershëm"),
        ("Use both passwords", "Përdor të dy fjalëkalimet"),
        ("Set permanent password", "Vendos fjalëkalimin e përhershëm"),
        ("Enable Remote Restart", "Aktivizo rinisjen në distancë"),
        ("Allow remote restart", "Lejo rinisjen në distancë"),
        ("Restart Remote Device", "Rinisni pajisjen në distancë"),
        ("Are you sure you want to restart", "A jeni i sigurt që dëshironi të rinisni"),
        ("Restarting Remote Device", "Rinisja e pajisjes në distancë"),
        ("remote_restarting_tip", "Pajisja në distancë po riniset, ju lutemi mbyllni këtë kuti mesazhi dhe lidheni përsëri me fjalëkalim të përhershëm pas një kohe"),
        ("Copied", "Kopjuar"),
        ("Exit Fullscreen", "Dil nga ekrani i plotë"),
        ("Fullscreen", "Ekran i plotë"),
        ("Mobile Actions", "Veprimet celulare"),
        ("Select Monitor", "Zgjidh Monitor"),
        ("Control Actions", "Veprimet e kontrollit"),
        ("Display Settings", "Cilësimet e ekranit"),
        ("Ratio", "Raport"),
        ("Image Quality", "Cilësia e imazhit"),
        ("Scroll Style", "Stili i lëvizjes"),
        ("Show Toolbar", ""),
        ("Hide Toolbar", ""),
        ("Direct Connection", "Lidhja e drejtpërdrejtë"),
        ("Relay Connection", "Lidhja rele"),
        ("Secure Connection", "Lidhje e sigurt"),
        ("Insecure Connection", "Lidhje e pasigurt"),
        ("Scale original", "Shkalla origjinale"),
        ("Scale adaptive", " E përsjhtatshme në shkallë"),
        ("General", "Gjeneral"),
        ("Security", "Siguria"),
        ("Theme", "Theme"),
        ("Dark Theme", "Theme e errët"),
        ("Light Theme", ""),
        ("Dark", "E errët"),
        ("Light", "Drita"),
        ("Follow System", "Ndiq sistemin"),
        ("Enable hardware codec", "Aktivizo kodekun e harduerit"),
        ("Unlock Security Settings", "Zhbllokoni cilësimet e sigurisë"),
        ("Enable Audio", "Aktivizo audio"),
        ("Unlock Network Settings", "Zhbllokoni cilësimet e rrjetit"),
        ("Server", "Server"),
        ("Direct IP Access", "Qasje e drejtpërdrejtë IP"),
        ("Proxy", "Proxy"),
        ("Apply", "Apliko"),
        ("Disconnect all devices?", "Shkyç të gjitha pajisjet?"),
        ("Clear", "Pastro"),
        ("Audio Input Device", "Pajisja e hyrjes audio"),
        ("Use IP Whitelisting", "Përdor listën e bardhë IP"),
        ("Network", "Rrjeti"),
        ("Enable RDP", "Aktivizo RDP"),
        ("Pin Toolbar", ""),
        ("Unpin Toolbar", ""),
        ("Recording", "Regjistrimi"),
        ("Directory", "Direktoria"),
        ("Automatically record incoming sessions", "Regjistro automatikisht seancat hyrëse"),
        ("Change", "Ndrysho"),
        ("Start session recording", "Fillo regjistrimin e sesionit"),
        ("Stop session recording", "Ndalo regjistrimin e sesionit"),
        ("Enable Recording Session", "Aktivizo seancën e regjistrimit"),
        ("Allow recording session", "Lejo regjistrimin e sesionit"),
        ("Enable LAN Discovery", "Aktivizo zbulimin e LAN"),
        ("Deny LAN Discovery", "Mohoni zbulimin e LAN"),
        ("Write a message", "Shkruani një mesazh"),
        ("Prompt", "Prompt"),
        ("Please wait for confirmation of UAC...", "Ju lutemi prisni për konfirmimin e UAC..."),
        ("elevated_foreground_window_tip", "Përkohësisht është e pamundur për të përdorur mausin dhe tastierën, për shkak se dritarja aktuale e desktopit në distancë kërkon privilegj më të lartë për të vepruar,ju mund t'i kërkoni përdoruesit në distancë të minimizojë dritaren aktuale. Për të shmangur këtë problem, rekomandohet të instaloni softuerin në pajisjen në distancë ose ekzekutoni atë me privilegje administratori."),
        ("Disconnected", "Shkyçur"),
        ("Other", "Tjetër"),
        ("Confirm before closing multiple tabs", "Konfirmo përpara se të mbyllësh shumë skeda"),
        ("Keyboard Settings", "Cilësimet e tastierës"),
        ("Full Access", "Qasje e plotë"),
        ("Screen Share", "Ndarja e ekranit"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland kërkon Ubuntu 21.04 ose version më të lartë"),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland kërkon një version më të lartë të shpërndarjes linux. Ju lutemi provoni desktopin X11 ose ndryshoni OS."),
        ("JumpLink", "JumpLink"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Ju lutemi zgjidhni ekranin që do të ndahet (Vepro në anën e kolegëve"),
        ("Show RustDesk", "Shfaq RustDesk"),
        ("This PC", "Ky PC"),
        ("or", "ose"),
        ("Continue with", "Vazhdo me"),
        ("Elevate", "Ngritja"),
        ("Zoom cursor", "Zmadho kursorin"),
        ("Accept sessions via password", "Prano sesionin nëpërmjet fjalëkalimit"),
        ("Accept sessions via click", "Prano sesionet nëpërmjet klikimit"),
        ("Accept sessions via both", "Prano sesionet nëpërmjet të dyjave"),
        ("Please wait for the remote side to accept your session request...", "Ju lutem prisni që ana në distancë të pranoj kërkësen tuaj"),
        ("One-time Password", "Fjalëkalim Një-herë"),
        ("Use one-time password", "Përdorni fjalëkalim Një-herë"),
        ("One-time password length", "Gjatësia e fjalëkalimit një herë"),
        ("Request access to your device", "Kërko akses në pajisjejn tuaj"),
        ("Hide connection management window", "Fshih dritaren e menaxhimit të lidhjes"),
        ("hide_cm_tip", "Kjo është e mundur vetëm nëse aksesi bëhet nëpërmjet një fjalëkalimi të përhershëm"),
        ("wayland_experiment_tip", ""),
        ("Right click to select tabs", ""),
        ("Skipped", ""),
        ("Add to Address Book", ""),
        ("Group", ""),
        ("Search", ""),
        ("Closed manually by web console", ""),
        ("Local keyboard type", ""),
        ("Select local keyboard type", ""),
        ("software_render_tip", ""),
        ("Always use software rendering", ""),
        ("config_input", ""),
        ("config_microphone", ""),
        ("request_elevation_tip", ""),
        ("Wait", ""),
        ("Elevation Error", ""),
        ("Ask the remote user for authentication", ""),
        ("Choose this if the remote account is administrator", ""),
        ("Transmit the username and password of administrator", ""),
        ("still_click_uac_tip", ""),
        ("Request Elevation", ""),
        ("wait_accept_uac_tip", ""),
        ("Elevate successfully", ""),
        ("uppercase", ""),
        ("lowercase", ""),
        ("digit", ""),
        ("special character", ""),
        ("length>=8", ""),
        ("Weak", ""),
        ("Medium", ""),
        ("Strong", ""),
        ("Switch Sides", ""),
        ("Please confirm if you want to share your desktop?", ""),
        ("Display", ""),
        ("Default View Style", ""),
        ("Default Scroll Style", ""),
        ("Default Image Quality", ""),
        ("Default Codec", ""),
        ("Bitrate", ""),
        ("FPS", ""),
        ("Auto", ""),
        ("Other Default Options", ""),
        ("Voice call", ""),
        ("Text chat", ""),
        ("Stop voice call", ""),
        ("relay_hint_tip", ""),
        ("Reconnect", ""),
        ("Codec", ""),
        ("Resolution", ""),
        ("No transfers in progress", ""),
        ("Set one-time password length", ""),
        ("install_cert_tip", ""),
        ("comfirm_install_cert_tip", ""),
        ("RDP Settings", ""),
        ("Sort by", ""),
        ("New Connection", ""),
        ("Restore", ""),
        ("Minimize", ""),
        ("Maximize", ""),
        ("Your Device", ""),
        ("empty_recent_tip", ""),
        ("empty_favorite_tip", ""),
        ("empty_lan_tip", ""),
        ("empty_address_book_tip", ""),
        ("eg: admin", ""),
        ("Empty Username", ""),
        ("Empty Password", ""),
        ("Me", ""),
        ("identical_file_tip", ""),
        ("show_monitors_tip", ""),
        ("View Mode", ""),
        ("login_linux_tip", ""),
        ("verify_rustdesk_password_tip", ""),
        ("remember_account_tip", ""),
        ("os_account_desk_tip", ""),
        ("OS Account", ""),
        ("another_user_login_title_tip", ""),
        ("another_user_login_text_tip", ""),
        ("xorg_not_found_title_tip", ""),
        ("xorg_not_found_text_tip", ""),
        ("no_desktop_title_tip", ""),
        ("no_desktop_text_tip", ""),
        ("No need to elevate", ""),
        ("System Sound", ""),
        ("Default", ""),
        ("New RDP", ""),
        ("Fingerprint", ""),
        ("Copy Fingerprint", ""),
        ("no fingerprints", ""),
        ("Select a peer", ""),
        ("Select peers", ""),
        ("Plugins", ""),
        ("Uninstall", ""),
        ("Update", ""),
        ("Enable", ""),
        ("Disable", ""),
        ("Options", ""),
        ("resolution_original_tip", ""),
        ("resolution_fit_local_tip", ""),
        ("resolution_custom_tip", ""),
        ("Collapse toolbar", ""),
        ("Accept and Elevate", ""),
        ("accept_and_elevate_btn_tooltip", ""),
        ("clipboard_wait_response_timeout_tip", ""),
        ("Incoming connection", ""),
        ("Outgoing connection", ""),
        ("Exit", ""),
        ("Open", ""),
        ("logout_tip", ""),
        ("Service", ""),
        ("Start", ""),
        ("Stop", ""),
        ("exceed_max_devices", ""),
        ("Sync with recent sessions", ""),
        ("Sort tags", ""),
        ("Separate remote window", ""),
        ("separate window", ""),
        ("Move tab to new window", ""),
    ].iter().cloned().collect();
}
