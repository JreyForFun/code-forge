mod art;
mod content;
mod data;
mod interp;
mod lesson;
mod screens;
mod tabs;

use art::*;
use data::*;
use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, RichText, Sense, Stroke, Vec2};
use std::collections::HashSet;

const XP_PER_LEVEL: u32 = 100;
const MAX_HEARTS: u32 = 5;
const HEART_SECS: u64 = 300;
const REFILL_COST: u32 = 10;
const SAVER_COST: u32 = 20;
const LEAGUE_UP_XP: u32 = 100;
const HINT_COST: u32 = 1;
const HDR: Color32 = Color32::from_rgb(0x0b, 0x12, 0x28);

fn now() -> u64 { std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) }
fn today() -> u64 { now() / 86400 }

// ---------------- saved progress ----------------
struct Save {
    name: String, country: String, xp: u32, gems: u32, hearts: u32, heart_t: u64,
    streak: u32, streak_max: u32, last_day: u64, savers: u32, week_id: u64, week_mask: u8, week_xp: u32,
    done: HashSet<String>, best: std::collections::HashMap<String, (u32, u32)>, qdone: std::collections::HashMap<String, Vec<usize>>, free_day: u64, goal: u32, league: u32, last_result: String, scan: bool, floor: bool, codes: Vec<(String, String, String)>, notif_read: bool,
}
impl Save {
    fn new() -> Self {
        Save { name: "Hero".into(), country: "Pixel Realm".into(), xp: 0, gems: 20, hearts: MAX_HEARTS, heart_t: 0, streak: 0, streak_max: 0, last_day: 0, savers: 0,
            week_id: 0, week_mask: 0, week_xp: 0, done: HashSet::new(), best: std::collections::HashMap::new(), qdone: std::collections::HashMap::new(), free_day: 0, goal: 7, league: 0, last_result: String::new(), scan: true, floor: true, codes: vec![], notif_read: false }
    }
    fn path() -> std::path::PathBuf {
        let base = std::env::var("APPDATA").or_else(|_| std::env::var("HOME")).unwrap_or(".".into());
        let d = std::path::Path::new(&base).join("pixel_learn");
        let _ = std::fs::create_dir_all(&d);
        d.join("save.txt")
    }
    fn load() -> Self {
        let mut s = Save::new();
        let Ok(t) = std::fs::read_to_string(Self::path()) else { return s };
        for line in t.lines() {
            let Some((k, v)) = line.split_once('=') else { continue };
            let n = v.parse::<u64>().unwrap_or(0);
            match k {
                "name" => s.name = v.into(), "country" => s.country = v.into(), "xp" => s.xp = n as u32, "gems" => s.gems = n as u32,
                "hearts" => s.hearts = (n as u32).min(MAX_HEARTS), "heart_t" => s.heart_t = n, "streak" => s.streak = n as u32, "streak_max" => s.streak_max = n as u32,
                "last_day" => s.last_day = n, "savers" => s.savers = n as u32, "week_id" => s.week_id = n, "week_mask" => s.week_mask = n as u8,
                "week_xp" => s.week_xp = n as u32, "free_day" => s.free_day = n, "goal" => s.goal = n as u32, "league" => s.league = (n as u32).min(6), "last_result" => s.last_result = v.into(), "scan" => s.scan = v == "1", "floor" => s.floor = v == "1", "notif" => s.notif_read = v == "1",
                "done" => { s.done.insert(v.into()); }
                "qdone" => { if let Some((k, v)) = v.split_once('|') { s.qdone.insert(k.into(), v.split(',').filter_map(|x| x.parse().ok()).collect()); } }
                "best" => { let p: Vec<&str> = v.split('|').collect(); if p.len() == 3 { s.best.insert(p[0].into(), (p[1].parse().unwrap_or(0), p[2].parse().unwrap_or(0))); } }
                "code" => { let p: Vec<&str> = v.splitn(3, '\u{1f}').collect(); if p.len() == 3 { s.codes.push((p[0].into(), p[1].into(), p[2].replace('\u{1e}', "\n"))); } }
                _ => {}
            }
        }
        s
    }
    fn store(&self) {
        let mut o = String::new();
        let mut w = |k: &str, v: String| { o.push_str(&format!("{}={}\n", k, v)); };
        w("name", self.name.replace('\n', " ")); w("country", self.country.replace('\n', " "));
        w("xp", self.xp.to_string()); w("gems", self.gems.to_string()); w("hearts", self.hearts.to_string()); w("heart_t", self.heart_t.to_string());
        w("streak", self.streak.to_string()); w("streak_max", self.streak_max.to_string()); w("last_day", self.last_day.to_string());
        w("savers", self.savers.to_string()); w("week_id", self.week_id.to_string()); w("week_mask", self.week_mask.to_string()); w("week_xp", self.week_xp.to_string()); w("free_day", self.free_day.to_string()); w("goal", self.goal.to_string()); w("league", self.league.to_string()); w("last_result", self.last_result.clone());
        w("scan", (self.scan as u8).to_string()); w("floor", (self.floor as u8).to_string()); w("notif", (self.notif_read as u8).to_string());
        for d in &self.done { w("done", d.clone()); }
        for (k, v) in &self.qdone { if !v.is_empty() { w("qdone", format!("{}|{}", k, v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))); } }
        for (k, (b, a)) in &self.best { w("best", format!("{}|{}|{}", k, b, a)); }
        for (a, b, c) in &self.codes { w("code", format!("{}\u{1f}{}\u{1f}{}", a.replace('\n', " "), b, c.replace('\n', "\u{1e}"))); }
        let _ = std::fs::write(Self::path(), o);
    }
}

