use crate::models::{AppState, ExcelSaveMode};
use crate::ui::theme::*;
use chrono::{Datelike, Local, NaiveDate};
use egui::{Color32, RichText, ScrollArea};
use std::path::{Path, PathBuf};

pub const DEFAULT_XLSX_PATH: &str = r"sample_excel\Inventory-NOV-16-23 (1).xlsx";

pub fn load_from_xlsx(path: &str, state: &mut AppState) -> Result<usize, String> {
    let p = Path::new(path);
    if !p.exists() {
        return Err(format!("Excel file not found: {}", path));
    }
    let book = umya_spreadsheet::reader::xlsx::read(p)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;

    let mut updated = 0usize;

    if let Some(ws) = book.get_sheet_by_name("November 2026") {
        // Rows 6..=42 have item names in col 2 (B) and days 1..=31 in cols 3..=33 (C..AG)
        for r in 6..=42u32 {
            let name = ws.get_value((2, r));
            let name_trimmed = name.trim();
            if name_trimmed.is_empty() {
                continue;
            }
            if let Some(item) = state
                .items
                .iter_mut()
                .find(|i| i.name.eq_ignore_ascii_case(name_trimmed))
            {
                for d in 0..31u32 {
                    let val_str = ws.get_value((3 + d, r));
                    if let Ok(v) = val_str.trim().parse::<i32>() {
                        item.daily_consumption[d as usize] = v;
                    }
                }
                if let Ok(exp) = NaiveDate::parse_from_str(ws.get_value((35, r)).trim(), "%Y-%m-%d")
                {
                    item.earliest_expiry = exp;
                }
                updated += 1;
            }
        }
    }

    // Also restore expiry & stock balances if "Expiry & Stock" sheet exists
    if let Some(ws) = book.get_sheet_by_name("Expiry & Stock") {
        let mut r = 2u32;
        loop {
            let name = ws.get_value((2, r));
            if name.trim().is_empty() {
                break;
            }
            if let Some(item) = state
                .items
                .iter_mut()
                .find(|i| i.name.eq_ignore_ascii_case(name.trim()))
            {
                if let Ok(whse) = ws.get_value((3, r)).trim().parse::<i32>() {
                    item.warehouse_qty = whse;
                }
                if let Ok(ta) = ws.get_value((4, r)).trim().parse::<i32>() {
                    item.treatment_area_qty = ta;
                }
                if let Ok(exp) = NaiveDate::parse_from_str(ws.get_value((6, r)).trim(), "%Y-%m-%d")
                {
                    item.earliest_expiry = exp;
                }
            }
            r += 1;
        }
    }

    Ok(updated)
}

pub fn resolve_output_path(base_path: &str, mode: ExcelSaveMode, stamp: &str) -> PathBuf {
    match mode {
        ExcelSaveMode::OverwriteOriginal => PathBuf::from(base_path),
        ExcelSaveMode::TimestampedCopy => {
            let p = Path::new(base_path);
            let stem = p
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Inventory");
            let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("xlsx");
            let parent = p.parent().unwrap_or_else(|| Path::new("."));
            parent.join(format!("{}_{}.{}", stem, stamp, ext))
        }
    }
}

