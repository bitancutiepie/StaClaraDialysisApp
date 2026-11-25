mod excel_sync;
mod models;
mod ui;

use eframe::egui;
use models::{AppState, NavView};
use ui::icons::{draw_icon, IconKind};
use ui::theme::*;

const LOGO_BYTES: &[u8] = include_bytes!("../assets/logo.png");
const BG_IMAGE_BYTES: &[u8] = include_bytes!("../assets/background.jpg");
const MASCOT_HOLDING_BYTES: &[u8] = include_bytes!("../assets/mascot_holding.png");
const MASCOT_EXCITED_BYTES: &[u8] = include_bytes!("../assets/mascot_excited.png");

fn load_png_rgba(bytes: &[u8]) -> Option<(Vec<u8>, u32, u32)> {
    let img = image::load_from_memory(bytes).ok()?.into_rgba8();
    let (w, h) = img.dimensions();
    Some((img.into_raw(), w, h))
}

struct DialysisApp {
    state: AppState,
    theme_applied: bool,
    ui_debug: bool,
    logo_tex: Option<egui::TextureHandle>,
    mascot_holding_tex: Option<egui::TextureHandle>,
    mascot_excited_tex: Option<egui::TextureHandle>,
    bg_tex: Option<egui::TextureHandle>,
    screenshot_step: Option<usize>,
    wait_frames: u8,
    last_view: NavView,
    transition_start: Option<f64>,
}

impl Default for DialysisApp {
    fn default() -> Self {
        let capture = std::env::args().any(|a| a == "--screenshots");
        let state = AppState::new_demo();
        let last_view = state.active_view;
        Self {
            state,
            theme_applied: false,
            ui_debug: false,
            logo_tex: None,
            mascot_holding_tex: None,
            mascot_excited_tex: None,
            bg_tex: None,
            screenshot_step: if capture { Some(0) } else { None },
            wait_frames: 0,
            last_view,
            transition_start: None,
        }
    }
}

impl eframe::App for DialysisApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_screenshots(ctx);
        self.render_ui(ctx);
    }
}