// ---------------- state ----------------
#[derive(Clone, Copy, PartialEq)]
enum Tab { Learn, Community, Leader, Create, Profile }
#[derive(Clone, Copy, PartialEq)]
enum Modal { None, Notif, Shop, Courses, NoHearts, Reset }

#[derive(Clone)]
enum Kind { Lesson(usize, usize, usize), Coach(usize), Duel(usize) }

struct RunState {
    title: String, kind: Kind, lang: &'static str, slides: Vec<(&'static str, &'static str)>, qs: Vec<Q>, stage: usize, queue: Vec<usize>,
    sel: Option<usize>, text: String, fb: Option<(bool, String)>, mistakes: u32, right: u32, total: usize, last: Option<usize>, hint_q: Option<usize>,
}
struct Complete { title: String, xp: u32, gems: u32, perfect: bool, score: u32, best: u32, duel: Option<(u32, u32, String)>, new_streak: bool }
struct Editor { idx: Option<usize>, title: String, lang: String, code: String, out: Vec<(interp::Kind, String)> }

enum Screen { Main, Settings, Run(Box<RunState>), Complete(Complete), Editor(Editor), Post(usize, Vec<(interp::Kind, String)>), Thread(usize), NewThread }

struct App {
    s: Save, tab: Tab, screen: Screen, modal: Modal, courses: Vec<Course>, cur: usize, open_mod: Option<usize>,
    opp: usize, sub_comm: usize, sub_create: usize, posts: Vec<Post>, voted: HashSet<usize>, threads: Vec<Thread>, extra_answers: Vec<(usize, String)>,
    input: String, input2: String, cat: usize, query: String, view_key: u32, view_t0: f64, toast: Option<(String, f64)>, shot: Option<String>, frames: u32, feed: Vec<String>,
}

impl App {
    fn new(shot: Option<String>) -> Self {
        let mut s = Save::load();
        if s.heart_t == 0 { s.heart_t = now(); }
        let mut a = App { s, tab: Tab::Learn, screen: Screen::Main, modal: Modal::None, courses: courses(), cur: 0, open_mod: None, opp: 0, sub_comm: 0, sub_create: 0,
            posts: posts(), voted: HashSet::new(), threads: threads(), extra_answers: vec![], input: String::new(), input2: String::new(), cat: 0, query: String::new(), view_key: 999, view_t0: 0.0, toast: None, shot, frames: 0, feed: vec![] };
        a.week_roll();
        if a.shot.is_some() { a.s.xp = 399; a.s.gems = 65; a.s.streak = 3; a.s.streak_max = 4; a.s.week_xp = 120; a.s.week_mask = 0b0000101; a.s.done.insert("0.0.0".into()); a.s.done.insert("0.0.1".into()); a.s.done.insert("0.1.0".into()); a.s.done.insert("0.1.1".into()); }
        a.open_mod = a.current_module(0);
        a
    }
    fn week_roll(&mut self) {
        let wk = (today() + 3) / 7;
        if self.s.week_id != wk {
            // Weekly league result: the original reports LEVEL_UP, LEVEL_DOWN or FREEZE per user.
            if self.s.week_id != 0 {
                let res = if self.s.week_xp >= LEAGUE_UP_XP && self.s.league < 6 { self.s.league += 1; "LEVEL_UP" }
                    else if self.s.week_xp == 0 && self.s.league > 0 { self.s.league -= 1; "LEVEL_DOWN" }
                    else { "FREEZE" };
                self.s.last_result = res.into();
                let names = ["Copper", "Venus", "Terra", "Mars", "Jupiter", "Saturn", "Neptune"];
                let msg = match res { "LEVEL_UP" => format!("Promoted to the {} league!", names[self.s.league as usize]), "LEVEL_DOWN" => format!("Dropped to the {} league", names[self.s.league as usize]), _ => "League unchanged this week".to_string() };
                self.toast = Some((msg, 4.0));
            }
            self.s.week_id = wk; self.s.week_mask = 0; self.s.week_xp = 0; self.s.store();
        }
    }
    fn level(&self) -> u32 { self.s.xp / XP_PER_LEVEL + 1 }
    fn key(c: usize, m: usize, l: usize) -> String { format!("{}.{}.{}", c, m, l) }
    fn is_done(&self, c: usize, m: usize, l: usize) -> bool { self.s.done.contains(&Self::key(c, m, l)) }
    fn module_done(&self, c: usize, m: usize) -> bool { (0..self.courses[c].modules[m].lessons.len()).all(|l| self.is_done(c, m, l)) }
    fn current_module(&self, c: usize) -> Option<usize> { (0..self.courses[c].modules.len()).find(|m| !self.module_done(c, *m)) }
    fn lesson_unlocked(&self, c: usize, m: usize, l: usize) -> bool {
        for mm in 0..m { if !self.module_done(c, mm) { return false; } }
        (0..l).all(|ll| self.is_done(c, m, ll))
    }
    fn progress(&self, c: usize) -> f32 {
        let total: usize = self.courses[c].modules.iter().map(|m| m.lessons.len()).sum();
        let done = self.s.done.iter().filter(|k| k.starts_with(&format!("{}.", c))).count();
        done as f32 / total.max(1) as f32
    }
    fn tick(&mut self) {
        if self.s.hearts >= MAX_HEARTS { self.s.heart_t = now(); return; }
        let t = now();
        while self.s.hearts < MAX_HEARTS && t >= self.s.heart_t + HEART_SECS { self.s.hearts += 1; self.s.heart_t += HEART_SECS; }
    }
    fn lose_heart(&mut self) {
        if self.s.hearts == MAX_HEARTS { self.s.heart_t = now(); }
        self.s.hearts = self.s.hearts.saturating_sub(1);
        self.s.store();
    }
    fn say(&mut self, t: &str) { self.toast = Some((t.into(), 2.5)); }

