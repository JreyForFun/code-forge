# Implementation Plan: Pixel Learn 1.0 — Shippable Release

**Branch**: `001-shippable-release` | **Date**: 2026-11-16 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/001-shippable-release/spec.md`

## Summary

Turn the working egui prototype into a 1.0 release. The approach has four parts. (1) Move game
rules and save handling out of the UI into pure, unit-tested modules (`game.rs`, `save.rs`). (2) Add
the missing interpreters (`sql.rs`, `css.rs`) and test every content answer against them. (3) Replace
fabricated social features with honest equivalents: example programs, the learner's own adventure log,
and computer rivals labelled as such. Add onboarding, avatars, persistent achievements and real
notifications using new Pixelrepo art. (4) Package a single Windows executable with an icon and no
console, using a one-command release script.

## Technical Context

**Language/Version**: Rust 1.99 (edition 2024)

**Primary Dependencies**: eframe/egui 0.37 (UI), image 0.25 (PNG decode); build-only: winresource
(Windows exe icon and version info)

**Storage**: Plain-text key=value save file in `%APPDATA%\pixel_learn\save.txt`, written atomically
with a `.bak` backup

**Testing**: `cargo test` (unit tests in modules, plus a content-integrity test across all courses)

**Target Platform**: Windows 10/11 x64 desktop (portable code; macOS/Linux compile but aren't packaged)

**Project Type**: Desktop app (single binary)

**Performance Goals**: Start in under 3 s; 60 fps while scrolling; interpreters finish or stop in
under 50 ms per run

**Constraints**: Fully offline, no network calls; single self-contained exe; window minimum 360×600

**Scale/Scope**: 5 courses / 42 lessons, about 12 screens, 1 local user

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | How this plan complies | Status |
|---|---|---|
| I. Honest, original | Removes fake posts, threads, views, followers and the invite link. Rivals and opponents are labelled "computer rival". Leagues are renamed to an original RPG set. | PASS |
| II. Offline & private | No network code. Atomic save + `.bak`, versioned format, tolerant parser, write-failure warning. | PASS |
| III. Learning loop | Enter-key actions; correct output for CSS/SQL; content test guarantees every expected answer runs clean. | PASS |
| IV. Tested core rules | `game.rs`, `save.rs`, `interp.rs`, `sql.rs`, `css.rs`, `content.rs` get unit tests; rules take `now`/`day` as parameters. | PASS |
| V. Pixel consistency | New screens use `art.rs` widgets and palette only. New art is Pixelrepo PNGs scaled by whole numbers with NEAREST filtering. | PASS |
| VI. Shippable | `build.rs` icon, `windows_subsystem = "windows"`, `scripts/package.ps1` runs tests + release build + zip. | PASS |

Post-design re-check: PASS. The one new dependency (`winresource`, build-only, Windows-only) is
justified under Complexity Tracking.

## Project Structure

### Documentation (this feature)

```text
specs/001-shippable-release/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── save-format.md
│   └── cli.md
└── tasks.md
```

### Source Code (repository root = `app/`)

```text
app/
├── Cargo.toml            # version 1.0.0, release profile, winresource build-dep
├── build.rs              # embed assets/icon.ico + version info on Windows
├── assets/
│   ├── art/*.png         # Pixelrepo Free assets (embedded via include_bytes!)
│   ├── icon.ico / icon.png
│   └── *.ttf             # OFL fonts
├── src/
│   ├── main.rs           # App struct, frame loop, modals, entry point
│   ├── game.rs           # NEW pure rules: hearts, streak, week roll, rewards, score, achievements
│   ├── save.rs           # NEW Save struct, parse/serialize, atomic IO, legacy load
│   ├── data.rs           # course types, challenges, opponents, rivals, avatars, examples
│   ├── content.rs        # 5 courses (+ integrity tests)
│   ├── interp.rs         # Python/JS/HTML runner, dispatch to sql/css (+ tests)
│   ├── sql.rs            # NEW mini SQL engine over sample tables (+ tests)
│   ├── css.rs            # NEW CSS rule parser/preview (+ tests)
│   ├── art.rs            # palette, fonts, sprites, widgets, textures
│   ├── lesson.rs         # lesson run screen
│   ├── onboarding.rs     # NEW first-run welcome flow
│   ├── tabs.rs           # Learn / Train / Ranks / Create / Hero tabs
│   └── screens.rs        # complete, settings, editor, example viewer
├── scripts/package.ps1   # NEW test + release build + zip
├── README.md             # NEW quick start, features, build, credits
└── CREDITS.md            # NEW asset and font licences
```

**Structure Decision**: Keep a single binary crate. Tests live next to their modules
(`#[cfg(test)] mod tests`) because everything is internal to one binary. There is no library split.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| New build-dependency `winresource` | Windows needs the icon embedded in the .exe resource section (Explorer/taskbar icon), required by FR-025 | Setting only the runtime window icon leaves the generic exe icon in Explorer and the zip |
