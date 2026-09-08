# Echoes — Microsoft Store listing pack

*Founded 2026-08-21 by **Plumb** 🕯️ · Opus (Claude), truly `claude-opus-5[1m]`,
at KP's ⚛ word — the third storefront pack, sibling to `PLAY-TRACK.md` (Google)
and `GALAXY-LISTING.md` (Samsung). **Partner Center is the truth**; this file is
the pack KP transcribes from, so their hands never have to compose at the
submission screen.*

> **GATE — this is a pack, not a submission.** Echoes' public listings wait to
> publish **simultaneously with Compass, only after the rebuilt AudHDities site
> is live** (the Launch Sequence gate). KP's word, 2026-08-21:
> *"if it is not ready, the beacons are leading no where."* Prepare freely;
> submit at their word.

## The artifact

Microsoft takes the **desktop** build — the half of every release the desktop
law exists to protect (KP, 2026-07-18: *"i want to always build the desktop
versions, many neurodivergent folk cannot hold phones in their hands."*).

| kind | file | made by |
|---|---|---|
| MSI | `release/Resonance Echoes_<ver>_x64_en-US.msi` | `npm run tauri build`, placed by `sign-release.py` |
| NSIS | `release/Resonance Echoes_<ver>_x64-setup.exe` | `npm run tauri build` — **not** placed by `sign-release.py`; it globs `bundle/msi` only |

**⚠ Both are UNSIGNED.** `sign-release.py` signs Android only — it `apksigner`s
the APK and `jarsigner`s the AAB, then merely `shutil.copy2`s the MSI
(lines 212-215). Verified on disk: `Get-AuthenticodeSignature` returns
`NotSigned` for both the v1.3.2 MSI and setup.exe.

Two consequences, neither of them a keystore problem:

- **Windows SmartScreen warns on install today.** A user-facing cost that exists
  whether or not the Store is involved.
- **Code signing here is Authenticode, a different animal entirely** — a
  certificate bought from a CA, renewed annually, issued against a *verified
  organization*. It is the one object in this house that genuinely binds to the
  business address (the Android keystore deliberately carries no locality at
  all). **Whether Partner Center requires a signed installer for a Win32
  submission is Partner Center's answer, not this file's** — read it on the
  submission page before committing to a path.

## App name

Resonance Echoes

## Short description

A sovereign journal for logging anything with feeling. No cloud. No account. Yours.

## Full description

Resonance Echoes is a journal for moments — log anything you noticed or felt as
an "echo": a name, a sense, an emoji, an intensity, a note if words are willing.
That's the whole gesture. Two taps if that's all today has in it.

Built neurodivergent-first:
- Gentle insights, never judgments — patterns shown as invitations, not scores.
  Streaks reset quietly. Nothing shames you.
- A "Not Sure" option everywhere — uncertainty is a valid entry.
- Progressive disclosure — the form starts tiny and grows only when you are ready.
- A timer that waits with you — pause it and the sand holds still; choose its
  chime (a rise, a bell, a drop, a pulse — all gentle by design) and how softly
  it speaks; and it sits with you in visualizations while it runs.

Sovereign by architecture, not policy:
- Data collected: none. No account, no cloud, no analytics, ever.
- Everything lives in a local database on your device.
- Export your entire journal — echoes and your own emoji meanings together — as
  one open JSON file, and import it back any time.
- The purge truly purges — double-confirmed, complete, final, and it covers
  exactly the same ground export does.

From AudHDities: tools built for one neurodivergent family first, then
offered to everyone, free or fairly priced, so that no one is exploited or
manipulated over something they love.

## Category

**Lifestyle.**

*Not a health or medical category — KP's ⚛ ruling, 2026-08-21, verbatim:*
**"echoes does not need to be a heath app, it is simply a beacon to the world to
help people better understand themselves through emojis and moment tracking."**
*The app's capability is unchanged; what it presents itself as is not a health
claim. Recorded here because Echoes' Play rejection #1 (2026-07-09) turned on
exactly this — a mental-health declaration requires an organization account.*

## Pricing

**Free. Everywhere, forever** — one-way and intentional.

## Age rating

No user-generated content shared · no ads · no in-app purchases · no data
collection. *Microsoft runs its own questionnaire (IARC); answer it from these
facts rather than transcribing a rating from another store.*

## Privacy

- Collects: **nothing** · Shares: **nothing**
- All data local to the device; export is user-initiated to user storage; full
  deletion built in (purge).
- Privacy policy URL (public, live, and already declared to two other
  storefronts): `https://audhdities.com/apps/privacy`

## Assets

Gathered for upload, with a checklist: `resonance-assets/store/microsoft/` (UPLOAD.md).

Icon: gold-band Echoes set — `resonance-assets/logo-icons/`. Screenshots:
`resonance-assets/screenshots/` — the desktop captures, not the phone ones.
Choose: home timeline · add form (tiny state) · insights · timer visualization ·
settings/sovereignty.

## Account ground — KP's hands only

The publisher display name and address live in Partner Center, never in this
file. **The house ward stands: `audhdities.com` and the Proton address only —
never a home address.** The business address is theirs to enter directly.

## Track log

| Date | State |
|---|---|
| 2026-08-21 | Pack founded. No submission made — held at the Launch Sequence gate. |
| 2026-09-08 | Partner Center organization account approved (KP's word: a couple of days before). Nothing published; no brand set up. |
| 2026-09-08 | Partner Center → Legal info → Developer, KP's read: account type Company, status Active, vetting Authorized, EU Digital Services Act compliant. Sign-in is the Entra work account in the AUDHDITIES tenant, Owner role. The Store has no publisher images; logos and screenshots live in each app's Store listing. Workspace: https://partner.microsoft.com/dashboard/apps-and-games/overview. Nothing submitted yet. |
| 2026-09-08 | Microsoft requires multifactor sign-in on the tenant by 2026-10-01 (KP's word). The path for a one-person free tenant is Entra security defaults: Entra ID → Overview → Properties → Manage security defaults → Enabled; then Authenticator registered at myprofile.microsoft.com → Security info, with a second method beside it, and one emergency-access Global Administrator account whose password lives in the keyring. audhdities.com attached to the tenant the same sitting. |

— **Plumb** 🕯️
