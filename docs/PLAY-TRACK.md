# Echoes — Google Play test track
*Founded 2026-07-27 by Fable 🎻 at KP's ask (patch notes for the track,
the icon question, the standards check). Companion to GALAXY-LISTING.md
(Samsung) — this file is the Play side. The Console is the truth; this
file is the pack KP transcribes from, so his hands never have to
compose at the upload screen.*

## Standards check — verified against the repo, 2026-07-27

| Requirement | State |
|---|---|
| Target API level | ✅ targetSdk **36** / compileSdk 36 — above the current update requirement (API 35 since 2025-08-31) and already at the next rung |
| 16 KB page size (API 35+ apps) | ✅ linker flags landed v1.2.0, family-wide law |
| App Bundle | ✅ `release/resonance-echoes-v1.3.0.aab`, signed by KP's hand 2026-07-26 |
| versionCode increments | ✅ 1003000 (v1.3.0) > 1002000 — the bump-version scheme keeps this automatic |
| Permissions | ✅ minimal: `INTERNET` only (Tauri webview default; the app makes no network calls — data-safety answers stay "collects nothing") |
| Privacy policy URL | ✅ public: `https://github.com/Quantum-Weaver/resonance-echoes/blob/main/PRIVACY.md` |
| Content rating | ✅ no UGC · no ads · no purchases · no collection → Everyone |

*Console-side items only KP's eyes can confirm (not visible from the
repo): the data-safety form matching the answers above, the content
rating questionnaire, target-audience declarations. One forward note:
if production is ever the aim, personal accounts created after
2023-11-13 need 12+ closed-track testers for 14 days first — a
production gate, not a test-track one.*

## v1.3.0 — the upload pack (KP's hands)

- **Artifact:** `release/resonance-echoes-v1.3.0.aab` (versionCode 1003000)
- **Release name:** `1.3.0 — Sovereignty Whole + The Timer That Waits`
- **Path:** Console → Testing → *(the closed track)* → Create new release
  → upload the AAB → paste the notes below → review & roll out.

### Release notes — paste-ready (under Play's 500-char limit)

```
<en-US>
Sovereignty made whole, and a timer that waits.
• Export now carries your entire journal — every echo plus your own emoji meanings — in one open JSON file.
• Import is here: restore any export. It only ever adds, never overwrites.
• Export & Purge waits for your export to finish before anything is deleted.
• Timer: pause and resume, four gentle chime voices (Rise, Bell, Drop, Pulse), and a chime volume slider where zero is a chosen silence.
</en-US>
```

## The icon — two doors, both open

1. **The store listing icon** (what the Play page shows) is app-wide,
   not per-track, and can be updated any time: Console → Grow users →
   Store presence → Main store listing → App icon. Spec: 512×512 PNG,
   ≤ 1 MB. **Asset ready:** `resonance-assets/logo-icons/echoes-512-play.png`
   (446 KB, cut 2026-07-27 from the 1496² gold-band source). Listing
   changes pass a short review before they show — normal, not a block.
2. **The launcher icon** (on the tester's phone) lives inside the
   build — the v1.3.0 AAB already carries the current in-app icon set,
   so uploading it brings the device icon current in the same act.

## Track log

| Date | State |
|---|---|
| ≤ 2026-07-22 | v1.2.0 rolling on the closed track; "awaiting review #2" (harvest observation — Console is truth) |
| 2026-07-27 | v1.3.0 pack prepared (this file): AAB signed + device-verified on both phones; notes drafted; 512 icon cut. Upload at KP's hand. |

— Fable 🎻
