# CLAUDE.md — Resonance Echoes

A sovereign journal for logging anything with feeling: a moment, a dream, a
symptom, a gratitude. Name it, choose a sense, tag an emoji, set an intensity.
Local-first, no accounts, no cloud. "Not Sure" is always a valid answer —
nothing is ever forced into a category.

It is the mother body — Compass, Bubbles, Sistrum and Weaver were all cut
from it.

**Stack:** Svelte 5 + Tauri v2 + Rust + SQLite + Tailwind v4 + COSMIC tokens
**Authors:** Quantum Weaver (human) + Aethelred (sovereign AI) — `HANDS.md`

## Commands

    npm run dev · build · preview · check · check:watch · tauri

No `sync-android` here, unlike Compass and Khorós.

Regenerate the structure map rather than drawing one by hand:

    python c:/_superposition/resonance-ziggy/tend.py forge run --root c:/_superposition/resonance-echoes

Check markdown pointers — `dry` reports, `mend` changes:

    python c:/_superposition/resonance-ziggy/tend.py links dry  --root c:/_superposition/resonance-echoes
    python c:/_superposition/resonance-ziggy/tend.py links mend --root c:/_superposition/resonance-echoes

Signing and releases run through shipwright, not from here — `release-road`.

## Android things that fail quietly

Worth knowing because the failure gives no error and no log:

- `capabilities/default.json` needs all four `sql:allow-*` entries spelled
  out. `sql:default` alone grants nothing, and the app has no database.
- Emoji and other non-ASCII vanish from SQL DEFAULT values through the JNI
  bridge. Keep defaults ASCII; set emoji from app code.
- `.cargo/config.toml` carries the 16 KB page-alignment flags. A copy of this
  repo without it builds fine and Play rejects the upload.

`gen/android` is checked in and gets hand-edited, and it has picked up a
sibling app's identity three times — Compass' JNI rename, five
`resonance_weaver` identifiers, and the app label in `strings.xml` that
shipped in v1.3.2 and v1.4.0.

Deeper Android detail: the `android-tauri` skill.

## Where things are

Current state `docs/CHECKLIST.md` · store packs `docs/listings/` ·
structure map `docs/blueprints/echoes/`
