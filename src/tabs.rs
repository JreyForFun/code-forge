use super::*;

impl App {
    pub fn learn(&mut self, ui: &mut egui::Ui) {
        let c = self.cur;
        banner(ui, self.courses[c].name, self.courses[c].desc, self.courses[c].color, self.progress(c));
        ui.add_space(8.0);
        stats_row(ui, self, true);
        ui.add_space(8.0);
        bar(ui, self.progress(c), GREEN, 14.0);
        ui.add_space(12.0);
        for mi in 0..self.courses[c].modules.len() {
            let done = self.module_done(c, mi);
            let unlocked = (0..mi).all(|m| self.module_done(c, m));
            let open = self.open_mod == Some(mi);
            let (r, resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 62.0), Sense::click());
            let p = ui.painter();
            p.rect_filled(r.translate(Vec2::new(0.0, 4.0)), 0.0, EDGE);
            p.rect_filled(r, 0.0, if open { PANEL_HI } else { PANEL });
            p.rect_stroke(r, 0.0, Stroke::new(2.0, EDGE), egui::StrokeKind::Inside);
            let ir = Rect::from_min_size(r.min + Vec2::new(10.0, 10.0), Vec2::splat(42.0));
            let col = if done { GREEN } else if unlocked { GOLD } else { Color32::from_rgb(0x47, 0x55, 0x69) };
            p.rect_filled(ir.expand(2.0), 0.0, EDGE);
            p.rect_filled(ir, 0.0, if done { Color32::from_rgb(0x1a, 0x3a, 0x12) } else { BG });
            p.rect_stroke(ir, 0.0, Stroke::new(3.0, col), egui::StrokeKind::Inside);
            for (cx, cy) in [(ir.min.x, ir.min.y), (ir.max.x - 6.0, ir.min.y), (ir.min.x, ir.max.y - 6.0), (ir.max.x - 6.0, ir.max.y - 6.0)] { p.rect_filled(Rect::from_min_size(Pos2::new(cx, cy), Vec2::splat(6.0)), 0.0, if open { PANEL_HI } else { PANEL }); }
            if done { draw_spr(p, ir.min + Vec2::new(9.0, 9.0), Spr::Check, 2.0, &[('w', GREEN)]); }
            else if unlocked { p.text(ir.center(), Align2::CENTER_CENTER, format!("{}", mi + 1), FontId::new(14.0, press()), GOLD); }
            else { draw_spr(p, ir.min + Vec2::new(9.0, 9.0), Spr::Lock, 2.0, &[('a', Color32::from_rgb(0x64, 0x74, 0x8b))]); }
            p.text(Pos2::new(r.min.x + 66.0, r.center().y), Align2::LEFT_CENTER, self.courses[c].modules[mi].name, FontId::new(9.0, press()), INK);
            // chevron
            let ax = r.max.x - 24.0;
            for i in 0..4 {
                let w = 16.0 - 4.0 * i as f32;
                let y = if open { r.center().y + 4.0 - 2.0 * i as f32 } else { r.center().y - 4.0 + 2.0 * i as f32 };
                p.rect_filled(Rect::from_min_size(Pos2::new(ax - w / 2.0 + 8.0, y), Vec2::new(w, 2.0)), 0.0, DIM);
            }
            if resp.clicked() { self.open_mod = if open { None } else { Some(mi) }; }
            ui.add_space(8.0);
            if open {
                for li in 0..self.courses[c].modules[mi].lessons.len() {
                    let ld = self.is_done(c, mi, li);
                    let lu = self.lesson_unlocked(c, mi, li);
                    ui.horizontal(|ui| {
                        ui.add_space(14.0);
                        ui.label(body(format!("Exercise {}", li + 1), 12.0).color(DIM));
                        ui.add_space(6.0);
                        let tl = self.courses[c].modules[mi].lessons[li].title;
                        ui.label(body(if lu { tl } else { "???" }, 15.0).color(if lu { INK } else { DIM }));
                        if !ld { if let Some(d) = self.s.qdone.get(&Self::key(c, mi, li)) { if !d.is_empty() { ui.label(body(format!("in progress {}/{}", d.len(), self.courses[c].modules[mi].lessons[li].qs.len()), 11.0).color(GOLD)); } } }
                        if let Some((b, a)) = self.s.best.get(&Self::key(c, mi, li)) { ui.label(body(format!("best {}% x{}", b, a), 11.0).color(DIM)); }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if lu {
                                if pbtn(ui, if ld { "REPLAY" } else { "START" }, if ld { Color32::from_rgb(0x47, 0x55, 0x69) } else { BLUE }, 92.0).clicked() {
                                    let l = self.courses[c].modules[mi].lessons[li].clone();
                                    self.start_run(l.title, Kind::Lesson(c, mi, li), l.slides.clone(), l.qs.clone());
                                }
                                if ld { spr(ui, Spr::Check, 2.0, &[('w', GREEN)]); }
                            } else { spr(ui, Spr::Lock, 2.0, &[('a', Color32::from_rgb(0x64, 0x74, 0x8b))]); }
                        });
                    });
                    ui.add_space(8.0);
                }
            }
        }
        ui.add_space(6.0);
        let prog = self.progress(c);
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            ui.vertical_centered(|ui| {
                let (r, _) = ui.allocate_exact_size(Vec2::new(210.0, 120.0), Sense::hover());
                let p = ui.painter();
                bevel(p, r, if prog >= 1.0 { PARCH } else { Color32::from_rgb(0x7c, 0x76, 0x92) }, false);
                for (i, w) in [150.0, 70.0, 70.0].iter().enumerate() { p.rect_filled(Rect::from_min_size(r.min + Vec2::new(18.0, 20.0 + i as f32 * 20.0), Vec2::new(*w, 6.0)), 0.0, Color32::from_black_alpha(90)); }
                let sc = r.max - Vec2::new(40.0, 36.0);
                p.rect_filled(Rect::from_center_size(sc, Vec2::splat(40.0)), 0.0, EDGE);
                p.rect_filled(Rect::from_center_size(sc, Vec2::splat(30.0)), 0.0, if prog >= 1.0 { GOLD } else { Color32::from_rgb(0x4a, 0x44, 0x66) });
                draw_spr(p, sc - Vec2::new(12.0, 12.0), Spr::Star, 2.0, &[]);
                ui.add_space(8.0);
                ui.label(title(if prog >= 1.0 { "Certificate earned!" } else { "Your Certificate is close" }, 10.0));
                ui.add_space(4.0);
                ui.label(body(if prog >= 1.0 { "You cleared every quest in this course." } else { "Keep learning to unlock your certificate!" }, 13.0).color(DIM));
            });
        });
        ui.add_space(8.0);
        ui.label(title(">> More courses to explore", 10.0));
        ui.add_space(8.0);
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
            if resp.clicked() { self.cur = ci; self.open_mod = self.current_module(ci); }
        }
        ui.add_space(14.0);
    }

    pub fn community(&mut self, ui: &mut egui::Ui) {
        subtabs(ui, &["COURSES", "PRACTICE", "FEED"], &mut self.sub_comm);
        ui.add_space(4.0);
        if self.sub_comm == 0 { self.courses_tab(ui); return; }
        if self.sub_comm == 2 {
            panel(ui, PANEL, EDGE, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    let (r, _) = ui.allocate_exact_size(Vec2::splat(56.0), Sense::hover());
                    bevel(ui.painter(), r, Color32::from_rgb(0x2a, 0x4a, 0x78), false);
                    draw_spr(ui.painter(), r.min + Vec2::new(4.0, 2.0), Spr::Hero, 3.0, &[]);
                    ui.vertical(|ui| { ui.label(title(self.s.name.clone(), 10.0)); ui.label(body("What's on your mind?", 14.0).color(BLUE)); });
                });
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.input).hint_text("Share a quest update...").desired_width((ui.available_width() - 110.0).max(60.0)));
                    if pbtn(ui, "POST", GREEN, 96.0).clicked() && !self.input.trim().is_empty() {
                        let t = std::mem::take(&mut self.input);
                        self.feed.push(format!("said: {}", t));
                    }
                });
            });
            ui.label(title("ACTIVITY FEED", 9.0).color(DIM));
            ui.add_space(8.0);
            if self.feed.is_empty() { ui.add_space(30.0); ui.vertical_centered(|ui| ui.label(title("No Activity", 12.0).color(DIM))); }
            for f in self.feed.clone().iter().rev() {
                panel(ui, PANEL, EDGE, |ui| { ui.set_width(ui.available_width()); ui.horizontal(|ui| { spr(ui, Spr::Star, 2.5, &[]); ui.label(body(format!("{} {}", self.s.name, f), 14.0)); }); });
            }
            return;
        }
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                let (r, _) = ui.allocate_exact_size(Vec2::new(70.0, 70.0), Sense::hover());
                draw_spr(ui.painter(), r.min, Spr::Star, 5.0, &[]);
                ui.vertical(|ui| {
                    ui.label(body("Double your XP for 30 minutes once your friend completes 2 lessons.", 14.0));
                    if pbtn(ui, "INVITE A FRIEND", BLUE, 0.0).clicked() { self.say("Invite link copied"); ui.ctx().copy_text("https://pixel-learn.example/invite".into()); }
                });
            });
        });
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            ui.label(title("Code Coach", 11.0));
            ui.label(body("Practice by solving coding challenges & earn points", 13.0).color(DIM));
            ui.add_space(8.0);
            egui::ScrollArea::horizontal().id_salt("coach").show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (i, c) in coaches().into_iter().enumerate() {
                        ui.vertical(|ui| {
                            ui.set_width(110.0);
                            let (r, resp) = ui.allocate_exact_size(Vec2::splat(104.0), Sense::click());
                            bevel(ui.painter(), r, c.color, true);
                            ui.painter().text(r.center(), Align2::CENTER_CENTER, c.glyph, FontId::new(34.0, press()), Color32::WHITE);
                            ui.label(body(c.title, 13.0));
                            let lc = match c.level { "Easy" => GREEN, "Hard" => RED, _ => BLUE };
                            let (lr, _) = ui.allocate_exact_size(Vec2::new(70.0, 18.0), Sense::hover());
                            ui.painter().rect_filled(lr, 0.0, lc);
                            ui.painter().text(lr.center(), Align2::CENTER_CENTER, c.level, FontId::new(8.0, press()), Color32::WHITE);
                            if resp.clicked() { self.start_run(c.title, Kind::Coach(i), vec![], c.qs.clone()); }
                        });
                        ui.add_space(8.0);
                    }
                });
            });
        });
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            ui.label(title("Code Challenge", 11.0));
            ui.label(body("Pick a foe and duel them with 3 quick questions", 13.0).color(DIM));
            ui.add_space(8.0);
            let ow = ((ui.available_width() - 24.0) / 3.0).max(60.0);
            ui.horizontal(|ui| {
                for (i, o) in opponents().iter().enumerate() {
                    let sel = self.opp == i;
                    let (r, resp) = ui.allocate_exact_size(Vec2::new(ow, 56.0), Sense::click());
                    bevel(ui.painter(), r, if sel { o.color } else { PANEL_HI }, false);
                    ui.painter().text(r.center(), Align2::CENTER_CENTER, o.name.split(' ').next().unwrap_or(""), FontId::new(8.0, press()), Color32::WHITE);
                    if sel { ui.painter().rect_stroke(r.expand(3.0), 0.0, Stroke::new(2.0, GOLD), egui::StrokeKind::Outside); }
                    if resp.clicked() { self.opp = i; }
                }
            });
            ui.add_space(10.0);
            let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 100.0), Sense::hover());
            let p = ui.painter();
            p.rect_filled(r, 0.0, Color32::from_rgb(0x3a, 0x6c, 0xd0));
            p.add(egui::Shape::convex_polygon(vec![Pos2::new(r.center().x + 20.0, r.min.y), r.right_top(), r.right_bottom(), Pos2::new(r.center().x - 30.0, r.max.y)], opponents()[self.opp].color, Stroke::NONE));
            draw_spr(p, Pos2::new(r.min.x + 30.0, r.min.y + 16.0), Spr::Hero, 4.0, &[]);
            p.text(r.center(), Align2::CENTER_CENTER, "VS", FontId::new(20.0, press()), Color32::WHITE);
            p.text(Pos2::new(r.max.x - 64.0, r.center().y), Align2::CENTER_CENTER, opponents()[self.opp].name.split(' ').next().unwrap_or(""), FontId::new(8.0, press()), Color32::WHITE);
            ui.add_space(6.0);
            if pbtn(ui, "START A CODE CHALLENGE", RED, 0.0).clicked() {
                let o = self.opp;
                self.start_run(&format!("Duel vs {}", opponents()[o].name), Kind::Duel(o), vec![], duel_questions());
            }
        });
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            ui.label(title("By The Community", 11.0));
            ui.label(body("Developed by and for the community", 13.0).color(DIM));
            ui.add_space(8.0);
            let cw = ((ui.available_width() - 48.0) / 5.0).max(50.0);
            ui.horizontal(|ui| {
                for (n, col) in [("HTML", TEAL), ("CSS", BLUE), ("JS", GOLD), ("PY", PURPLE), ("SQL", RED)] {
                    let (r, resp) = ui.allocate_exact_size(Vec2::splat(cw), Sense::click());
                    bevel(ui.painter(), r, col, true);
                    ui.painter().text(r.center(), Align2::CENTER_CENTER, n, FontId::new(10.0, press()), Color32::WHITE);
                    if resp.clicked() { if let Some(ci) = self.courses.iter().position(|c| c.tag == n) { self.cur = ci; self.tab = Tab::Learn; self.open_mod = self.current_module(ci); } }
                }
            });
        });
        ui.add_space(10.0);
    }

    pub fn leader(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            chip(ui, Spr::Bolt, &[('a', GREEN)], &self.s.streak.to_string());
            chip(ui, Spr::Gem, &[], &self.s.gems.to_string());
            chip(ui, Spr::Star, &[], &format!("{} XP", self.s.xp));
        });
        ui.add_space(10.0);
        let league = (self.s.league as usize).min(6);
        let names = ["Copper", "Venus", "Terra", "Mars", "Jupiter", "Saturn", "Neptune"];
        let cols = [ORANGE, PURPLE, BLUE, RED, Color32::from_rgb(0xc0, 0x8a, 0x4a), GOLD, TEAL];
        ui.horizontal(|ui| {
            for i in 0..7 {
                let (r, _) = ui.allocate_exact_size(Vec2::new(40.0, 40.0), Sense::hover());
                if i <= league {
                    bevel(ui.painter(), r, cols[i], false);
                    if i == league { ui.painter().rect_stroke(r.expand(3.0), 0.0, Stroke::new(2.0, GOLD), egui::StrokeKind::Outside); }
                    draw_spr(ui.painter(), r.min + Vec2::new(8.0, 8.0), Spr::Gem, 2.0, &[]);
                } else {
                    bevel(ui.painter(), r, Color32::from_rgb(0x30, 0x2a, 0x4a), false);
                    draw_spr(ui.painter(), r.min + Vec2::new(8.0, 8.0), Spr::Lock, 2.0, &[('a', Color32::from_rgb(0x6b, 0x66, 0x88))]);
                }
            }
        });
        ui.add_space(8.0);
        ui.vertical_centered(|ui| ui.label(title(format!("{} League", names[league]), 12.0)));
        ui.add_space(14.0);
        if self.s.week_xp == 0 {
            ui.add_space(30.0);
            ui.vertical_centered(|ui| {
                let (r, _) = ui.allocate_exact_size(Vec2::new(140.0, 140.0), Sense::hover());
                draw_spr(ui.painter(), r.min + Vec2::new(10.0, 0.0), Spr::Trophy, 10.0, &[]);
                ui.add_space(12.0);
                ui.label(title("Complete a lesson or earn XP to join this week's Leaderboard!", 10.0));
                ui.add_space(14.0);
                if pbtn(ui, "START LEARNING", BLUE, 0.0).clicked() { self.tab = Tab::Learn; }
            });
            return;
        }
        let bots = [("PixelFox", 3.1), ("ByteKnight", 2.4), ("NesGal", 1.9), ("LoopLord", 1.4), ("Sprite99", 1.1), ("TinyDev", 0.8), ("Glitch", 0.55), ("Noodle", 0.4), ("Rune", 0.25)];
        let mut rows: Vec<(String, u32, bool)> = bots.iter().map(|(n, f)| (n.to_string(), (((self.s.week_xp.max(40) as f32) * f) as u32) / 10 * 10 + 30, false)).collect();
        rows.push((self.s.name.clone(), self.s.week_xp, true));
        rows.sort_by(|a, b| b.1.cmp(&a.1));
        for (i, (n, x, me)) in rows.iter().enumerate() {
            panel(ui, if *me { Color32::from_rgb(0x4a, 0x3a, 0x1a) } else { PANEL }, if *me { GOLD } else { EDGE }, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    let medal = [GOLD, Color32::from_rgb(0xc8, 0xcc, 0xd8), Color32::from_rgb(0xc0, 0x80, 0x40)];
                    let (r, _) = ui.allocate_exact_size(Vec2::splat(30.0), Sense::hover());
                    bevel(ui.painter(), r, if i < 3 { medal[i] } else { PANEL_HI }, false);
                    ui.painter().text(r.center(), Align2::CENTER_CENTER, format!("{}", i + 1), FontId::new(10.0, press()), if i < 3 { EDGE } else { INK });
                    spr(ui, Spr::Hero, 2.0, &[]);
                    ui.label(body(n.clone(), 15.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { ui.label(title(format!("{} XP", x), 9.0).color(GOLD)); });
                });
            });
        }
        ui.add_space(4.0);
        ui.label(body(format!("Earn {} XP this week to move up a league ({} so far). Earn none and you drop one.", LEAGUE_UP_XP, self.s.week_xp), 12.0).color(DIM));
        bar(ui, self.s.week_xp as f32 / LEAGUE_UP_XP as f32, GOLD, 10.0);
        ui.add_space(10.0);
    }

    pub fn create(&mut self, ui: &mut egui::Ui) {
        subtabs(ui, &["CODE", "DISCUSS"], &mut self.sub_create);
        if self.sub_create == 0 {
            if !self.s.codes.is_empty() {
                ui.label(title("MY CODES", 9.0).color(DIM));
                ui.add_space(6.0);
                for i in 0..self.s.codes.len() {
                    let (t, lg, _) = self.s.codes[i].clone();
                    panel(ui, PANEL, EDGE, |ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| { ui.label(body(t, 15.0)); ui.label(body(lg, 12.0).color(BLUE)); });
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if pbtn(ui, "OPEN", GREEN, 80.0).clicked() { let c = self.s.codes[i].clone(); self.screen = Screen::Editor(Editor { idx: Some(i), title: c.0, lang: c.1, code: c.2, out: vec![] }); }
                            });
                        });
                    });
                }
            }
            ui.label(title("HOT TODAY", 9.0).color(DIM));
            ui.add_space(6.0);
            for i in 0..self.posts.len() {
                let votes = self.posts[i].votes + self.voted.contains(&i) as i32;
                let resp = panel_resp(ui, PANEL, EDGE, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        let (r, _) = ui.allocate_exact_size(Vec2::splat(46.0), Sense::hover());
                        bevel(ui.painter(), r, Color32::from_rgb(0x2a, 0x4a, 0x78), false);
                        draw_spr(ui.painter(), r.min + Vec2::new(3.0, 1.0), Spr::Hero, 2.6, &[]);
                        ui.vertical(|ui| {
                            ui.label(body(self.posts[i].title, 16.0));
                            ui.label(body(self.posts[i].author, 13.0).color(BLUE));
                            ui.horizontal(|ui| {
                                let (tr, _) = ui.allocate_exact_size(Vec2::new(14.0 + self.posts[i].tag.len() as f32 * 8.0, 18.0), Sense::hover());
                                ui.painter().rect_filled(tr, 0.0, BLUE);
                                ui.painter().text(tr.center(), Align2::CENTER_CENTER, self.posts[i].tag, FontId::new(7.0, press()), Color32::WHITE);
                                ui.label(body(format!("+{}  {} cmts  {}  {} views", votes, self.posts[i].comments, self.posts[i].when, self.posts[i].views), 12.0).color(DIM));
                            });
                        });
                    });
                });
                if resp.clicked() {
                    let out = interp::run(self.posts[i].lang, self.posts[i].code);
                    self.screen = Screen::Post(i, out);
                }
            }
            ui.add_space(70.0);
        } else {
            ui.label(title("HOT TODAY", 9.0).color(DIM));
            ui.add_space(6.0);
            for i in 0..self.threads.len() {
                let extra = self.extra_answers.iter().filter(|(t, _)| *t == i).count();
                let resp = panel_resp(ui, PANEL, EDGE, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| { ui.label(title(format!("+{}", self.threads[i].votes), 8.0).color(GREEN)); ui.label(title(format!("{}", self.threads[i].answers.len() + extra), 10.0).color(GREEN)); });
                        ui.vertical(|ui| {
                            ui.label(body(self.threads[i].title, 16.0));
                            ui.horizontal(|ui| {
                                for t in &self.threads[i].tags {
                                    let (tr, _) = ui.allocate_exact_size(Vec2::new(14.0 + t.len() as f32 * 8.0, 18.0), Sense::hover());
                                    ui.painter().rect_filled(tr, 0.0, PANEL_HI);
                                    ui.painter().text(tr.center(), Align2::CENTER_CENTER, *t, FontId::new(7.0, press()), INK);
                                }
                            });
                            ui.label(body(format!("{} ago   {} views   by {}", self.threads[i].when, self.threads[i].views, self.threads[i].author), 12.0).color(DIM));
                        });
                    });
                });
                if resp.clicked() { self.screen = Screen::Thread(i); self.input.clear(); }
            }
            ui.add_space(70.0);
        }
        let r = ui.clip_rect();
        let fr = Rect::from_center_size(Pos2::new(r.max.x - 44.0, r.max.y - 44.0), Vec2::splat(54.0));
        bevel(ui.painter(), fr, GREEN, true);
        ui.painter().rect_filled(Rect::from_center_size(fr.center(), Vec2::new(24.0, 6.0)), 0.0, Color32::WHITE);
        ui.painter().rect_filled(Rect::from_center_size(fr.center(), Vec2::new(6.0, 24.0)), 0.0, Color32::WHITE);
        if ui.interact(fr, ui.id().with("fab"), Sense::click()).clicked() {
            if self.sub_create == 0 { self.screen = Screen::Editor(Editor { idx: None, title: "Untitled".into(), lang: "python".into(), code: "print(\"Hello, pixels!\")\n".into(), out: vec![] }); }
            else { self.screen = Screen::NewThread; self.input.clear(); self.input2.clear(); }
        }
    }

    pub fn profile(&mut self, ui: &mut egui::Ui) {
        let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 250.0), Sense::hover());
        let p = ui.painter();
        p.rect_filled(r, 0.0, HDR);
        for i in 0..20 { p.rect_filled(Rect::from_min_size(Pos2::new(r.min.x + (i as f32 * 53.0) % r.width(), r.min.y + (i as f32 * 37.0) % 120.0), Vec2::splat(3.0)), 0.0, Color32::from_rgb(0x6f, 0x8a, 0xf0)); }
        let ar = Rect::from_center_size(Pos2::new(r.center().x, r.min.y + 70.0), Vec2::splat(112.0));
        bevel(p, ar, Color32::from_rgb(0x2a, 0x4a, 0x78), false);
        draw_spr(p, ar.min + Vec2::new(8.0, 0.0), Spr::Hero, 6.0, &[]);
        p.text(Pos2::new(r.center().x, r.min.y + 150.0), Align2::CENTER_CENTER, self.s.name.as_str(), FontId::new(14.0, press()), Color32::WHITE);
        p.text(Pos2::new(r.center().x, r.min.y + 176.0), Align2::CENTER_CENTER, format!("{}  *  Lvl {}", self.s.country, self.level()), FontId::new(9.0, press()), PARCH);
        p.text(Pos2::new(r.center().x, r.min.y + 200.0), Align2::CENTER_CENTER, "0 Followers  *  1 Following", FontId::new(8.0, press()), Color32::from_rgb(0xc8, 0xd4, 0xff));
        p.text(Pos2::new(r.center().x, r.min.y + 226.0), Align2::CENTER_CENTER, format!("{} XP", self.s.xp), FontId::new(12.0, press()), GOLD);
        let er = Rect::from_center_size(Pos2::new(r.max.x - 34.0, r.min.y + 30.0), Vec2::new(54.0, 34.0));
        bevel(p, er, GOLD, false);
        p.text(er.center(), Align2::CENTER_CENTER, "EDIT", FontId::new(8.0, press()), EDGE);
        if ui.interact(er, ui.id().with("edit"), Sense::click()).clicked() { self.screen = Screen::Settings; }
        ui.add_space(8.0);
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            ui.label(title(format!("LEVEL {}  -  {} / {} XP", self.level(), self.s.xp % XP_PER_LEVEL, XP_PER_LEVEL), 9.0));
            ui.add_space(6.0);
            bar(ui, (self.s.xp % XP_PER_LEVEL) as f32 / XP_PER_LEVEL as f32, BLUE, 14.0);
        });
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            ui.label(title("STREAK", 9.0).color(DIM));
            ui.add_space(6.0);
            ui.vertical_centered(|ui| {
                ui.label(body("Current Streak", 14.0).color(DIM));
                ui.horizontal(|ui| {
                    ui.add_space((ui.available_width() / 2.0 - 90.0).max(0.0));
                    spr(ui, Spr::Bolt, 5.0, &[('a', GREEN)]);
                    ui.label(title(format!("{}", self.s.streak), 26.0));
                    ui.label(body("days", 16.0).color(DIM));
                });
            });
            ui.add_space(8.0);
            let today_i = ((today() + 3) % 7) as usize;
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                let w = (ui.available_width() - 6.0 * 4.0) / 7.0;
                for (i, d) in ["M", "T", "W", "T", "F", "S", "S"].iter().enumerate() {
                    ui.vertical(|ui| {
                        ui.set_width(w);
                        let lit = self.s.week_mask & (1 << i) != 0;
                        let (r, _) = ui.allocate_exact_size(Vec2::new(w, 36.0), Sense::hover());
                        draw_spr(ui.painter(), Pos2::new(r.center().x - 18.0, r.min.y), Spr::Bolt, 3.0, &[('a', if lit { GREEN } else { Color32::from_rgb(0x6b, 0x66, 0x88) }), ('w', if lit { Color32::WHITE } else { Color32::from_rgb(0x8e, 0x89, 0xa8) })]);
                        ui.vertical_centered(|ui| ui.label(title(*d, 9.0).color(if i == today_i { GREEN } else { DIM })));
                    });
                }
            });
            ui.add_space(8.0);
            ui.label(body(format!("Streak goal: {} days  ({} / {})", self.s.goal, self.s.streak.min(self.s.goal), self.s.goal), 14.0).color(if self.s.streak >= self.s.goal { GOLD } else { DIM }));
            bar(ui, self.s.streak as f32 / self.s.goal.max(1) as f32, GREEN, 12.0);
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                let gw = ((ui.available_width() - 24.0) / 4.0).max(40.0);
                for g in [3u32, 7, 14, 30] {
                    if pbtn(ui, &g.to_string(), if self.s.goal == g { GOLD } else { PANEL_HI }, gw).clicked() { self.s.goal = g; self.s.store(); }
                }
            });
            ui.add_space(8.0);
            ui.columns(2, |c| {
                c[0].vertical_centered(|ui| {
                    ui.label(body("Streak Saver", 14.0).color(DIM));
                    ui.label(title(format!("{} of 3", self.s.savers), 12.0).color(BLUE));
                    if ui.link(body("GET STREAK SAVER", 12.0).color(BLUE)).clicked() { self.modal = Modal::Shop; }
                });
                c[1].vertical_centered(|ui| {
                    ui.label(body("Longest Streak", 14.0).color(DIM));
                    ui.label(title(format!("{} days", self.s.streak_max), 12.0).color(ORANGE));
                });
            });
        });
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            ui.label(title("COURSES PROGRESS", 9.0).color(DIM));
            ui.add_space(8.0);
            for ci in 0..self.courses.len() {
                let p = self.progress(ci);
                if p == 0.0 && ci > 2 { continue; }
                ui.horizontal(|ui| {
                    emblem(ui, self.courses[ci].tag, self.courses[ci].color, 52.0);
                    ui.vertical(|ui| {
                        ui.label(body(self.courses[ci].name, 15.0));
                        ui.horizontal(|ui| { ui.allocate_ui(Vec2::new((ui.available_width() - 48.0).max(40.0), 12.0), |ui| bar(ui, p, GREEN, 10.0)); ui.label(body(format!("{}%", (p * 100.0) as u32), 13.0)); });
                    });
                });
                ui.add_space(6.0);
            }
            if pbtn(ui, "MORE COURSES", BLUE, 0.0).clicked() { self.modal = Modal::Courses; }
        });
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            ui.label(title("ACHIEVEMENTS", 9.0).color(DIM));
            ui.add_space(6.0);
            let lessons = self.s.done.len();
            let list = [("First Steps", "Finish a lesson", lessons >= 1), ("On Fire", "3 day streak", self.s.streak_max >= 3), ("Scholar", "Reach 100 XP", self.s.xp >= 100),
                ("Rich", "Hold 50 gems", self.s.gems >= 50), ("Coder", "Save a code", !self.s.codes.is_empty())];
            for (n, d, got) in list {
                ui.horizontal(|ui| {
                    spr(ui, if got { Spr::Star } else { Spr::Lock }, 3.0, &[('a', Color32::from_rgb(0x6b, 0x66, 0x88))]);
                    ui.vertical(|ui| { ui.label(body(n, 15.0).color(if got { GOLD } else { DIM })); ui.label(body(d, 12.0).color(DIM)); });
                });
                ui.add_space(4.0);
            }
        });
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            ui.label(title("ACTIVITY", 9.0).color(DIM));
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);
                ui.label(body("Code bits, Solutions, Posts and Questions", 15.0));
                ui.add_space(8.0);
                if pbtn(ui, "SEE ACTIVITY", GREEN, 200.0).clicked() { self.tab = Tab::Community; self.sub_comm = 2; }
            });
        });
        ui.add_space(10.0);
    }
}