impl DialysisApp {
    fn handle_screenshots(&mut self, ctx: &egui::Context) {
        let Some(step) = self.screenshot_step else {
            return;
        };
        let steps: &[(NavView, bool, &str)] = &[
            (NavView::Home, false, "01_home.png"),
            (NavView::Dashboard, false, "02_dashboard.png"),
            (NavView::Requisition, false, "03_kiosk_order.png"),
            (NavView::StockIn, false, "04_stock_in.png"),
            (NavView::Adjustment, false, "05_wastage.png"),
            (NavView::TechLogs, false, "06_tech_logs.png"),
            (NavView::MonthlyConsumption, false, "07_monthly_usage.png"),
            (NavView::Dashboard, true, "08_excel_modal.png"),
        ];

        // Check if a screenshot event arrived this frame
        let mut saved = false;
        ctx.input(|i| {
            for ev in &i.events {
                if let egui::Event::Screenshot { image, .. } = ev {
                    if let Some((_, _, fname)) = steps.get(step) {
                        let _ = std::fs::create_dir_all("screenshots");
                        let path = format!("screenshots/{}", fname);
                        let [w, h] = image.size;
                        let rgba: Vec<u8> = image
                            .pixels
                            .iter()
                            .flat_map(|c| c.to_array())
                            .collect();
                        let _ = image::save_buffer(
                            &path,
                            &rgba,
                            w as u32,
                            h as u32,
                            image::ColorType::Rgba8,
                        );
                        saved = true;
                    }
                }
            }
        });

        if saved {
            let next = step + 1;
            if next >= steps.len() {
                self.screenshot_step = None;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            } else {
                self.screenshot_step = Some(next);
                self.wait_frames = 3;
            }
            return;
        }

        if let Some(&(view, modal, _)) = steps.get(step) {
            self.state.active_view = view;
            self.state.excel_modal_open = modal;
            if self.wait_frames == 0 {
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot);
                self.wait_frames = 20;
            } else {
                self.wait_frames -= 1;
            }
            ctx.request_repaint();
        }
    }

    fn render_ui(&mut self, ctx: &egui::Context) {
        if !self.theme_applied {
            apply_hospital_theme(ctx);
            self.theme_applied = true;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::F12)) {
            self.ui_debug = !self.ui_debug;
        }
        ctx.set_debug_on_hover(self.ui_debug);
        if self.logo_tex.is_none() {
            if let Some((rgba, w, h)) = load_png_rgba(LOGO_BYTES) {
                let color_img =
                    egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &rgba);
                self.logo_tex = Some(ctx.load_texture(
                    "clinic_logo",
                    color_img,
                    egui::TextureOptions::LINEAR,
                ));
            }
        }
        if self.mascot_holding_tex.is_none() {
            if let Some((rgba, w, h)) = load_png_rgba(MASCOT_HOLDING_BYTES) {
                let color_img =
                    egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &rgba);
                self.mascot_holding_tex = Some(ctx.load_texture(
                    "clinic_mascot_holding",
                    color_img,
                    egui::TextureOptions::LINEAR,
                ));
            }
        }
        if self.bg_tex.is_none() {
            if let Some((rgba, w, h)) = load_png_rgba(BG_IMAGE_BYTES) {
                let color_img =
                    egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &rgba);
                self.bg_tex = Some(ctx.load_texture(
                    "clinic_bg",
                    color_img,
                    egui::TextureOptions::LINEAR,
                ));
            }
        }
        if self.mascot_excited_tex.is_none() {
            if let Some((rgba, w, h)) = load_png_rgba(MASCOT_EXCITED_BYTES) {
                let color_img =
                    egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &rgba);
                self.mascot_excited_tex = Some(ctx.load_texture(
                    "clinic_mascot_excited",
                    color_img,
                    egui::TextureOptions::LINEAR,
                ));
            }
        }

        // Keyboard shortcuts: Esc closes modal/banner; 1-7 switches views when not typing
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            if self.state.pending_confirm.is_some() {
                self.state.pending_confirm = None;
            } else if self.state.excel_modal_open {
                self.state.excel_modal_open = false;
            } else if self.state.report_modal_open {
                self.state.report_modal_open = false;
            } else {
                self.state.banner = None;
            }
        }
        if !ctx.wants_keyboard_input() {
            ctx.input(|i| {
                if i.key_pressed(egui::Key::Num1) {
                    self.state.active_view = NavView::Home;
                } else if i.key_pressed(egui::Key::Num2) {
                    self.state.active_view = NavView::Dashboard;
                } else if i.key_pressed(egui::Key::Num3) {
                    self.state.active_view = NavView::Requisition;
                } else if i.key_pressed(egui::Key::Num4) {
                    self.state.active_view = NavView::StockIn;
                } else if i.key_pressed(egui::Key::Num5) {
                    self.state.active_view = NavView::Adjustment;
                } else if i.key_pressed(egui::Key::Num6) {
                    self.state.active_view = NavView::TechLogs;
                } else if i.key_pressed(egui::Key::Num7) {
                    self.state.active_view = NavView::MonthlyConsumption;
                }
            });
        }

        let screen_w = ctx.screen_rect().width();
        let is_desktop = screen_w >= 820.0;

        let nav_items = [
            (NavView::Home, IconKind::Home, "Home", "Home"),
            (NavView::Dashboard, IconKind::Dashboard, "Dashboard", "Dash"),
            (NavView::Requisition, IconKind::ShoppingCart, "Kiosk Order", "Order"),
            (NavView::StockIn, IconKind::Package, "Stock-In", "Stock"),
            (NavView::Adjustment, IconKind::AlertTriangle, "Wastage", "Waste"),
            (NavView::TechLogs, IconKind::Wrench, "Tech Logs", "Tech"),
            (NavView::MonthlyConsumption, IconKind::Calendar, "Monthly Usage", "Month"),
        ];

        // Compact Top Header Bar (Centered Brand + Right Actions)
        egui::TopBottomPanel::top("top_header")
            .frame(
                egui::Frame::none()
                    .fill(CARD_WHITE)
                    .stroke(egui::Stroke::new(1.0_f32, BORDER_SOFT))
                    .inner_margin(egui::Margin::symmetric(16.0, 10.0)),
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            if let Some(tex) = &self.logo_tex {
                                ui.add(
                                    egui::Image::new(tex)
                                        .fit_to_exact_size(egui::vec2(34.0, 34.0)),
                                );
                            } else {
                                draw_icon(ui, IconKind::Activity, 22.0, ACCENT_BLUE);
                            }
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new("STA. CLARA DIALYSIS CENTER, INC.")
                                    .size(16.0)
                                    .strong()
                                    .color(TEXT_SLATE),
                            );
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .selectable_label(self.ui_debug, "Debug UI (F12)")
                                .clicked()
                            {
                                self.ui_debug = !self.ui_debug;
                            }
                            if ui.small_button("Reset").clicked() {
                                self.state.pending_confirm = Some(models::PendingConfirm {
                                    title: "Confirm Reset Demo Data".to_string(),
                                    summary: "This will reset all inventory, logs, and cart items back to initial demo values.".to_string(),
                                    action: models::ConfirmAction::ResetDemo,
                                });
                            }
                            if ui
                                .add(
                                    egui::Button::new(
                                        egui::RichText::new("Save .xlsx")
                                            .color(egui::Color32::WHITE)
                                            .size(12.0)
                                            .strong(),
                                    )
                                    .fill(ACCENT_BLUE)
                                    .rounding(egui::Rounding::same(RADIUS_SM)),
                                )
                                .clicked()
                            {
                                self.state.pending_confirm = Some(models::PendingConfirm {
                                    title: "Confirm Save to Excel".to_string(),
                                    summary: format!(
                                        "Save current inventory & daily usage to {}?",
                                        excel_sync::DEFAULT_XLSX_PATH
                                    ),
                                    action: models::ConfirmAction::SaveXlsx,
                                });
                            }
                            if ui.small_button("Excel View").clicked() {
                                self.state.excel_modal_open = true;
                            }
                            if ui.small_button("Load .xlsx").clicked() {
                                match excel_sync::load_from_xlsx(
                                    excel_sync::DEFAULT_XLSX_PATH,
                                    &mut self.state,
                                ) {
                                    Ok(n) => self.state.set_banner(
                                        format!("Loaded {} items from Excel.", n),
                                        false,
                                    ),
                                    Err(e) => self.state.set_banner(e, true),
                                }
                            }
                        });
                    });
                });
            });

        // Adaptive Navigation: Sidebar on Desktop (>= 820px), Bottom Bar on Mobile/Kiosk (< 820px)
        if is_desktop {
            egui::SidePanel::left("desktop_sidebar")
                .exact_width(216.0)
                .frame(
                    egui::Frame::none()
                        .fill(SIDEBAR_BG)
                        .inner_margin(egui::Margin {
                            left: 12.0,
                            right: 0.0,
                            top: 12.0,
                            bottom: 12.0,
                        }),
                )
                .show(ctx, |ui| {
                    ui.style_mut().interaction.selectable_labels = false;
                    ui.label(
                        egui::RichText::new("NAVIGATION")
                            .size(11.0)
                            .color(TEXT_MUTED)
                            .strong(),
                    );
                    ui.add_space(8.0);
                    let sidebar_right = ui.max_rect().right();
                    for (view, icon, label, _) in &nav_items {
                        let selected = self.state.active_view == *view;
                        let nav_id = ui.make_persistent_id(("sidebar_nav", *label));
                        let row_w = ui.available_width();
                        let row_rect = egui::Rect::from_min_size(
                            ui.cursor().min,
                            egui::vec2(row_w, 38.0),
                        );
                        let hovered = ui.rect_contains_pointer(row_rect);
                        let hover_t = ctx.animate_bool_with_time(nav_id, hovered, 0.14);

                        let fg = if selected || hover_t > 0.4 {
                            ACCENT_BLUE
                        } else {
                            TEXT_SLATE
                        };

                        if selected {
                            // Inverted border-radius (concave outer curves) connecting flush to CentralPanel
                            let full_rect = egui::Rect::from_min_max(
                                row_rect.min,
                                egui::pos2(sidebar_right, row_rect.max.y),
                            );
                            let r = 14.0_f32;
                            ui.painter().rect_filled(
                                full_rect,
                                egui::Rounding {
                                    nw: r,
                                    sw: r,
                                    ne: 0.0,
                                    se: 0.0,
                                },
                                BG_ICE_BLUE,
                            );
                            // Top-right concave corner
                            let tr_sq = egui::Rect::from_min_size(
                                egui::pos2(sidebar_right - r, full_rect.top() - r),
                                egui::vec2(r, r),
                            );
                            ui.painter().rect_filled(tr_sq, 0.0, BG_ICE_BLUE);
                            ui.painter().circle_filled(
                                egui::pos2(sidebar_right - r, full_rect.top() - r),
                                r,
                                SIDEBAR_BG,
                            );
                            // Bottom-right concave corner
                            let br_sq = egui::Rect::from_min_size(
                                egui::pos2(sidebar_right - r, full_rect.bottom()),
                                egui::vec2(r, r),
                            );
                            ui.painter().rect_filled(br_sq, 0.0, BG_ICE_BLUE);
                            ui.painter().circle_filled(
                                egui::pos2(sidebar_right - r, full_rect.bottom() + r),
                                r,
                                SIDEBAR_BG,
                            );
                        } else if hover_t > 0.01 {
                            let hover_fill = egui::Color32::from_rgba_unmultiplied(
                                0xDB,
                                0xEA,
                                0xFE,
                                (hover_t * 160.0) as u8,
                            );
                            ui.painter().rect_filled(
                                row_rect.shrink2(egui::vec2(4.0, 1.0)),
                                egui::Rounding::same(RADIUS_SM),
                                hover_fill,
                            );
                        }

                        let resp = ui.interact(row_rect, nav_id, egui::Sense::click());
                        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(row_rect), |ui| {
                            ui.add_space(9.0);
                            ui.horizontal(|ui| {
                                ui.add_space(12.0 + if selected { 2.0 } else { hover_t * 4.0 });
                                draw_icon(ui, *icon, 16.0, fg);
                                ui.add_space(6.0);
                                ui.label(
                                    egui::RichText::new(*label)
                                        .color(fg)
                                        .size(13.5)
                                        .strong(),
                                );
                            });
                        });
                        if resp.hovered() {
                            ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                        if resp.clicked() {
                            self.state.active_view = *view;
                        }
                        ui.add_space(4.0);
                    }
                });
        } else {
            egui::TopBottomPanel::bottom("mobile_bottom_nav")
                .frame(
                    egui::Frame::none()
                        .fill(CARD_WHITE)
                        .stroke(egui::Stroke::new(1.0_f32, BORDER_SOFT))
                        .inner_margin(egui::Margin::symmetric(6.0, 6.0)),
                )
                .show(ctx, |ui| {
                    ui.columns(nav_items.len(), |cols| {
                        for (i, (view, icon, _, short_label)) in nav_items.iter().enumerate() {
                            let selected = self.state.active_view == *view;
                            let fill = if selected { ACCENT_SKY } else { egui::Color32::TRANSPARENT };
                            let fg = if selected { ACCENT_BLUE } else { TEXT_MUTED };
                            let resp = egui::Frame::none()
                                .fill(fill)
                                .rounding(egui::Rounding::same(RADIUS_SM))
                                .inner_margin(egui::Margin::symmetric(2.0, 6.0))
                                .show(&mut cols[i], |ui| {
                                    ui.vertical_centered(|ui| {
                                        draw_icon(ui, *icon, 18.0, fg);
                                        ui.add_space(2.0);
                                        ui.label(
                                            egui::RichText::new(*short_label)
                                                .color(fg)
                                                .size(11.0)
                                                .strong(),
                                        );
                                    });
                                })
                                .response
                                .interact(egui::Sense::click());
                            if resp.clicked() {
                                self.state.active_view = *view;
                            }
                        }
                    });
                });
        }

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(BG_ICE_BLUE)
                    .inner_margin(egui::Margin::same(18.0)),
            )
            .show(ctx, |ui| {
                // Static background.jpg + very light veil so it stays clearly visible
                if let Some(bg) = &self.bg_tex {
                    let panel_rect = ui.max_rect();
                    ui.painter().image(
                        bg.id(),
                        panel_rect,
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                    ui.painter().rect_filled(
                        panel_rect,
                        0.0,
                        egui::Color32::from_rgba_unmultiplied(0xF4, 0xF7, 0xFB, 45),
                    );
                }
                if let Some(banner) = self.state.banner.clone() {
                    let (bg, fg) = if banner.is_error {
                        (BADGE_RED_BG, BADGE_RED_FG)
                    } else {
                        (BADGE_GREEN_BG, BADGE_GREEN_FG)
                    };
                    egui::Frame::none()
                        .fill(bg)
                        .rounding(egui::Rounding::same(RADIUS_SM))
                        .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(&banner.text).color(fg).strong());
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if ui.small_button("Dismiss").clicked() {
                                            self.state.banner = None;
                                        }
                                    },
                                );
                            });
                        });
                    ui.add_space(10.0);
                }

                // Dramatic Slide + Fade Page Transition
                let now = ctx.input(|i| i.time);
                if self.state.active_view != self.last_view {
                    self.last_view = self.state.active_view;
                    self.transition_start = Some(now);
                }
                let mut slide_x = 0.0_f32;
                let mut opacity = 1.0_f32;
                if let Some(start) = self.transition_start {
                    let raw = ((now - start) / 0.28).clamp(0.0, 1.0) as f32;
                    let eased = 1.0 - (1.0 - raw).powi(3); // Cubic-out
                    slide_x = (1.0 - eased) * 48.0;
                    opacity = 0.25 + 0.75 * eased;
                    if raw < 1.0 {
                        ctx.request_repaint();
                    } else {
                        self.transition_start = None;
                    }
                }
                ui.set_opacity(opacity);
                if slide_x > 0.1 {
                    ui.add_space(2.0);
                }
                let content_rect = ui.available_rect_before_wrap();
                let shifted_rect = content_rect.translate(egui::vec2(slide_x, 0.0));
                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(shifted_rect), |ui| {
                if self.state.active_view == NavView::Home {
                    show_home_launcher(
                        ui,
                        &mut self.state,
                        self.logo_tex.as_ref(),
                        self.mascot_holding_tex.as_ref(),
                        self.mascot_excited_tex.as_ref(),
                    );
                } else {
                    egui::ScrollArea::vertical()
                        .id_salt("main_content_scroll")
                        .show(ui, |ui| {
                            let max_w = ui.available_width().min(1440.0);
                            ui.vertical_centered(|ui| {
                                ui.set_max_width(max_w);
                                match self.state.active_view {
                                    NavView::Home => {}
                                    NavView::Dashboard => {
                                        ui::dashboard::show_dashboard(ui, &mut self.state, ctx)
                                    }
                                    NavView::StockIn => {
                                        ui::stock_in::show_stock_in(ui, &mut self.state)
                                    }
                                    NavView::Requisition => {
                                        ui::requisition::show_requisition(ui, &mut self.state)
                                    }
                                    NavView::Adjustment => {
                                        ui::adjustment::show_adjustment(ui, &mut self.state)
                                    }
                                    NavView::TechLogs => {
                                        ui::tech_logs::show_tech_logs(ui, &mut self.state)
                                    }
                                    NavView::MonthlyConsumption => {
                                        ui::monthly_consumption::show_monthly_consumption(
                                            ui,
                                            &mut self.state,
                                        )
                                    }
                                }
                            });
                        });
                }
                });

                if self.state.excel_modal_open {
                    excel_sync::show_excel_modal(ctx, &mut self.state);
                }

                show_confirmation_modal(ctx, &mut self.state);

                if self.ui_debug {
                    let screen = ctx.screen_rect();
                    let pointer_pos = ctx
                        .pointer_hover_pos()
                        .map(|p| format!("{:.0}, {:.0}", p.x, p.y))
                        .unwrap_or_else(|| "-".to_string());
                    if ctx.input(|i| i.pointer.button_double_clicked(egui::PointerButton::Primary))
                    {
                        let tag = format!(
                            "[UI_DEBUG] Screen: {:?} | Size: {:.0}x{:.0} | Pos: ({})",
                            self.state.active_view,
                            screen.width(),
                            screen.height(),
                            pointer_pos
                        );
                        ctx.copy_text(tag.clone());
                        self.state.set_banner(format!("Copied: {}", tag), false);
                    }
                    egui::Area::new(egui::Id::new("debug_component_hud"))
                        .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-12.0, -12.0))
                        .show(ctx, |ui| {
                            egui::Frame::none()
                                .fill(egui::Color32::from_rgba_unmultiplied(15, 23, 42, 220))
                                .rounding(egui::Rounding::same(8.0))
                                .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                                .show(ui, |ui| {
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Screen: {:?} | Size: {:.0}x{:.0} | Pointer: ({})",
                                            self.state.active_view,
                                            screen.width(),
                                            screen.height(),
                                            pointer_pos
                                        ))
                                        .color(egui::Color32::WHITE)
                                        .size(11.5)
                                        .monospace(),
                                    );
                                });
                        });
                }
            });
    }
}