pub fn save_to_xlsx(path: &str, state: &AppState) -> Result<String, String> {
    let settings = &state.excel_save_settings;
    let base = Path::new(path);
    let mut book = if base.exists() {
        umya_spreadsheet::reader::xlsx::read(base)
            .map_err(|e| format!("Failed to open {}: {}", path, e))?
    } else {
        umya_spreadsheet::new_file()
    };

    // Update "November 2026" daily consumption cells + Earliest Expiry column (col 35) in-place
    if settings.include_daily_sheet {
        if let Some(ws) = book.get_sheet_by_name_mut("November 2026") {
            ws.get_cell_mut((35, 5)).set_value("Earliest Expiry");
            for r in 6..=42u32 {
                let name = ws.get_value((2, r));
                let name_trimmed = name.trim();
                if let Some(item) = state
                    .items
                    .iter()
                    .find(|i| i.name.eq_ignore_ascii_case(name_trimmed))
                {
                    for d in 0..31u32 {
                        let val = item.daily_consumption[d as usize];
                        if val > 0 {
                            ws.get_cell_mut((3 + d, r)).set_value_number(val);
                        }
                    }
                    ws.get_cell_mut((35, r))
                        .set_value(item.earliest_expiry.format("%Y-%m-%d").to_string());
                }
            }
        }
    }

    // Create or update "Expiry & Stock" sheet
    if settings.include_expiry_sheet {
        if book.get_sheet_by_name("Expiry & Stock").is_none() {
            let _ = book.new_sheet("Expiry & Stock");
        }
        if let Some(ws) = book.get_sheet_by_name_mut("Expiry & Stock") {
            let headers = [
                "Department",
                "Item Name",
                "Warehouse",
                "Treatment Area",
                "Ending Balance",
                "Earliest Expiry",
                "Days Left",
                "Status",
            ];
            for (c, h) in headers.iter().enumerate() {
                ws.get_cell_mut((c as u32 + 1, 1)).set_value(*h);
            }
            for (idx, item) in state.items.iter().enumerate() {
                let r = idx as u32 + 2;
                let days = item.days_until_expiry(state.today);
                let status = if item.is_low_stock() && item.is_near_expiry(state.today) {
                    "Low Stock & Near Expiry"
                } else if item.is_low_stock() {
                    "Low Stock (3-Wk Lead)"
                } else if item.is_near_expiry(state.today) {
                    "Near Expiry (<60d)"
                } else {
                    "Healthy"
                };
                ws.get_cell_mut((1, r)).set_value(item.department.label());
                ws.get_cell_mut((2, r)).set_value(&item.name);
                ws.get_cell_mut((3, r)).set_value_number(item.warehouse_qty);
                ws.get_cell_mut((4, r))
                    .set_value_number(item.treatment_area_qty);
                ws.get_cell_mut((5, r))
                    .set_value_number(item.ending_balance());
                ws.get_cell_mut((6, r))
                    .set_value(item.earliest_expiry.format("%Y-%m-%d").to_string());
                ws.get_cell_mut((7, r)).set_value_number(days as i32);
                ws.get_cell_mut((8, r)).set_value(status);
            }
        }
    }

    let stamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let out_path = resolve_output_path(path, settings.mode, &stamp);

    if settings.mode == ExcelSaveMode::OverwriteOriginal && settings.auto_backup && out_path.exists()
    {
        let bak_path = resolve_output_path(path, ExcelSaveMode::TimestampedCopy, "bak");
        let _ = std::fs::copy(&out_path, bak_path);
    }

    umya_spreadsheet::writer::xlsx::write(&book, &out_path)
        .map_err(|e| format!("Failed to save {}: {}", out_path.display(), e))?;
    Ok(out_path.display().to_string())
}

