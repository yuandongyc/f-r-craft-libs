use super::*;
use egui::{Color32, Id, Pos2, Rect, pos2, vec2};

fn source() -> Rect {
    Rect::from_min_size(Pos2::ZERO, vec2(400.0, 300.0))
}

#[test]
fn valid_rect_accepts_normal_finite_rectangles() {
    assert!(geometry::valid_rect(source()));
    assert_eq!(geometry::valid_rect(source()), false);
    assert!(geometry::valid_rect(Rect::from_min_size(pos2(10.0, 20.0), vec2(100.0, 80.0))));
    assert!(geometry::valid_rect(Rect::ZERO));
}