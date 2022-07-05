// 24FPS (actually 23.976FPS) is what video professionals ages ago determined to be the
// slowest playback rate that still looks smooth enough to feel real.
// Our eyes can see a slight difference and even though 30FPS actually shows
// more information and is more realistic.
// 60FPS is commonly used in game, teamviewer 12 support this for video editing user.

// how to capture with mouse cursor:
// https://docs.microsoft.com/zh-cn/windows/win32/direct3ddxgi/desktop-dup-api?redirectedfrom=MSDN

// RECORD: The following Project has implemented audio capture, hardware codec and mouse cursor drawn.
// https://github.com/PHZ76/DesktopSharing

// dxgi memory leak issue
// https://stackoverflow.com/questions/47801238/memory-leak-in-creating-direct2d-device
// but per my test, it is more related to AcquireNextFrame,
// https://forums.developer.nvidia.com/t/dxgi-outputduplication-memory-leak-when-using-nv-but-not-amd-drivers/108582

// to-do:
// https://slhck.info/video/2017/03/01/rate-control.html

use super::*;
use hbb_common::tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    Mutex as TokioMutex,
};
use scrap::{
    codec::{Encoder, EncoderCfg, HwEncoderConfig},
    vpxcodec::{VpxEncoderConfig, VpxVideoCodecId},
    Capturer, Display, Frame,
};
use std::{
    collections::HashSet,
    io::{ErrorKind::WouldBlock, Result},
    time::{self, Duration, Instant},
};
#[cfg(windows)]
use virtual_display;

pub const NAME: &'static str = "video";
const FPS: u8 = 30;

lazy_static::lazy_static! {
    static ref CURRENT_DISPLAY: Arc<Mutex<usize>> = Arc::new(Mutex::new(usize::MAX));
    static ref LAST_ACTIVE: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
    static ref SWITCH: Arc<Mutex<bool>> = Default::default();
    static ref FRAME_FETCHED_NOTIFIER: (UnboundedSender<(i32, Option<Instant>)>, Arc<TokioMutex<UnboundedReceiver<(i32, Option<Instant>)>>>) = {
        let (tx, rx) = unbounded_channel();
        (tx, Arc::new(TokioMutex::new(rx)))
    };
    static ref PRIVACY_MODE_CONN_ID: Mutex<i32> = Mutex::new(0);
    static ref IS_CAPTURER_MAGNIFIER_SUPPORTED: bool = is_capturer_mag_supported();
    pub static ref VIDEO_QOS: Arc<Mutex<VideoQoS>> = Default::default();
}

trait Percent {
    fn as_percent(&self) -> u32;
}

impl Percent for ImageQuality {
    fn as_percent(&self) -> u32 {
        match self {
            ImageQuality::NotSet => 0,
            ImageQuality::Low => 50,
            ImageQuality::Balanced => 66,
            ImageQuality::Best => 100,
        }
    }
}

pub struct VideoQoS {
    width: u32,
    height: u32,
    user_image_quality: u32,
    current_image_quality: u32,
    enable_abr: bool,
    pub current_delay: u32,
    pub fps: u8,             // abr
    pub target_bitrate: u32, // abr
    updated: bool,
    state: DelayState,
    debounce_count: u32,
}

#[derive(PartialEq, Debug)]
enum DelayState {
    Normal = 0,
    LowDelay = 200,
    HighDelay = 500,
    Broken = 1000,
}

impl DelayState {
    fn from_delay(delay: u32) -> Self {
        if delay > DelayState::Broken as u32 {
            DelayState::Broken
        } else if delay > DelayState::HighDelay as u32 {
            DelayState::HighDelay
        } else if delay > DelayState::LowDelay as u32 {
            DelayState::LowDelay
        } else {
            DelayState::Normal
        }
    }
}

impl Default for VideoQoS {
    fn default() -> Self {
        VideoQoS {
            fps: FPS,
            user_image_quality: ImageQuality::Balanced.as_percent(),
            current_image_quality: ImageQuality::Balanced.as_percent(),
            enable_abr: false,
            width: 0,
            height: 0,
            current_delay: 0,
            target_bitrate: 0,
            updated: false,
            state: DelayState::Normal,
            debounce_count: 0,
        }
    }
}

