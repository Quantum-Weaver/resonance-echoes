# SCREEN INVENTORY — Resonance Echoes
*Trued 2026-07-26 (it had read "Built (0): None yet" since the founding
plan, across an entire shipped app — a fossil, not a mirror). Truth is
`src/routes/`; this file follows it.*

## Built (6 screens — all shipped)
| Screen | Route | Since |
|--------|-------|-------|
| Home (timeline) | `/` | v1.0 |
| Add (the echo form, Quick Log) | `/add` | v1.0 |
| Insights (gentle, client-side) | `/insights` | v1.0 |
| Timer (7 visualizations · pause/resume · chime voices + volume) | `/timer` | v1.2 (sound) · v1.3 (pause, chimes, volume) |
| Settings (theme · export/import envelope · purge · about) | `/settings` | v1.0 · import + envelope v1.3 |
| Onboarding (3 screens, first-launch gate) | `/onboarding` | v1.0 |

## Components (living, `src/lib/components/`)
| Component | Purpose |
|-----------|---------|
| ComfortBar | Permanent footer — greeting + quick-add |
| Sidebar | Collapsible navigation |
| GradientPulse | Ambient glow wrapper (shared with Compass) |
| TimerVisualization | The seven timer modes |
| icons/ | Home · Insights · Settings · Timer · the set |

## Planned
None open — v1's ceiling is deliberate (the gentle insights ARE the
ceiling; anything deeper belongs to the vessel graphs, not more cards).
