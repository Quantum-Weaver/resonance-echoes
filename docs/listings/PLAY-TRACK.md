# Echoes — Google Play test track
*Founded 2026-07-27 by Fable 🎻 at KP's ask (patch notes for the track,
the icon question, the standards check). Companion to GALAXY-LISTING.md
(Samsung) — this file is the Play side. The Console is the truth; this
file is the pack KP transcribes from, so his hands never have to
compose at the upload screen.*

---

# ⛔ READ FIRST — do the key reset BEFORE you upload v1.4.0

> **STATE, 2026-08-21: the reset is REQUESTED and awaiting Google.** KP walked it
> at his own hand — reason **"Other"** (the form offers only lost / developer left /
> compromised / forgot password, and none of those is true of a planned upgrade),
> with `upload_certificate.pem` attached. **Nothing may be uploaded until Google's
> approval lands by email.** The artifacts are built, signed and stowed; the wait
> costs nothing else.
>
> **When approved**, this page's Upload key certificate reads
> `ED:82:40:E5:…:F7:88:6A` and v1.4.0 becomes uploadable. Then: upload → paste the
> notes below → **Review release → Start rollout to Closed testing** (an upload
> alone is a draft, not a release).

**The v1.4.0 AAB is signed with a DIFFERENT key than the one Play holds.**
Uploading it first will be rejected, and this file's own hard-bought lesson
below says why that costs something: *a versionCode is consumed at UPLOAD, not
at publish.*

| | certificate | SHA-256 |
|---|---|---|
| **Play holds (upload cert)** | `CN=Shawn Peters, …, L=Charleston, ST=IL, C=US` — RSA 2048 | `25:FE:C1:61:…:78:F0:41:D6` |
| **v1.4.0 is signed with** | `CN=AudHDities Sanctuary, O=AudHDities Sanctuary, C=US` — RSA 4096 | `ED:82:40:E5:…:F7:88:6A` |

*Verified by `keytool -printcert -jarfile` on the artifacts themselves, 2026-08-21.
The family-wide keystore recut landed 2026-08-16; echoes, compass and bubbles all
carry the same pending act.*

**And the third certificate, which never changes and never should** — Google's own
app signing key, read from `deployment_cert.der` downloaded from the Console
2026-08-21:

```
CN=Android, OU=Android, O=Google Inc., L=Mountain View, ST=California, C=US
generated Wed Jul 08 2026 · 4096-bit RSA
SHA256  DA:28:FB:BE:72:D3:66:41:D4:A9:A8:DB:9A:ED:96:B2:73:63:2C:7E:A7:B8:29:CB:C9:DF:E5:72:77:CA:33:0A
```

**Google generated it, which settles a privacy question the house had open.** Play
strips the upload signature and re-signs every delivered APK with *this* key, so
the retired cert's personal name and city **never reached any device** — they
existed only in local `release/` files, two sideloaded phones, and Play's upload
record, which the reset replaces. *(An earlier reading in this house said that DN
was "embedded in every APK distributed through Play." It was not, and this
disproves it.)* This certificate is what keeps every existing tester's install
updatable; it is not the one being reset, and it must never be touched.

