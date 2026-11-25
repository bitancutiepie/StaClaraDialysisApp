use crate::models::{
    alert_counts, filter_and_sort_alerts, AlertTab, AppState, Department, NavView,
};
use crate::ui::icons::{draw_icon, IconKind};
use crate::ui::theme::*;
use egui::{Color32, RichText, ScrollArea, Ui};

pub fn show_dashboard(ui: &mut Ui, state: &mut AppState, ctx: &egui::Context) {
    ui.horizontal_wrapped(|ui| {
        page_header(
            ui,
            IconKind::Dashboard,
            "Inventory & Daily Ending Summary",
            &format!("Sta. Clara Dialysis Center, Inc.  •  {}", state.today),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let btn = egui::Button::new(
                RichText::new("Daily Ending Report")
                    .color(Color32::WHITE)
                    .strong(),
            )
            .fill(ACCENT_BLUE)
            .rounding(egui::Rounding::same(RADIUS_SM));
            if ui.add_sized([170.0, 34.0], btn).clicked() {
                state.report_modal_open = true;
            }
            if ui.button("+ Kiosk Order").clicked() {
                state.active_view = NavView::Requisition;
            }
            if ui.button("+ Stock-In").clicked() {
                state.active_view = NavView::StockIn;
            }
        });
    });

    ui.add_space(12.0);

    let total_items = state.items.len();
    let low_stock_count = state.items.iter().filter(|i| i.is_low_stock()).count();
    let near_expiry_count = state
        .items
        .iter()
        .filter(|i| i.is_near_expiry(state.today))
        .count();

    // Responsive KPI Grid (4 cols on desktop, 2x2 on narrow screens)
    let kpis = [
        (
            IconKind::Package,
            "Total Items",
            total_items.to_string(),
            "Active medical supplies",
            ACCENT_BLUE,
        ),
        (
            IconKind::AlertTriangle,
            "Dangerous / Low Stock",
            low_stock_count.to_string(),
            "Below 150 pcs / 3-wk lead",
            if low_stock_count > 0 {
                BADGE_RED_FG
            } else {
                BADGE_GREEN_FG
            },
        ),
        (
            IconKind::Calendar,
            "Near-Expiry (<60d)",
            near_expiry_count.to_string(),
            "Monitor batch dates",
            if near_expiry_count > 0 {
                BADGE_AMBER_FG
            } else {
                BADGE_GREEN_FG
            },
        ),
        (
            IconKind::Activity,
            "Today's Patients",
            state.today_patient_count.to_string(),
            "Scheduled treatments",
            ACCENT_BLUE,
        ),
    ];

    if ui.available_width() >= 700.0 {
        ui.columns(4, |cols| {
            for (i, (icon, title, val, sub, accent)) in kpis.iter().enumerate() {
                summary_card(&mut cols[i], *icon, title, val, sub, *accent);
            }
        });
    } else {
        ui.columns(2, |cols| {
            summary_card(
                &mut cols[0],
                kpis[0].0,
                kpis[0].1,
                &kpis[0].2,
                kpis[0].3,
                kpis[0].4,
            );
            summary_card(
                &mut cols[1],
                kpis[1].0,
                kpis[1].1,
                &kpis[1].2,
                kpis[1].3,
                kpis[1].4,
            );
        });
        ui.add_space(8.0);
        ui.columns(2, |cols| {
            summary_card(
                &mut cols[0],
                kpis[2].0,
                kpis[2].1,
                &kpis[2].2,
                kpis[2].3,
                kpis[2].4,
            );
            summary_card(
                &mut cols[1],
                kpis[3].0,
                kpis[3].1,
                &kpis[3].2,
                kpis[3].3,
                kpis[3].4,
            );
        });
    }

    ui.add_space(12.0);

    // Department Stock Health Visualization + Needs Attention Today
    let show_side_by_side = ui.available_width() >= 760.0;
    if show_side_by_side {
        ui.columns(2, |cols| {
            show_department_health_card(&mut cols[0], state);
            show_needs_attention_card(&mut cols[1], state);
        });
    } else {
        show_department_health_card(ui, state);
        ui.add_space(10.0);
        show_needs_attention_card(ui, state);
    }

    ui.add_space(12.0);

    // Supply Stock List
    card_frame().show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    RichText::new("Supply Stock List")
                        .size(15.5)
                        .color(TEXT_SLATE)
                        .strong(),
                );
                ui.label(
                    RichText::new(if state.dash_show_full_numbers {
                        "Full accounting view: Beginning, In, Out, Warehouse + Treatment Area"
                    } else {
                        "Simplified view — toggle 'Show Full Numbers' for accounting breakdown"
                    })
                    .size(12.0)
                    .color(TEXT_MUTED),
                );
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.checkbox(&mut state.dash_show_full_numbers, "Show Full Numbers");
                ui.add_space(8.0);
                ui.add(
                    egui::TextEdit::singleline(&mut state.dash_search)
                        .hint_text("Search item...")
                        .desired_width(170.0),
                );
            });
        });
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            for dept in [
                Department::All,
                Department::Treatment,
                Department::ReverseOsmosis,
                Department::CenterSupplies,
            ] {
                ui.selectable_value(&mut state.dash_dept_filter, dept, dept.label());
            }
        });
        ui.add_space(8.0);

        let search_lower = state.dash_search.trim().to_lowercase();
        let filtered_count = state
            .items
            .iter()
            .filter(|item| {
                (state.dash_dept_filter == Department::All
                    || item.department == state.dash_dept_filter)
                    && (search_lower.is_empty()
                        || item.name.to_lowercase().contains(&search_lower))
            })
            .count();

        if filtered_count == 0 {
            empty_state(
                ui,
                IconKind::Search,
                "No matching items found",
                "Try clearing the search box or switching to All Departments.",
            );
        } else {
            ScrollArea::horizontal()
                .id_salt("dashboard_table_scroll")
                .show(ui, |ui| {
                    egui::Grid::new("ending_inventory_grid")
                        .striped(true)
                        .spacing([16.0, 10.0])
                        .show(ui, |ui| {
                            if state.dash_show_full_numbers {
                                for h in [
                                    "Dept",
                                    "Item Name",
                                    "Beginning",
                                    "Stock-In",
                                    "Stock-Out",
                                    "Ending Balance (Whse + TA)",
                                    "Earliest Expiry",
                                    "Status",
                                ] {
                                    ui.label(RichText::new(h).color(TEXT_MUTED).strong().size(12.0));
                                }
                            } else {
                                for h in ["Item Name", "Available Stock", "Expiry", "Status"] {
                                    ui.label(RichText::new(h).color(TEXT_MUTED).strong().size(12.0));
                                }
                            }
                            ui.end_row();

                            for item in &state.items {
                                if state.dash_dept_filter != Department::All
                                    && item.department != state.dash_dept_filter
                                {
                                    continue;
                                }
                                if !search_lower.is_empty()
                                    && !item.name.to_lowercase().contains(&search_lower)
                                {
                                    continue;
                                }
                                let days = item.days_until_expiry(state.today);
                                if state.dash_show_full_numbers {
                                    ui.label(
                                        RichText::new(item.department.label())
                                            .size(11.5)
                                            .color(TEXT_MUTED),
                                    );
                                    ui.label(RichText::new(&item.name).color(TEXT_SLATE).strong());
                                    ui.label(item.beginning_stock.to_string());
                                    ui.label(
                                        RichText::new(format!("+{}", item.stock_in))
                                            .color(BADGE_GREEN_FG),
                                    );
                                    ui.label(
                                        RichText::new(format!("-{}", item.stock_out))
                                            .color(BADGE_RED_FG),
                                    );
                                    ui.label(
                                        RichText::new(format!(
                                            "{} ({} + {})",
                                            item.ending_balance(),
                                            item.warehouse_qty,
                                            item.treatment_area_qty
                                        ))
                                        .strong()
                                        .color(TEXT_SLATE),
                                    );
                                    ui.label(format!("{} ({}d)", item.earliest_expiry, days));
                                } else {
                                    ui.label(RichText::new(&item.name).color(TEXT_SLATE).strong());
                                    ui.label(
                                        RichText::new(format!("{} pcs", item.ending_balance()))
                                            .strong()
                                            .color(TEXT_SLATE),
                                    );
                                    ui.label(format!("In {} days", days));
                                }

                                ui.horizontal(|ui| {
                                    if item.is_dangerous_stock() {
                                        badge(ui, "Dangerous (<150)", BADGE_RED_BG, BADGE_RED_FG);
                                    } else if item.is_low_stock() {
                                        badge(ui, "Low Stock", BADGE_RED_BG, BADGE_RED_FG);
                                    }
                                    if item.is_near_expiry(state.today) {
                                        badge(ui, "Near Expiry", BADGE_AMBER_BG, BADGE_AMBER_FG);
                                    }
                                    if !item.is_low_stock() && !item.is_near_expiry(state.today) {
                                        badge(ui, "Healthy", BADGE_GREEN_BG, BADGE_GREEN_FG);
                                    }
                                });
                                ui.end_row();
                            }
                        });
                });
        }
    });

    if state.report_modal_open {
        show_report_modal(ctx, state);
    }
}

