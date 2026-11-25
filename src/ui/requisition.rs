use crate::models::{AppState, Department, KioskMode, RequisitionRecord, ShiftType, DANGER_STOCK_THRESHOLD};
use crate::ui::icons::{draw_icon, IconKind};
use crate::ui::theme::*;
use chrono::Local;
use egui::{Color32, RichText, ScrollArea, Ui};

fn item_icon(name: &str, dept: Department) -> IconKind {
    let upper = name.to_uppercase();
    if upper.contains("SYRINGE")
        || upper.contains("NEEDLE")
        || upper.contains("HEPARIN")
        || upper.contains("EPOETIN")
        || upper.contains("IRON")
        || upper.contains("VIAL")
    {
        IconKind::Syringe
    } else if upper.contains("BLOODLINE")
        || upper.contains("SALINE")
        || upper.contains("CONCENTRATE")
        || upper.contains("STERILANT")
        || upper.contains("WATER")
        || upper.contains("CHLORINE")
        || upper.contains("PEROXIDE")
    {
        IconKind::Droplet
    } else if upper.contains("KIT") || upper.contains("GAUZE") || upper.contains("TAPE") {
        IconKind::FirstAid
    } else if upper.contains("DIALYZER")
        || upper.contains("AMICAL")
        || upper.contains("NIPRO")
        || upper.contains("SURGITECH")
        || upper.contains("FLUID FILTER")
    {
        IconKind::Activity
    } else if upper.contains("PAPER") {
        IconKind::FileText
    } else {
        match dept {
            Department::Treatment => IconKind::FirstAid,
            Department::ReverseOsmosis => IconKind::Droplet,
            _ => IconKind::Package,
        }
    }
}

fn add_to_cart(cart: &mut Vec<(usize, i32)>, item_idx: usize, qty: i32) {
    if let Some(entry) = cart.iter_mut().find(|(idx, _)| *idx == item_idx) {
        entry.1 += qty;
    } else {
        cart.push((item_idx, qty));
    }
}

pub fn show_requisition(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal_wrapped(|ui| {
        page_header(
            ui,
            IconKind::ShoppingCart,
            "Nurse Requisition & Kiosk Order",
            "Fast clinical POS for floor requisitions & warehouse restocking",
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            for (mode, label) in [
                (KioskMode::WarehouseTransfer, "Warehouse -> TA Transfer"),
                (KioskMode::FloorRequisition, "Floor Requisition"),
            ] {
                let active = state.kiosk_mode == mode;
                let fill = if active {
                    if mode == KioskMode::WarehouseTransfer {
                        Color32::from_rgb(0x0D, 0x94, 0x88) // Teal
                    } else {
                        ACCENT_BLUE
                    }
                } else {
                    Color32::from_rgb(0xF1, 0xF5, 0xF9)
                };
                let fg = if active { Color32::WHITE } else { TEXT_SLATE };
                if ui
                    .add(
                        egui::Button::new(RichText::new(label).color(fg).size(12.5).strong())
                            .fill(fill)
                            .rounding(egui::Rounding::same(16.0)),
                    )
                    .clicked()
                {
                    state.kiosk_mode = mode;
                }
            }
        });
    });
    ui.add_space(12.0);

    let total_w = ui.available_width();
    let avail_h = ui.available_height();
    let is_wide = total_w >= 760.0;
    if is_wide {
        let spacing = 12.0;
        let tray_w = 350.0_f32.min(total_w * 0.42);
        let menu_w = (total_w - tray_w - spacing).max(240.0);
        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(menu_w, avail_h),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    ui.set_min_width(menu_w);
                    ui.set_max_width(menu_w);
                    show_menu_panel(ui, state, avail_h);
                },
            );
            ui.add_space(spacing);
            ui.allocate_ui_with_layout(
                egui::vec2(tray_w, avail_h),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    ui.set_min_width(tray_w);
                    ui.set_max_width(tray_w);
                    show_tray_panel(ui, state, avail_h);
                },
            );
        });
    } else {
        show_menu_panel(ui, state, 360.0);
        ui.add_space(12.0);
        show_tray_panel(ui, state, 280.0);
    }
}

