use crate::models::{AppState, StockInRecord, UnitType};
use crate::ui::icons::IconKind;
use crate::ui::theme::*;
use chrono::{Local, NaiveDate};
use egui::{Color32, RichText, Ui};

pub fn show_stock_in(ui: &mut Ui, state: &mut AppState) {
    page_header(
        ui,
        IconKind::Package,
        "Stock-In (Supplier Delivery)",
        "Record incoming deliveries from Helande Kidney Care",
    );
    ui.add_space(12.0);

    card_frame().show(ui, |ui| {
        ui.set_max_width(ui.available_width());
        let field_w = (ui.available_width() - 180.0).clamp(140.0, 280.0);
        egui::Grid::new("stock_in_form")
            .spacing([12.0, 12.0])
            .show(ui, |ui| {
                ui.label(RichText::new("Supplier:").strong());
                ui.label(RichText::new("Helande Kidney Care").color(ACCENT_BLUE).strong());
                ui.end_row();

                ui.label(RichText::new("Item:").strong());
                let selected_name = state.items[state.si_selected_item].name.clone();
                egui::ComboBox::from_id_salt("si_item_combo")
                    .width(field_w)
                    .selected_text(selected_name)
                    .show_ui(ui, |ui| {
                        for (idx, item) in state.items.iter().enumerate() {
                            ui.selectable_value(&mut state.si_selected_item, idx, &item.name);
                        }
                    });
                ui.end_row();

                ui.label(RichText::new("Quantity & Unit:").strong());
                ui.horizontal_wrapped(|ui| {
                    ui.add(egui::DragValue::new(&mut state.si_qty).range(1..=1000));
                    ui.radio_value(&mut state.si_unit, UnitType::Box, "Box");
                    ui.radio_value(&mut state.si_unit, UnitType::Pcs, "Pcs");
                    let pcs_per_box = state.items[state.si_selected_item].pcs_per_box;
                    let total_pcs = match state.si_unit {
                        UnitType::Box => state.si_qty * pcs_per_box,
                        UnitType::Pcs => state.si_qty,
                    };
                    ui.label(
                        RichText::new(format!("(= {} pcs, 1 Box = {} pcs)", total_pcs, pcs_per_box))
                            .color(TEXT_MUTED)
                            .size(12.0),
                    );
                });
                ui.end_row();

                ui.label(RichText::new("Batch Number:").strong());
                ui.add(egui::TextEdit::singleline(&mut state.si_batch).desired_width(field_w));
                ui.end_row();

                ui.label(RichText::new("Expiry (YYYY-MM-DD):").strong());
                ui.add(egui::TextEdit::singleline(&mut state.si_expiry_str).desired_width(field_w));
                ui.end_row();
            });

        ui.add_space(12.0);
        let submit = egui::Button::new(
            RichText::new("Record Stock-In Delivery")
                .color(Color32::WHITE)
                .strong(),
        )
        .fill(ACCENT_BLUE)
        .rounding(egui::Rounding::same(RADIUS_SM));
        if ui.add_sized([220.0, 36.0], submit).clicked() {
            match NaiveDate::parse_from_str(state.si_expiry_str.trim(), "%Y-%m-%d") {
                Ok(exp_date) => {
                    let idx = state.si_selected_item;
                    let pcs_per_box = state.items[idx].pcs_per_box;
                    let qty_pcs = match state.si_unit {
                        UnitType::Box => state.si_qty * pcs_per_box,
                        UnitType::Pcs => state.si_qty,
                    };
                    state.pending_confirm = Some(crate::models::PendingConfirm {
                        title: "Confirm Stock-In Delivery".to_string(),
                        summary: format!(
                            "Add +{} pcs of {} (Batch: {}, Exp: {}) to Warehouse?",
                            qty_pcs, state.items[idx].name, state.si_batch, exp_date
                        ),
                        action: crate::models::ConfirmAction::StockInConfirm,
                    });
                }
                Err(_) => {
                    state.set_banner("Invalid expiry date. Use YYYY-MM-DD format.", true);
                }
            }
        }
    });

    ui.add_space(12.0);
    card_frame().show(ui, |ui| {
        ui.label(RichText::new("Recent Deliveries").strong().size(14.5));
        ui.add_space(6.0);
        if state.stock_in_logs.is_empty() {
            empty_state(
                ui,
                IconKind::Package,
                "No deliveries recorded yet",
                "Submitted Helande Kidney Care deliveries will appear here.",
            );
        } else {
            for log in state.stock_in_logs.iter().take(5) {
                ui.horizontal_wrapped(|ui| {
                    badge(ui, &format!("+{} pcs", log.qty_pcs), BADGE_GREEN_BG, BADGE_GREEN_FG);
                    ui.label(RichText::new(&log.item_name).strong());
                    ui.label(
                        RichText::new(format!(
                            "{} | Batch: {} | Exp: {} ({})",
                            log.unit_label, log.batch_no, log.expiry, log.timestamp
                        ))
                        .size(12.0)
                        .color(TEXT_MUTED),
                    );
                });
                ui.add_space(4.0);
            }
        }
    });
}

pub fn execute_stock_in(state: &mut AppState) {
    let Ok(exp_date) = NaiveDate::parse_from_str(state.si_expiry_str.trim(), "%Y-%m-%d") else {
        state.set_banner("Invalid expiry date. Use YYYY-MM-DD format.", true);
        return;
    };
    let idx = state.si_selected_item;
    let pcs_per_box = state.items[idx].pcs_per_box;
    let qty_pcs = match state.si_unit {
        UnitType::Box => state.si_qty * pcs_per_box,
        UnitType::Pcs => state.si_qty,
    };
    let unit_label = match state.si_unit {
        UnitType::Box => format!("{} Box", state.si_qty),
        UnitType::Pcs => format!("{} Pcs", state.si_qty),
    };
    state.items[idx].stock_in += qty_pcs;
    state.items[idx].warehouse_qty += qty_pcs;
    if exp_date < state.items[idx].earliest_expiry {
        state.items[idx].earliest_expiry = exp_date;
    }
    let item_name = state.items[idx].name.clone();
    state.stock_in_logs.insert(
        0,
        StockInRecord {
            timestamp: Local::now().format("%H:%M").to_string(),
            supplier: "Helande Kidney Care".to_string(),
            item_name: item_name.clone(),
            qty_pcs,
            unit_label,
            batch_no: state.si_batch.clone(),
            expiry: exp_date,
        },
    );
    state.set_banner(
        format!("Added +{} pcs of {} to Warehouse.", qty_pcs, item_name),
        false,
    );
}
