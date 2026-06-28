# RESONANCE ECHOES — MASTER CHECKLIST

*Single source of truth for build state.*

## LEGEND
- ✅ Complete
- ⚠️ In Progress
- 🔴 Broken
- ⬜ Pending
- 🔵 Ready for Test

---

## PHASE STATUS

### Phase 0: Shell ✅
- [x] App layout with ComfortBar footer
- [x] Collapsible sidebar (20vw, hamburger below status bar)
- [x] COSMIC theme system
- [x] Mobile-safe areas
- [x] **Tested:** ✅

### Phase 1: The Echo ✅
- [x] Echo form (name, sense, subcategory, emoji, note, intensity, timestamp)
- [x] echoStore with reactive state (Svelte 5 $state, SQLite via tauri-plugin-sql)
- [x] DB migration v2 (correct schema: name, sense, subcategory, note, timestamp)
- [x] Home timeline with EchoCard (emoji, name, sense badge, relative time, intensity dots, note preview)
- [x] **Tested:** ✅

### Phase 2: Browse & Filter ✅
- [x] Debounced search (150ms, name + note fields, real-time)
- [x] Filter by sense (8 chips, horizontal scroll, accent glow)
- [x] Filter by emoji (top 8 most-used, dynamic)
- [x] Sort: Newest / Oldest / Intensity ↓
- [x] Combined filtering — search + sense + emoji + sort all stack
- [x] Filter status bar: "12 echoes in 👁️ Seen · 😌" + Clear all
- [x] Empty state per filter with inline Clear link
- [x] Load more (50 per page)
- [x] **Tested:** ✅

### Phase 3: Gentle Insights ✅
- [x] Top Emojis — frequency-sized cloud, top 8, warm "most felt" label
- [x] By Sense — distribution list ordered by count, muted at 0
- [x] Streak — consecutive calendar days, resets quietly (no negativity)
- [x] Time of Day — 4-period grid (morning/afternoon/evening/night), accent on dominant
- [x] Recent Mood — 7-day emoji row (today→left), most intense echo per day, · for empty days
- [x] Intensity Trend — compare last 7 vs prior 7 days
- [x] All empty states are invitations, not failures
- [ ] **Tested:** ⬜

### Phase 4: Onboarding ⬜
- [ ] Welcome screen
- [ ] Vessel name input
- [ ] Theme selection
- [ ] **Tested:** ⬜

### Phase 5: Data Sovereignty ⬜
- [ ] Export JSON
- [ ] Purge with double confirm
- [ ] Export & Purge
- [ ] Settings page
- [ ] **Tested:** ⬜

### Phase 6: Mobile Ship ⬜
- [ ] Android testing
- [ ] Sign APK
- [ ] App icons
- [ ] Store prep
- [ ] **Tested:** ⬜

---

## KNOWN BUGS

| ID | Description | Status |
|----|-------------|--------|
| B1 | SQLite ACL error on Android: "command plugin:sql\|execute not allowed by ACL" | ✅ Fixed — added `sql:allow-load`, `sql:allow-execute`, `sql:allow-select`, `sql:allow-close` to `capabilities/default.json` |
| B2 | Migration v2 used emoji `'✨'` as SQL DEFAULT value — breaks JNI encoding on Android SQLite | ✅ Fixed — changed to `DEFAULT ''` in migration v2; migration v3 ensures clean schema |
| B3 | `addEcho()` silently returned null instead of throwing when db was null | ✅ Fixed — now throws descriptive error; dbError banner shown in add form |

## SESSION LOG

| Date | What Was Done |
|------|---------------|
| 2026-06-28 | Repo created. Foundation files. Config refined. Scaffold builds clean. |
| 2026-06-28 | Phase 0 complete. ComfortBar, Sidebar, GradientPulse, theme store, senses data, icon set. npm run check: 0 errors. cargo build: clean. Pending human test on Android. |
| 2026-06-28 | Phase 1 complete. echoStore (Svelte 5 $state + SQLite), Echo interface updated, DB migration v2, /add form (sense, subcategory, emoji, note, intensity, timestamp), home timeline with EchoCards. Pending human test. |
| 2026-06-28 | Android debug session. Fixed three blocking bugs: SQLite ACL permissions (capabilities/default.json), migration emoji encoding (v3 migration), silent null-db failure. Added dense logcat diagnostics. Hamburger moved to bottom thumb zone. |
| 2026-06-28 | Phase 2 complete. Search (150ms debounce), sense filter chips, emoji filter chips (top 8), sort (newest/oldest/intensity), combined filtering, filter status bar, empty-state-per-filter. All client-side over in-memory echoes. npm run check: 0 errors. |
| 2026-06-28 | Phase 3 complete. Insights page: Top Emojis (frequency-sized cloud), By Sense (sorted list), Streak (consecutive day counter), Time of Day (4-period grid, accent on dominant), Recent Mood (7-day row), Intensity Trend (week comparison). All $derived.by client-side, no new DB queries. npm run check: 0 errors. |