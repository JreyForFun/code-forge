# Feature Specification: Deeper Courses, Course Preview, Badges & Visual Consistency

**Feature Branch**: `002-course-depth-preview`

**Created**: 2026-11-16

**Status**: Ready for planning

**Input**: User feedback after testing 1.0:
- Clicking "START THE QUIZ" crashed the app.
- The course header picture should become something that fits the concept but is not an image.
- Courses should have a preview screen (not in the main page or nav bar) instead of jumping back to
  the Learn screen.
- Study the Codédex courses deeply and make HTML, CSS and JS as thorough.
- Spacing and design should be consistent.
- Add badges for specific courses.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Lessons never crash (Priority: P1)

Learners can go through every slide, question and result screen of every lesson without the app
closing.

**Independent Test**: An automated test plays every lesson of every course through the real UI
using keyboard input and reaches "Quest Complete".

**Acceptance Scenarios**:
1. **Given** the last slide of a lesson, **When** the learner presses START THE QUIZ, **Then** the
   first question appears and the app keeps running.

### User Story 2 - Course preview before starting (Priority: P1)

Selecting a course anywhere outside the Learn map (Train → Courses, "More courses", the course menu,
the Hero profile) opens a full-screen **Course Preview**. It shows the course badge, level, category,
description, what you'll learn, size (chapters, lessons, projects, estimated time), the chapter
outline with each lesson and checkpoint project, the learner's progress, and one main action:
Start, Continue or Review.

**Independent Test**: From Train → Courses, open CSS. The preview appears with no bottom nav bar.
START opens CSS's next lesson directly. Back returns to where the learner was.

**Acceptance Scenarios**:
1. **Given** a course with no progress, **When** previewed, **Then** the action reads START COURSE and
   starts lesson 1.
2. **Given** a course in progress, **When** previewed, **Then** the action reads CONTINUE and names
   the next lesson.
3. **Given** a finished course, **When** previewed, **Then** the action reads REVIEW and the earned
   badge is shown with its date.

### User Story 3 - Codédex-depth HTML, CSS and JavaScript (Priority: P1)

The three web courses follow a complete beginner path with a checkpoint project closing each
chapter:

- **HTML**: Elements → Structure → Forms → Semantic HTML.
- **CSS**: Selectors → Properties → Box Model → Layout → Flexbox.
- **JavaScript**: The Console → Variables → Conditionals → Loops → Arrays → Functions → Objects →
  The Page (DOM).

All lesson text is original. Every typed answer produces correct, meaningful output.

**Independent Test**: The content-integrity test passes over every lesson. Each course has at least
4 chapters, and every chapter ends with a checkpoint project.

**Acceptance Scenarios**:
1. **Given** a JavaScript objects lesson, **When** the learner types an object literal across
   several lines and logs a property, **Then** the value prints.
2. **Given** a DOM lesson, **When** the learner sets `textContent` on an element, **Then** the output
   shows the page change.
3. **Given** progress saved by 1.0, **When** 1.1 starts, **Then** lessons whose topic still exists
   stay cleared.

### User Story 4 - Course badges (Priority: P2)

Each course has its own named badge (e.g. HTML "Stonemason"). Finishing every lesson in the course
earns it, with a toast and a log entry. Badges appear in the course preview (locked or earned) and on
the Hero profile's badge shelf.

**Independent Test**: Finish a course, then see the badge earned in the preview and on the profile,
still there after a restart.

### User Story 5 - A header that fits the concept (Priority: P2)

The Learn tab's course header is no longer a landscape picture. It is a "quest card" made of
interface elements: the course badge, title, level and category, the next quest, progress, and a
code line in the course's language that types itself out.

### User Story 6 - Consistent spacing and design (Priority: P2)

Every screen uses the same spacing scale, section titles, card padding and button sizes.

**Independent Test**: Screenshot review of every screen. Section titles look identical, card gaps
are uniform, and screen padding is the same on all screens.

### Edge Cases

- A save from 1.0 with lessons that no longer exist: those keys are dropped silently. XP, gems and
  streak are unaffected.
- A course preview opened while the learner has 0 hearts: Start shows the Out of Hearts dialog.
- A very long course description or lesson title: the text wraps and nothing is clipped.

## Requirements *(mandatory)*

- **FR-001**: Lesson state changes MUST apply after a whole frame is drawn. A UI test MUST play every
  lesson end to end.
- **FR-002**: A Course Preview screen MUST exist with: badge, title, level, category, description,
  "what you'll learn" (3–6 items), stats (chapters, lessons, projects, hours), chapter outline with
  lesson titles, done/locked/project markers, progress, badge status, and a primary action.
- **FR-003**: Every course entry point except the Learn map itself MUST open the preview.
- **FR-004**: HTML MUST have ≥ 4 chapters, CSS ≥ 5 and JavaScript ≥ 8. Each chapter MUST end with a
  checkpoint project lesson. Projects give +10 bonus XP on first clear.
- **FR-005**: The JavaScript runner MUST support objects (multi-line literals, dot access, property
  updates), template literals, `typeof`, `Math.random`, `break`, string methods, and a simulated page
  (`document.getElementById`/`querySelector`, `textContent`, `innerHTML`, `style.*`,
  `addEventListener`, `alert`).
- **FR-006**: The HTML preview MUST render form controls (input types, textarea, select, label) and
  semantic containers. The CSS preview MUST show borders, padding, underline and italics where
  present.
- **FR-007**: Lesson progress MUST be keyed by stable lesson ids (`course/lesson-slug`). 1.0 saves
  MUST migrate.
- **FR-008**: Each course MUST define a badge (name, icon, colour). Earning MUST be persisted with
  its day.
- **FR-009**: The Learn header MUST be a code-drawn quest card (no scenery picture).
- **FR-010**: Spacing MUST come from a shared scale (4/8/12/16/24) with one section-title style and
  one screen padding.

## Success Criteria *(mandatory)*

- **SC-001**: 0 crashes across a full UI playthrough of all lessons (automated).
- **SC-002**: 100% of typed answers run without error (automated).
- **SC-003**: Lesson count for HTML+CSS+JS grows from 24 to ≥ 80.
- **SC-004**: Every course entry point leads to the preview (manual check against the FR-003 list).

## Assumptions

- Python and SQL keep their current content this round (the user asked to focus on HTML, CSS and
  JS first).
- Badge art is drawn in code around existing Pixelrepo icons. itch.io's download flow needs a browser
  session, so no itch assets are bundled this round.
- Lesson topics and order are informed by Codédex's public curriculum. All wording, examples and
  project themes are original and RPG-flavoured.
