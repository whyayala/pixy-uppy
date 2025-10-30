use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use pixy_core::devices::detect_vulkan_devices;
use pixy_core::encoder::{EncoderKind, EncoderOptions};
use pixy_core::frames::{FrameExtractOptions, Prefilter};
use pixy_core::models::{curated_models, ModelKind};
use pixy_core::pipeline::{run_upscale_job, UpscaleJob};
use pixy_core::upscalers::{UpscalerBinary, UpscalerKind};

#[derive(Parser)]
#[command(name = "pixy-uppy")] 
#[command(about = "Video upscaler (NCNN/Vulkan + FFmpeg)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Devices,
    Models,
    Upscale(ArgsUpscale),
}

#[derive(clap::Args)]
struct ArgsUpscale {
    #[arg(short, long)]
    input: PathBuf,
    #[arg(short, long)]
    output: PathBuf,
    #[arg(short = 'm', long)]
    model: String,
    #[arg(long)]
    scale: Option<u32>,
    #[arg(long)]
    width: Option<u32>,
    #[arg(long)]
    height: Option<u32>,
    #[arg(long, default_value_t = 0)]
    gpu: usize,
    #[arg(long)]
    tile_size: Option<u32>,
    #[arg(long)]
    threads: Option<u32>,
    #[arg(long, value_enum, default_value_t = Enc::HevcNvenc)]
    encoder: Enc,
    #[arg(long)]
    preset: Option<String>,
    #[arg(long)]
    tune: Option<String>,
    #[arg(long)]
    crf: Option<u8>,
    #[arg(long, default_value = "png")]
    frame_format: String,
    #[arg(long, value_enum, default_value_t = Filter::None)]
    prefilter: Filter,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Enc { H264Nvenc, HevcNvenc, H264Amf, HevcAmf, H264Qsv, HevcQsv, H264Vaapi, HevcVaapi, Libx264, Libx265 }

impl From<Enc> for EncoderKind {
    fn from(e: Enc) -> Self {
        match e {
            Enc::H264Nvenc => EncoderKind::H264Nvenc,
            Enc::HevcNvenc => EncoderKind::HevcNvenc,
            Enc::H264Amf => EncoderKind::H264Amf,
            Enc::HevcAmf => EncoderKind::HevcAmf,
            Enc::H264Qsv => EncoderKind::H264Qsv,
            Enc::HevcQsv => EncoderKind::HevcQsv,
            Enc::H264Vaapi => EncoderKind::H264Vaapi,
            Enc::HevcVaapi => EncoderKind::HevcVaapi,
            Enc::Libx264 => EncoderKind::Libx264,
            Enc::Libx265 => EncoderKind::Libx265,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Filter { Yadif, Hqdn3d, Deband, None }

impl From<Filter> for Prefilter {
    fn from(f: Filter) -> Self {
        match f { Filter::Yadif => Prefilter::Yadif, Filter::Hqdn3d => Prefilter::Hqdn3d, Filter::Deband => Prefilter::Deband, Filter::None => Prefilter::None }
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Devices => {
            // try Real-ESRGAN first
            let devices = detect_vulkan_devices("realesrgan-ncnn-vulkan").unwrap_or_default();
            if devices.is_empty() { println!("No devices detected (binary not found or no Vulkan devices)"); }
            for d in devices { println!("{}: {}", d.index, d.name); }
        }
        Commands::Models => {
            for m in curated_models() {
                let kind = match m.kind { ModelKind::RealEsrgan => "realesrgan", ModelKind::RealCugan => "realcugan", ModelKind::Waifu2x => "waifu2x" };
                println!("{}\t(kind: {}, scale: {}x)", m.name, kind, m.scale);
            }
        }
        Commands::Upscale(args) => {
            let model = curated_models().into_iter().find(|m| m.name == args.model).expect("model not found");
            let upscaler = UpscalerBinary {
                kind: match model.kind { ModelKind::RealEsrgan => UpscalerKind::RealEsrgan, ModelKind::RealCugan => UpscalerKind::RealCugan, ModelKind::Waifu2x => UpscalerKind::Waifu2x },
                path: which::which(match model.kind { ModelKind::RealEsrgan => "realesrgan-ncnn-vulkan", ModelKind::RealCugan => "realcugan-ncnn-vulkan", ModelKind::Waifu2x => "waifu2x-ncnn-vulkan" }).expect("upscaler binary not found"),
            };

            let job = UpscaleJob {
                input: args.input,
                output: args.output,
                model,
                upscaler,
                gpu_index: args.gpu,
                tile_size: args.tile_size,
                threads: args.threads,
                target_width: args.width,
                target_height: args.height,
                scale: args.scale,
                extract: FrameExtractOptions { prefilter: args.prefilter.into(), frame_format: args.frame_format },
                encoder: EncoderOptions { encoder: args.encoder.into(), preset: args.preset, tune: args.tune, crf: args.crf, pix_fmt: Some("yuv420p".into()), container: None },
                container: "mkv".into(),
            };
            if let Err(e) = run_upscale_job(&job) { eprintln!("error: {}", e); std::process::exit(1); }
        }
    }
}


