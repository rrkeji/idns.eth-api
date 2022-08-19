lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Status"),
        ("Your Desktop", "dit skrivebord"),
        ("desk_tip", "Du kan få adgang til dit skrivebord med dette ID og adgangskode."),
        ("Password", "Kodeord"),
        ("Ready", "Klar"),
        ("Established", "Etableret"),
        ("connecting_status", "Opretter forbindelse til IdnsEth-netværket..."),
        ("Enable Service", "Tænd forbindelsesserveren"),
        ("Start Service", "Starte forbindelsesserveren"),
        ("Service is running", "Tjenesten kører"),
        ("Service is not running", "Den tilknyttede tjeneste kører ikke"),
        ("not_ready_status", "Ikke klar. Tjek venligst din forbindelse"),
        ("Control Remote Desktop", "Styr fjernskrivebord"),
        ("Transfer File", "Overføre fil"),
        ("Connect", "Forbind"),
        ("Recent Sessions", "Sidste sessioner"),
        ("Address Book", "Adressebog"),
        ("Confirmation", "Bekræftelse"),
        ("TCP Tunneling", "TCP tunneling"),
        ("Remove", "Fjern"),
        ("Refresh random password", "Opdater tilfældig adgangskode"),
        ("Set your own password", "Indstil din egen adgangskode"),
        ("Enable Keyboard/Mouse", "Tænd for tastatur/mus"),
        ("Enable Clipboard", "Tænd for udklipsholderen"),
        ("Enable File Transfer", "Aktiver filoverførsel"),
        ("Enable TCP Tunneling", "Slå TCP-tunneling til"),
        ("IP Whitelisting", "IP-udgivelsesliste"),
        ("ID/Relay Server", "ID/forbindelsesserver"),
        ("Stop service", "Sluk for forbindelsesserveren"),
        ("Change ID", "Ændre ID"),
        ("Website", "Hjemmeside"),
        ("About", "Omkring"),
        ("Mute", "Sluk for mikrofonen"),
        ("Audio Input", "Lydindgang"),
        ("Enhancements", ""),
        ("Hardware Codec", ""),
        ("Adaptive Bitrate", ""),
        ("ID Server", "identifikations Server"),
        ("Relay Server", "Relæ Server"),
        ("API Server", "API Server"),
        ("invalid_http", "Skal begynde med http:// eller https://"),
        ("Invalid IP", "Ugyldig IP-adresse"),
        ("id_change_tip", "Kun tegnene a-z, A-Z, 0-9 og _ (understregning) er tilladt. Det første bogstav skal være a-z, A-Z. Længde mellem 6 og 16."),
        ("Invalid format", "Ugyldigt format"),
        ("server_not_support", "Endnu ikke understøttet af serveren"),
        ("Not available", "ikke Tilgængelig"),
        ("Too frequent", "For ofte"),
        ("Cancel", "Abort"),
        ("Skip", "Spring over"),
        ("Close", "Luk"),
        ("Retry", "Prøv igen"),
        ("OK", "OK"),
        ("Password Required", "Adgangskode kræves"),
        ("Please enter your password", "Indtast venligst dit kodeord"),
        ("Remember password", "Husk kodeord"),
        ("Wrong Password", "Forkert kodeord"),
        ("Do you want to enter again?", "Vil du forbinde igen?"),
        ("Connection Error", "Forbindelsesfejl"),
        ("Error", "fejl"),
        ("Reset by the peer", "Nulstil ved peer"),
        ("Connecting...", "Opretter forbindelse..."),
        ("Connection in progress. Please wait.", "Forbindelsen er etableret. Vent venligst."),
        ("Please try 1 minute later", "Prøv igen, 1 minut senere"),
        ("Login Error", "Login fejl"),
        ("Successful", "Vellykket"),
        ("Connected, waiting for image...", "Tilsluttet, venter på billede..."),
        ("Name", "Navn"),
        ("Type", "Type"),
        ("Modified", "Ændret"),
        ("Size", "Størrelse"),
        ("Show Hidden Files", "Vis skjulte filer"),
        ("Receive", "Modtag"),
        ("Send", "Send"),
        ("Refresh File", "Genopfrisk fil"),
        ("Local", "Lokalt"),
        ("Remote", "Remote"),
        ("Remote Computer", "Fjern computer"),
        ("Local Computer", "Lokal Computer"),
        ("Confirm Delete", "Bekræft sletning"),
        ("Delete", "Slet"),
        ("Properties", "Egenskaber"),
        ("Multi Select", "Flere valg"),
        ("Empty Directory", "Tom bibliotek"),
        ("Not an empty directory", "Intet tomt bibliotek"),
        ("Are you sure you want to delete this file?", "Er du sikker på, at du vil slette denne fil?"),
        ("Are you sure you want to delete this empty directory?", "Er du sikker på, at du vil slette dette tomme bibliotek?"),
        ("Are you sure you want to delete the file of this directory?", "Er du sikker på, at du vil slette filen til dette bibliotek?"),
        ("Do this for all conflicts", "Gør dette for alle konflikter"),
        ("This is irreversible!", "Dette er irreversibelt!"),
        ("Deleting", "Sletter"),
        ("files", "Filer"),
        ("Waiting", "Venter"),
        ("Finished", "Færdig"),
        ("Speed", "hastighed"),
        ("Custom Image Quality", "Individuel billedkvalitet"),
        ("Privacy mode", "Databeskyttelsestilstand (Privatlivstilstand)"),
        ("Block user input", "Bloker brugerinput"),
        ("Unblock user input", "Fjern blokering af brugerinput"),
        ("Adjust Window", "Juster vinduet"),
        ("Original", "Original"),
        ("Shrink", "Krymp"),
        ("Stretch", "Strak"),
        ("Good image quality", "God billedkvalitet"),
        ("Balanced", "Afbalanceret"),
        ("Optimize reaction time", "Optimeret responstid"),
        ("Custom", "Brugerdefineret"),
        ("Show remote cursor", "Vis fjernbetjeningskontrolleret markør"),
        ("Show quality monitor", ""),
        ("Disable clipboard", "Deaktiver udklipsholder"),
        ("Lock after session end", "Lås efter afslutningen af fjernstyring"),
        ("Insert", "Indsæt"),
        ("Insert Lock", "Indsæt lås"),
        ("Refresh", "Genopfrisk"),
        ("ID does not exist", "ID findes ikke"),
        ("Failed to connect to rendezvous server", "Forbindelse til forbindelsesserveren mislykkedes"),
        ("Please try later", "Prøv det senere"),
        ("Remote desktop is offline", "Fjernet desktop er offline"),
        ("Key mismatch", "Nøgle uoverensstemmelse"),
        ("Timeout", "Timeout"),
        ("Failed to connect to relay server", "Forbindelse til relæ-serveren mislykkedes"),
        ("Failed to connect via rendezvous server", "Forbindelse via Rendezvous-server mislykkedes"),
        ("Failed to connect via relay server", "Forbindelse via relæ-serveren mislykkedes"),
        ("Failed to make direct connection to remote desktop", "Direkte forbindelse til fjernskrivebord kunne ikke etableres"),
        ("Set Password", "Indstil adgangskode"),
        ("OS Password", "Operativsystemadgangskode"),
        ("install_tip", "På grund af UAC kan Rustdesk ikke fungere korrekt på den anden side i nogle tilfælde. For at undgå UAC skal du klikke på knappen nedenfor for at installere Rustdesk på systemet"),
        ("Click to upgrade", "Klik for at opgradere"),
        ("Click to download", "Klik for at downloade"),
        ("Click to update", "Klik for at opdatere"),
        ("Configure", "Konfigurer"),
        ("config_acc", "For at kontrollere dit skrivebord på afstand skal du give Rustdesk \"Access \" Rettigheder."),
        ("config_screen", "For at kunne få adgang til dit skrivebord langtfra, skal du give Rustdesk \"skærmstøtte \" tilladelser."),
        ("Installing ...", "Installere ..."),
        ("Install", "installere"),
        ("Installation", "Installation"),
        ("Installation Path", "Installationsti"),
        ("Create start menu shortcuts", "Opret startmenu links"),
        ("Create desktop icon", "Opret skrivebords-symbol"),
        ("agreement_tip", "Hvis du starter installationen, skal du acceptere licensaftalen"),
        ("Accept and Install", "Accepter og installer"),
        ("End-user license agreement", "Licensaftale for slutbrugere"),
        ("Generating ...", "Generer kode ..."),
        ("Your installation is lower version.", "Din installation er en lavere version."),
        ("not_close_tcp_tip", "Luk ikke dette vindue, mens du bruger tunnelen."),
        ("Listening ...", "Lytter ..."),
        ("Remote Host", "Fjern-Host"),
        ("Remote Port", "Fjern-Port"),
        ("Action", "Рandling"),
        ("Add", "Tilføj"),
        ("Local Port", "Lokal Port"),
        ("setup_server_tip", "For en hurtigere forbindelse skal du indstille din egen forbindelsesserver"),
        ("Too short, at least 6 characters.", "For kort, mindst 6 tegn."),
        ("The confirmation is not identical.", "Bekræftelsen er ikke identisk."),
        ("Permissions", "Tilladelser"),
        ("Accept", "Acceptere"),
        ("Dismiss", "Afvise"),
        ("Disconnect", "Frakobl"),
        ("Allow using keyboard and mouse", "Tillad brug af tastatur og mus"),
        ("Allow using clipboard", "Tillad brug af udklipsholderen"),
        ("Allow hearing sound", "Tillader hørelse fra lyd"),
        ("Allow file copy and paste", "Tillad fil kopiering og indsættelse"),
        ("Connected", "Forbundet"),
        ("Direct and encrypted connection", "Direkte og krypteret forbindelse"),
        ("Relayed and encrypted connection", "Brugt relæet og krypteret forbindelse"),
        ("Direct and unencrypted connection", "Direkte og ukrypteret forbindelse"),
        ("Relayed and unencrypted connection", "Brugt relæet og ukrypteret forbindelse"),
        ("Enter Remote ID", "Indtast Remote-ID"),
        ("Enter your password", "Skriv dit kodeord"),
        ("Logging in...", "Logger ind..."),
        ("Enable RDP session sharing", "RDP-Aktivér sessiongodkendelse"),
        ("Auto Login", "Automatisk login (kun gyldigt hvis du har konfigureret \"Lock efter afslutningen af sessionen\")"),
        ("Enable Direct IP Access", "Aktivér direkte IP-adgang"),
        ("Rename", "Omdøb"),
        ("Space", "Plads"),
        ("Create Desktop Shortcut", "Opret skrivebords-genvej"),
        ("Change Path", "Skift stien"),
        ("Create Folder", "Opret mappe"),
        ("Please enter the folder name", "Indtast venligst mappenavnet"),
        ("Fix it", "Kør reparation"),
        ("Warning", "Advarsel"),
        ("Login screen using Wayland is not supported", "Registreringsskærm med Wayland understøttes ikke"),
        ("Reboot required", "Genstart krævet"),
        ("Unsupported display server ", "Ikke-understøttet displayserver"),
        ("x11 expected", "X11 Forventet"),
        ("Port", "Port"),
        ("Settings", "Indstillinger"),
        ("Username", " Brugernavn"),
        ("Invalid port", "Ugyldig port"),
        ("Closed manually by the peer", "Manuelt lukket af peer"),
        ("Enable remote configuration modification", "Tillad at ændre afstandskonfigurationen"),
        ("Run without install", "Kør uden installation"),
        ("Always connected via relay", "Tilslut altid via relæ-server"),
        ("Always connect via relay", "Forbindelse via relæ-server"),
        ("whitelist_tip", "Kun IP'er på udgivelseslisten kan få adgang til mig"),
        ("Login", "Login"),
        ("Logout", "logger af"),
        ("Tags", "Nøgleord"),
        ("Search ID", "Søg ID"),
        ("Current Wayland display server is not supported", "Den aktuelle Wayland-Anzege-server understøttes ikke"),
        ("whitelist_sep", "Adskilt af komma, semikolon, rum eller linjepaus"),
        ("Add ID", "Tilføj ID"),
        ("Add Tag", "Tilføj nøgleord"),
        ("Unselect all tags", "Fravælg alle nøgleord"),
        ("Network error", "Netværksfejl"),
        ("Username missed", "Benutzername fehlt"),
        ("Password missed", "Glemt kodeord"),
        ("Wrong credentials", "Forkerte registreringsdata"),
        ("Edit Tag", "Rediger nøgleord"),
        ("Unremember Password", "Bemærk ikke adgangskoden"),
        ("Favorites", "Favorit"),
        ("Add to Favorites", "Tilføj til favoritter"),
        ("Remove from Favorites", "Fjern favoritter"),
        ("Empty", "Tom"),
        ("Invalid folder name", "Ugyldigt mappenavn"),
        ("Socks5 Proxy", "Socks5 Proxy"),
        ("Hostname", "Computernavn"),
        ("Discovered", "Fundet"),
        ("install_daemon_tip", "Til at begynde med opstart, skal du installere systemtjenesten"),
        ("Remote ID", "Fjern ID"),
        ("Paste", "Indsæt"),
        ("Paste here?", "Indsæt her?"),
        ("Are you sure to close the connection?", "Sind Sie sicher, dass Sie die Verbindung schließen wollen?"),
        ("Download new version", "Neue Version herunterladen"),
        ("Touch mode", "Touch-tilstand"),
        ("Mouse mode", "Musse-tilstand"),
        ("One-Finger Tap", "En fingerspids-tap"),
        ("Left Mouse", "Venstre mus"),
        ("One-Long Tap", "Tryk med en finger lang"),
        ("Two-Finger Tap", "Tryk med to fingre-tap"),
        ("Right Mouse", "Højre mus"),
        ("One-Finger Move", "En fingerbevægelse"),
        ("Double Tap & Move", "Dobbelt og flytte"),
        ("Mouse Drag", "Mus"),
        ("Three-Finger vertically", "Tre fingre lodret"),
        ("Mouse Wheel", "Mussehjul"),
        ("Two-Finger Move", "To fingreflytning"),
        ("Canvas Move", "Flyt lærred"),
        ("Pinch to Zoom", "Zoom ind"),
        ("Canvas Zoom", "Lærred zoom"),
        ("Reset canvas", "Nulstil skærm"),
        ("No permission of file transfer", "Ingen tilladelse til at overføre filen"),
        ("Note", "Note"),
        ("Connection", "Forbindelse"),
        ("Share Screen", "Del skærmen"),
        ("CLOSE", "LUK"),
        ("OPEN", "ÅBEN"),
        ("Chat", "Chat"),
        ("Total", "Total"),
        ("items", "artikel"),
        ("Selected", "Valgte"),
        ("Screen Capture", "Skærmoptagelse"),
        ("Input Control", "Inputkontrol"),
        ("Audio Capture", "Lydoptagelse"),
        ("File Connection", "Filforbindelse"),
        ("Screen Connection", "Færdiggørelse"),
        ("Do you accept?", "Accepterer du?"),
        ("Open System Setting", "Åbn systemindstillingen"),
        ("How to get Android input permission?", "Hvordan får jeg en Android-input tilladelse?"),
        ("android_input_permission_tip1", "For at en ekstern enhed kan kontrollere din Android-enhed via mus eller berøring, skal du give Rustdesk mulighed for at bruge tjenesten \"tilgængelighed \"."),
        ("android_input_permission_tip2", "Gå til den næste systemindstillingsside, søg og indtast [installerede tjenester], tænd for [Rustdesk Input] Service."),
        ("android_new_connection_tip", "En ny kontrolanmodning blev modtaget, der gerne ville kontrollere din nuværende enhed."),
        ("android_service_will_start_tip", "Ved at tænde for skærmoptagelsen startes tjenesten automatisk, så andre enheder kan anmode om en forbindelse fra denne enhed."),
        ("android_stop_service_tip", "Ved at lukke tjenesten lukkes alle fremstillede forbindelser automatisk."),
        ("android_version_audio_tip", "Den aktuelle Android -version understøtter ikke lydoptagelse, skal du opdatere om Android 10 eller højere."),
        ("android_start_service_tip", "Tryk på [Start Service] eller åbn autorisationen [skærmoptagelse] for at starte skærmudgivelsen."),
        ("Account", "Konto"),
        ("Overwrite", "Overskriv"),
        ("This file exists, skip or overwrite this file?", "Denne fil findes, springer over denne fil eller overskriver?"),
        ("Quit", "Afslut"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "Hjælp"),
        ("Failed", "Mislykkedet"),
        ("Succeeded", "Vellykket"),
        ("Someone turns on privacy mode, exit", "Nogen aktiverede databeskyttelsestilstand, slut"),
        ("Unsupported", "Ikke understøttet"),
        ("Peer denied", "Peer nægtet"),
        ("Please install plugins", "Venligst Installer plugins"),
        ("Peer exit", "Peer-Afslut"),
        ("Failed to turn off", "Slukke"),
        ("Turned off", "Slukket"),
        ("In privacy mode", "I databeskyttelsestilstand"),
        ("Out privacy mode", "Databeskyttelsestilstand fra"),
        ("Language", ""),
        ("Keep IdnsEth background service", ""),
        ("Ignore Battery Optimizations", ""),
        ("android_open_battery_optimizations_tip", ""),
        ("Connection not allowed", ""),
        ("Use temporary password", ""),
        ("Use permanent password", ""),
        ("Use both passwords", ""),
        ("Set permanent password", ""),
        ("Set temporary password length", ""),
        ("Enable Remote Restart", ""),
        ("Allow remote restart", ""),
        ("Restart Remote Device", ""),
        ("Are you sure you want to restart", ""),
        ("Restarting Remote Device", ""),
        ("remote_restarting_tip", ""),
    ].iter().cloned().collect();
}
