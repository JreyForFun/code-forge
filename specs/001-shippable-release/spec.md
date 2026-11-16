# Feature Specification: Pixel Learn 1.0 — Shippable Release

**Feature Branch**: `001-shippable-release`

**Created**: 2026-11-16

**Status**: Ready for planning

**Input**: User description: "The project is done but vague, with no proper plan. Use Pixelrepo and other
assets. Fully implement and create a shippable app."

## Overview

Pixel Learn is an offline desktop app for beginners learning to code, styled as a pixel-art RPG. A
learner (the "hero") picks a course (HTML, CSS, JavaScript, Python, SQL) and clears short lessons. Each
lesson has three steps: read, fill in the missing code, then type the code. Lessons earn XP, gems and
a daily streak, while hearts limit careless guessing. This release turns the working prototype into a
1.0 that a stranger can download, run and use without help. It removes anything that pretends to be
online, fixes wrong behaviour, and adds first-run onboarding, persistence of everything the learner
does, real feedback for every course language, and a one-command release package.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - First launch to first lesson cleared (Priority: P1)

A new learner opens the app for the first time. A short welcome asks for a hero name, lets them pick
an avatar, choose a first course and set a daily streak goal. They land on that course's map, start
lesson 1, go through all three steps and see a "Quest Complete" screen with their rewards.

**Why this priority**: This is the whole product in one path. If it works, the app delivers value.

**Independent Test**: Delete the save file, start the app, complete onboarding and lesson 1 of any
course. Check that XP, gems and the streak went up and that the lesson shows as cleared after a
restart.

**Acceptance Scenarios**:

1. **Given** no saved progress, **When** the app starts, **Then** the welcome flow appears before any
   other screen.
2. **Given** the welcome flow, **When** the learner leaves the name empty, **Then** "Hero" is used
   and they can still continue.
3. **Given** onboarding is finished, **When** the app restarts, **Then** the welcome flow does not
   appear again and the chosen name, avatar, course and goal are kept.
4. **Given** a lesson in the "type the code" step, **When** the learner presses Enter in a
   single-line answer or clicks CHECK, **Then** the answer is checked and feedback shows the correct
   answer if it was wrong.
5. **Given** the learner answers wrongly, **When** feedback appears, **Then** one heart is lost and
   that question comes back later in the same lesson.

---

### User Story 2 - Correct feedback for every course language (Priority: P1)

When a learner types correct code in any course, the app shows a meaningful result: Python and
JavaScript print what a real runtime would print, HTML shows a preview, CSS shows the rule applied to
sample text, and SQL shows the result table from a small built-in sample database.

**Why this priority**: Currently, correct CSS and SQL answers show a bogus "error", which teaches the
wrong thing and breaks trust (Constitution III).

**Independent Test**: Run the type-in steps of each course with the expected answer and check that
the output panel shows a sensible result with no error.

**Acceptance Scenarios**:

1. **Given** a SQL lesson, **When** the learner types `SELECT name FROM heroes;`, **Then** a table
   of hero names from the sample data appears.
2. **Given** a CSS lesson, **When** the learner types `p { color: red; }`, **Then** the app shows
   the selector, each property and value, and a sample line styled with that colour.
3. **Given** the code editor, **When** the learner writes an endless loop, **Then** execution stops
   with a clear "too many steps" message and the app stays responsive.

---

### User Story 3 - Progress that is never lost (Priority: P1)

Everything the learner earns or creates is saved immediately and survives closing the app, crashes
and app updates: progress, settings, saved code, the adventure log and achievements.

**Why this priority**: A learning app that loses streaks loses learners (Constitution II).

**Independent Test**: Earn progress, kill the process mid-session, restart and verify state. Corrupt
the save file by hand, restart, and verify the app starts with whatever could be read, and that a
backup of the previous good save exists.

**Acceptance Scenarios**:

1. **Given** a save written by the prototype (no version line), **When** 1.0 starts, **Then** all
   progress loads.
2. **Given** a save with garbage lines, **When** the app starts, **Then** it ignores unknown lines
   and does not crash.
3. **Given** the learner posts to their adventure log, **When** the app restarts, **Then** the
   entry is still there.

---

### User Story 4 - Practice, rivals and the honest "guild" (Priority: P2)

The learner can practise outside lessons: quick challenges, duels against named computer
opponents, and a weekly league against named computer rivals. The app always presents these
clearly as in-game characters. Nothing claims to be another real person.

**Why this priority**: These features keep learners coming back, but only the core loop is essential.

