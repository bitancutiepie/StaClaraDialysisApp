use chrono::{Datelike, Local, NaiveDate};
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfirmAction {
    KioskConfirm,
    StockInConfirm,
    AdjustmentConfirm,
    ResetDemo,
    SaveXlsx,
}

#[derive(Debug, Clone)]
pub struct PendingConfirm {
    pub title: String,
    pub summary: String,
    pub action: ConfirmAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExcelSaveMode {
    OverwriteOriginal,
    TimestampedCopy,
}

#[derive(Debug, Clone)]
pub struct ExcelSaveSettings {
    pub mode: ExcelSaveMode,
    pub include_expiry_sheet: bool,
    pub include_daily_sheet: bool,
    pub auto_backup: bool,
}

impl Default for ExcelSaveSettings {
    fn default() -> Self {
        Self {
            mode: ExcelSaveMode::OverwriteOriginal,
            include_expiry_sheet: true,
            include_daily_sheet: true,
            auto_backup: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KioskMode {
    FloorRequisition,
    WarehouseTransfer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertTab {
    All,
    CriticalStock,
    ExpiringSoon,
}

#[derive(Debug, Clone)]
pub struct InventoryAlert {
    pub item_idx: usize,
    pub name: String,
    pub department: String,
    pub quantity: u32,
    pub min_threshold: u32,
    pub days_until_expiry: Option<u32>,
}

impl AppState {
    pub fn build_alerts(&self) -> Vec<InventoryAlert> {
        self.items
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| {
                let qty = item.ending_balance().max(0) as u32;
                let min_thresh = DANGER_STOCK_THRESHOLD.max(item.reorder_threshold) as u32;
                let days = item.days_until_expiry(self.today);
                let exp = if days < 60 {
                    Some(days.max(0) as u32)
                } else {
                    None
                };
                if qty < min_thresh || exp.is_some() {
                    Some(InventoryAlert {
                        item_idx: idx,
                        name: item.name.clone(),
                        department: item.department.label().to_string(),
                        quantity: qty,
                        min_threshold: min_thresh,
                        days_until_expiry: exp,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn alert_counts(alerts: &[InventoryAlert]) -> (usize, usize, usize) {
    let all = alerts.len();
    let critical = alerts
        .iter()
        .filter(|a| a.quantity < a.min_threshold)
        .count();
    let expiring = alerts
        .iter()
        .filter(|a| a.days_until_expiry.is_some())
        .count();
    (all, critical, expiring)
}

pub fn filter_and_sort_alerts(alerts: &[InventoryAlert], tab: AlertTab) -> Vec<&InventoryAlert> {
    let mut list: Vec<&InventoryAlert> = match tab {
        AlertTab::All => alerts.iter().collect(),
        AlertTab::CriticalStock => alerts
            .iter()
            .filter(|a| a.quantity < a.min_threshold)
            .collect(),
        AlertTab::ExpiringSoon => alerts
            .iter()
            .filter(|a| a.days_until_expiry.is_some())
            .collect(),
    };
    match tab {
        AlertTab::ExpiringSoon => {
            list.sort_by_key(|a| a.days_until_expiry.unwrap_or(u32::MAX));
        }
        AlertTab::CriticalStock => {
            list.sort_by_key(|a| a.quantity);
        }
        AlertTab::All => {}
    }
    list
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavView {
    Home,
    Dashboard,
    StockIn,
    Requisition,
    Adjustment,
    TechLogs,
    MonthlyConsumption,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Department {
    All,
    Treatment,
    ReverseOsmosis,
    CenterSupplies,
}

impl Department {
    pub fn label(&self) -> &'static str {
        match self {
            Department::All => "All Departments",
            Department::Treatment => "TREATMENT",
            Department::ReverseOsmosis => "Reverse Osmosis",
            Department::CenterSupplies => "CENTER SUPPLIES",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitType {
    Pcs,
    Box,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShiftType {
    Mwf,
    Tths,
}

impl ShiftType {
    pub fn label(&self) -> &'static str {
        match self {
            ShiftType::Mwf => "MWF (Mon / Wed / Fri)",
            ShiftType::Tths => "TThS (Tue / Thu / Sat)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdjustmentReason {
    Damaged,
    Expired,
    Borrowed,
}

impl AdjustmentReason {
    pub fn label(&self) -> &'static str {
        match self {
            AdjustmentReason::Damaged => "Damaged",
            AdjustmentReason::Expired => "Expired",
            AdjustmentReason::Borrowed => "Borrowed",
        }
    }
}

#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub id: usize,
    pub department: Department,
    pub name: String,
    pub pcs_per_box: i32,
    pub beginning_stock: i32,
    pub stock_in: i32,
    pub stock_out: i32,
    pub warehouse_qty: i32,
    pub treatment_area_qty: i32,
    pub earliest_expiry: NaiveDate,
    pub reorder_threshold: i32,
    pub daily_consumption: [i32; 31],
}

pub const DANGER_STOCK_THRESHOLD: i32 = 150;

impl InventoryItem {
    pub fn ending_balance(&self) -> i32 {
        self.warehouse_qty + self.treatment_area_qty
    }

    pub fn is_dangerous_stock(&self) -> bool {
        self.ending_balance() < DANGER_STOCK_THRESHOLD
    }

    pub fn is_low_stock(&self) -> bool {
        self.ending_balance() <= self.reorder_threshold || self.is_dangerous_stock()
    }

    pub fn days_until_expiry(&self, today: NaiveDate) -> i64 {
        (self.earliest_expiry - today).num_days()
    }

    pub fn is_near_expiry(&self, today: NaiveDate) -> bool {
        self.days_until_expiry(today) < 60
    }

    pub fn monthly_total(&self) -> i32 {
        self.daily_consumption.iter().sum()
    }
}

#[derive(Debug, Clone)]
pub struct StockInRecord {
    pub timestamp: String,
    pub supplier: String,
    pub item_name: String,
    pub qty_pcs: i32,
    pub unit_label: String,
    pub batch_no: String,
    pub expiry: NaiveDate,
}

#[derive(Debug, Clone)]
pub struct RequisitionRecord {
    pub timestamp: String,
    pub shift: ShiftType,
    pub patient_count: i32,
    pub summary: String,
}

#[derive(Debug, Clone)]
pub struct AdjustmentRecord {
    pub timestamp: String,
    pub item_name: String,
    pub qty: i32,
    pub reason: AdjustmentReason,
    pub note: String,
}

#[derive(Debug, Clone)]
pub struct DialyzerReusePatient {
    pub patient_id: String,
    pub patient_name: String,
    pub reuse_count: u32,
    pub max_reuse: u32,
}

#[derive(Debug, Clone)]
pub struct RoWaterCheck {
    pub timestamp: String,
    pub hardness_pass: bool,
    pub chlorine_pass: bool,
    pub peroxide_pass: bool,
    pub technician: String,
}

#[derive(Debug, Clone)]
pub struct BannerMessage {
    pub text: String,
    pub is_error: bool,
}

pub struct AppState {
    pub active_view: NavView,
    pub today: NaiveDate,
    pub today_patient_count: i32,
    pub items: Vec<InventoryItem>,
    pub stock_in_logs: Vec<StockInRecord>,
    pub requisition_logs: Vec<RequisitionRecord>,
    pub adjustment_logs: Vec<AdjustmentRecord>,
    pub dialyzer_patients: Vec<DialyzerReusePatient>,
    pub ro_water_logs: Vec<RoWaterCheck>,
    pub report_modal_open: bool,
    pub excel_modal_open: bool,
    pub excel_modal_tab: usize,
    pub excel_save_settings: ExcelSaveSettings,
    pub pending_confirm: Option<PendingConfirm>,
    pub banner: Option<BannerMessage>,

    // Dashboard & Monthly Consumption filters / simplicity toggles
    pub dash_dept_filter: Department,
    pub dash_search: String,
    pub dash_show_full_numbers: bool,
    pub alert_tab: AlertTab,
    pub mc_dept_filter: Department,
    pub mc_show_calendar: bool,

    // Form states
    pub si_selected_item: usize,
    pub si_qty: i32,
    pub si_unit: UnitType,
    pub si_batch: String,
    pub si_expiry_str: String,

    pub req_shift: ShiftType,
    pub req_patients: i32,
    pub req_dialyzer_choice: usize, // 0 = None (Reuse), 1..4 = Dialyzer types
    pub req_dialyzer_qty: i32,
    pub req_extra_item: usize,
    pub req_extra_qty: i32,
    pub kiosk_cart: Vec<(usize, i32)>,
    pub kiosk_dept_filter: Department,
    pub kiosk_mode: KioskMode,
    pub kiosk_search: String,
    pub kiosk_station: String,

    pub tr_selected_item: usize,
    pub tr_qty: i32,

    pub adj_selected_item: usize,
    pub adj_qty: i32,
    pub adj_reason: AdjustmentReason,
    pub adj_note: String,

    pub tech_tab: usize,
    pub ro_hardness_pass: bool,
    pub ro_chlorine_pass: bool,
    pub ro_peroxide_pass: bool,
    pub ro_technician: String,
}

fn make_item(
    id: usize,
    department: Department,
    name: &str,
    pcs_per_box: i32,
    whse: i32,
    ta: i32,
    reorder: i32,
    exp_days: i64,
    today: NaiveDate,
    sample_daily: i32,
) -> InventoryItem {
    let mut daily_consumption = [0i32; 31];
    let current_day = (today.day() as usize).min(31);
    for d in 0..current_day.saturating_sub(1) {
        //Realistic alternating MWF / TThS pattern from sample Excel
        if d % 7 != 6 {
            daily_consumption[d] = if d % 2 == 0 {
                sample_daily
            } else {
                (sample_daily * 3 / 4).max(1)
            };
        }
    }
    InventoryItem {
        id,
        department,
        name: name.to_string(),
        pcs_per_box,
        beginning_stock: whse + ta,
        stock_in: 0,
        stock_out: 0,
        warehouse_qty: whse,
        treatment_area_qty: ta,
        earliest_expiry: today + chrono::Duration::days(exp_days),
        reorder_threshold: reorder,
        daily_consumption,
    }
}

impl AppState {
    pub fn new_demo() -> Self {
        let today = Local::now().date_naive();
        let catalog = [
            // TREATMENT (0..16)
            (Department::Treatment, "BLOODLINES", 24, 96, 24, 40, 120, 22),
            (Department::Treatment, "RENAL KIT", 20, 22, 8, 35, 95, 14), // Low stock!
            (Department::Treatment, "SUBCLAVIAN KIT", 10, 28, 6, 15, 150, 8),
            (Department::Treatment, "NSS/SALINE", 24, 120, 36, 60, 180, 32),
            (Department::Treatment, "NEEDLE G16", 50, 110, 30, 50, 210, 18),
            (Department::Treatment, "NEEDLE G17", 50, 95, 25, 50, 210, 14),
            (Department::Treatment, "5cc SYRINGE", 100, 160, 40, 80, 190, 8),
            (Department::Treatment, "10CC SYRINGE", 100, 180, 60, 100, 180, 52),
            (Department::Treatment, "4X4 GAUZE", 24, 48, 16, 50, 45, 32), // Near-expiry (<60d)!
            (Department::Treatment, "MICROPORE TAPE", 12, 36, 12, 20, 140, 7),
            (Department::Treatment, "HEPARIN", 10, 24, 10, 18, 38, 9), // Near-expiry (<60d)!
            (Department::Treatment, "EPOETIN ALFA (PRONIVEL)", 10, 30, 8, 15, 75, 12),
            (Department::Treatment, "EPOETIN BETA (RECORMON)", 10, 22, 6, 15, 85, 6),
            (Department::Treatment, "IRON SUCROSE (FEROSE)", 10, 26, 8, 15, 110, 8),
            (Department::Treatment, "D50'50 VIAL", 20, 40, 10, 20, 160, 4),
            (Department::Treatment, "PERACETIC ACID TEST PAPER", 50, 80, 20, 30, 130, 5),
            // Reverse Osmosis (16..30)
            (Department::ReverseOsmosis, "WATER HARDNESS TEST STRIPS", 50, 60, 15, 25, 200, 2),
            (Department::ReverseOsmosis, "COLD STERILANT", 4, 16, 4, 8, 150, 2),
            (Department::ReverseOsmosis, "RESIDUAL PEROXIDE TEST PAPER", 50, 55, 15, 25, 180, 2),
            (Department::ReverseOsmosis, "RESIDUAL CHLORINE TEST STRIPS", 50, 50, 15, 25, 180, 2),
            (Department::ReverseOsmosis, "SODIUM (DIACHEM) 24 PER BOX", 24, 72, 24, 48, 240, 12),
            (Department::ReverseOsmosis, "ACID CONCENTRATE", 4, 24, 8, 16, 220, 6),
            (Department::ReverseOsmosis, "CITRIC ACID", 10, 30, 10, 15, 300, 3),
            (Department::ReverseOsmosis, "INDUSTRIAL SALT", 10, 20, 5, 12, 365, 2),
            (Department::ReverseOsmosis, "SEDIMENT FILTER SL20 MICRON", 12, 18, 4, 10, 365, 1),
            (Department::ReverseOsmosis, "DIALYZER", 24, 48, 12, 24, 180, 6),
            (Department::ReverseOsmosis, "AMICAL (LOW FLUX)", 24, 36, 8, 20, 160, 4),
            (Department::ReverseOsmosis, "NIPRO (HI-FLUX)", 24, 30, 8, 20, 170, 4),
            (Department::ReverseOsmosis, "SURGITECH (HI-FLUX)", 24, 24, 6, 20, 170, 3),
            (Department::ReverseOsmosis, "DIALYSIS FLUID FILTER", 6, 10, 2, 6, 240, 1),
            // CENTER SUPPLIES (30..36)
            (Department::CenterSupplies, "LEGAL SIZE BOND PAPER", 5, 15, 3, 5, 365, 1),
            (Department::CenterSupplies, "A4 BOND PAPER", 5, 18, 4, 5, 365, 1),
            (Department::CenterSupplies, "SMALL GLOVES", 100, 200, 50, 80, 240, 15),
            (Department::CenterSupplies, "MEDIUM GLOVES", 100, 240, 60, 80, 240, 20),
            (Department::CenterSupplies, "XL GLOVES (BLACK)", 100, 140, 30, 60, 240, 10),
            (Department::CenterSupplies, "XL GLOVES (WHITE)", 100, 150, 30, 60, 240, 10),
        ];

        let items = catalog
            .iter()
            .enumerate()
            .map(|(id, &(dept, name, ppb, whse, ta, reorder, exp, daily))| {
                make_item(id, dept, name, ppb, whse, ta, reorder, exp, today, daily)
            })
            .collect();

        let dialyzer_patients = vec![
            DialyzerReusePatient {
                patient_id: "P-101".to_string(),
                patient_name: "Dela Cruz, Juan".to_string(),
                reuse_count: 3,
                max_reuse: 12,
            },
            DialyzerReusePatient {
                patient_id: "P-102".to_string(),
                patient_name: "Santos, Maria".to_string(),
                reuse_count: 7,
                max_reuse: 12,
            },
            DialyzerReusePatient {
                patient_id: "P-103".to_string(),
                patient_name: "Reyes, Ricardo".to_string(),
                reuse_count: 11,
                max_reuse: 12,
            },
            DialyzerReusePatient {
                patient_id: "P-104".to_string(),
                patient_name: "Mendoza, Elena".to_string(),
                reuse_count: 1,
                max_reuse: 12,
            },
        ];

        let ro_water_logs = vec![RoWaterCheck {
            timestamp: "06:30 AM".to_string(),
            hardness_pass: true,
            chlorine_pass: true,
            peroxide_pass: true,
            technician: "Tech. Marco".to_string(),
        }];

        Self {
            active_view: NavView::Home,
            today,
            today_patient_count: 12,
            items,
            stock_in_logs: Vec::new(),
            requisition_logs: Vec::new(),
            adjustment_logs: Vec::new(),
            dialyzer_patients,
            ro_water_logs,
            report_modal_open: false,
            excel_modal_open: false,
            excel_modal_tab: 0,
            excel_save_settings: ExcelSaveSettings::default(),
            pending_confirm: None,
            banner: None,

            dash_dept_filter: Department::All,
            dash_search: String::new(),
            dash_show_full_numbers: false,
            alert_tab: AlertTab::All,
            mc_dept_filter: Department::All,
            mc_show_calendar: false,

            si_selected_item: 0,
            si_qty: 1,
            si_unit: UnitType::Box,
            si_batch: "HKC-2026-089".to_string(),
            si_expiry_str: (today + chrono::Duration::days(365))
                .format("%Y-%m-%d")
                .to_string(),

            req_shift: ShiftType::Mwf,
            req_patients: 4,
            req_dialyzer_choice: 0,
            req_dialyzer_qty: 1,
            req_extra_item: 3,
            req_extra_qty: 0,
            kiosk_cart: Vec::new(),
            kiosk_dept_filter: Department::Treatment,
            kiosk_mode: KioskMode::FloorRequisition,
            kiosk_search: String::new(),
            kiosk_station: "Station 1-4".to_string(),

            tr_selected_item: 0,
            tr_qty: 10,

            adj_selected_item: 8,
            adj_qty: 1,
            adj_reason: AdjustmentReason::Damaged,
            adj_note: String::new(),

            tech_tab: 0,
            ro_hardness_pass: true,
            ro_chlorine_pass: true,
            ro_peroxide_pass: true,
            ro_technician: "Tech. Marco".to_string(),
        }
    }

    pub fn set_banner(&mut self, text: impl Into<String>, is_error: bool) {
        self.banner = Some(BannerMessage {
            text: text.into(),
            is_error,
        });
    }

    pub fn transfer_to_treatment_area(&mut self, item_idx: usize, qty: i32) -> Result<(), String> {
        if qty <= 0 {
            return Err("Transfer quantity must be greater than 0.".to_string());
        }
        let item = self
            .items
            .get_mut(item_idx)
            .ok_or_else(|| "Invalid item".to_string())?;
        if qty > item.warehouse_qty {
            return Err(format!(
                "Cannot transfer {} pcs of {}: only {} pcs in Warehouse.",
                qty, item.name, item.warehouse_qty
            ));
        }
        item.warehouse_qty -= qty;
        item.treatment_area_qty += qty;
        Ok(())
    }

    pub fn deduct_item(&mut self, item_idx: usize, qty: i32) -> Result<(), String> {
        if qty <= 0 {
            return Ok(());
        }
        let day_idx = (self.today.day() as usize).saturating_sub(1).min(30);
        let item = self
            .items
            .get_mut(item_idx)
            .ok_or_else(|| "Invalid item".to_string())?;
        let avail = item.ending_balance();
        if qty > avail {
            return Err(format!(
                "Insufficient stock for {}: requested {}, available {}",
                item.name, qty, avail
            ));
        }
        let from_ta = qty.min(item.treatment_area_qty);
        item.treatment_area_qty -= from_ta;
        let rem = qty - from_ta;
        item.warehouse_qty -= rem;
        item.stock_out += qty;
        item.daily_consumption[day_idx] += qty;
        Ok(())
    }

    pub fn generate_report_csv(&self) -> String {
        let mut out = String::with_capacity(self.items.len() * 96 + 128);
        out.push_str(
            "Department,Item Name,Beginning Stock,Stock-In,Stock-Out,Warehouse,Treatment Area,Ending Balance,Earliest Expiry,Status\n",
        );
        for item in &self.items {
            let status = if item.is_low_stock() && item.is_near_expiry(self.today) {
                "Low Stock & Near Expiry"
            } else if item.is_low_stock() {
                "Low Stock (3-Wk Lead)"
            } else if item.is_near_expiry(self.today) {
                "Near Expiry (<60d)"
            } else {
                "Healthy"
            };
            let _ = writeln!(
                out,
                "{},{},{},{},{},{},{},{},{},{}",
                item.department.label(),
                item.name,
                item.beginning_stock,
                item.stock_in,
                item.stock_out,
                item.warehouse_qty,
                item.treatment_area_qty,
                item.ending_balance(),
                item.earliest_expiry,
                status
            );
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deduct_prevents_negative_stock() {
        let mut state = AppState::new_demo();
        let avail = state.items[1].ending_balance(); // RENAL KIT = 30
        let res = state.deduct_item(1, avail + 5);
        assert!(res.is_err());
        assert_eq!(state.items[1].ending_balance(), avail);
    }

    #[test]
    fn test_deduct_updates_ending_balance_and_daily_consumption() {
        let mut state = AppState::new_demo();
        let before = state.items[0].ending_balance(); // BLOODLINES = 120
        let day_idx = (state.today.day() as usize).saturating_sub(1).min(30);
        let daily_before = state.items[0].daily_consumption[day_idx];
        assert!(state.deduct_item(0, 10).is_ok());
        assert_eq!(state.items[0].ending_balance(), before - 10);
        assert_eq!(state.items[0].stock_out, 10);
        assert_eq!(state.items[0].daily_consumption[day_idx], daily_before + 10);
    }

    #[test]
    fn test_transfer_to_treatment_area_prevents_over_transfer() {
        let mut state = AppState::new_demo();
        let whse = state.items[0].warehouse_qty;
        let ta = state.items[0].treatment_area_qty;
        assert!(state.transfer_to_treatment_area(0, whse + 1).is_err());
        assert!(state.transfer_to_treatment_area(0, 10).is_ok());
        assert_eq!(state.items[0].warehouse_qty, whse - 10);
        assert_eq!(state.items[0].treatment_area_qty, ta + 10);
    }

    #[test]
    fn test_report_csv_contains_headers_and_items() {
        let state = AppState::new_demo();
        let csv = state.generate_report_csv();
        assert!(csv.starts_with("Department,Item Name,Beginning Stock"));
        assert!(csv.contains("BLOODLINES"));
        assert!(csv.contains("RENAL KIT"));
    }

    #[test]
    fn test_below_150_is_dangerous_stock() {
        let mut state = AppState::new_demo();
        // BLOODLINES starts at 120 (< 150) -> dangerous
        assert!(state.items[0].is_dangerous_stock());
        assert!(state.items[0].is_low_stock());
        // 10CC SYRINGE starts at 240 (>= 150) -> not dangerous
        assert!(!state.items[7].is_dangerous_stock());
        // Deduct 91 so 240 - 91 = 149 (< 150) -> becomes dangerous
        assert!(state.deduct_item(7, 91).is_ok());
        assert!(state.items[7].is_dangerous_stock());
    }

    #[test]
    fn test_alert_tab_filtering_and_sorting() {
        let state = AppState::new_demo();
        let alerts = state.build_alerts();
        let (all, crit, exp) = alert_counts(&alerts);
        assert!(all >= crit && exp == 2);

        let exp_sorted = filter_and_sort_alerts(&alerts, AlertTab::ExpiringSoon);
        assert_eq!(exp_sorted.len(), 2);
        assert_eq!(exp_sorted[0].name, "HEPARIN"); // 38d before 45d
        assert_eq!(exp_sorted[1].name, "4X4 GAUZE");

        let crit_sorted = filter_and_sort_alerts(&alerts, AlertTab::CriticalStock);
        assert!(crit_sorted[0].quantity <= crit_sorted[1].quantity);
    }
}