    fn start_run(&mut self, title: &str, kind: Kind, slides: Vec<(&'static str, &'static str)>, qs: Vec<Q>) {
        if self.s.hearts == 0 && !matches!(kind, Kind::Duel(_)) { self.modal = Modal::NoHearts; return; }
        let n = qs.len();
        let (mut resume_stage, mut resume_queue, mut resume_right) = (0usize, (0..n).collect::<Vec<usize>>(), 0u32);
        if let Kind::Lesson(c, mi, l) = &kind {
            let k = Self::key(*c, *mi, *l);
            // original rule (LessonProgress.isQuizCompleted): a finished lesson counts every quiz as done; otherwise only quizzes marked completed are
            if !self.s.done.contains(&k) {
                if let Some(d) = self.s.qdone.get(&k) {
                    if !d.is_empty() { resume_queue.retain(|i| !d.contains(i)); resume_right = d.len() as u32; resume_stage = slides.len(); }
                }
            }
        }
        let lang = match &kind { Kind::Lesson(c, _, _) => self.courses[*c].lang, _ => "" };
        self.screen = Screen::Run(Box::new(RunState { title: title.into(), lang, kind, slides, qs, stage: resume_stage, queue: resume_queue, sel: None, text: String::new(), fb: None, mistakes: 0, right: resume_right, total: n, last: None, hint_q: None }));
    }

    fn finish(&mut self, rs: &RunState) {
        self.week_roll();
        let mut xp = 0; let mut gems = 0; let mut duel = None;
        let perfect = rs.mistakes == 0;
        match rs.kind {
            Kind::Lesson(c, m, l) => {
                let first = !self.is_done(c, m, l);
                self.s.qdone.remove(&Self::key(c, m, l));
                xp = if first { 10 + if perfect { 5 } else { 0 } } else { 5 };
                if first { gems = 5; self.s.done.insert(Self::key(c, m, l)); self.feed.push(format!("completed '{}'", rs.title)); }
                self.open_mod = self.current_module(c).or(Some(m));
            }
            Kind::Coach(_) => { xp = 8; gems = 2; self.feed.push(format!("solved '{}'", rs.title)); }
            Kind::Duel(o) => {
                let me = rs.right;
                let bot = opponents()[o].skill.min(rs.total as u32);
                let won = me > bot || (me == bot && me as usize == rs.total);
                if won { xp = 20; gems = 5; } else { xp = 5; }
                duel = Some((me, bot, opponents()[o].name.to_string()));
                self.feed.push(format!("{} a duel vs {}", if won { "won" } else { "lost" }, opponents()[o].name));
            }
        }
        let attempts_made = rs.total as u32 + rs.mistakes;
        let score = if attempts_made == 0 { 100 } else { (rs.total as u32 * 100 / attempts_made).min(100) };
        let mut best = score;
        if let Kind::Lesson(c, mi, l) = rs.kind {
            let e = self.s.best.entry(Self::key(c, mi, l)).or_insert((0, 0));
            e.0 = e.0.max(score); e.1 += 1; best = e.0;
        }
        let new_streak = self.bump_streak();
        self.s.xp += xp; self.s.gems += gems; self.s.week_xp += xp;
        self.s.store();
        self.screen = Screen::Complete(Complete { title: rs.title.clone(), xp, gems, perfect, score, best, duel, new_streak });
    }
    fn bump_streak(&mut self) -> bool {
        let t = today();
        let dow = ((t + 3) % 7) as u8;
        self.s.week_mask |= 1 << dow;
        if self.s.last_day == t { return false; }
        if self.s.last_day + 1 == t { self.s.streak += 1; }
        else if self.s.last_day + 2 == t && self.s.savers > 0 { self.s.savers -= 1; self.s.streak += 1; }
        else { self.s.streak = 1; }
        self.s.streak_max = self.s.streak_max.max(self.s.streak);
        self.s.last_day = t;
        true
    }
}

// ---------------- shared ui helpers ----------------
fn chip(ui: &mut egui::Ui, s: Spr, over: &[(char, Color32)], text: &str) {
    let (r, _) = ui.allocate_exact_size(Vec2::new(40.0 + text.len() as f32 * 12.0, 34.0), Sense::hover());
    let p = ui.painter();
    p.rect_filled(r.expand(2.0), 0.0, EDGE);
    p.rect_filled(r, 0.0, Color32::from_rgb(0x10, 0x0c, 0x1c));
    draw_spr(p, Pos2::new(r.min.x + 6.0, r.center().y - 12.0), s, 2.0, over);
    p.text(Pos2::new(r.min.x + 34.0, r.center().y + 1.0), Align2::LEFT_CENTER, text, FontId::new(11.0, press()), INK);
}