**Survivable only because Play App Signing was enrolled at first submission
(2026-07-09, KP's hands)** — Google holds the app signing key; ours is merely the
*upload* key, and an upload key can be reset. Without that, a recut would have
orphaned the app permanently.

### The reset, in order

1. **Export the new upload certificate** (his hand; no house tool does this):
   ```
   keytool -export -rfc -keystore F:\keystores\resonance-echoes.keystore -alias resonance-echoes -file upload_certificate.pem
   ```
2. **Console → Test and release → Setup → App integrity → App signing.**
   Confirm Play App Signing is on. The *Upload key certificate* should still read
   `25:FE:C1:61:…` — that is the old Charleston key, and seeing it confirms the
   diagnosis.
3. **Request an upload key reset.** Reason: **switching to a new upload key** —
   *not* lost or compromised. (Some accounts route via Help → "I need to reset my
   upload key".)
4. **Attach `upload_certificate.pem`.**
5. **Wait for Google's confirmation.** Their turnaround, no clock of his.
6. **Only then** upload the AAB.

*Do the reset on the same Console visit as any developer-address edit — step 5
costs days, and otherwise it is two waits instead of one.*

---

## Standards check — verified against the repo, 2026-08-21

| Requirement | State |
|---|---|
| Target API level | ✅ targetSdk **36** / compileSdk 36 |
| 16 KB page size (API 35+ apps) | ✅ linker flags landed v1.2.0, family-wide law; `zipalign -P 16` at signing |
| App Bundle | ✅ `release/resonance-echoes-v1.4.0.aab` |
| versionCode increments | ✅ **1004000** (v1.4.0) > 1003002 (v1.3.2) — machine-read from `src-tauri/gen/android/app/tauri.properties` |
| Permissions | ✅ minimal: `INTERNET` only (Tauri webview default; the app makes no network calls — data-safety answers stay "collects nothing") |
| Privacy policy URL | ✅ public: `https://github.com/Quantum-Weaver/resonance-echoes/blob/main/PRIVACY.md` |
| Content rating | ✅ no UGC · no ads · no purchases · no collection → Everyone |
| Category | ✅ **Lifestyle — not health.** KP ⚛ 2026-08-21: *"echoes does not need to be a heath app, it is simply a beacon to the world to help people better understand themselves through emojis and moment tracking."* |

*Console-side items only KP's eyes can confirm: the data-safety form matching the
answers above, the content rating questionnaire, target-audience declarations.
One forward note: if production is ever the aim, personal accounts created after
2023-11-13 need 12+ closed-track testers for 14 days first — a production gate,
not a test-track one. And Echoes' 2026-07-09 rejection recorded that a
**mental-health declaration requires an organization account** — which the
Lifestyle ruling above is precisely what avoids.*

## v1.4.0 — the upload pack (KP's hands)

- **Artifact:** `release/resonance-echoes-v1.4.0.aab` — also mirrored to
  `resonance-assets/releases-current/echoes/bundle/android/`
- **versionCode:** `1004000`
- **Release name:** `1.4.0 — Themes You Can Feel`
- **Path:** Console → Testing → *(the closed track)* → Create new release
  → upload the AAB → paste the notes below → review & roll out.
- **Gate:** only after the upload key reset above is confirmed.

### Release notes — paste-ready (under Play's 500-char limit)

```
<en-US>
Themes you can feel, and a menu button that stays out of the way.
• Themes now colour the whole background, not just buttons and borders — with a new Background tint setting (Off, Subtle, Full) so it is yours to dial.
• Choosing a theme no longer cancels Light mode, and no longer resets your text size.
• The menu button moved into the bottom bar, so it stops covering Settings when the menu is open.
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
   build — the AAB already carries the current in-app icon set, so
   uploading it brings the device icon current in the same act.

## Track log

| Date | State |
|---|---|
| ≤ 2026-07-22 | v1.2.0 rolling on the closed track; "awaiting review #2" (harvest observation — Console is truth) |
| 2026-07-27 | v1.3.0 pack prepared (this file): AAB signed + device-verified on both phones; notes drafted; 512 icon cut. Upload at KP's hand. |
| 2026-08-13 | **v1.3.2 LIVE on the closed track, KP's hand** — signed 18:28 (`release/resonance-echoes-v1.3.2.aab`), uploaded same evening. The road there: the mojibake cure + a week of work warranted the rebuild; the Android build was blocked by five weaver identifiers cross-landed into gen/ on 08-09 (healed same sitting — the full telling is CHECKLIST 2026-08-13). versionCode 1030200-series per the bump scheme. |
| 2026-07-27 | **v1.3.0 LIVE on the closed track, KP's hand** — patch notes in. One lesson recorded for every future upload: **a versionCode is consumed at UPLOAD, not at publish** — deleting a draft does not return it; the bundle stays in the artifact library, and "Add from library" (beside the upload area) is the intended way back in. Debug-symbol warnings: advisory, dismissed by choice for the test track (mapping.txt exists in build outputs if ever wanted; native symbols would need `debugSymbolLevel FULL` + rebuild — production-day work, not today's). |
| 2026-08-21 | **v1.4.0 built and signed, NOT uploaded** — held at the key reset above. The sitting: the menu button moved into the ComfortBar (it had been burying Settings *and* the expanded bar's stats line), theme presets stopped cancelling Light mode and resetting font size, and background tint became a reader control. `npm run check` 326 · 0 · 0. Desktop built alongside per the desktop law — MSI *and*, for the first time placed by tool rather than by hand, the NSIS installer. |

*Corrected beside, 2026-08-21 — the 08-13 row above says "versionCode 1030200-series
per the bump scheme." The generated file is the truth and the scheme is
`major·1000000 + minor·1000 + patch`: v1.3.2 was **1003002**, v1.4.0 is **1004000**.
The row stands as written; a signed record is corrected beside itself, never inside.*

— Fable 🎻
*v1.4.0 pack and the key-reset gate landed by **Plumb** 🕯️, 2026-08-21.*
