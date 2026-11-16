//! First-run welcome flow: name -> avatar -> course -> daily goal.
use super::*;
use save::GOALS;

#[derive(Default)]
pub struct Onboard { pub step: usize, pub name: String }

const STEPS: usize = 4;

impl App {
    pub fn onboarding_screen(&mut self, ctx: &mut egui::Ui) {
        let Some(mut ob) = self.onboard.take() else { return };
        let mut finish = false;
        let enter = ctx.ctx().input(|i| i.key_pressed(egui::Key::Enter));
        egui::Panel::bottom("ob_bot").frame(egui::Frame::new().fill(BG2).inner_margin(16.0)).show(ctx, |ui| {
            ledge(ui, false);
            ui.horizontal(|ui| {
                if ob.step > 0 && pbtn(ui, "BACK", PANEL_HI, 110.0).clicked() { ob.step -= 1; }
                let last = ob.step + 1 == STEPS;
                let w = ui.available_width();
                if pbtn(ui, if last { "BEGIN THE QUEST" } else { "NEXT" }, GOLD, w).clicked() || enter {
                    if ob.step == 0 { self.s.name = Save::clean_name(&ob.name); }
                    if last { finish = true; } else { ob.step += 1; }
                }
            });
        });
        egui::CentralPanel::default().frame(screen_frame(BG)).show(ctx, |ui| {
            floor(ui.painter(), ui.clip_rect(), true);
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                // step dots
                ui.horizontal(|ui| {
                    for i in 0..STEPS {
                        let (r, _) = ui.allocate_exact_size(Vec2::new(28.0, 6.0), Sense::hover());
                        ui.painter().rect_filled(r, 0.0, if i <= ob.step { GOLD } else { PANEL_HI });
                    }
                });
                ui.add_space(SP_L);
                match ob.step {
                    0 => {
                        ui.vertical_centered(|ui| {
                            let (r, _) = ui.allocate_exact_size(Vec2::new(200.0, 120.0), Sense::hover());
                            draw_icon(ui.painter(), Rect::from_center_size(r.center() - Vec2::new(54.0, 0.0), Vec2::splat(90.0)), "spellbook", Color32::WHITE);
                            draw_icon(ui.painter(), Rect::from_center_size(r.center() + Vec2::new(56.0, 4.0), Vec2::splat(84.0)), "lantern", Color32::WHITE);
                            ui.add_space(SP_S);
                            ui.label(title("PIXEL LEARN", 20.0).color(GOLD));
                            ui.add_space(SP_S);
                            ui.label(body("Learn to code one quest at a time. Read a scroll, fill in the missing code, then cast it yourself.", 16.0));
                        });
                        ui.add_space(SP_XL);
                        ui.label(title("WHAT IS YOUR NAME, HERO?", 10.0));
                        ui.add_space(SP_S);
                        let r = ui.add(egui::TextEdit::singleline(&mut ob.name).hint_text("Hero").char_limit(save::NAME_MAX).desired_width(f32::INFINITY).font(FontId::new(18.0, egui::FontFamily::Proportional)));
                        if !r.has_focus() && ob.name.is_empty() { r.request_focus(); }
                        ui.label(body("You can change this later in Settings.", 12.0).color(DIM));
                    }
                    1 => {
                        ui.label(title("CHOOSE YOUR HERO", 12.0).color(GOLD));
                        ui.add_space(SP_XS);
                        ui.label(body(format!("Pick a look for {}.", self.s.name), 15.0).color(DIM));
                        ui.add_space(SP_M);
                        avatar_grid(ui, &mut self.s.avatar);
                    }
                    2 => {
                        ui.label(title("PICK YOUR FIRST QUEST", 12.0).color(GOLD));
                        ui.add_space(SP_XS);
                        ui.label(body("You can switch courses any time.", 15.0).color(DIM));
                        ui.add_space(SP_M);
                        for ci in 0..self.courses.len() {
                            let on = self.cur == ci;
                            let resp = panel_resp(ui, if on { Color32::from_rgb(0x3a, 0x30, 0x10) } else { PANEL }, if on { GOLD } else { EDGE }, |ui| {
                                ui.set_width(ui.available_width());
                                ui.horizontal(|ui| {
                                    emblem(ui, self.courses[ci].tag, self.courses[ci].color, 52.0);
                                    ui.vertical(|ui| {
                                        ui.label(title(self.courses[ci].name, 10.0));
                                        ui.label(body(self.courses[ci].desc, 13.0).color(DIM));
                                    });
                                });
                            });
                            if resp.clicked() { self.cur = ci; self.s.course = ci as u32; }
                        }
                    }
                    _ => {
                        ui.label(title("SET A DAILY GOAL", 12.0).color(GOLD));
                        ui.add_space(SP_XS);
                        ui.label(body("How many days in a row do you want to learn? One lesson a day keeps your streak alive.", 15.0).color(DIM));
                        ui.add_space(SP_L);
                        for g in GOALS {
                            let (lab, sub) = match g { 3 => ("3 DAYS", "A gentle start"), 7 => ("7 DAYS", "One full week"), 14 => ("14 DAYS", "Building a habit"), _ => ("30 DAYS", "Legendary") };
                            let on = self.s.goal == g;
                            let resp = panel_resp(ui, if on { Color32::from_rgb(0x3a, 0x30, 0x10) } else { PANEL }, if on { GOLD } else { EDGE }, |ui| {
                                ui.set_width(ui.available_width());
                                ui.horizontal(|ui| {
                                    icon(ui, if g >= 14 { "campfire" } else { "flame" }, 36.0);
                                    ui.vertical(|ui| { ui.label(title(lab, 10.0)); ui.label(body(sub, 13.0).color(DIM)); });
                                });
                            });
                            if resp.clicked() { self.s.goal = g; }
                        }
                    }
                }
            });
        });
        if finish {
            self.s.onboarded = true;
            self.open_mod = self.current_module(self.cur);
            self.tab = Tab::Learn;
            self.log(format!("{} began the adventure", self.s.name));
            self.persist();
            self.toast(format!("Welcome, {}!", self.s.name));
        } else {
            self.onboard = Some(ob);
        }
    }
}