impl VideoQoS {
    pub fn set_size(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.width = width;
        self.height = height;
    }

    pub fn spf(&mut self) -> Duration {
        if self.fps <= 0 {
            self.fps = FPS;
        }
        time::Duration::from_secs_f32(1. / (self.fps as f32))
    }

    // update_network_delay periodically
    // decrease the bitrate when the delay gets bigger
    pub fn update_network_delay(&mut self, delay: u32) {
        if self.current_delay.eq(&0) {
            self.current_delay = delay;
            return;
        }

        self.current_delay = delay / 2 + self.current_delay / 2;
        log::trace!(
            "VideoQoS update_network_delay:{}, {}, state:{:?}",
            self.current_delay,
            delay,
            self.state,
        );

        // ABR
        if !self.enable_abr {
            return;
        }
        let current_state = DelayState::from_delay(self.current_delay);
        if current_state != self.state && self.debounce_count > 5 {
            log::debug!(
                "VideoQoS state changed:{:?} -> {:?}",
                self.state,
                current_state
            );
            self.state = current_state;
            self.debounce_count = 0;
            self.refresh_quality();
        } else {
            self.debounce_count += 1;
        }
    }

    fn refresh_quality(&mut self) {
        match self.state {
            DelayState::Normal => {
                self.fps = FPS;
                self.current_image_quality = self.user_image_quality;
            }
            DelayState::LowDelay => {
                self.fps = FPS;
                self.current_image_quality = std::cmp::min(self.user_image_quality, 50);
            }
            DelayState::HighDelay => {
                self.fps = FPS / 2;
                self.current_image_quality = std::cmp::min(self.user_image_quality, 25);
            }
            DelayState::Broken => {
                self.fps = FPS / 4;
                self.current_image_quality = 10;
            }
        }
        let _ = self.generate_bitrate().ok();
        self.updated = true;
    }

    // handle image_quality change from peer
    pub fn update_image_quality(&mut self, image_quality: i32) {
        let image_quality = Self::convert_quality(image_quality) as _;
        log::debug!("VideoQoS update_image_quality: {}", image_quality);
        if self.current_image_quality != image_quality {
            self.current_image_quality = image_quality;
            let _ = self.generate_bitrate().ok();
            self.updated = true;
        }

        self.user_image_quality = self.current_image_quality;
    }

    pub fn generate_bitrate(&mut self) -> ResultType<u32> {
        // https://www.nvidia.com/en-us/geforce/guides/broadcasting-guide/
        if self.width == 0 || self.height == 0 {
            bail!("Fail to generate_bitrate, width or height is not set");
        }
        if self.current_image_quality == 0 {
            self.current_image_quality = ImageQuality::Balanced.as_percent();
        }

        let base_bitrate = ((self.width * self.height) / 800) as u32;

        #[cfg(target_os = "android")]
        {
            // fix when andorid screen shrinks
            let fix = Display::fix_quality() as u32;
            log::debug!("Android screen, fix quality:{}", fix);
            let base_bitrate = base_bitrate * fix;
            self.target_bitrate = base_bitrate * self.current_image_quality / 100;
            Ok(self.target_bitrate)
        }
        #[cfg(not(target_os = "android"))]
        {
            self.target_bitrate = base_bitrate * self.current_image_quality / 100;
            Ok(self.target_bitrate)
        }
    }

    pub fn check_if_updated(&mut self) -> bool {
        if self.updated {
            self.updated = false;
            return true;
        }
        return false;
    }

    pub fn reset(&mut self) {
        *self = Default::default();
    }

    fn check_abr_config(&mut self) -> bool {
        self.enable_abr = if let Some(v) = Config2::get().options.get("enable-abr") {
            v != "N"
        } else {
            true // default is true
        };
        self.enable_abr
    }

    pub fn convert_quality(q: i32) -> i32 {
        if q == ImageQuality::Balanced.value() {
            100 * 2 / 3
        } else if q == ImageQuality::Low.value() {
            100 / 2
        } else if q == ImageQuality::Best.value() {
            100
        } else {
            (q >> 8 & 0xFF) * 2
        }
    }
}

