# Pixel Learn

Learn to code in a pixel-art RPG. Pick a quest, read a short scroll, fill in the missing code, then
type it yourself and watch it run. Pixel Learn works fully offline and keeps your progress on your
computer.

## Quick start (players)

1. Unzip `PixelLearn-1.0.0-windows.zip` anywhere.
2. Double-click **PixelLearn.exe**.
3. Choose a name, a hero and your first course, then start learning.

Nothing to install, no account, no internet needed.

## What's inside

- **5 courses, 42 lessons**: HTML, CSS, JavaScript, Python and SQL. Every lesson has three steps:
  *Learn → Complete the code → Type the code*.
- **Real feedback**: Python and JavaScript snippets really run, HTML shows a preview, CSS shows the
  styled result, and SQL queries run against a small sample database (`heroes`, `items`).
- **Spell Forge**: write, run and save your own programs in any of the five languages, or start
  from bundled examples.
- **Practice**: quick challenges and duels against computer opponents (duels never cost hearts).
- **Progress**: XP and levels, gems, a daily streak with streak savers, 7 weekly leagues against
  computer rivals, 12 achievements and an adventure log.
- **Keyboard friendly**: Enter = next/check/continue, 1–5 = pick an answer, Esc = leave a lesson,
  Ctrl+Enter or F5 = run code in the editor.

### Rules at a glance

| | |
|---|---|
| Hearts | 5 max. A wrong answer costs 1. Hearts refill 1 every 5 minutes, or 10 gems for a full refill, or 1 free per day |
| XP | First clear 10 XP (+5 if flawless) and 5 gems. Replays give 5 XP. 100 XP per level |
| Streak | Clear one lesson a day. A streak saver (20 gems) covers one missed day |
| Leagues | Earn 100 XP in a week to move up. Earn none and you drop one |

## Your data

Progress is saved to `%APPDATA%\pixel_learn\save.txt`. Every save is written safely: the previous
save is kept as `save.bak`. Delete the folder to start over, or use *Settings → Reset progress*.

## Building from source

Requirements: Rust 1.85+ (`rustup`). On Windows, the Windows SDK lets the build embed the app icon
(the build still works without it).

```powershell
cargo test            # rules, saves, interpreters and course-content checks
cargo run             # debug build
cargo run --release   # optimized build, no console window
powershell -ExecutionPolicy Bypass -File scripts/package.ps1   # dist/PixelLearn-<ver>-windows.zip
```

QA helpers: `--screenshot out.png [--tab learn|train|ranks|create|hero] [--screen settings|editor|sql|complete|shop|notif|avatar|nohearts] [--onboarding] [--size 360x600]`.
Screenshot mode uses an in-memory demo profile and never touches your save.

Project plan and specs live in `specs/` (Spec Kit). Project principles are in
`.specify/memory/constitution.md`.

> **Note on assets:** the Pixelrepo license forbids redistributing the raw art files. Before
> publishing this source code publicly, remove or replace `assets/art/`.

## Credits

Assets by Pixelrepo (https://pixelrepo.com). Fonts: Press Start 2P and Silkscreen (SIL OFL).
See [CREDITS.md](CREDITS.md).