fn stats_row(ui: &mut egui::Ui, app: &mut App, show_hearts: bool) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        if show_hearts { chip(ui, Spr::Heart, &[], &app.s.hearts.to_string()); }
        chip(ui, Spr::Bolt, &[('a', if app.s.streak > 0 { GREEN } else { Color32::GRAY })], &app.s.streak.to_string());
        chip(ui, Spr::Gem, &[], &app.s.gems.to_string());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if pbtn(ui, "SHOP", GOLD, 80.0).clicked() { app.modal = Modal::Shop; }
        });
    });
}

fn header(ctx: &mut egui::Ui, app: &mut App, title_text: &str, back: bool, dropdown: bool) {
    egui::Panel::top("hdr").frame(egui::Frame::new().fill(HDR).inner_margin(egui::Margin { left: 12, right: 12, top: 12, bottom: 12 })).show(ctx, |ui| {
        let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 30.0), Sense::hover());
        let rr = ui.max_rect();
        ui.painter().rect_filled(Rect::from_min_size(Pos2::new(rr.min.x - 12.0, rr.max.y + 8.0), Vec2::new(rr.width() + 24.0, 4.0)), 0.0, GOLD);
        let mut x = r.min.x;
        if back {
            let br = Rect::from_min_size(Pos2::new(x, r.min.y), Vec2::new(30.0, 30.0));
            let resp = ui.interact(br, ui.id().with("back"), Sense::click());
            ui.painter().text(br.center(), Align2::CENTER_CENTER, "<", FontId::new(16.0, press()), Color32::WHITE);
            if resp.clicked() { app.screen = Screen::Main; }
            x += 36.0;
        }
        let galley = ui.painter().layout_no_wrap(title_text.to_string(), FontId::new(12.0, press()), Color32::WHITE);
        let tw = galley.size().x.min(r.width() - 130.0);
        let th = galley.size().y;
        let tr = Rect::from_min_size(Pos2::new(x, r.min.y), Vec2::new(tw + if dropdown { 28.0 } else { 0.0 }, 30.0));
        ui.painter().galley(Pos2::new(x, r.center().y - th / 2.0), galley, Color32::WHITE);
        if dropdown {
            let c = Pos2::new(x + tw + 14.0, r.center().y);
            for i in 0..4 { ui.painter().rect_filled(Rect::from_min_size(c + Vec2::new(-8.0 + i as f32 * 2.0, -3.0 + i as f32 * 2.0), Vec2::new(16.0 - i as f32 * 4.0, 2.0)), 0.0, Color32::WHITE); }
            if ui.interact(tr, ui.id().with("drop"), Sense::click()).clicked() { app.modal = Modal::Courses; }
        }
        let mr = Rect::from_min_size(Pos2::new(r.max.x - 24.0, r.min.y), Vec2::new(24.0, 30.0));
        for i in 0..3 { ui.painter().rect_filled(Rect::from_min_size(Pos2::new(mr.center().x - 2.0, mr.min.y + 5.0 + i as f32 * 8.0), Vec2::splat(5.0)), 0.0, Color32::WHITE); }
        if ui.interact(mr, ui.id().with("menu"), Sense::click()).clicked() { app.screen = Screen::Settings; }
        let br = Rect::from_min_size(Pos2::new(r.max.x - 72.0, r.min.y), Vec2::new(36.0, 30.0));
        draw_spr(ui.painter(), Pos2::new(br.min.x + 2.0, br.min.y + 1.0), Spr::Bell, 2.4, &[]);
        if !app.s.notif_read {
            ui.painter().rect_filled(Rect::from_min_size(Pos2::new(br.max.x - 10.0, br.min.y - 3.0), Vec2::splat(13.0)), 0.0, RED);
            ui.painter().text(Pos2::new(br.max.x - 3.5, br.min.y + 3.5), Align2::CENTER_CENTER, "1", FontId::new(8.0, press()), Color32::WHITE);
        }
        if ui.interact(br, ui.id().with("bell"), Sense::click()).clicked() { app.modal = Modal::Notif; app.s.notif_read = true; app.s.store(); }
    });
}

fn subtabs(ui: &mut egui::Ui, labels: &[&str], cur: &mut usize) {
    let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 38.0), Sense::hover());
    ui.painter().rect_filled(r, 0.0, HDR);
    let n = labels.len() as f32;
    for (i, lab) in labels.iter().enumerate() {
        let tr = Rect::from_min_size(Pos2::new(r.min.x + r.width() / n * i as f32, r.min.y), Vec2::new(r.width() / n, 38.0));
        let resp = ui.interact(tr, ui.id().with(("st", labels[0], i)), Sense::click());
        let on = *cur == i;
        ui.painter().text(tr.center() - Vec2::new(0.0, 2.0), Align2::CENTER_CENTER, *lab, FontId::new(9.0, press()), if on { Color32::WHITE } else if resp.hovered() { INK } else { DIM });
        if on { ui.painter().rect_filled(Rect::from_min_size(Pos2::new(tr.min.x, tr.max.y - 4.0), Vec2::new(tr.width(), 4.0)), 0.0, GOLD); }
        if resp.clicked() { *cur = i; }
    }
    ui.add_space(8.0);
}

