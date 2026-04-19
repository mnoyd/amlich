# Cuốn TUI: The Scrolling Almanac Concept

**Date:** 2026-03-05
**Status:** Draft / Conceptual

## Vision: From Dashboard to Scroll

The current Am Lich TUI follows a traditional "dashboard" paradigm: fixed panels, side-by-side columns, and information squeezed into predetermined boxes. While functional, it doesn't take full advantage of the terminal as a reading canvas, nor does it gracefully handle the immense depth of data provided by `amlich-api`.

**The new concept, "Cuốn" (The Scroll), abandons the dashboard metaphor for a scrolling, page-based reading experience.** 

Think of a traditional Vietnamese almanac scroll unrolled in your terminal. It flows vertically, breathes with generous whitespace, and adapts fluidly to any window size—just like a responsive webpage or a well-typeset markdown document.

---

## 1. Responsive & Fluid Layout

The single-column scroll design is inherently responsive. Instead of hiding entire panels (like the calendar) when the window shrinks, the content simply reflows.

*   **Narrow (< 60 cols):** Information is stacked. The timeline becomes a text list. The week ribbon compresses.
*   **Medium (60–100 cols):** `Nên/Tránh` (Do/Don't) splits into two columns. The visual hours timeline appears.
*   **Wide (100+ cols):** Full visual richness. The 12-hour timeline spans the screen. Deep data like `Nạp Âm` (Na Am) appears inline. Content is elegantly center-aligned.

---

## 2. The "Verdict" Badge (TL;DR)

Users usually open the calendar with a specific question: *"Is today a good day to do X?"*

Before diving into complex astrological data, "Cuốn" provides an immediate, synthesized, one-line "Verdict" right below the hero date. This badge interprets the raw data into a human-readable takeaway, color-coded for quick scanning.

*   `[ Cát Tinh Tụ Hội - Rất tốt cho khởi sự ]` *(Green - Very Good)*
*   `[ Ngày Xung Thái Tuế - Chỉ nên làm việc nhỏ ]` *(Red - Proceed with Caution)*
*   `[ Tiết Lập Hạ - Thời tiết thay đổi, chú ý sức khỏe ]` *(Amber - Notice)*

---

## 3. Terminal Data Visualizations

Text is great, but visualizations are better. We will use terminal characters (braille, blocks, box-drawing) to create inline visual widgets that bring the data to life:

*   **Visual Timeline (Giờ Hoàng Đạo):** A horizontal bar chart `██░░██` showing the progression of good/bad hours throughout the day, rather than just a list of names.
*   **The Lunar Phase:** Calculate and display the exact moon phase natively based on the lunar day: `🌒 🌓 🌔 🌕 🌖 🌗 🌘 🌑`.
*   **Clash/Harmony Wheel (Xung Hợp):** A mini 12-branch ASCII circular dial showing the harmony (Tam Hợp) or clash (Lục Xung) triangles visually.
*   **The Kua Compass (Bát Quái / Tứ Mệnh):** If personal profile data (birth year) is provided, display a 3x3 grid showing the user's favorable (Green) and unfavorable (Red) directions for the day.

---

## 4. Accordion Scrolling (Progressive Disclosure)

The `amlich-api` returns deep, encyclopedic lore (e.g., Tiết Khí has Astronomy, Agriculture, Health, Weather). Showing all this by default would create an overwhelming wall of text.

Instead, we use **Accordion UIs** for sections with heavy text.
*   The default scroll view shows only a concise summary (e.g., *Vũ Thuỷ · Mưa Xuân - Mưa nhiều, chuẩn bị gieo mạ*).
*   If the user navigates to the section and presses `Enter` or `→`, the section smoothly expands downwards to reveal the full lore, pushing the rest of the document down.

---

## 5. Lenses (Focus Modes)

Different users (or the same user at different times) need different information. Putting everything on one screen is visually noisy. 

Users can press `Tab` to cycle the entire layout between specific "Lenses":

1.  **Chung (General):** The default daily planner view. Focuses on `Nên/Tránh`, `Giờ Hoàng Đạo`, `Tiết Khí`, and `Lễ Tết` (Holidays).
2.  **Hành Sự (Planning):** Emphasizes precise hour charts, `Xuất Hành` (Travel directions: Hỉ Thần/Tài Thần/Hạc Thần), and auspicious stars.
3.  **Học Thuật (Scholarly):** Built for Feng Shui enthusiasts. Focuses heavily on Bát Tự, `Thập Thần` (Ten Gods), `Tàng Can` (Hidden Stems), and `Xung Hợp`.
4.  **Cá Nhân (Personal):** Morphs the display to compare the day's energy against the user's specific profile (`Tuổi xung`, `Mệnh xung`, `Cửu Tinh`, `Đại Vận`).

---

## Interaction Model

*   `h/l` or `←/→` — Previous/next day
*   `j/k` or `↑/↓` — Scroll up/down within the daily document
*   `Enter` — Expand/collapse the currently focused accordion section
*   `Tab` — Cycle through Lenses (General -> Planning -> Scholarly -> Personal)
*   `Space` — Full-screen calendar popup
*   `t` — Jump to today
*   `g` — Date jump modal
*   `/` — Search holidays
*   `L` — Toggle language (Vi/En)
*   `q` — Quit

## Architecture Notes

*   This will be a new crate: `crates/amlich-tui` to keep it cleanly separated from the older implementation.
*   It will consume the `amlich-api` directly, leveraging its v2 bundle endpoint to pull exactly the data needed for the active Lens.
*   Built entirely on `ratatui` with custom widgets for the scrollview and visualizations.
