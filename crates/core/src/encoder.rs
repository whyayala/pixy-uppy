#[derive(Debug, Clone)]
pub enum EncoderKind {
    H264Nvenc,
    HevcNvenc,
    H264Amf,
    HevcAmf,
    H264Qsv,
    HevcQsv,
    H264Vaapi,
    HevcVaapi,
    Libx264,
    Libx265,
}

#[derive(Debug, Clone)]
pub struct EncoderOptions {
    pub encoder: EncoderKind,
    pub preset: Option<String>,
    pub tune: Option<String>,
    pub crf: Option<u8>,
    pub pix_fmt: Option<String>,
    pub container: Option<String>,
}

impl EncoderOptions {
    pub fn to_ffmpeg_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        args.push("-c:v".into());
        args.push(match self.encoder {
            EncoderKind::H264Nvenc => "h264_nvenc",
            EncoderKind::HevcNvenc => "hevc_nvenc",
            EncoderKind::H264Amf => "h264_amf",
            EncoderKind::HevcAmf => "hevc_amf",
            EncoderKind::H264Qsv => "h264_qsv",
            EncoderKind::HevcQsv => "hevc_qsv",
            EncoderKind::H264Vaapi => "h264_vaapi",
            EncoderKind::HevcVaapi => "hevc_vaapi",
            EncoderKind::Libx264 => "libx264",
            EncoderKind::Libx265 => "libx265",
        }
        .into());

        if let Some(preset) = &self.preset {
            args.push("-preset".into());
            args.push(preset.clone());
        }
        if let Some(tune) = &self.tune {
            args.push("-tune".into());
            args.push(tune.clone());
        }
        if let Some(crf) = self.crf {
            args.push("-crf".into());
            args.push(crf.to_string());
        }
        if let Some(pix_fmt) = &self.pix_fmt {
            args.push("-pix_fmt".into());
            args.push(pix_fmt.clone());
        }
        args
    }
}


