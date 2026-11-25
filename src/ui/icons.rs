use egui::{Color32, Pos2, Rect, Stroke, Ui, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconKind {
    Home,
    Dashboard,
    Package,
    ShoppingCart,
    AlertTriangle,
    Wrench,
    Calendar,
    Upload,
    Save,
    Refresh,
    Plus,
    Activity,
    CheckCircle,
    FileText,
    Search,
    Layers,
    Syringe,
    Droplet,
    FirstAid,
}

pub fn draw_icon(ui: &mut Ui, kind: IconKind, size: f32, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::hover());
    paint_icon(ui.painter(), rect, kind, color);
}

pub fn paint_icon(painter: &egui::Painter, rect: Rect, kind: IconKind, color: Color32) {
    let s = rect.width().min(rect.height());
    let stroke = Stroke::new((s * 0.095).clamp(1.25, 2.2), color);
    let p = |x: f32, y: f32| -> Pos2 {
        Pos2::new(rect.left() + x * s, rect.top() + y * s)
    };

    match kind {
        IconKind::Home => {
            painter.line_segment([p(0.15, 0.45), p(0.5, 0.15)], stroke);
            painter.line_segment([p(0.5, 0.15), p(0.85, 0.45)], stroke);
            painter.line_segment([p(0.22, 0.42), p(0.22, 0.85)], stroke);
            painter.line_segment([p(0.78, 0.42), p(0.78, 0.85)], stroke);
            painter.line_segment([p(0.22, 0.85), p(0.78, 0.85)], stroke);
            painter.rect_stroke(
                Rect::from_min_max(p(0.41, 0.56), p(0.59, 0.85)),
                1.0,
                stroke,
            );
        }
        IconKind::Dashboard => {
            for (x0, y0, x1, y1) in [
                (0.15, 0.15, 0.45, 0.48),
                (0.55, 0.15, 0.85, 0.36),
                (0.55, 0.46, 0.85, 0.85),
                (0.15, 0.58, 0.45, 0.85),
            ] {
                painter.rect_stroke(Rect::from_min_max(p(x0, y0), p(x1, y1)), 2.0, stroke);
            }
        }
        IconKind::Package => {
            painter.rect_stroke(Rect::from_min_max(p(0.18, 0.25), p(0.82, 0.82)), 2.0, stroke);
            painter.line_segment([p(0.18, 0.42), p(0.82, 0.42)], stroke);
            painter.line_segment([p(0.5, 0.25), p(0.5, 0.82)], stroke);
        }
        IconKind::ShoppingCart => {
            painter.line_segment([p(0.12, 0.2), p(0.25, 0.2)], stroke);
            painter.line_segment([p(0.25, 0.2), p(0.35, 0.65)], stroke);
            painter.line_segment([p(0.35, 0.65), p(0.8, 0.65)], stroke);
            painter.line_segment([p(0.8, 0.65), p(0.88, 0.32)], stroke);
            painter.line_segment([p(0.28, 0.32), p(0.88, 0.32)], stroke);
            painter.circle_filled(p(0.38, 0.8), s * 0.06, color);
            painter.circle_filled(p(0.75, 0.8), s * 0.06, color);
        }
        IconKind::AlertTriangle => {
            painter.line_segment([p(0.5, 0.16), p(0.14, 0.82)], stroke);
            painter.line_segment([p(0.14, 0.82), p(0.86, 0.82)], stroke);
            painter.line_segment([p(0.86, 0.82), p(0.5, 0.16)], stroke);
            painter.line_segment([p(0.5, 0.42), p(0.5, 0.62)], stroke);
            painter.circle_filled(p(0.5, 0.73), s * 0.045, color);
        }
        IconKind::Wrench => {
            painter.circle_stroke(p(0.68, 0.32), s * 0.18, stroke);
            painter.line_segment([p(0.54, 0.46), p(0.22, 0.78)], stroke);
            painter.line_segment([p(0.18, 0.74), p(0.26, 0.82)], stroke);
        }
        IconKind::Calendar => {
            painter.rect_stroke(Rect::from_min_max(p(0.16, 0.24), p(0.84, 0.84)), 2.0, stroke);
            painter.line_segment([p(0.16, 0.42), p(0.84, 0.42)], stroke);
            painter.line_segment([p(0.34, 0.14), p(0.34, 0.30)], stroke);
            painter.line_segment([p(0.66, 0.14), p(0.66, 0.30)], stroke);
        }
        IconKind::Upload => {
            painter.line_segment([p(0.5, 0.65), p(0.5, 0.18)], stroke);
            painter.line_segment([p(0.5, 0.18), p(0.32, 0.36)], stroke);
            painter.line_segment([p(0.5, 0.18), p(0.68, 0.36)], stroke);
            painter.line_segment([p(0.18, 0.82), p(0.82, 0.82)], stroke);
        }
        IconKind::Save => {
            painter.rect_stroke(Rect::from_min_max(p(0.18, 0.18), p(0.82, 0.82)), 2.0, stroke);
            painter.rect_stroke(Rect::from_min_max(p(0.32, 0.18), p(0.68, 0.44)), 1.0, stroke);
            painter.rect_stroke(Rect::from_min_max(p(0.30, 0.56), p(0.70, 0.82)), 1.0, stroke);
        }
        IconKind::Refresh => {
            painter.circle_stroke(p(0.5, 0.5), s * 0.30, stroke);
            painter.line_segment([p(0.76, 0.28), p(0.82, 0.44)], stroke);
            painter.line_segment([p(0.82, 0.44), p(0.66, 0.44)], stroke);
        }
        IconKind::Plus => {
            painter.line_segment([p(0.5, 0.2), p(0.5, 0.8)], stroke);
            painter.line_segment([p(0.2, 0.5), p(0.8, 0.5)], stroke);
        }
        IconKind::Activity => {
            painter.line_segment([p(0.12, 0.52), p(0.30, 0.52)], stroke);
            painter.line_segment([p(0.30, 0.52), p(0.42, 0.22)], stroke);
            painter.line_segment([p(0.42, 0.22), p(0.58, 0.78)], stroke);
            painter.line_segment([p(0.58, 0.78), p(0.70, 0.52)], stroke);
            painter.line_segment([p(0.70, 0.52), p(0.88, 0.52)], stroke);
        }
        IconKind::CheckCircle => {
            painter.circle_stroke(p(0.5, 0.5), s * 0.34, stroke);
            painter.line_segment([p(0.34, 0.52), p(0.45, 0.64)], stroke);
            painter.line_segment([p(0.45, 0.64), p(0.68, 0.38)], stroke);
        }
        IconKind::FileText => {
            painter.rect_stroke(Rect::from_min_max(p(0.22, 0.14), p(0.78, 0.86)), 2.0, stroke);
            painter.line_segment([p(0.34, 0.38), p(0.66, 0.38)], stroke);
            painter.line_segment([p(0.34, 0.54), p(0.66, 0.54)], stroke);
            painter.line_segment([p(0.34, 0.70), p(0.54, 0.70)], stroke);
        }
        IconKind::Search => {
            painter.circle_stroke(p(0.44, 0.44), s * 0.24, stroke);
            painter.line_segment([p(0.62, 0.62), p(0.82, 0.82)], stroke);
        }
        IconKind::Layers => {
            painter.line_segment([p(0.16, 0.36), p(0.5, 0.18)], stroke);
            painter.line_segment([p(0.5, 0.18), p(0.84, 0.36)], stroke);
            painter.line_segment([p(0.84, 0.36), p(0.5, 0.54)], stroke);
            painter.line_segment([p(0.5, 0.54), p(0.16, 0.36)], stroke);
            painter.line_segment([p(0.16, 0.62), p(0.5, 0.80)], stroke);
            painter.line_segment([p(0.5, 0.80), p(0.84, 0.62)], stroke);
        }
        IconKind::Syringe => {
            painter.line_segment([p(0.18, 0.82), p(0.32, 0.68)], stroke); // needle
            painter.line_segment([p(0.32, 0.68), p(0.68, 0.32)], stroke); // barrel
            painter.line_segment([p(0.68, 0.32), p(0.82, 0.18)], stroke); // plunger
            painter.line_segment([p(0.74, 0.14), p(0.86, 0.26)], stroke); // thumb rest
            painter.line_segment([p(0.42, 0.42), p(0.58, 0.58)], stroke); // collar
        }
        IconKind::Droplet => {
            painter.line_segment([p(0.5, 0.15), p(0.24, 0.56)], stroke);
            painter.line_segment([p(0.5, 0.15), p(0.76, 0.56)], stroke);
            painter.circle_stroke(p(0.5, 0.60), s * 0.26, stroke);
        }
        IconKind::FirstAid => {
            painter.rect_stroke(Rect::from_min_max(p(0.16, 0.26), p(0.84, 0.82)), 2.0, stroke);
            painter.line_segment([p(0.36, 0.26), p(0.36, 0.16)], stroke);
            painter.line_segment([p(0.64, 0.26), p(0.64, 0.16)], stroke);
            painter.line_segment([p(0.36, 0.16), p(0.64, 0.16)], stroke);
            painter.line_segment([p(0.5, 0.40), p(0.5, 0.68)], stroke);
            painter.line_segment([p(0.36, 0.54), p(0.64, 0.54)], stroke);
        }
    }
}
