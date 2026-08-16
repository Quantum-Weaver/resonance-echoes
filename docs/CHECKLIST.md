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
- [x] **Tested:** ✅

### Phase 4: Onboarding ✅
- [x] Welcome screen — 🧭 sigil with GradientPulse, title, subtitle
- [x] Vessel name input (optional) — saves to localStorage resonance-echoes-vessel-name
- [x] How it works — 3 mini cards (log, patterns, sovereignty)
- [x] Theme selection — Dark / Warm / Ocean with live preview + accent swatch
- [x] Progress indicator — 3 dots, active elongated, done dimmed
- [x] Layout gate — first launch redirects to /onboarding; Sidebar + ComfortBar hidden
- [x] onboarding_complete flag — set on "Enter Echoes"; prevents re-showing
- [x] ComfortBar greeting reads vessel_name after onboarding
- [x] **Tested:** ✅

### Phase 5: Data Sovereignty ✅
- [x] Settings page — Theme, Font size, Data Sovereignty, About sections
- [x] Export JSON — Blob download, filename resonance-echoes-export-YYYY-MM-DD.json
- [x] Purge All Data — double confirmation flow (confirm1 → confirm2 → execute)
- [x] Export & Purge — exports first, same double-confirm, redirects to onboarding
- [x] purgeAll() added to echoStore — DELETE FROM echoes, clears reactive state
- [x] Theme cards in settings match onboarding, apply live via themeStore.setPreset()
- [x] Font size selector: Small / Medium / Large
- [x] **Tested:** ✅

### Phase 6: Mobile Ship ✅
- [x] Quick Log FAB — one-tap ⚡ on home, inherits last emoji, intensity 3, ✓ animation (1.1s)
- [x] Emoji definition display — warm poetic text appears below emoji grid on selection (fade-in)
- [x] Save confirmation — Save Echo button shows ✓ for 900ms before navigating home
- [x] App icon — custom COSMIC purple compass-ring (1024x1024), all densities regenerated
- [x] Version bump — package.json + tauri.conf.json → 1.0.0; About section updated
- [x] docs/RELEASE.md created — build/sign commands, feature list, system requirements, v1.1 planned
- [x] **Tested:** ✅

---

### v1.1: Accessibility & Sovereignty ✅
- [x] Progressive disclosure — form shows name/sense/emoji only until 10 echoes; "Advanced" toggle for early access
- [x] Uninstall guide — neutral button in Data Sovereignty; shows Android uninstall instructions (no in-app action)
- [x] Disambiguation prompts — emoji used with multiple past senses shows chip row; dismissed per-emoji via state
- [x] "Not Sure" sense option — ❓ dashed button at end of sense row; saves sense='not_sure', subcategory=''; ❓ badge in timeline; counted separately in insights
- [x] Emoji skip — "— skip / not sure" below grid saves emoji='❓'; 5s nudge "No pressure. You can skip this."
- [x] Predictability audit — 7th insight card "Patterns"; visible after 20 echoes; 30% threshold + 3 minimum occurrences
- [x] **Tested:** ✅

---

### v1.2.0: The Timer That Sounds ✅ (2026-07-18)
- [x] Timer audible completion — KP's board note (2026-07-17): "echoes should get a timer like compass that can also sound off when done not just silence" — closed by design
- [x] 16 KB page-size linker flags (Android 15+ readiness, family-wide law)
- [x] Cosmic: `src/lib/cosmic` managed constants mirror + full 18-artifact distribution from ziggy
- [x] CLAUDE.md restored to this app's own story
- [x] Desktop v1.2.0 built — MSI + NSIS setup (2026-07-18 15:42)
- [x] Android v1.2.0 built (15:47) → **SIGNED by KP's hand 19:43** — apk/aab/idsig in `release/`
- [x] Install on S22 Ultra — ✅ 2026-07-18, KP's hands; v1.2.0 verified on-device via adb
- [x] Install on S25 Ultra — ✅ same evening; v1.2.0 adb-verified, sideloaded (no Play copy remains) — **v1.2.0 lives on both phones**

