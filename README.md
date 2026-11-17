# Code Forge

Learn to code in a pixel-art RPG. Pick a quest, read a short scroll, fill in the missing code, then
type it yourself and watch it run. Code Forge works fully offline and keeps your progress on your
computer.

## Quick start (players)

1. Unzip `CodeForge-1.0.0-windows.zip` anywhere.
2. Double-click **CodeForge.exe**.
3. Choose a name, a hero and your first course, then start learning.

Nothing to install, no account, no internet needed.

## What's inside

- **5 courses, 181 lessons**: HTML (4 chapters), CSS (5), JavaScript (8), Python (9) and SQL (7).
  Every chapter ends with a **checkpoint project** (+10 bonus XP).
- **Deep lessons**: picture slides, then **The Full Scroll** (a long, plain-language explanation with
  examples and common mistakes), then a **summary** of key points, then at least six practice steps:
  concept checks, *complete the code*, *type the code* and *code it from memory*. Re-read any cleared
  lesson's notes from the course preview.
- **Course preview**: see a course's chapters, lessons, projects, estimated time, what you'll learn
  and its badge before you start.
- **Course badges** that grow with you: bronze after the first chapter, silver at half, gold when the
  course is done (Stonemason, Prism Painter, Lightning Mage, Serpent Sage, Vault Keeper).
- **Dungeon look**: stone-brick walls, iron-studded stone cards, carved stone buttons, parchment pages
  and wooden frames for highlights.
- **Real feedback**: Python and JavaScript snippets really run (functions, loops, lists, dictionaries,
  classes, `math` and `random`, objects, template strings), JavaScript DOM lessons show the simulated
  page changing, HTML shows a preview including forms, CSS shows the styled result, and SQL runs
  against a sample database (heroes, items, guilds, quests) with filters, totals, joins, INSERT,
  UPDATE, DELETE and CREATE TABLE.
- **Spell Lab**: write, run and save your own programs in any of the five languages, or start
  from bundled examples.
- **Practice**: a **Daily Quest** (5 mixed questions, +15 XP once a day), **course reviews** from lessons
  you've cleared, 10 **quick challenges** across all five languages, and a **Duel Arena** with four
  computer champions. Daily quests, reviews and duels never cost hearts.
- **Certificates**: finish a course to earn its certificate (name, course, skills, date, number), which
  you can save as an image. Separate from the badge, which grows bronze → silver → gold as you go.
- **Text size**: Normal, Large or Huge in Settings.
- **Fits any screen**: from small phones (320 px wide) to tablets and desktops; on wide screens the content
  sits in a centred column and card grids gain columns.
- **Music and sound**: a grand title theme, a chill lo-fi theme around the hub, a calm study theme while
  you learn (lessons, course pages, notes, the code editor), a fast battle theme for duels, plus distinct
  sounds for every kind of button, right and wrong answers, rewards, level-ups and duel hits. All of it
  is synthesised by the app itself (no audio files). Volumes in Settings; `--export-audio <dir>` writes
  everything as WAV files.
- **Animated welcome**: the logo is forged in front of you: the iron head drops, the handle slides in,
  the runes `< / >` fly into place and a final hammer strike sends sparks flying.
- **Weekly leagues**: 7 leagues. Each week you race five computer rivals tuned to your own pace (each
  with a style: steady, weekend sprinter, early bird, late rush, streaky) and a ghost of your own last
  week. Finish top 3 to move up (with a podium chest), bottom 2 to move down. Monday brings a results
  ceremony.
- **Progress**: XP and levels, gems, a daily streak with streak savers, 12 achievements, course badges,
  certificates and an adventure log.
- **Keyboard friendly**: Enter = next/check/continue, 1–5 = pick an answer, Esc = leave a lesson,
  Ctrl+Enter or F5 = run code in the editor.

### Rules at a glance

| | |
|---|---|
| Hearts | 5 max. A wrong answer costs 1. Hearts refill 1 every 5 minutes, or 10 gems for a full refill, or 1 free per day |
| XP | First clear 10 XP (+5 if flawless) and 5 gems. 100 XP per level. Repeats (replays, extra daily quests, reviews, solved challenges, extra duels) give up to 60 XP a day in total |
| Duels | Win: +20 XP and 5 gems, for the first 3 wins each day. Duels never cost hearts |
| Streak | Clear one lesson a day. A streak saver (20 gems) covers one missed day |
| Leagues | Top 3 of 7 move up (10 / 6 / 3 gems), bottom 2 move down, no XP at all also drops you |
| Days | A new day starts at your local midnight. Turning the clock back does not give a day back |

## Your data

Progress is saved to `%APPDATA%\code_forge\save.txt` (saves from Pixel Learn 1.x in `%APPDATA%\pixel_learn` are copied over automatically). Every save is written safely: the previous
save is kept as `save.bak`. Delete the folder to start over, or use *Settings → Reset progress*.

**Changing device:** *Settings → Export* saves your progress to a `.codeforge` file. On the new device,
choose *Import progress* on the welcome screen or in Settings (or drop the file onto the window). The
file is checked before it is used, and the progress it replaces is kept as `save.before-import.txt`.

**Fair play:** the save carries a checksum. If it is edited by hand, the game notices, resets the gems
and drops anything it could not have produced (unknown lessons, certificates for unfinished courses).
This keeps casual cheating out; it is an offline game, so it is not meant as real security.

## Building from source

Requirements: Rust 1.85+ (`rustup`). On Windows, the Windows SDK lets the build embed the app icon
(the build still works without it).

```powershell
cargo test            # rules, saves, interpreters and course-content checks
cargo run             # debug build
cargo run --release   # optimized build, no console window
powershell -ExecutionPolicy Bypass -File scripts/package.ps1   # dist/CodeForge-<ver>-windows.zip
```

QA helpers: `--screenshot out.png [--tab learn|train|ranks|create|hero] [--tab practice] [--scroll N] [--screen settings|editor|sql|complete|shop|notif|avatar|nohearts|preview|notes|cert] [--lesson C.M.L --stage N] [--onboarding] [--size 360x600]`.
Screenshot mode uses an in-memory demo profile and never touches your save.

Project plan and specs live in `specs/` (Spec Kit). Project principles are in
`.specify/memory/constitution.md`.

> **Note on assets:** the Pixelrepo and GTORAVERSE licenses do not allow redistributing the raw art
> files. Before publishing this source code publicly, remove or replace `assets/art/` and
> `assets/badges/` (the Kenney tiles in `assets/ui/` are CC0 and may stay).

## Credits

Assets by Pixelrepo (https://pixelrepo.com), Kenney (https://www.kenney.nl, CC0) and GTORAVERSE
(Achievement Icon Pack). Fonts: Press Start 2P and Silkscreen (SIL OFL).
See [CREDITS.md](CREDITS.md).
