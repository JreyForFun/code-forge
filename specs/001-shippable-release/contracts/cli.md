# Contract: Command-line flags (developer/QA)

Normal users launch with no arguments. These flags exist for screenshot QA and are stable:

| Flag | Effect |
|---|---|
| `--screenshot <file.png>` | Render ~120 frames, save a PNG of the window, exit. Uses a demo save in memory (never written to disk). |
| `--tab <learn\|train\|ranks\|create\|hero>` | Start on that tab (aliases: `community`=train, `leader`=ranks, `profile`=hero). |
| `--lesson C.M.L [--stage N] [--skip N]` | Open a lesson directly at a slide/question. |
| `--onboarding` | Force the welcome flow (with `--screenshot`). |
| `--screen <settings\|editor\|complete>` | Open a secondary screen (with `--screenshot`). |
| `--version` | Print version and exit. |

In screenshot mode the save is never written (protects the developer's real progress).
