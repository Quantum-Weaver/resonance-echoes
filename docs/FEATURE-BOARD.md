# ECHOES — THE FEATURE BOARD
*Assembled 2026-07-19 (the workspace honoring). Echoes is the family's
reference implementation and is COMPLETE for its purpose — this board is
short because the app is finished, not because it is neglected.*

## ✅ CLOSED IN CODE — 2026-07-26, sequentially at KP's word ("e1-4 we can do this sequentially")

*All four rows closed in one sitting, one commit each, check clean at every
step. **The suspension of the "fully green" declaration lifts at the code
level; device verification rides the v1.3.0 build** — the board stays honest
about that seam: fixed-in-code and proven-on-KP's-phone are different states,
and only his hands close the second.*

- [x] **E1 · Export queries the database, never the loaded page** (`7719314`) —
  `getAllEchoes()` walks the table unbounded, throws on a null db; the purge
  flow now AWAITS the export, so export-then-purge can never destroy the
  remainder.
- [x] **E2+E3 · One versioned envelope, KP's one-breath ruling** (`a1154c6`) —
  `envelope: resonance-export · envelopeVersion: 1 · app: resonance-echoes`,
  data carrying BOTH echoes and folksonomy, counts written on the envelope so
  the file shows what it carries. Purge and export now cover exactly the same
  ground. **The shape is the family's to inherit** (the vessel-graphs cheap
  win #2, made real here first).
- [x] **E4 · Import exists** (`698c278`) — reads the v1 envelope AND the legacy
  bare-array export; non-destructive by law (INSERT OR IGNORE on id; an
  existing folksonomy definition is never overwritten by an older file);
  reports plainly; refuses other apps' envelopes by name.

## 🎐 The timer grew, same sitting (KP's commission, 2026-07-26) — `ec4f236`
- [x] Pause / resume — the sand holds still; resume re-unlocks audio as a
  fresh gesture.
- [x] Four chime voices, all synthesized in the sensory-friendly idiom (Rise ·
  Bell · Drop · Pulse), previewed in the tap that chooses them.
- [x] Chime volume slider (0–100%, preview on release; zero is a chosen
  silence). Persisted beside the sound toggle.

## Maintenance — ✅ ALL CLOSED 2026-07-19 (the day Echoes finished)
- [x] Hex residue → cosmic: the three semantic drifts (#f39c12/#27ae60/
  #e74c3c → warning/success/emergency-high vars) swapped across four
  routes; canvas token-colors now flow from the constants mirror
  (QUANTUM_COLORS); the sand's browns and theme-mode neutrals DECLARED
  deliberate in comments (art is art; AMOLED black has no token by
  design). check 0/0 · build clean.
- [x] Galaxy listing pack DRAFTED → `docs/GALAXY-LISTING.md` (KP
  blesses tone, transcribes at upload).

**With this, Echoes' own board is fully green — the first realm to
finish. Deliberate rest, earned twice.**

## Relations (features that arrive FROM elsewhere)
- **The vessel-graphs surface** (seed: THE-VESSEL-GRAPHS...) — echoes
  data walked as an immersive graph in the vessel's Sanctuary space;
  Echoes' own export is already the honest data door.
- **Hearth integration** (Hearth Tier D): reflections ↔ recovery logs.

## Deliberate rest
No new in-app features planned. The gentle insights ARE v1's ceiling by
design; anything deeper belongs to the graphs, not more cards.
