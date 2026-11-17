# Code Forge: logo variants

Made with the `logo-design` skill (rampstackco/claude-skills). Brief: rename Pixel Learn to **Code Forge**;
a pixel-style hammer that still shows `</>`. Audience: beginners learning to code in a dungeon-RPG app.
Contexts tested: 256 px app icon, 32 px and 16 px window/taskbar icon, single colour (light on dark,
dark on light). Review sheets: `logo-variants/review.png`, `logo-variants/c_refined_review.png`.

## 01, Upright forge hammer (rejected)
- **Architecture:** symbol. **Symbol:** literal hammer, `</>` carved into the head in ember.
- **Application:** strong at 256 px; `</>` disappears at 16 px and in single colour (silhouette is a plain "T").
- **Signals:** craft, solidity. **Rejects:** nothing about code once the colour is gone.

## 02, Hammer mid-swing with code sparks (rejected)
- **Architecture:** symbol. **Symbol:** literal hammer at 45°, `<` and `>` as sparks.
- **Application:** readable, but the chevrons sit on a diagonal and no longer read as one `</>`.
- **Signals:** action, energy. **Rejects:** calm/corporate tech.

## 03, `</>` with the slash as a hammer (v1.4, replaced)
- **Architecture:** letterform-as-symbol + pixel wordmark lockup ("CODE" gold, "FORGE" steel, Press Start 2P).
- **Symbol:** letterform-derived. The `/` of `</>` is a forge hammer at exactly 45° (clean pixel staircases);
  the `<` `>` are glowing ember chevrons. One idea: forging code.
- **Colour tokens:** ember `#B04014 / #F08A24 / #FACC15`, steel `#4A5568 / #9AA7B8 / #D5DDE8`,
  wood `#5C381C / #8B5A2B / #B07A3C`, outline `#1A1424`, icon tile stone `#161322 / #242036 / #3A3454`.
  Single colour: `#F0F0F0` on dark, `#14141E` on light (both pass, parts stay separated).
- **Application:** 32 px grid master (`assets/art/logo_mark.png`), 24 px cut for 16 to 32 px icons,
  multi-size `assets/icon.ico` (16, 32, 48, 64, 128, 256) and `assets/icon.png` (256).
- **Signals:** making things with code, craft, a little magic. **Rejects:** generic laptop/terminal icons.

## Files
`assets/icon.ico`, `assets/icon.png`, `assets/art/logo_mark.png`, `design/logo-variants/lockup.png`.

## 04, Rune hammer (CHOSEN, v1.5), from the owner's sketch
- **Architecture:** symbol + pixel wordmark lockup. **Symbol:** literal war hammer at 45°, dark riveted
  iron head with flared striking faces, leather-wrapped handle and iron pommel; the runes `</>` are carved
  into the head and glow like embers, reading left to right as in the sketch.
- **Why it is distinctive:** code logos are usually brackets, terminals or laptops. This is a dungeon
  weapon first, and the code is the enchantment on it.
- **Motion:** the sketch's exploded frame becomes the welcome animation (head drops onto the anvil,
  handle slides in, the three runes fly in, final strike with sparks; idle glow and drifting embers).
- **Layers:** `assets/art/logo_{head,handle,lt,sl,gt}.png` share one 48 px canvas; `logo_mark.png` is
  the composite. Icons: `assets/icon.ico`, `assets/icon.png`. Review: `logo-variants/v2_review.png`.
- **Tests:** reads at 256, 64, 32 px; at 16 px the silhouette is a hammer with an orange core; single
  colour keeps the runes as cut-outs on both dark and light.