fn nav(ctx: &mut egui::Ui, app: &mut App) {
    egui::Panel::bottom("nav").frame(egui::Frame::new().fill(Color32::from_rgb(0x1d, 0x16, 0x32)).inner_margin(egui::Margin { left: 4, right: 4, top: 6, bottom: 6 })).show(ctx, |ui| {
        let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 56.0), Sense::hover());
        let rr = ui.max_rect();
        ui.painter().rect_filled(Rect::from_min_size(Pos2::new(rr.min.x - 4.0, rr.min.y - 6.0), Vec2::new(rr.width() + 8.0, 3.0)), 0.0, GOLD);
        let items = [(Tab::Learn, "LEARN"), (Tab::Community, "TRAIN"), (Tab::Leader, "RANKS"), (Tab::Create, "CREATE"), (Tab::Profile, "HERO")];
        let w = r.width() / 5.0;
        for (i, (t, name)) in items.iter().enumerate() {
            let tr = Rect::from_min_size(Pos2::new(r.min.x + w * i as f32, r.min.y), Vec2::new(w, 56.0));
            let resp = ui.interact(tr, ui.id().with(("nav", i)), Sense::click());
            let on = app.tab == *t;
            if on {
                ui.painter().rect_filled(tr.shrink2(Vec2::new(3.0, 0.0)).translate(Vec2::new(0.0, 3.0)), 0.0, EDGE);
                ui.painter().rect_filled(tr.shrink2(Vec2::new(3.0, 0.0)), 0.0, Color32::from_rgb(0x3a, 0x2e, 0x6b));
                ui.painter().rect_stroke(tr.shrink2(Vec2::new(3.0, 0.0)), 0.0, Stroke::new(2.0, GOLD), egui::StrokeKind::Inside);
            }
            let art = match t { Tab::Learn => "book", Tab::Community => "sword", Tab::Leader => "shield", Tab::Create => "staff", Tab::Profile => "helmet" };
            draw_icon(ui.painter(), Rect::from_center_size(Pos2::new(tr.center().x, tr.min.y + 20.0), Vec2::splat(30.0)), art, if on { Color32::WHITE } else { Color32::from_gray(150) });
            ui.painter().text(Pos2::new(tr.center().x, tr.max.y - 9.0), Align2::CENTER_CENTER, *name, FontId::new(8.0, press()), if on { GOLD } else { DIM });
            if resp.clicked() { app.tab = *t; }
        }
    });
}

fn out_view(ui: &mut egui::Ui, out: &[(interp::Kind, String)]) {
    use interp::Kind::*;
    for (k, t) in out {
        match k {
            Out => { ui.label(RichText::new(format!("> {}", t)).monospace().size(14.0).color(GREEN)); }
            Err => { ui.label(RichText::new(format!("! {}", t)).monospace().size(13.0).color(RED)); }
            H1 => { ui.label(title(t.clone(), 15.0).color(PARCH)); ui.add_space(4.0); }
            H2 => { ui.label(title(t.clone(), 11.0).color(PARCH)); ui.add_space(3.0); }
            P => { ui.label(body(t.clone(), 15.0)); }
            Li => { ui.label(body(format!("  * {}", t), 15.0)); }
            Btn => { let _ = pbtn(ui, t, BLUE, 160.0); }
        }
    }
}

fn dim_back(ctx: &egui::Context) {
    let r = ctx.content_rect();
    egui::Area::new(egui::Id::new("dimmer")).order(egui::Order::Middle).fixed_pos(r.min).show(ctx, |ui| {
        ui.painter().rect_filled(r, 0.0, Color32::from_black_alpha(190));
        let _ = ui.allocate_rect(r, Sense::click());
    });
}

fn dialog(ctx: &egui::Context, id: &str, add: impl FnOnce(&mut egui::Ui)) {
    egui::Window::new(id).title_bar(false).collapsible(false).resizable(false).anchor(Align2::CENTER_CENTER, [0.0, 0.0]).order(egui::Order::Foreground)
        .fixed_size([340.0, 0.0])
        .frame(egui::Frame::new().fill(PANEL).stroke(Stroke::new(4.0, GOLD)).inner_margin(16.0).shadow(egui::epaint::Shadow { offset: [0, 6], blur: 0, spread: 0, color: EDGE }))
        .show(ctx, add);
}

fn option_btn(ui: &mut egui::Ui, text: &str, selected: bool, locked: bool, sel: &mut Option<usize>, i: usize) {
    let (r, resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 48.0), Sense::click());
    let fill = if selected { Color32::from_rgb(0x3a, 0x5a, 0xb8) } else if resp.hovered() { PANEL_HI } else { PANEL };
    bevel(ui.painter(), r, fill, true);
    if selected { ui.painter().rect_stroke(r.expand(2.0), 0.0, Stroke::new(3.0, GOLD), egui::StrokeKind::Outside); }
    ui.painter().text(Pos2::new(r.min.x + 16.0, r.center().y), Align2::LEFT_CENTER, text, FontId::new(16.0, egui::FontFamily::Proportional), INK);
    if resp.clicked() && !locked { *sel = Some(i); }
    ui.add_space(8.0);
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
            if resp.clicked() { *v = !*v; ch = true; }
        });
    });
    ui.add_space(6.0);
    ch
}

