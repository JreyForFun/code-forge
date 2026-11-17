//! Pure game rules. No UI, no clock: every time-based function takes `now` (seconds) or `day`.
use crate::save::{Save, MAX_HEARTS, MAX_SAVERS};

pub const XP_PER_LEVEL: u32 = 100;
pub const HEART_SECS: u64 = 300;
pub const REFILL_COST: u32 = 10;
pub const SAVER_COST: u32 = 20;
pub const HINT_COST: u32 = 1;
/// Extra XP for the first clear of a checkpoint project.
pub const PROJECT_BONUS: u32 = 10;
pub const LEAGUES: [&str; 7] = ["Wood", "Stone", "Bronze", "Iron", "Silver", "Gold", "Crystal"];

pub fn day_of(now: u64) -> u64 { now / 86_400 }
/// Weeks start on Monday (1970-01-01 was a Thursday).
pub fn week_of(day: u64) -> u64 { (day + 3) / 7 }
/// 0 = Monday ... 6 = Sunday.
pub fn weekday(day: u64) -> usize { ((day + 3) % 7) as usize }

pub fn level(xp: u32) -> u32 { xp / XP_PER_LEVEL + 1 }
pub fn level_frac(xp: u32) -> f32 { (xp % XP_PER_LEVEL) as f32 / XP_PER_LEVEL as f32 }

/// Regenerate hearts. Safe if the clock moved backwards.
pub fn tick_hearts(s: &mut Save, now: u64) {
    if s.hearts >= MAX_HEARTS || now < s.heart_t { s.heart_t = now; s.hearts = s.hearts.min(MAX_HEARTS); return; }
    while s.hearts < MAX_HEARTS && now >= s.heart_t + HEART_SECS { s.hearts += 1; s.heart_t += HEART_SECS; }
    if s.hearts >= MAX_HEARTS { s.heart_t = now; }
}

/// Seconds until the next heart, or None when full.
pub fn next_heart_in(s: &Save, now: u64) -> Option<u64> {
    if s.hearts >= MAX_HEARTS { return None; }
    Some((s.heart_t + HEART_SECS).saturating_sub(now))
}

pub fn lose_heart(s: &mut Save, now: u64) {
    if s.hearts >= MAX_HEARTS { s.heart_t = now; }
    s.hearts = s.hearts.saturating_sub(1);
}

#[derive(Debug, PartialEq)]
pub enum ShopErr { Full, NotEnoughGems, AlreadyClaimed, MaxHeld }

impl ShopErr {
    pub fn msg(&self) -> &'static str {
        match self { ShopErr::Full => "Hearts already full", ShopErr::NotEnoughGems => "Not enough gems", ShopErr::AlreadyClaimed => "Come back tomorrow", ShopErr::MaxHeld => "You hold the maximum" }
    }
}

pub fn buy_refill(s: &mut Save) -> Result<(), ShopErr> {
    if s.hearts >= MAX_HEARTS { return Err(ShopErr::Full); }
    if s.gems < REFILL_COST { return Err(ShopErr::NotEnoughGems); }
    s.gems -= REFILL_COST; s.hearts = MAX_HEARTS; Ok(())
}

pub fn claim_free_heart(s: &mut Save, day: u64) -> Result<(), ShopErr> {
    if s.free_day == day { return Err(ShopErr::AlreadyClaimed); }
    if s.hearts >= MAX_HEARTS { return Err(ShopErr::Full); }
    s.free_day = day; s.hearts += 1; Ok(())
}

pub fn buy_saver(s: &mut Save) -> Result<(), ShopErr> {
    if s.savers >= MAX_SAVERS { return Err(ShopErr::MaxHeld); }
    if s.gems < SAVER_COST { return Err(ShopErr::NotEnoughGems); }
    s.gems -= SAVER_COST; s.savers += 1; Ok(())
}

