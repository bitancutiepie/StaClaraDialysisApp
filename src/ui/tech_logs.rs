use crate::models::{AppState, RoWaterCheck};
use crate::ui::icons::IconKind;
use crate::ui::theme::*;
use chrono::Local;
use egui::{Color32, RichText, ScrollArea, Ui};

pub fn show_tech_logs(ui: &mut Ui, state: &mut AppState) {
    page_header(
        ui,
        IconKind::Wrench,
        "Technician Logs",
        "Dialyzer Reuse Counter & Daily RO Water Quality Check",
    );
    ui.add_space(12.0);

    ui.horizontal_wrapped(|ui| {
        ui.selectable_value(&mut state.tech_tab, 0, "Patient Dialyzer Reuse");
        ui.selectable_value(&mut state.tech_tab, 1, "Daily RO Water Quality");
    });
    ui.add_space(8.0);

    if state.tech_tab == 0 {
        card_frame().show(ui, |ui| {
            ui.label(
                RichText::new("Patient Dialyzer Reuse Counter")
                    .strong()
                    .size(15.5),
            );
            ui.add_space(8.0);

            ScrollArea::horizontal()
                .id_salt("dialyzer_table_scroll")
                .show(ui, |ui| {
                    egui::Grid::new("dialyzer_grid")
                        .striped(true)
                        .spacing([18.0, 10.0])
                        .show(ui, |ui| {
                            for h in ["Patient ID", "Patient Name", "Reuse Cycle", "Status", "Action"] {
                                ui.label(RichText::new(h).color(TEXT_MUTED).strong().size(12.0));
                            }
                            ui.end_row();

                            for p in &mut state.dialyzer_patients {
                                ui.label(&p.patient_id);
                                ui.label(RichText::new(&p.patient_name).strong());
                                let ratio = p.reuse_count as f32 / p.max_reuse.max(1) as f32;
                                ui.horizontal(|ui| {
                                    ui.add(
                                        egui::ProgressBar::new(ratio)
                                            .desired_width(70.0)
                                            .desired_height(6.0),
                                    );
                                    ui.label(format!("{}/{}", p.reuse_count, p.max_reuse));
                                });
                                if p.reuse_count >= p.max_reuse {
                                    badge(ui, "Replace", BADGE_RED_BG, BADGE_RED_FG);
                                } else if p.reuse_count >= p.max_reuse - 2 {
                                    badge(ui, "Near Limit", BADGE_AMBER_BG, BADGE_AMBER_FG);
                                } else {
                                    badge(ui, "Active", BADGE_GREEN_BG, BADGE_GREEN_FG);
                                }
                                if ui
                                    .add_enabled(
                                        p.reuse_count < p.max_reuse,
                                        egui::Button::new("+1 Reuse Cycle"),
                                    )
                                    .clicked()
                                {
                                    p.reuse_count += 1;
                                }
                                ui.end_row();
                            }
                        });
                });
        });
    } else {
        card_frame().show(ui, |ui| {
            ui.set_max_width(ui.available_width());
            let field_w = (ui.available_width() - 180.0).clamp(140.0, 260.0);
            ui.label(
                RichText::new("Daily RO Water Quality Check")
                    .strong()
                    .size(15.5),
            );
            ui.add_space(8.0);

            egui::Grid::new("ro_form")
                .spacing([12.0, 10.0])
                .show(ui, |ui| {
                    ui.label(RichText::new("Technician:").strong());
                    ui.add(
                        egui::TextEdit::singleline(&mut state.ro_technician).desired_width(field_w),
                    );
                    ui.end_row();

                    ui.label(RichText::new("Water Hardness:").strong());
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut state.ro_hardness_pass, true, "Pass");
                        ui.radio_value(&mut state.ro_hardness_pass, false, "Fail");
                    });
                    ui.end_row();

                    ui.label(RichText::new("Total Chlorine:").strong());
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut state.ro_chlorine_pass, true, "Pass");
                        ui.radio_value(&mut state.ro_chlorine_pass, false, "Fail");
                    });
                    ui.end_row();

                    ui.label(RichText::new("Peroxide Residual:").strong());
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut state.ro_peroxide_pass, true, "Pass");
                        ui.radio_value(&mut state.ro_peroxide_pass, false, "Fail");
                    });
                    ui.end_row();
                });

            ui.add_space(10.0);
            let btn = egui::Button::new(
                RichText::new("Log RO Water Check")
                    .color(Color32::WHITE)
                    .strong(),
            )
            .fill(ACCENT_BLUE)
            .rounding(egui::Rounding::same(RADIUS_SM));
            if ui.add_sized([200.0, 34.0], btn).clicked() {
                state.ro_water_logs.insert(
                    0,
                    RoWaterCheck {
                        timestamp: Local::now().format("%I:%M %p").to_string(),
                        hardness_pass: state.ro_hardness_pass,
                        chlorine_pass: state.ro_chlorine_pass,
                        peroxide_pass: state.ro_peroxide_pass,
                        technician: state.ro_technician.clone(),
                    },
                );
                state.set_banner("RO Water Quality Check logged.", false);
            }

            ui.add_space(12.0);
            ui.label(RichText::new("Recent RO Water Logs").strong());
            ui.add_space(4.0);
            for log in state.ro_water_logs.iter().take(5) {
                let all_pass = log.hardness_pass && log.chlorine_pass && log.peroxide_pass;
                ui.horizontal_wrapped(|ui| {
                    if all_pass {
                        badge(ui, "ALL PASS", BADGE_GREEN_BG, BADGE_GREEN_FG);
                    } else {
                        badge(ui, "FAIL DETECTED", BADGE_RED_BG, BADGE_RED_FG);
                    }
                    ui.label(
                        RichText::new(format!(
                            "[{}] {} — Hardness: {} | Chlorine: {} | Peroxide: {}",
                            log.timestamp,
                            log.technician,
                            if log.hardness_pass { "Pass" } else { "Fail" },
                            if log.chlorine_pass { "Pass" } else { "Fail" },
                            if log.peroxide_pass { "Pass" } else { "Fail" },
                        ))
                        .size(12.0),
                    );
                });
                ui.add_space(3.0);
            }
        });
    }
}
