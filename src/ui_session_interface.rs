use crate::client::io_loop::Remote;
use crate::client::{
    check_if_retry, get_key_state, handle_hash, handle_login_from_ui, handle_test_delay,
    input_os_password, load_config, send_mouse, start_video_audio_threads, FileManager, Key,
    LoginConfigHandler, QualityStatus, KEY_MAP,
};
use crate::common;

use crate::{client::Data, client::Interface};
use async_trait::async_trait;

use hbb_common::config::{Config, LocalConfig, PeerConfig};

use hbb_common::tokio::{self, sync::mpsc};

use hbb_common::{allow_err, message_proto::*};
use hbb_common::{fs, get_version_number, log, Stream};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};

#[derive(Clone, Default)]
pub struct Session<T: InvokeUi> {
    pub cmd: String,
    pub id: String,
    pub password: String,
    pub args: Vec<String>,
    pub lc: Arc<RwLock<LoginConfigHandler>>,
    pub sender: Arc<RwLock<Option<mpsc::UnboundedSender<Data>>>>,
    pub thread: Arc<Mutex<Option<std::thread::JoinHandle<()>>>>,
    pub ui_handler: T,
}

impl<T: InvokeUi> Session<T> {
    pub fn get_view_style(&self) -> String {
        self.lc.read().unwrap().view_style.clone()
    }

    pub fn get_image_quality(&self) -> String {
        self.lc.read().unwrap().image_quality.clone()
    }

    pub fn save_view_style(&mut self, value: String) {
        self.lc.write().unwrap().save_view_style(value);
    }

    pub fn toggle_option(&mut self, name: String) {
        let msg = self.lc.write().unwrap().toggle_option(name.clone());
        if name == "enable-file-transfer" {
            self.send(Data::ToggleClipboardFile);
        }
        if let Some(msg) = msg {
            self.send(Data::Message(msg));
        }
    }

    pub fn get_toggle_option(&self, name: String) -> bool {
        self.lc.read().unwrap().get_toggle_option(&name)
    }

    pub fn is_privacy_mode_supported(&self) -> bool {
        self.lc.read().unwrap().is_privacy_mode_supported()
    }

    pub fn refresh_video(&self) {
        self.send(Data::Message(LoginConfigHandler::refresh()));
    }

    pub fn save_custom_image_quality(&mut self, custom_image_quality: i32) {
        let msg = self
            .lc
            .write()
            .unwrap()
            .save_custom_image_quality(custom_image_quality);
        self.send(Data::Message(msg));
    }

    pub fn save_image_quality(&mut self, value: String) {
        let msg = self.lc.write().unwrap().save_image_quality(value);
        if let Some(msg) = msg {
            self.send(Data::Message(msg));
        }
    }

    pub fn get_remember(&self) -> bool {
        self.lc.read().unwrap().remember
    }

    pub fn set_write_override(
        &mut self,
        job_id: i32,
        file_num: i32,
        is_override: bool,
        remember: bool,
        is_upload: bool,
    ) -> bool {
        self.send(Data::SetConfirmOverrideFile((
            job_id,
            file_num,
            is_override,
            remember,
            is_upload,
        )));
        true
    }

    pub fn has_hwcodec(&self) -> bool {
        #[cfg(not(feature = "hwcodec"))]
        return false;
        #[cfg(feature = "hwcodec")]
        return true;
    }

    pub fn change_prefer_codec(&self) {
        let msg = self.lc.write().unwrap().change_prefer_codec();
        self.send(Data::Message(msg));
    }

    pub fn restart_remote_device(&self) {
        let mut lc = self.lc.write().unwrap();
        lc.restarting_remote_device = true;
        let msg = lc.restart_remote_device();
        self.send(Data::Message(msg));
    }

    pub fn t(&self, name: String) -> String {
        crate::client::translate(name)
    }

    pub fn get_audit_server(&self) -> String {
        if self.lc.read().unwrap().conn_id <= 0
            || LocalConfig::get_option("access_token").is_empty()
        {
            return "".to_owned();
        }
        crate::get_audit_server(
            Config::get_option("api-server"),
            Config::get_option("custom-rendezvous-server"),
        )
    }

