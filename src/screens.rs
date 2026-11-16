use super::*;

impl App {
    pub fn complete_screen(&mut self, ctx: &mut egui::Ui, c: &Complete) -> bool {
        let mut done = false;
        egui::CentralPanel::default().frame(egui::Frame::new().fill(BG).inner_margin(20.0)).show(ctx, |ui| {
            floor(ui.painter(), ui.max_rect(), self.s.scan);
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(16.0);
                    let lost = c.duel.as_ref().map(|d| d.0 <= d.1 && !(d.0 == d.1 && d.0 >= 3)).unwrap_or(false);
                    let (r, _) = ui.allocate_exact_size(Vec2::new(120.0, 120.0), Sense::hover());
                    draw_spr(ui.painter(), r.min, if lost { Spr::Heart } else { Spr::Trophy }, 10.0, &if lost { vec![('r', Color32::from_rgb(0x55, 0x4a, 0x70)), ('R', Color32::from_rgb(0x3a, 0x32, 0x50))] } else { vec![] });
                    ui.add_space(8.0);
                    let head = if c.duel.is_some() { if lost { "DEFEATED..." } else { "VICTORY!" } } else { "QUEST COMPLETE!" };
                    ui.label(title(head, 15.0).color(GOLD));
                    ui.add_space(6.0);
                    ui.label(body(c.title.clone(), 15.0).color(DIM));
                    if let Some((m, b, n)) = &c.duel { ui.label(body(format!("You {}  -  {} {}", m, n, b), 15.0)); }
                    ui.add_space(14.0);
                    panel(ui, PANEL, GOLD, |ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal(|ui| { spr(ui, Spr::Star, 3.0, &[]); ui.label(title(format!("+{} XP", c.xp), 12.0).color(BLUE)); });
                        if c.gems > 0 { ui.horizontal(|ui| { spr(ui, Spr::Gem, 3.0, &[]); ui.label(title(format!("+{} gems", c.gems), 12.0).color(PURPLE)); }); }
                        ui.horizontal(|ui| { spr(ui, Spr::Bolt, 3.0, &[('a', GREEN)]); ui.label(body(format!("Streak {} days (best {}){}", self.s.streak, self.s.streak_max, if c.new_streak { "  +1 today!" } else { "" }), 14.0)); });
                        ui.label(body(format!("Score {}%   (best {}%)", c.score, c.best), 14.0).color(DIM));
                        if c.perfect && c.duel.is_none() { ui.label(body("Flawless! +5 bonus XP", 14.0).color(GREEN)); }
                        ui.add_space(4.0);
                        ui.label(title(format!("LEVEL {}", self.level()), 10.0));
                        bar(ui, (self.s.xp % XP_PER_LEVEL) as f32 / XP_PER_LEVEL as f32, BLUE, 14.0);
                    });
                    ui.add_space(10.0);
                    if pbtn(ui, "CONTINUE", GREEN, 0.0).clicked() { done = true; }
                    ui.add_space(6.0);
                    if pbtn(ui, "LEADERBOARD", BLUE, 0.0).clicked() { done = true; self.tab = Tab::Leader; }
                });
            });
        });
        done
    }

    pub fn settings(&mut self, ctx: &mut egui::Ui) {
        header(ctx, self, "Settings", true, false);
        egui::CentralPanel::default().frame(egui::Frame::new().fill(BG).inner_margin(14.0)).show(ctx, |ui| {
            floor(ui.painter(), ui.max_rect(), self.s.scan);
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                panel(ui, PANEL, EDGE, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(title("ACCOUNT", 10.0).color(GOLD));
                    ui.add_space(6.0);
                    ui.label(body("Edit Profile", 15.0));
                    ui.add(egui::TextEdit::singleline(&mut self.s.name).desired_width(f32::INFINITY).char_limit(18));
                    ui.add(egui::TextEdit::singleline(&mut self.s.country).desired_width(f32::INFINITY).char_limit(22));
                    ui.add_space(8.0);
                    if pbtn(ui, "SAVE PROFILE", GREEN, 0.0).clicked() { self.s.store(); self.say("Profile saved"); }
                    ui.add_space(8.0);
                    if pbtn(ui, "RESET PROGRESS", RED, 0.0).clicked() { self.modal = Modal::Reset; }
                });
                panel(ui, PANEL, EDGE, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(title("SETTINGS", 10.0).color(GOLD));
                    ui.add_space(6.0);
                    if toggle(ui, "Scanlines", "CRT-style pixel filter", &mut self.s.scan) { self.s.store(); }
                    if toggle(ui, "Dungeon floor", "Brick texture behind the screen", &mut self.s.floor) { self.s.store(); }
                    ui.label(body("Language", 15.0)); ui.label(body("English", 12.0).color(DIM));
                    ui.add_space(6.0);
                    ui.label(body("Hearts refill 1 every 5 minutes. 100 XP per level.", 12.0).color(DIM));
                });
                panel(ui, PANEL, EDGE, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(title("CREDITS", 10.0).color(GOLD));
                    ui.add_space(6.0);
                    ui.horizontal(|ui| { icon(ui, "scroll_fire", 36.0); ui.label(body("Assets by Pixelrepo (https://pixelrepo.com) - free assets, used under the Pixelrepo Free Asset License.", 13.0)); });
                    ui.add_space(4.0);
                    ui.label(body("Course chapters follow the public course outlines on codedex.io. Lesson text is written for this app.", 13.0).color(DIM));
                    ui.label(body("Fonts: Press Start 2P and Silkscreen (SIL Open Font License).", 13.0).color(DIM));
                });
            });
        });
    }

    pub fn editor_screen(&mut self, ctx: &mut egui::Ui, mut ed: Editor) -> Option<Editor> {
        let mut keep = true;
        egui::Panel::top("etop").frame(egui::Frame::new().fill(HDR).inner_margin(12.0)).show(ctx, |ui| {
            ui.horizontal(|ui| {
                if pbtn(ui, "<", Color32::from_rgb(0x3a, 0x4a, 0xa8), 44.0).clicked() { keep = false; }
                ui.add(egui::TextEdit::singleline(&mut ed.title).desired_width(ui.available_width()).char_limit(30));
            });
        });
        egui::CentralPanel::default().frame(egui::Frame::new().fill(BG).inner_margin(12.0)).show(ctx, |ui| {
            floor(ui.painter(), ui.max_rect(), self.s.scan);
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (k, n) in [("python", "PYTHON"), ("javascript", "JS"), ("html", "HTML")] {
                        if pbtn(ui, n, if ed.lang == k { GOLD } else { PANEL_HI }, 100.0).clicked() { ed.lang = k.into(); }
                    }
                });
                ui.add_space(8.0);
                panel(ui, Color32::from_rgb(0x0f, 0x0b, 0x1a), GREEN, |ui| {
                    ui.add(egui::TextEdit::multiline(&mut ed.code).code_editor().desired_rows(11).desired_width(f32::INFINITY).text_color(PARCH));
                });
                ui.horizontal(|ui| {
                    if pbtn(ui, "RUN", GREEN, 100.0).clicked() { ed.out = interp::run(&ed.lang, &ed.code); }
                    if pbtn(ui, "SAVE", BLUE, 100.0).clicked() {
                        let e = (ed.title.clone(), ed.lang.clone(), ed.code.clone());
                        match ed.idx { Some(i) if i < self.s.codes.len() => self.s.codes[i] = e, _ => { self.s.codes.push(e); ed.idx = Some(self.s.codes.len() - 1); } }
                        self.s.store(); self.say("Code saved to My Codes");
                    }
                    if let Some(i) = ed.idx { if pbtn(ui, "DELETE", RED, 100.0).clicked() { if i < self.s.codes.len() { self.s.codes.remove(i); self.s.store(); } keep = false; } }
                });
                ui.add_space(8.0);
                panel(ui, PANEL, EDGE, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(title("OUTPUT", 9.0).color(GOLD));
                    ui.add_space(4.0);
                    if ed.out.is_empty() { ui.label(body("Press RUN to cast your code.", 14.0).color(DIM)); } else { out_view(ui, &ed.out); }
                });
            });
        });
        if keep { Some(ed) } else { None }
    }

    pub fn back_screen(&mut self, ctx: &mut egui::Ui, name: &str, add: impl FnOnce(&mut Self, &mut egui::Ui)) {
        header(ctx, self, name, true, false);
        egui::CentralPanel::default().frame(egui::Frame::new().fill(BG).inner_margin(14.0)).show(ctx, |ui| {
            floor(ui.painter(), ui.max_rect(), self.s.scan);
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| add(self, ui));
        });
    }
}
