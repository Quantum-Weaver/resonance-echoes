# Resonance Echoes — Repository Assessment
*Generated 2026-06-30. Local file read only — no GitHub access used.*

---

## 1. WHAT EXISTS

### Root
| File | Description |
|---|---|
| `CLAUDE.md` | Project instructions for Claude — note: describes the **Resonance Knowledge System** (Rust+SQLite+MCP grammar server), not this repo. Mismatched/inherited file (see Gaps). |
| `README.md` | Product overview, screenshots placeholder, install/build instructions, dev standards link. |
| `LICENSE` | Standard MIT license. |
| `PHILOSOPHY.md` | "The Resonance License" — non-binding ethical conditions (no exploitation, no extraction, data sovereignty, etc). |
| `helper.md` | Ad-hoc session notes (rebuild/sign/install commands, manual test checklist) from a debugging session. Listed in `.gitignore` but still tracked in git (see Gaps). |
| `generate_blueprint.py` | Python script that scans the codebase and writes `docs/blueprints/*.json`. Copy-pasted from the sibling "Resonance Compass" project — internally still titled `Resonance Compass Blueprint Generator` and contains Compass-specific route/phase maps unrelated to Echoes. Listed in `.gitignore` but still tracked (see Gaps). |
| `package.json` | npm manifest — SvelteKit 2 + Svelte 5 + Tauri v2 + Tailwind v4. |
| `package-lock.json` | npm lockfile. |
| `tsconfig.json` | TypeScript config extending SvelteKit's generated config. |
| `svelte.config.js` | SvelteKit config — `adapter-static`, SPA fallback to `index.html`. |
| `vite.config.js` | Vite config — Tailwind + SvelteKit plugins, fixed port 1420 for Tauri. |
| `.gitignore` | Excludes node_modules, build output, `.env*`, keystores/APKs, and (oddly) `helper.md` / `generate_blueprint.py`. |
| `.claude/settings.json` | Claude Code permissions, commit template, git identity. References a **stale path** `c:\_superposition\resonance-echoes` (pre-rename; repo now lives under `AudHDities-Resonance\resonance-echoes`). Currently shown as modified in git status. |

### docs/
| File | Description |
|---|---|
| `BUILD-SEQUENCE.md` | Phase 0–6 build plan (Shell → Echo → Browse → Insights → Onboarding → Sovereignty → Ship). |
| `CHECKLIST.md` | Master build-state checklist. Claims all of Phase 0–6 plus a "v1.1" accessibility pass are complete and tested. Also logs 3 known/fixed bugs (SQLite ACL, emoji-as-SQL-DEFAULT, silent null-db). |
| `CLAUDE-CONTEXT.md` | Naming conventions, sense list, ComfortBar spec, Android SQLite ACL deep-dive, debugging recipes (`adb logcat`). |
| `CONTRIBUTING.md` | Build protocol (branch-per-phase, `npm run check` + `cargo build` must be zero-error before merge, human test, then merge). |
| `RELEASE.md` | v1.0.0 release notes, build/sign instructions, "v1.1 Planned" list (now superseded — those features are already in CHECKLIST as shipped). |
| `RESONANCE-GRAMMAR.md` | Spec for the shared "Resonance Grammar" vocabulary system (atoms/molecules/folksonomy) that this app is meant to reference. Echoes does not actually implement or import this grammar anywhere in `src/`. |
| `SCREEN-INVENTORY.md` | Stale — says "Built (0) — None yet" and lists only 4 of 5 routes, while CHECKLIST/code show all 5 routes built and shipped. |
| `blueprints/INDEX.md` | How-to-read guide for the generated blueprint JSON files. References a `pbp_resonance_echoes.ai.json` file that does not exist — the actual generated file is `pbp_resonance_compass.ai.json` (wrong project name, see Gaps). |
| `blueprints/dbp_backend.ai.json`, `dbp_frontend.ai.json`, `pbp_resonance_compass.ai.json`, `layers/obp_*.ai.json` (6 files) | Auto-generated project-structure snapshots from `generate_blueprint.py`, last run 2026-06-28. Stale relative to current code (e.g., shows 1 Rust command, "Resonance Compass" as project name). |