fn is_capturer_mag_supported() -> bool {
    #[cfg(windows)]
    return scrap::CapturerMag::is_supported();
    #[cfg(not(windows))]
    false
}

pub fn notify_video_frame_feched(conn_id: i32, frame_tm: Option<Instant>) {
    FRAME_FETCHED_NOTIFIER.0.send((conn_id, frame_tm)).unwrap()
}

pub fn set_privacy_mode_conn_id(conn_id: i32) {
    *PRIVACY_MODE_CONN_ID.lock().unwrap() = conn_id
}

pub fn get_privacy_mode_conn_id() -> i32 {
    *PRIVACY_MODE_CONN_ID.lock().unwrap()
}

pub fn is_privacy_mode_supported() -> bool {
    #[cfg(windows)]
    return *IS_CAPTURER_MAGNIFIER_SUPPORTED;
    #[cfg(not(windows))]
    return false;
}

struct VideoFrameController {
    cur: Instant,
    send_conn_ids: HashSet<i32>,
}

impl VideoFrameController {
    fn new() -> Self {
        Self {
            cur: Instant::now(),
            send_conn_ids: HashSet::new(),
        }
    }

    fn reset(&mut self) {
        self.send_conn_ids.clear();
    }

    fn set_send(&mut self, tm: Instant, conn_ids: HashSet<i32>) {
        if !conn_ids.is_empty() {
            self.cur = tm;
            self.send_conn_ids = conn_ids;
        }
    }

    #[tokio::main(flavor = "current_thread")]
    async fn try_wait_next(&mut self, fetched_conn_ids: &mut HashSet<i32>, timeout_millis: u64) {
        if self.send_conn_ids.is_empty() {
            return;
        }

        let timeout_dur = Duration::from_millis(timeout_millis as u64);
        match tokio::time::timeout(timeout_dur, FRAME_FETCHED_NOTIFIER.1.lock().await.recv()).await
        {
            Err(_) => {
                // break if timeout
                // log::error!("blocking wait frame receiving timeout {}", timeout_millis);
            }
            Ok(Some((id, instant))) => {
                if let Some(tm) = instant {
                    log::trace!("Channel recv latency: {}", tm.elapsed().as_secs_f32());
                }
                fetched_conn_ids.insert(id);
            }
            Ok(None) => {
                // this branch would nerver be reached
            }
        }
    }
}

trait TraitCapturer {
    fn frame<'a>(&'a mut self, timeout: Duration) -> Result<Frame<'a>>;

    #[cfg(windows)]
    fn is_gdi(&self) -> bool;
    #[cfg(windows)]
    fn set_gdi(&mut self) -> bool;
}

impl TraitCapturer for Capturer {
    fn frame<'a>(&'a mut self, timeout: Duration) -> Result<Frame<'a>> {
        self.frame(timeout)
    }

    #[cfg(windows)]
    fn is_gdi(&self) -> bool {
        self.is_gdi()
    }

    #[cfg(windows)]
    fn set_gdi(&mut self) -> bool {
        self.set_gdi()
    }
}

#[cfg(windows)]
impl TraitCapturer for scrap::CapturerMag {
    fn frame<'a>(&'a mut self, _timeout_ms: Duration) -> Result<Frame<'a>> {
        self.frame(_timeout_ms)
    }

    fn is_gdi(&self) -> bool {
        false
    }

    fn set_gdi(&mut self) -> bool {
        false
    }
}

pub fn new() -> GenericService {
    let sp = GenericService::new(NAME, true);
    sp.run(run);
    sp
}

fn check_display_changed(
    last_n: usize,
    last_current: usize,
    last_width: usize,
    last_hegiht: usize,
) -> bool {
    let displays = match try_get_displays() {
        Ok(d) => d,
        _ => return false,
    };

    let n = displays.len();
    if n != last_n {
        return true;
    };

    for (i, d) in displays.iter().enumerate() {
        if d.is_primary() {
            if i != last_current {
                return true;
            };
            if d.width() != last_width || d.height() != last_hegiht {
                return true;
            };
        }
    }

    return false;
}