    pub fn send_note(&self, note: String) {
        let url = self.get_audit_server();
        let id = self.id.clone();
        let conn_id = self.lc.read().unwrap().conn_id;
        std::thread::spawn(move || {
            send_note(url, id, conn_id, note);
        });
    }

    pub fn is_xfce(&self) -> bool {
        crate::platform::is_xfce()
    }

    pub fn remove_port_forward(&self, port: i32) {
        let mut config = self.load_config();
        config.port_forwards = config
            .port_forwards
            .drain(..)
            .filter(|x| x.0 != port)
            .collect();
        self.save_config(config);
        self.send(Data::RemovePortForward(port));
    }

    pub fn add_port_forward(&mut self, port: i32, remote_host: String, remote_port: i32) {
        let mut config = self.load_config();
        if config
            .port_forwards
            .iter()
            .filter(|x| x.0 == port)
            .next()
            .is_some()
        {
            return;
        }
        let pf = (port, remote_host, remote_port);
        config.port_forwards.push(pf.clone());
        self.save_config(config);
        self.send(Data::AddPortForward(pf));
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_option(&self, k: String) -> String {
        if k.eq("remote_dir") {
            return self.lc.read().unwrap().get_remote_dir();
        }
        self.lc.read().unwrap().get_option(&k)
    }

    pub fn set_option(&self, k: String, mut v: String) {
        let mut lc = self.lc.write().unwrap();
        if k.eq("remote_dir") {
            v = lc.get_all_remote_dir(v);
        }
        lc.set_option(k, v);
    }

    #[inline]
    pub fn load_config(&self) -> PeerConfig {
        load_config(&self.id)
    }

    #[inline]
    pub(super) fn save_config(&self, config: PeerConfig) {
        self.lc.write().unwrap().save_config(config);
    }

    pub fn is_restarting_remote_device(&self) -> bool {
        self.lc.read().unwrap().restarting_remote_device
    }

    #[inline]
    pub fn peer_platform(&self) -> String {
        self.lc.read().unwrap().info.platform.clone()
    }

    pub fn ctrl_alt_del(&mut self) {
        if self.peer_platform() == "Windows" {
            let mut key_event = KeyEvent::new();
            key_event.set_control_key(ControlKey::CtrlAltDel);
            self.key_down_or_up(1, key_event, false, false, false, false);
        } else {
            let mut key_event = KeyEvent::new();
            key_event.set_control_key(ControlKey::Delete);
            self.key_down_or_up(3, key_event, true, true, false, false);
        }
    }

    pub fn key_down_or_up(
        &self,
        down_or_up: i32,
        evt: KeyEvent,
        alt: bool,
        ctrl: bool,
        shift: bool,
        command: bool,
    ) {
        let mut key_event = evt;

        if alt
            && !crate::is_control_key(&key_event, &ControlKey::Alt)
            && !crate::is_control_key(&key_event, &ControlKey::RAlt)
        {
            key_event.modifiers.push(ControlKey::Alt.into());
        }
        if shift
            && !crate::is_control_key(&key_event, &ControlKey::Shift)
            && !crate::is_control_key(&key_event, &ControlKey::RShift)
        {
            key_event.modifiers.push(ControlKey::Shift.into());
        }
        if ctrl
            && !crate::is_control_key(&key_event, &ControlKey::Control)
            && !crate::is_control_key(&key_event, &ControlKey::RControl)
        {
            key_event.modifiers.push(ControlKey::Control.into());
        }
        if command
            && !crate::is_control_key(&key_event, &ControlKey::Meta)
            && !crate::is_control_key(&key_event, &ControlKey::RWin)
        {
            key_event.modifiers.push(ControlKey::Meta.into());
        }
        if get_key_state(enigo::Key::CapsLock) {
            key_event.modifiers.push(ControlKey::CapsLock.into());
        }
        if self.peer_platform() != "Mac OS" {
            if get_key_state(enigo::Key::NumLock) && common::valid_for_numlock(&key_event) {
                key_event.modifiers.push(ControlKey::NumLock.into());
            }
        }
        if down_or_up == 1 {
            key_event.down = true;
        } else if down_or_up == 3 {
            key_event.press = true;
        }
        let mut msg_out = Message::new();
        msg_out.set_key_event(key_event);
        log::debug!("{:?}", msg_out);
        self.send(Data::Message(msg_out));
    }

    pub fn get_platform(&self, is_remote: bool) -> String {
        if is_remote {
            self.peer_platform()
        } else {
            whoami::platform().to_string()
        }
    }

    pub fn get_path_sep(&self, is_remote: bool) -> &'static str {
        let p = self.get_platform(is_remote);
        if &p == "Windows" {
            return "\\";
        } else {
            return "/";
        }
    }

