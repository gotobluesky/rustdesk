lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Durum"),
        ("Your Desktop", "Sizin Masaüstünüz"),
        ("desk_tip", "Masaüstünüze bu ID ve şifre ile erişilebilir"),
        ("Password", "Şifre"),
        ("Ready", "Hazır"),
        ("Established", "Bağlantı sağlandı"),
        ("connecting_status", "Bağlanılıyor "),
        ("Enable Service", "Servisi aktif et"),
        ("Start Service", "Servisi başlat"),
        ("Service is running", "Servis çalışıyor"),
        ("Service is not running", "Servis çalışmıyor"),
        ("not_ready_status", "Hazır değil. Bağlantınızı kontrol edin"),
        ("Control Remote Desktop", "Bağlanılacak Uzak Bağlantı ID"),
        ("Transfer File", "Dosya transferi"),
        ("Connect", "Bağlan"),
        ("Recent Sessions", "Son Bağlanılanlar"),
        ("Address Book", "Adres Defteri"),
        ("Confirmation", "Onayla"),
        ("TCP Tunneling", "TCP Tünelleri"),
        ("Remove", "Kaldır"),
        ("Refresh random password", "Yeni rastgele şifre oluştur"),
        ("Set your own password", "Kendi şifreni oluştur"),
        ("Enable Keyboard/Mouse", "Klavye ve Fareye izin ver"),
        ("Enable Clipboard", "Kopyalanan geçici veriye izin ver"),
        ("Enable File Transfer", "Dosya Transferine izin ver"),
        ("Enable TCP Tunneling", "TCP Tüneline izin ver"),
        ("IP Whitelisting", "İzinli IP listesi"),
        ("ID/Relay Server", "ID/Relay Sunucusu"),
        ("Import Server Config", "Sunucu ayarlarını içe aktar"),
        ("Export Server Config", "Sunucu Yapılandırmasını Dışa Aktar"),
        ("Import server configuration successfully", "Sunucu ayarları başarıyla içe aktarıldı"),
        ("Export server configuration successfully", "Sunucu yapılandırmasını başarıyla dışa aktar"),
        ("Invalid server configuration", "Geçersiz sunucu ayarı"),
        ("Clipboard is empty", "Kopyalanan geçici veri boş"),
        ("Stop service", "Servisi Durdur"),
        ("Change ID", "ID Değiştir"),
        ("Your new ID", ""),
        ("length %min% to %max%", ""),
        ("starts with a letter", ""),
        ("allowed characters", ""),
        ("id_change_tip", "Yalnızca a-z, A-Z, 0-9 ve _ (alt çizgi) karakterlerini kullanabilirsiniz. İlk karakter a-z veya A-Z olmalıdır. Uzunluk 6 ile 16 karakter arasında olmalıdır."),
        ("Website", "Website"),
        ("About", "Hakkında"),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "Sustur"),
        ("Build Date", ""),
        ("Version", ""),
        ("Home", ""),
        ("Audio Input", "Ses Girişi"),
        ("Enhancements", "Geliştirmeler"),
        ("Hardware Codec", "Donanımsal Codec"),
        ("Adaptive Bitrate", "Uyarlanabilir Bit Hızı"),
        ("ID Server", "ID Sunucu"),
        ("Relay Server", "Relay Sunucu"),
        ("API Server", "API Sunucu"),
        ("invalid_http", "http:// veya https:// ile başlamalıdır"),
        ("Invalid IP", "Geçersiz IP adresi"),
        ("Invalid format", "Hatalı Format"),
        ("server_not_support", "Henüz sunucu tarafından desteklenmiyor"),
        ("Not available", "Erişilebilir değil"),
        ("Too frequent", "Çok sık"),
        ("Cancel", "İptal"),
        ("Skip", "Geç"),
        ("Close", "Kapat"),
        ("Retry", "Tekrar Dene"),
        ("OK", "Tamam"),
        ("Password Required", "Şifre Gerekli"),
        ("Please enter your password", "Lütfen şifrenizi giriniz"),
        ("Remember password", "Şifreyi hatırla"),
        ("Wrong Password", "Hatalı şifre"),
        ("Do you want to enter again?", "Tekrar giriş yapmak ister misiniz?"),
        ("Connection Error", "Bağlantı Hatası"),
        ("Error", "Hata"),
        ("Reset by the peer", "Eş tarafında sıfırla"),
        ("Connecting...", "Bağlanılıyor..."),
        ("Connection in progress. Please wait.", "Bağlantı sağlanıyor. Lütfen bekleyiniz."),
        ("Please try 1 minute later", "Lütfen 1 dakika sonra tekrar deneyiniz"),
        ("Login Error", "Giriş Hatalı"),
        ("Successful", "Başarılı"),
        ("Connected, waiting for image...", "Bağlandı. Görüntü bekleniyor..."),
        ("Name", "Ad"),
        ("Type", "Tip"),
        ("Modified", "Değiştirildi"),
        ("Size", "Boyut"),
        ("Show Hidden Files", "Gizli Dosyaları Göster"),
        ("Receive", "Al"),
        ("Send", "Gönder"),
        ("Refresh File", "Dosyayı yenile"),
        ("Local", "Yerel"),
        ("Remote", "Uzak"),
        ("Remote Computer", "Uzak Bilgisayar"),
        ("Local Computer", "Yerel Bilgisayar"),
        ("Confirm Delete", "Silmeyi Onayla"),
        ("Delete", "Sil"),
        ("Properties", "Özellikler"),
        ("Multi Select", "Çoklu Seçim"),
        ("Select All", "Tümünü Seç"),
        ("Unselect All", "Tüm Seçimi Kaldır"),
        ("Empty Directory", "Boş Klasör"),
        ("Not an empty directory", "Klasör boş değil"),
        ("Are you sure you want to delete this file?", "Bu dosyayı silmek istediğinize emin misiniz?"),
        ("Are you sure you want to delete this empty directory?", "Bu boş klasörü silmek istediğinize emin misiniz?"),
        ("Are you sure you want to delete the file of this directory?", "Bu klasördeki dosyayı silmek istediğinize emin misiniz?"),
        ("Do this for all conflicts", "Bunu tüm çakışmalar için yap"),
        ("This is irreversible!", "Bu işlem geri döndürülemez!"),
        ("Deleting", "Siliniyor"),
        ("files", "dosyalar"),
        ("Waiting", "Bekleniyor"),
        ("Finished", "Tamamlandı"),
        ("Speed", "Hız"),
        ("Custom Image Quality", "Özel Görüntü Kalitesi"),
        ("Privacy mode", "Gizlilik modu"),
        ("Block user input", "Kullanıcı girişini engelle"),
        ("Unblock user input", "Kullanı girişine izin ver"),
        ("Adjust Window", "Pencereyi Ayarla"),
        ("Original", "Orjinal"),
        ("Shrink", "Küçült"),
        ("Stretch", "Uzat"),
        ("Scrollbar", "Kaydırma çubuğu"),
        ("ScrollAuto", "Otomatik Kaydır"),
        ("Good image quality", "İyi görüntü kalitesi"),
        ("Balanced", "Dengelenmiş"),
        ("Optimize reaction time", "Tepki süresini optimize et"),
        ("Custom", "Özel"),
        ("Show remote cursor", "Uzaktaki fare imlecini göster"),
        ("Show quality monitor", "Kalite monitörünü göster"),
        ("Disable clipboard", "Hafızadaki kopyalanmışları engelle"),
        ("Lock after session end", "Bağlantıdan sonra kilitle"),
        ("Insert", "Ekle"),
        ("Insert Lock", "Kilit Ekle"),
        ("Refresh", "Yenile"),
        ("ID does not exist", "ID bulunamadı"),
        ("Failed to connect to rendezvous server", "ID oluşturma sunucusuna bağlanılamadı"),
        ("Please try later", "Dağa sonra tekrar deneyiniz"),
        ("Remote desktop is offline", "Uzak masaüstü kapalı"),
        ("Key mismatch", "Anahtar uyumlu değil"),
        ("Timeout", "Zaman aşımı"),
        ("Failed to connect to relay server", "Relay sunucusuna bağlanılamadı"),
        ("Failed to connect via rendezvous server", "ID oluşturma sunucusuna bağlanılamadı"),
        ("Failed to connect via relay server", "Relay oluşturma sunucusuna bağlanılamadı"),
        ("Failed to make direct connection to remote desktop", "Uzak masaüstüne doğrudan bağlantı kurulamadı"),
        ("Set Password", "Şifre ayarla"),
        ("OS Password", "İşletim Sistemi Şifresi"),
        ("install_tip", "Kullanıcı Hesabı Denetimi nedeniyle, RustDesk bir uzak masaüstü olarak düzgün çalışmayabilir. Bu sorunu önlemek için, RustDesk'i sistem seviyesinde kurmak için aşağıdaki butona tıklayın."),
        ("Click to upgrade", "Yükseltmek için tıklayınız"),
        ("Click to download", "İndirmek için tıklayınız"),
        ("Click to update", "Güncellemek için tıklayınız"),
        ("Configure", "Ayarla"),
        ("config_acc", "Masaüstünüzü dışarıdan kontrol etmek için RustDesk'e \"Erişilebilirlik\""),
        ("config_screen", "Masaüstünüzü dışarıdan kontrol etmek için RustDesk'e \"Ekran Kaydı\" iznini vermeniz gerekir."),
        ("Installing ...", "Yükleniyor ..."),
        ("Install", "Yükle"),
        ("Installation", "Kurulum"),
        ("Installation Path", "Kurulacak olan konum"),
        ("Create start menu shortcuts", "Başlangıca kısayol oluştur"),
        ("Create desktop icon", "Masaüstüne kısayol oluştur"),
        ("agreement_tip", "Kurulumu başlatarak, lisans sözleşmesinin şartlarını kabul etmiş olursunuz."),
        ("Accept and Install", "Kabul Et ve Yükle"),
        ("End-user license agreement", "Son kullanıcı lisans anlaşması"),
        ("Generating ...", "Oluşturuluyor..."),
        ("Your installation is lower version.", "Kurulumunuz alt sürümdür."),
        ("not_close_tcp_tip", "Tüneli kullanırken bu pencereyi kapatmayın"),
        ("Listening ...", "Dinleniyor..."),
        ("Remote Host", "Uzak Sunucu"),
        ("Remote Port", "Uzak Port"),
        ("Action", "Eylem"),
        ("Add", "Ekle"),
        ("Local Port", "Yerel Port"),
        ("Local Address", "Yerel Adres"),
        ("Change Local Port", "Yerel Port'u Değiştir"),
        ("setup_server_tip", "Daha hızlı bağlantı için kendi sunucunuzu kurun"),
        ("Too short, at least 6 characters.", "Çok kısa en az 6 karakter gerekli."),
        ("The confirmation is not identical.", "Doğrulama yapılamadı."),
        ("Permissions", "İzinler"),
        ("Accept", "Kabul Et"),
        ("Dismiss", "Reddet"),
        ("Disconnect", "Bağlanıyı kes"),
        ("Allow using keyboard and mouse", "Klavye ve fare kullanımına izin ver"),
        ("Allow using clipboard", "Pano kullanımına izin ver"),
        ("Allow hearing sound", "Sesi duymaya izin ver"),
        ("Allow file copy and paste", "Dosya kopyalamaya ve yapıştırmaya izin ver"),
        ("Connected", "Bağlandı"),
        ("Direct and encrypted connection", "Doğrudan ve şifreli bağlantı"),
        ("Relayed and encrypted connection", "Aktarmalı ve şifreli bağlantı"),
        ("Direct and unencrypted connection", "Doğrudan ve şifrelenmemiş bağlantı"),
        ("Relayed and unencrypted connection", "Aktarmalı ve şifrelenmemiş bağlantı"),
        ("Enter Remote ID", "Uzak ID'yi Girin"),
        ("Enter your password", "Şifrenizi girin"),
        ("Logging in...", "Giriş yapılıyor..."),
        ("Enable RDP session sharing", "RDP oturum paylaşımını etkinleştir"),
        ("Auto Login", "Otomatik giriş"),
        ("Enable Direct IP Access", "Doğrudan IP Erişimini Etkinleştir"),
        ("Rename", "Yeniden adlandır"),
        ("Space", "Boşluk"),
        ("Create Desktop Shortcut", "Masaüstü kısayolu oluşturun"),
        ("Change Path", "Yolu değiştir"),
        ("Create Folder", "Klasör oluşturun"),
        ("Please enter the folder name", "Lütfen klasör adını girin"),
        ("Fix it", "Düzenle"),
        ("Warning", "Uyarı"),
        ("Login screen using Wayland is not supported", "Wayland kullanan giriş ekranı desteklenmiyor"),
        ("Reboot required", "Yeniden başlatma gerekli"),
        ("Unsupported display server ", "Desteklenmeyen görüntü sunucusu"),
        ("x11 expected", "x11 bekleniyor"),
        ("Port", "Port"),
        ("Settings", "Ayarlar"),
        ("Username", "Kullanıcı Adı"),
        ("Invalid port", "Geçersiz port"),
        ("Closed manually by the peer", "Eş tarafından manuel olarak kapatıldı"),
        ("Enable remote configuration modification", "Uzaktan yapılandırma değişikliğini etkinleştir"),
        ("Run without install", "Yüklemeden çalıştır"),
        ("Connect via relay", ""),
        ("Always connect via relay", "Always connect via relay"),
        ("whitelist_tip", "Bu masaüstüne yalnızca yetkili IP adresleri bağlanabilir"),
        ("Login", "Giriş yap"),
        ("Verify", ""),
        ("Remember me", ""),
        ("Trust this device", ""),
        ("Verification code", ""),
        ("verification_tip", ""),
        ("Logout", "Çıkış yap"),
        ("Tags", "Etiketler"),
        ("Search ID", "ID Arama"),
        ("whitelist_sep", "Virgül, noktalı virgül, boşluk veya yeni satır ile ayrılmış"),
        ("Add ID", "ID Ekle"),
        ("Add Tag", "Etiket Ekle"),
        ("Unselect all tags", "Tüm etiketlerin seçimini kaldır"),
        ("Network error", "Bağlantı hatası"),
        ("Username missed", "Kullanıcı adı boş"),
        ("Password missed", "Şifre boş"),
        ("Wrong credentials", "Yanlış kimlik bilgileri"),
        ("Edit Tag", "Etiketi düzenle"),
        ("Unremember Password", "Şifreyi Unut"),
        ("Favorites", "Favoriler"),
        ("Add to Favorites", "Favorilere ekle"),
        ("Remove from Favorites", "Favorilerden çıkar"),
        ("Empty", "Boş"),
        ("Invalid folder name", "Geçersiz klasör adı"),
        ("Socks5 Proxy", "Socks5 Proxy"),
        ("Hostname", "Ana bilgisayar adı"),
        ("Discovered", "Keşfedilenler"),
        ("install_daemon_tip", "Başlangıçta başlamak için sistem hizmetini yüklemeniz gerekir."),
        ("Remote ID", "Uzak ID"),
        ("Paste", "Yapıştır"),
        ("Paste here?", "Buraya yapıştır?"),
        ("Are you sure to close the connection?", "Bağlantıyı kapatmak istediğinize emin misiniz?"),
        ("Download new version", "Yeni sürümü indir"),
        ("Touch mode", "Dokunmatik mod"),
        ("Mouse mode", "Fare modu"),
        ("One-Finger Tap", "Tek Parmakla Dokunma"),
        ("Left Mouse", "Sol Fare"),
        ("One-Long Tap", "Tek-Uzun Dokunma"),
        ("Two-Finger Tap", "İki-Parmak Dokunma"),
        ("Right Mouse", "Sağ Fare"),
        ("One-Finger Move", "Tek Parmakla Hareket"),
        ("Double Tap & Move", "Çift Dokun ve Taşı"),
        ("Mouse Drag", "Fare Sürükleme"),
        ("Three-Finger vertically", "Dikey olarak üç parmak"),
        ("Mouse Wheel", "Fare Tekerliği"),
        ("Two-Finger Move", "İki Parmakla Hareket"),
        ("Canvas Move", "Tuval Hareketi"),
        ("Pinch to Zoom", "İki parmakla yakınlaştır"),
        ("Canvas Zoom", "Tuval Yakınlaştırma"),
        ("Reset canvas", "Tuvali sıfırla"),
        ("No permission of file transfer", "Dosya aktarımı izni yok"),
        ("Note", "Not"),
        ("Connection", "Bağlantı"),
        ("Share Screen", "Ekranı Paylaş"),
        ("CLOSE", "KAPAT"),
        ("OPEN", "AÇ"),
        ("Chat", "Mesajlaş"),
        ("Total", "Toplam"),
        ("items", "öğeler"),
        ("Selected", "Seçildi"),
        ("Screen Capture", "Ekran görüntüsü"),
        ("Input Control", "Giriş Kontrolü"),
        ("Audio Capture", "Ses Yakalama"),
        ("File Connection", "Dosya Bağlantısı"),
        ("Screen Connection", "Ekran Bağlantısı"),
        ("Do you accept?", "Kabul ediyor musun?"),
        ("Open System Setting", "Sistem Ayarını Aç"),
        ("How to get Android input permission?", "Android giriş izni nasıl alınır?"),
        ("android_input_permission_tip1", "Uzak bir cihazın Android cihazınızı fare veya dokunma yoluyla kontrol edebilmesi için, RustDesk'in \"Erişilebilirlik\" özelliğini kullanmasına izin vermelisiniz."),
        ("android_input_permission_tip2", "Sonraki sistem ayarları sayfasına gidin, [Yüklü Hizmetler]'i bulun ve erişin, [RustDesk Girişi] hizmetini etkinleştirin."),
        ("android_new_connection_tip", "Yeni bir kontrol talebi alındı, cihazınızı kontrol etmesine izin verilsin mi."),
        ("android_service_will_start_tip", "Ekran Yakalamanın etkinleştirilmesi, hizmeti otomatik olarak başlatacak ve diğer cihazların bu cihazdan bağlantı talep etmesine izin verecektir."),
        ("android_stop_service_tip", "Hizmetin kapatılması, kurulan tüm bağlantıları otomatik olarak kapatacaktır."),
        ("android_version_audio_tip", "Mevcut Android sürümü ses yakalamayı desteklemiyor, lütfen Android 10 veya sonraki bir sürüme yükseltin."),
        ("android_start_service_tip", "Ekran paylaşım hizmetini başlatmak için [Hizmeti Başlat] veya AÇ [Ekran Yakalama] iznine dokunun."),
        ("Account", "Hesap"),
        ("Overwrite", "üzerine yaz"),
        ("This file exists, skip or overwrite this file?", "Bu dosya var, bu dosya atlansın veya üzerine yazılsın mı?"),
        ("Quit", "Çıkış"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "Yardım"),
        ("Failed", "Arızalı"),
        ("Succeeded", "başarılı"),
        ("Someone turns on privacy mode, exit", "Birisi gizlilik modunu açarsa, çık"),
        ("Unsupported", "desteklenmiyor"),
        ("Peer denied", "eş reddedildi"),
        ("Please install plugins", "Lütfen eklentileri yükleyin"),
        ("Peer exit", "eş çıkışı"),
        ("Failed to turn off", "kapatılamadı"),
        ("Turned off", "Kapatıldı"),
        ("In privacy mode", "Gizlilik modunda"),
        ("Out privacy mode", "Gizlilik modu dışında"),
        ("Language", "Dil"),
        ("Keep RustDesk background service", "RustDesk arka plan hizmetini sürdürün"),
        ("Ignore Battery Optimizations", "Pil Optimizasyonlarını Yoksay"),
        ("android_open_battery_optimizations_tip", ""),
        ("Start on Boot", ""),
        ("Start the screen sharing service on boot, requires special permissions", ""),
        ("Connection not allowed", "bağlantıya izin verilmedi"),
        ("Legacy mode", "Eski mod"),
        ("Map mode", "Haritalama modu"),
        ("Translate mode", "Çeviri modu"),
        ("Use permanent password", "Kalıcı şifre kullan"),
        ("Use both passwords", "İki şifreyide kullan"),
        ("Set permanent password", "Kalıcı şifre oluştur"),
        ("Enable Remote Restart", "Uzaktan yeniden başlatmayı aktif et"),
        ("Allow remote restart", "Uzaktan yeniden başlatmaya izin ver"),
        ("Restart Remote Device", "Uzaktaki cihazı yeniden başlat"),
        ("Are you sure you want to restart", "Yeniden başlatmak istediğinize emin misin?"),
        ("Restarting Remote Device", "Uzaktan yeniden başlatılıyor"),
        ("remote_restarting_tip", "remote_restarting_tip"),
        ("Copied", "Kopyalandı"),
        ("Exit Fullscreen", "Tam ekrandan çık"),
        ("Fullscreen", "Tam ekran"),
        ("Mobile Actions", "Mobil İşlemler"),
        ("Select Monitor", "Monitörü Seç"),
        ("Control Actions", "Kontrol Eylemleri"),
        ("Display Settings", "Görüntü ayarları"),
        ("Ratio", "Oran"),
        ("Image Quality", "Görüntü kalitesi"),
        ("Scroll Style", "Kaydırma Stili"),
        ("Show Menubar", "Menü çubuğunu göster"),
        ("Hide Menubar", "menü çubuğunu gizle"),
        ("Direct Connection", "Doğrudan Bağlantı"),
        ("Relay Connection", "Röle Bağlantısı"),
        ("Secure Connection", "Güvenli bağlantı"),
        ("Insecure Connection", "Güvenli Bağlantı"),
        ("Scale original", "Orijinali ölçeklendir"),
        ("Scale adaptive", "Ölçek uyarlanabilir"),
        ("General", "Genel"),
        ("Security", "Güvenlik"),
        ("Theme", "Tema"),
        ("Dark Theme", "Koyu Tema"),
        ("Light Theme", ""),
        ("Dark", "Koyu"),
        ("Light", "Açık"),
        ("Follow System", "Sisteme Uy"),
        ("Enable hardware codec", "Donanımsal codec aktif et"),
        ("Unlock Security Settings", "Güvenlik Ayarlarını Aç"),
        ("Enable Audio", "Sesi Aktif Et"),
        ("Unlock Network Settings", "Ağ Ayarlarını Aç"),
        ("Server", "Sunucu"),
        ("Direct IP Access", "Direk IP Erişimi"),
        ("Proxy", "Vekil"),
        ("Apply", "Uygula"),
        ("Disconnect all devices?", "Tüm cihazların bağlantısını kes?"),
        ("Clear", "Temizle"),
        ("Audio Input Device", "Ses Giriş Aygıtı"),
        ("Deny remote access", "Uzak erişime izin verme"),
        ("Use IP Whitelisting", "IP Beyaz Listeyi Kullan"),
        ("Network", "Ağ"),
        ("Enable RDP", "RDP Aktif Et"),
        ("Pin menubar", "Menü çubuğunu sabitle"),
        ("Unpin menubar", "Menü çubuğunun sabitlemesini kaldır"),
        ("Recording", "Kayıt Ediliyor"),
        ("Directory", "Klasör"),
        ("Automatically record incoming sessions", "Gelen oturumları otomatik olarak kayıt et"),
        ("Change", "Değiştir"),
        ("Start session recording", "Oturum kaydını başlat"),
        ("Stop session recording", "Oturum kaydını sonlandır"),
        ("Enable Recording Session", "Kayıt Oturumunu Aktif Et"),
        ("Allow recording session", "Oturum kaydına izin ver"),
        ("Enable LAN Discovery", "Yerel Ağ Keşfine İzin Ver"),
        ("Deny LAN Discovery", "Yerl Ağ Keşfine İzin Verme"),
        ("Write a message", "Bir mesaj yazın"),
        ("Prompt", "İstem"),
        ("Please wait for confirmation of UAC...", "UAC onayı için lütfen bekleyiniz..."),
        ("elevated_foreground_window_tip", "elevated_foreground_window_tip"),
        ("Disconnected", "Bağlantı Kesildi"),
        ("Other", "Diğer"),
        ("Confirm before closing multiple tabs", "Çoklu sekmeleri kapatmadan önce onayla"),
        ("Keyboard Settings", "Klavye Ayarları"),
        ("Full Access", "Tam Erişim"),
        ("Screen Share", "Ekran Paylaşımı"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland, Ubuntu 21.04 veya daha yüksek bir sürüm gerektirir."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland, linux dağıtımının daha yüksek bir sürümünü gerektirir. Lütfen X11 masaüstünü deneyin veya işletim sisteminizi değiştirin."),
        ("JumpLink", "View"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Lütfen paylaşılacak ekranı seçiniz (Ekran tarafında çalıştırın)."),
        ("Show RustDesk", "RustDesk'i Göster"),
        ("This PC", "Bu PC"),
        ("or", "veya"),
        ("Continue with", "bununla devam et"),
        ("Elevate", "Yükseltme"),
        ("Zoom cursor", "Yakınlaştırma imleci"),
        ("Accept sessions via password", "Oturumları parola ile kabul etme"),
        ("Accept sessions via click", "Tıklama yoluyla oturumları kabul edin"),
        ("Accept sessions via both", "Her ikisi aracılığıyla oturumları kabul edin"),
        ("Please wait for the remote side to accept your session request...", "Lütfen uzak tarafın oturum isteğinizi kabul etmesini bekleyin..."),
        ("One-time Password", "Tek Kullanımlık Şifre"),
        ("Use one-time password", "Tek seferlik parola kullanın"),
        ("One-time password length", "Tek seferlik şifre uzunluğu"),
        ("Request access to your device", "Cihazınıza erişim talep edin"),
        ("Hide connection management window", "Bağlantı yönetimi penceresini gizle"),
        ("hide_cm_tip", ""),
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
        ("Set temporary password length", ""),
    ].iter().cloned().collect();
}