fn show_department_health_card(ui: &mut Ui, state: &AppState) {
    card_frame().show(ui, |ui| {
        ui.label(
            RichText::new("Department Stock Health")
                .size(15.0)
                .color(TEXT_SLATE)
                .strong(),
        );
        ui.add_space(8.0);
        for dept in [
            Department::Treatment,
            Department::ReverseOsmosis,
            Department::CenterSupplies,
        ] {
            let dept_items: Vec<_> = state.items.iter().filter(|i| i.department == dept).collect();
            let total = dept_items.len().max(1);
            let healthy = dept_items
                .iter()
                .filter(|i| !i.is_low_stock() && !i.is_near_expiry(state.today))
                .count();
            let ratio = healthy as f32 / total as f32;
            ui.horizontal(|ui| {
                ui.label(RichText::new(dept.label()).size(12.5).strong().color(TEXT_SLATE));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("{}/{} healthy", healthy, total))
                            .size(12.0)
                            .color(TEXT_MUTED),
                    );
                });
            });
            ui.add(
                egui::ProgressBar::new(ratio)
                    .fill(if ratio >= 0.9 {
                        BADGE_GREEN_FG
                    } else {
                        ACCENT_BLUE
                    })
                    .desired_height(7.0),
            );
            ui.add_space(6.0);
        }
    });
}

