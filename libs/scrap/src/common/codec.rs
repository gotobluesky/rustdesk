use std::ops::{Deref, DerefMut};
#[cfg(feature = "hwcodec")]
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[cfg(feature = "hwcodec")]
use crate::hwcodec::*;
use crate::vpxcodec::*;

use hbb_common::{
    anyhow::anyhow,
    config::Config2,
    log,
    message_proto::{test_delay, video_frame, Message, VP9s, VideoCodecState},
    ResultType,
};
#[cfg(feature = "hwcodec")]
use hbb_common::{
    lazy_static,
    message_proto::{H264s, H265s},
};

#[cfg(feature = "hwcodec")]
lazy_static::lazy_static! {
    static ref PEER_DECODER_STATES: Arc<Mutex<HashMap<i32, VideoCodecState>>> = Default::default();
    static ref MY_DECODER_STATE: Arc<Mutex<VideoCodecState>> = Default::default();
}
const SCORE_VPX: i32 = 90;

#[derive(Debug, Clone)]
pub struct HwEncoderConfig {
    pub codec_name: String,
    pub width: usize,
    pub height: usize,
    pub bitrate: i32,
}

#[derive(Debug, Clone)]
pub enum EncoderCfg {
    VPX(VpxEncoderConfig),
    HW(HwEncoderConfig),
}

pub trait EncoderApi {
    fn new(cfg: EncoderCfg) -> ResultType<Self>
    where
        Self: Sized;

    fn encode_to_message(&mut self, frame: &[u8], ms: i64) -> ResultType<Message>;

    fn use_yuv(&self) -> bool;

    fn set_bitrate(&mut self, bitrate: u32) -> ResultType<()>;

    fn get_codec_format(&self) -> test_delay::CodecFormat;
}

pub struct DecoderCfg {
    pub vpx: VpxDecoderConfig,
}

pub struct Encoder {
    pub codec: Box<dyn EncoderApi>,
}

impl Deref for Encoder {
    type Target = Box<dyn EncoderApi>;

    fn deref(&self) -> &Self::Target {
        &self.codec
    }
}

impl DerefMut for Encoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.codec
    }
}

pub struct Decoder {
    vpx: VpxDecoder,
    #[cfg(feature = "hwcodec")]
    hw: HwDecoders,
    #[cfg(feature = "hwcodec")]
    i420: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum EncoderUpdate {
    State(VideoCodecState),
    Remove,
    DisableHwIfNotExist,
}

impl Encoder {
    pub fn new(config: EncoderCfg) -> ResultType<Encoder> {
        log::info!("new encoder:{:?}", config);
        match config {
            EncoderCfg::VPX(_) => Ok(Encoder {
                codec: Box::new(VpxEncoder::new(config)?),
            }),

            #[cfg(feature = "hwcodec")]
            EncoderCfg::HW(_) => match HwEncoder::new(config) {
                Ok(hw) => Ok(Encoder {
                    codec: Box::new(hw),
                }),
                Err(e) => {
                    HwEncoder::best(true, true);
                    Err(e)
                }
            },
            #[cfg(not(feature = "hwcodec"))]
            _ => Err(anyhow!("unsupported encoder type")),
        }
    }

    // TODO
    pub fn update_video_encoder(id: i32, update: EncoderUpdate) {
        log::info!("encoder update: {:?}", update);
        #[cfg(feature = "hwcodec")]
        {
            let mut states = PEER_DECODER_STATES.lock().unwrap();
            match update {
                EncoderUpdate::State(state) => {
                    states.insert(id, state);
                }
                EncoderUpdate::Remove => {
                    states.remove(&id);
                }
                EncoderUpdate::DisableHwIfNotExist => {
                    if !states.contains_key(&id) {
                        states.insert(id, VideoCodecState::default());
                    }
                }
            }
            let current_encoder_name = HwEncoder::current_name();
            if states.len() > 0 {
                let (best, _) = HwEncoder::best(false, true);
                let enabled_h264 =
                    best.h264.is_some() && states.len() > 0 && states.iter().all(|(_, s)| s.H264);
                let enabled_h265 =
                    best.h265.is_some() && states.len() > 0 && states.iter().all(|(_, s)| s.H265);

                // score encoder
                let mut score_vpx = SCORE_VPX;
                let mut score_h264 = best.h264.as_ref().map_or(0, |c| c.score);
                let mut score_h265 = best.h265.as_ref().map_or(0, |c| c.score);

                // score decoder
                score_vpx += states.iter().map(|s| s.1.ScoreVpx).sum::<i32>();
                if enabled_h264 {
                    score_h264 += states.iter().map(|s| s.1.ScoreH264).sum::<i32>();
                }
                if enabled_h265 {
                    score_h265 += states.iter().map(|s| s.1.ScoreH265).sum::<i32>();
                }

                if enabled_h265 && score_h265 >= score_vpx && score_h265 >= score_h264 {
                    *current_encoder_name.lock().unwrap() = Some(best.h265.unwrap().name);
                } else if enabled_h264 && score_h264 >= score_vpx && score_h264 >= score_h265 {
                    *current_encoder_name.lock().unwrap() = Some(best.h264.unwrap().name);
                } else {
                    *current_encoder_name.lock().unwrap() = None;
                }
                log::info!(
                    "connection count:{}, h264:{}, h265:{}, score: vpx({}), h264({}), h265({}), set current encoder name {:?}",
                    states.len(),
                    enabled_h264,
                    enabled_h265,
                    score_vpx,
                    score_h264,
                    score_h265,
                    current_encoder_name.lock().unwrap()
                    )
            } else {
                *current_encoder_name.lock().unwrap() = None;
            }
        }
        #[cfg(not(feature = "hwcodec"))]
        {
            let _ = id;
            let _ = update;
        }
    }
    #[inline]
    pub fn current_hw_encoder_name() -> Option<String> {
        #[cfg(feature = "hwcodec")]
        if check_hwcodec_config() {
            return HwEncoder::current_name().lock().unwrap().clone();
        } else {
            return None;
        }
        #[cfg(not(feature = "hwcodec"))]
        return None;
    }
}

#[cfg(feature = "hwcodec")]
impl Drop for Decoder {
    fn drop(&mut self) {
        *MY_DECODER_STATE.lock().unwrap() = VideoCodecState {
            ScoreVpx: SCORE_VPX,
            ..Default::default()
        };
    }
}

impl Decoder {
    pub fn video_codec_state() -> VideoCodecState {
        // video_codec_state is mainted by creation and destruction of Decoder.
        // It has been ensured to use after Decoder's creation.
        #[cfg(feature = "hwcodec")]
        if check_hwcodec_config() {
            return MY_DECODER_STATE.lock().unwrap().clone();
        } else {
            return VideoCodecState {
                ScoreVpx: SCORE_VPX,
                ..Default::default()
            };
        }
        #[cfg(not(feature = "hwcodec"))]
        VideoCodecState {
            ScoreVpx: SCORE_VPX,
            ..Default::default()
        }
    }

