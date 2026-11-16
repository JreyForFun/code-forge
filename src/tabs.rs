use super::*;

fn locked_grey() -> Color32 { Color32::from_rgb(0x47, 0x55, 0x69) }

impl App {
    // ------------------------------------------------------------ LEARN
    pub fn learn(&mut self, ui: &mut egui::Ui) {
        let c = self.cur;
        self.quest_card(ui, c);
        stats_row(ui, self, true);
        ui.add_space(SP_S);
        if let Some((mi, li)) = self.next_lesson(c) {
            let name = self.courses[c].modules[mi].lessons[li].title;
            let resumed = self.s.qdone.get(&self.lkey(c, mi, li)).map(|d| !d.is_empty()).unwrap_or(false);
            if pbtn(ui, &format!("{}: {}", if resumed { "RESUME" } else { "NEXT QUEST" }, name.to_uppercase()), GOLD, 0.0).clicked() { self.open_lesson(c, mi, li); }
            ui.add_space(SP_M);
        }
        for mi in 0..self.courses[c].modules.len() {
            let done = self.module_done(c, mi);
            let unlocked = (0..mi).all(|m| self.module_done(c, m));
            let open = self.open_mod == Some(mi);
            let (r, resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 62.0), Sense::click());
            let resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand);
            let p = ui.painter();
            p.add(frame_shapes(r, if open || resp.hovered() { PANEL_HI } else { PANEL }, if open { GOLD } else if done { GREEN } else { EDGE }));
            let ir = Rect::from_min_size(r.min + Vec2::new(11.0, 10.0), Vec2::splat(42.0));
            if done { bevel(p, ir, Color32::from_rgb(0x2a, 0x6a, 0x22), false); draw_spr(p, ir.min + Vec2::new(9.0, 9.0), Spr::Check, 2.0, &[('w', Color32::WHITE)]); }
            else if unlocked { bevel(p, ir, Color32::from_rgb(0x8a, 0x6a, 0x1c), false); p.text(ir.center(), Align2::CENTER_CENTER, format!("{}", mi + 1), FontId::new(14.0, press()), Color32::WHITE); }
            else { p.rect_filled(ir, 0.0, EDGE); draw_icon(p, ir, "door", Color32::from_gray(170)); }
            let total = self.courses[c].modules[mi].lessons.len();
            let cleared = (0..total).filter(|l| self.is_done(c, mi, *l)).count();
            p.text(Pos2::new(r.min.x + 66.0, r.center().y - 8.0), Align2::LEFT_CENTER, self.courses[c].modules[mi].name, FontId::new(9.0, press()), if unlocked { INK } else { DIM });
            p.text(Pos2::new(r.min.x + 66.0, r.center().y + 12.0), Align2::LEFT_CENTER, format!("{} / {} lessons", cleared, total), FontId::new(12.0, egui::FontFamily::Proportional), DIM);
            let ax = r.max.x - 24.0;
            for i in 0..4 {
                let w = 16.0 - 4.0 * i as f32;
                let y = if open { r.center().y + 4.0 - 2.0 * i as f32 } else { r.center().y - 4.0 + 2.0 * i as f32 };
                p.rect_filled(Rect::from_min_size(Pos2::new(ax - w / 2.0 + 8.0, y), Vec2::new(w, 2.0)), 0.0, DIM);
            }
            if resp.clicked() { self.open_mod = if open { None } else { Some(mi) }; }
            ui.add_space(SP_S);
            if open {
                let blurb = self.courses[c].modules[mi].desc;
                if !blurb.is_empty() { ui.label(body(format!("  {}", blurb), 13.0).color(DIM)); ui.add_space(SP_S); }
                if !unlocked { ui.label(body("  Clear the previous chapter to unlock.", 13.0).color(DIM)); ui.add_space(SP_S); }
                panel(ui, Color32::from_rgb(0x14, 0x12, 0x20), EDGE, |ui| {
                ui.set_width(ui.available_width());
                for li in 0..total {
                    let ld = self.is_done(c, mi, li);
                    let lu = self.lesson_unlocked(c, mi, li);
                    ui.horizontal(|ui| {
                        let tl = self.courses[c].modules[mi].lessons[li].title;
                        ui.vertical(|ui| {
                            ui.label(body(format!("Lesson {}", li + 1), 12.0).color(DIM));
                            ui.horizontal(|ui| {
                                ui.label(body(if lu { tl } else { "???" }, 15.0).color(if lu { INK } else { DIM }));
                                if self.courses[c].modules[mi].lessons[li].project { tag(ui, "Project", ORANGE); }
                            });
                            let key = self.lkey(c, mi, li);
                            if !ld && let Some(d) = self.s.qdone.get(&key) && !d.is_empty() { ui.label(body(format!("in progress {}/{}", d.len(), self.courses[c].modules[mi].lessons[li].qs.len()), 11.0).color(GOLD)); }
                            if let Some((b, a)) = self.s.best.get(&key) { ui.label(body(format!("best {}%  -  played {}x", b, a), 11.0).color(DIM)); }
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if lu {
                                if pbtn(ui, if ld { "REPLAY" } else { "START" }, if ld { locked_grey() } else { BLUE }, 92.0).clicked() { self.open_lesson(c, mi, li); }
                                if ld { spr(ui, Spr::Check, 2.0, &[('w', GREEN)]); }
                            } else { icon(ui, "keyhole", 28.0); }
                        });
                    });
                    if li + 1 < total {
                        ui.add_space(SP_XS);
                        let (sr, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 2.0), Sense::hover());
                        ui.painter().rect_filled(sr, 0.0, Color32::from_white_alpha(12));
                        ui.add_space(SP_XS);
                    }
                }
                });
            }
        }
        ui.add_space(SP_S);
        let prog = self.progress(c);
        panel(ui, PANEL, if prog >= 1.0 { GOLD } else { EDGE }, |ui| {
            ui.set_width(ui.available_width());
            ui.vertical_centered(|ui| {
                let (r, _) = ui.allocate_exact_size(Vec2::splat(88.0), Sense::hover());
                draw_badge(ui.painter(), r, &self.courses[c], self.badge_tier(c));
                ui.label(title(format!("{} BADGE", self.courses[c].badge.name.to_uppercase()), 10.0).color(if prog >= 1.0 { GOLD } else { INK }));
                ui.add_space(SP_XS);
                let left = self.courses[c].lesson_count() - (prog * self.courses[c].lesson_count() as f32).round() as usize;
                ui.label(body(if prog >= 1.0 { "You cleared every quest in this course.".to_string() } else { format!("{} lesson{} to go. Clear them all to earn this badge.", left, if left == 1 { "" } else { "s" }) }, 13.0).color(DIM));
            });
        });
        ui.add_space(SP_S);
        section(ui, "MORE COURSES");
        for ci in 0..self.courses.len() {
            if ci == c { continue; }
            let p = self.progress(ci);
            let resp = panel_resp(ui, PANEL, EDGE, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    emblem(ui, self.courses[ci].tag, self.courses[ci].color, 56.0);
                    ui.vertical(|ui| {
                        ui.label(title(self.courses[ci].name, 8.0));
                        ui.label(body(self.courses[ci].desc, 13.0).color(DIM));
                        ui.horizontal(|ui| { ui.allocate_ui(Vec2::new((ui.available_width() - 48.0).max(40.0), 12.0), |ui| bar(ui, p, GREEN, 10.0)); ui.label(body(format!("{}%", (p * 100.0) as u32), 13.0)); });
                    });
                });
            });
            if resp.on_hover_text("Course details").clicked() { self.screen = Screen::Preview(ci); }
        }
        ui.add_space(SP_L);
    }

    // ------------------------------------------------------------ TRAIN
    pub fn train(&mut self, ui: &mut egui::Ui) {
        subtabs(ui, &["COURSES", "PRACTICE", "LOG"], &mut self.sub_train);
        ui.add_space(SP_XS);
        match self.sub_train { 0 => self.courses_tab(ui), 1 => self.practice(ui), _ => self.log_tab(ui) }
    }

    fn practice(&mut self, ui: &mut egui::Ui) {
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| { icon(ui, "scroll_lightning", 32.0); ui.vertical(|ui| { ui.label(title("Quick Challenges", 11.0)); ui.label(body("Short puzzles: +8 XP, +2 gems. Wrong answers cost hearts.", 13.0).color(DIM)); }); });
            ui.add_space(SP_S);
            egui::ScrollArea::horizontal().id_salt("coach").show(ui, |ui| {
                ui.horizontal(|ui| {
                    for c in coaches() {
                        ui.vertical(|ui| {
                            ui.set_width(110.0);
                            let (r, resp) = ui.allocate_exact_size(Vec2::splat(104.0), Sense::click());
                            bevel(ui.painter(), r, if resp.hovered() { shade(c.color, 1.15) } else { c.color }, true);
                            draw_icon(ui.painter(), r.shrink(18.0), c.art, Color32::WHITE);
                            ui.label(body(c.title, 13.0));
                            let lc = match c.level { "Easy" => GREEN, "Hard" => RED, _ => BLUE };
                            let (lr, _) = ui.allocate_exact_size(Vec2::new(70.0, 18.0), Sense::hover());
                            ui.painter().rect_filled(lr, 0.0, lc);
                            ui.painter().text(lr.center(), Align2::CENTER_CENTER, c.level, FontId::new(8.0, press()), Color32::WHITE);
                            if resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() { self.start_run(c.title, Kind::Coach, vec![], c.qs.clone()); }
                        });
                        ui.add_space(SP_S);
                    }
                });
            });
        });
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| { icon(ui, "axe", 32.0); ui.vertical(|ui| { ui.label(title("Duel Arena", 11.0)); ui.label(body("3 quick questions against a computer opponent. Duels never cost hearts.", 13.0).color(DIM)); }); });
            ui.add_space(SP_S);
            let ow = ((ui.available_width() - 24.0) / 3.0).max(60.0);
            let opps = opponents();
            ui.horizontal(|ui| {
                for (i, o) in opps.iter().enumerate() {
                    let sel = self.opp == i;
                    ui.vertical(|ui| {
                        ui.set_width(ow);
                        let (r, resp) = ui.allocate_exact_size(Vec2::new(ow, 64.0), Sense::click());
                        bevel(ui.painter(), r, if sel { o.color } else if resp.hovered() { PANEL_HI } else { PANEL }, false);
                        draw_icon(ui.painter(), r.shrink(10.0), o.art, Color32::WHITE);
                        if sel { ui.painter().rect_stroke(r.expand(3.0), 0.0, Stroke::new(2.0, GOLD), egui::StrokeKind::Outside); }
                        ui.label(body(o.name, 12.0).color(if sel { GOLD } else { DIM }));
                        ui.label(body(format!("Skill {}", "*".repeat(o.skill as usize)), 11.0).color(DIM));
                        if resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() { self.opp = i; }
                    });
                }
            });
            ui.add_space(SP_S);
            let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 96.0), Sense::hover());
            let p = ui.painter();
            p.rect_filled(r, 0.0, Color32::from_rgb(0x3a, 0x6c, 0xd0));
            p.add(egui::Shape::convex_polygon(vec![Pos2::new(r.center().x + 20.0, r.min.y), r.right_top(), r.right_bottom(), Pos2::new(r.center().x - 30.0, r.max.y)], opps[self.opp].color, Stroke::NONE));
            draw_avatar(p, Rect::from_min_size(Pos2::new(r.min.x + 20.0, r.min.y + 14.0), Vec2::splat(68.0)), self.s.avatar);
            p.text(r.center(), Align2::CENTER_CENTER, "VS", FontId::new(20.0, press()), Color32::WHITE);
            draw_icon(p, Rect::from_min_size(Pos2::new(r.max.x - 88.0, r.min.y + 14.0), Vec2::splat(68.0)), opps[self.opp].art, Color32::WHITE);
            ui.add_space(SP_S);
            if pbtn(ui, "START DUEL", RED, 0.0).clicked() {
                let o = self.opp;
                self.start_run(&format!("Duel vs {}", opps[o].name), Kind::Duel(o), vec![], duel_questions(today() * 3 + o as u64 + self.s.duel_wins as u64));
            }
        });
        ui.add_space(SP_M);
    }

    fn log_tab(&mut self, ui: &mut egui::Ui) {
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                let (r, _) = ui.allocate_exact_size(Vec2::splat(52.0), Sense::hover());
                draw_avatar(ui.painter(), r, self.s.avatar);
                ui.vertical(|ui| { ui.label(title(self.s.name.clone(), 10.0)); ui.label(body("Write a note in your adventure log", 13.0).color(BLUE)); });
            });
            ui.add_space(SP_S);
            ui.horizontal(|ui| {
                let r = ui.add(egui::TextEdit::singleline(&mut self.input).hint_text("Today I learned...").char_limit(140).desired_width((ui.available_width() - 104.0).max(60.0)));
                let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                if (pbtn(ui, "ADD", GREEN, 92.0).clicked() || enter) && !self.input.trim().is_empty() {
                    let t = std::mem::take(&mut self.input);
                    self.log(format!("noted: {}", t.trim()));
                    self.persist();
                }
            });
        });
        section(ui, "ADVENTURE LOG");
        if self.s.log.is_empty() {
            ui.add_space(30.0);
            ui.vertical_centered(|ui| { icon(ui, "book", 64.0); ui.label(title("Your story starts here", 10.0).color(DIM)); ui.label(body("Clear a lesson to write the first page.", 13.0).color(DIM)); });
        }
        let mut last_day = None;
        for (d, t) in self.s.log.clone().iter().rev() {
            if last_day != Some(*d) { ui.label(body(day_label(*d), 12.0).color(GOLD)); last_day = Some(*d); }
            panel(ui, PANEL, EDGE, |ui| { ui.set_width(ui.available_width()); ui.horizontal_wrapped(|ui| { icon(ui, if t.starts_with("noted") { "letter" } else if t.contains("achievement") { "medallion" } else if t.contains("duel") { "axe" } else if t.contains("league") { "banner_royal" } else { "crystal_blue" }, 26.0); ui.label(body(format!("{} {}", self.s.name, t), 14.0)); }); });
        }
        ui.add_space(SP_M);
    }

    // ------------------------------------------------------------ RANKS
    pub fn ranks(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            chip(ui, "flame", game::live_streak(&self.s, today()) == 0, &game::live_streak(&self.s, today()).to_string());
            chip(ui, "crystal_purple", false, &self.s.gems.to_string());
            chip(ui, "scroll_lightning", false, &format!("{} XP", self.s.week_xp));
        });
        ui.add_space(SP_M);
        let league = (self.s.league as usize).min(6);
        let cols = [Color32::from_rgb(0x8a, 0x5a, 0x32), Color32::from_rgb(0x6a, 0x6c, 0x80), Color32::from_rgb(0xc0, 0x80, 0x40), Color32::from_rgb(0x8a, 0x94, 0xa8), Color32::from_rgb(0xc8, 0xcc, 0xd8), GOLD, TEAL];
        let w = ((ui.available_width() - 6.0 * 6.0) / 7.0).clamp(30.0, 52.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            for i in 0..7 {
                let (r, resp) = ui.allocate_exact_size(Vec2::splat(w), Sense::hover());
                let _ = resp.on_hover_text(format!("{} league", LEAGUES[i]));
                if i <= league {
                    bevel(ui.painter(), r, cols[i], false);
                    if i == league { ui.painter().rect_stroke(r.expand(3.0), 0.0, Stroke::new(2.0, Color32::WHITE), egui::StrokeKind::Outside); }
                    draw_icon(ui.painter(), r.shrink(4.0), if i == 6 { "crown" } else { "banner" }, Color32::WHITE);
                } else {
                    bevel(ui.painter(), r, Color32::from_rgb(0x30, 0x2a, 0x4a), false);
                    draw_icon(ui.painter(), r.shrink(6.0), "keyhole", Color32::from_gray(110));
                }
            }
        });
        ui.add_space(SP_S);
        ui.vertical_centered(|ui| ui.label(title(format!("{} League", LEAGUES[league]), 12.0).color(cols[league])));
        ui.add_space(SP_XS);
        ui.vertical_centered(|ui| ui.label(body("You compete with computer rivals. The week resets every Monday.", 12.0).color(DIM)));
        ui.add_space(SP_M);
        if self.s.week_xp == 0 {
            ui.add_space(SP_XL);
            ui.vertical_centered(|ui| {
                let (r, _) = ui.allocate_exact_size(Vec2::new(140.0, 120.0), Sense::hover());
                draw_icon(ui.painter(), r, "chalice", Color32::WHITE);
                ui.add_space(SP_M);
                ui.label(title("Earn XP to join this week's table!", 10.0));
                ui.add_space(SP_L);
                if pbtn(ui, "START LEARNING", BLUE, 0.0).clicked() { self.tab = Tab::Learn; }
            });
            return;
        }
        let wd = game::weekday(today());
        let mut rows: Vec<(String, u32, Option<&'static str>)> = RIVALS.iter().map(|r| (r.name.to_string(), rival_xp(r, self.s.league, wd), Some(r.art))).collect();
        rows.push((self.s.name.clone(), self.s.week_xp, None));
        rows.sort_by(|a, b| b.1.cmp(&a.1).then(a.2.is_some().cmp(&b.2.is_some())));
        let medal = [GOLD, Color32::from_rgb(0xc8, 0xcc, 0xd8), Color32::from_rgb(0xc0, 0x80, 0x40)];
        for (i, (n, x, art)) in rows.iter().enumerate() {
            let me = art.is_none();
            panel(ui, if me { Color32::from_rgb(0x4a, 0x3a, 0x1a) } else { PANEL }, if me { GOLD } else { EDGE }, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    let (r, _) = ui.allocate_exact_size(Vec2::splat(30.0), Sense::hover());
                    bevel(ui.painter(), r, if i < 3 { medal[i] } else { PANEL_HI }, false);
                    ui.painter().text(r.center(), Align2::CENTER_CENTER, format!("{}", i + 1), FontId::new(10.0, press()), if i < 3 { EDGE } else { INK });
                    let (ar, _) = ui.allocate_exact_size(Vec2::splat(32.0), Sense::hover());
                    match art { Some(a) => draw_icon(ui.painter(), ar, a, Color32::WHITE), None => draw_avatar(ui.painter(), ar, self.s.avatar) }
                    ui.vertical(|ui| {
                        ui.label(body(n.clone(), 15.0));
                        ui.label(body(if me { "you" } else { "computer rival" }, 11.0).color(if me { GOLD } else { DIM }));
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { ui.label(title(format!("{} XP", x), 9.0).color(GOLD)); });
                });
            });
        }
        ui.add_space(SP_XS);
        let up = if self.s.league >= 6 { "You are in the top league. Stay active to keep your place.".to_string() } else { format!("Earn {} XP this week to reach {} ({} so far). Earn none and you drop a league.", LEAGUE_UP_XP, LEAGUES[(league + 1).min(6)], self.s.week_xp) };
        ui.label(body(up, 12.0).color(DIM));
        bar(ui, self.s.week_xp as f32 / LEAGUE_UP_XP as f32, GOLD, 10.0);
        ui.add_space(SP_M);
    }

    // ------------------------------------------------------------ CREATE
    pub fn create(&mut self, ui: &mut egui::Ui) {
        subtabs(ui, &["MY SPELLS", "EXAMPLES"], &mut self.sub_create);
        if self.sub_create == 0 {
            if self.s.codes.is_empty() {
                ui.add_space(SP_XL);
                ui.vertical_centered(|ui| {
                    icon(ui, "staff", 72.0);
                    ui.label(title("No spells yet", 11.0).color(DIM));
                    ui.add_space(SP_XS);
                    ui.label(body("Write and run your own Python, JavaScript, HTML, CSS or SQL. Tap + to start, or copy an example.", 14.0).color(DIM));
                });
            }
            for i in 0..self.s.codes.len() {
                let (t, lg, code) = self.s.codes[i].clone();
                let resp = panel_resp(ui, PANEL, EDGE, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        lang_tile(ui, &lg);
                        ui.vertical(|ui| {
                            ui.label(body(t, 16.0));
                            ui.label(body(format!("{}  -  {} lines", lg, code.lines().count()), 12.0).color(BLUE));
                        });
                    });
                });
                if resp.clicked() { self.screen = Screen::Editor(Editor { idx: Some(i), title: self.s.codes[i].0.clone(), lang: lg, code, out: vec![] }); }
            }
            ui.add_space(70.0);
        } else {
            ui.label(body("Sample programs to read, run and change. Opening one makes your own copy.", 13.0).color(DIM));
            ui.add_space(SP_S);
            for e in EXAMPLES {
                let resp = panel_resp(ui, PANEL, EDGE, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        lang_tile(ui, e.lang);
                        ui.vertical(|ui| { ui.label(body(e.title, 16.0)); ui.label(body(e.desc, 12.0).color(DIM)); });
                    });
                });
                if resp.clicked() { self.screen = Screen::Editor(Editor { idx: None, title: e.title.into(), lang: e.lang.into(), code: e.code.into(), out: interp::run(e.lang, e.code) }); }
            }
            ui.add_space(70.0);
        }
        let r = ui.clip_rect();
        let fr = Rect::from_center_size(Pos2::new(r.max.x - 44.0, r.max.y - 44.0), Vec2::splat(54.0));
        let resp = ui.interact(fr, ui.scope_id().with("fab"), Sense::click()).on_hover_text("New spell").on_hover_cursor(egui::CursorIcon::PointingHand);
        bevel(ui.painter(), fr, if resp.hovered() { shade(GREEN, 1.15) } else { GREEN }, true);
        ui.painter().rect_filled(Rect::from_center_size(fr.center(), Vec2::new(24.0, 6.0)), 0.0, Color32::WHITE);
        ui.painter().rect_filled(Rect::from_center_size(fr.center(), Vec2::new(6.0, 24.0)), 0.0, Color32::WHITE);
        if resp.clicked() { self.screen = Screen::Editor(Editor { idx: None, title: "Untitled".into(), lang: "python".into(), code: data::starter("python").into(), out: vec![] }); }
    }

    // ------------------------------------------------------------ HERO
    pub fn hero(&mut self, ui: &mut egui::Ui) {
        let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 232.0), Sense::hover());
        let p = ui.painter();
        p.rect_filled(r, 0.0, HDR);
        for i in 0..20 { p.rect_filled(Rect::from_min_size(Pos2::new(r.min.x + (i as f32 * 53.0) % r.width(), r.min.y + (i as f32 * 37.0) % 120.0), Vec2::splat(3.0)), 0.0, Color32::from_rgb(0x6f, 0x8a, 0xf0)); }
        let ar = Rect::from_center_size(Pos2::new(r.center().x, r.min.y + 70.0), Vec2::splat(112.0));
        draw_avatar(p, ar, self.s.avatar);
        let aresp = ui.interact(ar, ui.scope_id().with("ava"), Sense::click()).on_hover_text("Change avatar").on_hover_cursor(egui::CursorIcon::PointingHand);
        if aresp.hovered() { ui.painter().rect_stroke(ar.expand(3.0), 0.0, Stroke::new(2.0, GOLD), egui::StrokeKind::Outside); }
        if aresp.clicked() { self.modal = Modal::Avatar; }
        let p = ui.painter();
        p.text(Pos2::new(r.center().x, r.min.y + 150.0), Align2::CENTER_CENTER, self.s.name.as_str(), FontId::new(14.0, press()), Color32::WHITE);
        p.text(Pos2::new(r.center().x, r.min.y + 176.0), Align2::CENTER_CENTER, format!("{}  *  Lvl {}  *  {} League", avatar(self.s.avatar).name, self.level(), LEAGUES[self.s.league as usize]), FontId::new(8.0, press()), PARCH);
        p.text(Pos2::new(r.center().x, r.min.y + 206.0), Align2::CENTER_CENTER, format!("{} XP", self.s.xp), FontId::new(12.0, press()), GOLD);
        let er = Rect::from_center_size(Pos2::new(r.max.x - 34.0, r.min.y + 30.0), Vec2::new(54.0, 34.0));
        let eresp = ui.interact(er, ui.scope_id().with("edit"), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
        bevel(ui.painter(), er, if eresp.hovered() { shade(GOLD, 1.1) } else { GOLD }, false);
        ui.painter().text(er.center(), Align2::CENTER_CENTER, "EDIT", FontId::new(8.0, press()), EDGE);
        if eresp.clicked() { self.screen = Screen::Settings; }
        ui.add_space(SP_S);
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            ui.label(title(format!("LEVEL {}  -  {} / {} XP", self.level(), self.s.xp % XP_PER_LEVEL, XP_PER_LEVEL), 9.0));
            ui.add_space(SP_S);
            bar(ui, game::level_frac(self.s.xp), BLUE, 14.0);
        });
        let streak = game::live_streak(&self.s, today());
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            card_title(ui, "STREAK");
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space((ui.available_width() / 2.0 - 90.0).max(0.0));
                    icon(ui, if streak >= 7 { "campfire" } else { "flame" }, 48.0);
                    ui.label(title(format!("{}", streak), 26.0));
                    ui.label(body(if streak == 1 { "day" } else { "days" }, 16.0).color(DIM));
                });
            });
            ui.add_space(SP_S);
            let today_i = game::weekday(today());
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                let w = (ui.available_width() - 6.0 * 4.0) / 7.0;
                for (i, d) in ["M", "T", "W", "T", "F", "S", "S"].iter().enumerate() {
                    ui.vertical(|ui| {
                        ui.set_width(w);
                        let lit = self.s.week_mask & (1 << i) != 0;
                        let (r, _) = ui.allocate_exact_size(Vec2::new(w, 36.0), Sense::hover());
                        draw_icon(ui.painter(), r, "flame", if lit { Color32::WHITE } else { Color32::from_gray(60) });
                        ui.vertical_centered(|ui| ui.label(title(*d, 9.0).color(if i == today_i { GREEN } else { DIM })));
                    });
                }
            });
            ui.add_space(SP_S);
            ui.label(body(format!("Streak goal: {} days  ({} / {})", self.s.goal, streak.min(self.s.goal), self.s.goal), 14.0).color(if streak >= self.s.goal { GOLD } else { DIM }));
            bar(ui, streak as f32 / self.s.goal.max(1) as f32, GREEN, 12.0);
            ui.add_space(SP_S);
            ui.columns(2, |c| {
                c[0].vertical_centered(|ui| {
                    ui.label(body("Streak Savers", 14.0).color(DIM));
                    ui.label(title(format!("{} of 3", self.s.savers), 12.0).color(BLUE));
                    if ui.link(body("GET A SAVER", 12.0).color(BLUE)).clicked() { self.modal = Modal::Shop; }
                });
                c[1].vertical_centered(|ui| {
                    ui.label(body("Longest Streak", 14.0).color(DIM));
                    ui.label(title(format!("{} days", self.s.streak_max), 12.0).color(ORANGE));
                });
            });
        });
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            let n = (0..self.courses.len()).filter(|c| self.badge_earned(*c).is_some()).count();
            card_title(ui, &format!("COURSE BADGES  {} / {}", n, self.courses.len()));
            let bw = ((ui.available_width() - 4.0 * SP_S) / 5.0).clamp(52.0, 90.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = SP_S;
                for c in 0..self.courses.len() {
                    let earned = self.badge_earned(c);
                    ui.vertical(|ui| {
                        ui.set_width(bw);
                        let r = badge(ui, &self.courses[c], self.badge_tier(c), bw).on_hover_text(match earned { Some(d) => format!("{} - earned {}", self.courses[c].badge.name, day_label(d)), None => format!("{} - finish {} to earn", self.courses[c].badge.name, self.courses[c].name) });
                        if r.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() { self.screen = Screen::Preview(c); }
                        ui.vertical_centered(|ui| ui.label(body(self.courses[c].tag, 11.0).color(if earned.is_some() { GOLD } else { DIM })));
                    });
                }
            });
        });
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            let got = game::ACHIEVEMENTS.iter().filter(|a| self.s.ach.contains_key(a.id)).count();
            card_title(ui, &format!("ACHIEVEMENTS  {} / {}", got, game::ACHIEVEMENTS.len()));
            let cols = if ui.available_width() > 420.0 { 4 } else { 3 };
            let cw = (ui.available_width() - (cols as f32 - 1.0) * 8.0) / cols as f32;
            for row in game::ACHIEVEMENTS.chunks(cols) {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;
                    for a in row {
                        let got = self.s.ach.get(a.id).copied();
                        ui.vertical(|ui| {
                            ui.set_width(cw);
                            let (r, resp) = ui.allocate_exact_size(Vec2::new(cw, 64.0), Sense::hover());
                            bevel(ui.painter(), r, if got.is_some() { Color32::from_rgb(0x3a, 0x30, 0x10) } else { Color32::from_rgb(0x18, 0x16, 0x26) }, false);
                            if got.is_some() { ui.painter().rect_stroke(r, 0.0, Stroke::new(2.0, GOLD), egui::StrokeKind::Inside); }
                            draw_icon(ui.painter(), r.shrink(10.0), a.icon, if got.is_some() { Color32::WHITE } else { Color32::from_gray(45) });
                            let tip = match got { Some(d) => format!("{}: {} (earned {})", a.title, a.desc, day_label(d)), None => format!("{}: {}", a.title, a.desc) };
                            let _ = resp.on_hover_text(tip);
                            ui.vertical_centered(|ui| {
                                ui.label(body(a.title, 12.0).color(if got.is_some() { GOLD } else { DIM }));
                                ui.label(body(match got { Some(d) => day_label(d), None => a.desc.to_string() }, 10.0).color(DIM));
                            });
                        });
                    }
                });
                ui.add_space(SP_S);
            }
        });
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            card_title(ui, "COURSE PROGRESS");
            for ci in 0..self.courses.len() {
                let p = self.progress(ci);
                let resp = ui.horizontal(|ui| {
                    emblem(ui, self.courses[ci].tag, self.courses[ci].color, 52.0);
                    ui.vertical(|ui| {
                        ui.label(body(self.courses[ci].name, 15.0));
                        ui.horizontal(|ui| { ui.allocate_ui(Vec2::new((ui.available_width() - 48.0).max(40.0), 12.0), |ui| bar(ui, p, GREEN, 10.0)); ui.label(body(format!("{}%", (p * 100.0) as u32), 13.0)); });
                    });
                }).response;
                if ui.interact(resp.rect, ui.scope_id().with(("cp", ci)), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Course details").clicked() { self.screen = Screen::Preview(ci); }
                ui.add_space(SP_S);
            }
        });
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            card_title(ui, "ADVENTURE LOG");
            for (d, t) in self.s.log.iter().rev().take(3) { ui.label(body(format!("{}  -  {}", day_label(*d), t), 13.0)); }
            if self.s.log.is_empty() { ui.label(body("Nothing yet. Clear a lesson!", 13.0).color(DIM)); }
            ui.add_space(SP_S);
            if pbtn(ui, "OPEN LOG", GREEN, 0.0).clicked() { self.tab = Tab::Train; self.sub_train = 2; }
        });
        ui.add_space(SP_M);
    }

    // ------------------------------------------------------------ TRAIN > COURSES
    pub fn courses_tab(&mut self, ui: &mut egui::Ui) {
        let total: usize = self.courses.iter().map(|c| c.lesson_count()).sum();
        let done = self.s.done.len();
        section(ui, "YOUR PROGRESS");
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                let (r, _) = ui.allocate_exact_size(Vec2::splat(64.0), Sense::hover());
                draw_avatar(ui.painter(), r, self.s.avatar);
                ui.vertical(|ui| {
                    ui.label(title(format!("LEVEL {}", self.level()), 11.0));
                    ui.add_space(2.0);
                    ui.allocate_ui(Vec2::new(ui.available_width(), 14.0), |ui| bar(ui, game::level_frac(self.s.xp), BLUE, 10.0));
                    ui.add_space(2.0);
                    ui.label(body(format!("{} XP   {} of {} lessons cleared", self.s.xp, done.min(total), total), 12.0).color(DIM));
                });
            });
        });
        let c = self.cur;
        let next = self.next_lesson(c);
        section(ui, "CONTINUE");
        panel(ui, PANEL, GOLD, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                badge(ui, &self.courses[c], self.badge_tier(c), 56.0);
                ui.vertical(|ui| {
                    ui.label(title(self.courses[c].name, 10.0));
                    match next {
                        Some((mi, li)) => { ui.label(body(format!("Next: {} - {}", self.courses[c].modules[mi].name, self.courses[c].modules[mi].lessons[li].title), 13.0).color(DIM)); }
                        None => { ui.label(body("Course cleared. Pick another below.", 13.0).color(GREEN)); }
                    }
                    ui.allocate_ui(Vec2::new(ui.available_width(), 14.0), |ui| bar(ui, self.progress(c), GREEN, 10.0));
                });
            });
            ui.add_space(SP_S);
            if let Some((mi, li)) = next && pbtn(ui, "CONTINUE LESSON", GOLD, 0.0).clicked() { self.open_lesson(c, mi, li); }
        });
        section(ui, "BROWSE COURSES");
        ui.add(egui::TextEdit::singleline(&mut self.query).hint_text("Search courses").desired_width(f32::INFINITY));
        ui.add_space(SP_S);
        let cats = ["All", "Web", "Code", "Data"];
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            for (i, name) in cats.iter().enumerate() {
                let w = 24.0 + name.len() as f32 * 11.0;
                let (r, resp) = ui.allocate_exact_size(Vec2::new(w, 30.0), Sense::click());
                let on = self.cat == i;
                bevel(ui.painter(), r, if on { GOLD } else if resp.hovered() { PANEL_HI } else { PANEL }, false);
                ui.painter().text(r.center(), Align2::CENTER_CENTER, *name, FontId::new(8.0, press()), if on { EDGE } else { INK });
                if resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() { self.cat = i; }
            }
        });
        ui.add_space(SP_M);
        let q = self.query.trim().to_lowercase();
        let mut shown = 0;
        for ci in 0..self.courses.len() {
            let cs = &self.courses[ci];
            if self.cat > 0 && cs.cat != cats[self.cat] { continue; }
            if !q.is_empty() && !cs.name.to_lowercase().contains(&q) && !cs.desc.to_lowercase().contains(&q) { continue; }
            shown += 1;
            let p = self.progress(ci);
            let lessons = cs.lesson_count();
            panel(ui, PANEL, EDGE, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    badge(ui, &self.courses[ci], self.badge_tier(ci), 56.0);
                    ui.vertical(|ui| {
                        ui.label(title(self.courses[ci].name, 10.0));
                        ui.label(body(format!("{}  -  {}  -  {} chapters, {} lessons, {} projects", self.courses[ci].level, self.courses[ci].cat, self.courses[ci].modules.len(), lessons, self.courses[ci].project_count()), 12.0).color(DIM));
                    });
                });
                ui.add_space(SP_XS);
                ui.label(body(self.courses[ci].desc, 13.0));
                ui.add_space(SP_S);
                ui.horizontal(|ui| { ui.allocate_ui(Vec2::new((ui.available_width() - 48.0).max(40.0), 14.0), |ui| bar(ui, p, GREEN, 10.0)); ui.label(body(format!("{}%", (p * 100.0) as u32), 13.0)); });
                ui.add_space(SP_S);
                if pbtn(ui, "VIEW COURSE", if p >= 1.0 { PANEL_HI } else if p > 0.0 { GOLD } else { BLUE }, 0.0).clicked() { self.screen = Screen::Preview(ci); }
            });
        }
        if shown == 0 { ui.add_space(SP_XL); ui.vertical_centered(|ui| ui.label(body("No course matches that search.", 14.0).color(DIM))); }
        ui.add_space(SP_M);
    }
}

fn lang_tile(ui: &mut egui::Ui, lang: &str) {
    let (tag, col) = match lang { "python" => ("PY", PURPLE), "javascript" => ("JS", GOLD), "html" => ("HTML", ORANGE), "css" => ("CSS", BLUE), "sql" => ("SQL", RED), _ => ("?", PANEL_HI) };
    emblem(ui, tag, col, 46.0);
}
