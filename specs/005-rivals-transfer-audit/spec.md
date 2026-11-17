# Feature 005: Smarter rivals, device transfer, full audit (release 1.0.0)

**Status**: Implemented · **Branch**: `005-rivals-transfer-audit` · **Version**: 1.0.0 (first public release)

## Smarter rivals (offline leagues)
- Five rivals per week, picked by week number, tuned to the player's recent weeks (stretch 1.45×,
  ahead 1.12×, behind 0.9×, easier 0.68× / 0.42×; floor rises with the league).
- Styles spread XP over the week (steady, weekend sprinter, early bird, late rush, streaky) and over the
  day (08:00–22:00, evening-weighted), so the board moves believably.
- Ghost row: the player's own last week, replayed day by day.
- Placement decides the league: top 3 up, bottom 2 down, zero XP always down. Podium chest 10/6/3 gems.
- Monday ceremony modal; the week also rolls over while the app is open.

## Device transfer
- Export to `.codeforge` (save + tag + checksum) via a native file dialog; import from Settings, from the
  welcome screen (new device) or by drag-and-drop. Confirmation shows the incoming hero; the replaced
  save is backed up; imported data is validated.

## Audit findings and fixes
| # | Finding | Fix |
|---|---|---|
| 1 | Days changed at UTC midnight (08:00 in UTC+8) | Local time zone via chrono |
| 2 | Turning the clock back replayed the daily quest / streak | Day never goes below the furthest day seen |
| 3 | Duels farmable for unlimited gems (questions repeat, no cost) | Full rewards for 3 wins/day, then small XP from a daily budget; no hints in duels |
| 4 | Replays, extra daily quests, reviews, solved challenges: unlimited XP → trivial promotion | Shared 60 XP/day repeat budget |
| 5 | Reviews paid gems with no limit | No gems from reviews |
| 6 | Practice score could not drop below 50% (wrong answers aren't repeated), so all-wrong reviews paid full | Practice score = right / total |
| 7 | Typed-answer check ignored case and all spaces, even inside text (`Print("hiworld")` passed) | Whitespace-insensitive outside strings only, quotes/semicolons flexible, case-sensitive except HTML/CSS/SQL |
| 8 | QA command-line flags worked on the real save (`--screen cert` granted a certificate, `--skip` skipped questions) | Only in screenshot mode |
| 9 | Save editable by hand (gems, lessons, certificates) | Checksum; on mismatch gems reset and impossible data dropped |
| 10 | Unknown lesson ids / certificates without a finished course were trusted | Validated on load and import |
| 11 | Resumed lessons could count stale answer indexes | Indexes outside the lesson are ignored |
| 12 | An empty practice run would still pay its reward | Empty runs never start |
| 13 | `input()` always returned empty text in the editor | Program-input box; input()/prompt() read it |
| 14 | Saved spells had no limit | 200 spells |
| 15 | Import was impossible on a new device (welcome flow had no dialogs) | Import on the welcome screen; dialogs shown during onboarding |
| 16 | "Spell Forge" clashed with the new app name | Renamed "Spell Lab" |

## Known limits (offline by design)
- Moving the clock forward still refills hearts early; an offline app can't detect it.
- The checksum stops casual edits, not a determined cheater reading the source.
