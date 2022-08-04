lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Estado"),
        ("Your Desktop", "Tu escritorio"),
        ("desk_tip", "Tu escritorio puede ser ingresado con esta ID y contraseña."),
        ("Password", "Contraseña"),
        ("Ready", "Listo"),
        ("Established", "Establecido"),
        ("connecting_status", "Conexión a la red RustDesk en progreso..."),
        ("Enable Service", "Habilitar Servicio"),
        ("Start Service", "Iniciar Servicio"),
        ("Service is running", "El servicio se está ejecutando"),
        ("Service is not running", "El servicio no se está ejecutando"),
        ("not_ready_status", "No está listo. Comprueba tu conexión"),
        ("Control Remote Desktop", "Controlar Escritorio Remoto"),
        ("Transfer File", "Transferir archivo"),
        ("Connect", "Conectar"),
        ("Recent Sessions", "Sesiones recientes"),
        ("Address Book", "Directorio"),
        ("Confirmation", "Confirmación"),
        ("TCP Tunneling", "Tunel TCP"),
        ("Remove", "Remover"),
        ("Refresh random password", "Actualizar contraseña aleatoria"),
        ("Set your own password", "Establece tu propia contraseña"),
        ("Enable Keyboard/Mouse", "Habilitar teclado/ratón"),
        ("Enable Clipboard", "Habilitar portapapeles"),
        ("Enable File Transfer", "Habilitar transferencia de archivos"),
        ("Enable TCP Tunneling", "Habilitar tunel TCP"),
        ("IP Whitelisting", "Lista blanca de IP"),
        ("ID/Relay Server", "Servidor ID/Relay"),
        ("Stop service", "Parar servicio"),
        ("Change ID", "Cambiar ID"),
        ("Website", "Sitio web"),
        ("About", "Acerca de"),
        ("Mute", "Silencio"),
        ("Audio Input", "Entrada de audio"),
        ("Enhancements", "mejoras"),
        ("Hardware Codec", ""),
        ("Adaptive Bitrate", ""),
        ("ID Server", "ID server"),
        ("Relay Server", "Server relay"),
        ("API Server", "Server API"),
        ("invalid_http", "debe comenzar con http:// o https://"),
        ("Invalid IP", "IP inválida"),
        ("id_change_tip", "Solo puedes usar caracteres a-z, A-Z, 0-9 e _ (guion bajo). El primer carácter debe ser a-z o A-Z. La longitud debe estar entre 6 a 16 caracteres."),
        ("Invalid format", "Formato inválido"),
        ("server_not_support", "Aún no es compatible con el servidor"),
        ("Not available", "No disponible"),
        ("Too frequent", "Demasiado frecuente"),
        ("Cancel", "Cancelar"),
        ("Skip", "Saltar"),
        ("Close", "Cerrar"),
        ("Retry", "Reintentar"),
        ("OK", "OK"),
        ("Password Required", "Se requiere contraseña"),
        ("Please enter your password", "Por favor, introduzca su contraseña"),
        ("Remember password", "Recordar contraseña"),
        ("Wrong Password", "Contraseña incorrecta"),
        ("Do you want to enter again?", "Quieres volver a entrar?"),
        ("Connection Error", "Error de conexión"),
        ("Error", "Error"),
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
        ("Local", "Local"),
        ("Remote", "Remoto"),
        ("Remote Computer", "Computadora remota"),
        ("Local Computer", "Computadora local"),
        ("Confirm Delete", "Confirmar eliminación"),
        ("Delete", "Borrar"),
        ("Properties", "Propiedades"),
        ("Multi Select", "Selección múltiple"),
        ("Empty Directory", "Directorio vacío"),
        ("Not an empty directory", "No es un directorio vacío"),
        ("Are you sure you want to delete this file?", "Estás seguro de que quieres eliminar este archivo?"),
        ("Are you sure you want to delete this empty directory?", "Está seguro de que desea eliminar este directorio vacío?"),
        ("Are you sure you want to delete the file of this directory?", "Está seguro de que desea eliminar el archivo de este directorio?"),
        ("Do this for all conflicts", "Haga esto para todos los conflictos"),
        ("This is irreversible!", "Esto es irreversible!"),
        ("Deleting", "Borrando"),
        ("files", "archivos"),
        ("Waiting", "Esperando"),
        ("Finished", "Acabado"),
        ("Speed", "Velocidad"),
        ("Custom Image Quality", "Calidad de imagen personalizada"),
        ("Privacy mode", "Modo privado"),
        ("Block user input", "Bloquear entrada de usuario"),
        ("Unblock user input", "Desbloquear entrada de usuario"),
        ("Adjust Window", "Ajustar ventana"),
        ("Original", "Original"),
        ("Shrink", "Encogerse"),
        ("Stretch", "Estirar"),
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
        ("ID does not exist", "ID no existe"),
        ("Failed to connect to rendezvous server", "No se pudo conectar al servidor de encuentro"),
        ("Please try later", "Por favor intente mas tarde"),
        ("Remote desktop is offline", "El escritorio remoto está desconectado"),
        ("Key mismatch", "La clave no coincide"),
        ("Timeout", "Timeout"),
        ("Failed to connect to relay server", "No se pudo conectar al servidor de retransmisión"),
        ("Failed to connect via rendezvous server", "No se pudo conectar a través del servidor de encuentro"),
        ("Failed to connect via relay server", "No se pudo conectar a través del servidor de retransmisión"),
        ("Failed to make direct connection to remote desktop", "No se pudo establecer la conexión directa con el escritorio remoto"),
        ("Set Password", "Configurar la clave"),
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
        ("Create start menu shortcuts", "Crear accesos directos al menú de inicio"),
        ("Create desktop icon", "Crear icono de escritorio"),
        ("agreement_tip", "Al iniciar la instalación, acepta los términos del acuerdo de licencia."),
        ("Accept and Install", "Aceptar e instalar"),
        ("End-user license agreement", "Acuerdo de licencia de usuario final"),
        ("Generating ...", "Generando ..."),
        ("Your installation is lower version.", "Su instalación es una versión inferior."),
        ("not_close_tcp_tip", "No cierre esta ventana mientras esté usando el túnel"),
        ("Listening ...", "Escuchando ..."),
        ("Remote Host", "Servidor remoto"),
        ("Remote Port", "Puerto remoto"),
        ("Action", "Acción"),
        ("Add", "Agregar"),
        ("Local Port", "Puerto local"),
        ("setup_server_tip", "Para una conexión más rápida, configure su propio servidor"),
        ("Too short, at least 6 characters.", "Demasiado corto, al menos 6 caracteres."),
        ("The confirmation is not identical.", "La confirmación no es idéntica."),
        ("Permissions", "Permisos"),
        ("Accept", "Aceptar"),
        ("Dismiss", "Cancelar"),
        ("Disconnect", "Desconectar"),
        ("Allow using keyboard and mouse", "Permitir el uso del teclado y el mouse"),
        ("Allow using clipboard", "Permitir usar portapapeles"),
        ("Allow hearing sound", "Permitir escuchar sonido"),
        ("Allow file copy and paste", "Permitir copiar y pegar archivos"),
        ("Connected", "Conectado"),
        ("Direct and encrypted connection", "Conexión directa y encriptada"),
        ("Relayed and encrypted connection", "Conexión retransmitida y cifrada"),
        ("Direct and unencrypted connection", "Conexión directa y sin cifrar"),
        ("Relayed and unencrypted connection", "Conexión retransmitida y sin cifrar"),
        ("Enter Remote ID", "Ingrese el ID remoto"),
        ("Enter your password", "Ingrese su contraseña"),
        ("Logging in...", "Iniciando sesión..."),
        ("Enable RDP session sharing", "Habilitar el uso compartido de sesiones RDP"),
        ("Auto Login", "Ingreso automático"),
        ("Enable Direct IP Access", "Habilitar acceso IP directo"),
        ("Rename", "Renombrar"),
        ("Space", "Espacio"),
        ("Create Desktop Shortcut", "Crear acceso directo del escritorio"),
        ("Change Path", "Cambiar ruta"),
        ("Create Folder", "Crear carpeta"),
        ("Please enter the folder name", "Por favor ingrese el nombre de la carpeta"),
        ("Fix it", "Resolver"),
        ("Warning", "Aviso"),
        ("Login screen using Wayland is not supported", "La pantalla de inicio de sesión con Wayland no es compatible"),
        ("Reboot required", "Reinicio requerido"),
        ("Unsupported display server ", "Servidor de visualización no compatible"),
        ("x11 expected", "x11 necesario"),
        ("Port", "Puerto"),
        ("Settings", "Ajustes"),
        ("Username", " Nombre de usuario"),
        ("Invalid port", "Puerto inválido"),
        ("Closed manually by the peer", "Cerrado manualmente por el par"),
        ("Enable remote configuration modification", "Habilitar modificación de configuración remota"),
        ("Run without install", "Ejecutar sin instalar"),
        ("Always connected via relay", "Siempre conectado a través de relay"),
        ("Always connect via relay", "Conéctese siempre a través de relay"),
        ("whitelist_tip", "Solo las direcciones IP autorizadas pueden conectarse a este escritorio"),
        ("Login", "Iniciar sesión"),
        ("Logout", "Salir"),
        ("Tags", "Tags"),
        ("Search ID", "Buscar ID"),
        ("Current Wayland display server is not supported", "El servidor de visualización actual de Wayland no es compatible"),
        ("whitelist_sep", "Separados por coma, punto y coma, espacio o nueva línea"),
        ("Add ID", "Agregar ID"),
        ("Add Tag", "Agregar tag"),
        ("Unselect all tags", "Deseleccionar todos los tags"),
        ("Network error", "Error de red"),
        ("Username missed", "Olvidó su nombre de usuario"),
        ("Password missed", "Olvidó su contraseña"),
        ("Wrong credentials", "Credenciales incorrectas"),
        ("Edit Tag", "Editar tag"),
        ("Unremember Password", "Olvidaste tu contraseña"),
        ("Favorites", "Favoritos"),
        ("Add to Favorites", "Agregar a favoritos"),
        ("Remove from Favorites", "Quitar de favoritos"),
        ("Empty", "Vacío"),
        ("Invalid folder name", "Nombre de carpeta no válido"),
        ("Socks5 Proxy", "Proxy Socks5"),
        ("Hostname", "Nombre de host"),
        ("Discovered", "Descubierto"),
        ("install_daemon_tip", "Para comenzar en el encendido, debe instalar el servicio del sistema."),
        ("Remote ID", "ID remoto"),
        ("Paste", "Pegar"),
        ("Paste here?", "Pegar aqui?"),
        ("Are you sure to close the connection?", "Estás seguro de cerrar la conexión?"),
        ("Download new version", "Descargar nueva versión"),
        ("Touch mode", "Modo táctil"),
        ("Mouse mode", "Modo ratón"),
        ("One-Finger Tap", "Toque con un dedo"),
        ("Left Mouse", "Ratón izquierdo"),
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
        ("CLOSE", "CERRAR"),
        ("OPEN", "ABRIR"),
        ("Chat", "Chat"),
        ("Total", "Total"),
        ("items", "items"),
        ("Selected", "Seleccionado"),
        ("Screen Capture", "Captura de pantalla"),
        ("Input Control", "Control de entrada"),
        ("Audio Capture", "Captura de audio"),
        ("File Connection", "Conexión de archivos"),
        ("Screen Connection", "Conexión de pantalla"),
        ("Do you accept?", "Aceptas?"),
        ("Open System Setting", "Configuración del sistema abierto"),
        ("How to get Android input permission?", "Cómo obtener el permiso de entrada de Android?"),
        ("android_input_permission_tip1", "Para que un dispositivo remoto controle su dispositivo Android a través del mouse o toque, debe permitir que RustDesk use el servicio de \"Accesibilidad\"."),
        ("android_input_permission_tip2", "Vaya a la página de configuración del sistema que se abrirá a continuación, busque y acceda a [Servicios instalados], active el servicio [RustDesk Input]."),
        ("android_new_connection_tip", "Se recibió una nueva solicitud de control para el dispositivo actual."),
        ("android_service_will_start_tip", "Habilitar la captura de pantalla iniciará automáticamente el servicio, lo que permitirá que otros dispositivos soliciten una conexión desde este dispositivo."),
        ("android_stop_service_tip", "Cerrar el servicio cerrará automáticamente todas las conexiones establecidas."),
        ("android_version_audio_tip", "La versión actual de Android no admite la captura de audio, actualice a Android 10 o posterior."),
        ("android_start_service_tip", "Toque el permiso [Iniciar servicio] o ABRIR [Captura de pantalla] para iniciar el servicio de uso compartido de pantalla."),
        ("Account", "Cuenta"),
        ("Overwrite", "Sobrescribir"),
        ("This file exists, skip or overwrite this file?", "Este archivo existe, ¿omitir o sobrescribir este archivo?"),
        ("Quit", "Salir"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "Ayuda"),
        ("Failed", "Fallido"),
        ("Succeeded", "Logrado"),
        ("Someone turns on privacy mode, exit", "Alguien active el modo privacidad, salga"),
        ("Unsupported", "No soportado"),
        ("Peer denied", "Par negado"),
        ("Please install plugins", "Instale complementos"),
        ("Peer exit", "Par salio"),
        ("Failed to turn off", "Error al apagar"),
        ("Turned off", "Apagado"),
        ("In privacy mode", "En modo de privacidad"),
        ("Out privacy mode", "Fuera del modo de privacidad"),
        ("Language", "Idioma"),
        ("Keep RustDesk background service", "Dejar RustDesk como Servicio en 2do plano"),
        ("Ignore Battery Optimizations", "Ignorar optimizacioens de bateria"),
        ("android_open_battery_optimizations_tip", ""),
        ("Random Password After Session", ""),
        ("Keep", ""),
        ("Update", ""),
        ("Disable", ""),
        ("Onetime Password", ""),
        ("Verification Method", ""),
        ("Enable security password", ""),
        ("Enable random password", ""),
        ("Enable onetime password", ""),
        ("Disable onetime password", ""),
        ("Activate onetime password", ""),
        ("Set security password", ""),
        ("Connection not allowed", ""),
        ("Legacy mode", ""),
        ("Map mode", ""),
        ("Translate mode", ""),
        ("Connection not allowed", "Conexión no disponible"),
        ("Use temporary password", "Usar contraseña temporal"),
        ("Use permanent password", "Usar contraseña permamente"),
        ("Use both passwords", "Usar ambas comtraseñas"),
        ("Set permanent password", "Establecer contraseña permamente"),
        ("Set temporary password length", "Establecer largo de contraseña temporal"),
        ("Enable Remote Restart", "Activar reinicio remoto"),
        ("Allow remote restart", "Permitir reinicio remoto"),
        ("Restart Remote Device", "Reiniciar dispositivo"),
        ("Are you sure you want to restart", "Esta Seguro que desea reiniciar?"),
        ("Restarting Remote Device", "Reiniciando dispositivo remoto"),
        ("remote_restarting_tip", "Dispositivo remoto reiniciando, favor de cerrar este mensaje y reconectarse con la contraseña permamente despues de un momento."),
    ].iter().cloned().collect();
}