    pub fn new(config: DecoderCfg) -> Decoder {
        let vpx = VpxDecoder::new(config.vpx).unwrap();
        let decoder = Decoder {
            vpx,
            #[cfg(feature = "hwcodec")]
            hw: HwDecoder::new_decoders(),
            #[cfg(feature = "hwcodec")]
            i420: vec![],
        };

        #[cfg(feature = "hwcodec")]
        {
            let mut state = MY_DECODER_STATE.lock().unwrap();
            state.ScoreVpx = SCORE_VPX;
            state.H264 = decoder.hw.h264.is_some();
            state.ScoreH264 = decoder.hw.h264.as_ref().map_or(0, |d| d.info.score);
            state.H265 = decoder.hw.h265.is_some();
            state.ScoreH265 = decoder.hw.h265.as_ref().map_or(0, |d| d.info.score);
        }

        decoder
    }

    pub fn handle_video_frame(
        &mut self,
        frame: &video_frame::Union,
        rgb: &mut Vec<u8>,
    ) -> ResultType<bool> {
        match frame {
            video_frame::Union::vp9s(vp9s) => {
                Decoder::handle_vp9s_video_frame(&mut self.vpx, vp9s, rgb)
            }
            #[cfg(feature = "hwcodec")]
            video_frame::Union::h264s(h264s) => {
                if let Some(decoder) = &mut self.hw.h264 {
                    Decoder::handle_h264s_video_frame(decoder, h264s, rgb, &mut self.i420)
                } else {
                    Err(anyhow!("don't support h264!"))
                }
            }
            #[cfg(feature = "hwcodec")]
            video_frame::Union::h265s(h265s) => {
                if let Some(decoder) = &mut self.hw.h265 {
                    Decoder::handle_h265s_video_frame(decoder, h265s, rgb, &mut self.i420)
                } else {
                    Err(anyhow!("don't support h265!"))
                }
            }
            _ => Err(anyhow!("unsupported video frame type!")),
        }
    }

    fn handle_vp9s_video_frame(
        decoder: &mut VpxDecoder,
        vp9s: &VP9s,
        rgb: &mut Vec<u8>,
    ) -> ResultType<bool> {
        let mut last_frame = Image::new();
        for vp9 in vp9s.frames.iter() {
            for frame in decoder.decode(&vp9.data)? {
                drop(last_frame);
                last_frame = frame;
            }
        }
        for frame in decoder.flush()? {
            drop(last_frame);
            last_frame = frame;
        }
        if last_frame.is_null() {
            Ok(false)
        } else {
            last_frame.rgb(1, true, rgb);
            Ok(true)
        }
    }

    #[cfg(feature = "hwcodec")]
    fn handle_h264s_video_frame(
        decoder: &mut HwDecoder,
        h264s: &H264s,
        rgb: &mut Vec<u8>,
        i420: &mut Vec<u8>,
    ) -> ResultType<bool> {
        let mut ret = false;
        for h264 in h264s.h264s.iter() {
            for image in decoder.decode(&h264.data)? {
                // TODO: just process the last frame
                if image.bgra(rgb, i420).is_ok() {
                    ret = true;
                }
            }
        }
        return Ok(ret);
    }

    #[cfg(feature = "hwcodec")]
    fn handle_h265s_video_frame(
        decoder: &mut HwDecoder,
        h265s: &H265s,
        rgb: &mut Vec<u8>,
        i420: &mut Vec<u8>,
    ) -> ResultType<bool> {
        let mut ret = false;
        for h265 in h265s.h265s.iter() {
            for image in decoder.decode(&h265.data)? {
                // TODO: just process the last frame
                if image.bgra(rgb, i420).is_ok() {
                    ret = true;
                }
            }
        }
        return Ok(ret);
    }
}

fn check_hwcodec_config() -> bool {
    if let Some(v) = Config2::get().options.get("enable-hwcodec") {
        return v != "N";
    }
    return true; // default is true
}
