//! First-run welcome flow: name -> avatar -> course -> daily goal.
use super::*;
use save::GOALS;
use audio::Sfx;

#[derive(Default)]
pub struct Onboard {
    pub step: usize,
    pub name: String,
    /// when the welcome animation started, and which of its sounds already played
    pub t0: Option<f64>,
    pub fired: u8,
    /// when the current step appeared (steps slide in)
    pub step_t0: f64,
    pub last_step: usize,
}

const STEPS: usize = 4;

/// Draw one logo layer (all layers share the same 48px canvas) moved, turned and faded.
fn layer(p: &egui::Painter, name: &str, canvas: Rect, offset: Vec2, rot: f32, pivot: Pos2, alpha: f32) {
    let Some((id, _)) = TEX.get().and_then(|m| m.get(name)).copied() else { return };
    let a = (alpha.clamp(0.0, 1.0) * 255.0) as u8;
    let mut m = egui::Mesh::with_texture(id);
    m.add_rect_with_uv(canvas.translate(offset), Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::from_rgba_unmultiplied(255, 255, 255, a));
    if rot != 0.0 { m.rotate(egui::emath::Rot2::from_angle(rot), pivot + offset); }
    p.add(egui::Shape::mesh(m));
}

fn ease_out(t: f32) -> f32 { let t = t.clamp(0.0, 1.0); 1.0 - (1.0 - t).powi(3) }
fn bounce(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.7 { (t / 0.7).powi(2) } else { let u = (t - 0.7) / 0.3; 1.0 - 0.12 * (u * std::f32::consts::PI).sin() }
}

/// Moments in the forging animation (seconds) and the sound each one makes.
const BEATS: [(f32, Sfx); 6] = [(0.55, Sfx::Forge), (1.0, Sfx::Snap), (1.45, Sfx::Spark), (1.7, Sfx::Spark), (1.95, Sfx::Spark), (2.4, Sfx::Forge)];