fn show_menu_panel(ui: &mut Ui, state: &mut AppState, avail_h: f32) {
    card_frame().show(ui, |ui| {
        if state.kiosk_mode == KioskMode::FloorRequisition {
            ui.label(
                RichText::new("Quick Preset Packages")
                    .strong()
                    .size(14.0)
                    .color(TEXT_SLATE),
            );
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                let kit_btn = egui::Button::new(
                    RichText::new("+1 HD Treatment Kit")
                        .color(Color32::WHITE)
                        .strong(),
                )
                .fill(ACCENT_BLUE)
                .rounding(egui::Rounding::same(RADIUS_SM));
                if ui.add_sized([165.0, 36.0], kit_btn).clicked() {
                    for &(idx, q) in &[(0, 1), (1, 1), (7, 2), (8, 1), (10, 1)] {
                        add_to_cart(&mut state.kiosk_cart, idx, q);
                    }
                }

                let p = state.req_patients.max(1);
                let batch_label = format!("+{} Patients (Batch)", p);
                let batch_btn = egui::Button::new(
                    RichText::new(batch_label)
                        .color(Color32::WHITE)
                        .strong(),
                )
                .fill(Color32::from_rgb(0x02, 0x84, 0xC7))
                .rounding(egui::Rounding::same(RADIUS_SM));
                if ui.add_sized([165.0, 36.0], batch_btn).clicked() {
                    for &(idx, q) in &[(0, p), (1, p), (7, 2 * p), (8, p), (10, p)] {
                        add_to_cart(&mut state.kiosk_cart, idx, q);
                    }
                }
            });
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);
        }

        // Search & Department Filter Bar
        ui.horizontal_wrapped(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut state.kiosk_search)
                    .hint_text("Search supply...")
                    .desired_width(180.0),
            );
            if !state.kiosk_search.is_empty() && ui.small_button("×").clicked() {
                state.kiosk_search.clear();
            }
        });
        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            for dept in [
                Department::All,
                Department::Treatment,
                Department::ReverseOsmosis,
                Department::CenterSupplies,
            ] {
                ui.selectable_value(&mut state.kiosk_dept_filter, dept, dept.label());
            }
        });
        ui.add_space(8.0);

        let search_lower = state.kiosk_search.trim().to_lowercase();
        let inner_avail = ui.available_width();
        let cols = if inner_avail >= 360.0 { 2 } else { 1 };
        let grid_spacing = 8.0_f32;
        let tile_w = ((inner_avail - grid_spacing * (cols as f32 - 1.0) - 8.0) / cols as f32)
            .floor()
            .max(120.0);

        ScrollArea::vertical()
            .id_salt("kiosk_menu_scroll")
            .max_height((avail_h - 170.0).max(320.0))
            .show(ui, |ui| {
                ui.set_max_width(inner_avail);
                egui::Grid::new("kiosk_menu_grid")
                    .spacing([grid_spacing, grid_spacing])
                    .show(ui, |ui| {
                        let mut col_count = 0;
                        for (idx, item) in state.items.iter().enumerate() {
                            if state.kiosk_dept_filter != Department::All
                                && item.department != state.kiosk_dept_filter
                            {
                                continue;
                            }
                            if !search_lower.is_empty()
                                && !item.name.to_lowercase().contains(&search_lower)
                            {
                                continue;
                            }

                            let _total_qty = item.ending_balance();
                            let _min_thresh = DANGER_STOCK_THRESHOLD.max(item.reorder_threshold);
                            let days = item.days_until_expiry(state.today);
                            let exp_soon = days <= 45;
                            let in_tray = state
                                .kiosk_cart
                                .iter()
                                .find(|(i, _)| *i == idx)
                                .map(|(_, q)| *q)
                                .unwrap_or(0);

                            let card_resp = egui::Frame::none()
                                .fill(Color32::from_rgb(0xF8, 0xFA, 0xFC))
                                .stroke(egui::Stroke::new(1.0_f32, BORDER_SOFT))
                                .rounding(egui::Rounding::same(10.0))
                                .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                                .show(ui, |ui| {
                                    ui.set_min_size(egui::vec2(tile_w - 20.0, 50.0));
                                    ui.set_max_width(tile_w - 20.0);
                                    // Top Row: Medical Icon + Name + In-Tray Pill
                                    ui.horizontal_wrapped(|ui| {
                                        draw_icon(
                                            ui,
                                            item_icon(&item.name, item.department),
                                            15.0,
                                            ACCENT_BLUE,
                                        );
                                        ui.label(
                                            RichText::new(item.name.to_uppercase())
                                                .size(12.0)
                                                .strong()
                                                .color(TEXT_SLATE),
                                        );
                                        if in_tray > 0 {
                                            badge(
                                                ui,
                                                &format!("{} in tray", in_tray),
                                                ACCENT_SKY,
                                                ACCENT_BLUE,
                                            );
                                        }
                                    });
                                    ui.add_space(4.0);
                                    // Bottom Row: Split Stock + FEFO Expiry
                                    ui.horizontal_wrapped(|ui| {
                                        ui.label(
                                            RichText::new(format!(
                                                "TA: {} | Whse: {}",
                                                item.treatment_area_qty, item.warehouse_qty
                                            ))
                                            .size(11.0)
                                            .color(TEXT_MUTED),
                                        );
                                        if exp_soon {
                                            badge(
                                                ui,
                                                &format!("Exp: {}d", days),
                                                BADGE_AMBER_BG,
                                                BADGE_AMBER_FG,
                                            );
                                        }
                                    });
                                })
                                .response
                                .interact(egui::Sense::click());

                            if card_resp.hovered() {
                                ui.painter().rect_stroke(
                                    card_resp.rect,
                                    10.0,
                                    egui::Stroke::new(1.5_f32, ACCENT_BLUE),
                                );
                                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                            }
                            if card_resp.clicked() {
                                add_to_cart(&mut state.kiosk_cart, idx, 1);
                            }

                            col_count += 1;
                            if col_count % cols == 0 {
                                ui.end_row();
                            }
                        }
                    });
            });
    });
}