// A pixel landscape banner in the spirit of a course header.
fn banner(ui: &mut egui::Ui, name: &str, desc: &str, accent: Color32, prog: f32) {
    let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 150.0), Sense::hover());
    let p = ui.painter().with_clip_rect(r);
    let s = 6.0;
    let bands = [Color32::from_rgb(0x1f, 0x7a, 0xc8), Color32::from_rgb(0x32, 0x96, 0xdc), Color32::from_rgb(0x4d, 0xb0, 0xea), Color32::from_rgb(0x7d, 0xcb, 0xf2), Color32::from_rgb(0xa9, 0xe0, 0xf6)];
    let bh = (r.height() * 0.7) / bands.len() as f32;
    for (i, c) in bands.iter().enumerate() { p.rect_filled(Rect::from_min_size(Pos2::new(r.min.x, r.min.y + bh * i as f32), Vec2::new(r.width(), bh + 1.0)), 0.0, *c); }
    // mountains (stepped)
    let base = r.max.y - 34.0;
    for (cx, h, col) in [(0.22, 60.0, Color32::from_rgb(0x3b, 0x5b, 0x9a)), (0.55, 84.0, Color32::from_rgb(0x2f, 0x4a, 0x86)), (0.85, 52.0, Color32::from_rgb(0x3b, 0x5b, 0x9a))] {
        let mut y = 0.0;
        while y < h {
            let half = (h - y) * 1.1;
            let x0 = r.min.x + r.width() * cx - half;
            p.rect_filled(Rect::from_min_size(Pos2::new((x0 / s).floor() * s, base - y - s), Vec2::new(half * 2.0, s + 0.5)), 0.0, col);
            y += s;
        }
    }
    // clouds
    for (cx, cy) in [(0.12, 0.18), (0.7, 0.1), (0.9, 0.32)] {
        let o = Pos2::new(r.min.x + r.width() * cx, r.min.y + r.height() * cy);
        p.rect_filled(Rect::from_min_size(o, Vec2::new(36.0, 6.0)), 0.0, Color32::WHITE);
        p.rect_filled(Rect::from_min_size(o + Vec2::new(6.0, -6.0), Vec2::new(18.0, 6.0)), 0.0, Color32::WHITE);
    }
    // ground with grass and soil
    p.rect_filled(Rect::from_min_max(Pos2::new(r.min.x, base), r.max), 0.0, Color32::from_rgb(0x4c, 0xa6, 0x2e));
    p.rect_filled(Rect::from_min_size(Pos2::new(r.min.x, base), Vec2::new(r.width(), 6.0)), 0.0, Color32::from_rgb(0x7c, 0xd0, 0x3c));
    p.rect_filled(Rect::from_min_max(Pos2::new(r.min.x, base + 20.0), r.max), 0.0, Color32::from_rgb(0x7a, 0x4a, 0x24));
    let mut x = r.min.x;
    while x < r.max.x { p.rect_filled(Rect::from_min_size(Pos2::new(x, base + 20.0), Vec2::new(6.0, 6.0)), 0.0, Color32::from_rgb(0x5a, 0x34, 0x18)); x += 24.0; }
    // pine trees
    for fx in [0.04, 0.3, 0.62, 0.93] {
        let tx = r.min.x + r.width() * fx;
        p.rect_filled(Rect::from_min_size(Pos2::new(tx + 6.0, base - 8.0), Vec2::new(6.0, 14.0)), 0.0, Color32::from_rgb(0x5a, 0x34, 0x18));
        for i in 0..4 { let w = 30.0 - i as f32 * 7.0; p.rect_filled(Rect::from_min_size(Pos2::new(tx + 9.0 - w / 2.0, base - 10.0 - i as f32 * 8.0), Vec2::new(w, 8.0)), 0.0, if i % 2 == 0 { Color32::from_rgb(0x1f, 0x6b, 0x2c) } else { Color32::from_rgb(0x2a, 0x85, 0x35) }); }
    }
    // free pixel art props from Pixelrepo
    draw_icon(&p, Rect::from_min_size(Pos2::new(r.max.x - 120.0, base - 30.0), Vec2::splat(64.0)), "chest", Color32::WHITE);
    draw_icon(&p, Rect::from_min_size(Pos2::new(r.max.x - 70.0, base - 8.0), Vec2::splat(40.0)), "sword", Color32::WHITE);
    draw_icon(&p, Rect::from_min_size(Pos2::new(r.max.x - 190.0, base - 20.0), Vec2::splat(48.0)), "torch", Color32::WHITE);
    // text
    let badge = Rect::from_min_size(r.min + Vec2::new(12.0, 10.0), Vec2::new(150.0, 20.0));
    p.rect_filled(badge, 0.0, Color32::from_black_alpha(150));
    p.text(badge.center(), Align2::CENTER_CENTER, "BEGINNER COURSE", FontId::new(8.0, press()), Color32::WHITE);
    p.text(r.min + Vec2::new(14.0, 49.0), Align2::LEFT_CENTER, name, FontId::new(22.0, press()), EDGE);
    p.text(r.min + Vec2::new(12.0, 46.0), Align2::LEFT_CENTER, name, FontId::new(22.0, press()), Color32::WHITE);
    let g = p.layout(desc.to_string(), FontId::new(13.0, egui::FontFamily::Proportional), Color32::WHITE, r.width() - 24.0);
    p.rect_filled(Rect::from_min_size(r.min + Vec2::new(10.0, 64.0), g.size() + Vec2::new(4.0, 2.0)), 0.0, Color32::from_black_alpha(110));
    p.galley(r.min + Vec2::new(12.0, 65.0), g, Color32::WHITE);
    let pb = Rect::from_min_size(Pos2::new(r.min.x + 12.0, r.max.y - 18.0), Vec2::new(r.width() * 0.4, 10.0));
    p.rect_filled(pb.expand(2.0), 0.0, EDGE);
    p.rect_filled(pb, 0.0, Color32::from_rgb(0x1e, 0x29, 0x3b));
    p.rect_filled(Rect::from_min_size(pb.min, Vec2::new(pb.width() * prog, pb.height())), 0.0, accent);
    p.rect_stroke(r, 0.0, Stroke::new(3.0, EDGE), egui::StrokeKind::Inside);
}