### src-tauri/ (Rust / Tauri backend)
| File | Description |
|---|---|
| `Cargo.toml` | Crate manifest — tauri 2, plugins: opener, dialog, fs, sql(sqlite); serde/serde_json. Release profile strips symbols, opt-level "s", thin LTO. |
| `Cargo.lock` | Rust lockfile. |
| `build.rs` | Standard `tauri_build::build()` plus Android-specific linker flags to avoid a libc.a/rust_eh_personality duplicate-symbol conflict. |
| `src/main.rs` | Entry point — calls `resonance_echoes_lib::run()`. |
| `src/lib.rs` | App bootstrap: 3 SQL migrations (v1 create, v2 destructive recreate, v3 destructive recreate — all `DROP TABLE IF EXISTS` then recreate), registers sql/fs/opener/dialog plugins, exposes one Tauri command (`greet`, unused by the frontend — a scaffold leftover). |
| `tauri.conf.json` | App config — window 1280×800, identifier `com.audhd.resonance-echoes`, **`security.csp: null`** (no Content-Security-Policy). |
| `capabilities/default.json` | Tauri v2 capability grants — `core:default`, explicit `sql:allow-*` set (correctly fixes the documented ACL bug), `fs:default`, `opener:default`, `dialog:default`, `windows: ["*"]`. |
| `.gitignore` | Excludes `/target/` and `/gen/schemas`. |
| `icons/*` (24 files) | App icons — base PNGs/ICO/ICNS plus full Android mipmap set (5 densities × 3 variants) and iOS AppIcon set (14 sizes). |

