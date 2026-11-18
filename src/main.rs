#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//! Code Forge: learn to code in a pixel-art RPG. Offline, single player.
mod art;
mod audio;
mod content;
mod css;
mod data;
mod game;
mod interp;
mod lesson;
mod onboarding;
mod platform;
mod preview;
mod save;
mod screens;
mod sql;
mod tabs;

use art::*;
use preview::*;
use lesson::summary_card;
use data::*;
use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, RichText, Sense, Stroke, Vec2};
use game::{HINT_COST, LEAGUES, REFILL_COST, SAVER_COST, XP_PER_LEVEL};
use save::{Save, MAX_HEARTS};
use std::collections::VecDeque;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const HDR: Color32 = Color32::from_rgb(0x0b, 0x12, 0x28);

fn now() -> u64 { platform::now_secs() }
/// The furthest day already seen in this save: turning the clock back can't replay a day.
static DAY_FLOOR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// Seconds since 1970 in the player's own time zone, so days change at local midnight.
fn local_secs() -> u64 {
    let off = chrono::Local::now().offset().local_minus_utc() as i64;
    (now() as i64 + off).max(0) as u64
}
fn today() -> u64 { game::day_of(local_secs()).max(DAY_FLOOR.load(std::sync::atomic::Ordering::Relaxed)) }
/// How far through today we are (0..1).
fn day_frac() -> f32 { if game::day_of(local_secs()) < today() { 1.0 } else { (local_secs() % 86_400) as f32 / 86_400.0 } }
fn set_day_floor(s: &Save) {
    let d = s.seen_day.max(s.last_day).max(s.daily_day).max(s.rep_day).max(s.duel_day);
    DAY_FLOOR.fetch_max(d, std::sync::atomic::Ordering::Relaxed);
}

// ---------------- state ----------------
#[derive(Clone, Copy, PartialEq)]
enum Tab { Learn, Train, Ranks, Create, Hero }
#[derive(Clone, Copy, PartialEq)]
enum Modal { None, Notif, Shop, Courses, NoHearts, Reset, Avatar, DeleteSpell(usize), WeekResult, Import, CertName(usize) }

#[derive(Clone)]
enum Kind { Lesson(usize, usize, usize), Coach, Duel(usize), Daily, Review(usize) }
impl Kind {
    /// Duels, the Daily Quest and reviews are practice: wrong answers never cost hearts.
    fn safe(&self) -> bool { matches!(self, Kind::Duel(_) | Kind::Daily | Kind::Review(_)) }
}

struct RunState {
    title: String, kind: Kind, lang: &'static str, slides: Vec<(&'static str, &'static str)>, qs: Vec<Q>, stage: usize, queue: Vec<usize>,
    sel: Option<usize>, text: String, fb: Option<(bool, String)>, mistakes: u32, right: u32, total: usize, last: Option<usize>, hint_q: Option<usize>, project: bool,
    /// Long explanation and key points shown after the slides (lessons only).
    explain: &'static str, summary: Vec<&'static str>,
    /// Output of the learner's correct code, computed once per answer.
    ran: Option<Vec<(interp::Kind, String)>>,
}
impl RunState {
    /// Learn pages: picture slides, then the full explanation, then the summary.
    fn pages(&self) -> usize { self.slides.len() + usize::from(!self.explain.is_empty()) + usize::from(!self.summary.is_empty()) }
}
struct Complete { title: String, xp: u32, gems: u32, perfect: bool, score: u32, best: u32, duel: Option<(u32, u32, String, bool)>, new_streak: bool, next: Option<(usize, usize, usize)>, project: bool, summary: Vec<&'static str>, cert: Option<usize> }
struct Editor { idx: Option<usize>, title: String, lang: String, code: String, out: Vec<(interp::Kind, String)>, /** answers for input() / prompt(), one per line */ stdin: String }

enum Screen { Main, Settings, Run(Box<RunState>), Complete(Complete), Editor(Editor), Preview(usize), Notes(usize, usize, usize), Certificate(usize) }

struct App {
    s: Save, tab: Tab, screen: Screen, modal: Modal, courses: Vec<Course>, cur: usize, open_mod: Option<usize>,
    opp: usize, sub_train: usize, sub_create: usize, input: String, cat: usize, query: String,
    view_key: u32, view_t0: f64, toasts: VecDeque<(String, f64)>, demo: bool, shot: Option<String>, frames: u32, save_warned: bool,
    onboard: Option<onboarding::Onboard>,
    /// Sound output (None in tests, screenshots, or without a sound device).
    audio: Option<audio::Audio>,
    /// A progress file waiting for the player to confirm the import.
    pending_import: Option<Box<Save>>,
    /// The name being typed for a certificate.
    cert_input: String,
    /// Android: whether we asked for the on-screen keyboard.
    #[cfg_attr(not(target_os = "android"), allow(dead_code))]
    kb_open: bool,
    #[cfg_attr(not(target_os = "android"), allow(dead_code))]
    kb_t: f64,
    /// The opening animation shown every time the app starts (after the first-run welcome):
    /// start time (set on the first frame) and which of its sounds already played.
    splash: Option<(f64, u8)>,
}

impl App {
    fn new(shot: Option<String>) -> Self {
        let demo = shot.is_some();
        let mut s = if demo { Save::new() } else { Save::load() };
        content::migrate(&mut s);
        if s.heart_t == 0 { s.heart_t = now(); }
        if demo {
            // In-memory demo profile for screenshots. Never written to disk.
            s.onboarded = true; s.name = "Ada".into(); s.avatar = 1; s.xp = 399; s.gems = 65; s.streak = 3; s.streak_max = 4; s.last_day = today();
            s.week_id = game::week_of(today()); s.week_xp = 120; s.hist = vec![150]; s.ghost = [30, 25, 0, 40, 20, 15, 20]; s.week_mask = 0b0000101; s.league = 2; s.hearts = 4;
            for k in ["html/your-first-tags", "html/headings", "html/paragraphs-and-breaks", "html/bold-italic-and-more"] { s.done.insert(k.into()); }
            s.ach.insert("first_lesson".into(), today() - 3); s.ach.insert("streak3".into(), today() - 1); s.ach.insert("xp100".into(), today() - 2);
            s.codes.push(("Torch Counter".into(), "python".into(), "for i in range(3):\n  print(\"Torch\", i + 1)\n".into()));
            s.add_log(today() - 1, "cleared 'Tags and Elements'");
            s.add_log(today(), "reached a 3 day streak");
        }
        let courses = courses();
        if !demo { set_day_floor(&s); }
        let tampered = s.tampered;
        App::validate(&mut s, &courses);
        let cur = (s.course as usize).min(courses.len() - 1);
        let onboard = if s.onboarded { None } else { Some(onboarding::Onboard::default()) };
        let mut a = App { s, tab: Tab::Learn, screen: Screen::Main, modal: Modal::None, courses, cur, open_mod: None, opp: 0, sub_train: 0, sub_create: 0,
            input: String::new(), cat: 0, query: String::new(), view_key: 999, view_t0: 0.0, toasts: VecDeque::new(), demo, shot, frames: 0, save_warned: false, onboard, audio: None, pending_import: None, cert_input: String::new(), kb_open: false, kb_t: 0.0, splash: None };
        if !demo && a.onboard.is_none() { a.splash = Some((-1.0, 0)); }
        if a.shot.is_none() && !cfg!(test) { a.audio = audio::Audio::new(); }
        if tampered {
            a.log("the save file was changed outside Code Forge; gems were reset");
            a.toast("Your save was edited outside Code Forge, so your gems were reset.");
            a.persist();
        }
        a.week_check();
        a.open_mod = a.current_module(a.cur);
        a
    }

    /// Drop anything in a save the game could not have produced: unknown lessons, badges and
    /// certificates for unfinished courses, progress for lessons that no longer exist.
    fn validate(s: &mut Save, courses: &[Course]) {
        let ids: std::collections::HashSet<String> = courses.iter().flat_map(|c| (0..c.modules.len()).flat_map(move |m| (0..c.modules[m].lessons.len()).map(move |l| c.lesson_id(m, l)))).collect();
        s.done.retain(|k| ids.contains(k));
        s.best.retain(|k, _| ids.contains(k));
        s.qdone.retain(|k, _| ids.contains(k));
        for c in courses {
            let all = (0..c.modules.len()).all(|m| (0..c.modules[m].lessons.len()).all(|l| s.done.contains(&c.lesson_id(m, l))));
            if !all { s.ach.remove(&c.badge_id()); s.cert_names.remove(c.slug); }
        }
        s.course = s.course.min(courses.len() as u32 - 1);
        s.codes.truncate(200);
    }

    /// Save to disk (atomic). Warns once if the save folder can't be written.
    fn persist(&mut self) {
        if self.demo { return; }
        self.s.seen_day = self.s.seen_day.max(today());
        if let Err(e) = self.s.store()
            && !self.save_warned { self.save_warned = true; self.toast(format!("Can't save progress: {}", e)); }
    }
    fn toast(&mut self, t: impl Into<String>) {
        let t = t.into();
        if self.toasts.iter().any(|(x, _)| *x == t) { return; }
        self.toasts.push_back((t, 2.8));
    }
    fn log(&mut self, t: impl Into<String>) { let d = today(); self.s.add_log(d, t); }

    fn week_check(&mut self) {
        if let Some(r) = game::week_roll(&mut self.s, today()) {
            let name = LEAGUES[self.s.league as usize];
            let msg = match r { game::LeagueResult::Up => format!("Promoted to the {} league!", name), game::LeagueResult::Down => format!("Dropped to the {} league", name), game::LeagueResult::Stay => format!("Still in the {} league", name) };
            self.log(msg.clone());
            self.toast(msg);
            self.s.notif_read = false;
            self.award();
            self.persist();
        }
    }
    /// Save the progress to a file the player keeps (a download on the web).
    fn export_progress(&mut self) {
        let name: String = self.s.name.chars().filter(|c| c.is_alphanumeric()).collect();
        let file = format!("CodeForge-{}-{}.codeforge", if name.is_empty() { "Hero".into() } else { name }, preview::ymd(today()));
        match platform::export(&file, &self.s.export_text()) {
            Ok(msg) => { self.toast(msg); audio::play(audio::Sfx::Coin); }
            Err(e) if !e.is_empty() => self.toast(e),
            Err(_) => {} // cancelled
        }
    }