    pub fn input_os_password(&self, pass: String, activate: bool) {
        input_os_password(pass, activate, self.clone());
    }

    pub fn get_chatbox(&self) -> String {
        #[cfg(feature = "inline")]
        return super::inline::get_chatbox();
        #[cfg(not(feature = "inline"))]
        return "".to_owned();
    }

    pub fn get_icon(&self) -> String {
        crate::get_icon()
    }

    pub fn send_chat(&self, text: String) {
        let mut misc = Misc::new();
        misc.set_chat_message(ChatMessage {
            text,
            ..Default::default()
        });
        let mut msg_out = Message::new();
        msg_out.set_misc(misc);
        self.send(Data::Message(msg_out));
    }

    pub fn switch_display(&self, display: i32) {
        let mut misc = Misc::new();
        misc.set_switch_display(SwitchDisplay {
            display,
            ..Default::default()
        });
        let mut msg_out = Message::new();
        msg_out.set_misc(misc);
        self.send(Data::Message(msg_out));
    }

    pub fn lock_screen(&mut self) {
        let mut key_event = KeyEvent::new();
        key_event.set_control_key(ControlKey::LockScreen);
        self.key_down_or_up(1, key_event, false, false, false, false);
    }

    // flutter only TODO new input
    pub fn input_key(
        &self,
        name: &str,
        down: bool,
        press: bool,
        alt: bool,
        ctrl: bool,
        shift: bool,
        command: bool,
    ) {
        let chars: Vec<char> = name.chars().collect();
        if chars.len() == 1 {
            let key = Key::_Raw(chars[0] as _);
            self._input_key(key, down, press, alt, ctrl, shift, command);
        } else {
            if let Some(key) = KEY_MAP.get(name) {
                self._input_key(key.clone(), down, press, alt, ctrl, shift, command);
            }
        }
    }

    // flutter only TODO new input
    pub fn input_string(&self, value: &str) {
        let mut key_event = KeyEvent::new();
        key_event.set_seq(value.to_owned());
        let mut msg_out = Message::new();
        msg_out.set_key_event(key_event);
        self.send(Data::Message(msg_out));
    }

    // flutter only TODO new input
    fn _input_key(
        &self,
        key: Key,
        down: bool,
        press: bool,
        alt: bool,
        ctrl: bool,
        shift: bool,
        command: bool,
    ) {
        let v = if press {
            3
        } else if down {
            1
        } else {
            0
        };
        let mut key_event = KeyEvent::new();
        match key {
            Key::Chr(chr) => {
                key_event.set_chr(chr);
            }
            Key::ControlKey(key) => {
                key_event.set_control_key(key.clone());
            }
            Key::_Raw(raw) => {
                if raw > 'z' as u32 || raw < 'a' as u32 {
                    key_event.set_unicode(raw);
                    // TODO
                    // if down_or_up == 0 {
                    //     // ignore up, avoiding trigger twice
                    //     return;
                    // }
                    // down_or_up = 1; // if press, turn into down for avoiding trigger twice on server side
                } else {
                    // to make ctrl+c works on windows
                    key_event.set_chr(raw);
                }
            }
        }

        self.key_down_or_up(v, key_event, alt, ctrl, shift, command);
    }

