use crate::models::{AppState, Department};
use crate::ui::icons::IconKind;
use crate::ui::theme::*;
use chrono::Datelike;
use egui::{RichText, ScrollArea, Ui};
use std::borrow::Cow;

pub fn show_monthly_consumption(ui: &mut Ui, state: &mut AppState) {
    page_header(
        ui,
        IconKind::Calendar,
        "Monthly Usage Summary",
        &format!(
            "Day {} updates live from Nurse Requisitions & Stock Adjustments",
            state.today.day()
        ),
    );
    ui.add_space(12.0);

    // Top 5 Consumed Items Bar Chart
    let mut top_items: Vec<(&str, i32)> = state
        .items
        .iter()
        .filter(|i| state.mc_dept_filter == Department::All || i.department == state.mc_dept_filter)
        .map(|i| (i.name.as_str(), i.monthly_total()))
        .collect();
    top_items.sort_by(|a, b| b.1.cmp(&a.1));
    let max_val = top_items.first().map(|x| x.1).unwrap_or(1).max(1) as f32;

    card_frame().show(ui, |ui| {
        ui.label(
            RichText::new("Top Consumed Supplies (This Month)")
                .size(14.5)
                .strong()
                .color(TEXT_SLATE),
        );
        ui.add_space(8.0);
        for (name, total) in top_items.iter().take(5) {
            ui.horizontal(|ui| {
                ui.label(RichText::new(*name).size(12.0).strong().color(TEXT_SLATE));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(format!("{} pcs", total)).size(12.0).color(ACCENT_BLUE).strong());
                });
            });
            ui.add(
                egui::ProgressBar::new(*total as f32 / max_val)
                    .fill(ACCENT_BLUE)
                    .desired_height(6.0),
            );
            ui.add_space(4.0);
        }
    });

    ui.add_space(10.0);

    ui.horizontal_wrapped(|ui| {
        for dept in [
            Department::All,
            Department::Treatment,
            Department::ReverseOsmosis,
            Department::CenterSupplies,
        ] {
            ui.selectable_value(&mut state.mc_dept_filter, dept, dept.label());
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.checkbox(&mut state.mc_show_calendar, "Show 31-Day Calendar");
        });
    });
    ui.add_space(8.0);

    let today_day = state.today.day() as usize;
    let today_idx = today_day.saturating_sub(1).min(30);

    card_frame().show(ui, |ui| {
        ScrollArea::horizontal()
            .id_salt("monthly_table_scroll")
            .show(ui, |ui| {
                egui::Grid::new("monthly_consumption_grid")
                    .striped(true)
                    .spacing([14.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(RichText::new("Dept").color(TEXT_MUTED).strong().size(12.0));
                        ui.label(RichText::new("Item").color(TEXT_MUTED).strong().size(12.0));
                        if state.mc_show_calendar {
                            for d in 1..=31 {
                                let color = if d == today_day { ACCENT_BLUE } else { TEXT_MUTED };
                                ui.label(RichText::new(format!("{}", d)).color(color).strong().size(11.5));
                            }
                        } else {
                            ui.label(RichText::new("Used Today").color(ACCENT_BLUE).strong().size(12.0));
                        }
                        ui.label(RichText::new("Used This Month").color(TEXT_SLATE).strong().size(12.0));
                        ui.end_row();

                        for item in &state.items {
                            if state.mc_dept_filter != Department::All
                                && item.department != state.mc_dept_filter
                            {
                                continue;
                            }
                            ui.label(RichText::new(item.department.label()).size(11.0).color(TEXT_MUTED));
                            ui.label(RichText::new(&item.name).strong().size(12.0).color(TEXT_SLATE));
                            if state.mc_show_calendar {
                                for (idx, &val) in item.daily_consumption.iter().enumerate() {
                                    let is_today = idx + 1 == today_day;
                                    let text: Cow<'static, str> = if val == 0 {
                                        Cow::Borrowed("-")
                                    } else {
                                        Cow::Owned(val.to_string())
                                    };
                                    let color = if is_today && val > 0 {
                                        ACCENT_BLUE
                                    } else if val == 0 {
                                        TEXT_MUTED
                                    } else {
                                        TEXT_SLATE
                                    };
                                    ui.label(RichText::new(text.as_ref()).color(color).size(11.5));
                                }
                            } else {
                                let used_today = item.daily_consumption[today_idx];
                                ui.label(
                                    RichText::new(format!("{} pcs", used_today))
                                        .color(if used_today > 0 { ACCENT_BLUE } else { TEXT_MUTED })
                                        .strong(),
                                );
                            }
                            ui.label(
                                RichText::new(format!("{} pcs", item.monthly_total()))
                                    .strong()
                                    .color(TEXT_SLATE),
                            );
                            ui.end_row();
                        }
                    });
            });
    });
}
