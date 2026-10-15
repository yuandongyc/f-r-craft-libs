use super::*;
use egui::{Color32, Id, Pos2, Rect, pos2, vec2};

fn source() -> Rect {
    Rect::from_min_size(Pos2::ZERO, vec2(400.0, 300.0))
}

#[test]
fn valid_rect_accepts_normal_finite_rectangles() {
    assert!(geometry::valid_rect(source()));
    assert_eq!(geometry::valid_rect(source()), true);
    assert!(geometry::valid_rect(Rect::from_min_size(pos2(10.0, 20.0), vec2(100.0, 80.0))));
    assert!(geometry::valid_rect(Rect::ZERO));
}

#[test]
fn valid_rect_rejects_invalid_rectangles() {
    let nan_rect = Rect::from_min_size(pos2(f32::NAN, 0.0), vec2(100.0, 100.0));
    assert!(!geometry::valid_rect(nan_rect));

    let inf_rect = Rect::from_min_size(pos2(f32::INFINITY, 0.0), vec2(100.0, 100.0));
    assert!(!geometry::valid_rect(inf_rect));

    let neg_inf_rect = Rect::from_min_size(pos2(0.0, f32::NEG_INFINITY), vec2(100.0, 100.0));
    assert!(!geometry::valid_rect(neg_inf_rect));

    let reversed = Rect {
        min: pos2(200.0, 200.0),
        max: pos2(100.0, 100.0),
    };
    assert!(!geometry::valid_rect(reversed));
}

#[test]
fn valid_rect_accepts_zero_sized_rectangles() {
    let point = Rect::from_min_max(pos2(50.0, 50.0), pos2(50.0, 50.0));
    assert!(geometry::valid_rect(point));
    assert_eq!(point.width(), 0.0);
    assert_eq!(point.height(), 0.0);
}

#[test]
fn crop_mode_free_constructs_correctly() {
    let crop = CropRect { rect: source(), mode: CropMode::Free, min_size: None, max_size: None };
    assert_eq!(crop.mode, CropMode::Free);
    assert!(geometry::valid_rect(crop.rect));
    assert!(crop.min_size.is_none());
    assert!(crop.max_size.is_none());
}

#[test]
fn crop_mode_fixed_ratio_stores_ratio() {
    let ratios = [1.0, 4.0 / 3.0, 16.0 / 9.0, 3.0 / 4.0, 9.0 / 16.0];
    for ratio in ratios {
        let crop = CropRect { rect: source(), mode: CropMode::FixedRatio(ratio), min_size: None, max_size: None };
        assert_eq!(crop.mode, CropMode::FixedRatio(ratio));
    }
}

#[test]
fn crop_mode_fixed_size_stores_dimensions() {
    let crop = CropRect {
        rect: source(),
        mode: CropMode::FixedSize { width: 200.0, height: 150.0 },
        min_size: None,
        max_size: None,
    };
    assert_eq!(crop.mode, CropMode::FixedSize { width: 200.0, height: 150.0 });
}


#[test]
fn crop_rect_with_min_and_max_constraints() {
    let crop = CropRect {
        rect: source(),
        mode: CropMode::Free,
        min_size: Some((50.0, 50.0)),
        max_size: Some((300.0, 200.0)),
    };
    let (min_w, min_h) = crop.min_size.unwrap();
    let (max_w, max_h) = crop.max_size.unwrap();
    assert_eq!(min_w, 50.0);
    assert_eq!(min_h, 50.0);
    assert_eq!(max_w, 300.0);
    assert_eq!(max_h, 200.0);
    assert!(min_w <= max_w);
    assert!(min_h <= max_h);
}

#[test]
fn crop_anchor_variants_are_distinct() {
    let anchors = [CropAnchor::MinMin, CropAnchor::MinMax, CropAnchor::MaxMin, CropAnchor::MaxMax];
    for (i, a) in anchors.iter().enumerate() {
        for (j, b) in anchors.iter().enumerate() {
            assert_eq!(i == j, a == b);
        }
    }
}

#[test]
fn crop_handle_style_stores_colors() {
    let style = CropHandleStyle {
        hovered: Color32::GREEN,
        dragged: Color32::RED,
        focused: Color32::BLUE,
    };
    assert_eq!(style.hovered, Color32::GREEN);
    assert_eq!(style.dragged, Color32::RED);
    assert_eq!(style.focused, Color32::BLUE);
}

#[test]
fn crop_handle_new_constructs_from_parts() {
    let style = CropHandleStyle {
        hovered: Color32::GREEN,
        dragged: Color32::RED,
        focused: Color32::BLUE,
    };
    let id = Id::new("test-crop");
    let rect = source();
    let handle = CropHandle::new(id, rect, CropMode::Free, &style);
    assert_eq!(handle.id(), id);
    assert_eq!(handle.rect(), rect);
    assert_eq!(handle.mode(), CropMode::Free);
}