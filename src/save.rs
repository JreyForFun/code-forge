//! Learner save data: format v2 (see specs/001-shippable-release/contracts/save-format.md).
//! Parsing and serializing are pure string functions so they can be tested; file IO is atomic.
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

pub const SAVE_VERSION: u32 = 2;
pub const MAX_HEARTS: u32 = 5;
pub const MAX_SAVERS: u32 = 3;
pub const MAX_LOG: usize = 100;
pub const NAME_MAX: usize = 18;
pub const GOALS: [u32; 4] = [3, 7, 14, 30];

#[derive(Clone, Debug, PartialEq)]
pub struct Save {
    pub version: u32,
    pub onboarded: bool,
    pub name: String,
    pub avatar: u32,
    pub course: u32,
    pub xp: u32,
    pub gems: u32,
    pub hearts: u32,
    pub heart_t: u64,
    pub streak: u32,
    pub streak_max: u32,
    pub last_day: u64,
    pub savers: u32,
    pub week_id: u64,
    pub week_mask: u8,
    pub week_xp: u32,
    pub done: HashSet<String>,
    pub best: HashMap<String, (u32, u32)>,
    pub qdone: HashMap<String, Vec<usize>>,
    pub free_day: u64,
    pub goal: u32,
    pub league: u32,
    pub last_result: String,
    pub scan: bool,
    pub floor: bool,
    pub notif_read: bool,
    pub codes: Vec<(String, String, String)>,
    pub ach: BTreeMap<String, u64>,
    pub log: Vec<(u64, String)>,
    pub duel_wins: u32,
    pub flawless: u32,
}

impl Default for Save {
    fn default() -> Self { Save::new() }
}

impl Save {
    pub fn new() -> Self {
        Save {
            version: SAVE_VERSION, onboarded: false, name: "Hero".into(), avatar: 0, course: 0,
            xp: 0, gems: 20, hearts: MAX_HEARTS, heart_t: 0, streak: 0, streak_max: 0, last_day: 0, savers: 0,
            week_id: 0, week_mask: 0, week_xp: 0, done: HashSet::new(), best: HashMap::new(), qdone: HashMap::new(),
            free_day: 0, goal: 7, league: 0, last_result: String::new(), scan: true, floor: true, notif_read: false,
            codes: vec![], ach: BTreeMap::new(), log: vec![], duel_wins: 0, flawless: 0,
        }
    }

    /// Display name with the 18-char limit and no control characters.
    pub fn clean_name(raw: &str) -> String {
        let n: String = raw.chars().filter(|c| !c.is_control()).take(NAME_MAX).collect();
        let n = n.trim().to_string();
        if n.is_empty() { "Hero".into() } else { n }
    }

    pub fn add_log(&mut self, day: u64, text: impl Into<String>) {
        let t: String = text.into().replace(['\n', '\r'], " ");
        if t.trim().is_empty() { return; }
        self.log.push((day, t));
        if self.log.len() > MAX_LOG { let extra = self.log.len() - MAX_LOG; self.log.drain(0..extra); }
    }

    /// Parse a save file. Never fails: unknown or malformed lines are skipped.
    pub fn parse(text: &str) -> Self {
        let mut s = Save::new();
        let mut version = 1;
        let mut saw_onboarded = false;
        for line in text.lines() {
            let line = line.trim_end_matches('\r');
            let Some((k, v)) = line.split_once('=') else { continue };
            let n = v.trim().parse::<u64>().ok();
            let u = |d: u32| n.map(|x| x.min(u32::MAX as u64) as u32).unwrap_or(d);
            let b = v == "1";
            match k {
                "version" => version = u(1),
                "onboarded" => { s.onboarded = b; saw_onboarded = true; }
                "name" => s.name = Save::clean_name(v),
                "avatar" => s.avatar = u(0),
                "course" => s.course = u(0),
                "xp" => s.xp = u(0),
                "gems" => s.gems = u(0),
                "hearts" => s.hearts = u(MAX_HEARTS).min(MAX_HEARTS),
                "heart_t" => s.heart_t = n.unwrap_or(0),
                "streak" => s.streak = u(0),
                "streak_max" => s.streak_max = u(0),
                "last_day" => s.last_day = n.unwrap_or(0),
                "savers" => s.savers = u(0).min(MAX_SAVERS),
                "week_id" => s.week_id = n.unwrap_or(0),
                "week_mask" => s.week_mask = (u(0) & 0x7f) as u8,
                "week_xp" => s.week_xp = u(0),
                "free_day" => s.free_day = n.unwrap_or(0),
                "goal" => { let g = u(7); s.goal = if GOALS.contains(&g) { g } else { 7 }; }
                "league" => s.league = u(0).min(6),
                "last_result" => s.last_result = v.into(),
                "scan" => s.scan = b,
                "floor" => s.floor = b,
                "notif" => s.notif_read = b,
                "duel_wins" => s.duel_wins = u(0),
                "flawless" => s.flawless = u(0),
                "done" => { if !v.is_empty() { s.done.insert(v.into()); } }
                "qdone" => {
                    if let Some((key, list)) = v.split_once('|') {
                        let l: Vec<usize> = list.split(',').filter_map(|x| x.parse().ok()).collect();
                        if !l.is_empty() { s.qdone.insert(key.into(), l); }
                    }
                }
                "best" => {
                    let p: Vec<&str> = v.split('|').collect();
                    if p.len() == 3 { if let (Ok(a), Ok(c)) = (p[1].parse(), p[2].parse()) { s.best.insert(p[0].into(), (a, c)); } }
                }
                "code" => {
                    let p: Vec<&str> = v.splitn(3, '\u{1f}').collect();
                    if p.len() == 3 { s.codes.push((p[0].into(), p[1].into(), p[2].replace('\u{1e}', "\n"))); }
                }
                "ach" => { if let Some((id, d)) = v.split_once('|') { s.ach.insert(id.into(), d.parse().unwrap_or(0)); } }
                "log" => { if let Some((d, t)) = v.split_once('|') { if let Ok(d) = d.parse() { s.add_log(d, t); } } }
                _ => {}
            }
        }
        s.streak_max = s.streak_max.max(s.streak);
        s.version = SAVE_VERSION;
        if version < 2 && !saw_onboarded {
            // Legacy prototype save: anyone who already played skips the welcome flow.
            s.onboarded = s.xp > 0 || !s.done.is_empty() || !s.codes.is_empty();
        }
        s
    }