    /// Check a loaded progress file and ask before replacing the current hero.
    fn offer_import(&mut self, text: Result<String, String>) {
        match text.and_then(|t| Save::import_text(&t).map_err(|e| e.to_string())) {
            Ok(s) => { self.pending_import = Some(Box::new(s)); self.modal = Modal::Import; }
            Err(e) => self.toast(e),
        }
    }

    fn apply_import(&mut self) {
        let Some(mut s) = self.pending_import.take() else { return };
        // keep a copy of what is being replaced, just in case
        if !self.demo { self.s.backup_before_import(); }
        content::migrate(&mut s);
        App::validate(&mut s, &self.courses);
        if s.heart_t == 0 { s.heart_t = now(); }
        set_day_floor(&s);
        self.s = *s;
        self.onboard = None;
        self.cur = (self.s.course as usize).min(self.courses.len() - 1);
        self.open_mod = self.current_module(self.cur);
        self.screen = Screen::Main;
        self.week_check();
        self.persist();
        self.toast(format!("Welcome back, {}! Your progress is here.", self.s.name));
        audio::play(audio::Sfx::LevelUp);
    }

    fn courses_cleared(&self) -> usize { (0..self.courses.len()).filter(|c| self.progress(*c) >= 1.0).count() }
    /// Grant newly earned achievements, with a toast and log entry for each.
    fn award(&mut self) {
        for c in 0..self.courses.len() {
            let id = self.courses[c].badge_id();
            if self.progress(c) >= 1.0 && !self.s.ach.contains_key(&id) {
                self.s.ach.insert(id, today());
                let name = self.courses[c].badge.name;
                self.toast(format!("Badge earned: {}!", name));
                self.log(format!("earned the {} badge for {}", name, self.courses[c].name));
                self.s.notif_read = false;
            }
        }
        let ctx = game::Ctx { courses_cleared: self.courses_cleared() };
        for a in game::grant_achievements(&mut self.s, &ctx, today()) {
            self.toast(format!("Achievement: {}", a.title));
            self.log(format!("earned the '{}' achievement", a.title));
            self.s.notif_read = false;
        }
    }

    fn level(&self) -> u32 { game::level(self.s.xp) }

    /// Play this frame's sounds and keep the right music on: the title theme while onboarding,
    /// the battle theme in duels, the dungeon theme everywhere else.
    /// Returns true while the music is fading (the caller keeps repainting).
    fn update_audio(&mut self) -> bool {
        let sounds = audio::take();
        #[cfg(target_arch = "wasm32")]
        if let Some(a) = &mut self.audio && !sounds.is_empty() { a.unlock(); }
        let want = if self.onboard.is_some() || self.splash.is_some() { audio::Track::Title } else {
            match &self.screen {
                Screen::Run(rs) if matches!(rs.kind, Kind::Duel(_)) => audio::Track::Battle,
                Screen::Complete(c) if c.duel.is_some() => audio::Track::Battle,
                // learning: lessons, practice runs, course pages, notes, certificates and the code editor
                Screen::Run(_) | Screen::Complete(_) | Screen::Preview(_) | Screen::Notes(..) | Screen::Certificate(_) | Screen::Editor(_) => audio::Track::Study,
                _ => audio::Track::Hub,
            }
        };
        let (mv, sv) = (self.s.music as f32 / 100.0, self.s.sfx as f32 / 100.0);
        if let Some(a) = &mut self.audio {
            for s in sounds { a.sfx(s, sv); }
            return a.music(Some(want), mv);
        }
        false
    }
    fn lkey(&self, c: usize, m: usize, l: usize) -> String { self.courses[c].lesson_id(m, l) }
    fn is_done(&self, c: usize, m: usize, l: usize) -> bool { self.s.done.contains(&self.lkey(c, m, l)) }
    fn module_done(&self, c: usize, m: usize) -> bool { (0..self.courses[c].modules[m].lessons.len()).all(|l| self.is_done(c, m, l)) }
    fn current_module(&self, c: usize) -> Option<usize> { (0..self.courses[c].modules.len()).find(|m| !self.module_done(c, *m)) }
    fn lesson_unlocked(&self, c: usize, m: usize, l: usize) -> bool {
        (0..m).all(|mm| self.module_done(c, mm)) && (0..l).all(|ll| self.is_done(c, m, ll))
    }
    fn next_lesson(&self, c: usize) -> Option<(usize, usize)> {
        for mi in 0..self.courses[c].modules.len() { for li in 0..self.courses[c].modules[mi].lessons.len() { if !self.is_done(c, mi, li) { return Some((mi, li)); } } }
        None
    }
    fn progress(&self, c: usize) -> f32 {
        let total = self.courses[c].lesson_count();
        let done = (0..self.courses[c].modules.len()).map(|m| (0..self.courses[c].modules[m].lessons.len()).filter(|l| self.is_done(c, m, *l)).count()).sum::<usize>();
        done as f32 / total.max(1) as f32
    }
    fn select_course(&mut self, c: usize) {
        self.cur = c; self.s.course = c as u32; self.open_mod = self.current_module(c).or(Some(0)); self.tab = Tab::Learn; self.persist();
    }
    fn tick(&mut self) {
        let before = self.s.hearts;
        game::tick_hearts(&mut self.s, now());
        if self.s.hearts != before { self.persist(); }
    }

    fn open_lesson(&mut self, c: usize, m: usize, l: usize) {
        let ls = self.courses[c].modules[m].lessons[l].clone();
        self.start_run(ls.title, Kind::Lesson(c, m, l), ls.slides, ls.qs);
    }

    fn start_run(&mut self, title: &str, kind: Kind, slides: Vec<(&'static str, &'static str)>, qs: Vec<Q>) {
        self.tick();
        if qs.is_empty() { self.toast("Nothing to practise here yet. Clear a lesson first!"); return; }
        if self.s.hearts == 0 && !kind.safe() { self.modal = Modal::NoHearts; return; }
        audio::play(match kind { Kind::Duel(_) => audio::Sfx::DuelStart, Kind::Daily => audio::Sfx::Chest, _ => audio::Sfx::Confirm });
        let n = qs.len();
        let (mut stage, mut queue, mut right) = (0usize, (0..n).collect::<Vec<usize>>(), 0u32);
        if let Kind::Lesson(c, mi, l) = &kind {
            let k = self.lkey(*c, *mi, *l);
            // An unfinished lesson resumes at the first question not yet answered correctly.
            if !self.s.done.contains(&k)
                && let Some(d) = self.s.qdone.get(&k)
                    && !d.is_empty() { let d: Vec<usize> = d.iter().copied().filter(|i| *i < n).collect(); queue.retain(|i| !d.contains(i)); right = d.len() as u32; if !d.is_empty() { stage = usize::MAX; } }
        }
        let lang = match &kind { Kind::Lesson(c, _, _) => self.courses[*c].lang, _ => "" };
        let (project, explain, summary) = match kind {
            Kind::Lesson(c, m, l) => { let ls = &self.courses[c].modules[m].lessons[l]; (ls.project, ls.explain, ls.summary.clone()) }
            _ => (false, "", vec![]),
        };
        let mut rs = RunState { title: title.into(), lang, kind, slides, qs, stage, queue, sel: None, text: String::new(), fb: None, mistakes: 0, right, total: n, last: None, hint_q: None, project, explain, summary, ran: None };
        rs.stage = rs.stage.min(rs.pages());
        self.screen = Screen::Run(Box::new(rs));
    }

    /// Questions for practice: from lessons you cleared in `courses` (or the first lessons when none are
    /// cleared yet), shuffled with `seed`. Typing steps are skipped so practice stays quick.
    fn practice_set(&self, courses: &[usize], n: usize, seed: u64) -> Vec<Q> {
        let mut pool = vec![];
        let mut fallback = vec![];
        for &c in courses {
            for (mi, m) in self.courses[c].modules.iter().enumerate() {
                for (li, l) in m.lessons.iter().enumerate() {
                    let qs = l.qs.iter().filter(|q| !matches!(q, Q::Type { .. })).cloned();
                    if self.is_done(c, mi, li) { pool.extend(qs); } else if mi == 0 && li < 2 { fallback.extend(qs); }
                }
            }
        }
        if pool.len() < n { pool.extend(fallback); }
        let mut x = seed | 1;
        for i in (1..pool.len()).rev() { x ^= x << 13; x ^= x >> 7; x ^= x << 17; pool.swap(i, (x % (i as u64 + 1)) as usize); }
        pool.truncate(n);
        pool
    }

    /// Courses with at least one cleared lesson (the current course when none).
    fn started_courses(&self) -> Vec<usize> {
        let v: Vec<usize> = (0..self.courses.len()).filter(|c| self.progress(*c) > 0.0).collect();
        if v.is_empty() { vec![self.cur] } else { v }
    }

