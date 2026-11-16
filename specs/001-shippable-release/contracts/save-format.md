# Contract: Save file format (v2)

Location: `%APPDATA%\pixel_learn\save.txt` (fallback `$HOME/pixel_learn/save.txt`, then `./pixel_learn/`).
Backup: `save.bak` (previous good save). Temp: `save.tmp`.

Encoding: UTF-8, one `key=value` per line, `\n` line endings. Unknown keys MUST be ignored. Repeated keys
append (for set/list keys). Values never contain raw newlines.

| Key | Value | Since |
|---|---|---|
| `version` | `2` (first line) | v2 |
| `onboarded` | `0`/`1` | v2 |
| `name`, `country` | text | v1 |
| `avatar`, `course` | integer | v2 |
| `xp`,`gems`,`hearts`,`heart_t`,`streak`,`streak_max`,`last_day`,`savers`,`week_id`,`week_mask`,`week_xp`,`free_day`,`goal`,`league` | integer | v1 |
| `last_result` | text | v1 |
| `scan`,`floor`,`notif` | `0`/`1` | v1 |
| `done` | `c.m.l` (repeatable) | v1 |
| `qdone` | `c.m.l|i,j,k` (repeatable) | v1 |
| `best` | `c.m.l|best|attempts` (repeatable) | v1 |
| `code` | `title\x1Flang\x1Fcode` with newlines as `\x1E` (repeatable) | v1 |
| `ach` | `id|day` (repeatable) | v2 |
| `log` | `day|text` (repeatable, file order = chronological) | v2 |

Legacy (no `version`): load all v1 keys. Set `onboarded = 1` if any progress exists (xp > 0, done non-empty or codes non-empty).

Write procedure: serialize → write `save.tmp` → copy `save.txt` to `save.bak` (if present) → rename `save.tmp` to `save.txt`.
On failure: return error; caller shows a one-time warning toast.