fn show_home_launcher(
    ui: &mut egui::Ui,
    state: &mut AppState,
    logo_tex: Option<&egui::TextureHandle>,
    mascot_holding_tex: Option<&egui::TextureHandle>,
    mascot_excited_tex: Option<&egui::TextureHandle>,
) {
    ui.vertical_centered(|ui| {
        if let Some(tex) = logo_tex {
            ui.add(egui::Image::new(tex).fit_to_exact_size(egui::vec2(96.0, 96.0)));
        }
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new("STA. CLARA DIALYSIS CENTER, INC.")
                .size(24.0)
                .strong()
                .color(TEXT_SLATE),
        );
        ui.add_space(2.0);
        ui.label(
            egui::RichText::new("INVENTORY & DAILY OPERATIONS WORKSPACE")
                .size(12.5)
                .strong()
                .color(ACCENT_BLUE),
        );
    });
    ui.add_space(16.0);

    let tiles = [
        (NavView::Dashboard, IconKind::Dashboard, "Dashboard", "Alerts, stock & daily report"),
        (NavView::Requisition, IconKind::ShoppingCart, "Kiosk Order", "1-click treatment packages"),
        (NavView::StockIn, IconKind::Package, "Stock-In", "Helande Kidney Care deliveries"),
        (NavView::Adjustment, IconKind::AlertTriangle, "Wastage", "Damaged, expired & borrowed"),
        (NavView::TechLogs, IconKind::Wrench, "Tech Logs", "Dialyzer reuse & RO water"),
        (NavView::MonthlyConsumption, IconKind::Calendar, "Monthly Usage", "Department & daily usage"),
    ];

    let avail_w = ui.available_width();
    let avail_h = ui.available_height().max(260.0);
    let cols = if avail_w >= 720.0 { 3 } else { 2 };
    let rows = (tiles.len() + cols - 1) / cols;
    let spacing = 14.0;
    let tile_w = ((avail_w - spacing * (cols as f32 - 1.0)) / cols as f32).max(140.0);
    let tile_h = ((avail_h - spacing * (rows as f32 - 1.0)) / rows as f32).max(120.0);

    let grid_top_left = ui.cursor().min;
    let mut hovered_tile_rect: Option<egui::Rect> = None;

    egui::Grid::new("home_launcher_grid")
        .spacing([spacing, spacing])
        .show(ui, |ui| {
            for (i, (view, icon, title, sub)) in tiles.iter().enumerate() {
                let inner_w = (tile_w - 32.0).max(80.0);
                let inner_h = (tile_h - 32.0).max(80.0);
                let tile_id = ui.make_persistent_id(("home_tile", *title));
                let est_rect =
                    egui::Rect::from_min_size(ui.cursor().min, egui::vec2(tile_w, tile_h));
                let is_hovered = ui.rect_contains_pointer(est_rect);
                let hover_t = ui.ctx().animate_bool_with_time(tile_id, is_hovered, 0.16);
                let bg = egui::Color32::from_rgba_unmultiplied(
                    255,
                    255,
                    255,
                    (140.0 + hover_t * 64.0) as u8, // 0.55 -> 0.80
                );
                let border_col = if hover_t > 0.2 {
                    ACCENT_BLUE
                } else {
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 179) // 0.70 luminous border
                };
                let resp = egui::Frame::none()
                    .fill(bg)
                    .stroke(egui::Stroke::new(1.0 + hover_t * 0.8, border_col))
                    .rounding(egui::Rounding::same(RADIUS_MD))
                    .shadow(egui::epaint::Shadow {
                        offset: egui::vec2(0.0, 8.0 - hover_t * 2.0),
                        blur: 32.0,
                        spread: 0.0,
                        color: egui::Color32::from_rgba_unmultiplied(
                            31,
                            38,
                            135,
                            (16.0 + hover_t * 20.0) as u8, // rgba(31, 38, 135, 0.06+)
                        ),
                    })
                    .inner_margin(egui::Margin::same(16.0))
                    .show(ui, |ui| {
                        ui.set_min_size(egui::vec2(inner_w, inner_h));
                        ui.set_max_size(egui::vec2(inner_w, inner_h));
                        ui.vertical_centered(|ui| {
                            ui.add_space(((inner_h * 0.22) - hover_t * 2.0).max(2.0));
                            draw_icon(ui, *icon, 36.0 + hover_t * 4.0, ACCENT_BLUE);
                            ui.add_space(12.0);
                            ui.label(
                                egui::RichText::new(*title)
                                    .size(17.0)
                                    .strong()
                                    .color(if hover_t > 0.4 { ACCENT_BLUE } else { TEXT_SLATE }),
                            );
                            ui.add_space(4.0);
                            ui.label(egui::RichText::new(*sub).size(13.0).color(TEXT_MUTED));
                        });
                    })
                    .response
                    .interact(egui::Sense::click());
                if resp.hovered() {
                    hovered_tile_rect = Some(resp.rect);
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if resp.clicked() {
                    state.active_view = *view;
                }
                if (i + 1) % cols == 0 {
                    ui.end_row();
                }
            }
        });

    // Interactive Mascot: glides smoothly to whichever Home button is hovered!
    let any_hovered = hovered_tile_rect.is_some();
    let react_t = ui
        .ctx()
        .animate_bool_with_time(ui.make_persistent_id("mascot_react"), any_hovered, 0.16);
    let (target_x, target_y) = if let Some(r) = hovered_tile_rect {
        (r.right() - 88.0, r.top() - 58.0)
    } else {
        (grid_top_left.x + avail_w - 104.0, grid_top_left.y - 56.0)
    };
    let anim_x = ui
        .ctx()
        .animate_value_with_time(ui.make_persistent_id("mascot_x"), target_x, 0.18);
    let anim_y = ui
        .ctx()
        .animate_value_with_time(ui.make_persistent_id("mascot_y"), target_y, 0.18);

    let active_mascot = if any_hovered {
        mascot_excited_tex.or(mascot_holding_tex)
    } else {
        mascot_holding_tex.or(mascot_excited_tex)
    };
    if let Some(mtex) = active_mascot {
        let bounce_y = react_t * 8.0;
        let size = 80.0 + react_t * 6.0;
        let peek_rect =
            egui::Rect::from_min_size(egui::pos2(anim_x, anim_y - bounce_y), egui::vec2(size, size));
        ui.put(peek_rect, egui::Image::new(mtex).fit_to_exact_size(egui::vec2(size, size)));
    }
}