    pub fn serialize(&self) -> String {
        let mut o = String::new();
        let mut w = |k: &str, v: String| { o.push_str(k); o.push('='); o.push_str(&v); o.push('\n'); };
        let one = |x: &str| x.replace(['\n', '\r'], " ");
        w("version", SAVE_VERSION.to_string());
        w("onboarded", (self.onboarded as u8).to_string());
        w("name", one(&self.name));
        w("avatar", self.avatar.to_string());
        w("course", self.course.to_string());
        for (k, v) in [("xp", self.xp), ("gems", self.gems), ("hearts", self.hearts), ("streak", self.streak), ("streak_max", self.streak_max),
            ("savers", self.savers), ("week_xp", self.week_xp), ("goal", self.goal), ("league", self.league), ("duel_wins", self.duel_wins), ("flawless", self.flawless)] {
            w(k, v.to_string());
        }
        for (k, v) in [("heart_t", self.heart_t), ("last_day", self.last_day), ("week_id", self.week_id), ("free_day", self.free_day)] { w(k, v.to_string()); }
        w("week_mask", self.week_mask.to_string());
        w("last_result", one(&self.last_result));
        w("scan", (self.scan as u8).to_string());
        w("floor", (self.floor as u8).to_string());
        w("notif", (self.notif_read as u8).to_string());
        let mut done: Vec<&String> = self.done.iter().collect();
        done.sort();
        for d in done { w("done", d.clone()); }
        let mut qd: Vec<_> = self.qdone.iter().filter(|(_, v)| !v.is_empty()).collect();
        qd.sort_by(|a, b| a.0.cmp(b.0));
        for (k, v) in qd { w("qdone", format!("{}|{}", k, v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))); }
        let mut best: Vec<_> = self.best.iter().collect();
        best.sort_by(|a, b| a.0.cmp(b.0));
        for (k, (b, a)) in best { w("best", format!("{}|{}|{}", k, b, a)); }
        for (t, l, c) in &self.codes { w("code", format!("{}\u{1f}{}\u{1f}{}", one(t), one(l), c.replace('\r', "").replace('\n', "\u{1e}"))); }
        for (id, d) in &self.ach { w("ach", format!("{}|{}", id, d)); }
        for (d, t) in &self.log { w("log", format!("{}|{}", d, one(t))); }
        o
    }

    pub fn dir() -> PathBuf {
        let base = std::env::var("APPDATA").or_else(|_| std::env::var("HOME")).unwrap_or_else(|_| ".".into());
        Path::new(&base).join("pixel_learn")
    }
    pub fn path() -> PathBuf { Self::dir().join("save.txt") }

    /// Load from disk. Missing file -> fresh save. Unreadable main file -> try the backup.
    pub fn load() -> Self { Self::load_from(&Self::path()) }

    pub fn load_from(path: &Path) -> Self {
        match std::fs::read_to_string(path) {
            Ok(t) => Save::parse(&t),
            Err(_) => match std::fs::read_to_string(path.with_extension("bak")) {
                Ok(t) => Save::parse(&t),
                Err(_) => Save::new(),
            },
        }
    }

    pub fn store(&self) -> std::io::Result<()> { self.store_to(&Self::path()) }

    /// Atomic write: tmp file, keep the previous save as .bak, then rename into place.
    pub fn store_to(&self, path: &Path) -> std::io::Result<()> {
        if let Some(d) = path.parent() { std::fs::create_dir_all(d)?; }
        let tmp = path.with_extension("tmp");
        std::fs::write(&tmp, self.serialize())?;
        if path.exists() { let _ = std::fs::copy(path, path.with_extension("bak")); }
        std::fs::rename(&tmp, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Save {
        let mut s = Save::new();
        s.onboarded = true; s.name = "Ada".into(); s.avatar = 3; s.course = 2; s.xp = 420; s.gems = 7; s.hearts = 2; s.heart_t = 123;
        s.streak = 4; s.streak_max = 9; s.last_day = 20000; s.savers = 1; s.week_id = 2857; s.week_mask = 0b101; s.week_xp = 60;
        s.done.insert("0.0.0".into()); s.done.insert("1.2.3".into());
        s.best.insert("0.0.0".into(), (80, 2));
        s.qdone.insert("0.0.1".into(), vec![0, 2]);
        s.free_day = 19999; s.goal = 14; s.league = 3; s.last_result = "LEVEL_UP".into(); s.scan = false; s.notif_read = true;
        s.codes.push(("My | prog".into(), "python".into(), "x = 1\nprint(x)\n".into()));
        s.ach.insert("first_lesson".into(), 20000);
        s.add_log(20000, "cleared 'Tags'");
        s.duel_wins = 2; s.flawless = 5;
        s
    }

    #[test]
    fn round_trip_preserves_everything() {
        let s = sample();
        assert_eq!(Save::parse(&s.serialize()), s);
    }

    #[test]
    fn legacy_v1_save_loads_and_counts_as_onboarded() {
        let legacy = "name=Old Hero\ncountry=Pixel Realm\nxp=399\ngems=65\nhearts=5\nstreak=3\nstreak_max=4\ndone=0.0.0\nbest=0.0.0|100|1\ncode=T\u{1f}python\u{1f}print(1)\u{1e}print(2)\n";
        let s = Save::parse(legacy);
        assert_eq!(s.name, "Old Hero");
        assert_eq!(s.xp, 399);
        assert!(s.onboarded);
        assert!(s.done.contains("0.0.0"));
        assert_eq!(s.codes[0].2, "print(1)\nprint(2)");
        assert_eq!(s.version, SAVE_VERSION);
    }

    #[test]
    fn empty_legacy_save_is_not_onboarded() {
        assert!(!Save::parse("gems=20\n").onboarded);
        assert!(!Save::parse("").onboarded);
    }

    #[test]
    fn garbage_is_ignored_and_values_clamped() {
        let s = Save::parse("\u{0}\u{1}junk\nxp=abc\nhearts=99\nleague=40\ngoal=5\nsavers=10\n=\nqdone=nope\nbest=a|b\nlog=x|y\nweek_mask=255\n");
        assert_eq!(s.xp, 0);
        assert_eq!(s.hearts, MAX_HEARTS);
        assert_eq!(s.league, 6);
        assert_eq!(s.goal, 7);
        assert_eq!(s.savers, MAX_SAVERS);
        assert!(s.log.is_empty());
        assert_eq!(s.week_mask, 0x7f);
    }

    #[test]
    fn crlf_line_endings_parse() {
        let s = Save::parse("version=2\r\nonboarded=1\r\nxp=10\r\n");
        assert!(s.onboarded);
        assert_eq!(s.xp, 10);
    }

    #[test]
    fn log_is_capped_and_single_line() {
        let mut s = Save::new();
        for i in 0..150 { s.add_log(i, format!("entry\n{}", i)); }
        assert_eq!(s.log.len(), MAX_LOG);
        assert_eq!(s.log[0].1, "entry 50");
        s.add_log(1, "   ");
        assert_eq!(s.log.len(), MAX_LOG);
    }

    #[test]
    fn names_are_cleaned() {
        assert_eq!(Save::clean_name("   "), "Hero");
        assert_eq!(Save::clean_name("a\nb"), "ab");
        assert_eq!(Save::clean_name("abcdefghijklmnopqrstuvwxyz").chars().count(), NAME_MAX);
    }

    #[test]
    fn atomic_store_keeps_backup_and_loads_back() {
        let dir = std::env::temp_dir().join(format!("pixel_learn_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let p = dir.join("save.txt");
        let mut s = sample();
        s.store_to(&p).unwrap();
        s.xp = 999;
        s.store_to(&p).unwrap();
        assert_eq!(Save::load_from(&p).xp, 999);
        assert_eq!(Save::load_from(&p.with_extension("bak")).xp, 420);
        assert!(!p.with_extension("tmp").exists());
        // main file lost -> fall back to the backup
        std::fs::remove_file(&p).unwrap();
        assert_eq!(Save::load_from(&p).xp, 420);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn missing_file_gives_fresh_save() {
        let s = Save::load_from(Path::new("Z:/definitely/not/here/save.txt"));
        assert_eq!(s, Save::new());
    }
}