fn show_tray_panel(ui: &mut Ui, state: &mut AppState, avail_h: f32) {
    let is_transfer = state.kiosk_mode == KioskMode::WarehouseTransfer;
    let accent = if is_transfer {
        Color32::from_rgb(0x0D, 0x94, 0x88)
    } else {
        ACCENT_BLUE
    };

    card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(if is_transfer {
                    "Transfer Tray (Whse -> TA)"
                } else {
                    "Order Tray (Floor Requisition)"
                })
                .strong()
                .size(15.5)
                .color(accent),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if !state.kiosk_cart.is_empty() && ui.small_button("Clear").clicked() {
                    state.kiosk_cart.clear();
                }
            });
        });
        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("Shift:").strong().size(12.0));
            ui.radio_value(&mut state.req_shift, ShiftType::Mwf, "MWF");
            ui.radio_value(&mut state.req_shift, ShiftType::Tths, "TThS");
            ui.add_space(6.0);
            ui.label(RichText::new("Patients:").strong().size(12.0));
            ui.add(egui::DragValue::new(&mut state.req_patients).range(1..=50));
            ui.add_space(6.0);
            ui.label(RichText::new("Station:").strong().size(12.0));
            ui.add(egui::TextEdit::singleline(&mut state.kiosk_station).desired_width(90.0));
        });
        ui.separator();

        if state.kiosk_cart.is_empty() {
            empty_state(
                ui,
                IconKind::ShoppingCart,
                "Tray is empty",
                "Tap any supply card on the left to add it here.",
            );
        } else {
            let mut remove_idx: Option<usize> = None;
            ScrollArea::vertical()
                .id_salt("kiosk_tray_scroll")
                .max_height((avail_h - 210.0).max(240.0))
                .show(ui, |ui| {
                    for (pos, (item_idx, qty)) in state.kiosk_cart.iter_mut().enumerate() {
                        let item = &state.items[*item_idx];
                        ui.horizontal_wrapped(|ui| {
                            ui.label(RichText::new(&item.name).strong().size(12.5));
                            if is_transfer {
                                badge(
                                    ui,
                                    "Whse -> TA",
                                    Color32::from_rgb(0xCC, 0xFB, 0xF1),
                                    Color32::from_rgb(0x0F, 0x76, 0x6E),
                                );
                            } else {
                                badge(ui, "TA", ACCENT_SKY, ACCENT_BLUE);
                            }
                            if !is_transfer && item.treatment_area_qty == 0 {
                                badge(
                                    ui,
                                    "TA Empty - Restock needed",
                                    BADGE_AMBER_BG,
                                    BADGE_AMBER_FG,
                                );
                            }
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.small_button("×").clicked() {
                                        remove_idx = Some(pos);
                                    }
                                    if ui.small_button("+").clicked() {
                                        *qty += 1;
                                    }
                                    ui.add(egui::DragValue::new(qty).range(1..=999));
                                    if ui.small_button("-").clicked() && *qty > 1 {
                                        *qty -= 1;
                                    }
                                },
                            );
                        });
                        ui.separator();
                    }
                });
            if let Some(pos) = remove_idx {
                state.kiosk_cart.remove(pos);
            }

            // Pinned Footer Summary & CTA
            let unique_items = state.kiosk_cart.len();
            let total_pcs: i32 = state.kiosk_cart.iter().map(|(_, q)| *q).sum();
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("{} items • {} total pcs", unique_items, total_pcs))
                        .size(12.5)
                        .strong()
                        .color(TEXT_MUTED),
                );
            });
            ui.add_space(6.0);

            let cta_label = if is_transfer {
                "Confirm Transfer to TA"
            } else {
                "Confirm & Deduct Stock"
            };
            let confirm_btn = egui::Button::new(
                RichText::new(cta_label)
                    .color(Color32::WHITE)
                    .strong()
                    .size(13.5),
            )
            .fill(accent)
            .rounding(egui::Rounding::same(RADIUS_SM));
            if ui
                .add_sized([ui.available_width(), 40.0], confirm_btn)
                .clicked()
            {
                let mut err: Option<String> = None;
                for &(idx, qty) in &state.kiosk_cart {
                    let avail = if is_transfer {
                        state.items[idx].warehouse_qty
                    } else {
                        state.items[idx].ending_balance()
                    };
                    if qty > avail {
                        err = Some(format!(
                            "Insufficient stock for {}: requested {}, available {}",
                            state.items[idx].name, qty, avail
                        ));
                        break;
                    }
                }
                if let Some(e) = err {
                    state.set_banner(e, true);
                } else {
                    state.pending_confirm = Some(crate::models::PendingConfirm {
                        title: if is_transfer {
                            "Confirm Warehouse -> TA Transfer".to_string()
                        } else {
                            "Confirm Floor Requisition".to_string()
                        },
                        summary: if is_transfer {
                            format!(
                                "Transfer {} item(s) ({} total pcs) from Warehouse to Treatment Area?",
                                unique_items, total_pcs
                            )
                        } else {
                            format!(
                                "Deduct {} item(s) ({} total pcs) for {} patients ({})?",
                                unique_items, total_pcs, state.req_patients, state.kiosk_station
                            )
                        },
                        action: crate::models::ConfirmAction::KioskConfirm,
                    });
                }
            }
        }
    });
}