    pub fn send_mouse(
        &mut self,
        mask: i32,
        x: i32,
        y: i32,
        alt: bool,
        ctrl: bool,
        shift: bool,
        command: bool,
    ) {
        #[allow(unused_mut)]
        let mut command = command;
        #[cfg(windows)]
        {
            if !command && crate::platform::windows::get_win_key_state() {
                command = true;
            }
        }

        send_mouse(mask, x, y, alt, ctrl, shift, command, self);
        // on macos, ctrl + left button down = right button down, up won't emit, so we need to
        // emit up myself if peer is not macos
        // to-do: how about ctrl + left from win to macos
        if cfg!(target_os = "macos") {
            let buttons = mask >> 3;
            let evt_type = mask & 0x7;
            if buttons == 1 && evt_type == 1 && ctrl && self.peer_platform() != "Mac OS" {
                self.send_mouse((1 << 3 | 2) as _, x, y, alt, ctrl, shift, command);
            }
        }
    }

    pub fn reconnect(&self) {
        self.send(Data::Close);
        let cloned = self.clone();
        let mut lock = self.thread.lock().unwrap();
        lock.take().map(|t| t.join());
        *lock = Some(std::thread::spawn(move || {
            io_loop(cloned);
        }));
    }

    pub fn get_icon_path(&self, file_type: i32, ext: String) -> String {
        let mut path = Config::icon_path();
        if file_type == FileType::DirLink as i32 {
            let new_path = path.join("dir_link");
            if !std::fs::metadata(&new_path).is_ok() {
                #[cfg(windows)]
                allow_err!(std::os::windows::fs::symlink_file(&path, &new_path));
                #[cfg(not(windows))]
                allow_err!(std::os::unix::fs::symlink(&path, &new_path));
            }
            path = new_path;
        } else if file_type == FileType::File as i32 {
            if !ext.is_empty() {
                path = path.join(format!("file.{}", ext));
            } else {
                path = path.join("file");
            }
            if !std::fs::metadata(&path).is_ok() {
                allow_err!(std::fs::File::create(&path));
            }
        } else if file_type == FileType::FileLink as i32 {
            let new_path = path.join("file_link");
            if !std::fs::metadata(&new_path).is_ok() {
                path = path.join("file");
                if !std::fs::metadata(&path).is_ok() {
                    allow_err!(std::fs::File::create(&path));
                }
                #[cfg(windows)]
                allow_err!(std::os::windows::fs::symlink_file(&path, &new_path));
                #[cfg(not(windows))]
                allow_err!(std::os::unix::fs::symlink(&path, &new_path));
            }
            path = new_path;
        } else if file_type == FileType::DirDrive as i32 {
            if cfg!(windows) {
                path = fs::get_path("C:");
            } else if cfg!(target_os = "macos") {
                if let Ok(entries) = fs::get_path("/Volumes/").read_dir() {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            path = entry.path();
                            break;
                        }
                    }
                }
            }
        }
        fs::get_string(&path)
    }

    pub fn login(&self, password: String, remember: bool) {
        self.send(Data::Login((password, remember)));
    }

    pub fn new_rdp(&self) {
        self.send(Data::NewRDP);
    }

    pub fn close(&self) {
        self.send(Data::Close);
    }
}