pub fn show_excel_modal(ctx: &egui::Context, state: &mut AppState) {
    let mut open = state.excel_modal_open;
    egui::Window::new("Interactive Excel View — Inventory-NOV-16-23 (1).xlsx")
        .open(&mut open)
        .collapsible(false)
        .resizable(true)
        .default_width(820.0)
        .default_height(520.0)
        .show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.selectable_value(&mut state.excel_modal_tab, 0, "Sheet: Expiry & Stock");
                ui.selectable_value(&mut state.excel_modal_tab, 1, "Sheet: November 2026 (Daily)");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("Save to .xlsx")
                                    .color(Color32::WHITE)
                                    .strong(),
                            )
                            .fill(ACCENT_BLUE),
                        )
                        .clicked()
                    {
                        match save_to_xlsx(DEFAULT_XLSX_PATH, state) {
                            Ok(saved_path) => {
                                state.set_banner(format!("Saved to {}", saved_path), false)
                            }
                            Err(e) => state.set_banner(e, true),
                        }
                    }
                    if ui.button("Reload .xlsx").clicked() {
                        match load_from_xlsx(DEFAULT_XLSX_PATH, state) {
                            Ok(n) => state.set_banner(
                                format!("Reloaded {} items from {}", n, DEFAULT_XLSX_PATH),
                                false,
                            ),
                            Err(e) => state.set_banner(e, true),
                        }
                    }
                });
            });
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Save Mode:").strong().size(12.0));
                ui.radio_value(
                    &mut state.excel_save_settings.mode,
                    ExcelSaveMode::OverwriteOriginal,
                    "Overwrite Original",
                );
                ui.radio_value(
                    &mut state.excel_save_settings.mode,
                    ExcelSaveMode::TimestampedCopy,
                    "Timestamped Copy",
                );
                ui.separator();
                ui.checkbox(
                    &mut state.excel_save_settings.include_expiry_sheet,
                    "Expiry & Stock",
                );
                ui.checkbox(
                    &mut state.excel_save_settings.include_daily_sheet,
                    "Monthly Daily",
                );
                if state.excel_save_settings.mode == ExcelSaveMode::OverwriteOriginal {
                    ui.checkbox(
                        &mut state.excel_save_settings.auto_backup,
                        "Auto-Backup (.bak)",
                    );
                }
            });
            ui.separator();

            if state.excel_modal_tab == 0 {
                ScrollArea::both()
                    .id_salt("excel_modal_expiry_scroll")
                    .max_height(400.0)
                    .show(ui, |ui| {
                        egui::Grid::new("excel_modal_expiry_grid")
                            .striped(true)
                            .spacing([14.0, 8.0])
                            .show(ui, |ui| {
                                for h in [
                                    "Department",
                                    "Item Name",
                                    "Warehouse",
                                    "Treatment Area",
                                    "Ending Bal",
                                    "Days Left",
                                    "Status",
                                ] {
                                    ui.label(RichText::new(h).strong().color(TEXT_MUTED).size(12.0));
                                }
                                ui.end_row();

                                let today = state.today;
                                for item in &mut state.items {
                                    ui.label(
                                        RichText::new(item.department.label())
                                            .size(11.5)
                                            .color(TEXT_MUTED),
                                    );
                                    ui.label(RichText::new(&item.name).strong().size(12.0));
                                    ui.add(
                                        egui::DragValue::new(&mut item.warehouse_qty)
                                            .range(0..=9999),
                                    );
                                    ui.add(
                                        egui::DragValue::new(&mut item.treatment_area_qty)
                                            .range(0..=9999),
                                    );
                                    ui.label(
                                        RichText::new(item.ending_balance().to_string()).strong(),
                                    );
                                    let days = item.days_until_expiry(today);
                                    ui.label(format!("{}d ({})", days, item.earliest_expiry));
                                    ui.horizontal(|ui| {
                                        if item.is_dangerous_stock() {
                                            badge(
                                                ui,
                                                "Dangerous (<150)",
                                                BADGE_RED_BG,
                                                BADGE_RED_FG,
                                            );
                                        } else if item.is_low_stock() {
                                            badge(ui, "Low Stock", BADGE_RED_BG, BADGE_RED_FG);
                                        }
                                        if item.is_near_expiry(today) {
                                            badge(
                                                ui,
                                                "Near Expiry",
                                                BADGE_AMBER_BG,
                                                BADGE_AMBER_FG,
                                            );
                                        }
                                        if !item.is_low_stock() && !item.is_near_expiry(today) {
                                            badge(ui, "Healthy", BADGE_GREEN_BG, BADGE_GREEN_FG);
                                        }
                                    });
                                    ui.end_row();
                                }
                            });
                    });
            } else {
                let today_day = state.today.day() as usize;
                ScrollArea::both()
                    .id_salt("excel_modal_daily_scroll")
                    .max_height(400.0)
                    .show(ui, |ui| {
                        egui::Grid::new("excel_modal_daily_grid")
                            .striped(true)
                            .spacing([8.0, 6.0])
                            .show(ui, |ui| {
                                ui.label(RichText::new("Item Name").strong().size(12.0));
                                for d in 1..=31 {
                                    let color =
                                        if d == today_day { ACCENT_BLUE } else { TEXT_MUTED };
                                    ui.label(RichText::new(format!("D{}", d)).color(color).strong().size(11.0));
                                }
                                ui.label(RichText::new("Total").strong().size(12.0));
                                ui.end_row();

                                for item in &mut state.items {
                                    ui.label(RichText::new(&item.name).strong().size(11.5));
                                    for val in &mut item.daily_consumption {
                                        ui.add(egui::DragValue::new(val).range(0..=999));
                                    }
                                    ui.label(
                                        RichText::new(item.monthly_total().to_string())
                                            .strong()
                                            .color(ACCENT_BLUE),
                                    );
                                    ui.end_row();
                                }
                            });
                    });
            }
        });
    state.excel_modal_open = open;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_excel_output_path_modes() {
        let p1 = resolve_output_path(
            "sample_excel/Inventory.xlsx",
            ExcelSaveMode::OverwriteOriginal,
            "20261125_180000",
        );
        assert_eq!(p1, PathBuf::from("sample_excel/Inventory.xlsx"));

        let p2 = resolve_output_path(
            "sample_excel/Inventory.xlsx",
            ExcelSaveMode::TimestampedCopy,
            "20261125_180000",
        );
        assert_eq!(
            p2,
            PathBuf::from("sample_excel/Inventory_20261125_180000.xlsx")
        );
    }
}
