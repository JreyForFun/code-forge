# Feature 004: Polish, bigger Practice, certificates

**Status**: Implemented · **Branch**: `004-polish-practice` · **Version**: 1.3.0

## Feedback addressed
1. Completion ribbon text touched the ribbon's trim → ribbon sized from its border + padding.
2. Ranks: the player's avatar had a coloured block behind it → plain icon like the rivals.
3. Chapter chevron poked outside its stone frame → moved inside the 10px border.
4. Scrollbars were visible → invisible (scrolling by wheel, drag and touch still works).
5. Nav bar: selected tab's icon and label overlapped the frame → taller bar, content inside the border.
6. Code read as CAPITALS (`console.log` showed as CONSOLE.LOG) because both pixel fonts are caps-only →
   body text uses Pixelify Sans (real lowercase, ligatures removed so "fi" stays "fi").
7. Badge vs certificate → new **course certificate** screen (earned once, savable as PNG), separate from
   the progress badge (bronze/silver/gold).
8. Practice made much bigger: Daily Quest, course reviews, 10 challenges in a grid with solved marks,
   4-champion Duel Arena (5-question duels, wins/losses record).

## Review additions
- Text size setting (100 / 115 / 130 %).
- Practice runs (daily, review, duel) never cost hearts; challenges pay full rewards only the first time.