fn show_confirmation_modal(ctx: &egui::Context, state: &mut AppState) {
    let Some(pending) = state.pending_confirm.clone() else {
        return;
    };
    let mut keep_open = true;
    egui::Window::new(&pending.title)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .collapsible(false)
        .resizable(false)
        .default_width(380.0)
        .show(ctx, |ui| {
            ui.label(egui::RichText::new(&pending.summary).size(13.5).color(TEXT_SLATE));
            ui.add_space(14.0);
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let confirm_btn = egui::Button::new(
                        egui::RichText::new("Confirm")
                            .color(egui::Color32::WHITE)
                            .strong(),
                    )
                    .fill(ACCENT_BLUE);
                    if ui.add_sized([100.0, 32.0], confirm_btn).clicked() {
                        execute_confirmed_action(state, pending.action);
                        keep_open = false;
                    }
                    if ui.add_sized([90.0, 32.0], egui::Button::new("Cancel")).clicked() {
                        keep_open = false;
                    }
                });
            });
        });
    if !keep_open {
        state.pending_confirm = None;
    }
}

fn execute_confirmed_action(state: &mut AppState, action: models::ConfirmAction) {
    match action {
        models::ConfirmAction::ResetDemo => {
            *state = AppState::new_demo();
            state.set_banner("Demo data reset to initial state.", false);
        }
        models::ConfirmAction::SaveXlsx => {
            match excel_sync::save_to_xlsx(excel_sync::DEFAULT_XLSX_PATH, state) {
                Ok(p) => state.set_banner(format!("Saved to {}", p), false),
                Err(e) => state.set_banner(e, true),
            }
        }
        models::ConfirmAction::KioskConfirm => {
            ui::requisition::execute_kiosk_order(state);
        }
        models::ConfirmAction::StockInConfirm => {
            ui::stock_in::execute_stock_in(state);
        }
        models::ConfirmAction::AdjustmentConfirm => {
            ui::adjustment::execute_adjustment(state);
        }
    }
}

