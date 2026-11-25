use crate::ui::icons::{draw_icon, IconKind};
use egui::{Color32, Frame, Margin, RichText, Stroke, Ui};

// Healthcare SaaS Color Palette
pub const BG_ICE_BLUE: Color32 = Color32::from_rgb(0xF4, 0xF7, 0xFB); // Soft neutral blue-gray
pub const CARD_WHITE: Color32 = Color32::from_rgb(0xFF, 0xFF, 0xFF);
pub const SIDEBAR_BG: Color32 = Color32::from_rgb(0xFF, 0xFF, 0xFF);
pub const ACCENT_SKY: Color32 = Color32::from_rgb(0xDB, 0xEA, 0xFE); // Soft active pill tint
pub const ACCENT_BLUE: Color32 = Color32::from_rgb(0x25, 0x63, 0xEB); // Professional healthcare blue
pub const TEXT_SLATE: Color32 = Color32::from_rgb(0x0F, 0x17, 0x2A); // Dark navy/charcoal
pub const TEXT_MUTED: Color32 = Color32::from_rgb(0x64, 0x74, 0x8B);
pub const BORDER_SOFT: Color32 = Color32::from_rgb(0xE2, 0xE8, 0xF0); // Subtle neutral border

pub const BADGE_RED_BG: Color32 = Color32::from_rgb(0xFE, 0xE2, 0xE2);
pub const BADGE_RED_FG: Color32 = Color32::from_rgb(0xB9, 0x1C, 0x1C);
pub const BADGE_AMBER_BG: Color32 = Color32::from_rgb(0xFE, 0xF3, 0xC7);
pub const BADGE_AMBER_FG: Color32 = Color32::from_rgb(0xB4, 0x53, 0x09);
pub const BADGE_GREEN_BG: Color32 = Color32::from_rgb(0xDC, 0xFC, 0xE7);
pub const BADGE_GREEN_FG: Color32 = Color32::from_rgb(0x15, 0x80, 0x3D);

// Design Tokens
pub const SPACE_XS: f32 = 4.0;
pub const SPACE_SM: f32 = 8.0;
pub const SPACE_MD: f32 = 16.0;
pub const SPACE_LG: f32 = 24.0;
pub const RADIUS_SM: f32 = 8.0;
pub const RADIUS_MD: f32 = 12.0;

pub fn apply_hospital_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::light();
    visuals.panel_fill = BG_ICE_BLUE;
    visuals.window_fill = CARD_WHITE;
    visuals.widgets.noninteractive.bg_fill = CARD_WHITE;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, TEXT_SLATE);
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(0xF8, 0xFA, 0xFC);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, BORDER_SOFT);
    visuals.widgets.hovered.bg_fill = ACCENT_SKY;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, ACCENT_BLUE);
    visuals.widgets.active.bg_fill = ACCENT_BLUE;
    visuals.selection.bg_fill = ACCENT_SKY;
    visuals.selection.stroke = Stroke::new(1.0_f32, ACCENT_BLUE);
    ctx.set_visuals(visuals);
}

pub fn card_frame() -> Frame {
    Frame::none()
        .fill(Color32::from_rgba_unmultiplied(0xFF, 0xFF, 0xFF, 205))
        .stroke(Stroke::new(1.0_f32, BORDER_SOFT))
        .rounding(egui::Rounding::same(RADIUS_MD))
        .inner_margin(Margin::same(SPACE_MD))
}

pub fn page_header(ui: &mut Ui, icon: IconKind, title: &str, subtitle: &str) {
    ui.horizontal(|ui| {
        draw_icon(ui, icon, 22.0, ACCENT_BLUE);
        ui.add_space(6.0);
        ui.vertical(|ui| {
            ui.label(
                RichText::new(title)
                    .size(20.0)
                    .color(TEXT_SLATE)
                    .strong(),
            );
            ui.label(RichText::new(subtitle).size(12.5).color(TEXT_MUTED));
        });
    });
}

pub fn badge(ui: &mut Ui, label: &str, bg: Color32, fg: Color32) {
    Frame::none()
        .fill(bg)
        .rounding(egui::Rounding::same(6.0))
        .inner_margin(Margin::symmetric(8.0, 3.0_f32))
        .show(ui, |ui| {
            ui.label(RichText::new(label).color(fg).size(11.5).strong());
        });
}

pub fn empty_state(ui: &mut Ui, icon: IconKind, title: &str, hint: &str) {
    Frame::none()
        .fill(Color32::from_rgb(0xF8, 0xFA, 0xFC))
        .stroke(Stroke::new(1.0_f32, BORDER_SOFT))
        .rounding(egui::Rounding::same(RADIUS_SM))
        .inner_margin(Margin::same(SPACE_LG))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                draw_icon(ui, icon, 28.0, TEXT_MUTED);
                ui.add_space(SPACE_SM);
                ui.label(RichText::new(title).color(TEXT_SLATE).strong().size(14.0));
                ui.add_space(SPACE_XS);
                ui.label(RichText::new(hint).color(TEXT_MUTED).size(12.0));
            });
        });
}
