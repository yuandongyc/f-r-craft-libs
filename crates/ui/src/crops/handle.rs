use egui::{Color32, Id, Rect, Response , Vec2};

use super::CropMode;
use super::geometry::valid_rect;

#[derive(Clone, Copy, Debug)]
pub struct CropHandleStyle {
    pub hovered: Color32,
    pub dragged: Color32,
    pub focused: Color32,
}

pub struct CropResponse {
    pub response: Response,
    pub delta: Vec2,
}

pub struct CropHandle<'a> {
    id: Id,
    rect: Rect,
    mode: CropMode,
    style: &'a CropHandleStyle,
}

impl<'a> CropHandle<'a> {
    pub fn new(id: Id, rect: Rect, mode: CropMode, style: &'a CropHandleStyle) -> Self {
        Self {
            id,
            rect,
            mode,
            style,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn mode(&self) -> CropMode {
        self.mode
    }

    pub fn style(&self) -> &CropHandleStyle {
        self.style
    }
}