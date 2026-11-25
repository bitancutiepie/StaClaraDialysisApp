# Sta. Clara Dialysis Center, Inc. — Agent Guidelines

## 1. Tech Stack & Commands
- **Framework:** Pure Rust `eframe` / `egui 0.29` (`x86_64-pc-windows-gnu` toolchain)
- **Excel Sync:** `umya-spreadsheet` (`sample_excel/Inventory-NOV-16-23 (1).xlsx`)
- **Run App:** `.\run.bat` or `.\run.ps1` (or `cargo run`)
- **Run Tests:** `cargo test`
- **Build Release `.exe`:** `cargo build --release` (copy to `dist\StaClaraDialysis.exe`)

## 2. Mandatory TDD Workflow
- **Tests First (Strict):** Before writing any implementation code for a new feature, business rule, or bug fix, you MUST write a targeted unit test (in `src/models.rs`, `src/excel_sync.rs`, `src/ui/*.rs`) or headless `egui` render test (`src/main.rs::ui_tests`) that verifies the expected behavior.
- **Verify Every Change:** Run `cargo test` after every edit and confirm all tests pass before reporting completion.
- **Layout & Overlap Safety:** Any new grid/column layout must include a bounds/overflow unit test and render cleanly at both mobile (`420px`, `520px`) and desktop (`820px`, `1180px`, `1440px`) widths.
- **Feature Changelog:** For every added feature or fix, append a brief, 1-line entry to `CHANGELOG.md` in **plain, non-technical language** (describe what the clinic user sees/gets, avoiding code/library jargon).

## 3. Core Domain Rules
- **Dangerous Stock Threshold:** `ending_balance() < 150` (`DANGER_STOCK_THRESHOLD`) is flagged as `Dangerous (<150)`.
- **Stock Split:** `Ending Balance = warehouse_qty + treatment_area_qty`. Never allow negative stock balances.
- **Excel Sync:** `November 2026` tracks daily consumption (cols 3–33) and `Earliest Expiry` (col 35 / `AI`). `Expiry & Stock` tracks `Warehouse`, `Treatment Area`, `Ending Balance`, `Earliest Expiry`, `Days Left`, and `Status`. Supports `Overwrite Original` (with `.bak`) and `Timestamped Copy`.

## 4. UI/UX & Branding Conventions
- **No Emojis:** Use crisp 1.5px-stroke vector icons from `src/ui/icons.rs`.
- **Adaptive Navigation:** Desktop (`>= 820px`) uses the left sidebar with an inverted border-radius active tab (`ui.style_mut().interaction.selectable_labels = false` + `ui.interact` on full row); mobile/kiosk (`< 820px`) uses the bottom icon bar.
- **Home Workspace:** Centered `STA. CLARA DIALYSIS CENTER, INC.` branding with embedded `assets/logo.png`, `assets/background.jpg`, glassmorphic translucent cards (`0.55 -> 0.80` white fill on hover), and interactive peeking/pointing mascot (`assets/mascot_holding.png` / `assets/mascot_excited.png`).