fn main() -> eframe::Result<()> {
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([1120.0, 760.0])
        .with_title("Sta. Clara Dialysis Center — Inventory Management");
    if let Some((rgba, w, h)) = load_png_rgba(LOGO_BYTES) {
        viewport = viewport.with_icon(std::sync::Arc::new(egui::IconData {
            rgba,
            width: w,
            height: h,
        }));
    }
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "Sta. Clara Dialysis Center",
        options,
        Box::new(|_cc| Ok(Box::<DialysisApp>::default())),
    )
}

#[cfg(test)]
mod ui_tests {
    use super::*;

    #[test]
    fn test_all_views_render_without_panic_at_mobile_and_desktop_widths() {
        let ctx = egui::Context::default();
        let mut app = DialysisApp::default();
        for width in [420.0_f32, 520.0_f32, 820.0_f32, 1180.0_f32] {
            for view in [
                NavView::Home,
                NavView::Dashboard,
                NavView::StockIn,
                NavView::Requisition,
                NavView::Adjustment,
                NavView::TechLogs,
                NavView::MonthlyConsumption,
            ] {
                app.state.active_view = view;
                let raw_input = egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(width, 860.0),
                    )),
                    ..Default::default()
                };
                let _ = ctx.run(raw_input, |ctx| {
                    app.render_ui(ctx);
                });
            }
        }
    }
}