// Capturer object is expensive, avoiding to create it frequently.
fn create_capturer(
    privacy_mode_id: i32,
    display: Display,
    use_yuv: bool,
) -> ResultType<Box<dyn TraitCapturer>> {
    #[cfg(not(windows))]
    let c: Option<Box<dyn TraitCapturer>> = None;
    #[cfg(windows)]
    let mut c: Option<Box<dyn TraitCapturer>> = None;
    if privacy_mode_id > 0 {
        #[cfg(windows)]
        {
            use crate::ui::win_privacy::*;

            match scrap::CapturerMag::new(
                display.origin(),
                display.width(),
                display.height(),
                use_yuv,
            ) {
                Ok(mut c1) => {
                    let mut ok = false;
                    let check_begin = Instant::now();
                    while check_begin.elapsed().as_secs() < 5 {
                        match c1.exclude("", PRIVACY_WINDOW_NAME) {
                            Ok(false) => {
                                ok = false;
                                std::thread::sleep(std::time::Duration::from_millis(500));
                            }
                            Err(e) => {
                                bail!(
                                    "Failed to exclude privacy window {} - {}, err: {}",
                                    "",
                                    PRIVACY_WINDOW_NAME,
                                    e
                                );
                            }
                            _ => {
                                ok = true;
                                break;
                            }
                        }
                    }
                    if !ok {
                        bail!(
                            "Failed to exclude privacy window {} - {} ",
                            "",
                            PRIVACY_WINDOW_NAME
                        );
                    }
                    log::debug!("Create maginifier capture for {}", privacy_mode_id);
                    c = Some(Box::new(c1));
                }
                Err(e) => {
                    bail!(format!("Failed to create magnifier capture {}", e));
                }
            }
        }
    }

    let c = match c {
        Some(c1) => c1,
        None => {
            let c1 =
                Capturer::new(display, use_yuv).with_context(|| "Failed to create capturer")?;
            log::debug!("Create capturer dxgi|gdi");
            Box::new(c1)
        }
    };

    Ok(c)
}

#[cfg(windows)]
fn ensure_close_virtual_device() -> ResultType<()> {
    let num_displays = Display::all()?.len();
    if num_displays == 0 {
        // Device may sometimes be uninstalled by user in "Device Manager" Window.
        // Closing device will clear the instance data.
        virtual_display::close_device();
    } else if num_displays > 1 {
        // Try close device, if display device changed.
        if virtual_display::is_device_created() {
            virtual_display::close_device();
        }
    }
    Ok(())
}

pub fn test_create_capturer(privacy_mode_id: i32, timeout_millis: u64) -> bool {
    let test_begin = Instant::now();
    while test_begin.elapsed().as_millis() < timeout_millis as _ {
        if let Ok((_, _, display)) = get_current_display() {
            if let Ok(_) = create_capturer(privacy_mode_id, display, true) {
                return true;
            }
        }
        std::thread::sleep(Duration::from_millis(300));
    }
    false
}

#[cfg(windows)]
fn check_uac_switch(privacy_mode_id: i32, captuerer_privacy_mode_id: i32) -> ResultType<()> {
    if captuerer_privacy_mode_id != 0 {
        if privacy_mode_id != captuerer_privacy_mode_id {
            if !crate::ui::win_privacy::is_process_consent_running()? {
                bail!("consent.exe is running");
            }
        }
        if crate::ui::win_privacy::is_process_consent_running()? {
            bail!("consent.exe is running");
        }
    }
    Ok(())
}

