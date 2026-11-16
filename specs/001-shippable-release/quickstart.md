# Quickstart & Validation: Pixel Learn 1.0

## Prerequisites
- Windows 10/11, Rust 1.85+ (`rustup`), Windows SDK (for the exe icon; build still works without it).

## Build, test, run
```powershell
cd app
cargo test                 # all rule, save, interpreter and content tests
cargo run                  # debug build with console
cargo run --release        # release build, no console window
```

## Package
```powershell
powershell -ExecutionPolicy Bypass -File scripts/package.ps1
# -> dist/PixelLearn-1.0.0-windows.zip  (PixelLearn.exe, README.md, CREDITS.md)
```

## Validation scenarios
1. **First run (US1)**: rename `%APPDATA%\pixel_learn` → launch → welcome flow → finish lesson 1 → restart → no welcome, lesson cleared.
2. **Languages (US2)**: `cargo test content_answers_run_clean` passes; manually type `SELECT name FROM heroes;` in a SQL lesson → table output.
3. **Persistence (US3)**: `cargo test save::` passes; add garbage lines to `save.txt` → app starts; `save.bak` exists after any save.
4. **Honesty (US4)**: Visit every tab: no views/votes/followers/invite; rivals labelled "computer rival".
5. **Profile (US5)**: Clear a lesson → toast "Achievement: First Steps" → Hero tab shows it with date.
6. **Ship (US6)**: Extract the zip to an empty folder and double-click `PixelLearn.exe` → window with icon, no console.

## Screenshot QA
```powershell
target\debug\pixel_learn.exe --screenshot out.png --tab hero
```
