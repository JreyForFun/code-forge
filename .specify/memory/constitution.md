<!--
Sync Impact Report
- Version change: template → 1.0.0 (initial ratification)
- Principles added: I. Honest, Original Product; II. Offline-First & Private; III. Learning Loop First;
  IV. Tested Core Rules; V. Pixel-Perfect Consistency; VI. Shippable at Every Commit
- Sections added: Technology & Asset Constraints; Development Workflow & Quality Gates; Governance
- Removed sections: none
- Deferred TODOs: none
-->

# Pixel Learn Constitution

## Core Principles

### I. Honest, Original Product

Pixel Learn is an original, RPG-themed app for learning to code. Every name, lesson text, character
and screen layout MUST be our own work or a properly licensed asset.

- The app MUST NOT use another product's name, logo, branding, copied lesson text or copied art.
- The app MUST NOT show fabricated social proof: no fake view counts, votes, followers, other users'
  posts, or invite links that go nowhere. Computer-controlled characters (rivals, duel opponents)
  are allowed only when the app presents them clearly as in-game characters.
- Features that would need a server (real accounts, real community) are out of scope until a
  backend exists. They MUST NOT be faked to look real.

Rationale: a learning app earns trust by being honest about what it is.

### II. Offline-First & Private

- Every feature MUST work with no network connection. The app makes no network requests.
- Learner data stays on the device, in the platform's per-user app-data folder. No telemetry, no
  analytics, no accounts.
- Saved progress MUST survive crashes. Writes are atomic (write to a temp file, then rename). A
  corrupt or older-version save MUST load with safe defaults instead of crashing or wiping data.

### III. Learning Loop First

The core loop is **Learn → Complete the code → Type the code**, with rewards (XP, streak, hearts,
gems) that encourage daily practice.

- Every lesson MUST be finishable with keyboard and mouse alone, and each step MUST tell the
  learner clearly whether they were right or wrong and why.
- Each lesson MUST teach something correct. Code a learner types for a language the app can run
  MUST produce output that matches what a real runtime would print for those beginner examples.
- Rewards and penalties MUST never block learning permanently: hearts refill over time, and there
  is always a free path forward.

### IV. Tested Core Rules (NON-NEGOTIABLE)

- Game rules (XP, levels, hearts, streaks, leagues, scoring, unlocking) MUST live in plain,
  UI-free functions that take the current time as a parameter, and MUST have unit tests.
- The interpreters, save/load round-trip and course-content integrity (valid answer indexes,
  gaps present, no empty lessons) MUST have automated tests.
- `cargo test` MUST pass before any commit. Every bug fix comes with a test that would have
  caught it.

### V. Pixel-Perfect Consistency

- One visual language: the palette constants and widgets in `art.rs`, the pixel fonts, sharp
  nearest-neighbour scaling and square corners. New screens reuse these and add no ad-hoc styles.
- The UI MUST stay usable from 360×600 up to a maximized desktop window.
- Every interactive element has a visible hover or pressed state. Text stays readable against
  its background.

### VI. Shippable at Every Commit

- `cargo build --release` produces one self-contained executable with fonts and art embedded.
  It MUST start without extra files.
- A release is created with one command (`scripts/package.ps1`), which builds, tests and zips
  the app with its README and credits.
- No panics in normal use. Unexpected input (bad saves, odd code typed into the editor, endless
  loops) MUST be handled gracefully.

## Technology & Asset Constraints

- Stack: Rust (edition 2024) with eframe/egui for the UI and the `image` crate for PNG decoding.
  Adding a dependency requires a stated reason in the plan.
- Primary target: Windows 10/11 desktop. The code MUST stay portable (no Windows-only APIs
  outside small `cfg` blocks) so macOS and Linux builds remain possible.
- Art: Pixelrepo free assets (Pixelrepo Free Asset License v1.0) plus sprites drawn in code.
  Pixelrepo credit (`Assets by Pixelrepo (https://pixelrepo.com)`) MUST appear in Settings →
  Credits and in the README. Raw asset files MUST NOT be redistributed on their own. They ship only
  embedded in the executable, and a public source release must replace or exclude them.
- Fonts: Press Start 2P and Silkscreen (SIL OFL), credited in the app.

## Development Workflow & Quality Gates

- Work follows Spec Kit: constitution → spec → plan → tasks → implement. Each feature lives in
  `specs/NNN-name/`.
- Before each commit: `cargo build` with no new warnings in changed code, `cargo test` green, and
  a screenshot check of any screen that changed (`--screenshot <png> [--tab <name>]`).
- Commit messages describe the user-visible change. Keep commits small and focused.

## Governance

This constitution overrides other practice documents in this repository. Amendments must be made
by editing this file and must include a Sync Impact Report and a semantic version bump: MAJOR for
removing or redefining a principle, MINOR for adding one, PATCH for wording changes. Every plan
includes a Constitution Check that names any deviation and justifies it in its Complexity Tracking
table.

**Version**: 1.0.0 | **Ratified**: 2026-11-16 | **Last Amended**: 2026-11-16
