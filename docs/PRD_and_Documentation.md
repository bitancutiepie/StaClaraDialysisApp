# Sta. Clara Dialysis Center, Inc. — Inventory Management System
## Product Requirements Document (PRD), Gap Analysis & User Documentation

**Prepared For:** Sta. Clara Dialysis Center, Inc.  
**Prepared By:** [Your Name / Team Name]  
**Course / Subject:** [Course Code & Title, e.g., BAT 402]  
**Date:** November 26, 2026  

---

## 1. Executive Summary

The **Sta. Clara Dialysis Center Inventory Management System** is a Windows desktop application built to help clinic staff track dialysis supplies, prevent critical stockouts, monitor upcoming expiration dates, and stay synchronized with the clinic's official Excel inventory workbook.

Designed for nurses, stockroom custodians, and clinic supervisors, the application replaces error-prone manual counting with clear visual alerts, guided workflows, and safety checks—while keeping every number compatible with the clinic's existing Excel spreadsheets.

---

## 2. Product Requirements Document (PRD)

### 2.1 Clinic Goals
1. **Prevent Stockouts:** Immediately highlight any supply that drops below the clinic's **150-piece safety threshold** (`Dangerous (<150)`).
2. **Reduce Waste:** Track **Earliest Expiry** dates and warn staff before supplies expire.
3. **Separate Storage & Floor Stock:** Clearly track how many pieces are in the **Warehouse** versus the **Treatment Area**, with **Ending Balance = Warehouse + Treatment Area**.
4. **Prevent Accidental Errors:** Never allow negative stock balances, and require confirmation before saving changes or overwriting Excel files.
5. **Stay Compatible with Excel:** Allow staff to load, view, edit, and save directly to the clinic's `.xlsx` workbook without losing formatting or formulas.

### 2.2 Target Users
- **Treatment Area Nurses & Staff:** Request supplies from the stockroom, check what is running low, and record damaged or expired items.
- **Warehouse / Stockroom Custodians:** Record incoming deliveries, transfer supplies to the Treatment Area, and keep the Excel file up to date.
- **Clinic Supervisors / Administrators:** Review monthly usage, monitor critical alerts, and audit stock movements.

### 2.3 Core Screens & Features
1. **Home Workspace:** Welcoming clinic-branded starting screen with quick-access cards to every major task and an interactive clinic mascot.
2. **Dashboard (Needs Attention Today):** Shows **All**, **Critical Stock (<150)**, and **Expiring Soon** alerts with a one-click **`+ Order`** button to restock.
3. **Stock-In (Deliveries):** Record new supplier deliveries with batch/expiry dates.
4. **Requisition & Transfer (Kiosk Order Screen):** Supports **Floor Requisition** and **Warehouse → Treatment Area Transfer**, organized into a 2-column supply catalog with medical icons.
5. **Wastage & Adjustments:** Record expired, damaged, or lost items with confirmation prompts.
6. **Monthly Usage View:** Displays daily consumption across the month alongside the **Earliest Expiry** column.
7. **Excel Sync & Viewer:** Load `.xlsx` files, inspect/edit data in an interactive **Excel View** pop-up, and save either by **Overwriting Original** (with an automatic `.bak` safety backup) or saving a **Timestamped Copy**.

---

## 3. Implementation Status (Gap Analysis)

### 3.1 Fulfilled (Completed & Ready to Use)
- **Full Clinic Supply List:** All 36 real clinic items grouped into **Treatment**, **Reverse Osmosis**, and **Center Supplies**.
- **Low-Stock Safety Rule (`<150`):** Items with an ending balance below 150 pieces are automatically flagged as **Dangerous (<150)**.
- **Warehouse + Treatment Area Split:** Tracks both locations separately and prevents negative balances.
- **Excel Workbook Synchronization:** Reads and writes both the monthly usage sheet and the **Expiry & Stock** summary sheet, with `.bak` backup and timestamped copy options.
- **Safety Confirmation Pop-Ups:** Asks for confirmation before saving to Excel, resetting data, or recording deliveries, requisitions, and wastage.
- **Adaptive Layout:** Automatically adjusts between wide desktop monitors and smaller tablet/kiosk screens.
- **Clinic Branding & Packaging:** Official Sta. Clara Dialysis Center logo, background, frosted-glass cards, standalone `.exe`, and a formal Windows Installer (`StaClaraDialysis-Setup.exe`) with Desktop shortcut and uninstaller.

