# Tasks: Pixel Learn 1.0 — Shippable Release

**Input**: Design documents from `specs/001-shippable-release/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Required by Constitution IV. Test tasks come before or alongside their implementation.

Format: `[ID] [P?] [Story] Description`. [P] = can run in parallel (different files).

## Phase 1: Setup

- [x] T001 Add Pixelrepo assets to `assets/art/` (heart, crown, banners, hourglass, chalice, cup, lantern, campfire, medallion, letter, money_bag, coins, hat_wizard, helmet_knight, helmet_bronze, bow, axe, hammer, spellbook, console, cpu, keyhole, gear, mystic_figure, scroll_lightning, potion_magic, flame)
- [x] T002 [P] Generate `assets/icon.png` + multi-size `assets/icon.ico` from pixel art
- [x] T003 Set `Cargo.toml` version 1.0.0, metadata, release profile (lto, strip, opt-level 3), `winresource` build-dep; add `build.rs`

## Phase 2: Foundational (blocks all stories)

- [x] T004 [P] Write tests, then create `src/game.rs`: level, tick_hearts, lose_heart, bump_streak, week_roll, lesson_reward, score (clock-backwards safe)
- [x] T005 [P] Write tests, then create `src/save.rs`: Save struct, `parse`/`serialize`, v2 keys, legacy v1 load, atomic write + `.bak`, write-error reporting
- [x] T006 Rewire `main.rs` to use `game.rs`/`save.rs`; screenshot mode never writes the save
- [x] T007 Texture registry: load all new art in `art.rs` (generic loader over a name→bytes table)

## Phase 3: US1 First launch to first lesson (P1) 🎯 MVP

- [x] T008 [US1] `src/onboarding.rs`: 4-step welcome (name, avatar, course, goal) shown when `!onboarded`
- [x] T009 [US1] Avatars table in `data.rs`; avatar drawn in header/profile/leaderboard
- [x] T010 [US1] Enter key: NEXT / CHECK (single-line answers) / CONTINUE in `lesson.rs`
- [x] T011 [US1] Out-of-hearts flow keeps partial progress; dialog shows minutes until next heart

## Phase 4: US2 Correct feedback for every language (P1)

- [x] T012 [P] [US2] Tests + `src/sql.rs` mini engine with `heroes`/`items` sample tables; `Table` output kind
- [x] T013 [P] [US2] Tests + `src/css.rs` rule parser + `Styled` output kind
- [x] T014 [US2] `interp::run` dispatch for css/sql; `out_view` renders tables and styled lines
- [x] T015 [US2] Content-integrity test `content_answers_run_clean` over all courses (+ index/gap checks)
- [x] T016 [US2] Interpreter hardening tests: endless loop, 10k-char input, deep nesting

## Phase 5: US3 Progress never lost (P1)

- [x] T017 [US3] Persist adventure log + achievements via save.rs; cap log at 100
- [x] T018 [US3] One-time "can't save" warning toast

## Phase 6: US4 Practice, rivals, honest guild (P2)

- [x] T019 [US4] Remove posts/threads/invite/followers/Discuss; add `examples()` in `data.rs`
- [x] T020 [US4] Create tab: My Spells + Example Spells; editor supports css/sql, rename, delete confirm
- [x] T021 [US4] Ranks: renamed leagues with banner/crown art, rivals labelled "computer rival"
- [x] T022 [US4] Train: Practice (challenges + duels labelled computer opponents) and Log (own notes)

## Phase 7: US5 Profile, avatar, achievements (P2)

- [x] T023 [US5] Achievement definitions (≥10, unique icons) + `game::new_achievements` with tests
- [x] T024 [US5] Profile achievement grid with earned dates; avatar picker; toast on unlock
- [x] T025 [US5] Notifications modal with real data (heart timer, streak today, latest achievement/league)

## Phase 8: US6 Download and run (P1)

- [x] T026 [US6] `windows_subsystem` in release, runtime window icon, `--version` flag, version in Settings credits
- [x] T027 [US6] `scripts/package.ps1`: test → release build → `dist/PixelLearn-<ver>-windows.zip`
- [x] T028 [P] [US6] `README.md` (quick start, features, build, licence notes) + `CREDITS.md`

## Phase 9: Polish

- [x] T029 Screenshot QA of every tab/screen at 440×780 and 360×600; fix clipping
- [x] T030 Clear compiler warnings; `cargo clippy` clean for new modules
- [x] T031 Update `.gitignore` (dist/), final commit, run quickstart validation

## Dependencies

Setup → Foundational (T004–T007) → US1, US2, US3 (any order) → US4, US5 → US6 → Polish.
T012/T013 are independent of UI work. T015 depends on T012–T014.

## Implementation notes (2026-11-16)

- All 31 tasks complete. `cargo test`: 45 tests passing (rules, saves, interpreters, SQL, CSS, content integrity, art, end-to-end first launch → persisted lesson).
- The content test (T015) found real defects that were then fixed: the CSS/SQL courses had no language set (the old "bogus error" bug), and the Python and JavaScript courses taught functions and lists, but the interpreter had no support for them. SQL lessons taught `AND`, and the engine had no `AND`. 42 failing answers became 0.
- R8: switched the renderer from wgpu to glow after reproducible `egui-wgpu` staging-buffer panics. Release exe 15.3 MB → 8.2 MB, zip 3.3 MB.
- Added a panic hook that writes `%APPDATA%\pixel_learn\crash.log` (release builds have no console).
- Clippy: clean.
