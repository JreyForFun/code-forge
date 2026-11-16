# Tasks: 002 Deeper Courses, Preview, Badges, Consistency

## Phase 1: Crash (US1)
- [x] T001 Defer slide stage changes to end of frame (`lesson.rs`)
- [x] T002 Headless UI tests: play every lesson, wrong answer path, no-hearts path

## Phase 2: Foundations
- [x] T003 Split content into `src/content/` modules; stable lesson ids; uniqueness test
- [x] T004 Save v3 migration from `c.m.l` keys (+ test)
- [x] T005 Spacing tokens + `section_title` helper in `art.rs`

## Phase 3: Runners (US3)
- [x] T006 JS: Flow (break), objects, multi-line object literals, template literals, typeof, Math.random
- [x] T007 JS: simulated DOM (getElementById, querySelector, textContent, innerHTML, style, addEventListener, alert)
- [x] T008 HTML preview: textarea, select, checkbox/radio, label, semantic containers; CSS preview border/padding/underline/italic
- [x] T009 Interpreter tests for all of the above

## Phase 4: Content (US3)
- [x] T010 HTML: Elements, Structure, Forms, Semantic HTML (+checkpoints)
- [x] T011 CSS: Selectors, Properties, Box Model, Layout, Flexbox (+checkpoints)
- [x] T012 JS: Console, Variables, Conditionals, Loops, Arrays, Functions, Objects, The Page (+checkpoints)
- [x] T013 Project lessons: CHECKPOINT tag in UI, +10 XP bonus

## Phase 5: Preview, badges, header (US2, US4, US5)
- [x] T014 Course metadata: slug, teaches, hours, badge, snippet, chapter blurbs
- [x] T015 `Screen::CoursePreview`; route all entry points; Back returns to origin
- [x] T016 Badge drawing, grant on course completion, profile badge shelf, preview badge panel
- [x] T017 Quest-card header replacing the landscape banner

## Phase 6: Polish (US6)
- [x] T018 Apply spacing tokens across tabs and screens; screenshot review
- [x] T019 Tests green, clippy clean, package, commit

## Implementation notes (2026-11-16)
- Crash root cause: frame-ordering bug (slide index advanced mid-frame). Fixed and covered by a headless UI test that plays all 135 lessons.
- Lessons: HTML 8→27, CSS 8→31, JS 8→48 (24→106 web lessons). Every typed answer runs clean (test).
- The JS runner gained objects, templates, break/continue, arrow and expression functions, string/array methods and a simulated DOM. The HTML preview renders forms. The CSS preview shows borders, padding, underline and italics.
- Save keys migrated from positions to stable `course/slug` ids (old progress kept where the topic still exists).
- Badges: code-drawn medal + Pixelrepo icon. itch.io assets were not bundled because the download flow needs a browser session.
- Tests: 60 passing. Clippy: clean.