**Independent Test**: Win a duel, earn XP and confirm the league table ranks the learner among rivals
labelled as computer characters. Confirm there are no view counts, follower counts, other-user posts
or invite links anywhere.

**Acceptance Scenarios**:

1. **Given** the Ranks tab, **When** the learner has XP this week, **Then** a table of rival
   characters and the learner appears, with a note that the rivals are computer-controlled.
2. **Given** a new week starts, **When** last week's XP was at least the promotion threshold,
   **Then** the learner moves up one league and is told so.
3. **Given** the Create tab, **When** it opens, **Then** it shows "My Spells" (the learner's saved
   code) and "Example Spells" (bundled sample programs), with no fake authors, votes or views.

---

### User Story 5 - Hero profile, avatar and achievements (Priority: P2)

The learner's profile shows their avatar, level, streak calendar, course progress and a grid of
achievements. Each achievement has its own pixel icon and shows when it was earned.

**Why this priority**: Visible progress drives motivation, and the profile is where it's collected.

**Independent Test**: Unlock "First Steps" by clearing a lesson, then check that the profile shows it
earned with a date and that it is still there after a restart.

**Acceptance Scenarios**:

1. **Given** the profile, **When** the learner taps their avatar, **Then** they can choose from at
   least 6 avatars and the choice is saved.
2. **Given** an achievement's condition is met, **When** the learner finishes the action, **Then** a
   toast announces it and it shows as unlocked from then on, even if the condition later stops being
   true (for example, gems are spent).

---

### User Story 6 - Download and run (Priority: P1)

A person gets a single zip, extracts it and double-clicks the executable. The app opens in its own
window with an icon and no console window, and works with no installation, internet or extra files.

**Why this priority**: "Shippable" means exactly this.

**Independent Test**: Run the packaging script on a clean checkout, extract the zip into an empty
folder on the same machine and launch the app.

**Acceptance Scenarios**:

1. **Given** the release zip, **When** extracted and launched, **Then** the app starts in under
   3 seconds with no console window.
2. **Given** the release zip, **When** inspected, **Then** it contains the executable, a README with
   a quick-start and the credits/licences, and nothing else is needed.

---

### Edge Cases

- The system clock jumps backwards: heart regeneration and streaks must not underflow, go negative
  or grant rewards.
- The learner misses one day while holding a streak saver: the streak continues and one saver is
  used. Missing two or more days resets the streak to 1 on the next lesson.
- The learner runs out of hearts mid-lesson: the lesson closes, the "Out of hearts" dialog explains
  the refill timer and the gem refill, and partial progress in the lesson is kept.
- The window is resized to the minimum (360×600): every screen stays usable with no clipped
  buttons.
- The learner types very long code (over 10,000 characters) or deeply nested loops in the editor:
  the interpreter stops safely.
- The save folder is not writable: the app keeps running and shows a one-time warning that progress
  can't be saved.

## Requirements *(mandatory)*

### Functional Requirements

**Onboarding & profile**

- **FR-001**: The app MUST show a welcome flow on first launch (no save present) that collects a hero
  name (max 18 characters, defaults to "Hero"), an avatar, a first course and a daily streak goal
  (3, 7, 14 or 30 days).
- **FR-002**: The learner MUST be able to change name, avatar and goal later from the profile or
  settings.

**Learning loop**

- **FR-003**: Each lesson MUST run its slides, then its "complete the code" questions, then its
  "type the code" questions, and requeue wrongly answered questions.
- **FR-004**: Wrong answers in lessons and practice challenges MUST cost one heart. Duels MUST NOT
  cost hearts.
- **FR-005**: Hearts MUST regenerate one every 5 minutes up to 5. Learners can buy a full refill
  for 10 gems or claim one free heart per calendar day.
- **FR-006**: Lessons inside a chapter MUST unlock in order, and chapters MUST unlock when the
  previous chapter is cleared.
- **FR-007**: Pressing Enter MUST trigger the main action of the current lesson step (Next, Check
  or Continue) when it is safe to do so (not while typing a multi-line answer).
- **FR-008**: Rewards: first clear 10 XP, +5 for a flawless clear, 5 gems; replays 5 XP. A level
  is reached every 100 XP.

**Code running**

- **FR-009**: Python and JavaScript snippets MUST run in the built-in interpreter, covering
  variables, arithmetic, strings, comparisons, if/else, while/for loops and print/console.log.
- **FR-010**: HTML MUST render as a simplified preview of headings, paragraphs, list items and
  buttons.
- **FR-011**: CSS MUST be parsed into rules. The output shows each selector with its property/value
  pairs, plus a sample line styled with the colour, size and alignment where possible.
