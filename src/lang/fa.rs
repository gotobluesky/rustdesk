lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "وضعیت"),
        ("Your Desktop", "دسکتاپ شما"),
        ("desk_tip", "دسکتاپ شما با این شناسه و رمز عبور قابل دسترسی است"),
        ("Password", "رمز عبور"),
        ("Ready", "آماده به کار"),
        ("Established", "اتصال برقرار شد"),
        ("connecting_status", "...در حال برقراری ارتباط با سرور"),
        ("Enable Service", "فعالسازی سرویس"),
        ("Start Service", "اجرای سرویس"),
        ("Service is running", "سرویس در حال اجرا است"),
        ("Service is not running", "سرویس اجرا نشده"),
        ("not_ready_status", "ارتباط برقرار نشد. لطفا شبکه خود را بررسی کنید"),
        ("Control Remote Desktop", "کنترل دسکتاپ میزبان"),
        ("Transfer File", "انتقال فایل"),
        ("Connect", "اتصال"),
        ("Recent Sessions", "جلسات اخیر"),
        ("Address Book", "دفترچه آدرس"),
        ("Confirmation", "تایید"),
        ("TCP Tunneling", "TCP تانل"),
        ("Remove", "حذف"),
        ("Refresh random password", "بروزرسانی رمز عبور تصادفی"),
        ("Set your own password", "!رمز عبور دلخواه بگذارید"),
        ("Enable Keyboard/Mouse", " فعالسازی ماوس/صفحه کلید"),
        ("Enable Clipboard", "فعال سازی کلیپبورد"),
        ("Enable File Transfer", "انتقال فایل را فعال کنید"),
        ("Enable TCP Tunneling", "را فعال کنید TCP تانل"),
        ("IP Whitelisting", "های مجاز IP لیست"),
        ("ID/Relay Server", "ID/Relay سرور"),
        ("Import Server Config", "تنظیم سرور با فایل"),
        ("Export Server Config", "ایجاد فایل تظیمات از سرور فعلی"),
        ("Import server configuration successfully", "تنظیمات سرور با فایل کانفیگ با موفقیت انجام شد"),
        ("Export server configuration successfully", "ایجاد فایل کانفیگ از تنظیمات فعلی با موفقیت انجام شد"),
        ("Invalid server configuration", "تنظیمات سرور نامعتبر است"),
        ("Clipboard is empty", "کلیپبورد خالی است"),
        ("Stop service", "توقف سرویس"),
        ("Change ID", "تعویض شناسه"),
        ("Website", "وب سایت"),
        ("About", "درباره"),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "بستن صدا"),
        ("Audio Input", "ورودی صدا"),
        ("Enhancements", "بهبودها"),
        ("Hardware Codec", "کدک سخت افزاری"),
        ("Adaptive Bitrate", ""),
        ("ID Server", "شناسه سرور"),
        ("Relay Server", "Relay سرور"),
        ("API Server", "API سرور"),
        ("invalid_http", "شروع شود http:// یا https:// باید با"),
        ("Invalid IP", "نامعتبر است IP آدرس"),
        ("id_change_tip", "شناسه باید طبق این شرایط باشد : حروف کوچک و بزرگ انگلیسی و اعداد از 0 تا 9، _ و همچنین حرف اول آن فقط حروف بزرگ یا کوچک انگلیسی و طول آن بین 6 الی 16 کاراکتر باشد"),
        ("Invalid format", "فرمت نادرست است"),
        ("server_not_support", "هنوز توسط سرور مورد نظر پشتیبانی نمی شود"),
        ("Not available", "در دسترسی نیست"),
        ("Too frequent", "خیلی رایج"),
        ("Cancel", "لغو"),
        ("Skip", "رد کردن"),
        ("Close", "بستن"),
        ("Retry", "تلاش مجدد"),
        ("OK", "قبول"),
        ("Password Required", "رمز عبور لازم است"),
        ("Please enter your password", "رمز عبور خود را وارد کنید"),
        ("Remember password", "رمز عبور را به خاطر بسپار"),
        ("Wrong Password", "رمز عبور اشتباه است"),
        ("Do you want to enter again?", "آیا میخواهید مجددا وارد شوید؟"),
        ("Connection Error", "خطا در اتصال"),
        ("Error", "خطا"),
        ("Reset by the peer", "توسط میزبان حذف شد"),
        ("Connecting...", "...در حال اتصال"),
        ("Connection in progress. Please wait.", "در حال اتصال. لطفا متظر بمانید"),
        ("Please try 1 minute later", "لطفا بعد از 1 دقیقه مجددا تلاش کنید"),
        ("Login Error", "ورود ناموفق بود"),
        ("Successful", "ورود با موفقیت انجام شد"),
        ("Connected, waiting for image...", "...ارتباط برقرار شد. انتظار برای دریافت تصاویر"),
        ("Name", "نام"),
        ("Type", "نوع فایل"),
        ("Modified", "تاریخ تغییر"),
        ("Size", "سایز"),
        ("Show Hidden Files", "نمایش فایل های مخفی"),
        ("Receive", "دریافت"),
        ("Send", "ارسال"),
        ("Refresh File", "به روزرسانی فایل"),
        ("Local", "محلی"),
        ("Remote", "از راه دور"),
        ("Remote Computer", "سیستم میزبان"),
        ("Local Computer", "سیستم راه دور"),
        ("Confirm Delete", "تایید حذف"),
        ("Delete", "حذف"),
        ("Properties", "مشخصات"),
        ("Multi Select", "انتخاب دسته ای"),
        ("Select All", "انتخاب همه"),
        ("Unselect All", "لغو انتخاب همه"),
        ("Empty Directory", "پوشه خالی"),
        ("Not an empty directory", "پوشه خالی نیست"),
        ("Are you sure you want to delete this file?", "از حذف این فایل مطمئن هستید؟"),
        ("Are you sure you want to delete this empty directory?", "از حذف این پوشه خالی مطمئن هستید؟"),
        ("Are you sure you want to delete the file of this directory?", "از حذف فایل موجود در این پوشه مطمئن هستید؟"),
        ("Do this for all conflicts", "این عمل برای همه ی تضادها انجام شود"),
        ("This is irreversible!", "این اقدام برگشت ناپذیر است!"),
        ("Deleting", "در حال حذف"),
        ("files", "فایل ها"),
        ("Waiting", "انتظار"),
        ("Finished", "تکمیل شد"),
        ("Speed", "سرعت"),
        ("Custom Image Quality", "سفارشی سازی کیفیت تصاویر"),
        ("Privacy mode", "حالت حریم خصوصی"),
        ("Block user input", "بلاک کردن ورودی کاربر"),
        ("Unblock user input", "آنبلاک کردن ورودی کاربر"),
        ("Adjust Window", "تنظیم پنجره"),
        ("Original", "اصل"),
        ("Shrink", "کوچک کردن"),
        ("Stretch", "کشیدن تصویر"),
        ("Scrollbar", "اسکرول بار"),
        ("ScrollAuto", "پیمایش/اسکرول خودکار"),
        ("Good image quality", "کیفیت خوب تصویر"),
        ("Balanced", "متعادل"),
        ("Optimize reaction time", "بهینه سازی زمان واکنش"),
        ("Custom", "سفارشی"),
        ("Show remote cursor", "نمایش مکان نما موس میزبان"),
        ("Show quality monitor", "نمایش کیفیت مانیتور"),
        ("Disable clipboard", " غیرفعالسازی کلیپبورد"),
        ("Lock after session end", "قفل کردن حساب کاربری سیستم عامل پس از پایان جلسه"),
        ("Insert", "افزودن"),
        ("Insert Lock", "افزودن قفل"),
        ("Refresh", "تازه سازی"),
        ("ID does not exist", "شناسه وجود ندارد"),
        ("Failed to connect to rendezvous server", "اتصال به سرور تولید شناسه انجام نشد"),
        ("Please try later", "لطفا بعدا تلاش کنید"),
        ("Remote desktop is offline", "دسکتاپ راه دور آفلاین است"),
        ("Key mismatch", "عدم تطابق کلید"),
        ("Timeout", "زمان انتظار به پایان رسید"),
        ("Failed to connect to relay server", "سرور وصل نشد Relay به"),
        ("Failed to connect via rendezvous server", "اتصال از طریق سرور تولید شناسه انجام نشد"),
        ("Failed to connect via relay server", "انجام نشد Relay اتصال از طریق سرور"),
        ("Failed to make direct connection to remote desktop", "اتصال مستقیم به دسکتاپ راه دور انجام نشد"),
        ("Set Password", "تنظیم رمزعبور"),
        ("OS Password", "رمز عیور سیستم عامل"),
        ("install_tip", "لطفا برنامه را نصب کنید UAC و جلوگیری از خطای RustDesk برای راحتی در استفاده از نرم افزار"),
        ("Click to upgrade", "برای ارتقا کلیک کنید"),
        ("Click to download", "برای دانلود کلیک کنید"),
        ("Click to update", "برای به روز رسانی کلیک کنید"),
        ("Configure", "تنظیم"),
        ("config_acc", "بدهید \"access\" مجوز RustDesk برای کنترل از راه دور دسکتاپ باید به"),
        ("config_screen", "بدهید \"screenshot\" مجوز RustDesk برای کنترل از راه دور دسکتاپ باید به"),
        ("Installing ...", "...در حال نصب"),
        ("Install", "نصب"),
        ("Installation", "نصب و راه اندازی"),
        ("Installation Path", "محل نصب"),
        ("Create start menu shortcuts", "Start ایجاد میانبرها در منوی"),
        ("Create desktop icon", "ایجاد آیکن در دسکتاپ"),
        ("agreement_tip", "با شروع نصب، شرایط توافق نامه مجوز را می پذیرید"),
        ("Accept and Install", "قبول و شروع نصب"),
        ("End-user license agreement", "قرارداد مجوز کاربر نهایی"),
        ("Generating ...", "...در حال تولید"),
        ("Your installation is lower version.", "نسخه قدیمی تری نصب شده است"),
        ("not_close_tcp_tip", "هنگام استفاده از تونل این پنجره را نبندید"),
        ("Listening ...", "...انتظار"),
        ("Remote Host", "هاست راه دور"),
        ("Remote Port", "پورت راه دور"),
        ("Action", "عملیات"),
        ("Add", "افزودن"),
        ("Local Port", "پورت محلی"),
        ("Local Address", "آدرس محلی"),
        ("Change Local Port", "تغییر پورت محلی"),
        ("setup_server_tip", "برای اتصال سریعتر، سرور اتصال ضخصی خود را راه اندازی کنید"),
        ("Too short, at least 6 characters.", "بسیار کوتاه حداقل 6 کاراکتر مورد نیاز است"),
        ("The confirmation is not identical.", "تأیید ناموفق بود."),
        ("Permissions", "دسترسی ها"),
        ("Accept", "پذیرفتن"),
        ("Dismiss", "رد کردن"),
        ("Disconnect", "قطع اتصال"),
        ("Allow using keyboard and mouse", "مجاز بودن استفاده از صفحه کلید و ماوس"),
        ("Allow using clipboard", "مجاز بودن استفاده از کلیپبورد"),
        ("Allow hearing sound", "مجاز بودن شنیدن صدا"),
        ("Allow file copy and paste", "مجاز بودن کپی و چسباندن فایل"),
        ("Connected", "متصل شده"),
        ("Direct and encrypted connection", "اتصال مستقیم و رمزگذاری شده"),
        ("Relayed and encrypted connection", "و رمزگذاری شده Relay اتصال از طریق"),
        ("Direct and unencrypted connection", "اتصال مستقیم و بدون رمزگذاری"),
        ("Relayed and unencrypted connection", "و رمزگذاری نشده Relay اتصال از طریق"),
        ("Enter Remote ID", "شناسه از راه دور را وارد کنید"),
        ("Enter your password", "زمر عبور خود را وارد کنید"),
        ("Logging in...", "...در حال ورود"),
        ("Enable RDP session sharing", "اشتراک گذاری جلسه RDP را فعال کنید"),
        ("Auto Login", "ورود خودکار"),
        ("Enable Direct IP Access", "را فعال کنید IP دسترسی مستقیم"),
        ("Rename", "تغییر نام"),
        ("Space", "فضا"),
        ("Create Desktop Shortcut", "ساخت میانبر روی دسکتاپ"),
        ("Change Path", "تغییر مسیر"),
        ("Create Folder", "ایجاد پوشه"),
        ("Please enter the folder name", "نام پوشه را وارد کنید"),
        ("Fix it", "بازسازی"),
        ("Warning", "هشدار"),
        ("Login screen using Wayland is not supported", "پشتیبانی نمی شود Wayland ورود به سیستم با استفاده از "),
        ("Reboot required", "راه اندازی مجدد مورد نیاز است"),
        ("Unsupported display server ", "سرور تصویر پشتیبانی نشده است"),
        ("x11 expected", ""),
        ("Port", "پورت"),
        ("Settings", "تنظیمات"),
        ("Username", "نام کاربری"),
        ("Invalid port", "پورت نامعتبر است"),
        ("Closed manually by the peer", "به صورت دستی توسط میزبان بسته شد"),
        ("Enable remote configuration modification", "فعال بودن اعمال تغییرات پیکربندی از راه دور"),
        ("Run without install", "بدون نصب اجرا شود"),
        ("Always connected via relay", "متصل است Relay همیشه با"),
        ("Always connect via relay", "برای اتصال استفاده شود Relay از"),
        ("whitelist_tip", "های مجاز می توانند به این دسکتاپ متصل شوند IP فقط"),
        ("Login", "ورود"),
        ("Verify", "تأیید کنید"),
        ("Remember me", "مرا به یاد داشته باش"),
        ("Trust this device", "به این دستگاه اعتماد کنید"),
        ("Verification code", "کد تایید"),
        ("verification_tip", "یک دستگاه جدید شناسایی شده است و یک کد تأیید به آدرس ایمیل ثبت شده ارسال شده است، برای ادامه ورود، کد تأیید را وارد کنید."),
        ("Logout", "خروج"),
        ("Tags", "برچسب ها"),
        ("Search ID", "جستجوی شناسه"),
        ("Current Wayland display server is not supported", "پشتیبانی نمی شود Wayland سرور نمایش فعلی"),
        ("whitelist_sep", "با کاما، نقطه ویرگول، فاصله یا خط جدید از هم جدا می شوند"),
        ("Add ID", "افزودن شناسه"),
        ("Add Tag", "افزودن برچسب"),
        ("Unselect all tags", "همه برچسب ها را لغو انتخاب کنید"),
        ("Network error", "خطای شبکه"),
        ("Username missed", "نام کاربری وجود ندارد"),
        ("Password missed", "رمزعبور وجود ندارد"),
        ("Wrong credentials", "اعتبارنامه نادرست است"),
        ("Edit Tag", "ویرایش برچسب"),
        ("Unremember Password", "رمز عبور ذخیره نشود"),
        ("Favorites", "اتصالات دلخواه"),
        ("Add to Favorites", "افزودن به علاقه مندی ها"),
        ("Remove from Favorites", "از علاقه مندی ها حذف شود"),
        ("Empty", "موردی وجود ندارد"),
        ("Invalid folder name", "نام پوشه نامعتبر است"),
        ("Socks5 Proxy", "Socks5 Proxy"),
        ("Hostname", "نام هاست"),
        ("Discovered", "پیدا شده"),
        ("install_daemon_tip", "برای شروع در هنگام راه اندازی، باید سرویس سیستم را نصب کنید"),
        ("Remote ID", "شناسه راه دور"),
        ("Paste", "درج"),
        ("Paste here?", "اینجا درج شود؟"),
        ("Are you sure to close the connection?", "آیا مطمئن هستید که می خواهید اتصال را پایان دهید؟"),
        ("Download new version", "دانلود نسخه جدید"),
        ("Touch mode", "حالت لمسی"),
        ("Mouse mode", "حالت ماوس"),
        ("One-Finger Tap", "با یک انگشت لمس کنید"),
        ("Left Mouse", "دکمه سمت چپ ماوس"),
        ("One-Long Tap", "لمس طولانی با یک انگشت"),
        ("Two-Finger Tap", "لمس دو انگشتی"),
        ("Right Mouse", "دکمه سمت راست ماوس"),
        ("One-Finger Move", "با یک انگشت حرکت کنید"),
        ("Double Tap & Move", "دو ضربه سریع بزنید و حرکت دهید"),
        ("Mouse Drag", "کشیدن ماوس"),
        ("Three-Finger vertically", "سه انگشت عمودی"),
        ("Mouse Wheel", "چرخ ماوس"),
        ("Two-Finger Move", "با دو انگشت حرکت کنید"),
        ("Canvas Move", ""),
        ("Pinch to Zoom", "با دو انگشت بکشید تا زوم شود"),
        ("Canvas Zoom", ""),
        ("Reset canvas", ""),
        ("No permission of file transfer", "مجوز انتقال فایل داده نشده"),
        ("Note", "یادداشت"),
        ("Connection", "ارتباط"),
        ("Share Screen", "اشتراک گذاری صفحه"),
        ("CLOSE", "بستن"),
        ("OPEN", "باز کردن"),
        ("Chat", "چت"),
        ("Total", "مجموع"),
        ("items", "آیتم ها"),
        ("Selected", "انتخاب شده"),
        ("Screen Capture", "ضبط صفحه"),
        ("Input Control", "کنترل ورودی"),
        ("Audio Capture", "ضبط صدا"),
        ("File Connection", "ارتباط فایل"),
        ("Screen Connection", "ارتباط صفحه"),
        ("Do you accept?", "آیا می پذیرید؟"),
        ("Open System Setting", "باز کردن تنظیمات سیستم"),
        ("How to get Android input permission?", "چگونه مجوز ورود به سیستم اندروید را دریافت کنیم؟"),
        ("android_input_permission_tip1", "برای اینکه یک دستگاه راه دور بتواند دستگاه Android شما را از طریق ماوس یا لمسی کنترل کند، باید به RustDesk اجازه دهید از ویژگی \"Accessibility\" استفاده کند."),
        ("android_input_permission_tip2", "به صفحه تنظیمات سیستم زیر بروید، \"Installed Services\" را پیدا کرده و وارد کنید، سرویس \"RustDesk Input\" را فعال کنید"),
        ("android_new_connection_tip", "درخواست جدیدی برای مدیریت دستگاه فعلی شما دریافت شده است."),
        ("android_service_will_start_tip", "فعال کردن ضبط صفحه به طور خودکار سرویس را راه اندازی می کند و به دستگاه های دیگر امکان می دهد درخواست اتصال به آن دستگاه را داشته باشند."),
        ("android_stop_service_tip", "با بستن سرویس، تمام اتصالات برقرار شده به طور خودکار بسته می شود"),
        ("android_version_audio_tip", "نسخه فعلی اندروید از ضبط صدا پشتیبانی نمی‌کند، لطفاً به اندروید 10 یا بالاتر به‌روزرسانی کنید"),
        ("android_start_service_tip", "برای شروع سرویس اشتراک‌گذاری صفحه، روی مجوز \"شروع مرحله‌بندی سرور\" یا OPEN \"Screen Capture\" کلیک کنید."),
        ("Account", "حساب کاربری"),
        ("Overwrite", "بازنویسی"),
        ("This file exists, skip or overwrite this file?", "این فایل وجود دارد، از فایل رد شود یا آن را بازنویسی کند؟"),
        ("Quit", "خروج"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "راهنما"),
        ("Failed", "ناموفق"),
        ("Succeeded", "موفقیت آمیز"),
        ("Someone turns on privacy mode, exit", "اگر شخصی حالت حریم خصوصی را روشن کرد، خارج شوید"),
        ("Unsupported", "پشتیبانی نشده"),
        ("Peer denied", "توسط میزبان راه دور رد شد"),
        ("Please install plugins", "لطفا افزونه ها را نصب کنید"),
        ("Peer exit", "میزبان خارج شد"),
        ("Failed to turn off", "خاموش کردن انجام نشد"),
        ("Turned off", "خاموش شد"),
        ("In privacy mode", "در حالت حریم خصوصی"),
        ("Out privacy mode", "خارج از حالت حریم خصوصی"),
        ("Language", "زبان"),
        ("Keep RustDesk background service", "را در پس زمینه نگه دارید RustDesk سرویس"),
        ("Ignore Battery Optimizations", "بهینه سازی باتری نادیده گرفته شود"),
        ("android_open_battery_optimizations_tip", "به صفحه تنظیمات بعدی بروید"),
        ("Connection not allowed", "اتصال مجاز نیست"),
        ("Legacy mode", "legacy حالت"),
        ("Map mode", "map حالت"),
        ("Translate mode", "حالت ترجمه"),
        ("Use permanent password", "از رمز عبور دائمی استفاده شود"),
        ("Use both passwords", "از هر دو رمز عبور استفاده شود"),
        ("Set permanent password", "یک رمز عبور دائمی تنظیم شود"),
        ("Enable Remote Restart", "فعال کردن قابلیت ریستارت از راه دور"),
        ("Allow remote restart", "مجاز بودن ریستارت از راه دور"),
        ("Restart Remote Device", "ریستارت کردن از راه دور"),
        ("Are you sure you want to restart", "ایا مطمئن هستید میخواهید راه اندازی مجدد انجام بدید؟"),
        ("Restarting Remote Device", "در حال راه اندازی مجدد دستگاه راه دور"),
        ("remote_restarting_tip", "دستگاه راه دور در حال راه اندازی مجدد است. این پیام را ببندید و پس از مدتی با استفاده از یک رمز عبور دائمی دوباره وصل شوید."),
        ("Copied", "کپی شده است"),
        ("Exit Fullscreen", "از حالت تمام صفحه خارج شوید"),
        ("Fullscreen", "تمام صفحه"),
        ("Mobile Actions", "اقدامات موبایل"),
        ("Select Monitor", "مانیتور را انتخاب کنید"),
        ("Control Actions", "اقدامات مدیریتی"),
        ("Display Settings", "تنظیمات نمایشگر"),
        ("Ratio", "نسبت"),
        ("Image Quality", "کیفیت تصویر"),
        ("Scroll Style", "سبک اسکرول"),
        ("Show Menubar", "نمایش نوار منو"),
        ("Hide Menubar", "پنهان کردن نوار منو"),
        ("Direct Connection", "ارتباط مستقیم"),
        ("Relay Connection", "Relay ارتباط"),
        ("Secure Connection", "ارتباط امن"),
        ("Insecure Connection", "ارتباط غیر امن"),
        ("Scale original", "مقیاس اصلی"),
        ("Scale adaptive", "مقیاس تطبیقی"),
        ("General", "عمومی"),
        ("Security", "امنیت"),
        ("Theme", "نمایه"),
        ("Dark Theme", "نمایه تیره"),
        ("Dark", "تیره"),
        ("Light", "روشن"),
        ("Follow System", "پیروی از سیستم"),
        ("Enable hardware codec", "فعال سازی کدک سخت افزاری"),
        ("Unlock Security Settings", "آنلاک شدن تنظیمات امنیتی"),
        ("Enable Audio", "فعال شدن صدا"),
        ("Unlock Network Settings", "آنلاک شدن تنظیمات شبکه"),
        ("Server", "سرور"),
        ("Direct IP Access", "IP دسترسی مستقیم به"),
        ("Proxy", "پروکسی"),
        ("Apply", "ثبت"),
        ("Disconnect all devices?", "همه دستگاه ها قطع شوند؟"),
        ("Clear", "پاک کردن"),
        ("Audio Input Device", "منبع صدا"),
        ("Deny remote access", "دسترسی از راه دور را رد کنید"),
        ("Use IP Whitelisting", "های مجاز IP استفاده از"),
        ("Network", "شبکه"),
        ("Enable RDP", "RDP فعال شدن"),
        ("Pin menubar", "پین کردن نوار منو"),
        ("Unpin menubar", "آنپین کردن نوار منو"),
        ("Recording", "در حال ضبط"),
        ("Directory", "مسیر"),
        ("Automatically record incoming sessions", "ضبط خودکار جلسات ورودی"),
        ("Change", "تغییر"),
        ("Start session recording", "شروع ضبط جلسه"),
        ("Stop session recording", "توقف ضبط جلسه"),
        ("Enable Recording Session", "فعالسازی ضبط جلسه"),
        ("Allow recording session", "مجومجاز بودن ضبط جلسه"),
        ("Enable LAN Discovery", "فعالسازی جستجو در شبکه"),
        ("Deny LAN Discovery", "غیر فعالسازی جستجو در شبکه"),
        ("Write a message", "یک پیام بنویسید"),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", ""),
        ("elevated_foreground_window_tip", ""),
        ("Disconnected", "قطع ارتباط"),
        ("Other", "سایر"),
        ("Confirm before closing multiple tabs", "تایید بستن دسته ای برگه ها"),
        ("Keyboard Settings", "تنظیمات صفحه کلید"),
        ("Full Access", "دسترسی کامل"),
        ("Screen Share", "اشتراک گذاری صفحه"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "نیازمند اوبونتو نسخه 21.04 یا بالاتر است Wayland"),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "استفاده کنید و یا سیستم عامل خود را تغییر دهید X11 نیازمند نسخه بالاتری از توزیع لینوکس است. لطفا از دسکتاپ با سیستم"),
        ("JumpLink", "چشم انداز"),
        ("Please Select the screen to be shared(Operate on the peer side).", "لطفاً صفحه‌ای را برای اشتراک‌گذاری انتخاب کنید (در سمت همتا به همتا کار کنید)."),
        ("Show RustDesk", "RustDesk نمایش"),
        ("This PC", "This PC"),
        ("or", "یا"),
        ("Continue with", "ادامه با"),
        ("Elevate", "افزایش سطح"),
        ("Zoom cursor", " بزرگنمایی نشانگر ماوس"),
        ("Accept sessions via password", "قبول درخواست با رمز عبور"),
        ("Accept sessions via click", "قبول درخواست با کلیک موس"),
        ("Accept sessions via both", "قبول درخواست با هر دو"),
        ("Please wait for the remote side to accept your session request...", "...لطفا صبر کنید تا میزبان درخواست شما را قبول کند"),
        ("One-time Password", "رمز عبور یکبار مصرف"),
        ("Use one-time password", "استفاده از رمز عبور یکبار مصرف"),
        ("One-time password length", "طول رمز عبور یکبار مصرف"),
        ("Request access to your device", "دسترسی به دستگاه خود را درخواست کنید"),
        ("Hide connection management window", "پنهان کردن پنجره مدیریت اتصال"),
        ("hide_cm_tip", "فقط در صورت پذیرفتن جلسات از طریق رمز عبور و استفاده از رمز عبور دائمی، مخفی شدن مجاز است"),
        ("wayland_experiment_tip", "پشتیبانی Wayland در مرحله آزمایشی است، لطفاً در صورت نیاز به دسترسی بدون مراقبت از X11 استفاده کنید."),
        ("Right click to select tabs", "برای انتخاب تب ها راست کلیک کنید"),
        ("Skipped", "رد شد"),
        ("Add to Address Book", "افزودن به دفترچه آدرس"),
        ("Group", "گروه"),
        ("Search", "جستجو"),
        ("Closed manually by web console", "به صورت دستی توسط کنسول وب بسته شد"),
        ("Local keyboard type", "نوع صفحه کلید محلی"),
        ("Select local keyboard type", "نوع صفحه کلید محلی را انتخاب کنید"),
        ("software_render_tip", "اگر کارت گرافیک Nvidia دارید و پنجره راه دور بلافاصله پس از اتصال بسته می شود، درایور nouveau را نصب نمایید و انتخاب گزینه استفاده از رندر نرم افزار می تواند کمک کننده باشد. راه اندازی مجدد نرم افزار مورد نیاز است."),
        ("Always use software rendering", "همیشه از رندر نرم افزاری استفاده کنید"),
        ("config_input", "برای کنترل دسکتاپ از راه دور با صفحه کلید، باید مجوز RustDesk \"Input Monitoring\" را بدهید."),
        ("request_elevation_tip", "همچنین می توانید در صورت وجود شخصی در سمت راه دور درخواست ارتفاع دهید."),
        ("Wait", "صبر کنید"),
        ("Elevation Error", "خطای ارتفاع"),
        ("Ask the remote user for authentication", "درخواست احراز هویت از یک کاربر راه دور"),
        ("Choose this if the remote account is administrator", "اگر حساب راه دور یک مدیر است، این را انتخاب کنید"),
        ("Transmit the username and password of administrator", "نام کاربری و رمز عبور مدیر را منتقل کنید"),
        ("still_click_uac_tip", "همچنان کاربر از راه دور نیاز دارد که روی OK در پنجره UAC اجرای RustDesk کلیک کند."),
        ("Request Elevation", "درخواست ارتفاع"),
        ("wait_accept_uac_tip", "لطفاً منتظر بمانید تا کاربر راه دور درخواست پنجره UAC را بپذیرد."),
        ("Elevate successfully", "با موفقیت بالا ببرید"),
        ("uppercase", "حروف بزرگ"),
        ("lowercase", "حروف کوچک"),
        ("digit", "عدد"),
        ("special character", "کاراکتر خاص"),
        ("length>=8", "حداقل طول 8 کاراکتر"),
        ("Weak", "ضعیف"),
        ("Medium", "متوسط"),
        ("Strong", "قوی"),
        ("Switch Sides", "طرفین را عوض کنید"),
        ("Please confirm if you want to share your desktop?", "لطفاً تأیید کنید که آیا می خواهید دسکتاپ خود را به اشتراک بگذارید؟"),
        ("Closed as expected", ""),
    ].iter().cloned().collect();
}