pub trait InvokeUi: Send + Sync + Clone + 'static + Sized + Default {
    fn set_cursor_data(&self, cd: CursorData);
    fn set_cursor_id(&self, id: String);
    fn set_cursor_position(&self, cp: CursorPosition);
    fn set_display(&self, x: i32, y: i32, w: i32, h: i32);
    fn switch_display(&self, display: &SwitchDisplay);
    fn set_peer_info(
        &self,
        username: &str,
        hostname: &str,
        platform: &str,
        sas_enabled: bool,
        displays: &Vec<HashMap<&str, i32>>,
        version: &str,
        current_display: usize,
        is_file_transfer: bool,
    ); // flutter
    fn update_privacy_mode(&self);
    fn set_permission(&self, name: &str, value: bool);
    fn update_pi(&self, pi: PeerInfo);
    fn close_success(&self);
    fn update_quality_status(&self, qs: QualityStatus);
    fn set_connection_type(&self, is_secured: bool, direct: bool);
    fn job_error(&self, id: i32, err: String, file_num: i32);
    fn job_done(&self, id: i32, file_num: i32);
    fn clear_all_jobs(&self);
    fn add_job(
        &self,
        id: i32,
        path: String,
        to: String,
        file_num: i32,
        show_hidden: bool,
        is_remote: bool,
    );
    fn new_message(&self, msg: String);
    fn update_transfer_list(&self);
    // fn update_folder_files(&self); // TODO flutter with file_dir and update_folder_files
    fn confirm_delete_files(&self, id: i32, i: i32, name: String);
    fn override_file_confirm(&self, id: i32, file_num: i32, to: String, is_upload: bool);
    fn update_block_input_state(&self, on: bool);
    fn job_progress(&self, id: i32, file_num: i32, speed: f64, finished_size: f64);
    fn adapt_size(&self);
    fn on_rgba(&self, data: &[u8]);
    fn msgbox(&self, msgtype: &str, title: &str, text: &str, retry: bool);
    #[cfg(any(target_os = "android", target_os = "ios"))]
    fn clipboard(&self, content: String);
}

impl<T: InvokeUi> Deref for Session<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.ui_handler
    }
}

impl<T: InvokeUi> DerefMut for Session<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ui_handler
    }
}

impl<T: InvokeUi> FileManager for Session<T> {}

#[async_trait]
impl<T: InvokeUi> Interface for Session<T> {
    fn send(&self, data: Data) {
        if let Some(sender) = self.sender.read().unwrap().as_ref() {
            sender.send(data).ok();
        }
    }

    // TODO flutter
    fn is_file_transfer(&self) -> bool {
        self.cmd == "--file-transfer"
    }

    // TODO flutter
    fn is_port_forward(&self) -> bool {
        self.cmd == "--port-forward" || self.is_rdp()
    }

    // TODO flutter
    fn is_rdp(&self) -> bool {
        self.cmd == "--rdp"
    }

    fn msgbox(&self, msgtype: &str, title: &str, text: &str) {
        let retry = check_if_retry(msgtype, title, text);
        self.ui_handler.msgbox(msgtype, title, text, retry);
    }

    fn handle_login_error(&mut self, err: &str) -> bool {
        self.lc.write().unwrap().handle_login_error(err, self)
    }