    fn finish(&mut self, rs: &RunState) {
        self.week_check();
        let (mut xp, gems);
        let (mut duel, mut next, mut cert) = (None, None, None);
        let perfect = rs.mistakes == 0;
        let score = if rs.kind.safe() { (rs.right * 100).checked_div(rs.total as u32).unwrap_or(0) } else { game::score(rs.total as u32, rs.mistakes) };
        let day = today();
        match rs.kind {
            Kind::Lesson(c, m, l) => {
                let first = !self.is_done(c, m, l);
                self.s.qdone.remove(&self.lkey(c, m, l));
                (xp, gems) = game::lesson_reward(first, perfect);
                if !first { xp = game::repeat_xp(&mut self.s, day, xp); }
                if first && self.courses[c].modules[m].lessons[l].project { xp += game::PROJECT_BONUS; }
                if first {
                    self.s.done.insert(self.lkey(c, m, l));
                    self.log(format!("cleared '{}' ({})", rs.title, self.courses[c].name));
                    if perfect { self.s.flawless += 1; }
                    if self.progress(c) >= 1.0 { self.log(format!("finished the {} course!", self.courses[c].name)); cert = Some(c); }
                }
                self.open_mod = self.current_module(c).or(Some(m));
                next = self.next_lesson(c).map(|(m, l)| (c, m, l));
            }
            Kind::Coach => {
                let first = self.s.solved.insert(rs.title.clone());
                (xp, gems) = if first { (8, 2) } else { (game::repeat_xp(&mut self.s, day, 4), 0) };
                self.log(format!("solved the '{}' challenge", rs.title));
            }
            Kind::Daily => {
                let t = today();
                if self.s.daily_day != t {
                    self.s.daily_streak = if self.s.daily_day + 1 == t { self.s.daily_streak + 1 } else { 1 };
                    self.s.daily_day = t;
                    (xp, gems) = (15, 5);
                    self.log(format!("cleared the Daily Quest ({} days in a row)", self.s.daily_streak));
                } else { (xp, gems) = (game::repeat_xp(&mut self.s, day, 5), 0); }
            }
            Kind::Review(c) => {
                self.s.reviews += 1;
                // a review pays XP only for real effort, from the daily repeat budget
                (xp, gems) = (game::repeat_xp(&mut self.s, day, if score >= 50 { 6 } else { 2 }), 0);
                self.log(format!("reviewed {}", self.courses[c].name));
            }
            Kind::Duel(o) => {
                let me = rs.right;
                let bot = opponents()[o].skill.min(rs.total as u32);
                let won = me > bot || (me == bot && me as usize == rs.total);
                if won {
                    self.s.duel_wins += 1;
                    (xp, gems) = if game::duel_reward_left(&mut self.s, day) { (20, 5) } else { (game::repeat_xp(&mut self.s, day, 4), 0) };
                } else { (xp, gems) = (game::repeat_xp(&mut self.s, day, 3), 0); self.s.duel_losses += 1; }
                duel = Some((me, bot, opponents()[o].name.to_string(), won));
                self.log(format!("{} a duel against {}", if won { "won" } else { "lost" }, opponents()[o].name));
            }
        }
        let mut best = score;
        if let Kind::Lesson(c, mi, l) = rs.kind {
            let e = self.s.best.entry(self.lkey(c, mi, l)).or_insert((0, 0));
            e.0 = e.0.max(score); e.1 += 1; best = e.0;
        }
        let new_streak = game::bump_streak(&mut self.s, today());
        let lvl = self.level();
        game::add_xp(&mut self.s, xp, gems, day);
        audio::play(match &duel { Some((_, _, _, true)) => audio::Sfx::Victory, Some(_) => audio::Sfx::Defeat, None if self.level() > lvl => audio::Sfx::LevelUp, None => audio::Sfx::Complete });
        self.award();
        self.persist();
        self.screen = Screen::Complete(Complete { title: rs.title.clone(), xp, gems, perfect, score, best, duel, new_streak, next, project: rs.project, summary: rs.summary.clone(), cert });
    }
}

// ---------------- shared ui helpers ----------------
fn chip(ui: &mut egui::Ui, art: &str, dim: bool, text: &str) -> egui::Response {
    let (r, resp) = ui.allocate_exact_size(Vec2::new(44.0 + text.len() as f32 * 12.0, 34.0), Sense::click());
    let p = ui.painter();
    bevel(p, r, if resp.hovered() { PANEL_HI } else { Color32::from_rgb(0x1a, 0x18, 0x2a) }, false);
    draw_icon(p, Rect::from_min_size(Pos2::new(r.min.x + 4.0, r.min.y + 2.0), Vec2::splat(30.0)), art, if dim { Color32::from_gray(80) } else { Color32::WHITE });
    p.text(Pos2::new(r.min.x + 38.0, r.center().y + 1.0), Align2::LEFT_CENTER, text, FontId::new(11.0, press()), INK);
    resp
}

fn stats_row(ui: &mut egui::Ui, app: &mut App, show_hearts: bool) {
    let streak = game::live_streak(&app.s, today());
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        if show_hearts {
            let r = chip(ui, "heart", app.s.hearts == 0, &app.s.hearts.to_string());
            let tip = match game::next_heart_in(&app.s, now()) { Some(sec) => format!("Next heart in {}:{:02}", sec / 60, sec % 60), None => "Hearts are full".into() };
            if r.on_hover_text(tip).clicked() { app.modal = Modal::Shop; }
        }
        let st = chip(ui, "flame", streak == 0, &streak.to_string()).on_hover_text("Day streak");
        if st.clicked() { app.tab = Tab::Hero; }
        if chip(ui, "crystal_purple", false, &app.s.gems.to_string()).on_hover_text("Gems").clicked() { app.modal = Modal::Shop; }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.available_width() > 70.0 && pbtn(ui, "SHOP", GOLD, 80.0).clicked() { app.modal = Modal::Shop; }
        });
    });
}

fn draw_avatar(p: &egui::Painter, r: Rect, idx: u32) {
    let a = avatar(idx);
    bevel(p, r, a.bg, false);
    draw_icon(p, r.shrink(r.width() * 0.14), a.art, Color32::WHITE);
}

fn header(ctx: &mut egui::Ui, app: &mut App, title_text: &str, back: bool, dropdown: bool) {
    egui::Panel::top("hdr").frame(egui::Frame::new().fill(HDR).inner_margin(egui::Margin { left: 12, right: 12, top: 12, bottom: 12 })).show(ctx, |ui| {
        ledge(ui, true);
        let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 30.0), Sense::hover());
        let mut x = r.min.x;
        if back {
            let br = Rect::from_min_size(Pos2::new(x, r.min.y), Vec2::new(30.0, 30.0));
            let resp = ui.interact(br, ui.scope_id().with("back"), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
            ui.painter().text(br.center(), Align2::CENTER_CENTER, "<", FontId::new(16.0, press()), if resp.hovered() { GOLD } else { Color32::WHITE });
            if resp.clicked() { app.screen = Screen::Main; }
            x += 36.0;
        }
        let galley = ui.painter().layout_no_wrap(title_text.to_string(), FontId::new(12.0, press()), Color32::WHITE);
        let tw = galley.size().x.min(r.width() - 130.0);
        let th = galley.size().y;
        let tr = Rect::from_min_size(Pos2::new(x, r.min.y), Vec2::new(tw + if dropdown { 28.0 } else { 0.0 }, 30.0));
        ui.painter().with_clip_rect(Rect::from_min_size(Pos2::new(x, r.min.y), Vec2::new(tw, 30.0))).galley(Pos2::new(x, r.center().y - th / 2.0), galley, Color32::WHITE);
        if dropdown {
            let c = Pos2::new(x + tw + 14.0, r.center().y);
            for i in 0..4 { ui.painter().rect_filled(Rect::from_min_size(c + Vec2::new(-8.0 + i as f32 * 2.0, -3.0 + i as f32 * 2.0), Vec2::new(16.0 - i as f32 * 4.0, 2.0)), 0.0, Color32::WHITE); }
            if ui.interact(tr, ui.scope_id().with("drop"), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Choose a course").clicked() { app.modal = Modal::Courses; }
        }
        let mr = Rect::from_min_size(Pos2::new(r.max.x - 24.0, r.min.y), Vec2::new(24.0, 30.0));
        let mresp = ui.interact(mr, ui.scope_id().with("menu"), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Settings");
        for i in 0..3 { ui.painter().rect_filled(Rect::from_min_size(Pos2::new(mr.center().x - 2.0, mr.min.y + 5.0 + i as f32 * 8.0), Vec2::splat(5.0)), 0.0, if mresp.hovered() { GOLD } else { Color32::WHITE }); }
        if mresp.clicked() { app.screen = Screen::Settings; }
        let br = Rect::from_min_size(Pos2::new(r.max.x - 72.0, r.min.y), Vec2::new(36.0, 30.0));
        draw_icon(ui.painter(), br, "letter", Color32::WHITE);
        if !app.s.notif_read {
            ui.painter().rect_filled(Rect::from_min_size(Pos2::new(br.max.x - 8.0, br.min.y - 3.0), Vec2::splat(11.0)), 0.0, RED);
        }
        if ui.interact(br, ui.scope_id().with("bell"), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Messages").clicked() { app.modal = Modal::Notif; app.s.notif_read = true; app.persist(); }
    });
}

fn subtabs(ui: &mut egui::Ui, labels: &[&str], cur: &mut usize) {
    let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 40.0), Sense::hover());
    ui.painter().add(frame_shapes(r, Color32::from_rgb(0x16, 0x14, 0x22), EDGE));
    let n = labels.len() as f32;
    for (i, lab) in labels.iter().enumerate() {
        let tr = Rect::from_min_size(Pos2::new(r.min.x + r.width() / n * i as f32, r.min.y), Vec2::new(r.width() / n, 38.0));
        let resp = ui.interact(tr, ui.scope_id().with(("st", labels[0], i)), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
        let on = *cur == i;
        ui.painter().text(tr.center() - Vec2::new(0.0, 2.0), Align2::CENTER_CENTER, *lab, FontId::new(9.0, press()), if on { Color32::WHITE } else if resp.hovered() { INK } else { DIM });
        if on { ui.painter().rect_filled(Rect::from_min_size(Pos2::new(tr.min.x, tr.max.y - 4.0), Vec2::new(tr.width(), 4.0)), 0.0, GOLD); }
        if resp.clicked() { if *cur != i { audio::play(audio::Sfx::Page); } *cur = i; }
    }
    ui.add_space(SP_S);
}

fn nav(ctx: &mut egui::Ui, app: &mut App) {
    egui::Panel::bottom("nav").frame(egui::Frame::new().fill(Color32::from_rgb(0x1d, 0x16, 0x32)).inner_margin(egui::Margin { left: 4, right: 4, top: 6, bottom: 6 })).show(ctx, |ui| {
        ledge(ui, false);
        let (full, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 64.0), Sense::hover());
        let r = Rect::from_center_size(full.center(), Vec2::new(full.width().min(MAX_W), 64.0));
        let items = [(Tab::Learn, "LEARN", "book"), (Tab::Train, "TRAIN", "sword"), (Tab::Ranks, "RANKS", "banner_royal"), (Tab::Create, "CREATE", "staff"), (Tab::Hero, "HERO", "helmet")];
        let w = r.width() / 5.0;
        for (i, (t, name, art)) in items.iter().enumerate() {
            let tr = Rect::from_min_size(Pos2::new(r.min.x + w * i as f32, r.min.y), Vec2::new(w, 64.0));
            let resp = ui.interact(tr, ui.scope_id().with(("nav", i)), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
            let on = app.tab == *t;
            if on {
                ui.painter().add(frame_shapes(tr.shrink2(Vec2::new(3.0, 2.0)), Color32::from_rgb(0x2a, 0x20, 0x18), GOLD));
            } else if resp.hovered() {
                ui.painter().rect_filled(tr.shrink2(Vec2::new(4.0, 2.0)), 2.0, Color32::from_white_alpha(14));
            }
            // icon and label sit inside the frame's 10px border
            draw_icon(ui.painter(), Rect::from_center_size(Pos2::new(tr.center().x, tr.min.y + 24.0), Vec2::splat(26.0)), art, if on || resp.hovered() { Color32::WHITE } else { Color32::from_gray(150) });
            ui.painter().text(Pos2::new(tr.center().x, tr.max.y - 18.0), Align2::CENTER_CENTER, *name, FontId::new(7.0, press()), if on { GOLD } else { DIM });
            if resp.clicked() { if app.tab != *t { audio::play(audio::Sfx::Nav); } app.tab = *t; }
        }
    });
}

fn c32(c: [u8; 3]) -> Color32 { Color32::from_rgb(c[0], c[1], c[2]) }

fn out_view(ui: &mut egui::Ui, out: &[(interp::Kind, String)]) {
    use interp::Kind::*;
    for (i, (k, t)) in out.iter().enumerate() {
        match k {
            Out => { ui.label(RichText::new(format!("> {}", t)).monospace().size(14.0).color(GREEN)); }
            Err => { ui.label(RichText::new(format!("! {}", t)).monospace().size(13.0).color(RED)); }
            H1 => { ui.label(title(t.clone(), 15.0).color(PARCH)); ui.add_space(SP_XS); }
            H2 => { ui.label(title(t.clone(), 11.0).color(PARCH)); ui.add_space(3.0); }
            P => { ui.label(body(t.clone(), 15.0)); }
            Li => { ui.label(body(format!("  * {}", t), 15.0)); }
            Btn => { let _ = pbtn(ui, t, BLUE, 180.0); }
            Page => {
                ui.horizontal_wrapped(|ui| {
                    let (r, _) = ui.allocate_exact_size(Vec2::new(46.0, 18.0), Sense::hover());
                    ui.painter().rect_filled(r, 0.0, TEAL);
                    ui.painter().text(r.center(), Align2::CENTER_CENTER, "PAGE", FontId::new(8.0, press()), EDGE);
                    ui.label(RichText::new(t).monospace().size(14.0).color(TEAL));
                });
            }
            Img => {
                ui.horizontal(|ui| { icon(ui, "map", 34.0); ui.label(body(format!("[picture: {}]", t), 14.0).color(DIM)); });
            }
            Table => {
                let rows: Vec<Vec<&str>> = t.lines().map(|l| l.split('\u{1f}').collect()).collect();
                egui::ScrollArea::horizontal().id_salt(("tbl", i)).show(ui, |ui| {
                    egui::Grid::new(("grid", i)).striped(true).spacing([18.0, 4.0]).show(ui, |ui| {
                        for (ri, r) in rows.iter().enumerate() {
                            for c in r { ui.label(RichText::new(*c).monospace().size(14.0).color(if ri == 0 { GOLD } else { PARCH })); }
                            ui.end_row();
                        }
                    });
                });
            }
            Styled => {
                let (st, text) = css::Style::decode(t);
                let size = st.size.unwrap_or(15.0);
                let pad = st.pad.unwrap_or(0.0);
                let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), size + 18.0 + pad * 2.0), Sense::hover());
                ui.painter().rect_filled(r, 0.0, Color32::WHITE);
                let inner = r.shrink(6.0);
                ui.painter().rect_filled(inner, 0.0, st.bg.map(c32).unwrap_or(Color32::WHITE));
                if let Some((w, c)) = st.border { ui.painter().rect_stroke(inner, 0.0, Stroke::new(w, c32(c)), egui::StrokeKind::Inside); }
                let tr = inner.shrink(pad + 4.0);
                let (pos, al) = match st.align.as_deref() { Some("center") => (tr.center(), Align2::CENTER_CENTER), Some("right") => (Pos2::new(tr.max.x, tr.center().y), Align2::RIGHT_CENTER), _ => (Pos2::new(tr.min.x, tr.center().y), Align2::LEFT_CENTER) };
                let col = st.color.map(c32).unwrap_or(Color32::BLACK);
                let p = ui.painter().with_clip_rect(inner);
                let font = FontId::new(size, if st.italic { egui::FontFamily::Monospace } else { egui::FontFamily::Proportional });
                let rect = p.text(pos, al, &text, font.clone(), col);
                if st.bold { p.text(pos + Vec2::new(1.0, 0.0), al, &text, font, col); }
                if st.underline { p.rect_filled(Rect::from_min_size(Pos2::new(rect.min.x, rect.max.y - 1.0), Vec2::new(rect.width(), 2.0)), 0.0, col); }
                ui.painter().rect_stroke(r, 0.0, Stroke::new(2.0, EDGE), egui::StrokeKind::Inside);
                ui.add_space(SP_XS);
            }
        }
    }
}

