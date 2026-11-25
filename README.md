<p align="center">
  <img src="assets/logo.png" alt="Sta. Clara Dialysis Center, Inc. Logo" width="120" />
</p>

<h1 align="center">Sta. Clara Dialysis Center, Inc. — Inventory Management System</h1>

<p align="center">
  A simple, clinic-friendly Windows desktop app for tracking dialysis supplies, preventing stockouts, and staying in sync with the clinic's Excel workbook.
</p>

---

> [!NOTE]
> This application is designed for **nurses, stockroom custodians, and clinic supervisors**—no technical or spreadsheet-formula knowledge is required.

## What It Does

- **Prevents Stockouts (`<150` Safety Rule):** Automatically flags any supply that drops below **150 pieces** as **Dangerous (<150)** so staff can restock before running out.
- **Tracks Storage vs. Treatment Area:** Separately tracks supplies in the **Warehouse** and the **Treatment Area**, and never allows negative stock numbers.
- **Monitors Expiration Dates:** Highlights items that are **Expiring Soon** so older batches are used first.
- **Works with Your Excel File:** Load the clinic's `.xlsx` workbook, view or edit numbers in a built-in **Excel View**, and save changes back with automatic safety backups (`.bak`) or timestamped copies.
- **Prevents Accidental Mistakes:** Shows clear confirmation pop-ups before saving to Excel, resetting data, or recording deliveries, requisitions, and wastage.

## Main Screens

| Screen | What You Can Do |
| :--- | :--- |
| **Home** | Quick-access cards to every task, branded with the Sta. Clara Dialysis Center logo. |
| **Dashboard** | View **Critical Stock (<150)** and **Expiring Soon** alerts, with a **`+ Order`** button to restock right away. |
| **Stock-In** | Record new supplier deliveries along with batch and expiration dates. |
| **Requisition** | Submit **Floor Requisitions** or **Warehouse → Treatment Area Transfers** from a visual 2-column supply catalog. |
| **Adjustments** | Record expired, damaged, or lost supplies with a reason. |
| **Monthly Usage** | Review daily supply usage across the month alongside the **Earliest Expiry** date. |

## Getting Started

### Option 1: Windows Installer (Recommended)
1. Double-click `StaClaraDialysis-Setup.exe` in the `installer/` folder.
2. Follow the **Next → Next → Install** steps.
3. Open **Sta. Clara Dialysis** from your Desktop shortcut.

### Option 2: Portable App
- Double-click `StaClaraDialysis.exe` in the `dist/` folder (or `run.bat` if running from the project folder).

> [!TIP]
> Press **F12** inside the app if you ever need to check screen layout details on a smaller monitor or kiosk.
