# ECHOES — THE FEATURE BOARD
*Assembled 2026-07-19 (the workspace honoring). Echoes is the family's
reference implementation and is COMPLETE for its purpose — this board is
short because the app is finished, not because it is neglected.*

## 🔴 OPEN — TOP OF THE LIST (KP's word, 2026-07-21)

*Two data-sovereignty defects, found in a review at KP's asking and **verified in
code, not inferred**. They are placed above the maintenance section deliberately:
**this board's "fully green / deliberate rest" declaration below is suspended
until these close.** It was true when written on 07-19. It is not true now, and a
board that says finished while sovereignty leaks would be the worst kind of
stale.*

- [ ] **E1 · Export ships at most 200 echoes — and export-then-purge destroys the
  rest.** `loadEchoes()` is only ever called with its defaults (`LIMIT 200 OFFSET
  0`); there is no pagination anywhere and `echoes` is *replaced* on each load,
  never accumulated. `exportData()` serialises that one page. Settings meanwhile
  shows `echoStore.totalCount`, a true `SELECT COUNT(*)` — so a vessel with 300
  echoes **sees 300 and exports 200**. The purge flow runs
  `if (pendingExport) exportData()` and then deletes everything, so the
  export-and-purge path **silently loses everything past the newest 200.**
  Contradicts the README's *"Export everything as JSON with one tap"* and
  Resonance License §7. *Not yet bitten — the app is three weeks old — which is
  exactly why now.* **Fix: export queries the database directly; never the
  loaded page.**

- [ ] **E2 · The folksonomy is purgeable but not exportable.** Personal emoji
  definitions live in `localStorage` under `emoji_def_*`. `exportData()`
  serialises only the echoes table. Purge calls `localStorage.clear()`. So a
  vessel's **own meanings can be destroyed but never carried out** — asymmetric
  in the worst possible direction, and it is the one part of the data that is
  irreplaceable: echoes can be re-lived, a private lexicon cannot be
  reconstructed.

- [ ] **E3 · One envelope — KP's ruling, same breath:** *"we need folksonomy and
  echoes to export in the same manner."* Export becomes a single versioned
  envelope carrying **both** — echoes *and* definitions — so the two can never
  drift apart again, and so purge and export cover exactly the same ground.
  *This is also the vessel-graphs seed's cheap win #2 (a common export envelope,
  schema-versioned and app-namespaced) getting its first real instance — the
  shape chosen here is the one the family should inherit, so choose it as though
  Compass and Skapa are watching, because they are.*

- [ ] **E4 · There is no import.** Verified: no `importData`, no file input, no
  `readAsText` anywhere in `src/`. **CLAUDE.md's structure block advertises
  "export/import."** Data can leave and never return. Not required by §7, which
  asks only that data be exportable — but "the same manner" reads oddly when
  only one direction exists, and a round-trip is what makes an envelope worth
  versioning. *Scope is KP's call; noted rather than assumed.*

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