/// The welcome logo: the parts of the sketch fly together and are forged into the hammer:
/// the iron head drops onto the anvil, the handle slides in, the runes < / > fly into place, and a
/// final strike sends sparks flying. Afterwards the runes glow and embers drift up.
fn forge_logo(ui: &mut egui::Ui, t: f32, fired: &mut u8) {
    let scale = 4.0;
    let (area, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 48.0 * scale + 24.0), Sense::hover());
    let p = ui.painter().with_clip_rect(area.expand(40.0));
    // screen shake right after each strike
    let shake = BEATS.iter().filter(|(_, s)| *s == Sfx::Forge).map(|(b, _)| t - b).filter(|d| (0.0..0.25).contains(d)).map(|d| (1.0 - d / 0.25) * 4.0).fold(0.0, f32::max);
    let jitter = Vec2::new((t * 90.0).sin() * shake, (t * 71.0).cos() * shake);
    let canvas = Rect::from_center_size(area.center() + jitter, Vec2::splat(48.0 * scale));
    let g = |x: f32, y: f32| canvas.min + Vec2::new(x, y) * scale;
    let head_c = g(27.5, 19.5);
    let pommel = g(7.0, 40.0);
    // anvil glow under the logo
    let glow = if t > 0.55 { 0.35 + 0.15 * (t * 3.0).sin() } else { t / 0.55 * 0.3 };
    p.circle_filled(head_c + Vec2::new(0.0, 6.0), 70.0, Color32::from_rgba_unmultiplied(240, 120, 30, (glow * 40.0) as u8));
    // the final strike: the whole hammer swings back and forward around the pommel
    let swing = if (2.0..2.5).contains(&t) {
        let u = (t - 2.0) / 0.5;
        if u < 0.6 { -0.35 * ease_out(u / 0.6) } else { -0.35 + 0.4 * ((u - 0.6) / 0.4).min(1.0) - 0.05 * ((u - 0.6) / 0.4).min(1.0) }
    } else { 0.0 };
    let head_drop = (1.0 - bounce(t / 0.55)) * -260.0;
    let handle_in = 1.0 - ease_out((t - 0.6) / 0.4);
    layer(&p, "logo_handle", canvas, Vec2::new(-170.0, 170.0) * handle_in, swing, pommel, ((t - 0.6) / 0.15).min(1.0));
    layer(&p, "logo_head", canvas, Vec2::new(0.0, head_drop), swing, pommel, (t / 0.2).min(1.0));
    for (i, (name, from, spin)) in [("logo_lt", Vec2::new(-90.0, -110.0), -1.4), ("logo_sl", Vec2::new(10.0, -150.0), 2.2), ("logo_gt", Vec2::new(110.0, -40.0), -2.0)].iter().enumerate() {
        let start = 1.05 + i as f32 * 0.25;
        let k = 1.0 - ease_out((t - start) / 0.4);
        if t < start { continue; }
        // glow halo once landed
        if k <= 0.0 && t > 2.4 {
            let pulse = 0.25 + 0.2 * (t * 2.6 + i as f32).sin();
            for d in [Vec2::new(-2.0, 0.0), Vec2::new(2.0, 0.0), Vec2::new(0.0, -2.0), Vec2::new(0.0, 2.0)] { layer(&p, name, canvas, d, swing, pommel, pulse); }
        }
        layer(&p, name, canvas, *from * k, swing + spin * k, head_c, ((t - start) / 0.12).min(1.0));
    }
    // spark bursts at each strike
    for (b, s) in BEATS {
        let d = t - b;
        if !(0.0..0.7).contains(&d) { continue; }
        let n = if s == Sfx::Forge { 18 } else { 7 };
        let origin = if s == Sfx::Forge { head_c + Vec2::new(0.0, 30.0) } else { head_c };
        for k in 0..n {
            let a = k as f32 / n as f32 * std::f32::consts::TAU + b * 3.0;
            let speed = 160.0 + (k * 37 % 90) as f32;
            let pos = origin + Vec2::new(a.cos(), a.sin() - 0.6) * speed * d + Vec2::new(0.0, 420.0 * d * d);
            let c = if k % 3 == 0 { Color32::from_rgb(255, 230, 120) } else { Color32::from_rgb(246, 140, 36) };
            let fade = (1.0 - d / 0.7).max(0.0);
            p.rect_filled(Rect::from_center_size(pos, Vec2::splat(4.0 * fade + 1.0)), 0.0, c.gamma_multiply(fade));
        }
    }
    // embers drifting up from the finished hammer
    if t > 2.5 {
        for k in 0..10 {
            let ph = (t * 0.35 + k as f32 * 0.137).fract();
            let x = head_c.x + ((k as f32 * 2.3).sin() * 60.0) + (t * 1.5 + k as f32).sin() * 6.0;
            let y = head_c.y + 30.0 - ph * 150.0;
            p.rect_filled(Rect::from_center_size(Pos2::new(x, y), Vec2::splat(3.0)), 0.0, Color32::from_rgb(246, 140, 36).gamma_multiply(1.0 - ph));
        }
    }
    for (i, (b, s)) in BEATS.iter().enumerate() {
        if t >= *b && *fired & (1 << i) == 0 { *fired |= 1 << i; audio::play(*s); }
    }
    ui.ctx().request_repaint(); // keeps the parts moving and the embers drifting
}


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
            let now_s = ui.ctx().input(|i| i.time);
            if ob.step != ob.last_step || ob.step_t0 == 0.0 { ob.step_t0 = now_s; ob.last_step = ob.step; }
            let appear = ease_out(((now_s - ob.step_t0) / 0.35) as f32);
            if appear < 1.0 { ui.ctx().request_repaint(); }
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                // step dots
                ui.set_opacity(appear);
                ui.horizontal(|ui| {
                    for i in 0..STEPS {
                        let (r, _) = ui.allocate_exact_size(Vec2::new(28.0, 6.0), Sense::hover());
                        ui.painter().rect_filled(r, 0.0, if i <= ob.step { GOLD } else { PANEL_HI });
                    }
                });
                ui.add_space(SP_L + (1.0 - appear) * 28.0);
                match ob.step {
                    0 => {
                        ui.vertical_centered(|ui| {
                            let t0 = *ob.t0.get_or_insert(now_s);
                            let t = (now_s - t0) as f32;
                            ui.set_opacity(1.0);
                            forge_logo(ui, t, &mut ob.fired);
                            // the title is hammered out letter by letter after the final strike
                            let shown = (((t - 2.4) / 0.06).max(0.0) as usize).min(10);
                            let (tr, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 30.0), Sense::hover());
                            let full = "CODE FORGE";
                            let part: String = full.chars().take(shown).collect();
                            let gal = ui.painter().layout_no_wrap(full.to_string(), FontId::new(22.0, press()), GOLD);
                            let x0 = tr.center().x - gal.size().x / 2.0;
                            ui.painter().text(Pos2::new(x0 + 2.0, tr.center().y + 2.0), Align2::LEFT_CENTER, &part, FontId::new(22.0, press()), EDGE);
                            ui.painter().text(Pos2::new(x0, tr.center().y), Align2::LEFT_CENTER, &part, FontId::new(22.0, press()), GOLD);
                            ui.add_space(SP_S);
                            ui.set_opacity(((t - 2.9) / 0.5).clamp(0.0, 1.0));
                            ui.label(body("Forge your coding skills one quest at a time. Read the scroll, fill in the missing code, then cast it yourself.", 16.0));
                            ui.set_opacity(1.0);
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