// ---------------- app glue ----------------
impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _f: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.tick();
        if self.modal == Modal::Courses { self.modal = Modal::None; self.tab = Tab::Community; self.sub_comm = 0; }
        let tnow = ctx.input(|i| i.time);
        let vkey = (self.tab as u32) * 10 + self.sub_comm as u32 + self.sub_create as u32 * 3;
        if vkey != self.view_key { self.view_key = vkey; self.view_t0 = tnow; }
        let fade_t = ((tnow - self.view_t0) / 0.18).clamp(0.0, 1.0);
        let alpha = (1.0 - (1.0 - fade_t).powi(3)) as f32;
        if fade_t < 1.0 { ctx.request_repaint(); }
        if let Some((_, t)) = &mut self.toast { *t -= ctx.input(|i| i.stable_dt) as f64; if *t <= 0.0 { self.toast = None; } ctx.request_repaint(); }
        let screen = std::mem::replace(&mut self.screen, Screen::Main);
        match screen {
            Screen::Main => {
                let (t, dd) = match self.tab { Tab::Learn => (self.courses[self.cur].name.to_string(), true), Tab::Community => ("Training".into(), false), Tab::Leader => ("Leaderboard".into(), false), Tab::Create => ("Create".into(), false), Tab::Profile => (self.s.name.clone(), false) };
                header(ui, self, &t, false, dd);
                nav(ui, self);
                let tab = self.tab;
                egui::CentralPanel::default().frame(egui::Frame::new().fill(BG).inner_margin(egui::Margin { left: 12, right: 12, top: 10, bottom: 0 })).show(ui, |ui| {
                    if self.s.floor { floor(ui.painter(), ui.max_rect(), self.s.scan); }
                    ui.set_opacity(alpha);
                    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| match tab {
                        Tab::Learn => self.learn(ui),
                        Tab::Community => self.community(ui),
                        Tab::Leader => self.leader(ui),
                        Tab::Create => self.create(ui),
                        Tab::Profile => self.profile(ui),
                    });
                });
            }
            Screen::Settings => { self.screen = Screen::Settings; self.settings(ui); }
            Screen::Run(rs) => { if let Some(rs) = self.run_screen(ui, rs) { self.screen = Screen::Run(rs); } else if matches!(self.screen, Screen::Run(_)) { } }
            Screen::Complete(c) => { if !self.complete_screen(ui, &c) { self.screen = Screen::Complete(c); } }
            Screen::Editor(ed) => { if let Some(ed) = self.editor_screen(ui, ed) { self.screen = Screen::Editor(ed); } }
            Screen::Post(i, out) => {
                self.screen = Screen::Post(i, out.clone());
                self.back_screen(ui, "Code", |app, ui| {
                    ui.label(title(app.posts[i].title, 12.0));
                    ui.label(body(format!("by {}   ({} views)", app.posts[i].author, app.posts[i].views), 13.0).color(BLUE));
                    ui.add_space(8.0);
                    panel(ui, Color32::from_rgb(0x0f, 0x0b, 0x1a), GREEN, |ui| { ui.set_width(ui.available_width()); ui.label(RichText::new(app.posts[i].code).monospace().size(14.0).color(PARCH)); });
                    panel(ui, PANEL, EDGE, |ui| { ui.set_width(ui.available_width()); ui.label(title("OUTPUT", 9.0).color(GOLD)); out_view(ui, &out); });
                    ui.horizontal(|ui| {
                        let voted = app.voted.contains(&i);
                        if pbtn(ui, if voted { "VOTED +1" } else { "UPVOTE" }, if voted { GOLD } else { PANEL_HI }, 130.0).clicked() { if voted { app.voted.remove(&i); } else { app.voted.insert(i); } }
                        if pbtn(ui, "OPEN IN EDITOR", GREEN, 190.0).clicked() { app.screen = Screen::Editor(Editor { idx: None, title: format!("Copy of {}", app.posts[i].title), lang: app.posts[i].lang.into(), code: app.posts[i].code.into(), out: vec![] }); }
                    });
                });
            }
            Screen::Thread(i) => {
                self.screen = Screen::Thread(i);
                self.back_screen(ui, "Discuss", |app, ui| {
                    ui.label(title(app.threads[i].title, 12.0));
                    ui.label(body(format!("by {}   {} ago", app.threads[i].author, app.threads[i].when), 13.0).color(BLUE));
                    ui.add_space(10.0);
                    ui.label(title("ANSWERS", 10.0).color(GOLD));
                    ui.add_space(6.0);
                    let mut all: Vec<String> = app.threads[i].answers.iter().map(|s| s.to_string()).collect();
                    all.extend(app.extra_answers.iter().filter(|(t, _)| *t == i).map(|(_, s)| s.clone()));
                    for a in all { panel(ui, PANEL, EDGE, |ui| { ui.set_width(ui.available_width()); ui.label(body(a, 15.0)); }); }
                    ui.add_space(6.0);
                    ui.add(egui::TextEdit::multiline(&mut app.input).hint_text("Write an answer...").desired_width(f32::INFINITY).desired_rows(3));
                    if pbtn(ui, "POST ANSWER", GREEN, 0.0).clicked() && !app.input.trim().is_empty() {
                        let t = std::mem::take(&mut app.input);
                        app.extra_answers.push((i, t));
                        app.feed.push("answered a question".into());
                    }
                });
            }
            Screen::NewThread => {
                self.screen = Screen::NewThread;
                self.back_screen(ui, "Ask", |app, ui| {
                    ui.label(body("Question title", 14.0));
                    ui.add(egui::TextEdit::singleline(&mut app.input).desired_width(f32::INFINITY));
                    ui.label(body("Tag", 14.0));
                    ui.add(egui::TextEdit::singleline(&mut app.input2).desired_width(f32::INFINITY));
                    ui.add_space(8.0);
                    if pbtn(ui, "POST QUESTION", GREEN, 0.0).clicked() && !app.input.trim().is_empty() {
                        let t: &'static str = Box::leak(std::mem::take(&mut app.input).into_boxed_str());
                        let tg = if app.input2.trim().is_empty() { "general".to_string() } else { app.input2.trim().to_lowercase() };
                        let tag: &'static str = Box::leak(tg.into_boxed_str());
                        app.threads.insert(0, Thread { title: t, tags: vec![tag], votes: 0, answers: vec![], author: "You", when: "now", views: 1 });
                        app.extra_answers.iter_mut().for_each(|(i, _)| *i += 1);
                        app.feed.push("asked a question".into());
                        app.screen = Screen::Main;
                    }
                });
            }
        }
        // modals
        if self.modal != Modal::None {
            dim_back(&ctx);
            let mut close = false;
            match self.modal {
                Modal::Notif => dialog(&ctx, "notif", |ui| {
                    ui.label(title("NOTIFICATIONS", 11.0).color(GOLD)); ui.add_space(8.0);
                    ui.label(body("Welcome to Pixel Learn! Finish a lesson to start your streak.", 14.0)); ui.add_space(4.0);
                    ui.label(body(format!("You are level {} with {} XP.", self.level(), self.s.xp), 14.0)); ui.add_space(10.0);
                    if pbtn(ui, "CLOSE", BLUE, 0.0).clicked() { close = true; }
                }),
                Modal::Shop => dialog(&ctx, "shop", |ui| {
                    ui.horizontal(|ui| { ui.label(title("SHOP", 12.0).color(GOLD)); ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { ui.label(title(format!("{}", self.s.gems), 11.0)); spr(ui, Spr::Gem, 2.0, &[]); }); });
                    ui.add_space(10.0);
                    ui.horizontal(|ui| { spr(ui, Spr::Heart, 3.0, &[]); ui.label(body(format!("Refill all hearts  ({} gems)", REFILL_COST), 14.0)); });
                    if pbtn(ui, "BUY HEARTS", RED, 0.0).clicked() {
                        if self.s.hearts >= MAX_HEARTS { self.say("Hearts already full"); } else if self.s.gems < REFILL_COST { self.say("Not enough gems"); } else { self.s.gems -= REFILL_COST; self.s.hearts = MAX_HEARTS; self.s.store(); self.say("Hearts refilled!"); }
                    }
                    if pbtn(ui, if self.s.free_day == today() { "FREE HEART (TOMORROW)" } else { "FREE HEART (1 PER DAY)" }, GREEN, 0.0).clicked() {
                        if self.s.free_day == today() { self.say("Come back tomorrow"); } else if self.s.hearts >= MAX_HEARTS { self.say("Hearts already full"); } else { self.s.free_day = today(); self.s.hearts += 1; self.s.store(); self.say("+1 heart!"); }
                    }
                    ui.add_space(10.0);
                    ui.horizontal(|ui| { spr(ui, Spr::Bolt, 3.0, &[('a', BLUE)]); ui.label(body(format!("Streak saver  ({} gems, max 3)", SAVER_COST), 14.0)); });
                    if pbtn(ui, "BUY SAVER", BLUE, 0.0).clicked() {
                        if self.s.savers >= 3 { self.say("You hold the maximum"); } else if self.s.gems < SAVER_COST { self.say("Not enough gems"); } else { self.s.gems -= SAVER_COST; self.s.savers += 1; self.s.store(); self.say("Streak saver added!"); }
                    }
                    ui.add_space(10.0);
                    if pbtn(ui, "CLOSE", PANEL_HI, 0.0).clicked() { close = true; }
                }),
                Modal::Courses => dialog(&ctx, "courses", |ui| {
                    ui.label(title("CHOOSE A COURSE", 11.0).color(GOLD)); ui.add_space(8.0);
                    for ci in 0..self.courses.len() {
                        ui.horizontal(|ui| {
                            emblem(ui, self.courses[ci].tag, self.courses[ci].color, 44.0);
                            if pbtn(ui, self.courses[ci].tag, if ci == self.cur { GOLD } else { PANEL_HI }, ui.available_width()).clicked() { self.cur = ci; self.tab = Tab::Learn; self.open_mod = self.current_module(ci); close = true; }
                        });
                        ui.add_space(4.0);
                    }
                    if pbtn(ui, "CLOSE", PANEL_HI, 0.0).clicked() { close = true; }
                }),
                Modal::NoHearts => dialog(&ctx, "nohearts", |ui| {
                    ui.vertical_centered(|ui| {
                        spr(ui, Spr::Heart, 8.0, &[('r', Color32::from_rgb(0x55, 0x4a, 0x70)), ('R', Color32::from_rgb(0x3a, 0x32, 0x50))]);
                        ui.label(title("OUT OF HEARTS", 12.0).color(RED)); ui.add_space(6.0);
                        ui.label(body("Hearts refill 1 every 5 minutes, or buy a refill with gems.", 14.0)); ui.add_space(8.0);
                    });
                    if pbtn(ui, &format!("REFILL ({} GEMS)", REFILL_COST), RED, 0.0).clicked() {
                        if self.s.gems >= REFILL_COST { self.s.gems -= REFILL_COST; self.s.hearts = MAX_HEARTS; self.s.store(); close = true; } else { self.say("Not enough gems"); }
                    }
                    ui.add_space(6.0);
                    if pbtn(ui, "LATER", PANEL_HI, 0.0).clicked() { close = true; }
                }),
                Modal::Reset => dialog(&ctx, "reset", |ui| {
                    ui.label(title("RESET PROGRESS?", 11.0).color(RED)); ui.add_space(6.0);
                    ui.label(body("This erases XP, gems, streak and finished lessons.", 14.0)); ui.add_space(8.0);
                    if pbtn(ui, "YES, RESET", RED, 0.0).clicked() { let (n, c) = (self.s.name.clone(), self.s.country.clone()); self.s = Save::new(); self.s.heart_t = now(); self.s.name = n; self.s.country = c; self.s.store(); self.open_mod = self.current_module(self.cur); close = true; }
                    ui.add_space(6.0);
                    if pbtn(ui, "CANCEL", PANEL_HI, 0.0).clicked() { close = true; }
                }),
                Modal::None => {}
            }
            if close { self.modal = Modal::None; }
        }
        if let Some((t, _)) = &self.toast {
            let r = ctx.content_rect();
            let t = t.clone();
            egui::Area::new(egui::Id::new("toast")).order(egui::Order::Tooltip).fixed_pos(Pos2::new(r.center().x - 130.0, r.max.y - 130.0)).show(&ctx, |ui| {
                let (rr, _) = ui.allocate_exact_size(Vec2::new(260.0, 38.0), Sense::hover());
                bevel(ui.painter(), rr, Color32::from_rgb(0x1f, 0x1a, 0x35), false);
                ui.painter().rect_stroke(rr, 0.0, Stroke::new(2.0, GOLD), egui::StrokeKind::Inside);
                ui.painter().text(rr.center(), Align2::CENTER_CENTER, t, FontId::new(15.0, egui::FontFamily::Proportional), INK);
            });
        }
        if let Some(path) = self.shot.clone() {
            self.frames += 1;
            ctx.request_repaint();
            if self.frames == 120 {
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::ScreenshotCallback::new(move |img| {
                    let (w, h) = (img.width() as u32, img.height() as u32);
                    let raw: Vec<u8> = img.pixels.iter().flat_map(|c| c.to_array()).collect();
                    if let Some(b) = image::RgbaImage::from_raw(w, h, raw) { let _ = b.save(&path); }
                    std::process::exit(0);
                })));
            }
        }
    }
}

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    let shot = args.iter().position(|a| a == "--screenshot").and_then(|i| args.get(i + 1).cloned());
    let args2 = args.clone();
    let tab = args.iter().position(|a| a == "--tab").and_then(|i| args.get(i + 1).cloned());
    let mut vp = egui::ViewportBuilder::default().with_inner_size([440.0, 780.0]).with_min_inner_size([360.0, 600.0]).with_title("Pixel Learn - RPG");
    if shot.is_some() { vp = vp.with_active(false).with_window_level(egui::WindowLevel::AlwaysOnBottom); }
    eframe::run_native("Pixel Learn", eframe::NativeOptions { viewport: vp, ..Default::default() }, Box::new(move |cc| {
        install_fonts(&cc.egui_ctx);
        Box::leak(Box::new(load_textures(&cc.egui_ctx)));
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.global_style_mut(|s| {
            s.visuals.panel_fill = BG;
            s.visuals.window_fill = PANEL;
            s.visuals.override_text_color = Some(INK);
            s.visuals.extreme_bg_color = Color32::from_rgb(0x0f, 0x0b, 0x1a);
            s.visuals.selection.bg_fill = Color32::from_rgb(0x3a, 0x5a, 0xb8);
            s.visuals.widgets.inactive.bg_stroke = Stroke::new(2.0, EDGE);
            for w in [&mut s.visuals.widgets.noninteractive, &mut s.visuals.widgets.inactive, &mut s.visuals.widgets.hovered, &mut s.visuals.widgets.active, &mut s.visuals.widgets.open] { w.corner_radius = 0.0.into(); }
            s.visuals.widgets.inactive.weak_bg_fill = PANEL_HI;
            s.spacing.item_spacing = Vec2::new(8.0, 6.0);
            s.visuals.window_corner_radius = 0.0.into();
        });
        let mut app = App::new(shot);
        if let Some(t) = tab { app.tab = match t.as_str() { "community" => Tab::Community, "leader" => Tab::Leader, "create" => Tab::Create, "profile" => Tab::Profile, _ => Tab::Learn }; }
        // preview helpers: --lesson C.M.L [--stage N] [--skip N] opens a lesson directly (used for screenshots)
        if let Some(spec) = args2.iter().position(|a| a == "--lesson").and_then(|i| args2.get(i + 1).cloned()) {
            let p: Vec<usize> = spec.split('.').filter_map(|x| x.parse().ok()).collect();
            if p.len() == 3 && p[0] < app.courses.len() {
                let l = app.courses[p[0]].modules[p[1]].lessons[p[2]].clone();
                app.start_run(l.title, Kind::Lesson(p[0], p[1], p[2]), l.slides.clone(), l.qs.clone());
                let num = |k: &str| args2.iter().position(|a| a == k).and_then(|i| args2.get(i + 1)).and_then(|v| v.parse::<usize>().ok()).unwrap_or(0);
                if let Screen::Run(rs) = &mut app.screen { rs.stage = num("--stage"); for _ in 0..num("--skip") { if !rs.queue.is_empty() { rs.queue.remove(0); } } }
            }
        }
        Ok(Box::new(app))
    }))
}