fn dim_back(ctx: &egui::Context) {
    let r = ctx.content_rect();
    egui::Area::new(egui::Id::unique("dimmer")).order(egui::Order::Middle).fixed_pos(r.min).show(ctx, |ui| {
        ui.painter().rect_filled(r, 0.0, Color32::from_black_alpha(190));
        let _ = ui.allocate_rect(r, Sense::click());
    });
}

fn dialog(ctx: &egui::Context, id: &str, add: impl FnOnce(&mut egui::Ui)) {
    let w = (ctx.content_rect().width() - 40.0).clamp(260.0, 360.0);
    egui::Window::new(id).title_bar(false).collapsible(false).resizable(false).anchor(Align2::CENTER_CENTER, [0.0, 0.0]).order(egui::Order::Foreground)
        .min_width(w).max_width(w)
        .frame(egui::Frame::new().fill(PANEL).stroke(Stroke::new(4.0, GOLD)).inner_margin(16.0).shadow(egui::epaint::Shadow { offset: [0, 6], blur: 0, spread: 0, color: EDGE }))
        .show(ctx, |ui| {
            ui.set_width(w);
            egui::ScrollArea::vertical().auto_shrink([false, true]).max_height((ctx.content_rect().height() - 90.0).max(200.0)).show(ui, add);
        });
}

fn option_btn(ui: &mut egui::Ui, text: &str, selected: bool, locked: bool, sel: &mut Option<usize>, i: usize) {
    let (r, resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 48.0), Sense::click());
    let fill = if selected { Color32::from_rgb(0x3a, 0x5a, 0xb8) } else if resp.hovered() && !locked { PANEL_HI } else { PANEL };
    bevel(ui.painter(), r, fill, true);
    if selected { ui.painter().rect_stroke(r.expand(2.0), 0.0, Stroke::new(3.0, GOLD), egui::StrokeKind::Outside); }
    ui.painter().text(Pos2::new(r.min.x + 12.0, r.center().y), Align2::LEFT_CENTER, format!("{}", i + 1), FontId::new(9.0, press()), DIM);
    ui.painter().with_clip_rect(r).text(Pos2::new(r.min.x + 32.0, r.center().y), Align2::LEFT_CENTER, text, FontId::new(16.0, egui::FontFamily::Proportional), INK);
    if resp.clicked() && !locked { *sel = Some(i); audio::play(audio::Sfx::Select); }
    ui.add_space(SP_S);
}

fn toggle(ui: &mut egui::Ui, name: &str, sub: &str, v: &mut bool) -> bool {
    let mut ch = false;
    ui.horizontal(|ui| {
        ui.vertical(|ui| { ui.label(body(name, 15.0)); ui.label(body(sub, 12.0).color(DIM)); });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let (r, resp) = ui.allocate_exact_size(Vec2::new(60.0, 28.0), Sense::click());
            bevel(ui.painter(), r, if *v { GREEN } else { Color32::from_rgb(0x4a, 0x44, 0x66) }, false);
            let kx = if *v { r.max.x - 22.0 } else { r.min.x + 4.0 };
            ui.painter().rect_filled(Rect::from_min_size(Pos2::new(kx, r.min.y + 4.0), Vec2::new(18.0, 20.0)), 0.0, Color32::WHITE);
            if resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() { *v = !*v; ch = true; }
        });
    });
    ui.add_space(SP_S);
    ch
}

// ---------------- app glue ----------------
impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _f: &mut eframe::Frame) { self.frame_ui(ui); }
}

