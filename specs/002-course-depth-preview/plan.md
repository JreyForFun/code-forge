# Implementation Plan: Deeper Courses, Course Preview, Badges & Consistency

**Branch**: `002-course-depth-preview` | **Spec**: [spec.md](spec.md)

## Summary
Fix the frame-ordering crash (done, with UI playthrough tests). Restructure content into
`src/content/{mod,html,css,js,python,sql}.rs`, with stable lesson ids and a checkpoint project
per chapter. Extend the JS interpreter (objects, template literals, break, simulated DOM), the HTML
preview (forms, semantics) and the CSS preview (border, padding, underline). Add a `CoursePreview`
screen, course badges, a code-drawn quest-card header, and shared spacing tokens.

## Technical Context
Rust 2024, eframe/egui 0.37 (glow), no new dependencies. Save format v3: lesson keys become
`course/slug`, with a migration table from v2 `c.m.l` keys. Tests: content integrity, UI
playthrough, interpreter, migration.

## Constitution Check
| Principle | Status |
|---|---|
| I Honest/original | PASS: topics informed by Codédex's public outline, all text original |
| II Offline/private | PASS: no network |
| III Learning loop | PASS: deeper courses, every answer runs clean (test) |
| IV Tested rules | PASS: migration, interpreter, UI playthrough tests |
| V Pixel consistency | PASS: spacing tokens, shared section title, badge frame drawn with palette |
| VI Shippable | PASS: package script unchanged |

## Structure
```text
src/content/mod.rs     helpers, all(), lesson ids, migration table, tests
src/content/html.rs    4 chapters, checkpoint each
src/content/css.rs     5 chapters
src/content/js.rs      8 chapters
src/content/python.rs  (moved, unchanged)
src/content/sql.rs     (moved, unchanged)
src/preview.rs         Course Preview screen + quest-card header + badge drawing
```

## Key decisions
- **Lesson id** = `{course.slug}/{slugify(title)}`. Titles are unique per course (tested).
- **Migration**: `OLD_KEYS` maps the 42 v2 keys to new ids. Unmapped keys are dropped. Partial
  progress (`qdone`) for migrated keys is dropped because question lists changed.
- **Badges** are stored in `ach` as `badge:<slug>` → day, so the existing persistence is reused.