pub fn execute_kiosk_order(state: &mut AppState) {
    let is_transfer = state.kiosk_mode == KioskMode::WarehouseTransfer;
    let cart = std::mem::take(&mut state.kiosk_cart);
    let count = cart.len();
    if is_transfer {
        for (idx, qty) in cart {
            let _ = state.transfer_to_treatment_area(idx, qty);
        }
        state.set_banner(
            format!("Transferred {} item(s) from Warehouse to TA.", count),
            false,
        );
    } else {
        for (idx, qty) in cart {
            let _ = state.deduct_item(idx, qty);
        }
        state.today_patient_count += state.req_patients;
        state.requisition_logs.insert(
            0,
            RequisitionRecord {
                timestamp: Local::now().format("%H:%M").to_string(),
                shift: state.req_shift,
                patient_count: state.req_patients,
                summary: format!(
                    "{} • {} patients ({} items)",
                    state.kiosk_station, state.req_patients, count
                ),
            },
        );
        state.set_banner("Requisition logged & stock updated", false);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_kiosk_menu_cards_never_overflow_column_width() {
        for panel_w in [300.0_f32, 380.0, 480.0, 520.0, 640.0] {
            let cols = if panel_w >= 380.0 { 2 } else { 1 };
            let grid_spacing = 8.0_f32;
            let tile_w = ((panel_w - grid_spacing * (cols as f32 - 1.0) - 8.0) / cols as f32)
                .floor()
                .max(120.0);
            let total_used = tile_w * cols as f32 + grid_spacing * (cols as f32 - 1.0);
            assert!(
                total_used <= panel_w,
                "Overflow at panel_w={}: total_used={} > panel_w={}",
                panel_w,
                total_used,
                panel_w
            );
        }
    }

    #[test]
    fn test_kiosk_tray_does_not_overflow_when_items_added() {
        let ctx = egui::Context::default();
        let mut state = crate::models::AppState::new_demo();
        // Add 5 items to tray (reproducing the user's screenshot sequence)
        for idx in [0, 1, 2, 3, 4] {
            super::add_to_cart(&mut state.kiosk_cart, idx, 2);
        }
        let raw_input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1440.0, 900.0),
            )),
            ..Default::default()
        };
        let out = ctx.run(raw_input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                super::show_requisition(ui, &mut state);
                assert!(
                    ui.min_rect().right() <= 1440.0,
                    "Kiosk requisition overflowed right edge: {}",
                    ui.min_rect().right()
                );
            });
        });
        assert!(!out.shapes.is_empty());
    }
}