impl App {
    /// One frame of the whole app. Separate from eframe so tests can drive it headlessly.
    fn frame_ui(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        if self.update_audio() { ctx.request_repaint(); }
        // the phone's Back button (and a mouse's back button): close, leave, or return to Learn
        if ctx.input(|i| i.key_pressed(egui::Key::BrowserBack)) && self.onboard.is_none() {
            if self.modal != Modal::None { self.modal = Modal::None; }
            else { match &self.screen {
                Screen::Main => self.tab = Tab::Learn,
                Screen::Notes(c, _, _) | Screen::Certificate(c) => { let c = *c; self.screen = Screen::Preview(c); }
                _ => self.screen = Screen::Main,
            } }
            audio::play(audio::Sfx::Back);
        }
        #[cfg(target_os = "android")]
        {
            // open the phone keyboard while a text box has focus, close it otherwise
            let want = ctx.egui_wants_keyboard_input();
            let now_t = ctx.input(|i| i.time);
            let ppp = ctx.pixels_per_point();
            let h = ctx.content_rect().height();
            // how much of the screen the keyboard really covers right now (None = keyboard closed)
            let inset = platform::keyboard_inset_px(h * ppp).map(|px| px / ppp);
            if want != self.kb_open {
                self.kb_open = want;
                if !want || inset.is_none() { platform::soft_keyboard(want); }
                self.kb_t = now_t;
            } else if want && inset.is_none() && now_t - self.kb_t > 0.8 && ctx.input(|i| i.pointer.any_released()) {
                // the player closed the keyboard and tapped the text box again: bring it back
                platform::soft_keyboard(true);
                self.kb_t = now_t;
            }
            // lift the bottom buttons (NEXT, CHECK...) above the keyboard only while it is open
            match inset.filter(|_| want) {
                Some(px) => { egui::Panel::bottom("keyboard_space").exact_size(px).resizable(false).frame(egui::Frame::new().fill(BG)).show(ui, |_| {}); }
                None => { egui::Panel::bottom("gesture_space").exact_size(14.0).resizable(false).frame(egui::Frame::new().fill(BG2)).show(ui, |_| {}); }
            }
        }
        if self.onboard.is_none() && self.modal == Modal::None && matches!(self.screen, Screen::Main) && !self.s.res_seen && self.s.res_place > 0 {
            self.modal = Modal::WeekResult;
            audio::play(if self.s.res_to > self.s.res_from { audio::Sfx::Victory } else if self.s.res_to < self.s.res_from { audio::Sfx::Defeat } else { audio::Sfx::Complete });
        }
        let dropped = ctx.input(|i| i.raw.dropped_files.first().cloned());
        if let Some(f) = dropped { platform::dropped(f.as_ref()); }
        if let Some(t) = platform::take_import() { self.offer_import(t); }
        // text size setting (applies from the next frame)
        let z = self.s.zoom.clamp(100, 130) as f32 / 100.0;
        if (ctx.zoom_factor() - z).abs() > 0.01 { ctx.set_zoom_factor(z); }
        self.tick();
        if self.s.hearts < MAX_HEARTS { ctx.request_repaint_after(std::time::Duration::from_secs(1)); }
        if self.onboard.is_none() { self.week_check(); }
        if self.splash.is_some() && self.onboard.is_none() {
            self.splash_screen(ui);
            self.draw_toast(&ctx);
            return;
        }
        if self.onboard.is_some() {
            self.onboarding_screen(ui);
            self.modals(&ctx);
            self.draw_toast(&ctx);
            self.screenshot(&ctx);
            return;
        }
        let tnow = ctx.input(|i| i.time);
        let vkey = (self.tab as u32) * 10 + self.sub_train as u32 + self.sub_create as u32 * 3;
        if vkey != self.view_key { self.view_key = vkey; self.view_t0 = tnow; }
        let fade_t = ((tnow - self.view_t0) / 0.18).clamp(0.0, 1.0);
        let alpha = (1.0 - (1.0 - fade_t).powi(3)) as f32;
        if fade_t < 1.0 { ctx.request_repaint(); }
        let screen = std::mem::replace(&mut self.screen, Screen::Main);
        match screen {
            Screen::Main => {
                let (t, dd) = match self.tab { Tab::Learn => (self.courses[self.cur].name.to_string(), true), Tab::Train => ("Training".into(), false), Tab::Ranks => ("Guild Ranks".into(), false), Tab::Create => ("Spell Lab".into(), false), Tab::Hero => (self.s.name.clone(), false) };
                header(ui, self, &t, false, dd);
                nav(ui, self);
                let tab = self.tab;
                egui::CentralPanel::default().frame(egui::Frame::new().fill(BG).inner_margin(egui::Margin { left: SCREEN_PAD, right: SCREEN_PAD, top: SCREEN_PAD, bottom: 0 })).show(ui, |ui| {
                    if self.s.floor { floor(ui.painter(), ui.clip_rect(), self.s.scan); }
                    ui.set_opacity(alpha);
                    let mut sa = egui::ScrollArea::vertical().id_salt(("main", tab as u32)).auto_shrink([false, false]);
                    if let Some(y) = SHOT_SCROLL.get() && self.frames < 100 { sa = sa.vertical_scroll_offset(*y); }
                    sa.show(ui, |ui| centered(ui, |ui| match tab {
                        Tab::Learn => self.learn(ui),
                        Tab::Train => self.train(ui),
                        Tab::Ranks => self.ranks(ui),
                        Tab::Create => self.create(ui),
                        Tab::Hero => self.hero(ui),
                    }));
                });
            }
            Screen::Settings => { self.screen = Screen::Settings; self.settings(ui); }
            Screen::Run(rs) => { if let Some(rs) = self.run_screen(ui, rs) { self.screen = Screen::Run(rs); } }
            Screen::Complete(c) => { if !self.complete_screen(ui, &c) && matches!(self.screen, Screen::Main) { self.screen = Screen::Complete(c); } }
            Screen::Editor(ed) => { if let Some(ed) = self.editor_screen(ui, ed) { self.screen = Screen::Editor(ed); } }
            Screen::Preview(c) => { self.screen = Screen::Preview(c); self.preview_screen(ui, c); }
            Screen::Notes(c, m, l) => { self.screen = Screen::Notes(c, m, l); self.notes_screen(ui, c, m, l); }
            Screen::Certificate(c) => { self.screen = Screen::Certificate(c); self.certificate_screen(ui, c); }
        }
        self.modals(&ctx);
        self.draw_toast(&ctx);
        self.screenshot(&ctx);
    }

