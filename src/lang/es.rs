lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Estado"),
        ("Your Desktop", "Tu escritorio"),
        ("desk_tip", "Puedes acceder a tu escritorio con esta ID y contraseña."),
        ("Password", "Contraseña"),
        ("Ready", "Listo"),
        ("Established", "Establecido"),
        ("connecting_status", "Conexión a la red RustDesk en progreso..."),
        ("Enable service", "Habilitar Servicio"),
        ("Start service", "Iniciar Servicio"),
        ("Service is running", "El servicio se está ejecutando"),
        ("Service is not running", "El servicio no se está ejecutando"),
        ("not_ready_status", "No está listo. Comprueba tu conexión"),
        ("Control Remote Desktop", "Controlar escritorio remoto"),
        ("Transfer file", "Transferir archivo"),
        ("Connect", "Conectar"),
        ("Recent sessions", "Sesiones recientes"),
        ("Address book", "Directorio"),
        ("Confirmation", "Confirmación"),
        ("TCP tunneling", "Túnel TCP"),
        ("Remove", "Quitar"),
        ("Refresh random password", "Actualizar contraseña aleatoria"),
        ("Set your own password", "Establece tu propia contraseña"),
        ("Enable keyboard/mouse", "Habilitar teclado/ratón"),
        ("Enable clipboard", "Habilitar portapapeles"),
        ("Enable file transfer", "Habilitar transferencia de archivos"),
        ("Enable TCP tunneling", "Habilitar túnel TCP"),
        ("IP Whitelisting", "Direcciones IP admitidas"),
        ("ID/Relay Server", "Servidor ID/Relay"),
        ("Import server config", "Importar configuración de servidor"),
        ("Export Server Config", "Exportar configuración del servidor"),
        ("Import server configuration successfully", "Configuración de servidor importada con éxito"),
        ("Export server configuration successfully", "Configuración de servidor exportada con éxito"),
        ("Invalid server configuration", "Configuración de servidor incorrecta"),
        ("Clipboard is empty", "El portapapeles está vacío"),
        ("Stop service", "Detener servicio"),
        ("Change ID", "Cambiar ID"),
        ("Your new ID", "Tu nueva ID"),
        ("length %min% to %max%", "de %min% a %max% de longitud"),
        ("starts with a letter", "comenzar con una letra"),
        ("allowed characters", "Caracteres permitidos"),
        ("id_change_tip", "Solo puedes usar caracteres a-z, A-Z, 0-9 e _ (guion bajo). El primer carácter debe ser a-z o A-Z. La longitud debe estar entre 6 y 16 caracteres."),
        ("Website", "Sitio web"),
        ("About", "Acerca de"),
        ("Slogan_tip", "¡Hecho con corazón en este mundo caótico!"),
        ("Privacy Statement", "Declaración de privacidad"),
        ("Mute", "Silenciar"),
        ("Build Date", "Fecha de compilación"),
        ("Version", "¨Versión"),
        ("Home", "Inicio"),
        ("Audio Input", "Entrada de audio"),
        ("Enhancements", "Mejoras"),
        ("Hardware Codec", "Códec de hardware"),
        ("Adaptive bitrate", "Tasa de bits adaptativa"),
        ("ID Server", "Servidor de IDs"),
        ("Relay Server", "Servidor Relay"),
        ("API Server", "Servidor API"),
        ("invalid_http", "debe comenzar con http:// o https://"),
        ("Invalid IP", "IP incorrecta"),
        ("Invalid format", "Formato incorrecto"),
        ("server_not_support", "Aún no es compatible con el servidor"),
        ("Not available", "No disponible"),
        ("Too frequent", "Demasiado frecuente"),
        ("Cancel", "Cancelar"),
        ("Skip", "Omitir"),
        ("Close", "Cerrar"),
        ("Retry", "Reintentar"),
        ("OK", ""),
        ("Password Required", "Se requiere contraseña"),
        ("Please enter your password", "Por favor, introduzca su contraseña"),
        ("Remember password", "Recordar contraseña"),
        ("Wrong Password", "Contraseña incorrecta"),
        ("Do you want to enter again?", "¿Quieres volver a entrar?"),
        ("Connection Error", "Error de conexión"),
        ("Error", ""),
        ("Reset by the peer", "Restablecido por el par"),
        ("Connecting...", "Conectando..."),
        ("Connection in progress. Please wait.", "Conexión en curso. Espere por favor."),
        ("Please try 1 minute later", "Intente 1 minuto más tarde"),
        ("Login Error", "Error de inicio de sesión"),
        ("Successful", "Exitoso"),
        ("Connected, waiting for image...", "Conectado, esperando imagen..."),
        ("Name", "Nombre"),
        ("Type", "Tipo"),
        ("Modified", "Modificado"),
        ("Size", "Tamaño"),
        ("Show Hidden Files", "Mostrar archivos ocultos"),
        ("Receive", "Recibir"),
        ("Send", "Enviar"),
        ("Refresh File", "Actualizar archivo"),
        ("Local", ""),
        ("Remote", "Remoto"),
        ("Remote Computer", "Computadora remota"),
        ("Local Computer", "Computadora local"),
        ("Confirm Delete", "Confirmar eliminación"),
        ("Delete", "Eliminar"),
        ("Properties", "Propiedades"),
        ("Multi Select", "Selección múltiple"),
        ("Select All", "Seleccionar Todo"),
        ("Unselect All", "Deseleccionar Todo"),
        ("Empty Directory", "Directorio vacío"),
        ("Not an empty directory", "No es un directorio vacío"),
        ("Are you sure you want to delete this file?", "¿Estás seguro de que quieres eliminar este archivo?"),
        ("Are you sure you want to delete this empty directory?", "¿Estás seguro de que deseas eliminar este directorio vacío?"),
        ("Are you sure you want to delete the file of this directory?", "¿Estás seguro de que deseas eliminar el archivo de este directorio?"),
        ("Do this for all conflicts", "Haga esto para todos los conflictos"),
        ("This is irreversible!", "¡Esto es irreversible!"),
        ("Deleting", "Eliminando"),
        ("files", "archivos"),
        ("Waiting", "Esperando"),
        ("Finished", "Terminado"),
        ("Speed", "Velocidad"),
        ("Custom Image Quality", "Calidad de imagen personalizada"),
        ("Privacy mode", "Modo privado"),
        ("Block user input", "Bloquear entrada de usuario"),
        ("Unblock user input", "Desbloquear entrada de usuario"),
        ("Adjust Window", "Ajustar ventana"),
        ("Original", "Original"),
        ("Shrink", "Encoger"),
        ("Stretch", "Estirar"),
        ("Scrollbar", "Barra de desplazamiento"),
        ("ScrollAuto", "Desplazamiento automático"),
        ("Good image quality", "Buena calidad de imagen"),
        ("Balanced", "Equilibrado"),
        ("Optimize reaction time", "Optimizar el tiempo de reacción"),
        ("Custom", "Personalizado"),
        ("Show remote cursor", "Mostrar cursor remoto"),
        ("Show quality monitor", "Mostrar calidad del monitor"),
        ("Disable clipboard", "Deshabilitar portapapeles"),
        ("Lock after session end", "Bloquear después del final de la sesión"),
        ("Insert", "Insertar"),
        ("Insert Lock", "Insertar bloqueo"),
        ("Refresh", "Actualizar"),
        ("ID does not exist", "La ID no existe"),
        ("Failed to connect to rendezvous server", "No se pudo conectar al servidor de encuentro"),
        ("Please try later", "Por favor intente mas tarde"),
        ("Remote desktop is offline", "El escritorio remoto está desconectado"),
        ("Key mismatch", "La clave no coincide"),
        ("Timeout", "Tiempo agotado"),
        ("Failed to connect to relay server", "No se pudo conectar al servidor de retransmisión"),
        ("Failed to connect via rendezvous server", "No se pudo conectar a través del servidor de encuentro"),
        ("Failed to connect via relay server", "No se pudo conectar a través del servidor de retransmisión"),
        ("Failed to make direct connection to remote desktop", "No se pudo establecer la conexión directa con el escritorio remoto"),
        ("Set Password", "Configurar la contraseña"),
        ("OS Password", "Contraseña del sistema operativo"),
        ("install_tip", "Debido al Control de cuentas de usuario, es posible que RustDesk no funcione correctamente como escritorio remoto. Para evitar este problema, haga clic en el botón de abajo para instalar RustDesk a nivel de sistema."),
        ("Click to upgrade", "Clic para actualizar"),
        ("Click to download", "Clic para descargar"),
        ("Click to update", "Clic para refrescar"),
        ("Configure", "Configurar"),
        ("config_acc", "Para controlar su escritorio desde el exterior, debe otorgar permiso a RustDesk de \"Accesibilidad\"."),
        ("config_screen", "Para controlar su escritorio desde el exterior, debe otorgar permiso a RustDesk de \"Grabación de pantalla\"."),
        ("Installing ...", "Instalando ..."),
        ("Install", "Instalar"),
        ("Installation", "Instalación"),
        ("Installation Path", "Ruta de instalación"),
        ("Create start menu shortcuts", "Crear accesos directos en el menú de inicio"),
        ("Create desktop icon", "Crear icono de escritorio"),
        ("agreement_tip", "Al iniciar la instalación, acepta los términos del acuerdo de licencia."),
        ("Accept and Install", "Aceptar e instalar"),
        ("End-user license agreement", "Acuerdo de licencia de usuario final"),
        ("Generating ...", "Generando ..."),
        ("Your installation is lower version.", "Su instalación es una versión inferior."),
        ("not_close_tcp_tip", "No cierre esta ventana mientras esté usando el túnel"),
        ("Listening ...", "Escuchando ..."),
        ("Remote Host", "Anfitrión remoto"),
        ("Remote Port", "Puerto remoto"),
        ("Action", "Acción"),
        ("Add", "Agregar"),
        ("Local Port", "Puerto local"),
        ("Local Address", "Dirección Local"),
        ("Change Local Port", "Cambiar Puerto Local"),
        ("setup_server_tip", "Para una conexión más rápida, configure su propio servidor"),
        ("Too short, at least 6 characters.", "Demasiado corto, al menos 6 caracteres."),
        ("The confirmation is not identical.", "La confirmación no coincide."),
        ("Permissions", "Permisos"),
        ("Accept", "Aceptar"),
        ("Dismiss", "Cancelar"),
        ("Disconnect", "Desconectar"),
        ("Enable file copy and paste", "Permitir copiar y pegar archivos"),
        ("Connected", "Conectado"),
        ("Direct and encrypted connection", "Conexión directa y cifrada"),
        ("Relayed and encrypted connection", "Conexión retransmitida y cifrada"),
        ("Direct and unencrypted connection", "Conexión directa y sin cifrar"),
        ("Relayed and unencrypted connection", "Conexión retransmitida y sin cifrar"),
        ("Enter Remote ID", "Introduzca el ID remoto"),
        ("Enter your password", "Introduzca su contraseña"),
        ("Logging in...", "Iniciando sesión..."),
        ("Enable RDP session sharing", "Habilitar el uso compartido de sesiones RDP"),
        ("Auto Login", "Inicio de sesión automático"),
        ("Enable direct IP access", "Habilitar acceso IP directo"),
        ("Rename", "Renombrar"),
        ("Space", "Espacio"),
        ("Create desktop shortcut", "Crear acceso directo en el escritorio"),
        ("Change Path", "Cambiar ruta"),
        ("Create Folder", "Crear carpeta"),
        ("Please enter the folder name", "Por favor introduzca el nombre de la carpeta"),
        ("Fix it", "Resolver"),
        ("Warning", "Aviso"),
        ("Login screen using Wayland is not supported", "La pantalla de inicio de sesión con Wayland no es compatible"),
        ("Reboot required", "Reinicio requerido"),
        ("Unsupported display server", "Servidor de visualización no compatible"),
        ("x11 expected", "x11 necesario"),
        ("Port", "Puerto"),
        ("Settings", "Ajustes"),
        ("Username", "Nombre de usuario"),
        ("Invalid port", "Puerto incorrecto"),
        ("Closed manually by the peer", "Cerrado manualmente por el par"),
        ("Enable remote configuration modification", "Habilitar modificación remota de configuración"),
        ("Run without install", "Ejecutar sin instalar"),
        ("Connect via relay", ""),
        ("Always connect via relay", "Conéctese siempre a través de relay"),
        ("whitelist_tip", "Solo las direcciones IP autorizadas pueden conectarse a este escritorio"),
        ("Login", "Iniciar sesión"),
        ("Verify", "Verificar"),
        ("Remember me", "Recordarme"),
        ("Trust this device", "Confiar en este dispositivo"),
        ("Verification code", "Código de verificación"),
        ("verification_tip", "Se ha detectado un nuevo dispositivo y se ha enviado un código de verificación a la dirección de correo registrada. Introduzca el código de verificación para continuar con el inicio de sesión."),
        ("Logout", "Salir"),
        ("Tags", "Tags"),
        ("Search ID", "Buscar ID"),
        ("whitelist_sep", "Separados por coma, punto y coma, espacio o nueva línea"),
        ("Add ID", "Agregar ID"),
        ("Add Tag", "Agregar tag"),
        ("Unselect all tags", "Deseleccionar todos los tags"),
        ("Network error", "Error de red"),
        ("Username missed", "Olvidó su nombre de usuario"),
        ("Password missed", "Olvidó su contraseña"),
        ("Wrong credentials", "Credenciales incorrectas"),
        ("The verification code is incorrect or has expired", ""),
        ("Edit Tag", "Editar tag"),
        ("Forget Password", "Olvidar contraseña"),
        ("Favorites", "Favoritos"),
        ("Add to Favorites", "Agregar a favoritos"),
        ("Remove from Favorites", "Quitar de favoritos"),
        ("Empty", "Vacío"),
        ("Invalid folder name", "Nombre de carpeta incorrecto"),
        ("Socks5 Proxy", "Proxy Socks5"),
        ("Socks5/Http(s) Proxy", "Proxy Socks5/Http(s)"),
        ("Discovered", "Descubierto"),
        ("install_daemon_tip", "Para comenzar en el encendido, debe instalar el servicio del sistema."),
        ("Remote ID", "ID remoto"),
        ("Paste", "Pegar"),
        ("Paste here?", "¿Pegar aquí?"),
        ("Are you sure to close the connection?", "¿Estás seguro de cerrar la conexión?"),
        ("Download new version", "Descargar nueva versión"),
        ("Touch mode", "Modo táctil"),
        ("Mouse mode", "Modo ratón"),
        ("One-Finger Tap", "Toque con un dedo"),
        ("Left Mouse", "Botón izquierdo"),
        ("One-Long Tap", "Un toque largo"),
        ("Two-Finger Tap", "Toque con dos dedos"),
        ("Right Mouse", "Botón derecho"),
        ("One-Finger Move", "Movimiento con un dedo"),
        ("Double Tap & Move", "Toca dos veces y mueve"),
        ("Mouse Drag", "Arrastre de ratón"),
        ("Three-Finger vertically", "Tres dedos verticalmente"),
        ("Mouse Wheel", "Rueda de ratón"),
        ("Two-Finger Move", "Movimiento con dos dedos"),
        ("Canvas Move", "Movimiento de lienzo"),
        ("Pinch to Zoom", "Pellizcar para ampliar"),
        ("Canvas Zoom", "Ampliar lienzo"),
        ("Reset canvas", "Restablecer lienzo"),
        ("No permission of file transfer", "Sin permiso de transferencia de archivos"),
        ("Note", "Nota"),
        ("Connection", "Conexión"),
        ("Share Screen", "Compartir pantalla"),
        ("Chat", "Chat"),
        ("Total", "Total"),
        ("items", "items"),
        ("Selected", "Seleccionado"),
        ("Screen Capture", "Captura de pantalla"),
        ("Input Control", "Control de entrada"),
        ("Audio Capture", "Captura de audio"),
        ("File Connection", "Conexión de archivos"),
        ("Screen Connection", "Conexión de pantalla"),
        ("Do you accept?", "¿Aceptas?"),
        ("Open System Setting", "Configuración del sistema abierto"),
        ("How to get Android input permission?", "¿Cómo obtener el permiso de entrada de Android?"),
        ("android_input_permission_tip1", "Para que un dispositivo remoto controle su dispositivo Android a través del ratón o toque, debe permitir que RustDesk use el servicio de \"Accesibilidad\"."),
        ("android_input_permission_tip2", "Vaya a la página de configuración del sistema que se abrirá a continuación, busque y acceda a [Servicios instalados], active el servicio [RustDesk Input]."),
        ("android_new_connection_tip", "Se recibió una nueva solicitud de control para el dispositivo actual."),
        ("android_service_will_start_tip", "Habilitar la captura de pantalla iniciará automáticamente el servicio, lo que permitirá que otros dispositivos soliciten una conexión desde este dispositivo."),
        ("android_stop_service_tip", "Cerrar el servicio cerrará automáticamente todas las conexiones establecidas."),
        ("android_version_audio_tip", "La versión actual de Android no admite la captura de audio, actualice a Android 10 o posterior."),
        ("android_start_service_tip", ""),
        ("android_permission_may_not_change_tip", ""),
        ("Account", "Cuenta"),
        ("Overwrite", "Sobrescribir"),
        ("This file exists, skip or overwrite this file?", "Este archivo existe, ¿omitir o sobrescribir este archivo?"),
        ("Quit", "Salir"),
        ("Help", "Ayuda"),
        ("Failed", "Fallido"),
        ("Succeeded", "Logrado"),
        ("Someone turns on privacy mode, exit", "Alguien active el modo privacidad, salga"),
        ("Unsupported", "No soportado"),
        ("Peer denied", "Par denegado"),
        ("Please install plugins", "Instale complementos"),
        ("Peer exit", "Par salio"),
        ("Failed to turn off", "Error al apagar"),
        ("Turned off", "Apagado"),
        ("Language", "Idioma"),
        ("Keep RustDesk background service", "Dejar RustDesk como Servicio en 2do plano"),
        ("Ignore Battery Optimizations", "Ignorar optimizacioens de bateria"),
        ("android_open_battery_optimizations_tip", "Si deseas deshabilitar esta característica, por favor, ve a la página siguiente de ajustes, busca y entra en  [Batería] y desmarca [Sin restricción]"),
        ("Start on boot", ""),
        ("Start the screen sharing service on boot, requires special permissions", ""),
        ("Connection not allowed", "Conexión no disponible"),
        ("Legacy mode", "Modo heredado"),
        ("Map mode", "Modo mapa"),
        ("Translate mode", "Modo traducido"),
        ("Use permanent password", "Usar contraseña permamente"),
        ("Use both passwords", "Usar ambas contraseñas"),
        ("Set permanent password", "Establecer contraseña permamente"),
        ("Enable remote restart", "Habilitar reinicio remoto"),
        ("Restart remote device", "Reiniciar dispositivo"),
        ("Are you sure you want to restart", "¿Estás seguro de que deseas reiniciar?"),
        ("Restarting remote device", "Reiniciando dispositivo remoto"),
        ("remote_restarting_tip", "El dispositivo remoto se está reiniciando. Por favor cierre este mensaje y vuelva a conectarse con la contraseña peremanente en unos momentos."),
        ("Copied", "Copiado"),
        ("Exit Fullscreen", "Salir de pantalla completa"),
        ("Fullscreen", "Pantalla completa"),
        ("Mobile Actions", "Acciones móviles"),
        ("Select Monitor", "Seleccionar monitor"),
        ("Control Actions", "Acciones de control"),
        ("Display Settings", "Configuración de pantalla"),
        ("Ratio", "Relación"),
        ("Image Quality", "Calidad de imagen"),
        ("Scroll Style", "Estilo de desplazamiento"),
        ("Show Toolbar", ""),
        ("Hide Toolbar", ""),
        ("Direct Connection", "Conexión directa"),
        ("Relay Connection", "Conexión Relay"),
        ("Secure Connection", "Conexión segura"),
        ("Insecure Connection", "Conexión insegura"),
        ("Scale original", "Escala original"),
        ("Scale adaptive", "Escala adaptativa"),
        ("General", ""),
        ("Security", "Seguridad"),
        ("Theme", "Tema"),
        ("Dark Theme", "Tema Oscuro"),
        ("Light Theme", ""),
        ("Dark", "Oscuro"),
        ("Light", "Claro"),
        ("Follow System", "Tema del sistema"),
        ("Enable hardware codec", "Habilitar códec por hardware"),
        ("Unlock Security Settings", "Desbloquear ajustes de seguridad"),
        ("Enable audio", "Habilitar Audio"),
        ("Unlock Network Settings", "Desbloquear Ajustes de Red"),
        ("Server", "Servidor"),
        ("Direct IP Access", "Acceso IP Directo"),
        ("Proxy", ""),
        ("Apply", "Aplicar"),
        ("Disconnect all devices?", "¿Desconectar todos los dispositivos?"),
        ("Clear", "Borrar"),
        ("Audio Input Device", "Dispositivo de entrada de audio"),
        ("Use IP Whitelisting", "Usar lista de IPs admitidas"),
        ("Network", "Red"),
        ("Pin Toolbar", ""),
        ("Unpin Toolbar", ""),
        ("Recording", "Grabando"),
        ("Directory", "Directorio"),
        ("Automatically record incoming sessions", "Grabación automática de sesiones entrantes"),
        ("Change", "Cambiar"),
        ("Start session recording", "Comenzar grabación de sesión"),
        ("Stop session recording", "Detener grabación de sesión"),
        ("Enable recording session", "Habilitar grabación de sesión"),
        ("Enable LAN discovery", "Habilitar descubrimiento de LAN"),
        ("Deny LAN discovery", "Denegar descubrimiento de LAN"),
        ("Write a message", "Escribir un mensaje"),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", "Por favor, espera confirmación de UAC"),
        ("elevated_foreground_window_tip", "La ventana actual del escritorio remoto necesita privilegios elevados para funcionar, así que no puedes usar ratón y teclado temporalmente. Puedes solicitar al usuario remoto que minimize la ventana actual o hacer clic en el botón de elevación de la ventana de gestión de conexión. Para evitar este problema, se recomienda instalar el programa en el dispositivo remto."),
        ("Disconnected", "Desconectado"),
        ("Other", "Otro"),
        ("Confirm before closing multiple tabs", "Confirmar antes de cerrar múltiples pestañas"),
        ("Keyboard Settings", "Ajustes de teclado"),
        ("Full Access", "Acceso completo"),
        ("Screen Share", "Compartir pantalla"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland requiere Ubuntu 21.04 o una versión superior."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland requiere una versión superior de la distribución de Linux. Pruebe el escritorio X11 o cambie su sistema operativo."),
        ("JumpLink", "Ver"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Seleccione la pantalla que se compartirá (Operar en el lado del par)."),
        ("Show RustDesk", "Mostrar RustDesk"),
        ("This PC", "Este PC"),
        ("or", "o"),
        ("Continue with", "Continuar con"),
        ("Elevate", "Elevar privilegios"),
        ("Zoom cursor", "Ampliar cursor"),
        ("Accept sessions via password", "Aceptar sesiones a través de contraseña"),
        ("Accept sessions via click", "Aceptar sesiones a través de clic"),
        ("Accept sessions via both", "Aceptar sesiones a través de ambos"),
        ("Please wait for the remote side to accept your session request...", "Por favor, espere a que el lado remoto acepte su solicitud de sesión"),
        ("One-time Password", "Constaseña de un solo uso"),
        ("Use one-time password", "Usar contraseña de un solo uso"),
        ("One-time password length", "Longitud de la contraseña de un solo uso"),
        ("Request access to your device", "Solicitud de acceso a su dispositivo"),
        ("Hide connection management window", "Ocultar ventana de gestión de conexión"),
        ("hide_cm_tip", "Permitir ocultar solo si se aceptan sesiones a través de contraseña y usando contraseña permanente"),
        ("wayland_experiment_tip", "El soporte para Wayland está en fase experimental, por favor, use X11 si necesita acceso desatendido."),
        ("Right click to select tabs", "Clic derecho para seleccionar pestañas"),
        ("Skipped", "Omitido"),
        ("Add to address book", "Añadir al directorio"),
        ("Group", "Grupo"),
        ("Search", "Búsqueda"),
        ("Closed manually by web console", "Cerrado manualmente por la consola web"),
        ("Local keyboard type", "Tipo de teclado local"),
        ("Select local keyboard type", "Seleccionar tipo de teclado local"),
        ("software_render_tip", "Si tienes una gráfica Nvidia y la ventana remota se cierra inmediatamente, instalar el driver nouveau y elegir renderizado por software podría ayudar. Se requiere reiniciar la aplicación."),
        ("Always use software rendering", "Usar siempre renderizado por software"),
        ("config_input", "Para controlar el escritorio remoto con el teclado necesitas dar a RustDesk permisos de \"Monitorización de entrada\"."),
        ("config_microphone", "Para poder hablar de forma remota necesitas darle a RustDesk permisos de \"Grabar Audio\"."),
        ("request_elevation_tip", "También puedes solicitar elevación de privilegios si hay alguien en el lado remoto."),
        ("Wait", "Esperar"),
        ("Elevation Error", "Error de elevación de privilegios"),
        ("Ask the remote user for authentication", "Pida autenticación al usuario remoto"),
        ("Choose this if the remote account is administrator", "Elegir si la cuenta remota es de administrador"),
        ("Transmit the username and password of administrator", "Transmitir usuario y contraseña del administrador"),
        ("still_click_uac_tip", "Aún se necesita que el usuario remoto haga click en OK en la ventana UAC del RusDesk en ejecución."),
        ("Request Elevation", "Solicitar Elevación de privilegios"),
        ("wait_accept_uac_tip", "Por favor espere a que el usuario remoto acepte el diálogo UAC."),
        ("Elevate successfully", "Elevación de privilegios exitosa"),
        ("uppercase", "mayúsculas"),
        ("lowercase", "minúsculas"),
        ("digit", "dígito"),
        ("special character", "carácter especial"),
        ("length>=8", "longitud>=8"),
        ("Weak", "Débil"),
        ("Medium", "Media"),
        ("Strong", "Fuerte"),
        ("Switch Sides", "Intercambiar lados"),
        ("Please confirm if you want to share your desktop?", "Por favor, confirma si quieres compartir tu escritorio"),
        ("Display", "Pantalla"),
        ("Default View Style", "Estilo de vista predeterminado"),
        ("Default Scroll Style", "Estilo de desplazamiento predeterminado"),
        ("Default Image Quality", "Calidad de imagen predeterminada"),
        ("Default Codec", "Códec predeterminado"),
        ("Bitrate", "Tasa de bits"),
        ("FPS", ""),
        ("Auto", ""),
        ("Other Default Options", "Otras opciones predeterminadas"),
        ("Voice call", "Llamada de voz"),
        ("Text chat", "Chat de texto"),
        ("Stop voice call", "Detener llamada de voz"),
        ("relay_hint_tip", "Puede que no sea posible conectar directamente. Puedes tratar de conectar a través de relay. \nAdicionalmente, si quieres usar relay en el primer intento, puedes añadir el sufijo \"/r\" a la ID o seleccionar la opción \"Conectar siempre a través de relay\" en la tarjeta del par."),
        ("Reconnect", "Reconectar"),
        ("Codec", "Códec"),
        ("Resolution", "Resolución"),
        ("No transfers in progress", "No hay transferencias en curso"),
        ("Set one-time password length", "Establecer contraseña de un solo uso"),
        ("RDP Settings", "Ajustes RDP"),
        ("Sort by", "Ordenar por"),
        ("New Connection", "Nueva conexión"),
        ("Restore", "Restaurar"),
        ("Minimize", "Minimizar"),
        ("Maximize", "Maximizar"),
        ("Your Device", "Tu dispositivo"),
        ("empty_recent_tip", "¡Vaya, no hay conexiones recientes!\nEs hora de planificar una nueva."),
        ("empty_favorite_tip", "¿Sin pares favoritos aún?\nEncontremos uno al que conectarte y ¡añádelo a tus favoritos!"),
        ("empty_lan_tip", "Oh no, parece que aún no has descubierto ningún par."),
        ("empty_address_book_tip", "Parece que actualmente no hay pares en tu directorio."),
        ("eg: admin", "ej.: admin"),
        ("Empty Username", "Nombre de usuario vacío"),
        ("Empty Password", "Contraseña vacía"),
        ("Me", "Yo"),
        ("identical_file_tip", "Este archivo es idéntico al del par."),
        ("show_monitors_tip", "Mostrar monitores en la barra de herramientas"),
        ("View Mode", "Modo Vista"),
        ("login_linux_tip", "Necesitas iniciar sesión con la cueneta del Linux remoto para activar una sesión de escritorio X"),
        ("verify_rustdesk_password_tip", "Verificar la contraseña de RustDesk"),
        ("remember_account_tip", "Recordar esta cuenta"),
        ("os_account_desk_tip", "Esta cueneta se usa para iniciar sesión en el sistema operativo remoto y habilitar la sesión de escritorio en headless."),
        ("OS Account", "Cuenta del SO"),
        ("another_user_login_title_tip", "Otro usuario ya ha iniciado sesión"),
        ("another_user_login_text_tip", "Desconectar"),
        ("xorg_not_found_title_tip", "Xorg no hallado"),
        ("xorg_not_found_text_tip", "Por favor, instala Xorg"),
        ("no_desktop_title_tip", "No hay escritorio disponible"),
        ("no_desktop_text_tip", "Por favor, instala GNOME Desktop"),
        ("No need to elevate", "No es necesario elevar privilegios"),
        ("System Sound", "Sonido del Sistema"),
        ("Default", "Predeterminado"),
        ("New RDP", "Nuevo RDP"),
        ("Fingerprint", "Huella digital"),
        ("Copy Fingerprint", "Copiar huella digital"),
        ("no fingerprints", "sin huellas digitales"),
        ("Select a peer", "Seleccionar un par"),
        ("Select peers", "Seleccionar pares"),
        ("Plugins", "Complementos"),
        ("Uninstall", "Desinstalar"),
        ("Update", "Actualizar"),
        ("Enable", "Habilitar"),
        ("Disable", "Inhabilitar"),
        ("Options", "Opciones"),
        ("resolution_original_tip", "Resolución original"),
        ("resolution_fit_local_tip", "Ajustar resolución local"),
        ("resolution_custom_tip", "Resolución personalizada"),
        ("Collapse toolbar", "Contraer barra de herramientas"),
        ("Accept and Elevate", "Aceptar y Elevar"),
        ("accept_and_elevate_btn_tooltip", "Aceptar la conexión y elevar permisos UAC."),
        ("clipboard_wait_response_timeout_tip", "Tiempo de espera para copia agotado."),
        ("Incoming connection", "Conexión entrante"),
        ("Outgoing connection", "Conexión saliente"),
        ("Exit", "Salir"),
        ("Open", "Abrir"),
        ("logout_tip", "¿Seguro que deseas cerrar sesión?"),
        ("Service", "Servicio"),
        ("Start", "Iniciar"),
        ("Stop", "Detener"),
        ("exceed_max_devices", "Has alcanzado el máximo número de dispositivos administrados."),
        ("Sync with recent sessions", "Sincronizar con sesiones recientes"),
        ("Sort tags", "Ordenar etiquetas"),
        ("Open connection in new tab", "Abrir conexión en nueva pestaña"),
        ("Move tab to new window", "Mover pestaña a nueva ventana"),
        ("Can not be empty", "No puede estar vacío"),
        ("Already exists", "Ya existe"),
        ("Change Password", "Cambiar contraseña"),
        ("Refresh Password", "Refrescar contraseña"),
        ("ID", ""),
        ("Grid View", "Vista Cuadrícula"),
        ("List View", "Vista Lista"),
        ("Select", "Seleccionar"),
        ("Toggle Tags", "Alternar Etiquetas"),
        ("pull_ab_failed_tip", "No se ha podido refrescar el directorio"),
        ("push_ab_failed_tip", "No se ha podido sincronizar el directorio con el servidor"),
        ("synced_peer_readded_tip", "Los dispositivos presentes en sesiones recientes se sincronizarán con el directorio."),
        ("Change Color", "Cambiar Color"),
        ("Primary Color", "Color Primario"),
        ("HSV Color", "Color HSV"),
        ("Installation Successful!", "Instalación exitosa"),
        ("Installation failed!", "La instalación ha fallado"),
        ("Reverse mouse wheel", "Invertir rueda del ratón"),
        ("{} sessions", "{} sesiones"),
        ("scam_title", "Podrías estar siendo ESTAFADO!"),
        ("scam_text1", "Si estás al teléfono con alguien a quien NO conoces y en quien NO confías y te ha pedido que uses RustDesk e inicies el servicio, no lo hagas y cuelga inmediatamente."),
        ("scam_text2", "Probablemente son estafadores tratando de robar tu dinero o información privada."),
        ("Don't show again", "No mostrar de nuevo"),
        ("I Agree", "Estoy de acuerdo"),
        ("Decline", "Rechazar"),
        ("Timeout in minutes", "Tiempo de espera en minutos"),
        ("auto_disconnect_option_tip", "Cerrar sesiones entrantes automáticamente por inactividad del usuario."),
        ("Connection failed due to inactivity", "Desconectar automáticamente por inactividad."),
        ("Check for software update on startup", "Comprobar actualización al iniciar"),
        ("upgrade_rustdesk_server_pro_to_{}_tip", "¡Por favor, actualiza RustDesk Server Pro a la versión {} o superior"),
        ("pull_group_failed_tip", "No se ha podido refrescar el grupo"),
        ("Filter by intersection", "Filtrar por intersección"),
        ("Remove wallpaper during incoming sessions", "Quitar el fonde de pantalla durante sesiones entrantes"),
        ("Test", "Probar"),
        ("display_is_plugged_out_msg", "La pantalla está desconectada, cambia a la principal."),
        ("No displays", "No hay pantallas"),
        ("elevated_switch_display_msg", "Cambiar a la pantalla principal porque mútliples pantallas no están soportadas en modo elevado."),
        ("Open in new window", "Abrir en una nueva ventana"),
        ("Show displays as individual windows", "Mostrar pantallas como ventanas individuales"),
        ("Use all my displays for the remote session", "Usar todas mis pantallas para la sesión remota"),
        ("selinux_tip", "SELinux está activado en tu dispositivo, lo que puede hacer que RustDesk no se ejecute correctamente como lado controlado."),
        ("Change view", "Cambiar vista"),
        ("Big tiles", "Mosaicos grandes"),
        ("Small tiles", "Mosaicos pequeños"),
        ("List", "Lista"),
        ("Virtual display", "Pantalla virtual"),
        ("Plug out all", "Desconectar todo"),
        ("True color (4:4:4)", "Color real (4:4:4)"),
        ("Enable blocking user input", "Habilitar el bloqueo de la entrada del usuario"),
        ("id_input_tip", "Puedes introducir una ID, una IP directa o un dominio con un puerto (<dominio>:<puerto>).\nSi quieres acceder a un dispositivo en otro servidor, por favor añade la ip del servidor (<id>@<dirección_servidor>?key=<clave_valor>), por ejemplo,\n9123456234@192.168.16.1:21117?key=5Qbwsde3unUcJBtrx9ZkvUmwFNoExHzpryHuPUdqlWM=.\nSi quieres acceder a un dispositivo en un servidor público, por favor, introduce \"<id>@public\", la clave no es necesaria para un servidor público."),
        ("privacy_mode_impl_mag_tip", "Modo 1"),
        ("privacy_mode_impl_virtual_display_tip", "Modo 2"),
        ("Enter privacy mode", "Entrar al modo privado"),
        ("Exit privacy mode", "Salir del modo privado"),
        ("idd_not_support_under_win10_2004_tip", "El controlador de pantalla indirecto no está soportado. Se necesita Windows 10, versión 2004 o superior."),
        ("switch_display_elevated_connections_tip", "Cambiar a una pantalla no principal no está soportado en el modo elevado cuando hay múltiples conexiones. Por favor, inténtalo de nuevo tras la instalación si quieres controlar múltiples pantallas."),
        ("input_source_1_tip", "Fuente de entrada 1"),
        ("input_source_2_tip", "Fuente de entrada 2"),
        ("capture_display_elevated_connections_tip", "La captura de múltiples pantallas en el modo de usaurio con privilegios elevados no está soportada. Por favor, inténtalo de nuevo tras la instalación si quieres controlar múltiples pantallas."),
        ("Swap control-command key", "Intercambiar teclas control-comando"),
        ("swap-left-right-mouse", "Intercambiar botones derecho-izquierdo del ratón"),
        ("2FA code", "Código 2FA"),
        ("More", "Más"),
        ("enable-2fa-title", "Habilitar autenticación en dos pasos"),
        ("enable-2fa-desc", "Por favor, configura tu autenticador ahora. Puedes usar una app de autenticación como Authy, Microsoft o Google Authenticator en tu teléfono u ordenador.\n\nEscanea el código QR con tu app e introduce el código mostrado para habilitar la autenticación en dos pasos."),
        ("wrong-2fa-code", "No se puede verificar el código. Comprueba que el código y los ajustes de hora local son correctos"),
        ("enter-2fa-title", "Autenticación en dos pasos"),
        ("Email verification code must be 6 characters.", "El código de verificación por mail debe tener 6 caracteres"),
        ("2FA code must be 6 digits.", "El cóidigo 2FA debe tener 6 dígitos"),
        ("Multiple Windows sessions found", "Encontradas sesiones de múltiples ventanas"),
        ("Please select the session you want to connect to", "Por favor, seleccione la sesión a la que se desea conectar"),
        ("powered_by_me", "Con tecnología de RustDesk"),
        ("outgoing_only_desk_tip", "Esta es una edición personalizada.\nPuedes conectarte a otros dispositivos, pero ellos no pueden conectarse al tuyo."),
        ("preset_password_warning", "Esta edición personalizada viene con una contraseña preestablecida. Cualquiera que la conozca podrá tener control total de tu dispositivo.Si no es esto lo que esperabas, desinstala el software inmediatamente."),
        ("Security Alert", "Alerta de Seguridad"),
        ("My address book", "Mi directorio"),
        ("Personal", "Personal"),
        ("Owner", "Propietario"),
        ("Set shared password", "Establecer contraseña compartida"),
        ("Exist in", "Existe en"),
        ("Read-only", "Solo-lectura"),
        ("Read/Write", "Lectura/Escritura"),
        ("Full Control", "Control Total"),
        ("share_warning_tip", "Los campos mostrados arriba son compartidos y visibles por otros."),
        ("Everyone", "Todos"),
        ("ab_web_console_tip", "Más en consola web"),
        ("allow-only-conn-window-open-tip", "Permitir la conexión solo si la ventana RusDesk está abierta"),
        ("no_need_privacy_mode_no_physical_displays_tip", "No hay pantallas físicas, no es necesario usar el modo privado."),
        ("Follow remote cursor", "Seguir cursor remoto"),
        ("Follow remote window focus", "Seguir ventana remota activa"),
        ("default_proxy_tip", "El protocolo y puerto predeterminados es Socks5 y 1080"),
        ("no_audio_input_device_tip", "No se ha encontrado ningún dispositivo de entrada de autio."),
        ("Incoming", "Entrante"),
        ("Outgoing", "Saliente"),
        ("Clear Wayland screen selection", "Borrar la selección de pantalla Wayland"),
        ("clear_Wayland_screen_selection_tip", "Tras borrar la selección de pantalla, puedes volver a seleccionarla para compartir."),
        ("confirm_clear_Wayland_screen_selection_tip", "¿Seguro que deseas borrar la selección de pantalla Wayland?"),
        ("android_new_voice_call_tip", "Se ha recibido una nueva solicitud de llamada de voz. Si aceptas el audio cambiará a comunicación de voz."),
        ("texture_render_tip", ""),
        ("Use texture rendering", ""),
    ].iter().cloned().collect();
}
