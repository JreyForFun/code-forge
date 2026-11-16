# Research: Pixel Learn 1.0

## R1. Asset source and licence

- **Decision**: Use the Pixelrepo Free collection (https://pixelrepo.com/collections/free) for new icons:
  heart, crown, banners, hourglass, chalice, silver cup, lantern, campfire, medallion, sealed letter,
  money bag, gold coins, wizard hat, knight/bronze helmets, bow, axe, hammer, spellbook, console, CPU,
  keyhole, gear, mystic figure, lightning scroll, magic potion, candle flame.
- **Licence**: Pixelrepo Free Asset License v1.0 (15.07.2025) allows commercial and personal use in
  apps and games, and allows modification. It requires visible credit ("Assets by Pixelrepo
  (https://pixelrepo.com)"). It forbids reselling or redistributing the assets themselves and forbids
  using them for AI training. Pixelrepo may revoke the licence.
- **Implications**: Embed the PNGs with `include_bytes!`, so the release zip contains no loose asset
  files. Credit in Settings and the README. Note in the README that a public source release must
  remove or replace `assets/art/`.
- **Alternatives considered**: Kenney (CC0) packs fit less well with the existing RPG item style.
  Drawing everything in code doesn't match the quality of the existing Pixelrepo items.

## R2. Exe icon and console window

- **Decision**: `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` to hide the
  console in release. Use the `winresource` build-dependency to embed `assets/icon.ico` and the
  version info. The Windows SDK `rc.exe` (10.0.22621) is present. The runtime window icon is set via
  `ViewportBuilder::with_icon` from `assets/icon.png`.
- **Alternatives**: `embed-resource` (needs a hand-written .rc file); no exe icon (fails FR-025).

## R3. Save robustness

- **Decision**: Keep the existing line-based `key=value` format (human-readable and already in the
  field). Add a `version=2` first line and `key=value` keys for new data (`avatar`, `onboarded`,
  `course`, `ach=<id>|<day>`, `log=<day>|<text>`). Write to `save.tmp`, copy the old `save.txt` to
  `save.bak`, then rename `tmp` → `txt`. Unknown keys are ignored; a missing version means legacy v1.
- **Alternatives**: JSON via serde (adds 2 deps and a migration of existing saves for no user gain).

## R4. SQL support scope

- **Decision**: Write a hand-rolled tokenizer and evaluator for one-table `SELECT` covering exactly
  what the course teaches: column list or `*`, aggregates `COUNT(*)/COUNT(col)/SUM/MIN/MAX/AVG`,
  one `WHERE` comparison (`= != <> < <= > >=`) on number or quoted text, `GROUP BY` one column,
  `ORDER BY` one column with `ASC/DESC`, and `LIMIT n`. Keywords are case-insensitive. Sample tables:
  `heroes(name, class, hp, level)` and `items(name, type, price)`.
- **Alternatives**: Embedding SQLite via rusqlite (adds a C build dependency and binary size, and is
  overkill for teaching examples).

## R5. CSS preview scope

- **Decision**: Parse `selector { prop: value; ... }` blocks. Output one line per rule
  (`selector → prop: value`) and a "sample" line styled with `color`, `background-color`,
  `font-size` (px) and `text-align` where present. Support named colours (CSS basic 16 + gold,
  orange, purple, pink, white…) and `#rgb`/`#rrggbb`. A new output kind `Styled` carries the style.

## R6. Removing fabricated social features

- **Decision**: Create → Code becomes "My Spells" plus "Example Spells" (the same five sample
  programs with no author, votes or views). Create → Discuss is removed. Train → Feed becomes
  "Log" (the learner's own persisted adventure log and notes). The invite card and the
  "followers" line are removed. Leaderboard bots become named "computer rivals" with a visible label.
  Leagues are renamed Wood, Stone, Bronze, Iron, Silver, Gold, Crystal.

## R7. Testability of time-based rules

- **Decision**: Rule functions take `now_secs: u64` or `day: u64` parameters. The UI calls them with
  the real clock. Tests use fixed values, including clocks that go backwards (`saturating_sub`
  everywhere).