    fn modals(&mut self, ctx: &egui::Context) {
        if self.modal == Modal::None { return; }
        dim_back(ctx);
        let mut close = ctx.input(|i| i.key_pressed(egui::Key::Escape));
        match self.modal {
            Modal::Notif => dialog(ctx, "notif", |ui| {
                ui.horizontal(|ui| { icon(ui, "letter", 36.0); ui.label(title("MESSAGES", 11.0).color(GOLD)); });
                ui.add_space(SP_S);
                let day = today();
                let streak = game::live_streak(&self.s, day);
                let msg_streak = if self.s.last_day == day { format!("Today's lesson is done. Streak: {} day{}.", streak, if streak == 1 { "" } else { "s" }) }
                    else if streak > 0 { format!("Clear a lesson today to keep your {} day streak.", streak) }
                    else { "Clear a lesson today to start a streak.".into() };
                ui.horizontal_wrapped(|ui| { icon(ui, "flame", 28.0); ui.label(body(msg_streak, 14.0)); });
                let hearts = match game::next_heart_in(&self.s, now()) { Some(s) => format!("{} of {} hearts. Next heart in {}:{:02}.", self.s.hearts, MAX_HEARTS, s / 60, s % 60), None => "Your hearts are full.".into() };
                ui.horizontal_wrapped(|ui| { icon(ui, "heart", 28.0); ui.label(body(hearts, 14.0)); });
                if let Some((id, d)) = self.s.ach.iter().max_by_key(|(_, d)| **d)
                    && let Some(a) = game::achievement(id) { ui.horizontal_wrapped(|ui| { icon(ui, a.icon, 28.0); ui.label(body(format!("Latest achievement: {} ({})", a.title, day_label(*d)), 14.0)); }); }
                if !self.s.last_result.is_empty() {
                    let lr = match self.s.last_result.as_str() { "LEVEL_UP" => "Last week: promoted!", "LEVEL_DOWN" => "Last week: dropped a league.", _ => "Last week: league unchanged." };
                    ui.horizontal_wrapped(|ui| { icon(ui, "banner_royal", 28.0); ui.label(body(lr, 14.0)); });
                }
                ui.add_space(SP_M);
                if pbtn(ui, "CLOSE", BLUE, 0.0).clicked() { close = true; }
            }),
            Modal::Shop => dialog(ctx, "shop", |ui| {
                ui.horizontal(|ui| { icon(ui, "money_bag", 36.0); ui.label(title("SHOP", 12.0).color(GOLD)); ui.add_space(SP_M); icon(ui, "crystal_purple", 28.0); ui.label(title(format!("{}", self.s.gems), 11.0)); });
                ui.add_space(SP_M);
                ui.horizontal_wrapped(|ui| { icon(ui, "heart", 34.0); ui.label(body(format!("Refill all hearts ({} gems)", REFILL_COST), 14.0)); });
                if pbtn(ui, "BUY HEARTS", RED, 0.0).clicked() { match game::buy_refill(&mut self.s) { Ok(()) => { self.persist(); self.toast("Hearts refilled!"); audio::play(audio::Sfx::Coin); } Err(e) => self.toast(e.msg()) } }
                let claimed = self.s.free_day == today();
                if pbtn(ui, if claimed { "FREE HEART (TOMORROW)" } else { "FREE HEART (1 PER DAY)" }, if claimed { PANEL_HI } else { GREEN }, 0.0).clicked() {
                    match game::claim_free_heart(&mut self.s, today()) { Ok(()) => { self.persist(); self.toast("+1 heart!"); } Err(e) => self.toast(e.msg()) }
                }
                ui.add_space(SP_M);
                ui.horizontal_wrapped(|ui| { icon(ui, "hourglass", 34.0); ui.label(body(format!("Streak saver ({} gems, hold up to 3). Protects your streak if you miss one day.", SAVER_COST), 14.0)); });
                if pbtn(ui, &format!("BUY SAVER ({}/3)", self.s.savers), BLUE, 0.0).clicked() { match game::buy_saver(&mut self.s) { Ok(()) => { self.persist(); self.toast("Streak saver added!"); audio::play(audio::Sfx::Coin); } Err(e) => self.toast(e.msg()) } }
                ui.add_space(SP_M);
                ui.label(body("Earn gems by clearing lessons, challenges and duels.", 12.0).color(DIM));
                if pbtn(ui, "CLOSE", PANEL_HI, 0.0).clicked() { close = true; }
            }),
            Modal::Courses => dialog(ctx, "courses", |ui| {
                ui.label(title("CHOOSE A COURSE", 11.0).color(GOLD)); ui.add_space(SP_S);
                for ci in 0..self.courses.len() {
                    ui.horizontal(|ui| {
                        emblem(ui, self.courses[ci].tag, self.courses[ci].color, 44.0);
                        let label = format!("{}  {}%", self.courses[ci].name, (self.progress(ci) * 100.0) as u32);
                        let w = (ui.available_width() - 52.0).max(80.0);
                        if pbtn(ui, &label, if ci == self.cur { GOLD } else { PANEL_HI }, w).clicked() { self.select_course(ci); close = true; }
                        if pbtn(ui, "i", BLUE, 44.0).on_hover_text("Course details").clicked() { self.screen = Screen::Preview(ci); close = true; }
                    });
                    ui.add_space(SP_XS);
                }
                if pbtn(ui, "CLOSE", PANEL_HI, 0.0).clicked() { close = true; }
            }),
            Modal::NoHearts => dialog(ctx, "nohearts", |ui| {
                ui.vertical_centered(|ui| {
                    let (r, _) = ui.allocate_exact_size(Vec2::splat(96.0), Sense::hover());
                    draw_icon(ui.painter(), r, "heart", Color32::from_gray(70));
                    ui.label(title("OUT OF HEARTS", 12.0).color(RED)); ui.add_space(SP_S);
                    let wait = game::next_heart_in(&self.s, now()).map(|s| format!("Next heart in {}:{:02}. ", s / 60, s % 60)).unwrap_or_default();
                    ui.label(body(format!("{}Hearts refill 1 every 5 minutes. Your lesson progress is saved.", wait), 14.0)); ui.add_space(SP_S);
                });
                if pbtn(ui, &format!("REFILL ({} GEMS)", REFILL_COST), RED, 0.0).clicked() {
                    match game::buy_refill(&mut self.s) { Ok(()) => { self.persist(); close = true; audio::play(audio::Sfx::Coin); } Err(e) => self.toast(e.msg()) }
                }
                if self.s.free_day != today() && pbtn(ui, "FREE DAILY HEART", GREEN, 0.0).clicked()
                    && game::claim_free_heart(&mut self.s, today()).is_ok() { self.persist(); close = true; }
                ui.add_space(SP_S);
                if pbtn(ui, "PRACTISE A DUEL (NO HEARTS)", PANEL_HI, 0.0).clicked() { self.tab = Tab::Train; self.sub_train = 1; self.screen = Screen::Main; close = true; }
                if pbtn(ui, "LATER", PANEL_HI, 0.0).clicked() { close = true; }
            }),
            Modal::CertName(c) => dialog(ctx, "certname", |ui| {
                let cs_name = self.courses[c].name;
                let name = Save::clean_cert_name(&self.cert_input);
                ui.label(title("IS THIS NAME CORRECT?", 11.0).color(GOLD)); ui.add_space(SP_S);
                ui.vertical_centered(|ui| ui.label(title(name.clone(), 16.0).color(INK)));
                ui.add_space(SP_S);
                ui.label(body(format!("This name will be printed on your {} certificate. Once you confirm, it is locked and can never be changed. Check the spelling!", cs_name), 14.0));
                ui.add_space(SP_S);
                if pbtn(ui, "YES, PRINT IT", GOLD, 0.0).clicked() {
                    let slug = self.courses[c].slug;
                    match self.s.lock_cert_name(slug, &name) {
                        Ok(n) => {
                            self.log(format!("signed the {} certificate as {}", cs_name, n));
                            self.persist();
                            audio::play(audio::Sfx::Complete);
                            self.cert_input.clear();
                        }
                        Err(e) => self.toast(e),
                    }
                    close = true;
                }
                ui.add_space(SP_S);
                if pbtn(ui, "LET ME FIX IT", PANEL_HI, 0.0).clicked() { close = true; }
            }),
            Modal::WeekResult => dialog(ctx, "week", |ui| {
                let (pl, from, to) = (self.s.res_place as usize, self.s.res_from as usize, self.s.res_to as usize);
                ui.vertical_centered(|ui| {
                    ribbon(ui, "WEEK RESULTS");
                    let (r, _) = ui.allocate_exact_size(Vec2::splat(96.0), Sense::hover());
                    draw_icon(ui.painter(), r, match pl { 1 => "gold_trophy", 2 => "silver_trophy", 3 => "bronze_trophy", _ => "banner" }, Color32::WHITE);
                    ui.label(title(format!("YOU FINISHED #{}", pl), 13.0).color(GOLD));
                    ui.add_space(SP_XS);
                    ui.label(body(format!("{} XP last week", self.s.res_xp), 14.0).color(DIM));
                    ui.add_space(SP_S);
                    let (msg, col) = if to > from { (format!("Promoted to the {} league!", LEAGUES[to.min(6)]), GREEN) } else if to < from { (format!("Dropped to the {} league", LEAGUES[to.min(6)]), RED) } else { (format!("You stay in the {} league", LEAGUES[to.min(6)]), INK) };
                    ui.label(title(msg, 10.0).color(col));
                    if self.s.res_gems > 0 { ui.add_space(SP_S); ui.horizontal(|ui| { icon(ui, "open_gold_red_chest", 40.0); ui.label(body(format!("Podium chest: +{} gems", self.s.res_gems), 15.0).color(PURPLE)); }); }
                    ui.add_space(SP_S);
                    ui.label(body("A new week has begun with new rivals. Top 3 move up, the bottom 2 move down.", 13.0).color(DIM));
                });
                ui.add_space(SP_S);
                if pbtn(ui, "TO THE NEW WEEK", GOLD, 0.0).clicked() { self.s.res_seen = true; self.persist(); close = true; }
            }),
            Modal::Import => dialog(ctx, "import", |ui| {
                let Some(s) = self.pending_import.as_ref() else { close = true; return };
                ui.label(title("IMPORT PROGRESS?", 11.0).color(GOLD)); ui.add_space(SP_S);
                let lessons = s.done.len();
                ui.label(body(format!("Hero: {}\nLevel {}  -  {} XP\n{} lessons cleared  -  {} gems", s.name, game::level(s.xp), s.xp, lessons, s.gems), 15.0));
                ui.add_space(SP_S);
                ui.label(body(format!("This replaces the progress of {} on this device. A copy of the current progress is kept in the save folder.", self.s.name), 13.0).color(DIM));
                ui.add_space(SP_S);
                if pbtn(ui, "YES, IMPORT", GOLD, 0.0).clicked() { self.apply_import(); close = true; }
                ui.add_space(SP_S);
                if pbtn(ui, "CANCEL", PANEL_HI, 0.0).clicked() { self.pending_import = None; close = true; }
            }),
            Modal::Reset => dialog(ctx, "reset", |ui| {
                ui.label(title("RESET PROGRESS?", 11.0).color(RED)); ui.add_space(SP_S);
                ui.label(body("This erases XP, gems, streak, achievements, saved spells and finished lessons. It cannot be undone.", 14.0)); ui.add_space(SP_S);
                if pbtn(ui, "YES, RESET", RED, 0.0).clicked() {
                    let (n, a) = (self.s.name.clone(), self.s.avatar);
                    self.s = Save::new(); self.s.heart_t = now(); self.s.name = n; self.s.avatar = a; self.s.onboarded = true;
                    game::week_roll(&mut self.s, today());
                    self.persist(); self.cur = 0; self.open_mod = self.current_module(0); self.screen = Screen::Main; self.toast("Progress reset");
                    close = true;
                }
                ui.add_space(SP_S);
                if pbtn(ui, "CANCEL", PANEL_HI, 0.0).clicked() { close = true; }
            }),
            Modal::Avatar => dialog(ctx, "avatar", |ui| {
                ui.label(title("CHOOSE YOUR HERO", 11.0).color(GOLD)); ui.add_space(SP_S);
                if avatar_grid(ui, &mut self.s.avatar) { self.persist(); close = true; }
                ui.add_space(SP_S);
                if pbtn(ui, "CLOSE", PANEL_HI, 0.0).clicked() { close = true; }
            }),
            Modal::DeleteSpell(i) => dialog(ctx, "delspell", |ui| {
                let name = self.s.codes.get(i).map(|c| c.0.clone()).unwrap_or_default();
                ui.label(title("DELETE SPELL?", 11.0).color(RED)); ui.add_space(SP_S);
                ui.label(body(format!("'{}' will be gone for good.", name), 14.0)); ui.add_space(SP_S);
                if pbtn(ui, "DELETE", RED, 0.0).clicked() {
                    if i < self.s.codes.len() { self.s.codes.remove(i); self.persist(); }
                    self.screen = Screen::Main; close = true;
                }
                if pbtn(ui, "KEEP IT", PANEL_HI, 0.0).clicked() { close = true; }
            }),
            Modal::None => {}
        }
        if close { self.modal = Modal::None; }
    }

    fn draw_toast(&mut self, ctx: &egui::Context) {
        let dt = ctx.input(|i| i.stable_dt).min(0.1) as f64;
        let Some((t, left)) = self.toasts.front_mut() else { return };
        *left -= dt;
        let t = t.clone();
        if *left <= 0.0 { self.toasts.pop_front(); }
        ctx.request_repaint();
        let r = ctx.content_rect();
        let w = (r.width() - 32.0).min(320.0);
        egui::Area::new(egui::Id::unique("toast")).order(egui::Order::Tooltip).fixed_pos(Pos2::new(r.center().x - w / 2.0, r.max.y - 140.0)).interactable(false).show(ctx, |ui| {
            let (rr, _) = ui.allocate_exact_size(Vec2::new(w, 40.0), Sense::hover());
            bevel(ui.painter(), rr, Color32::from_rgb(0x1f, 0x1a, 0x35), false);
            ui.painter().rect_stroke(rr, 0.0, Stroke::new(2.0, GOLD), egui::StrokeKind::Inside);
            ui.painter().with_clip_rect(rr).text(rr.center(), Align2::CENTER_CENTER, t, FontId::new(15.0, egui::FontFamily::Proportional), INK);
        });
    }

    fn screenshot(&mut self, ctx: &egui::Context) {
        let Some(path) = self.shot.clone() else { return };
        self.frames += 1;
        ctx.request_repaint();
        if self.frames == *SHOT_FRAME.get().unwrap_or(&120) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::ScreenshotCallback::new(move |img| {
                let (w, h) = (img.width() as u32, img.height() as u32);
                let raw: Vec<u8> = img.pixels.iter().flat_map(|c| c.to_array()).collect();
                if let Some(b) = image::RgbaImage::from_raw(w, h, raw) { let _ = b.save(&path); }
                std::process::exit(0);
            })));
        }
    }
}

/// "today", "yesterday" or "N days ago".
/// Screenshot QA: scroll the main tab down by this many points.
static SHOT_SCROLL: std::sync::OnceLock<f32> = std::sync::OnceLock::new();
/// Screenshot QA: which frame to capture (default 120).
static SHOT_FRAME: std::sync::OnceLock<u32> = std::sync::OnceLock::new();

fn day_label(d: u64) -> String {
    match today().saturating_sub(d) { 0 => "today".into(), 1 => "yesterday".into(), n => format!("{} days ago", n) }
}

