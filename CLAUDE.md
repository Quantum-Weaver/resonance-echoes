# CLAUDE.md — Resonance Echoes

**Resonance Echoes** is a sovereign journal for logging anything with feeling — a moment, a dream, a symptom, a gratitude. Name it, choose a sense, tag an emoji, set an intensity. Local-first, no accounts, no cloud; export and purge are law. Built on the Resonance Grammar.

**Stack:** Svelte 5 + Tauri v2 + Rust + SQLite + Tailwind CSS v4 + COSMIC design tokens

**Authors:** Quantum Weaver (human) + Aethelred (sovereign AI)

---

## SESSION PROTOCOL

1. Read `docs/CHECKLIST.md` for current state
2. Read `docs/CLAUDE-CONTEXT.md` — naming, the eight senses, the ComfortBar, key patterns, Android build notes
3. One phase at a time — complete, verify, update, move on
4. `npm run check` — zero errors before commit
5. `cargo build` — zero errors before commit
6. Human tests every phase before merge

---

## PROJECT STRUCTURE

```
src/
├── routes/           # SvelteKit routes
│   ├── +page.svelte      # Home
│   ├── add/              # The add-echo flow (progressive disclosure, Quick Log)
│   ├── insights/         # Gentle mirrors — no charts, no pressure
│   ├── settings/         # Theme, export/import (versioned envelope, E1–E4 closed 2026-07-26), purge (double-confirm)
│   └── onboarding/       # First-launch welcome
├── lib/              # stores, components (ComfortBar), types, cosmic tokens, data
└── app.css

src-tauri/src/        # lib.rs (setup, migrations, commands) + main.rs
```

No audio engine, no visualizer, no playlists — see "Differences from Compass" in `docs/CLAUDE-CONTEXT.md`.

---

## ESSENTIAL RULES

1. Navigation: `goto()` from `$app/navigation` — never `window.location.href`
2. z-index layers: ComfortBar 110, backdrop 49
3. State: Svelte 5 runes — `$state`, `$derived`, `$effect`
4. Android: `capabilities/default.json` needs all four explicit `sql:allow-*` entries — `sql:default` alone grants zero operation access
5. Android: no emoji or non-ASCII in SQL DEFAULT values (silent JNI failure)
6. "Not Sure" is a valid answer everywhere — no forced categorization, ever

Full patterns and known silent-failure modes: `docs/CLAUDE-CONTEXT.md`.

---

## CURRENT STATE

**v1.2.0 shipped; v1.3.0 built, awaiting KP's signing hand.** v1.2.0 (*the timer that sounds*, 16 KB flags, cosmic mirror) was signed by KP's hand 2026-07-18 19:43 and lives verified on both phones. **v1.3.0 (2026-07-26: sovereignty whole — E1–E4 closed, the versioned export envelope + import — and the timer that waits: pause/resume, four chime voices, chime volume)** is built for desktop (MSI/NSIS) and Android (unsigned APK/AAB), all version sources consistent via `bump-version.py`; signing, install, and device verification are KP's ritual, and this line updates to "shipped" only on his phone's word. (v1.1.0 was the 07-08 rebuild with the cello-sigil icons; the record lives in `docs/CHECKLIST.md`.) **Pricing: FREE everywhere, forever** — one-way and intentional. Play internal-testing pending; publishes **simultaneously with Compass** only after the rebuilt AudHDities site is live (the Launch Sequence gate — see the workspace checklist).