fn show_needs_attention_card(ui: &mut Ui, state: &mut AppState) {
    let alerts = state.build_alerts();
    let (all_count, critical_count, expiring_count) = alert_counts(&alerts);
    let visible = filter_and_sort_alerts(&alerts, state.alert_tab);
    let mut order_to_add: Option<(usize, String)> = None;

    egui::Frame::none()
        .fill(CARD_WHITE)
        .stroke(egui::Stroke::new(1.0_f32, BORDER_SOFT))
        .rounding(egui::Rounding::same(12.0))
        .inner_margin(egui::Margin::same(20.0))
        .show(ui, |ui| {
            ui.label(
                RichText::new("Needs Attention Today")
                    .size(15.5)
                    .color(TEXT_SLATE)
                    .strong(),
            );
            ui.add_space(8.0);

            // Segmented Pill Tabs
            ui.horizontal_wrapped(|ui| {
                for (tab, label) in [
                    (AlertTab::All, format!("All ({})", all_count)),
                    (
                        AlertTab::CriticalStock,
                        format!("Critical Stock ({})", critical_count),
                    ),
                    (
                        AlertTab::ExpiringSoon,
                        format!("Expiring Soon ({})", expiring_count),
                    ),
                ] {
                    let active = state.alert_tab == tab;
                    let fill = if active {
                        ACCENT_BLUE
                    } else {
                        Color32::from_rgb(0xF1, 0xF5, 0xF9)
                    };
                    let fg = if active { Color32::WHITE } else { TEXT_SLATE };
                    let btn = egui::Button::new(RichText::new(label).color(fg).size(12.0).strong())
                        .fill(fill)
                        .rounding(egui::Rounding::same(16.0));
                    if ui.add(btn).clicked() {
                        state.alert_tab = tab;
                    }
                }
            });
            ui.add_space(10.0);

            if visible.is_empty() {
                ui.horizontal(|ui| {
                    draw_icon(ui, IconKind::CheckCircle, 16.0, BADGE_GREEN_FG);
                    ui.label(
                        RichText::new("All supplies are healthy in this category.")
                            .color(BADGE_GREEN_FG)
                            .size(13.0),
                    );
                });
            } else {
                ScrollArea::vertical()
                    .id_salt("needs_attention_scroll")
                    .max_height(360.0)
                    .show(ui, |ui| {
                        for alert in visible {
                            let row_resp = egui::Frame::none()
                                .inner_margin(egui::Margin::symmetric(6.0, 6.0))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        // Left column: Uppercase name + muted subtext
                                        ui.vertical(|ui| {
                                            ui.label(
                                                RichText::new(alert.name.to_uppercase())
                                                    .size(13.0)
                                                    .strong()
                                                    .color(Color32::from_rgb(0x1E, 0x29, 0x3B)),
                                            );
                                            ui.label(
                                                RichText::new(format!(
                                                    "{} • Min: {}",
                                                    alert.department, alert.min_threshold
                                                ))
                                                .size(11.0)
                                                .color(TEXT_MUTED),
                                            );
                                        });

                                        // Right column: Right-aligned consistent pills + + Order
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                if ui
                                                    .add(
                                                        egui::Button::new(
                                                            RichText::new("+ Order")
                                                                .size(11.5)
                                                                .color(ACCENT_BLUE)
                                                                .strong(),
                                                        )
                                                        .fill(ACCENT_SKY),
                                                    )
                                                    .clicked()
                                                {
                                                    order_to_add = Some((
                                                        alert.item_idx,
                                                        alert.name.clone(),
                                                    ));
                                                }
                                                if let Some(days) = alert.days_until_expiry {
                                                    badge(
                                                        ui,
                                                        &format!("Exp: {}d", days),
                                                        BADGE_AMBER_BG,
                                                        BADGE_AMBER_FG,
                                                    );
                                                }
                                                if alert.quantity < alert.min_threshold {
                                                    badge(
                                                        ui,
                                                        &format!("{} left", alert.quantity),
                                                        BADGE_RED_BG,
                                                        BADGE_RED_FG,
                                                    );
                                                }
                                            },
                                        );
                                    });
                                })
                                .response;

                            if row_resp.hovered() {
                                ui.painter().rect_stroke(
                                    row_resp.rect,
                                    6.0,
                                    egui::Stroke::new(1.0_f32, ACCENT_SKY),
                                );
                            }
                            let sep_y = row_resp.rect.bottom();
                            ui.painter().line_segment(
                                [
                                    egui::pos2(row_resp.rect.left(), sep_y),
                                    egui::pos2(row_resp.rect.right(), sep_y),
                                ],
                                egui::Stroke::new(
                                    1.0_f32,
                                    Color32::from_rgb(0xF1, 0xF5, 0xF9),
                                ),
                            );
                        }
                    });
            }
        });

    if let Some((idx, name)) = order_to_add {
        if let Some(entry) = state.kiosk_cart.iter_mut().find(|(i, _)| *i == idx) {
            entry.1 += 1;
        } else {
            state.kiosk_cart.push((idx, 1));
        }
        state.set_banner(format!("Added 1x {} to Kiosk Order Tray.", name), false);
    }
}