fn run(sp: GenericService) -> ResultType<()> {
    #[cfg(windows)]
    ensure_close_virtual_device()?;

    let (ndisplay, current, display) = get_current_display()?;
    let (origin, width, height) = (display.origin(), display.width(), display.height());
    log::debug!(
        "#displays={}, current={}, origin: {:?}, width={}, height={}, cpus={}/{}",
        ndisplay,
        current,
        &origin,
        width,
        height,
        num_cpus::get_physical(),
        num_cpus::get(),
    );

    let mut video_qos = VIDEO_QOS.lock().unwrap();

    video_qos.set_size(width as _, height as _);
    let mut spf = video_qos.spf();
    let bitrate = video_qos.generate_bitrate()?;
    let abr = video_qos.check_abr_config();
    drop(video_qos);
    log::info!("init bitrate={}, abr enabled:{}", bitrate, abr);

    let encoder_cfg = match Encoder::current_hw_encoder_name() {
        Some(codec_name) => EncoderCfg::HW(HwEncoderConfig {
            codec_name,
            width,
            height,
            bitrate: bitrate as _,
        }),
        None => EncoderCfg::VPX(VpxEncoderConfig {
            width: width as _,
            height: height as _,
            timebase: [1, 1000], // Output timestamp precision
            bitrate,
            codec: VpxVideoCodecId::VP9,
            num_threads: (num_cpus::get() / 2) as _,
        }),
    };

    let mut encoder;
    match Encoder::new(encoder_cfg) {
        Ok(x) => encoder = x,
        Err(err) => bail!("Failed to create encoder: {}", err),
    }
    
    let privacy_mode_id = *PRIVACY_MODE_CONN_ID.lock().unwrap();
    #[cfg(not(windows))]
    let captuerer_privacy_mode_id = privacy_mode_id;
    #[cfg(windows)]
    let mut captuerer_privacy_mode_id = privacy_mode_id;
    #[cfg(windows)]
    if crate::ui::win_privacy::is_process_consent_running()? {
        captuerer_privacy_mode_id = 0;
    }
    log::debug!(
        "Try create capturer with captuerer privacy mode id {}",
        captuerer_privacy_mode_id,
    );

    if privacy_mode_id != captuerer_privacy_mode_id {
        log::info!("In privacy mode, but show UAC prompt window for now");
    } else {
        log::info!("In privacy mode, the peer side cannot watch the screen");
    }
    let mut c = create_capturer(captuerer_privacy_mode_id, display, encoder.use_yuv())?;

    if *SWITCH.lock().unwrap() {
        log::debug!("Broadcasting display switch");
        let mut misc = Misc::new();
        misc.set_switch_display(SwitchDisplay {
            display: current as _,
            x: origin.0 as _,
            y: origin.1 as _,
            width: width as _,
            height: height as _,
            ..Default::default()
        });
        let mut msg_out = Message::new();
        msg_out.set_misc(misc);
        *SWITCH.lock().unwrap() = false;
        sp.send(msg_out);
    }

    let mut frame_controller = VideoFrameController::new();

    let start = time::Instant::now();
    let mut last_check_displays = time::Instant::now();
    #[cfg(windows)]
    let mut try_gdi = 1;
    #[cfg(windows)]
    log::info!("gdi: {}", c.is_gdi());

    while sp.ok() {
        #[cfg(windows)]
        check_uac_switch(privacy_mode_id, captuerer_privacy_mode_id)?;

        {
            let mut video_qos = VIDEO_QOS.lock().unwrap();
            if video_qos.check_if_updated() {
                log::debug!(
                    "qos is updated, target_bitrate:{}, fps:{}",
                    video_qos.target_bitrate,
                    video_qos.fps
                );
                encoder.set_bitrate(video_qos.target_bitrate).unwrap();
                spf = video_qos.spf();
            }
        }

        if *SWITCH.lock().unwrap() {
            bail!("SWITCH");
        }
        if current != *CURRENT_DISPLAY.lock().unwrap() {
            *SWITCH.lock().unwrap() = true;
            bail!("SWITCH");
        }
        check_privacy_mode_changed(&sp, privacy_mode_id)?;
        #[cfg(windows)]
        {
            if crate::platform::windows::desktop_changed() {
                bail!("Desktop changed");
            }
        }
        let now = time::Instant::now();
        if last_check_displays.elapsed().as_millis() > 1000 {
            last_check_displays = now;
            if ndisplay != get_display_num() {
                log::info!("Displays changed");
                *SWITCH.lock().unwrap() = true;
                bail!("SWITCH");
            }
        }

        *LAST_ACTIVE.lock().unwrap() = now;

        frame_controller.reset();

        #[cfg(any(target_os = "android", target_os = "ios"))]
        let res = match c.frame(spf) {
            Ok(frame) => {
                let time = now - start;
                let ms = (time.as_secs() * 1000 + time.subsec_millis() as u64) as i64;
                match frame {
                    scrap::Frame::VP9(data) => {
                        let send_conn_ids = handle_one_frame_encoded(&sp, data, ms)?;
                        frame_controller.set_send(now, send_conn_ids);
                    }
                    scrap::Frame::RAW(data) => {
                        if (data.len() != 0) {
                            let send_conn_ids = handle_one_frame(&sp, data, ms, &mut encoder)?;
                            frame_controller.set_send(now, send_conn_ids);
                        }
                    }
                    _ => {}
                };
                Ok(())
            }
            Err(err) => Err(err),
        };

        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        let res = match c.frame(spf) {
            Ok(frame) => {
                let time = now - start;
                let ms = (time.as_secs() * 1000 + time.subsec_millis() as u64) as i64;
                let send_conn_ids = handle_one_frame(&sp, &frame, ms, &mut encoder)?;
                frame_controller.set_send(now, send_conn_ids);
                #[cfg(windows)]
                {
                    try_gdi = 0;
                }
                Ok(())
            }
            Err(err) => Err(err),
        };

        match res {
            Err(ref e) if e.kind() == WouldBlock =>
            {
                #[cfg(windows)]
                if try_gdi > 0 && !c.is_gdi() {
                    if try_gdi > 3 {
                        c.set_gdi();
                        try_gdi = 0;
                        log::info!("No image, fall back to gdi");
                    }
                    try_gdi += 1;
                }
            }
            Err(err) => {
                if check_display_changed(ndisplay, current, width, height) {
                    log::info!("Displays changed");
                    *SWITCH.lock().unwrap() = true;
                    bail!("SWITCH");
                }

                #[cfg(windows)]
                if !c.is_gdi() {
                    c.set_gdi();
                    log::info!("dxgi error, fall back to gdi: {:?}", err);
                    continue;
                }

                return Err(err.into());
            }
            _ => {}
        }

        let mut fetched_conn_ids = HashSet::new();
        let timeout_millis = 3_000u64;
        let wait_begin = Instant::now();
        while wait_begin.elapsed().as_millis() < timeout_millis as _ {
            check_privacy_mode_changed(&sp, privacy_mode_id)?;
            #[cfg(windows)]
            check_uac_switch(privacy_mode_id, captuerer_privacy_mode_id)?;
            frame_controller.try_wait_next(&mut fetched_conn_ids, 300);
            // break if all connections have received current frame
            if fetched_conn_ids.len() >= frame_controller.send_conn_ids.len() {
                break;
            }
        }

        let elapsed = now.elapsed();
        // may need to enable frame(timeout)
        log::trace!("{:?} {:?}", time::Instant::now(), elapsed);
        if elapsed < spf {
            std::thread::sleep(spf - elapsed);
        }
    }
    Ok(())
}

