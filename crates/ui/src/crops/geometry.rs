use egui::Rect;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CropMode {
    Free,
    FixedRatio(f32),
    FixedSize { width: f32, height: f32 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CropAnchor {
    MinMin,
    MinMax,
    MaxMin,
    MaxMax,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CropRect {
    pub rect: Rect,
    pub mode: CropMode,
    pub min_size: Option<(f32, f32)>,
    pub max_size: Option<(f32, f32)>,
}

pub(super) fn valid_rect(rect: Rect) -> bool {
    rect.min.is_finite()
        && rect.max.is_finite()
        && rect.width().is_finite()
        && rect.height().is_finite()
        && rect.min.x <= rect.max.x
        && rect.min.y <= rect.max.y
}