/// Grid of avatar tiles. Returns true when one was clicked.
fn avatar_grid(ui: &mut egui::Ui, cur: &mut u32) -> bool {
    let mut picked = false;
    let cols = 4;
    let size = ((ui.available_width() - (cols as f32 - 1.0) * 10.0) / cols as f32).clamp(52.0, 96.0);
    let lead = ((ui.available_width() - (cols as f32 * size + (cols as f32 - 1.0) * 10.0)) / 2.0).max(0.0);
    for row in AVATARS.chunks(cols).enumerate() {
        ui.horizontal(|ui| {
            ui.add_space(lead); // centre the grid
            ui.spacing_mut().item_spacing.x = 10.0;
            for (j, a) in row.1.iter().enumerate() {
                let i = (row.0 * cols + j) as u32;
                ui.vertical(|ui| {
                    let (r, resp) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());
                    draw_avatar(ui.painter(), r, i);
                    if *cur == i { ui.painter().rect_stroke(r.expand(3.0), 0.0, Stroke::new(3.0, GOLD), egui::StrokeKind::Outside); }
                    else if resp.hovered() { ui.painter().rect_stroke(r.expand(3.0), 0.0, Stroke::new(2.0, DIM), egui::StrokeKind::Outside); }
                    ui.label(body(a.name, 12.0).color(if *cur == i { GOLD } else { DIM }));
                    if resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() { *cur = i; picked = true; }
                });
            }
        });
        ui.add_space(SP_XS);
    }
    picked
}

/// Fonts, pictures and style shared by the desktop and web versions.
fn setup(cc: &eframe::CreationContext<'_>) {
    install_fonts(&cc.egui_ctx);
    Box::leak(Box::new(load_textures(&cc.egui_ctx)));
    setup_style(&cc.egui_ctx);
}

/// Android entry point: the app's NativeActivity calls this when it starts.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    platform::set_android_dirs(app.internal_data_path(), app.external_data_path());
    platform::set_app(app.clone());
    let opts = eframe::NativeOptions { android_app: Some(app), ..Default::default() };
    let _ = eframe::run_native("Code Forge", opts, Box::new(|cc| { setup(cc); Ok(Box::new(App::new(None))) }));
}