impl App {
    /// Training hub: your progress, the next step, and every course you can start.
    pub fn courses_tab(&mut self, ui: &mut egui::Ui) {
        let total: usize = self.courses.iter().map(|c| c.modules.iter().map(|m| m.lessons.len()).sum::<usize>()).sum();
        let done = self.s.done.len();
        ui.label(title("YOUR PROGRESS", 9.0).color(DIM));
        ui.add_space(6.0);
        panel(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                let (r, _) = ui.allocate_exact_size(Vec2::splat(64.0), Sense::hover());
                bevel(ui.painter(), r, PANEL_HI, false);
                draw_icon(ui.painter(), r.shrink(8.0), "helmet", Color32::WHITE);
                ui.vertical(|ui| {
                    ui.label(title(format!("LEVEL {}", self.level()), 11.0));
                    ui.add_space(2.0);
                    ui.allocate_ui(Vec2::new(ui.available_width(), 14.0), |ui| bar(ui, (self.s.xp % XP_PER_LEVEL) as f32 / XP_PER_LEVEL as f32, BLUE, 10.0));
                    ui.add_space(2.0);
                    ui.label(body(format!("{} XP   {} of {} lessons cleared", self.s.xp, done, total), 12.0).color(DIM));
                });
            });
        });
        // continue where you left off
        let c = self.cur;
        let mut next = None;
        'find: for mi in 0..self.courses[c].modules.len() {
            for li in 0..self.courses[c].modules[mi].lessons.len() { if !self.is_done(c, mi, li) { next = Some((mi, li)); break 'find; } }
        }
        ui.label(title("CONTINUE", 9.0).color(DIM));
        ui.add_space(6.0);
        panel(ui, PANEL, GOLD, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                emblem(ui, self.courses[c].tag, self.courses[c].color, 56.0);
                ui.vertical(|ui| {
                    ui.label(title(self.courses[c].name, 10.0));
                    match next {
                        Some((mi, li)) => { ui.label(body(format!("Next: {} - {}", self.courses[c].modules[mi].name, self.courses[c].modules[mi].lessons[li].title), 13.0).color(DIM)); }
                        None => { ui.label(body("Course cleared. Pick another below.", 13.0).color(GREEN)); }
                    }
                    ui.allocate_ui(Vec2::new(ui.available_width(), 14.0), |ui| bar(ui, self.progress(c), GREEN, 10.0));
                });
            });
            ui.add_space(8.0);
            if let Some((mi, li)) = next {
                if pbtn(ui, "CONTINUE LESSON", GOLD, 0.0).clicked() {
                    let l = self.courses[c].modules[mi].lessons[li].clone();
                    self.start_run(l.title, Kind::Lesson(c, mi, li), l.slides.clone(), l.qs.clone());
                }
            }
        });
        // browse
        ui.label(title("BROWSE COURSES", 9.0).color(DIM));
        ui.add_space(6.0);
        ui.add(egui::TextEdit::singleline(&mut self.query).hint_text("Search courses").desired_width(f32::INFINITY));
        ui.add_space(8.0);
        let cats = ["All", "Web", "Code", "Data"];
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            for (i, name) in cats.iter().enumerate() {
                let w = 24.0 + name.len() as f32 * 11.0;
                let (r, resp) = ui.allocate_exact_size(Vec2::new(w, 30.0), Sense::click());
                let on = self.cat == i;
                bevel(ui.painter(), r, if on { GOLD } else if resp.hovered() { PANEL_HI } else { PANEL }, false);
                ui.painter().text(r.center(), Align2::CENTER_CENTER, *name, FontId::new(8.0, press()), if on { EDGE } else { INK });
                if resp.clicked() { self.cat = i; }
            }
        });
        ui.add_space(10.0);
        let q = self.query.trim().to_lowercase();
        let mut shown = 0;
        for ci in 0..self.courses.len() {
            let cs = &self.courses[ci];
            if self.cat > 0 && cs.cat != cats[self.cat] { continue; }
            if !q.is_empty() && !cs.name.to_lowercase().contains(&q) && !cs.desc.to_lowercase().contains(&q) { continue; }
            shown += 1;
            let p = self.progress(ci);
            let lessons: usize = cs.modules.iter().map(|m| m.lessons.len()).sum();
            panel(ui, PANEL, EDGE, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    emblem(ui, self.courses[ci].tag, self.courses[ci].color, 56.0);
                    ui.vertical(|ui| {
                        ui.label(title(self.courses[ci].name, 10.0));
                        ui.label(body(format!("{}  -  {}  -  {} chapters, {} lessons", self.courses[ci].level, self.courses[ci].cat, self.courses[ci].modules.len(), lessons), 12.0).color(DIM));
                    });
                });
                ui.add_space(4.0);
                ui.label(body(self.courses[ci].desc, 13.0));
                ui.add_space(6.0);
                ui.horizontal(|ui| { ui.allocate_ui(Vec2::new((ui.available_width() - 48.0).max(40.0), 14.0), |ui| bar(ui, p, GREEN, 10.0)); ui.label(body(format!("{}%", (p * 100.0) as u32), 13.0)); });
                ui.add_space(6.0);
                let label = if p >= 1.0 { "REVIEW" } else if p > 0.0 { "CONTINUE" } else { "START" };
                if pbtn(ui, label, if p >= 1.0 { PANEL_HI } else if p > 0.0 { GOLD } else { BLUE }, 0.0).clicked() {
                    self.cur = ci; self.open_mod = self.current_module(ci).or(Some(0)); self.tab = Tab::Learn;
                }
            });
        }
        if shown == 0 { ui.add_space(20.0); ui.vertical_centered(|ui| ui.label(body("No course matches that search.", 14.0).color(DIM))); }
        ui.add_space(10.0);
    }
}
