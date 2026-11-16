# Feature 003: Dungeon UI, deep lessons, full Python and SQL

**Status**: Implemented · **Branch**: `003-dungeon-depth` · **Version**: 1.2.0

## Why
Playtesting 1.1 showed three gaps:
1. A lesson takes only a couple of minutes. Learners want longer, more complete lessons with a full
   explanation in plain words and a short summary to remember.
2. The UI still looks like flat panels. It should look like a dungeon: stone walls, stone and wood
   tiles for cards and buttons.
3. Python and SQL were still the short 1.0 courses.

The user also supplied the GTORAVERSE Achievement Icon Pack for badges.

## User stories
- **US1 – Deep lessons (P1).** As a learner, every lesson gives me: short picture slides, a long
  explanation ("The Full Scroll") written for beginners, a summary of key points, then at least 6
  practice steps. The summary appears again when I finish, and I can re-read any lesson's notes from
  the course preview.
- **US2 – Dungeon look (P1).** Every screen sits on a stone-brick wall; cards are stone frames with
  iron studs; buttons and answer tiles are carved stone blocks; reading pages are parchment;
  progress bars are carved channels. Header and nav bars are stone ledges.
- **US3 – Python and SQL at full depth (P1).** Python follows the beginner path Hello World →
  Variables → Control Flow → Loops → Lists → Functions → Dictionaries → Classes & Objects → Modules.
  SQL covers querying, filtering, sorting, aggregates, changing data, creating tables and joins.
  Every chapter ends with a checkpoint project. All typed answers run without errors.
- **US4 – Tiered badges (P2).** Course badges use the GTORAVERSE icons and grow from bronze (first
  chapter cleared) to silver (half the course) to gold (whole course). Achievements use pack icons.

## Requirements
- FR-1 Every lesson has an explanation of at least 120 words and a 3–6 point summary.
- FR-2 Every lesson has at least 6 questions (concept checks, fill-the-gap, copy, recall).
- FR-3 The learn stage shows slides, then the explanation page, then the summary page.
- FR-4 The completion screen lists the lesson summary.
- FR-5 The interpreters support every construct used in Python and SQL answers (classes, imports of
  `math`/`random`, SQL `DISTINCT`, `AS`, `LIKE`, `IN`, `BETWEEN`, `IS NULL`, `HAVING`, `JOIN`,
  `INSERT`, `UPDATE`, `DELETE`, `CREATE TABLE`).
- FR-6 Old saves keep their progress (lesson ids are stable; renamed lessons are migrated).
- FR-7 Asset licences are respected and credited (Kenney CC0, GTORAVERSE credit, Pixelrepo).

## Success criteria
- SC-1 Average lesson length is at least 3× the 1.1 length (slides + explanation words).
- SC-2 All content tests pass, including the headless playthrough of every lesson.
- SC-3 Screens reviewed by screenshot at 360 px and desktop widths.
