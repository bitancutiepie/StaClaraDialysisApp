use crate::models::{AdjustmentReason, AdjustmentRecord, AppState};
use crate::ui::icons::IconKind;
use crate::ui::theme::*;
use chrono::Local;
use egui::{Color32, RichText, Ui};

pub fn show_adjustment(ui: &mut Ui, state: &mut AppState) {
    page_header(
        ui,
        IconKind::AlertTriangle,
        "Stock Adjustment & Wastage",
        "Record damaged, expired, or borrowed supplies with reason notes",
    );
    ui.add_space(12.0);

    card_frame().show(ui, |ui| {
        ui.set_max_width(ui.available_width());
        let field_w = (ui.available_width() - 180.0).clamp(140.0, 280.0);
        egui::Grid::new("adj_form")
            .spacing([12.0, 12.0])
            .show(ui, |ui| {
                ui.label(RichText::new("Item:").strong());
                let selected_name = state.items[state.adj_selected_item].name.clone();
                egui::ComboBox::from_id_salt("adj_item_combo")
                    .width(field_w)
                    .selected_text(selected_name)
                    .show_ui(ui, |ui| {
                        for (idx, item) in state.items.iter().enumerate() {
                            ui.selectable_value(&mut state.adj_selected_item, idx, &item.name);
                        }
                    });
                ui.end_row();

                ui.label(RichText::new("Quantity (Pcs):").strong());
                ui.add(egui::DragValue::new(&mut state.adj_qty).range(1..=500));
                ui.end_row();

                ui.label(RichText::new("Reason:").strong());
                ui.horizontal_wrapped(|ui| {
                    ui.radio_value(
                        &mut state.adj_reason,
                        AdjustmentReason::Damaged,
                        AdjustmentReason::Damaged.label(),
                    );
                    ui.radio_value(
                        &mut state.adj_reason,
                        AdjustmentReason::Expired,
                        AdjustmentReason::Expired.label(),
                    );
                    ui.radio_value(
                        &mut state.adj_reason,
                        AdjustmentReason::Borrowed,
                        AdjustmentReason::Borrowed.label(),
                    );
                });
                ui.end_row();

                ui.label(RichText::new("Reason Note:").strong());
                ui.add(egui::TextEdit::singleline(&mut state.adj_note).desired_width(field_w));
                ui.end_row();
            });

        ui.add_space(12.0);
        let btn = egui::Button::new(
            RichText::new("Record Adjustment")
                .color(Color32::WHITE)
                .strong(),
        )
        .fill(ACCENT_BLUE)
        .rounding(egui::Rounding::same(RADIUS_SM));
        if ui.add_sized([200.0, 36.0], btn).clicked() {
            let idx = state.adj_selected_item;
            let qty = state.adj_qty;
            let avail = state.items[idx].ending_balance();
            if qty > avail {
                state.set_banner(
                    format!(
                        "Insufficient stock for {}: requested {}, available {}",
                        state.items[idx].name, qty, avail
                    ),
                    true,
                );
            } else {
                state.pending_confirm = Some(crate::models::PendingConfirm {
                    title: "Confirm Stock Adjustment".to_string(),
                    summary: format!(
                        "Deduct {} pcs of {} ({}) from Ending Balance?",
                        qty,
                        state.items[idx].name,
                        state.adj_reason.label()
                    ),
                    action: crate::models::ConfirmAction::AdjustmentConfirm,
                });
            }
        }
    });

    ui.add_space(12.0);
    card_frame().show(ui, |ui| {
        ui.label(RichText::new("Adjustment History").strong().size(14.5));
        ui.add_space(6.0);
        if state.adjustment_logs.is_empty() {
            empty_state(
                ui,
                IconKind::AlertTriangle,
                "No adjustments recorded",
                "Damaged, expired, or borrowed deductions will appear here.",
            );
        } else {
            for log in state.adjustment_logs.iter().take(5) {
                ui.horizontal_wrapped(|ui| {
                    let (bg, fg) = match log.reason {
                        AdjustmentReason::Damaged => (BADGE_RED_BG, BADGE_RED_FG),
                        AdjustmentReason::Expired => (BADGE_AMBER_BG, BADGE_AMBER_FG),
                        AdjustmentReason::Borrowed => (ACCENT_SKY, ACCENT_BLUE),
                    };
                    badge(ui, log.reason.label(), bg, fg);
                    ui.label(RichText::new(&log.item_name).strong());
                    ui.label(
                        RichText::new(format!(
                            "-{} pcs | Note: {} ({})",
                            log.qty, log.note, log.timestamp
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

pub fn execute_adjustment(state: &mut AppState) {
    let idx = state.adj_selected_item;
    let qty = state.adj_qty;
    match state.deduct_item(idx, qty) {
        Ok(_) => {
            let item_name = state.items[idx].name.clone();
            let note = if state.adj_note.trim().is_empty() {
                "-".to_string()
            } else {
                state.adj_note.trim().to_string()
            };
            state.adjustment_logs.insert(
                0,
                AdjustmentRecord {
                    timestamp: Local::now().format("%H:%M").to_string(),
                    item_name: item_name.clone(),
                    qty,
                    reason: state.adj_reason,
                    note,
                },
            );
            state.adj_note.clear();
            state.set_banner(
                format!(
                    "Deducted {} pcs of {} ({}).",
                    qty,
                    item_name,
                    state.adj_reason.label()
                ),
                false,
            );
        }
        Err(e) => state.set_banner(e, true),
    }
}