    fn handle_peer_info(&mut self, pi: PeerInfo) {
        let mut lc = self.lc.write().unwrap();

        // let mut pi_sciter = Value::map();
        let username = lc.get_username(&pi);

        // flutter
        let mut displays = Vec::new();
        let mut current_index = pi.current_display as usize;

        // pi_sciter.set_item("username", username.clone());
        // pi_sciter.set_item("hostname", pi.hostname.clone());
        // pi_sciter.set_item("platform", pi.platform.clone());
        // pi_sciter.set_item("sas_enabled", pi.sas_enabled);
        if get_version_number(&pi.version) < get_version_number("1.1.10") {
            self.set_permission("restart", false);
        }
        if self.is_file_transfer() {
            if pi.username.is_empty() {
                self.on_error("No active console user logged on, please connect and logon first.");
                return;
            }
        } else if !self.is_port_forward() {
            if pi.displays.is_empty() {
                lc.handle_peer_info(username, pi);
                self.update_privacy_mode();
                self.msgbox("error", "Remote Error", "No Display");
                return;
            }
            // let mut displays = Value::array(0);
            // for ref d in pi.displays.iter() {
            //     let mut display = Value::map();
            //     display.set_item("x", d.x);
            //     display.set_item("y", d.y);
            //     display.set_item("width", d.width);
            //     display.set_item("height", d.height);
            //     displays.push(display);
            // }
            // pi_sciter.set_item("displays", displays);

            // flutter
            for ref d in pi.displays.iter() {
                let mut h: HashMap<&str, i32> = Default::default();
                h.insert("x", d.x);
                h.insert("y", d.y);
                h.insert("width", d.width);
                h.insert("height", d.height);
                displays.push(h);
            }
            if current_index >= pi.displays.len() {
                current_index = 0;
            }

            if current_index >= pi.displays.len() {
                current_index = 0;
            }
            // pi_sciter.set_item("current_display", current as i32);
            let current = &pi.displays[current_index];
            self.set_display(current.x, current.y, current.width, current.height);

            self.set_peer_info(
                &username,
                &pi.hostname,
                &pi.platform,
                pi.sas_enabled,
                &displays,
                &pi.version,
                current_index,
                lc.is_file_transfer,
            );

            // https://sciter.com/forums/topic/color_spaceiyuv-crash
            // Nothing spectacular in decoder – done on CPU side.
            // So if you can do BGRA translation on your side – the better.
            // BGRA is used as internal image format so it will not require additional transformations.
            // VIDEO.lock().unwrap().as_mut().map(|v| {
            //     let ok = v.start_streaming(
            //         (current.width as _, current.height as _),
            //         COLOR_SPACE::Rgb32,
            //         None,
            //     );
            //     log::info!("[video] initialized: {:?}", ok);
            // });
            let p = lc.should_auto_login();
            if !p.is_empty() {
                input_os_password(p, true, self.clone());
            }
        }
        lc.handle_peer_info(username, pi);
        self.update_privacy_mode();
        // self.update_pi(pi);
        if self.is_file_transfer() {
            self.close_success();
        } else if !self.is_port_forward() {
            self.msgbox("success", "Successful", "Connected, waiting for image...");
        }
        #[cfg(windows)]
        {
            let mut path = std::env::temp_dir();
            path.push(&self.id);
            let path = path.with_extension(crate::get_app_name().to_lowercase());
            std::fs::File::create(&path).ok();
            if let Some(path) = path.to_str() {
                crate::platform::windows::add_recent_document(&path);
            }
        }
        // self.start_keyboard_hook(); // TODO
    }

    async fn handle_hash(&mut self, pass: &str, hash: Hash, peer: &mut Stream) {
        handle_hash(self.lc.clone(), pass, hash, self, peer).await;
    }

    async fn handle_login_from_ui(&mut self, password: String, remember: bool, peer: &mut Stream) {
        handle_login_from_ui(self.lc.clone(), password, remember, peer).await;
    }

    async fn handle_test_delay(&mut self, t: TestDelay, peer: &mut Stream) {
        if !t.from_client {
            self.update_quality_status(QualityStatus {
                delay: Some(t.last_delay as _),
                target_bitrate: Some(t.target_bitrate as _),
                ..Default::default()
            });
            handle_test_delay(t, peer).await;
        }
    }

    fn set_force_relay(&mut self, direct: bool, received: bool) {
        let mut lc = self.lc.write().unwrap();
        lc.force_relay = false;
        if direct && !received {
            let errno = errno::errno().0;
            log::info!("errno is {}", errno);
            // TODO: check mac and ios
            if cfg!(windows) && errno == 10054 || !cfg!(windows) && errno == 104 {
                lc.force_relay = true;
                lc.set_option("force-always-relay".to_owned(), "Y".to_owned());
            }
        }
    }

    fn is_force_relay(&self) -> bool {
        self.lc.read().unwrap().force_relay
    }
}