### src/ (SvelteKit frontend)
| File | Description |
|---|---|
| `app.html` | SvelteKit HTML shell. |
| `app.css` | Imports Tailwind v4 + 3 of 7 generated COSMIC CSS files, global resets, reduced-motion and focus-visible rules. |
| `lib/types/types.ts` | `Echo`, `Sense`, `Subcategory`, `ThemeConfig` interfaces; re-exports `EmojiDef`. |
| `lib/stores/echo.svelte.ts` | SQLite-backed store (Svelte 5 runes) — `initDB`, `loadEchoes`, `addEcho`, `updateEcho`, `getEchoesBySense/ByEmoji`, `searchEchoes`, `purgeAll`. All queries are parameterized (no SQL-injection surface). Contains verbose `console.log`/`console.error` debug instrumentation left in from the Android ACL debugging session. |
| `lib/stores/theme.svelte.ts` | Theme store — localStorage-persisted `ThemeConfig`, preset/mode/font-size setters. |
| `lib/theme/theme.ts` | 6 preset themes (dark/warm/ocean/forest/sunset/amoled) and a `getThemeColors()` derivation function. |
| `lib/data/senses.ts` | 8 senses with subcategories (matches README/CLAUDE-CONTEXT). |
| `lib/data/emojis.ts` | 12 emoji definitions with poetic text + sensory lexicon (matches README claim of "12 emoji definitions"). |
| `lib/components/ComfortBar.svelte` | Footer greeting + quick-add bar, expand/collapse. |
| `lib/components/Sidebar.svelte` | Collapsible nav (Home/Insights/Settings), hamburger toggle, mobile backdrop. |
| `lib/components/GradientPulse.svelte` | Decorative glow wrapper used on the onboarding sigil. |
| `lib/cosmic/{index,colors,consciousness,dimensions,effects,motion,positioning,typography}.ts` | A large (~3,000-line) shared "COSMIC" design-token system imported wholesale from the sibling Sanctuary ecosystem. Only `colors.ts` (via `theme.ts`) is actually consumed by Echoes; the rest (consciousness tiers, council-member gradients/fonts for ~30 named entities, pagan/mystical/pride color palettes, 3D camera/zoom/parallax coordinate systems for a panorama UI Echoes doesn't have) is dead code for this app (see Gaps). |
| `lib/styles/custom_overrides.css` | Mostly-empty template for manual CSS overrides (Safari/Firefox fixes, commented-out examples). |
| `lib/styles/generated/{variables,animations,text_effects,domains,typography,parallax,zoom}.css` | 7 generated CSS files, 5,170 lines total. Only 3 (`variables`, `animations`, `text_effects` — ~1,419 lines) are imported by `app.css`; the other 4 (~3,751 lines: `domains.css`, `typography.css`, `parallax.css`, `zoom.css`) are unreferenced. `generate_blueprint.py` itself documents that `typography.css` and `domains.css` contain invalid CSS syntax. |
| `routes/+layout.svelte` | App shell — theme CSS vars, DB init, onboarding redirect gate, conditional Sidebar/ComfortBar. |
| `routes/+layout.ts` | `export const ssr = false` (required for Tauri's SPA mode). |
| `routes/+page.svelte` | Home — search/filter/sort timeline, Quick Log FAB, EchoCard list (inlined, not a separate component despite SCREEN-INVENTORY.md saying so). |
| `routes/add/+page.svelte` | Echo create/edit form — sense picker, progressive disclosure, emoji grid + disambiguation prompts, intensity, custom timestamp. |
| `routes/insights/+page.svelte` | 7 derived-only insight cards (top emojis, by-sense, streak, time-of-day, recent mood, intensity trend, patterns — gated at 20 echoes). |
| `routes/onboarding/+page.svelte` | 3-screen linear welcome flow (name → how-it-works → theme). |
| `routes/settings/+page.svelte` | Theme/font controls, export-as-JSON, double-confirm purge, uninstall guide, about block. |

### static/
10 image assets (app logos, avatar, framework logos) used across README/UI/onboarding.

### Generated/build directories (present locally, correctly gitignored from tracking *going forward*)
- `.svelte-kit/` — SvelteKit's generated types/routing glue.
- `node_modules/` — ~2,100+ npm packages.
- `src-tauri/target/` — Rust build cache (contains stale absolute paths from a prior repo location, see Gaps).
- `src-tauri/gen/` — only `gen/schemas` present; no `gen/android` scaffold currently materialized.

---

## 2. WHAT'S MISSING

- **No automated tests of any kind.** No `*.test.ts`, `*.spec.ts`, Vitest/Playwright config, or Rust `#[test]` functions anywhere in the repo. `CONTRIBUTING.md`'s "Build Protocol" relies entirely on `npm run check` (type-checking only) + `cargo build` + manual human testing on a physical Android device.
- **No CI configuration** — no `.github/workflows/`, no other CI provider config. Nothing enforces the "zero errors on check/build before merge" rule from `CONTRIBUTING.md` automatically.
- **No `reports/` directory** existed prior to this assessment.
- **No CHANGELOG.md** (RELEASE.md partially substitutes for this but only covers v1.0.0 and is already stale against CHECKLIST's "v1.1" entry).
- **No `.env.example`** despite `.gitignore` explicitly carving out an exception for one (`!.env.example`) — implies env vars were anticipated but none exist/are documented.
- **No CODE_OF_CONDUCT.md or SECURITY.md** — common "standard Sanctuary files" given CONTRIBUTING.md references a shared `sanctuary-standards` repo, but no local security-disclosure policy exists.
- **EmojiGrid and SensePicker components**, listed as planned in `docs/SCREEN-INVENTORY.md`, were never extracted — their logic lives inline in `routes/add/+page.svelte` instead. Not a functional bug, but the docs and code structure disagree.
- **No screenshots** — README's screenshots section is still a placeholder.
- **No rate limiting / size caps beyond UI-level `maxlength`** on note/name fields — acceptable for a fully local single-user app, but worth knowing if any sync/multi-user feature is ever added.

---

## 3. GAPS — stated vs. actual

1. **`CLAUDE.md` describes the wrong project.** It documents the "Resonance Knowledge System" (Rust+SQLite+MCP grammar server with `src/schema/`, `src/seed/`, `src/mcp/`) — none of which exists in this repo. This repo is a SvelteKit+Tauri journaling app. The instructions Claude is asked to follow at session start do not match the codebase.
2. **`docs/SCREEN-INVENTORY.md` is stale by several phases.** Says "Built (0) — None yet" while `CHECKLIST.md` and the actual route files show all 5 screens fully implemented and "Tested: ✅" through v1.1.
3. **`docs/blueprints/` is auto-generated from a misconfigured script and is stale.** `generate_blueprint.py` is a verbatim copy of the Compass project's generator: its `detect_phase()` phase map (`library`, `nowplaying`, `playlists`, `equalizer`, `sattva`, etc.) has zero overlap with Echoes' actual routes (`home`, `add`, `insights`, `onboarding`, `settings`), so every route blueprint reports `phase: 99` (unmapped). The top-level project blueprint is literally titled `"project": "Resonance Compass"`. `docs/blueprints/INDEX.md` references a `pbp_resonance_echoes.ai.json` that doesn't exist — the real file is `pbp_resonance_compass.ai.json`.
4. **Backend command count mismatch.** The generated blueprint reports only 1 Rust command (`greet`) registered — accurate, but it means all of CHECKLIST's claimed backend work (`add_echo`, `get_echoes` mentioned in `BUILD-SEQUENCE.md` Phase 1) was actually implemented as direct `@tauri-apps/plugin-sql` calls from the frontend store, not as custom Tauri commands. The docs (BUILD-SEQUENCE.md Phase 1: "Rust command: `add_echo`, `get_echoes`") describe an architecture that was never built; the real implementation (raw SQL from `echo.svelte.ts`) works but diverges from the plan.
5. **The Resonance Grammar is referenced but not implemented.** README and `docs/RESONANCE-GRAMMAR.md` describe Echoes as a "reference implementation" of a shared atom/molecule/folksonomy grammar, but the emoji/sense data (`emojis.ts`, `senses.ts`) is hand-written, app-local TypeScript with no schema linkage to the Grammar spec — no shared package, no grammar-validation, no folksonomy persistence of per-vessel emoji redefinitions described in the Grammar doc.
6. **`RELEASE.md`'s "v1.1 Planned"** section lists features (progressive disclosure, disambiguation prompts, "I don't know" option, predictability audit) that `CHECKLIST.md` already marks complete under a "v1.1" heading — the release notes were not updated after v1.1 shipped.
7. **Two dev-only files are `.gitignore`'d but still tracked in git** (`helper.md`, `generate_blueprint.py`) — they were committed before being added to `.gitignore`, so the ignore rule has no effect; both still show up in `git ls-files`.
8. **`cargo check` currently fails** (see Test Readiness) due to a stale absolute path baked into the local `target/` build cache from before the repo was moved from `C:\_superposition\resonance-echoes` to `C:\_superposition\AudHDities-Resonance\resonance-echoes`. `CONTRIBUTING.md` requires "`cargo build` — zero errors" before every merge; that gate is not currently passable on this machine without a clean rebuild.

---

## 4. VULNERABILITIES

| Severity | Finding | Detail |
|---|---|---|
| **High (repo hygiene)** | **`node_modules/` (2,105 files) and `.svelte-kit/` (15 files) are committed to git history**, despite being listed in `.gitignore`. `.gitignore` only prevents *new* tracking — these were added before the ignore rule existed and were never `git rm --cached`'d. This bloats `.git` to ~107 MB, slows clones, and risks committing transitive-dependency license/security issues directly into history. **Recommendation: `git rm -r --cached node_modules .svelte-kit` + commit.** |
| Low | `npm audit`: 3 low-severity advisories, all from `cookie <0.7.0` (GHSA-pxg6-pf52-xh8x, "accepts out-of-bounds characters in cookie name/path/domain"), pulled in transitively via `@sveltejs/kit`. Fix requires a semver-major bump of `@sveltejs/kit`/`adapter-static`. Low real-world impact for a local-only Tauri app with no server-rendered cookie handling. |
| Low–Medium | `tauri.conf.json` sets **`security.csp: null`** — no Content-Security-Policy is enforced in the Tauri webview. For a fully offline, no-remote-content app this is lower risk, but it's still a defense-in-depth gap (e.g., if any future feature ever renders untrusted HTML/markdown from echo notes). |
| Informational | `capabilities/default.json` grants `fs:default` (full default filesystem-plugin scope) even though the app only needs SQLite (sandboxed automatically) — no observed use of `@tauri-apps/plugin-fs` in `src/`. Export uses a browser `Blob`/`<a download>`, not the Tauri fs plugin. The fs capability appears unused and could be narrowed or removed to reduce attack surface. |
| Informational | `capabilities/default.json` and `tauri.conf.json` both reference `$schema` URLs hosted at `raw.githubusercontent.com/nickknapton88/tauri-docs/...` rather than the official `schema.tauri.app`. These are editor-validation-only references (not fetched at build/runtime) but point at a third-party fork — worth pointing back at the official schema source. |
| Informational | Debug `console.log`/`console.error` calls left throughout `echo.svelte.ts` and `add/+page.svelte` (instrumentation from the Android ACL bug hunt documented in `CLAUDE-CONTEXT.md`/`CHECKLIST.md`). Not a security issue on its own (no secrets logged — only echo content, which is the user's own local journal data), but it is production console noise that should be stripped or gated behind a debug flag before a store release. |
| Informational | `Cargo.toml`'s `[lib]` exposes `crate-type = ["staticlib", "cdylib", "rlib"]` and SQL migrations v2/v3 are destructive (`DROP TABLE IF EXISTS echoes` then recreate) — fine for the dev iteration they were used for, but any future migration must not reuse this destructive pattern against a database that already has user data, or it will silently delete all journal entries. |

No SQL injection surface was found — every query in `echo.svelte.ts` uses parameterized placeholders (`$1`, `$2`, …) via `tauri-plugin-sql`.

---

## 5. TEST READINESS

- **Type/lint check (`npm run check`): ✅ PASSES.** Verified live — `svelte-kit sync && svelte-check` reports **0 errors, 0 warnings** across 290 files.
- **Rust build (`cargo check` in `src-tauri/`): ❌ FAILS** in the current environment:
  ```
  failed to read plugin permissions: failed to read file
  '\\?\C:\_superposition\resonance-echoes\src-tauri\target\debug\build\tauri-...\out\permissions\app\autogenerated\commands\app_hide.toml'
  (os error 3)
  ```
  This is **not a code defect** — it's a stale `target/` build cache referencing the repo's old path (`C:\_superposition\resonance-echoes`, missing the `AudHDities-Resonance` segment that exists in the current path and in `.claude/settings.json`'s now-stale `additionalDirectories`/permission entries). A clean `cargo clean && cargo check` (or `cargo build`) from the current path should resolve it; this assessment did not perform that clean/rebuild since it would modify local build state beyond the scope of a read-only audit.
- **No automated test suite exists** — nothing to run beyond the two checks above. All functional verification to date has been manual, on-device Android testing per `CHECKLIST.md`'s session log ("Tested: ✅" entries are human-attested, not test-suite-attested).
- **No CI** to catch regressions on push/PR.
- `npm audit` and `npm outdated` both ran cleanly (read-only) — see Vulnerabilities/Recommendations.

---

## 6. RECOMMENDATIONS (priority-ordered)

1. **Untrack `node_modules/` and `.svelte-kit/` from git** (`git rm -r --cached node_modules .svelte-kit && git commit`). This is the single biggest piece of repo bloat (93% of tracked files) and the fix is low-risk/non-destructive to working files.
2. **Resolve the `cargo check` failure** by clean-rebuilding `src-tauri/target/` from the current path, then re-verify `CONTRIBUTING.md`'s "zero errors on `cargo build`" gate actually passes before the next merge.
3. **Fix or retire `generate_blueprint.py`.** Either rewrite its phase map / project name for Echoes (replacing the leftover Compass-specific `detect_phase()` table) or delete it and `docs/blueprints/` if blueprints aren't actively used by Claude sessions anymore — stale auto-docs are worse than no docs.
4. **Untrack `helper.md` and `generate_blueprint.py`** from git (`git rm --cached`) to make the existing `.gitignore` entries actually effective, or remove those `.gitignore` lines if you want them tracked.
5. **Replace `CLAUDE.md`** with content that actually describes Resonance Echoes (SvelteKit+Tauri journal), not the unrelated Resonance Knowledge System grammar server — this is the file every future Claude session reads first, and it's currently pointing at the wrong project.
6. **Refresh `docs/SCREEN-INVENTORY.md` and `RELEASE.md`** to match `CHECKLIST.md`'s actual (further-along) state, or delete them in favor of CHECKLIST as the single source of truth (CLAUDE.md itself says "One definition per object — defined once, referenced everywhere").
7. **Add a minimal automated test layer.** Given the app is mostly pure derivation logic (insights calculations, filter/sort logic in `+page.svelte`, `echo.svelte.ts` CRUD), even a small Vitest suite around the `$derived.by` logic and store functions would catch regressions that manual Android testing currently has to catch by hand.
8. **Add CI** (GitHub Actions) running `npm run check`, `npm audit`, and `cargo check` on push/PR so the `CONTRIBUTING.md` quality gate is enforced automatically rather than trusted to manual discipline.
9. **Strip or gate the debug `console.log` instrumentation** in `echo.svelte.ts` and `add/+page.svelte` behind a dev-only flag before the next store release.
10. **Set an explicit CSP** in `tauri.conf.json` instead of `null`, and audit whether `fs:default` is still needed given export uses Blob/download rather than the Tauri fs plugin — narrow capabilities to least-privilege.
11. **Bump dependencies** — `typescript` (5.6→6.0), `vite` (6→8), `@sveltejs/vite-plugin-svelte` (5→7) are all behind; `npm audit fix` (semver-major) would also clear the 3 low-severity `cookie` advisories via `@sveltejs/kit` upgrade. None are urgent (all low severity / no known exploits against this app's usage), but worth scheduling.
12. **Point `$schema` references** in `tauri.conf.json`/`capabilities/default.json` back to the official `https://schema.tauri.app/...` URLs instead of a third-party fork.