### 3.2 Partially Fulfilled (Works, With Room to Grow)
- **Activity & Technician Logs:** The app records inventory movements, but entries are not yet linked to individual staff login accounts or formal shift-handoff sign-offs.
- **Multi-Computer Sharing:** Multiple staff can share data by opening the same Excel file on a shared folder, but there is no live, simultaneous multi-user database yet.
- **Printing & Paper Reports:** Staff can export/save to Excel and print from Microsoft Excel, but there is no built-in one-click **"Print Report"** button inside the app.

### 3.3 Not Implemented (Future Enhancements)
- **User Login & Role Permissions:** No username/PIN login screen or restricted supervisor-only actions yet.
- **Automated Supplier Ordering / Email Alerts:** The app flags low stock on screen, but does not automatically send emails or purchase orders to suppliers.
- **Barcode / QR Scanner Support:** Supplies are selected from the on-screen catalog rather than scanned with a handheld barcode scanner.
- **Cloud Backup & Remote Mobile Access:** Data is stored locally on the clinic computer / Excel file rather than synced to a cloud server.

---

## 4. Recommendations

1. **Short-Term (Immediate Clinic Rollout):**
   - Use the Windows Installer (`StaClaraDialysis-Setup.exe`) on the main stockroom and nurse-station PCs.
   - Keep the clinic's master `.xlsx` file in a shared local folder (or OneDrive/Google Drive) and always use the **Timestamped Copy** or automatic `.bak` backup when saving.
2. **Medium-Term (Next Update):**
   - Add a simple **4-digit Staff PIN** at startup so every delivery, requisition, and wastage entry records *who* made the change.
   - Add a **One-Click Print / PDF Summary** button on the Dashboard and Monthly Usage screens for weekly supervisor meetings.
3. **Long-Term (Scaling Up):**
   - Connect the app to a shared local or cloud database so multiple stations update in real time without needing to manually load/save Excel files.
   - Add **Barcode Scanning** for faster stock-in and floor requisitions.

---

## 5. User Documentation (Quick-Start Guide)

### 5.1 Installing & Opening the App
- **Using the Installer:** Double-click `StaClaraDialysis-Setup.exe` and follow the **Next → Next → Install** steps. Open the app from the **Sta. Clara Dialysis** Desktop shortcut.
- **Portable Option:** Double-click `StaClaraDialysis.exe` (or `run.bat` if running from the project folder).

### 5.2 Daily Clinic Workflows
1. **Check What Needs Attention:** Open **Dashboard** to see **Critical Stock (<150)** and **Expiring Soon** items. Click **`+ Order`** next to any low item to restock it.
2. **Load the Clinic Excel File:** Click **Load .xlsx** and select the clinic's current inventory workbook.
3. **Record a Delivery (Stock-In):** Go to **Stock-In**, pick the item, enter the quantity received and expiry date, and confirm.
4. **Request or Transfer Supplies:** Go to **Requisition**, choose **Floor Requisition** or **Warehouse → TA Transfer**, select items from the catalog, and confirm.
5. **Record Damaged or Expired Items:** Go to **Adjustments**, select the item and reason, and confirm.
6. **Save Back to Excel:** Click **Save .xlsx** and choose **Overwrite Original** (creates a `.bak` backup automatically) or **Save Timestamped Copy**.

### 5.3 Helpful Tips
- **No Negative Numbers:** The app will stop you if you try to transfer or deduct more pieces than are currently available.
- **Reporting a Screen Issue:** Press **F12** to toggle the built-in layout helper if anything looks cut off on a smaller monitor.
