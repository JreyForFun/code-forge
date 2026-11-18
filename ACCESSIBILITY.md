# Accessibility

Code Forge is for anyone who wants to learn to code, so it should be usable by as many people as
possible. This page says honestly what works today, what doesn't yet, and how to tell us about a
barrier.

## Our commitment

- Every barrier report is treated as a bug, not a feature request.
- New screens are checked at small phone size, at the largest text size and with the keyboard only
  before they ship.
- Known limitations are listed here, not hidden.

## What's supported today

**Reading and seeing**
- **Text size**: *Settings → Text size* has Normal, Large and Huge. Layouts reflow at every size.
- **Readable fonts**: long explanations use a clean sans-serif font (Ubuntu), not the pixel font,
  and code is shown in a font with clear lowercase letters.
- **High contrast**: light text on dark stone backgrounds throughout.
- **Right and wrong answers** are spelled out in words ("CORRECT" or "NOT QUITE"), not shown by colour
  alone, and play different sounds.

**Moving and typing**
- **Keyboard-only play** on Windows: **Enter** = next / check / continue, **1–5** = pick an answer,
  **Esc** = leave a lesson, **Ctrl+Enter** or **F5** = run code in the Spell Lab.
- **Large touch targets** on phones and tablets.
- **No time limits** anywhere: lessons, reviews, the Daily Quest and even duels (a duel compares
  scores, not speed). Take as long as you need.
- **Phone back button** leaves the current screen on Android.

**Hearing and attention**
- **Sound is never required.** Every sound has something on screen that means the same thing.
  Music and effects have separate volume sliders in *Settings* and can be turned off completely.
- **No flashing**: animations are gentle (no strobing or rapid flashes), and the opening animation
  can be skipped with a tap, Enter or Esc.

## Supported environments

| Platform | Supported |
| --- | --- |
| Windows 10 and 11 (64-bit) | Yes |
| Android 8.0 and newer (arm64 phones; x86_64 emulators and Chromebooks) | Yes |
| Screen sizes | From 320 px wide phones to tablets and desktop monitors, portrait or landscape |
| iPhone / iPad, macOS, Linux | Not yet (Linux and macOS can build from source) |

## Known limitations

We know about these and want to fix them:

- **Screen readers.** The game draws its own interface, and most of it is not yet announced
  correctly by Narrator, NVDA, JAWS or TalkBack. Code Forge is not yet usable without sight.
- **No reduced-motion setting.** The welcome and opening animations can be skipped, but small
  background effects (torch flicker, sparks) can't be turned off yet.
- **Pixel font in headings and buttons.** Titles and button labels use a pixel-style font that some
  people find harder to read, even at Huge size.
- **No high-contrast or light theme**, and no colour-blind palettes yet.
- **Text input on Android** uses the system keyboard, which can occasionally need a second tap on
  the text box to reopen after it has been closed.

## Reporting a barrier

If something in Code Forge is hard or impossible for you to use, please tell us. You don't need to
know the technical cause.

- Open an [accessibility report](https://github.com/JreyForFun/code-forge/issues/new?template=accessibility.yml), or
- if you'd rather not post publicly, open an issue titled **"Private contact requested"** with no
  details, and the maintainer will reply with a private way to talk.

Helpful details: your device and operating system, any assistive technology you use (screen
reader, magnifier, switch control...), the screen you were on, and what happened. We aim to reply
within 7 days.