- **FR-012**: SQL MUST run against a built-in sample database (tables `heroes` and `items`)
  supporting `SELECT` columns or `*`, `COUNT/SUM/MIN/MAX/AVG`, `WHERE` with one comparison,
  `GROUP BY` one column, `ORDER BY` one column `ASC|DESC`, and `LIMIT`. Results show as a table.
- **FR-013**: All interpreters MUST stop after a bounded number of steps and report errors in plain
  language without crashing.

**Practice & rivals**

- **FR-014**: The Train tab MUST offer quick challenges and duels against named computer
  opponents, and say clearly that they are computer opponents.
- **FR-015**: The Ranks tab MUST show 7 leagues, a weekly table of the learner against computer
  rivals (marked as such), and the promotion/relegation rule. Weekly results MUST apply when a new
  week starts.

**Create**

- **FR-016**: The Create tab MUST let the learner write, run, save, rename and delete their own
  programs ("My Spells") in Python, JavaScript, HTML, CSS or SQL, and open bundled example programs
  as editable copies.
- **FR-017**: The app MUST NOT show fabricated other users, view counts, votes, follower counts,
  discussion threads from fake users, or invite links.

**Profile, achievements, log**

- **FR-018**: The profile MUST show avatar, name, level and XP bar, streak with weekly calendar,
  course progress, and an achievement grid of at least 10 achievements, each with a distinct pixel
  icon.
- **FR-019**: Achievements MUST be permanent once earned and record the day they were earned.
- **FR-020**: The adventure log MUST record the learner's milestones (lessons cleared, duels,
  achievements, their own notes), keep the newest 100 entries and persist them.
- **FR-021**: Notifications (bell) MUST show real, current information: hearts refill time, streak
  status for today, and the latest achievement or league result.

**Persistence & settings**

- **FR-022**: All learner data MUST persist in the per-user app-data folder, written atomically,
  with a backup of the previous save. The save MUST carry a format version and load older
  prototype saves.
- **FR-023**: Settings MUST include scanlines, background texture, a reset-progress action with
  confirmation, and credits listing Pixelrepo, the fonts and the app version.
- **FR-024**: If saving fails, the app MUST keep running and warn once.

**Shipping**

- **FR-025**: The release build MUST be a single executable with embedded art and fonts, a window
  icon, no console window on Windows, and a version shown in Settings.
- **FR-026**: One script MUST run tests, build the release and produce `PixelLearn-<version>-windows.zip`
  containing the executable, README and credits.

### Key Entities

- **Hero profile**: name, avatar, chosen course, streak goal, onboarding done flag.
- **Progress**: XP, gems, hearts and refill clock, streak (current, longest, last active day, savers),
  weekly XP and active-day mask, league, cleared lessons, partial lesson progress, best scores.
- **Course → Chapter → Lesson → Step/Question**: static content bundled with the app.
- **Achievement**: id, title, description, icon, earned day (if earned).
- **Spell (saved code)**: title, language, source text.
- **Log entry**: day, text.
- **Rival / Opponent**: in-game computer character with name, icon and skill/XP curve.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A first-time user goes from launching the app to clearing lesson 1 in under 4 minutes
  with no outside help.
- **SC-002**: 100% of type-in answers across all 5 courses produce output with no error when the
  expected answer is typed (checked by an automated test over all content).
- **SC-003**: Killing the app at any moment loses at most the current unanswered question, never
  cleared lessons or rewards (checked by restart tests).
- **SC-004**: The app starts in under 3 seconds on a typical laptop and stays smooth (no visible
  stutter) while scrolling every tab.
- **SC-005**: Zero on-screen elements show fabricated social data (verified by a review of every
  screen).
- **SC-006**: The release zip runs on a Windows 10/11 machine with no installs or network.

## Assumptions

- Audience: beginners aged about 12 and up, reading English. Localization is out of scope for 1.0.
- Platform: Windows desktop is the shipping target. macOS and Linux builds should compile but are not
  packaged in 1.0.
- No backend: real accounts, cloud sync and a real community are out of scope. The old "Feed" and
  "Discuss" features become the learner's own adventure log, and community posts become bundled
  example programs.
- Economy values (XP, gems, costs, regeneration time, league threshold of 100 weekly XP) carry
  over from the prototype as reasonable defaults.
- Art comes from the Pixelrepo Free collection (licence allows use in apps with credit) plus
  sprites drawn in code. Raw files are not redistributed outside the executable.
- No audio in 1.0.