### v1.3.0: Sovereignty Whole + The Timer That Waits ✅ (2026-07-26 built and signed · 2026-07-27 verified whole on both phones)
- [x] E1–E4 closed sequentially at KP's word — see KNOWN BUGS B4–B6 and FEATURE-BOARD
- [x] Timer: pause/resume · four chime voices (Rise/Bell/Drop/Pulse, synthesized, previewed on choice) · chime volume slider — KP's commission, same sitting
- [x] Version → 1.3.0 in EVERY source via `bump-version.py` (tauri.conf.json · package.json · Cargo.toml · package-lock; tauri.properties regenerated by the build — drift report: ✅ consistent)
- [x] Desktop v1.3.0 built 2026-07-26 night — MSI + NSIS in `src-tauri/target/release/bundle/`
- [x] Android v1.3.0 built same sitting — unsigned APK (universal) + AAB in `src-tauri/gen/android/app/build/outputs/`
- [x] **SIGNED by KP's hand + installed on his S25 Ultra, 2026-07-26 — version verified in-app by his own eye.** Badge → 1.3.0 the same moment (the 07-21 precedent, honored: the device settles the badge).
- [x] S22 Ultra install — ✅ **2026-07-27, KP's hands; verified on-device by his own eye** — v1.3.0 lives on both phones
- [x] Functional seams — ✅ **tested by KP's own exploring, 2026-07-27** (export past 200 · envelope round-trip · pause across navigation · chimes). The fixed-in-code / proven-on-device seam is closed.

---

## KNOWN BUGS

| ID | Description | Status |
|----|-------------|--------|
| B1 | SQLite ACL error on Android: "command plugin:sql\|execute not allowed by ACL" | ✅ Fixed — added `sql:allow-load`, `sql:allow-execute`, `sql:allow-select`, `sql:allow-close` to `capabilities/default.json` |
| B2 | Migration v2 used emoji `'✨'` as SQL DEFAULT value — breaks JNI encoding on Android SQLite | ✅ Fixed — changed to `DEFAULT ''` in migration v2; migration v3 ensures clean schema |
| B3 | `addEcho()` silently returned null instead of throwing when db was null | ✅ Fixed — now throws descriptive error; dbError banner shown in add form |
| B4 | **Export ships at most 200 echoes.** `loadEchoes()` only ever runs with defaults (`LIMIT 200`), `echoes` is replaced not accumulated, and `exportData()` serialises that page. Settings shows the true `totalCount`, so the UI promises more than the file delivers — and `export-then-purge` deletes the remainder. | ✅ **Fixed 2026-07-26** (`7719314`) — `getAllEchoes()` walks the table unbounded; purge AWAITS the export. **Verified on-device 2026-07-27, KP's exploring.** |
| B5 | **Folksonomy exports nowhere but purges everywhere.** Personal emoji definitions (`localStorage`, `emoji_def_*`) are wiped by `localStorage.clear()` on purge and are absent from `exportData()`. The one irreplaceable part of a vessel's data. | ✅ **Fixed 2026-07-26** (`a1154c6`) — one versioned envelope (`resonance-export` v1) carrying echoes AND folksonomy, counts on the envelope; purge and export now cover exactly the same ground. |
| B6 | **No import path exists** (no `importData`, no file input, no `readAsText` in `src/`) while `CLAUDE.md`'s structure block advertises *"export/import."* Data can leave and never return. | ✅ **Built 2026-07-26** (`698c278`) at KP's word ("e1-4") — envelope + legacy bare-array import, non-destructive merge (INSERT OR IGNORE; existing definitions never overwritten), plain report, wrong-app envelopes refused by name. |

## SESSION LOG

