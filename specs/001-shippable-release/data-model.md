# Data Model: Pixel Learn 1.0

All learner data lives in one `Save` record (see [contracts/save-format.md](contracts/save-format.md)).
Course content is static and compiled in.

## Save (learner state)

| Field | Type | Rules |
|---|---|---|
| version | u32 | 2 for this release; missing = 1 (legacy) |
| onboarded | bool | false → show welcome flow. Legacy saves with any progress count as onboarded |
| name | text ≤ 18 | empty → "Hero"; newlines stripped |
| avatar | u32 | index into AVATARS (0–7); out of range → 0 |
| course | u32 | last selected course index; out of range → 0 |
| xp, gems | u32 | never negative; saturating maths |
| hearts | u32 | 0..=5 |
| heart_t | u64 secs | refill clock anchor |
| streak, streak_max | u32 | streak_max ≥ streak |
| last_day | u64 day | day of last completed activity |
| savers | u32 | 0..=3 |
| goal | u32 | one of 3, 7, 14, 30 (else 7) |
| week_id, week_mask, week_xp | u64, u8, u32 | week resets roll these |
| league | u32 | 0..=6 |
| last_result | text | LEVEL_UP / LEVEL_DOWN / FREEZE / empty |
| free_day | u64 day | day the free heart was claimed |
| done | set<"c.m.l"> | cleared lessons |
| qdone | map<"c.m.l", list<q>> | partial lesson progress |
| best | map<"c.m.l", (best%, attempts)> | |
| codes | list<Spell> | learner programs |
| ach | map<id, day> | earned achievements; never removed except by reset |
| log | list<LogEntry> | newest last; capped at 100 |
| scan, floor, notif_read | bool | settings / UI state |

## Spell

`title` (≤ 30 chars), `lang` ∈ {python, javascript, html, css, sql}, `code` (text).

## LogEntry

`day: u64`, `text: String` (single line).

## Achievement (static definition)

`id` (stable string), `title`, `desc`, `icon` (texture name), `check(&Save) -> bool`.
At least 10: first_lesson, ten_lessons, course_clear, streak3, streak7, xp100, xp500, gems50,
first_spell, duel_win, flawless, league_up.

## Course content (static)

Course → Module (chapter) → Lesson { title, slides[(text, icon)], qs[Q] } with
`Q = Mc | Fill{code with "___"} | Type{answer, guide}`.
Integrity rules (tested): `right < opts.len()`; Fill code contains `___`; every lesson has ≥ 1 slide
and ≥ 1 question; every Type answer runs through its course's interpreter with no `Err` output.

## Opponent / Rival / Avatar (static)

- Opponent: name, skill (1–3), colour, icon.
- Rival: name, weekly-XP factor, icon. Shown as "computer rival".
- Avatar: name, texture.

## State transitions

- **Hearts**: on wrong answer `h -= 1` (if h was max, anchor = now). On tick, while `h < 5` and
  `now ≥ anchor + 300`: `h += 1, anchor += 300`. If `now < anchor` (clock went back), anchor = now.
- **Streak** (on activity, day d): if `last == d` no change; if `last + 1 == d` +1; if `last + 2 == d`
  and savers > 0, use a saver and +1; else reset to 1. If `d < last` (clock went back), no change.
- **Week roll** (week w ≠ stored): if the stored week ≠ 0, apply LEVEL_UP (week_xp ≥ 100, league < 6),
  LEVEL_DOWN (week_xp = 0, league > 0) or FREEZE. Then reset week fields.