#[tokio::main(flavor = "current_thread")]
pub async fn io_loop<T: InvokeUi>(handler: Session<T>) {
    let (sender, mut receiver) = mpsc::unbounded_channel::<Data>();
    *handler.sender.write().unwrap() = Some(sender.clone());
    let mut options = crate::ipc::get_options_async().await;
    let mut key = options.remove("key").unwrap_or("".to_owned());
    let token = LocalConfig::get_option("access_token");
    if key.is_empty() {
        key = crate::platform::get_license_key();
    }
    if handler.is_port_forward() {
        if handler.is_rdp() {
            let port = handler
                .get_option("rdp_port".to_owned())
                .parse::<i32>()
                .unwrap_or(3389);
            std::env::set_var(
                "rdp_username",
                handler.get_option("rdp_username".to_owned()),
            );
            std::env::set_var(
                "rdp_password",
                handler.get_option("rdp_password".to_owned()),
            );
            log::info!("Remote rdp port: {}", port);
            start_one_port_forward(handler, 0, "".to_owned(), port, receiver, &key, &token).await;
        } else if handler.args.len() == 0 {
            let pfs = handler.lc.read().unwrap().port_forwards.clone();
            let mut queues = HashMap::<i32, mpsc::UnboundedSender<Data>>::new();
            for d in pfs {
                sender.send(Data::AddPortForward(d)).ok();
            }
            loop {
                match receiver.recv().await {
                    Some(Data::AddPortForward((port, remote_host, remote_port))) => {
                        if port <= 0 || remote_port <= 0 {
                            continue;
                        }
                        let (sender, receiver) = mpsc::unbounded_channel::<Data>();
                        queues.insert(port, sender);
                        let handler = handler.clone();
                        let key = key.clone();
                        let token = token.clone();
                        tokio::spawn(async move {
                            start_one_port_forward(
                                handler,
                                port,
                                remote_host,
                                remote_port,
                                receiver,
                                &key,
                                &token,
                            )
                            .await;
                        });
                    }
                    Some(Data::RemovePortForward(port)) => {
                        if let Some(s) = queues.remove(&port) {
                            s.send(Data::Close).ok();
                        }
                    }
                    Some(Data::Close) => {
                        break;
                    }
                    Some(d) => {
                        for (_, s) in queues.iter() {
                            s.send(d.clone()).ok();
                        }
                    }
                    _ => {}
                }
            }
        } else {
            let port = handler.args[0].parse::<i32>().unwrap_or(0);
            if handler.args.len() != 3
                || handler.args[2].parse::<i32>().unwrap_or(0) <= 0
                || port <= 0
            {
                handler.on_error("Invalid arguments, usage:<br><br> rustdesk --port-forward remote-id listen-port remote-host remote-port");
            }
            let remote_host = handler.args[1].clone();
            let remote_port = handler.args[2].parse::<i32>().unwrap_or(0);
            start_one_port_forward(
                handler,
                port,
                remote_host,
                remote_port,
                receiver,
                &key,
                &token,
            )
            .await;
        }
        return;
    }
    let frame_count = Arc::new(AtomicUsize::new(0));
    let frame_count_cl = frame_count.clone();
    let ui_handler = handler.ui_handler.clone();
    let (video_sender, audio_sender) = start_video_audio_threads(move |data: &[u8]| {
        frame_count_cl.fetch_add(1, Ordering::Relaxed);
        ui_handler.on_rgba(data);
    });

    let mut remote = Remote::new(
        handler,
        video_sender,
        audio_sender,
        receiver,
        sender,
        frame_count,
    );
    remote.io_loop(&key, &token).await;
    remote.sync_jobs_status_to_local().await;
}

async fn start_one_port_forward<T: InvokeUi>(
    handler: Session<T>,
    port: i32,
    remote_host: String,
    remote_port: i32,
    receiver: mpsc::UnboundedReceiver<Data>,
    key: &str,
    token: &str,
) {
    if let Err(err) = crate::port_forward::listen(
        handler.id.clone(),
        handler.password.clone(),
        port,
        handler.clone(),
        receiver,
        key,
        token,
        handler.lc.clone(),
        remote_host,
        remote_port,
    )
    .await
    {
        handler.on_error(&format!("Failed to listen on {}: {}", port, err));
    }
    log::info!("port forward (:{}) exit", port);
}

#[tokio::main(flavor = "current_thread")]
async fn send_note(url: String, id: String, conn_id: i32, note: String) {
    let body = serde_json::json!({ "id": id, "Id": conn_id, "note": note });
    allow_err!(crate::post_request(url, body.to_string(), "").await);
}