fn summary_card(
    ui: &mut Ui,
    icon: IconKind,
    title: &str,
    value: &str,
    sub: &str,
    accent: Color32,
) {
    card_frame().show(ui, |ui| {
        ui.set_min_height(84.0);
        ui.horizontal(|ui| {
            draw_icon(ui, icon, 16.0, accent);
            ui.label(RichText::new(title).size(12.5).color(TEXT_MUTED).strong());
        });
        ui.add_space(4.0);
        ui.label(RichText::new(value).size(24.0).color(accent).strong());
        ui.add_space(2.0);
        ui.label(RichText::new(sub).size(11.5).color(TEXT_MUTED));
    });
}

fn show_report_modal(ctx: &egui::Context, state: &mut AppState) {
    let mut open = state.report_modal_open;
    egui::Window::new("Owner's Daily Ending Report")
        .open(&mut open)
        .collapsible(false)
        .resizable(true)
        .default_width(640.0)
        .show(ctx, |ui| {
            ui.label(
                RichText::new("Sta. Clara Dialysis Center, Inc. — End-of-Day Summary")
                    .size(15.0)
                    .strong()
                    .color(TEXT_SLATE),
            );
            ui.label(
                RichText::new(format!(
                    "Report Date: {}  |  Patients Treated: {}",
                    state.today, state.today_patient_count
                ))
                .size(12.5)
                .color(TEXT_MUTED),
            );
            ui.separator();

            ScrollArea::vertical()
                .id_salt("report_modal_scroll")
                .max_height(260.0)
                .show(ui, |ui| {
                    egui::Grid::new("report_modal_grid")
                        .striped(true)
                        .spacing([14.0, 6.0])
                        .show(ui, |ui| {
                            for h in ["Item", "Beg", "In", "Out", "End Bal", "Expiry"] {
                                ui.label(RichText::new(h).strong().size(12.0));
                            }
                            ui.end_row();
                            for item in &state.items {
                                ui.label(&item.name);
                                ui.label(item.beginning_stock.to_string());
                                ui.label(item.stock_in.to_string());
                                ui.label(item.stock_out.to_string());
                                ui.label(RichText::new(item.ending_balance().to_string()).strong());
                                ui.label(item.earliest_expiry.to_string());
                                ui.end_row();
                            }
                        });
                });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui
                    .button(RichText::new("Export CSV").color(ACCENT_BLUE).strong())
                    .clicked()
                {
                    let csv = state.generate_report_csv();
                    let filename = format!("daily_ending_report_{}.csv", state.today);
                    match std::fs::write(&filename, csv) {
                        Ok(_) => state.set_banner(format!("Exported {}", filename), false),
                        Err(e) => state.set_banner(format!("Export failed: {}", e), true),
                    }
                }
                if ui.button("Copy Summary").clicked() {
                    let csv = state.generate_report_csv();
                    ctx.copy_text(csv);
                    state.set_banner("Report summary copied to clipboard.", false);
                }
            });
        });
    state.report_modal_open = open;
}