fn check_privacy_mode_changed(sp: &GenericService, privacy_mode_id: i32) -> ResultType<()> {
    let privacy_mode_id_2 = *PRIVACY_MODE_CONN_ID.lock().unwrap();
    if privacy_mode_id != privacy_mode_id_2 {
        if privacy_mode_id_2 != 0 {
            let msg_out = crate::common::make_privacy_mode_msg(
                back_notification::PrivacyModeState::OnByOther,
            );
            sp.send_to_others(msg_out, privacy_mode_id_2);
        }
        bail!("SWITCH");
    }
    Ok(())
}

#[inline]
#[cfg(any(target_os = "android", target_os = "ios"))]
fn create_msg(vp9s: Vec<VP9>) -> Message {
    let mut msg_out = Message::new();
    let mut vf = VideoFrame::new();
    vf.set_vp9s(VP9s {
        frames: vp9s.into(),
        ..Default::default()
    });
    vf.timestamp = crate::common::get_time();
    msg_out.set_video_frame(vf);
    msg_out
}

#[inline]
fn handle_one_frame(
    sp: &GenericService,
    frame: &[u8],
    ms: i64,
    encoder: &mut Encoder,
) -> ResultType<HashSet<i32>> {
    sp.snapshot(|sps| {
        // so that new sub and old sub share the same encoder after switch
        if sps.has_subscribes() {
            bail!("SWITCH");
        }
        Ok(())
    })?;

    let mut send_conn_ids: HashSet<i32> = Default::default();
    if let Ok(msg) = encoder.encode_to_message(frame, ms) {
        send_conn_ids = sp.send_video_frame(msg);
    }
    Ok(send_conn_ids)
}