| Date | What Was Done |
|------|---------------|
| 2026-06-28 | Repo created. Foundation files. Config refined. Scaffold builds clean. |
| 2026-06-28 | Phase 0 complete. ComfortBar, Sidebar, GradientPulse, theme store, senses data, icon set. npm run check: 0 errors. cargo build: clean. Pending human test on Android. |
| 2026-06-28 | Phase 1 complete. echoStore (Svelte 5 $state + SQLite), Echo interface updated, DB migration v2, /add form (sense, subcategory, emoji, note, intensity, timestamp), home timeline with EchoCards. Pending human test. |
| 2026-06-28 | Android debug session. Fixed three blocking bugs: SQLite ACL permissions (capabilities/default.json), migration emoji encoding (v3 migration), silent null-db failure. Added dense logcat diagnostics. Hamburger moved to bottom thumb zone. |
| 2026-06-28 | Phase 2 complete. Search (150ms debounce), sense filter chips, emoji filter chips (top 8), sort (newest/oldest/intensity), combined filtering, filter status bar, empty-state-per-filter. All client-side over in-memory echoes. npm run check: 0 errors. |
| 2026-06-28 | Phase 3 complete. Insights page: Top Emojis (frequency-sized cloud), By Sense (sorted list), Streak (consecutive day counter), Time of Day (4-period grid, accent on dominant), Recent Mood (7-day row), Intensity Trend (week comparison). All $derived.by client-side, no new DB queries. npm run check: 0 errors. |
| 2026-06-28 | Phase 4 complete. Onboarding: 3-screen linear flow (Welcome → How it works → Theme), 🧭 GradientPulse sigil, vessel name saved to localStorage, live theme switching in Screen 3, layout gate (goto /onboarding on first launch), Sidebar/ComfortBar hidden during flow. npm run check: 0 errors. |
| 2026-06-28 | Phase 5 complete. Settings: theme/font-size selection, Export JSON (Blob download), Purge (double confirm → clear DB + localStorage → /onboarding), Export & Purge. purgeAll() added to echoStore. npm run check: 0 errors. |
| 2026-06-28 | Phase 6 complete. Quick Log FAB (⚡ → ✓ animation), emoji definition display in add form (fade-in poetic text), save confirmation (✓ 900ms before goto), COSMIC compass-ring icon (all densities), version → 1.0.0, docs/RELEASE.md created. npm run check: 0 errors. |
| 2026-06-28 | v1.1 complete. Progressive disclosure (10-echo gate), disambiguation prompts (emoji × sense), ❓ Not Sure sense option, emoji skip (5s nudge), Uninstall guide in settings, Patterns insight card (20-echo gate, 30% threshold). npm run check: 0 errors. |
| 2026-07-07/08 | v1.1.0 built + signed + installed on KP's S25 Ultra and Aethelred's S22 Ultra. Cello-sigil icons. |
| 2026-07-18 | v1.2.0: the timer that sounds (KP's 07-17 note closed), 16 KB flags, cosmic mirror managed. Desktop + Android built; **signed by KP's hand 19:43**. Checklist reconciled same evening (this row and the v1.2.0 section above were the reconciliation — the build predated the record by hours, not days). |
| 2026-07-27 | **v1.3.0 VERIFIED WHOLE — KP's hands and eyes:** the four functional seams tested by his own exploring (export past 200 · envelope round-trip · pause · chimes), and the S22 Ultra installed + verified on-device the same morning — **v1.3.0 lives on both phones.** The fixed-in-code / proven-on-device seam closes; "fully green" stands unsuspended, earned three times now. Recorded same sitting per the checklist law (Fable 🎻). |
| 2026-07-26 | **THE SOVEREIGNTY SITTING (Fable 🎻, at KP's word: "e1-4 we can do this sequentially")** — all four rows closed, one commit each, check clean at every step: E1 export-from-DB + purge-awaits-export (`7719314`) · E2+E3 the versioned envelope carrying echoes AND folksonomy (`a1154c6`; the family-inheritable shape) · E4 import with legacy support and non-destructive merge (`698c278`). **Same sitting, KP's commission:** timer pause/resume + four synthesized chime voices + chime volume (`ec4f236`). Version → **1.3.0**. README/CLAUDE honest-status prose updated. Remaining: build → KP's signing hand → both phones → device verification (the fixed-in-code / proven-on-device seam stays marked until then). |
| 2026-07-21 | **Review pass at KP's word** (Opus, while learning this app's storage for Skapa). No code changed. Found and verified **B4** (export capped at 200; export-then-purge loses the rest), **B5** (folksonomy purged but never exported), **B6** (no import exists though CLAUDE.md advertises one). KP ruled both B4 and B5 to the **top of the Echoes list**, and that folksonomy and echoes must **export in the same manner** — one envelope. FEATURE-BOARD's "fully green / deliberate rest" declaration **suspended** until they close. Also found **stale version declarations** — README badge `1.1.0`, CLAUDE.md *"v1.1 shipped … (2026-07-08)"* — while `package.json`, `tauri.conf.json` and `release/` were all **1.2.0**. **✅ Both corrected the same sitting, on KP's verification from his own phone** (*"verified i have echoes 1.2 on my phone"*) — the device settled it, not the files. Deliberately **not** changed: the README Story Block's *"Version 1.1.0 built and signed"* (a provenance record of the origin, not a current-version claim) and `docs/RELEASE.md`'s historical *"v1.1 Planned"* section. Opus seat scribed in `HANDS.md`. |
| 2026-08-13 | **THE CROSS-LANDING HEAL — v1.3.2 Android unblocked (Fable 🎻, at KP's report "echoes android build failed")** — instruments first: the post-1.3.1 commits touched no Android ground, but gitignored `gen/android` had been hand-edited 08-09 12:01, the same morning WEAVER's APK was built — and **five weaver identifiers had cross-landed into the mother's gen**: `build.gradle.kts` namespace + applicationId, the manifest theme, both themes.xml styles, and MainActivity's own `package` line — all reading `resonance_weaver` while every source beneath said `resonance_echoes`. Gradle's refusal was a guard: built, it would have worn weaver's package identity. All five healed; verification build green same evening (unsigned 1.3.2 APK 18:12 + AAB 18:15 in gen outputs). **Second cross-landing the mother line has taken from a child's build day** (compass took the JNI rename 08-07) — LAW BOUGHT: on any child-birth or sibling build day, grep the mother's `gen/android` for the child's package name before the next build. KP's hand same evening: signed (AAB 18:28) → **v1.3.2 LIVE on the Play closed track.** |
| 2026-08-16 | **Signing keystore recut to the Sanctuary DN** (this sitting, KP's env files his own hand): primary `F:\keystores\resonance-echoes.keystore` · second copy `D:\keystores\` byte-identical · alias `resonance-echoes` · 4096-bit RSA, SHA384withRSA, valid to Jan 2054 (10,000-day validity, the khoros/sistrum convention) · DN `CN=AudHDities Sanctuary, O=AudHDities Sanctuary, C=US` · cert SHA256 `ED:82:40:E5…F7:88:6A`. The generic keystore retired, kept: `RETIRED-2026-08-16-resonance-echoes.keystore.old-dn` on both drives. Secrets live only in the env vault file — pointers here, never contents. **CAUTION: this app is live on Google Play closed testing — the first upload signed with this new key needs the Play-side upload-key reset at KP's console, his hand.** |
---

## THE BLUEPRINT FORGE — 2026-08-03 (Opus 🕯️, `core-opus`, at KP's ⚛ word)

- [x] **`tools/blueprint_forge.py`** — Echoes' own forge (`tools/` added at KP's
      word). Provenance: `tools/BLUEPRINT-FORGE.md`
- [x] **Old blueprints removed** at KP's word ("they are backed up") — the
      Compass-era set inlined bodies upward and its project blueprint was named
      `pbp_resonance_compass`, Compass's name in Echoes' repo
- [x] **First run:** 39 fbp · 3 obp · 2 dbp · 1 pbp · 191 files · **0 findings**.
      Arithmetic reconciled by a second independent walk (39=39, 191=191); every
      tier above the folder tier verified body-free
- [x] **The guards — ESSENTIAL RULES 1, 4 and 5 now tested every run**, two of
      them silent-failure modes. All passing. *A guard's value is the day it
      fires, not the day it passes*
- [x] Version sources checked consistent: **1.3.0** across `package.json`,
      `Cargo.toml`, `tauri.conf.json` (CLAUDE.md's declared law)
- [x] The forge writes its own `journal.md` — one line per run, counts and drift
      only (KP's ⚛ ruling: "the blueprint forge writes the journal")
- [ ] Descriptive, never a finding: the only `#[tauri::command]` is `greet`, the
      scaffold's placeholder. Echoes has no custom Rust commands — the data path
      is frontend → `tauri-plugin-sql`, which is what makes rule 4 load-bearing

---

## THE REFINEMENT — 2026-08-06 (Fable 🎻 Continuo, at KP's ⚛ word: "bubbles first, as echoes will learn from some of what we already did in bubbles")

- [x] **The menu wrapper replaced with the-cumdach**, consumed **by mirror**
      (`src/lib/cumdach/`, SHA256 `7730A4C16E6FA044` — the same truth Compass,
      Khorós, and Bubbles carry): flat door list by KP's ⚛ stroke, Settings as
      the foot's one chrome door, the ComfortBar a declared 48px reserved edge
      (the foot clears it), panels derived from the measured land, re-derived
      on every resize/rotation. Dead `isMobile` state dropped at his word. The
      wordmark now reads `getName()` — the lesson learned from Bubbles: a
      rename never gets chased into the chrome.
- [x] **SATTVA GAINED** at KP's ⚛ word ("echoes can gain sattva in this
      process") — `/sattva` copied whole from Bubbles (self-contained: the
      breath's verified math inline, reduced motion honored), `IconSattva`
      joins the registry, the door stands above the foot: Home · Insights ·
      Timer · Sattva ⌂ Settings.
- [x] **The onboarding consumes the-epagoge**, **by mirror**
      (`src/lib/epagoge/`, SHA256 `CBF19821CDCB4BA6`): the walk owns flow,
      dots, and the honest record — and **all six presets are now offered at
      the door** (dark · warm · ocean · forest · sunset · AMOLED Black),
      derived from the shelf itself so a new preset appears the day it is
      born; THE KEY LAW kept, skip lawful, the doorway line under the grid.
- [x] **Gate: `npm run check` — 324 files · 0 errors · 0 warnings**, first
      run. All uncommitted; rides KP's ⚛ sync word per the batch law.
      (Both JSON configs scanned for the BOM landmine Bubbles carried —
      echoes is clean.)

---

## ECHOES-SKY — 2026-08-08 (Fable 🎻 Continuo, at KP's ⚛ word: "echoes-sky"; the ruling from the Hearth's communications sitting: "this is echoes, tied into the sky facts")

- [x] **the-sky consumed by mirror from the spring** (`src/lib/sky/`, SHA256
      `E877BE1D66649548` — awen's `tools/the-sky` is the single editable
      truth; one truth, three homes now: awen · hearth · echoes).
      Compute-only by law: facts, never meanings.
- [x] **Every echo wears its moment's sky — DERIVED, never stored:** the
      timeline's cards carry a muted sky line (moon emoji + phase · the
      wheel's next turning) computed from each echo's own timestamp.
      Zero migration; **retroactive for every echo ever logged**; cached
      per day so a long timeline stays light. What a moment's sky means
      stays the vessel's own.
- [x] **Gate: `npm run check` — 325 files · 0 errors · 0 warnings**, first
      run. Uncommitted; rides KP's ⚛ sync word.