/// Record activity on `day`. Returns true if the streak grew today.
pub fn bump_streak(s: &mut Save, day: u64) -> bool {
    s.week_mask |= 1 << weekday(day);
    if s.last_day == day || (s.last_day > day && s.last_day != 0) { return false; }
    if s.last_day != 0 && s.last_day + 1 == day { s.streak += 1; }
    else if s.last_day != 0 && s.last_day + 2 == day && s.savers > 0 { s.savers -= 1; s.streak += 1; }
    else { s.streak = 1; }
    s.streak_max = s.streak_max.max(s.streak);
    s.last_day = day;
    true
}

/// Streak as it stands today: a streak that can no longer continue reads as 0.
pub fn live_streak(s: &Save, day: u64) -> u32 {
    if s.last_day == 0 { return 0; }
    let gap = day.saturating_sub(s.last_day);
    if gap <= 1 || (gap == 2 && s.savers > 0) { s.streak } else { 0 }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LeagueResult { Up, Down, Stay }

impl LeagueResult {
    pub fn code(&self) -> &'static str { match self { LeagueResult::Up => "LEVEL_UP", LeagueResult::Down => "LEVEL_DOWN", LeagueResult::Stay => "FREEZE" } }
}

/// Apply the weekly league result when a new week starts. Returns the result if one was applied.
pub fn week_roll(s: &mut Save, day: u64) -> Option<LeagueResult> {
    let wk = week_of(day);
    if s.week_id == wk || (s.week_id > wk && s.week_id != 0) { return None; }
    let mut out = None;
    if s.week_id != 0 {
        // final standings of the week that just ended
        let rows = board(s, 7, 0.0);
        let pl = place(&rows);
        let from = s.league;
        let r = if s.week_xp > 0 && pl <= PROMOTE_TOP && s.league < 6 { s.league += 1; LeagueResult::Up }
            else if (s.week_xp == 0 || pl > rows.len() - DEMOTE_BOTTOM) && s.league > 0 { s.league -= 1; LeagueResult::Down }
            else { LeagueResult::Stay };
        let gems = if s.week_xp > 0 && pl <= 3 { PODIUM_GEMS[pl - 1] } else { 0 };
        s.gems = s.gems.saturating_add(gems);
        s.res_place = pl as u32; s.res_from = from; s.res_to = s.league; s.res_xp = s.week_xp; s.res_gems = gems; s.res_seen = false;
        s.last_result = r.code().into();
        // the ghost replays the week that just ended, unless whole weeks were skipped
        s.ghost = if wk == s.week_id + 1 { s.week_days } else { [0; 7] };
        s.hist.push(if wk == s.week_id + 1 { s.week_xp } else { 0 });
        if s.hist.len() > 4 { s.hist.remove(0); }
        out = Some(r);
    }
    s.week_id = wk; s.week_mask = 0; s.week_xp = 0; s.week_days = [0; 7];
    out
}

/// (xp, gems) for finishing a lesson.
pub fn lesson_reward(first: bool, perfect: bool) -> (u32, u32) {
    if first { (10 + if perfect { 5 } else { 0 }, 5) } else { (5, 0) }
}

/// Percent of answers that were right on the first try.
pub fn score(total: u32, mistakes: u32) -> u32 {
    let attempts = total + mistakes;
    (total * 100).checked_div(attempts).map(|s| s.min(100)).unwrap_or(100)
}

pub fn add_xp(s: &mut Save, xp: u32, gems: u32, day: u64) {
    s.xp = s.xp.saturating_add(xp);
    s.week_xp = s.week_xp.saturating_add(xp);
    s.week_days[weekday(day)] = s.week_days[weekday(day)].saturating_add(xp);
    s.gems = s.gems.saturating_add(gems);
}

// ------------------------------------------------------------------ anti-farming
/// Most XP per day from repeats: lesson replays, extra daily quests, reviews, solved challenges, duels
/// past the daily reward limit. First clears are never limited.
pub const REPEAT_XP_DAILY: u32 = 60;
/// Duel wins per day that pay the full reward.
pub const DUEL_REWARDS_DAILY: u32 = 3;

/// Take up to `want` XP from today's repeat budget.
pub fn repeat_xp(s: &mut Save, day: u64, want: u32) -> u32 {
    if s.rep_day != day { s.rep_day = day; s.rep_xp = 0; }
    let give = want.min(REPEAT_XP_DAILY.saturating_sub(s.rep_xp));
    s.rep_xp += give;
    give
}

/// True if a duel win today still pays the full reward (and counts it).
pub fn duel_reward_left(s: &mut Save, day: u64) -> bool {
    if s.duel_day != day { s.duel_day = day; s.duel_today = 0; }
    if s.duel_today < DUEL_REWARDS_DAILY { s.duel_today += 1; true } else { false }
}

// ------------------------------------------------------------------ league board
use crate::data::{Style, RIVALS};

/// Rivals per week, and how strong each is compared with the player's usual week
/// (one stretch, one just ahead, one just behind, two easier).
pub const RIVAL_FACTORS: [f32; 5] = [1.45, 1.12, 0.9, 0.68, 0.42];
/// Places that promote, and places from the bottom that demote.
pub const PROMOTE_TOP: usize = 3;
pub const DEMOTE_BOTTOM: usize = 2;
/// Gems for finishing 1st, 2nd and 3rd.
pub const PODIUM_GEMS: [u32; 3] = [10, 6, 3];

#[derive(Clone, Debug, PartialEq)]
pub enum Who { You, Ghost, Rival(Style) }

#[derive(Clone, Debug)]
pub struct Row { pub name: String, pub art: &'static str, pub xp: u32, pub who: Who }

/// XP a "usual week" is worth for this player: recent weeks, but never below the league's floor.
pub fn rival_target(s: &Save) -> f32 {
    let avg = if s.hist.is_empty() { 0.0 } else { s.hist.iter().sum::<u32>() as f32 / s.hist.len() as f32 };
    avg.max(40.0 + s.league as f32 * 15.0)
}

/// The five rivals of a week (indexes into RIVALS), shuffled by the week number.
pub fn week_rivals(week: u64) -> [usize; 5] {
    let mut idx: Vec<usize> = (0..RIVALS.len()).collect();
    let mut x = week.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1;
    for i in (1..idx.len()).rev() { x ^= x << 13; x ^= x >> 7; x ^= x << 17; idx.swap(i, (x % (i as u64 + 1)) as usize); }
    [idx[0], idx[1], idx[2], idx[3], idx[4]]
}

/// Share of a weekly total done by `wd` (0 = Monday) and `frac` of that day, following `days`.
/// Within a day, XP arrives between 8:00 and 22:00, more of it in the evening.
fn progress(days: &[f32; 7], wd: usize, frac: f32) -> f32 {
    let before: f32 = days[..wd.min(7)].iter().sum();
    let t = ((frac * 24.0 - 8.0) / 14.0).clamp(0.0, 1.0);
    let today = if wd < 7 { days[wd] * t * t * (3.0 - 2.0 * t) } else { 0.0 };
    (before + today).min(1.0)
}

/// The league board at weekday `wd` and fraction of day `frac` (use wd = 7 for the final standings).
pub fn board(s: &Save, wd: usize, frac: f32) -> Vec<Row> {
    let target = rival_target(s);
    let mut rows = vec![Row { name: s.name.clone(), art: "", xp: s.week_xp, who: Who::You }];
    for (k, i) in week_rivals(s.week_id).iter().enumerate() {
        let r = &RIVALS[*i];
        let xp = (target * RIVAL_FACTORS[k] * progress(&r.style.days(), wd, frac)).round() as u32;
        rows.push(Row { name: r.name.into(), art: r.art, xp, who: Who::Rival(r.style) });
    }
    if s.hist.last().is_some_and(|x| *x > 0) {
        let total: u32 = s.ghost.iter().sum();
        let mut g: u32 = s.ghost[..wd.min(7)].iter().sum();
        if wd < 7 { g += (s.ghost[wd] as f32 * frac) as u32; }
        rows.push(Row { name: "You, last week".into(), art: "", xp: g.min(total), who: Who::Ghost });
    }
    // ties go to the player
    rows.sort_by(|a, b| b.xp.cmp(&a.xp).then((a.who != Who::You).cmp(&(b.who != Who::You))));
    rows
}

/// 1-based place of the player on a board.
pub fn place(rows: &[Row]) -> usize { rows.iter().position(|r| r.who == Who::You).map(|p| p + 1).unwrap_or(rows.len()) }

// ---------------- achievements ----------------
pub struct Ctx { pub courses_cleared: usize }

pub struct Achievement { pub id: &'static str, pub title: &'static str, pub desc: &'static str, pub icon: &'static str, pub check: fn(&Save, &Ctx) -> bool }

pub const ACHIEVEMENTS: &[Achievement] = &[
    Achievement { id: "first_lesson", title: "First Steps", desc: "Clear your first lesson", icon: "book", check: |s, _| !s.done.is_empty() },
    Achievement { id: "ten_lessons", title: "Apprentice", desc: "Clear 10 lessons", icon: "spellbook", check: |s, _| s.done.len() >= 10 },
    Achievement { id: "course_clear", title: "Master", desc: "Clear a whole course", icon: "crown", check: |_, c| c.courses_cleared >= 1 },
    Achievement { id: "streak3", title: "Kindled", desc: "Reach a 3 day streak", icon: "flame", check: |s, _| s.streak_max >= 3 },
    Achievement { id: "streak7", title: "Campfire", desc: "Reach a 7 day streak", icon: "campfire", check: |s, _| s.streak_max >= 7 },
    Achievement { id: "xp100", title: "Scholar", desc: "Earn 100 XP", icon: "scroll_lightning", check: |s, _| s.xp >= 100 },
    Achievement { id: "xp500", title: "Sage", desc: "Earn 500 XP", icon: "hat_wizard", check: |s, _| s.xp >= 500 },
    Achievement { id: "gems50", title: "Treasure", desc: "Hold 50 gems", icon: "money_bag", check: |s, _| s.gems >= 50 },
    Achievement { id: "first_spell", title: "Spellwright", desc: "Save your own program", icon: "staff", check: |s, _| !s.codes.is_empty() },
    Achievement { id: "duel_win", title: "Duelist", desc: "Win a duel", icon: "axe", check: |s, _| s.duel_wins >= 1 },
    Achievement { id: "flawless", title: "Flawless", desc: "Clear a lesson with no mistakes", icon: "medallion", check: |s, _| s.flawless >= 1 },
    Achievement { id: "league_up", title: "Climber", desc: "Get promoted to a new league", icon: "banner_royal", check: |s, _| s.league >= 1 },
];

pub fn achievement(id: &str) -> Option<&'static Achievement> { ACHIEVEMENTS.iter().find(|a| a.id == id) }

/// Grant every achievement whose condition now holds. Returns the newly earned ones.
pub fn grant_achievements(s: &mut Save, ctx: &Ctx, day: u64) -> Vec<&'static Achievement> {
    let mut new = vec![];
    for a in ACHIEVEMENTS {
        if !s.ach.contains_key(a.id) && (a.check)(s, ctx) { s.ach.insert(a.id.into(), day); new.push(a); }
    }
    new
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn rivals_adapt_and_the_board_moves_through_the_week() {
        let mut s = Save::new();
        s.week_id = 3000; s.name = "Ada".into();
        let early = board(&s, 0, 0.0);
        assert!(early.iter().filter(|r| matches!(r.who, Who::Rival(_))).all(|r| r.xp == 0), "Monday morning starts at zero");
        let end = board(&s, 7, 0.0);
        let top = end.iter().map(|r| r.xp).max().unwrap();
        s.hist = vec![400, 400];
        let strong = board(&s, 7, 0.0).iter().map(|r| r.xp).max().unwrap();
        assert!(strong > top, "rivals grow with the player's usual week");
        assert_eq!(end.len(), 6, "no ghost before there is a last week");
        s.ghost = [10, 10, 10, 10, 10, 10, 10]; s.hist = vec![70];
        assert!(board(&s, 3, 0.5).iter().any(|r| r.who == Who::Ghost && r.xp == 35));
    }

    #[test]
    fn placement_decides_the_league_and_pays_the_podium() {
        let mut s = Save::new();
        s.week_id = 3000; s.league = 2; s.week_xp = 1000; s.week_days = [1000, 0, 0, 0, 0, 0, 0];
        let gems = s.gems;
        assert_eq!(week_roll(&mut s, 3001 * 7 - 3), Some(LeagueResult::Up));
        assert_eq!((s.res_place, s.res_from, s.res_to), (1, 2, 3));
        assert_eq!(s.gems, gems + PODIUM_GEMS[0]);
        assert_eq!(s.ghost[0], 1000, "last week becomes the ghost");
        assert_eq!(s.week_xp, 0);
        // a week with almost nothing ends in the bottom two
        s.week_xp = 1; s.week_days = [1, 0, 0, 0, 0, 0, 0];
        assert_eq!(week_roll(&mut s, 3002 * 7 - 3), Some(LeagueResult::Down));
    }

    #[test]
    fn repeats_and_duels_are_capped_per_day() {
        let mut s = Save::new();
        let mut total = 0;
        for _ in 0..50 { total += repeat_xp(&mut s, 100, 5); }
        assert_eq!(total, REPEAT_XP_DAILY);
        assert_eq!(repeat_xp(&mut s, 101, 5), 5, "the budget refills the next day");
        let wins = (0..10).filter(|_| duel_reward_left(&mut s, 100)).count();
        assert_eq!(wins as u32, DUEL_REWARDS_DAILY);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels() {
        assert_eq!(level(0), 1);
        assert_eq!(level(99), 1);
        assert_eq!(level(100), 2);
        assert_eq!(level(399), 4);
    }

    #[test]
    fn hearts_regenerate_every_five_minutes() {
        let mut s = Save::new();
        s.hearts = 5;
        lose_heart(&mut s, 1000);
        lose_heart(&mut s, 1010);
        assert_eq!(s.hearts, 3);
        assert_eq!(s.heart_t, 1000);
        tick_hearts(&mut s, 1299);
        assert_eq!(s.hearts, 3);
        assert_eq!(next_heart_in(&s, 1299), Some(1));
        tick_hearts(&mut s, 1300);
        assert_eq!(s.hearts, 4);
        tick_hearts(&mut s, 5000);
        assert_eq!(s.hearts, 5);
        assert_eq!(next_heart_in(&s, 5000), None);
    }

    #[test]
    fn hearts_survive_clock_going_backwards() {
        let mut s = Save::new();
        s.hearts = 1; s.heart_t = 10_000;
        tick_hearts(&mut s, 50);
        assert_eq!(s.hearts, 1);
        assert_eq!(s.heart_t, 50);
        lose_heart(&mut s, 60);
        lose_heart(&mut s, 70);
        assert_eq!(s.hearts, 0);
    }

    #[test]
    fn shop_rules() {
        let mut s = Save::new();
        assert_eq!(buy_refill(&mut s), Err(ShopErr::Full));
        s.hearts = 1; s.gems = 9;
        assert_eq!(buy_refill(&mut s), Err(ShopErr::NotEnoughGems));
        s.gems = 10;
        assert_eq!(buy_refill(&mut s), Ok(()));
        assert_eq!((s.hearts, s.gems), (5, 0));
        s.hearts = 2;
        assert_eq!(claim_free_heart(&mut s, 7), Ok(()));
        assert_eq!(claim_free_heart(&mut s, 7), Err(ShopErr::AlreadyClaimed));
        assert_eq!(s.hearts, 3);
        s.gems = 100;
        for _ in 0..3 { buy_saver(&mut s).unwrap(); }
        assert_eq!(buy_saver(&mut s), Err(ShopErr::MaxHeld));
        assert_eq!(s.gems, 40);
    }

    #[test]
    fn streak_grows_resets_and_uses_savers() {
        let mut s = Save::new();
        assert!(bump_streak(&mut s, 100));
        assert_eq!(s.streak, 1);
        assert!(!bump_streak(&mut s, 100));
        assert!(bump_streak(&mut s, 101));
        assert_eq!(s.streak, 2);
        s.savers = 1;
        assert!(bump_streak(&mut s, 103)); // missed day 102, saver used
        assert_eq!((s.streak, s.savers), (3, 0));
        assert!(bump_streak(&mut s, 106)); // missed two days
        assert_eq!(s.streak, 1);
        assert_eq!(s.streak_max, 3);
    }

    #[test]
    fn streak_ignores_clock_going_backwards() {
        let mut s = Save::new();
        bump_streak(&mut s, 200);
        bump_streak(&mut s, 201);
        assert!(!bump_streak(&mut s, 150));
        assert_eq!((s.streak, s.last_day), (2, 201));
    }

    #[test]
    fn live_streak_reads_zero_when_broken() {
        let mut s = Save::new();
        assert_eq!(live_streak(&s, 10), 0);
        bump_streak(&mut s, 10);
        bump_streak(&mut s, 11);
        assert_eq!(live_streak(&s, 12), 2);
        assert_eq!(live_streak(&s, 13), 0);
        s.savers = 1;
        assert_eq!(live_streak(&s, 13), 2);
    }

    #[test]
    fn week_roll_promotes_relegates_and_freezes() {
        let mut s = Save::new();
        let d0 = 20_000;
        assert_eq!(week_roll(&mut s, d0), None); // first run just records the week
        s.week_xp = 150;
        assert_eq!(week_roll(&mut s, d0 + 1 + 7), Some(LeagueResult::Up));
        assert_eq!((s.league, s.week_xp), (1, 0));
        assert_eq!(week_roll(&mut s, d0 + 1 + 7), None);
        assert_eq!(week_roll(&mut s, d0 + 15), Some(LeagueResult::Down));
        assert_eq!(s.league, 0);
        s.week_xp = 30;
        assert_eq!(week_roll(&mut s, d0 + 22), Some(LeagueResult::Stay));
        s.league = 6; s.week_xp = 500;
        assert_eq!(week_roll(&mut s, d0 + 29), Some(LeagueResult::Stay));
        assert_eq!(s.league, 6);
        // backwards clock: nothing happens
        assert_eq!(week_roll(&mut s, d0), None);
    }

    #[test]
    fn weekdays_start_monday() {
        // 2026-11-16 is a Monday: day 20773
        assert_eq!(weekday(20773), 0);
        assert_eq!(weekday(20779), 6);
        assert_eq!(week_of(20773), week_of(20779));
        assert_ne!(week_of(20779), week_of(20780));
    }

    #[test]
    fn rewards_and_score() {
        assert_eq!(lesson_reward(true, true), (15, 5));
        assert_eq!(lesson_reward(true, false), (10, 5));
        assert_eq!(lesson_reward(false, true), (5, 0));
        assert_eq!(score(4, 0), 100);
        assert_eq!(score(4, 4), 50);
        assert_eq!(score(0, 0), 100);
        let mut s = Save::new();
        s.xp = u32::MAX - 1;
        add_xp(&mut s, 10, 1, 20000);
        assert_eq!(s.xp, u32::MAX);
    }

    #[test]
    fn achievements_are_granted_once_and_kept() {
        let mut s = Save::new();
        let ctx = Ctx { courses_cleared: 0 };
        assert!(grant_achievements(&mut s, &ctx, 1).is_empty());
        s.done.insert("0.0.0".into());
        s.gems = 60;
        let got: Vec<&str> = grant_achievements(&mut s, &ctx, 5).iter().map(|a| a.id).collect();
        assert_eq!(got, vec!["first_lesson", "gems50"]);
        s.gems = 0; // spending gems does not take it away
        assert!(grant_achievements(&mut s, &ctx, 6).is_empty());
        assert_eq!(s.ach.get("gems50"), Some(&5));
        assert!(grant_achievements(&mut s, &Ctx { courses_cleared: 1 }, 7).iter().any(|a| a.id == "course_clear"));
    }

    #[test]
    fn achievement_table_is_well_formed() {
        assert!(ACHIEVEMENTS.len() >= 10);
        let mut ids: Vec<_> = ACHIEVEMENTS.iter().map(|a| a.id).collect();
        let mut icons: Vec<_> = ACHIEVEMENTS.iter().map(|a| a.icon).collect();
        ids.sort(); ids.dedup(); icons.sort(); icons.dedup();
        assert_eq!(ids.len(), ACHIEVEMENTS.len());
        assert_eq!(icons.len(), ACHIEVEMENTS.len(), "each achievement needs its own icon");
    }
}
