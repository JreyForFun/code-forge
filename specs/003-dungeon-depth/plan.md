# Plan: 003 Dungeon UI, deep lessons, full Python and SQL

## Approach
1. **Assets.** Download Kenney CC0 packs (Tiny Dungeon, UI Pack: Pixel Adventure) and cut the tiles
   the skin needs into `assets/ui/` (frames hollowed so the app fills the centre; the stone block is
   grayscale so it can be tinted to any colour). Copy the GTORAVERSE 32x32 badge icons the app uses
   into `assets/badges/`. Everything is embedded with `include_bytes!`.
2. **Skin (`art.rs`).** 9-slice renderer (`nine_shape`) and screen-anchored tiling (`tile_shape`,
   repeat-wrapped textures). Re-implement the shared widgets on top of it so every screen changes at
   once: `bevel` = carved stone block, `panel`/`panel_resp` = iron-studded stone frame (wood for gold
   highlights), `parchment`, `ribbon`, `ledge` for header/footer bars, `floor` = brick wall.
3. **Deep lessons.** `Lesson` gains `explain` and `summary`. Web courses attach them from per-course
   tables (`*_deep.rs`, matched by title; a test fails on unknown titles); Python and SQL are written
   inline with `Lesson::with`. The run screen adds two learn pages (The Full Scroll, Summary); the
   completion screen and a new Notes screen (from the course preview) show them again.
4. **Interpreters.** Python: slices, `items()`/`enumerate` with unpacking, `del`, more list/str/dict
   methods, `round(x, n)`, default parameters, `import math/random`, classes with `__init__`, methods
   and attribute defaults. SQL: rewritten engine with a per-run database, expressions, DISTINCT, AS,
   LIKE, IN, BETWEEN, IS NULL, HAVING, INNER/LEFT JOIN, INSERT, UPDATE, DELETE, CREATE/ALTER/DROP.
5. **Badges.** Tier from progress (bronze: chapter 1, silver: 50%, gold: done) drawn from the pack.

## Risks
- Long text in a pixel font is hard to read → explanations use egui's built-in Ubuntu font.
- Old saves: Python/SQL titles from 1.0 are kept, so stable ids and the 1.0 migration still work.
