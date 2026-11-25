# Sta. Clara Dialysis Center, Inc. — Inventory & Daily Ending Balance Prototype Design

## 1. Intent & Why
A minimal, local-first desktop prototype in Rust for **Sta. Clara Dialysis Center, Inc.** to demonstrate the end-to-end daily inventory workflow—from supplier deliveries (**Helande Kidney Care**) and **Nurse Requisitions** (1-Click Hemodialysis Treatment Package) through **Stock Adjustment/Wastage** and **Technician Logs** (Dialyzer Reuse & RO Water), culminating in the **Owner's Daily Ending Report**.

## 2. Goals & Non-Goals
- **Goals:**
  - Single-command runnable (`cargo run`) with zero external cloud or Node/WebView dependencies.
  - Calming pastel-blue hospital theme (`#F0F7FF` background, `#FFFFFF` cards, `#7CC4FA` / `#3B82F6` accents, `#1E293B` slate text, soft red/amber alert badges).
  - Left sidebar navigation across 5 views: Dashboard, Stock-In, Nurse Requisition, Stock Adjustment/Wastage, and Technician Logs.
  - Pre-loaded realistic sample data (Syringe 10cc, 4x4 Gauze, Sodium Bicarbonate, Bloodline, Renal Kit, Heparin) with a "Reset Demo Data" button.
  - Transparent `Ending Balance = Warehouse + Treatment Area` calculation and negative-stock prevention.
  - Owner's Daily Ending Report modal with "Export CSV" and "Copy Summary" actions.
- **Non-Goals:**
  - Multi-user authentication, cloud sync, or persistent SQLite database (in-memory store for clean repeatable demos).

## 3. Approaches Considered
1. **egui (`eframe`) — Selected:** Pure Rust immediate-mode GUI. Compiles fastest, zero external runtime dependencies, single `cargo run` command.
2. **Tauri + HTML/Tailwind CSS:** Richer CSS styling, but slower first compile and requires Node.js + WebView2 toolchain.

## 4. Architecture & Data Model
- `src/models.rs`:
  - `InventoryItem`: `id`, `name`, `unit`, `pcs_per_box`, `beginning_stock`, `stock_in`, `stock_out`, `warehouse_qty`, `treatment_area_qty`, `earliest_expiry` (days/date), `reorder_threshold` (3-week lead-time), `batches`.
  - `StockInRecord`, `RequisitionRecord`, `AdjustmentRecord`, `DialyzerReusePatient`, `RoWaterCheck`.
  - `AppState`: holds all collections, `today_patient_count`, `active_view`, `report_modal_open`, and `status_banner`.
- `src/ui/`:
  - `theme.rs`: Pastel-blue `egui::Visuals` & custom card/badge helpers.
  - `dashboard.rs`: 4 summary cards, Ending Inventory Table, Daily Ending Report modal (CSV export + clipboard copy).
  - `stock_in.rs`: Helande Kidney Care delivery form with Box/Pc auto-conversion.
  - `requisition.rs`: MWF/TThS shift selector, 1-Click Hemodialysis Treatment Package button, optional extras, negative-stock validation.
  - `adjustment.rs`: Wastage/Damaged/Expired/Borrowed form.
  - `tech_logs.rs`: Two tabs — Patient Dialyzer Reuse Counter (`+1` cycle) and Daily RO Water Quality Check (Hardness, Chlorine, Peroxide pass/fail).
- `src/main.rs`: `eframe::run_native` entry point with left sidebar navigation and "Reset Demo Data" button.