/// Web entry point: the page has a <canvas id="code_forge"> that the game draws into.
#[cfg(target_arch = "wasm32")]
fn main() {
    use wasm_bindgen::JsCast;
    wasm_bindgen_futures::spawn_local(async {
        let doc = web_sys::window().and_then(|w| w.document()).expect("no page");
        let canvas = doc.get_element_by_id("code_forge").and_then(|e| e.dyn_into::<web_sys::HtmlCanvasElement>().ok()).expect("no canvas");
        let started = eframe::WebRunner::new().start(canvas, eframe::WebOptions::default(), Box::new(|cc| { setup(cc); Ok(Box::new(App::new(None))) })).await;
        if let Some(el) = doc.get_element_by_id("loading") { if started.is_ok() { el.remove(); } else { el.set_inner_html("Code Forge couldn't start in this browser. Try Chrome, Edge, Safari or Firefox."); } }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    let arg = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1).cloned());
    if let Some(i) = args.iter().position(|a| a == "--export-audio") {
        let dir = args.get(i + 1).cloned().unwrap_or_else(|| "audio".into());
        match audio::export(std::path::Path::new(&dir)) { Ok(n) => println!("wrote {} sounds to {}", n, dir), Err(e) => eprintln!("export failed: {}", e) }
        return Ok(());
    }
    if args.iter().any(|a| a == "--version") { println!("Code Forge {}", VERSION); return Ok(()); }
    // Release builds have no console: record any crash next to the save so it can be reported.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = std::fs::create_dir_all(Save::dir());
        let _ = std::fs::write(Save::dir().join("crash.log"), format!("Code Forge {} crashed (day {}):
{}
", VERSION, today(), info));
        default_hook(info);
    }));
    let shot = arg("--screenshot");
    if let Some(f) = arg("--shot-frame").and_then(|v| v.parse::<u32>().ok()) { let _ = SHOT_FRAME.set(f.max(2)); }
    if let Some(y) = arg("--scroll").and_then(|v| v.parse::<f32>().ok()) { let _ = SHOT_SCROLL.set(y); }
    let tab = arg("--tab");
    let screen = arg("--screen");
    let lesson = arg("--lesson");
    let force_onboarding = args.iter().any(|a| a == "--onboarding");
    let num = |k: &str| arg(k).and_then(|v| v.parse::<usize>().ok()).unwrap_or(0);
    let (stage, skip) = (num("--stage"), num("--skip"));
    let mut vp = egui::ViewportBuilder::default().with_inner_size([440.0, 780.0]).with_min_inner_size([320.0, 480.0]).with_title(format!("Code Forge {}", VERSION)).with_app_id("code_forge");
    if let Ok(icon) = eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon.png")) { vp = vp.with_icon(std::sync::Arc::new(icon)); }
    if let Some(sz) = arg("--size").and_then(|s| s.split_once('x').and_then(|(w, h)| Some((w.parse::<f32>().ok()?, h.parse::<f32>().ok()?)))) { vp = vp.with_inner_size([sz.0, sz.1]); }
    if shot.is_some() { vp = vp.with_active(false).with_window_level(egui::WindowLevel::AlwaysOnBottom); }
    eframe::run_native("Code Forge", eframe::NativeOptions { viewport: vp, ..Default::default() }, Box::new(move |cc| {
        setup(cc);
        let mut app = App::new(shot);
        if force_onboarding { app.onboard = Some(onboarding::Onboard::default()); }
        if let Some(t) = tab { if t == "practice" { app.sub_train = 1; } app.tab = match t.as_str() { "train" | "practice" | "community" => Tab::Train, "ranks" | "leader" => Tab::Ranks, "create" => Tab::Create, "hero" | "profile" => Tab::Hero, _ => Tab::Learn }; }
        let qa = app.demo;
        if let Some(sc) = screen.filter(|_| qa) {
            match sc.as_str() {
                "settings" => app.screen = Screen::Settings,
                "editor" => app.screen = Screen::Editor(Editor { idx: None, title: "Royal Banner".into(), lang: "css".into(), code: EXAMPLES[5].code.into(), out: interp::run("css", EXAMPLES[5].code), stdin: String::new() }),
                "sql" => app.screen = Screen::Editor(Editor { idx: None, title: "Strongest Heroes".into(), lang: "sql".into(), code: EXAMPLES[6].code.into(), out: interp::run("sql", EXAMPLES[6].code), stdin: String::new() }),
                "complete" => app.screen = Screen::Complete(Complete { title: "Project: Wanted Poster".into(), xp: 25, gems: 5, perfect: true, score: 100, best: 100, duel: None, new_streak: true, next: Some((0, 1, 0)), project: true, summary: app.courses[0].modules[0].lessons[7].summary.clone(), cert: Some(0) }),
                "preview" => app.screen = Screen::Preview(1),
                "preview-js" => app.screen = Screen::Preview(2),
                "notes" => app.screen = Screen::Notes(2, 1, 4),
                "week" => { app.s.res_place = 2; app.s.res_from = 2; app.s.res_to = 3; app.s.res_xp = 180; app.s.res_gems = 6; app.s.res_seen = false; }
                "cert" => { app.s.ach.insert(app.courses[0].badge_id(), today()); app.s.cert_names.insert("html".into(), "Ada Lovelace".into()); app.screen = Screen::Certificate(0); }
                "certname" => { app.s.ach.insert(app.courses[0].badge_id(), today()); app.screen = Screen::Certificate(0); }
                "shop" => app.modal = Modal::Shop,
                "notif" => app.modal = Modal::Notif,
                "avatar" => app.modal = Modal::Avatar,
                "nohearts" => app.modal = Modal::NoHearts,
                _ => {}
            }
        }
        // QA helper: --lesson C.M.L [--stage N] [--skip N] opens a lesson directly.
        if let Some(spec) = lesson.filter(|_| qa) {
            let p: Vec<usize> = spec.split('.').filter_map(|x| x.parse().ok()).collect();
            if p.len() == 3 && p[0] < app.courses.len() && p[1] < app.courses[p[0]].modules.len() && p[2] < app.courses[p[0]].modules[p[1]].lessons.len() {
                app.open_lesson(p[0], p[1], p[2]);
                if let Screen::Run(rs) = &mut app.screen { rs.stage = stage.min(rs.pages()); for _ in 0..skip { if rs.queue.len() > 1 { rs.queue.remove(0); } } }
            }
        }
        Ok(Box::new(app))
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// End-to-end over the real App logic (no window): first launch -> onboarding -> clear a
    /// lesson -> restart -> everything persisted (US1 + US3).
    #[test]
    fn first_launch_to_cleared_lesson_persists() {
        let dir = std::env::temp_dir().join(format!("code_forge_e2e_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        // SAFETY: only this test reads or writes APPDATA/HOME in the test binary.
        unsafe { std::env::set_var("APPDATA", &dir); std::env::set_var("HOME", &dir); }

        let mut app = App::new(None);
        assert!(app.onboard.is_some(), "first launch shows the welcome flow");
        app.s.name = Save::clean_name("  Ada  ");
        app.s.avatar = 2;
        app.cur = 3; app.s.course = 3; // Python
        app.s.onboarded = true; app.onboard = None;
        app.persist();

        app.open_lesson(3, 0, 0);
        let Screen::Run(mut rs) = std::mem::replace(&mut app.screen, Screen::Main) else { panic!("lesson did not open") };
        rs.queue.clear(); rs.right = rs.total as u32; // answered everything right first time
        app.finish(&rs);
        assert!(matches!(app.screen, Screen::Complete(_)));

        let again = App::new(None);
        assert!(again.onboard.is_none(), "welcome flow is not shown twice");
        assert_eq!(again.s.name, "Ada");
        assert_eq!(again.s.avatar, 2);
        assert_eq!(again.cur, 3);
        assert!(again.s.done.contains("python/printing"));
        assert_eq!(again.s.xp, 15, "first clear + flawless bonus");
        assert_eq!(again.s.gems, 25);
        assert_eq!(again.s.streak, 1);
        assert!(again.s.ach.contains_key("first_lesson"));
        assert!(again.s.ach.contains_key("flawless"));
        assert!(again.s.log.iter().any(|(_, t)| t.contains("cleared")));
        assert!(dir.join("code_forge").join("save.bak").exists(), "previous save kept as backup");
        let _ = std::fs::remove_dir_all(&dir);
    }

    // ---------- headless UI driving ----------
    fn key(k: egui::Key, ctrl: bool) -> egui::Event {
        let m = if ctrl { egui::Modifiers { ctrl: true, command: true, ..Default::default() } } else { egui::Modifiers::NONE };
        egui::Event::Key { key: k, physical_key: None, pressed: true, repeat: false, modifiers: m }
    }
    fn ui_ctx() -> egui::Context {
        let ctx = egui::Context::default();
        install_fonts(&ctx);
        setup_style(&ctx);
        Box::leak(Box::new(load_textures(&ctx)));
        ctx
    }
    fn frame(ctx: &egui::Context, app: &mut App, events: Vec<egui::Event>) {
        let modifiers = events.iter().find_map(|e| if let egui::Event::Key { modifiers, .. } = e { Some(*modifiers) } else { None }).unwrap_or_default();
        let mut events = events;
        events.insert(0, egui::Event::ModifiersChanged(modifiers));
        let raw = egui::RawInput { screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(440.0, 780.0))), events, ..Default::default() };
        let mut out = ctx.run_ui(raw, |ui| app.frame_ui(ui));
        out.textures_delta.clear();
    }
    fn demo_app() -> App { let mut a = App::new(Some(String::new())); a.shot = None; a }

    /// Play a running lesson with the keyboard until it completes. `wrong_first` answers the
    /// first question wrongly once. Returns the number of frames used.
    fn play(ctx: &egui::Context, app: &mut App, wrong_first: bool) -> usize {
        let mut wrong_left = wrong_first;
        for f in 0..400 {
            let mut ev = vec![];
            match &mut app.screen {
                Screen::Run(rs) => {
                    if rs.stage < rs.pages() || rs.fb.is_some() { ev.push(key(egui::Key::Enter, false)); }
                    else if let Some(&qi) = rs.queue.first() {
                        let mut ctrl = false;
                        match &rs.qs[qi] {
                            Q::Mc { right, opts, .. } | Q::Fill { right, opts, .. } => rs.sel = Some(if wrong_left { (right + 1) % opts.len() } else { *right }),
                            Q::Type { answer, .. } => { rs.text = if wrong_left { "nope".into() } else { answer.to_string() }; ctrl = answer.contains('\n'); }
                        }
                        wrong_left = false;
                        ev.push(key(egui::Key::Enter, ctrl));
                    }
                }
                Screen::Complete(_) => return f,
                _ => panic!("left the lesson unexpectedly"),
            }
            frame(ctx, app, ev);
        }
        let st = match &app.screen { Screen::Run(rs) => format!("{} stage {}/{} queue {:?} fb {:?} sel {:?} text {:?}", rs.title, rs.stage, rs.pages(), rs.queue, rs.fb, rs.sel, rs.text), _ => "?".into() };
        panic!("lesson did not finish: {}", st);
    }

    /// Regression for the START THE QUIZ crash: click through every lesson of every course.
    #[test]
    fn every_lesson_can_be_played_through_the_ui() {
        let ctx = ui_ctx();
        let mut app = demo_app();
        app.s.hearts = MAX_HEARTS;
        for c in 0..app.courses.len() {
            for m in 0..app.courses[c].modules.len() {
                for l in 0..app.courses[c].modules[m].lessons.len() {
                    app.open_lesson(c, m, l);
                    frame(&ctx, &mut app, vec![]);
                    play(&ctx, &mut app, false);
                    assert!(app.s.done.contains(&app.lkey(c, m, l)), "{}.{}.{} not cleared", c, m, l);
                    app.screen = Screen::Main;
                    frame(&ctx, &mut app, vec![]);
                }
            }
        }
        for t in [Tab::Learn, Tab::Train, Tab::Ranks, Tab::Create, Tab::Hero] { app.tab = t; frame(&ctx, &mut app, vec![]); }
    }

    #[test]
    fn wrong_answer_costs_a_heart_and_comes_back() {
        let ctx = ui_ctx();
        let mut app = demo_app();
        app.s.hearts = MAX_HEARTS;
        app.open_lesson(2, 0, 0);
        frame(&ctx, &mut app, vec![]);
        play(&ctx, &mut app, true);
        assert_eq!(app.s.hearts, MAX_HEARTS - 1);
        let Screen::Complete(c) = &app.screen else { panic!() };
        assert!(!c.perfect);
        assert!(c.score < 100);
    }

    #[test]
    fn practice_modes_reward_once_and_spare_hearts() {
        let ctx = ui_ctx();
        let mut app = demo_app();
        app.tab = Tab::Train; app.sub_train = 1;
        frame(&ctx, &mut app, vec![]);
        // daily quest: wrong answers cost no hearts, first clear pays +15 XP, replay pays less
        let hearts = app.s.hearts;
        let qs = app.practice_set(&app.started_courses(), 5, 1);
        assert_eq!(qs.len(), 5);
        app.start_run("Daily Quest", Kind::Daily, vec![], qs.clone());
        play(&ctx, &mut app, true);
        assert_eq!(app.s.hearts, hearts, "practice must not cost hearts");
        assert_eq!(app.s.daily_day, today());
        let Screen::Complete(c) = &app.screen else { panic!() };
        assert_eq!(c.xp, 15);
        app.start_run("Daily Quest", Kind::Daily, vec![], qs);
        play(&ctx, &mut app, false);
        let Screen::Complete(c) = &app.screen else { panic!() };
        assert_eq!(c.xp, 5, "replays give less");
        // review of a course with cleared lessons
        let qs = app.practice_set(&[0], 8, 7);
        assert!(!qs.is_empty() && qs.iter().all(|q| !matches!(q, Q::Type { .. })));
        app.start_run("HTML Review", Kind::Review(0), vec![], qs);
        play(&ctx, &mut app, false);
        assert_eq!(app.s.reviews, 1);
        // a quick challenge is remembered as solved
        let c0 = &coaches()[0];
        app.start_run(c0.title, Kind::Coach, vec![], c0.qs.clone());
        play(&ctx, &mut app, false);
        assert!(app.s.solved.contains(c0.title));
        // the certificate renders both before and after it is earned
        for earned in [false, true] {
            if earned { app.s.ach.insert(app.courses[1].badge_id(), today()); }
            app.screen = Screen::Certificate(1);
            frame(&ctx, &mut app, vec![]);
            assert!(matches!(app.screen, Screen::Certificate(1)));
        }
        assert_eq!(crate::preview::ymd(0), "1970-01-01");
        assert_eq!(crate::preview::ymd(20409), "2025-11-17");
    }

    #[test]
    fn saves_are_validated_and_progress_imports() {
        let mut app = demo_app();
        // a hand-made save claiming a certificate and a lesson that doesn't exist
        let mut s = Save::new();
        s.onboarded = true; s.name = "Rook".into(); s.xp = 777;
        s.done.insert("html/your-first-tags".into()); s.done.insert("html/not-a-lesson".into());
        s.ach.insert(app.courses[0].badge_id(), 1);
        App::validate(&mut s, &app.courses);
        assert!(!s.done.contains("html/not-a-lesson"));
        assert!(!s.ach.contains_key(&app.courses[0].badge_id()), "no certificate without finishing the course");
        // export from one device, import on another
        let file = s.export_text();
        let incoming = Save::import_text(&file).expect("valid export");
        app.pending_import = Some(Box::new(incoming));
        app.apply_import();
        assert_eq!(app.s.name, "Rook");
        assert_eq!(app.s.xp, 777);
        assert!(app.s.done.contains("html/your-first-tags"));
        // practice with nothing to practise never pays out
        let xp = app.s.xp;
        app.start_run("Empty", Kind::Daily, vec![], vec![]);
        assert!(!matches!(app.screen, Screen::Run(_)));
        assert_eq!(app.s.xp, xp);
    }

    #[test]
    fn course_preview_starts_the_next_lesson() {
        let ctx = ui_ctx();
        let mut app = demo_app();
        app.screen = Screen::Preview(1);
        frame(&ctx, &mut app, vec![]);
        assert!(matches!(app.screen, Screen::Preview(1)), "preview stays open until an action");
        frame(&ctx, &mut app, vec![key(egui::Key::Enter, false)]);
        let Screen::Run(rs) = &app.screen else { panic!("START did not open a lesson") };
        assert_eq!(rs.title, app.courses[1].modules[0].lessons[0].title);
        assert_eq!(app.cur, 1, "previewed course becomes the active course");
        for c in 0..app.courses.len() { app.screen = Screen::Preview(c); frame(&ctx, &mut app, vec![]); }
    }

    #[test]
    fn finishing_a_course_earns_its_badge_once() {
        let mut app = demo_app();
        assert!(app.badge_earned(0).is_none());
        for m in 0..app.courses[0].modules.len() { for l in 0..app.courses[0].modules[m].lessons.len() { let k = app.lkey(0, m, l); app.s.done.insert(k); } }
        app.award();
        assert!(app.badge_earned(0).is_some());
        assert!(app.badge_earned(1).is_none());
        let n = app.s.log.len();
        app.award();
        assert_eq!(app.s.log.len(), n, "badge is granted only once");
    }

    #[test]
    fn project_lessons_give_bonus_xp() {
        let ctx = ui_ctx();
        let mut app = demo_app();
        let (m, l) = (0, app.courses[0].modules[0].lessons.len() - 1);
        assert!(app.courses[0].modules[m].lessons[l].project);
        let xp0 = app.s.xp;
        app.open_lesson(0, m, l);
        frame(&ctx, &mut app, vec![]);
        play(&ctx, &mut app, false);
        assert_eq!(app.s.xp - xp0, 15 + game::PROJECT_BONUS);
    }

    #[test]
    fn no_hearts_blocks_lessons_but_not_duels() {
        let mut app = demo_app();
        app.s.hearts = 0; app.s.heart_t = now();
        app.open_lesson(0, 0, 0);
        assert!(matches!(app.screen, Screen::Main));
        assert!(app.modal == Modal::NoHearts);
        app.modal = Modal::None;
        app.start_run("duel", Kind::Duel(0), vec![], duel_questions(1));
        assert!(matches!(app.screen, Screen::Run(_)));
    }
}
