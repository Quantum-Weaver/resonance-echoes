# Echoes — AudHDities storefront pack

*Founded 2026-08-21 by **Plumb** 🕯️ · Opus (Claude), truly `claude-opus-5[1m]`,
at KP's ⚛ word — *"might as well create a AudHDities storefront pack too as
everything will also be in our storefont."* The fourth of the four storefronts,
and the only one the house owns outright.*

> **This one is different, and the difference is the point.** The other three
> packs are drafts a hand transcribes into somebody else's console, under
> somebody else's rules. **Here there is no console, no reviewer, no
> questionnaire and no gatekeeper** — which means nothing forces honesty except
> the house's own covenant. So this pack carries the disclosures the other three
> are never asked for.

> **GATE.** This storefront *is* the Launch Sequence gate. Echoes and Compass
> publish together, and only once the rebuilt AudHDities site is live. KP,
> 2026-08-21: *"if it is not ready, the beacons are leading no where."*

## What ships here

All three, always — **the desktop law is not optional** (KP, 2026-07-18: *"i want
to always build the desktop versions, many neurodivergent folk cannot hold
phones in their hands."*).

| platform | artifact | note |
|---|---|---|
| Windows | `Resonance Echoes_<ver>_x64_en-US.msi` | the managed installer |
| Windows | `Resonance Echoes_<ver>_x64-setup.exe` | the friendlier one for most people |
| Android | `resonance-echoes-v<ver>.apk` | direct sideload — Play's AAB does not apply here |

Stowed for upload by `python resonance-ziggy/modules/shipwright/stow-release.py resonance-echoes`,
which mirrors them into `resonance-assets/releases-current/echoes/bundle/`.

**Not shipped here:** the `.aab` (a Play upload format, useless to a human) and
the `.apk.idsig` (an `adb install --incremental` receipt, useless to a human).

## The two disclosures no other storefront will ask for

**1. The Windows installers are not code-signed.** Verified, not assumed:
`Get-AuthenticodeSignature` returns `NotSigned`. **Windows SmartScreen will warn
on install.** A person downloading a journal for hard days should not meet a
scary blue box with no warning and no explanation. Say it plainly on the page,
before the download button — something like:

> Windows will warn you about this installer. That warning means the file isn't
> signed with a paid certificate — not that anything is wrong with it. You can
> check the file yourself against the checksum below. We'd rather tell you than
> let the box surprise you.

**2. Android will call it an unknown source.** Sideloading an APK requires
allowing installs from your browser or file manager. Same principle: say it
first, in plain words, with the steps.

## Checksums — publish them

The other three storefronts verify signatures for the user. **Here, nobody does
it but us**, so the page carries a SHA-256 per file and the one-line command to
check it:

```
Windows    Get-FileHash .\<file>
Android    sha256sum <file>
```

`stow-release.py` verifies sha256 on every copy it makes; the same digest is
what belongs on the page. *A sovereign download that cannot be verified is a
promise with no receipt.*

## Copy for the page

**Name:** Resonance Echoes

**One line:** A sovereign journal for logging anything with feeling. No cloud. No account. Yours.

**The description:**

Echoes is your space to log moments with feeling — a song that moved you, a
dream you remember, a thought that won't let go, gratitude you don't want to
forget. Name it, choose a sense, tag an emoji, set an intensity. Two taps if
that's all today has in it.

Built neurodivergent-first: gentle insights that are invitations and never
scores, a "Not Sure" option everywhere because uncertainty is a valid entry, a
form that starts tiny and grows only when you're ready, and a timer that waits
with you.

Sovereign by architecture, not policy: no account, no cloud, no analytics, ever.
Everything lives in a local database on your device. Export your whole journal —
echoes and your own emoji meanings — as one open JSON file, and import it back
any time. The purge truly purges.

**Price: free. Everywhere, forever** — one-way and intentional.

**What it is not** *(KP's ⚛ ruling, 2026-08-21, verbatim)*: *"echoes does not
need to be a heath app, it is simply a beacon to the world to help people better
understand themselves through emojis and moment tracking."* Not a medical
device, not a diagnostic tool, not therapy. A journal that takes emoji seriously.

## Privacy

Collects nothing. Shares nothing. All data local to the device; export is
user-initiated; deletion is built in and complete. The policy stands at
`PRIVACY.md` in the repo and is the same document declared to Play and Galaxy —
**one published promise, one address**.

## What this page owes that a store does not

- **The version and its date**, visible — so nobody downloads a stale build without knowing.
- **The changelog**, or a link to it.
- **A link to the source.** The Resonance License is the whole argument; a
  storefront that hides the source undercuts it.
- **No account, no email capture, no analytics on the download.** The download
  page must keep the promise the app keeps.
- **The address ward:** `audhdities.com` and the Proton address only. Never a home address.

## Track log

| Date | State |
|---|---|
| 2026-08-21 | Pack founded. Site rebuild pending — nothing published. |

— **Plumb** 🕯️
