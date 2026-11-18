# Contributing to Code Forge

Thanks for wanting to help! Code Forge is a small project, so every bug report, lesson fix and idea
really matters. You don't need to be an expert: spotting a typo in a lesson is a real contribution.

By taking part you agree to follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## Ways to help

| You want to... | Do this |
| --- | --- |
| Report something broken | Open a [bug report](https://github.com/JreyForFun/code-forge/issues/new?template=bug_report.yml) |
| Fix a mistake in a lesson | Open a [lesson issue](https://github.com/JreyForFun/code-forge/issues/new?template=lesson_content.yml), or send a pull request |
| Suggest a feature or a course | Open a [feature request](https://github.com/JreyForFun/code-forge/issues/new?template=feature_request.yml) |
| Report something hard to use | Open an [accessibility report](https://github.com/JreyForFun/code-forge/issues/new?template=accessibility.yml) |
| Report a security problem | **Don't open an issue.** Follow [SECURITY.md](SECURITY.md) |

Before opening an issue, please search the existing ones; someone may already have reported it.

## Setting up

You need [Rust](https://rustup.rs) 1.85 or newer. On Windows, the Windows SDK lets the build embed
the app icon (the build still works without it).

```powershell
git clone https://github.com/JreyForFun/code-forge.git
cd code-forge
cargo run              # debug build
cargo run --release    # fast build, no console window
```

Your own progress is never touched by screenshot mode, so you can test freely with:

```powershell
cargo run --release -- --screenshot out.png --tab practice --size 390x780
```

See the README for every QA flag.

## Before you open a pull request

```powershell
cargo test                       # rules, saves, interpreters and course-content checks
cargo clippy -- -D warnings      # must be clean
```

The code uses long, dense lines on purpose; please match the style of the file you are editing
rather than running `cargo fmt` over it.

- **Keep it focused.** One fix or feature per pull request is much easier to review.
- **Add a test** when you fix a bug in the rules, the save file, the Python/JavaScript interpreter or
  the SQL engine.
- **Show it.** For anything visible, attach a screenshot at phone size (`--size 390x780`) and at
  desktop size (`--size 1280x800`).
- **Don't add raw art files.** The game's pixel art comes from packs whose licences don't allow
  sharing the files on their own (see [CREDITS.md](CREDITS.md)). New art must be your own work or
  CC0, and you must say where it came from.

## Where things live

| Path | What's in it |
| --- | --- |
| `src/content/` | The courses: lesson slides, the long explanations, summaries and exercises |
| `src/lesson.rs` | How a lesson plays (slides, questions, fill-in, type-it, from memory) |
| `src/interp.rs` | The small Python and JavaScript interpreter that runs player code |
| `src/sql.rs` | The SQL engine and sample database |
| `src/css.rs` | HTML and CSS preview |
| `src/game.rs` | XP, hearts, streaks, leagues and rivals |
| `src/save.rs` | The save file, checksum, export and import |
| `src/tabs.rs`, `src/screens.rs` | The main screens |
| `src/art.rs` | The dungeon look: frames, tiles, fonts, colours |
| `src/audio.rs` | Music and sound, all synthesised in code |
| `src/platform.rs` | Windows / Android differences |
| `scripts/` | Packaging: `package.ps1` (Windows zip), `build_android.py` (APK) |
| `specs/` | Feature specs and plans |

## Writing lesson content

Lessons are written for complete beginners:

- Use short sentences and everyday words. Explain a term the first time it appears.
- Every new idea gets an example the player can read before they have to write it.
- Exercises must have exactly one sensible answer, or accept all the reasonable ones.
- Keep the dungeon flavour light: it should make lessons fun, never harder to understand.
- Run `cargo test`; it checks that every lesson is complete and every exercise can be solved.

## Commit messages

Use the [Conventional Commits](https://www.conventionalcommits.org) style the history already uses:

```text
feat: add a Python course chapter on files
fix(android): keyboard covers the CHECK button
docs: explain how leagues work
```

## Licence

By contributing, you agree that your contribution is released under the project's
[MIT licence](LICENSE).
