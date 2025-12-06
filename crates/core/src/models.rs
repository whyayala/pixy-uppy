/// Known upscaler model families supported out of the box.
#[derive(Debug, Clone)]
pub enum ModelKind {
    RealEsrgan,
    RealCugan,
    Waifu2x,
}

/// Model specification including scale and optional denoise level.
/// Why: Encapsulates selection metadata surfaced in CLI/GUI.
#[derive(Debug, Clone)]
pub struct ModelSpec {
    pub name: String,
    pub kind: ModelKind,
    pub scale: u32,
    pub denoise_level: Option<u8>,
    pub path: Option<String>,
}

/// Returns curated models suitable for live-action and animation sources.
/// Why: Provides sane defaults without requiring users to hunt models.
pub fn curated_models() -> Vec<ModelSpec> {
    vec![
        ModelSpec {
            name: "realesrgan-x4plus".into(),
            kind: ModelKind::RealEsrgan,
            scale: 4,
            denoise_level: None,
            path: None,
        },
        ModelSpec {
            name: "realesrgan-x4plus-anime".into(),
            kind: ModelKind::RealEsrgan,
            scale: 4,
            denoise_level: None,
            path: None,
        },
        ModelSpec {
            name: "realesr-animevideov3-x4".into(),
            kind: ModelKind::RealEsrgan,
            scale: 4,
            denoise_level: None,
            path: None,
        },
        ModelSpec {
            name: "realesr-animevideov3-x3".into(),
            kind: ModelKind::RealEsrgan,
            scale: 3,
            denoise_level: None,
            path: None,
        },
        ModelSpec {
            name: "realesr-animevideov3-x2".into(),
            kind: ModelKind::RealEsrgan,
            scale: 2,
            denoise_level: None,
            path: None,
        },
        ModelSpec {
            name: "realcugan_se_x2".into(),
            kind: ModelKind::RealCugan,
            scale: 2,
            denoise_level: Some(1),
            path: None,
        },
        ModelSpec {
            name: "waifu2x_cunet_x2".into(),
            kind: ModelKind::Waifu2x,
            scale: 2,
            denoise_level: Some(1),
            path: None,
        },
    ]
}
