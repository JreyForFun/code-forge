//! Pure game rules. No UI, no clock: every time-based function takes `now` (seconds) or `day`.
use crate::save::{Save, MAX_HEARTS, MAX_SAVERS};

pub const XP_PER_LEVEL: u32 = 100;
pub const HEART_SECS: u64 = 300;
pub const REFILL_COST: u32 = 10;
pub const SAVER_COST: u32 = 20;
pub const LEAGUE_UP_XP: u32 = 100;
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
        let r = if s.week_xp >= LEAGUE_UP_XP && s.league < 6 { s.league += 1; LeagueResult::Up }
            else if s.week_xp == 0 && s.league > 0 { s.league -= 1; LeagueResult::Down }
            else { LeagueResult::Stay };
        s.last_result = r.code().into();
        out = Some(r);
    }
    s.week_id = wk; s.week_mask = 0; s.week_xp = 0;
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

pub fn add_xp(s: &mut Save, xp: u32, gems: u32) {
    s.xp = s.xp.saturating_add(xp);
    s.week_xp = s.week_xp.saturating_add(xp);
    s.gems = s.gems.saturating_add(gems);
}

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
        add_xp(&mut s, 10, 1);
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