#[inline]
#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn handle_one_frame_encoded(
    sp: &GenericService,
    frame: &[u8],
    ms: i64,
) -> ResultType<HashSet<i32>> {
    sp.snapshot(|sps| {
        // so that new sub and old sub share the same encoder after switch
        if sps.has_subscribes() {
            bail!("SWITCH");
        }
        Ok(())
    })?;
    let mut send_conn_ids: HashSet<i32> = Default::default();
    let vp9_frame = VP9 {
        data: frame.to_vec(),
        key: true,
        pts: ms,
        ..Default::default()
    };
    send_conn_ids = sp.send_video_frame(create_msg(vec![vp9_frame]));
    Ok(send_conn_ids)
}

fn get_display_num() -> usize {
    if let Ok(d) = try_get_displays() {
        d.len()
    } else {
        0
    }
}

pub fn get_displays() -> ResultType<(usize, Vec<DisplayInfo>)> {
    // switch to primary display if long time (30 seconds) no users
    if LAST_ACTIVE.lock().unwrap().elapsed().as_secs() >= 30 {
        *CURRENT_DISPLAY.lock().unwrap() = usize::MAX;
    }
    let mut displays = Vec::new();
    let mut primary = 0;
    for (i, d) in try_get_displays()?.iter().enumerate() {
        if d.is_primary() {
            primary = i;
        }
        displays.push(DisplayInfo {
            x: d.origin().0 as _,
            y: d.origin().1 as _,
            width: d.width() as _,
            height: d.height() as _,
            name: d.name(),
            online: d.is_online(),
            ..Default::default()
        });
    }
    let mut lock = CURRENT_DISPLAY.lock().unwrap();
    if *lock >= displays.len() {
        *lock = primary
    }
    Ok((*lock, displays))
}

pub fn switch_display(i: i32) {
    let i = i as usize;
    if let Ok((_, displays)) = get_displays() {
        if i < displays.len() {
            *CURRENT_DISPLAY.lock().unwrap() = i;
        }
    }
}

pub fn refresh() {
    #[cfg(target_os = "android")]
    Display::refresh_size();
    *SWITCH.lock().unwrap() = true;
}

fn get_primary() -> usize {
    if let Ok(all) = try_get_displays() {
        for (i, d) in all.iter().enumerate() {
            if d.is_primary() {
                return i;
            }
        }
    }
    0
}

pub fn switch_to_primary() {
    switch_display(get_primary() as _);
}

#[cfg(not(windows))]
fn try_get_displays() -> ResultType<Vec<Display>> {
    Ok(Display::all()?)
}

#[cfg(windows)]
fn try_get_displays() -> ResultType<Vec<Display>> {
    let mut displays = Display::all()?;
    if displays.len() == 0 {
        log::debug!("no displays, create virtual display");
        // Try plugin monitor
        if !virtual_display::is_device_created() {
            if let Err(e) = virtual_display::create_device() {
                log::debug!("Create device failed {}", e);
            }
        }
        if virtual_display::is_device_created() {
            if let Err(e) = virtual_display::plug_in_monitor() {
                log::debug!("Plug in monitor failed {}", e);
            } else {
                if let Err(e) = virtual_display::update_monitor_modes() {
                    log::debug!("Update monitor modes failed {}", e);
                }
            }
        }
        displays = Display::all()?;
    } else if displays.len() > 1 {
        // If more than one displays exists, close RustDeskVirtualDisplay
        if virtual_display::is_device_created() {
            virtual_display::close_device()
        }
    }
    Ok(displays)
}

fn get_current_display() -> ResultType<(usize, usize, Display)> {
    let mut current = *CURRENT_DISPLAY.lock().unwrap() as usize;
    let mut displays = try_get_displays()?;
    if displays.len() == 0 {
        bail!("No displays");
    }
    let n = displays.len();
    if current >= n {
        current = 0;
        for (i, d) in displays.iter().enumerate() {
            if d.is_primary() {
                current = i;
                break;
            }
        }
        *CURRENT_DISPLAY.lock().unwrap() = current;
    }
    return Ok((n, current, displays.remove(current)));
}
