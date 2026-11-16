use std::collections::HashMap;
use std::sync::OnceLock;
use eframe::egui::{self, Color32, FontFamily, FontId, Pos2, Rect, RichText, Sense, Stroke, Vec2};

// ---------- RPG dungeon palette ----------
pub const BG: Color32 = Color32::from_rgb(0x02, 0x06, 0x17);
pub const BG2: Color32 = Color32::from_rgb(0x0b, 0x12, 0x28);
pub const PANEL: Color32 = Color32::from_rgb(0x0f, 0x17, 0x2a);
pub const PANEL_HI: Color32 = Color32::from_rgb(0x1e, 0x29, 0x3b);
pub const EDGE: Color32 = Color32::from_rgb(0x00, 0x03, 0x0c);
pub const PARCH: Color32 = Color32::from_rgb(0xf0, 0xe2, 0xb6);
pub const INK: Color32 = Color32::from_rgb(0xf8, 0xfa, 0xfc);
/// Dark ink for text on parchment.
pub const INK_DARK: Color32 = Color32::from_rgb(0x2a, 0x1c, 0x12);
pub const DIM: Color32 = Color32::from_rgb(0x94, 0xa3, 0xb8);
pub const GREEN: Color32 = Color32::from_rgb(0x4f, 0xc8, 0x3c);
pub const GOLD: Color32 = Color32::from_rgb(0xfa, 0xcc, 0x15);
pub const ORANGE: Color32 = Color32::from_rgb(0xf0, 0x8a, 0x24);
pub const RED: Color32 = Color32::from_rgb(0xe0, 0x36, 0x4c);
pub const BLUE: Color32 = Color32::from_rgb(0x14, 0xad, 0xff);
pub const PURPLE: Color32 = Color32::from_rgb(0xa0, 0x66, 0xe8);
pub const TEAL: Color32 = Color32::from_rgb(0x2f, 0xb9, 0xa5);

pub fn shade(c: Color32, f: f32) -> Color32 {
    let m = |v: u8| ((v as f32 * f).clamp(0.0, 255.0)) as u8;
    Color32::from_rgb(m(c.r()), m(c.g()), m(c.b()))
}

// ---------- spacing scale (use these instead of ad-hoc numbers) ----------
pub const SP_XS: f32 = 4.0;
pub const SP_S: f32 = 8.0;
pub const SP_M: f32 = 12.0;
pub const SP_L: f32 = 16.0;
pub const SP_XL: f32 = 24.0;
/// Inner padding of every screen.
pub const SCREEN_PAD: i8 = 12;
/// Gap left under each card (includes the 4px drop shadow).
pub const CARD_GAP: f32 = SP_M;

/// Section heading outside cards: small dim caps with consistent spacing.
pub fn section(ui: &mut egui::Ui, text: &str) {
    ui.add_space(SP_XS);
    ui.label(title(text, 9.0).color(DIM));
    ui.add_space(SP_S);
}
/// Heading inside a card.
pub fn card_title(ui: &mut egui::Ui, text: &str) {
    ui.label(title(text, 10.0).color(GOLD));
    ui.add_space(SP_S);
}
pub fn screen_frame(fill: Color32) -> egui::Frame { egui::Frame::new().fill(fill).inner_margin(egui::Margin::same(SCREEN_PAD)) }

// ---------- fonts ----------
pub fn press() -> FontFamily { FontFamily::Name("press".into()) }
pub fn read() -> FontFamily { FontFamily::Name("read".into()) }
pub fn title(s: impl Into<String>, size: f32) -> RichText {
    RichText::new(s.into()).font(FontId::new(size, press())).color(INK)
}
pub fn body(s: impl Into<String>, size: f32) -> RichText {
    RichText::new(s.into()).font(FontId::new(size, FontFamily::Proportional)).color(INK)
}

pub fn install_fonts(ctx: &egui::Context) {
    let mut f = egui::FontDefinitions::default();
    f.font_data.insert("silk".into(), std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/Silkscreen.ttf"))));
    f.font_data.insert("press".into(), std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/PressStart2P.ttf"))));
    f.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "silk".into());
    f.families.insert(press(), vec!["press".into()]);
    // a calm, readable face for long explanations
    f.families.insert(read(), vec!["Ubuntu-Light".into(), "Hack".into(), "silk".into()]);
    ctx.set_fonts(f);
}

// ---------- code-drawn sprites ----------
#[derive(Clone, Copy, PartialEq)]
pub enum Spr { Check }

pub fn rows(s: Spr) -> Vec<String> {
    match s {
        Spr::Check => ["............", "..........ww", ".........ww.", "........ww..", "ww.....ww...", ".ww...ww....", "..ww.ww.....", "...www......", "....w.......", "............", "............", "............"]
            .iter().map(|x| x.to_string()).collect(),
    }
}

pub fn draw_spr(p: &egui::Painter, at: Pos2, s: Spr, px: f32, over: &[(char, Color32)]) {
    for (y, row) in rows(s).iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let col = over.iter().find(|(a, _)| *a == c).map(|(_, c)| *c).or(if c == 'w' { Some(Color32::WHITE) } else { None });
            if let Some(col) = col {
                p.rect_filled(Rect::from_min_size(at + Vec2::new(x as f32 * px, y as f32 * px), Vec2::splat(px + 0.5)), 0.0, col);
            }
        }
    }
}

pub fn spr_size(s: Spr, px: f32) -> Vec2 {
    let r = rows(s);
    Vec2::new(r[0].chars().count() as f32 * px, r.len() as f32 * px)
}

pub fn spr(ui: &mut egui::Ui, s: Spr, px: f32, over: &[(char, Color32)]) {
    let (r, _) = ui.allocate_exact_size(spr_size(s, px), Sense::hover());
    draw_spr(ui.painter(), r.min, s, px, over);
}

// ---------- pixel widgets: dungeon skin (Kenney CC0 tiles drawn as 9-slices) ----------
/// Skin tiles are drawn at 2x so their pixels match the 16px dungeon tiles.
pub const SKIN_SCALE: f32 = 2.0;
/// Border of a frame tile, in source pixels.
const FRAME_SRC: f32 = 5.0;
/// Space a stone frame takes on each side of a card's content (drawn border + breathing room).
pub const FRAME_PAD: i8 = (FRAME_SRC * SKIN_SCALE) as i8 + 6;

fn tex(name: &str) -> Option<(egui::TextureId, Vec2)> { TEX.get().and_then(|m| m.get(name)).copied() }

/// `name` as a 9-slice over `r`: corners keep their size, edges and centre stretch.
pub fn nine_shape(r: Rect, name: &str, border: f32, tint: Color32) -> egui::Shape {
    let Some((id, sz)) = tex(name) else { return egui::Shape::Noop };
    let b = (border * SKIN_SCALE).min(r.width() / 2.0).min(r.height() / 2.0);
    let (bu, bv) = (border / sz.x, border / sz.y);
    let xs = [r.min.x, r.min.x + b, r.max.x - b, r.max.x];
    let ys = [r.min.y, r.min.y + b, r.max.y - b, r.max.y];
    let us = [0.0, bu, 1.0 - bu, 1.0];
    let vs = [0.0, bv, 1.0 - bv, 1.0];
    let mut m = egui::Mesh::with_texture(id);
    for j in 0..3 {
        for i in 0..3 {
            if xs[i + 1] <= xs[i] || ys[j + 1] <= ys[j] { continue; }
            m.add_rect_with_uv(Rect::from_min_max(Pos2::new(xs[i], ys[j]), Pos2::new(xs[i + 1], ys[j + 1])),
                Rect::from_min_max(Pos2::new(us[i], vs[j]), Pos2::new(us[i + 1], vs[j + 1])), tint);
        }
    }
    egui::Shape::mesh(m)
}

/// A repeat-wrapped texture over `r`, anchored to the screen so neighbouring areas line up.
pub fn tile_shape(r: Rect, name: &str, tint: Color32) -> egui::Shape {
    let Some((id, sz)) = tex(name) else { return egui::Shape::Noop };
    let t = sz * SKIN_SCALE;
    let uv = Rect::from_min_max(Pos2::new(r.min.x / t.x, r.min.y / t.y), Pos2::new(r.max.x / t.x, r.max.y / t.y));
    let mut m = egui::Mesh::with_texture(id);
    m.add_rect_with_uv(r, uv, tint);
    egui::Shape::mesh(m)
}

/// A carved stone block in the given colour (buttons, answer tiles, chips).
pub fn bevel(p: &egui::Painter, r: Rect, fill: Color32, shadow: bool) {
    if shadow { p.rect_filled(r.translate(Vec2::new(0.0, 4.0)), 3.0, Color32::from_black_alpha(150)); }
    p.rect_filled(r.shrink(3.0), 0.0, fill);
    p.add(nine_shape(r, "block", 5.0, shade(fill, 1.25)));
}

pub fn pbtn(ui: &mut egui::Ui, label: &str, color: Color32, width: f32) -> egui::Response {
    let w = if width <= 0.0 { ui.available_width() } else { width };
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 44.0), Sense::click());
    let down = resp.is_pointer_button_down_on();
    let hov = ui.ctx().animate_bool_with_time(resp.id.with("hov"), resp.hovered(), 0.12);
    let fill = shade(color, 1.0 + 0.14 * hov);
    let r = if down { rect.translate(Vec2::new(0.0, 3.0)) } else { rect.translate(Vec2::new(0.0, -2.0)) };
    bevel(ui.painter(), r, fill, !down);
    let txt_col = if color.r() as u32 + color.g() as u32 + color.b() as u32 > 460 { EDGE } else { Color32::WHITE };
    if txt_col == Color32::WHITE {
        ui.painter().text(r.center() + Vec2::splat(1.0), egui::Align2::CENTER_CENTER, label, FontId::new(11.0, press()), Color32::from_black_alpha(170));
    }
    ui.painter().text(r.center(), egui::Align2::CENTER_CENTER, label, FontId::new(11.0, press()), txt_col);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// Frame tile for an edge colour: iron-studded stone normally, wood for gold highlights, and stone
/// tinted by any other accent (green = done, red = warning).
fn frame_for(edge: Color32) -> (&'static str, Color32) {
    if edge == EDGE { ("frame_stone", Color32::WHITE) }
    else if edge == GOLD || edge == ORANGE { ("frame_wood", Color32::WHITE) }
    else { ("frame_stone", Color32::from_rgb(edge.r() / 2 + 128, edge.g() / 2 + 128, edge.b() / 2 + 128)) }
}

/// Shapes for a stone (or wood) frame around `r` with `fill` inside.
pub fn frame_shapes(r: Rect, fill: Color32, edge: Color32) -> egui::Shape {
    let (name, tint) = frame_for(edge);
    egui::Shape::Vec(vec![
        egui::Shape::rect_filled(r.translate(Vec2::new(0.0, 4.0)), 4.0, Color32::from_black_alpha(140)),
        egui::Shape::rect_filled(r.shrink(3.0), 0.0, fill),
        // a faint brick pattern so cards read as carved into the wall
        tile_shape(r.shrink(FRAME_SRC * SKIN_SCALE - 2.0), "wall", Color32::from_white_alpha(9)),
        nine_shape(r, name, FRAME_SRC, tint),
    ])
}

fn framed<R>(ui: &mut egui::Ui, fill: Color32, edge: Color32, f: impl FnOnce(&mut egui::Ui) -> R) -> egui::InnerResponse<R> {
    // reserve a slot below the content, lay the content out, then paint the frame into the slot
    let slot = ui.painter().add(egui::Shape::Noop);
    let out = egui::Frame::new().inner_margin(egui::Margin::same(FRAME_PAD)).show(ui, f);
    ui.painter().set(slot, frame_shapes(out.response.rect, fill, edge));
    ui.add_space(CARD_GAP);
    out
}

pub fn panel<R>(ui: &mut egui::Ui, fill: Color32, edge: Color32, f: impl FnOnce(&mut egui::Ui) -> R) -> R {
    framed(ui, fill, edge, f).inner
}

/// A parchment page with a wooden rim, for reading long text (dark ink on light paper).
pub fn parchment<R>(ui: &mut egui::Ui, f: impl FnOnce(&mut egui::Ui) -> R) -> R {
    let slot = ui.painter().add(egui::Shape::Noop);
    let out = egui::Frame::new().inner_margin(egui::Margin::same(FRAME_PAD + 2)).show(ui, f);
    let r = out.response.rect;
    ui.painter().set(slot, egui::Shape::Vec(vec![
        egui::Shape::rect_filled(r.translate(Vec2::new(0.0, 4.0)), 4.0, Color32::from_black_alpha(140)),
        nine_shape(r, "parch", 5.0, Color32::WHITE),
    ]));
    ui.add_space(CARD_GAP);
    out.inner
}

/// A red ribbon banner with a title on it.
pub fn ribbon(ui: &mut egui::Ui, text: &str) {
    let galley = ui.painter().layout_no_wrap(text.to_string(), FontId::new(12.0, press()), Color32::WHITE);
    let w = (galley.size().x + 80.0).min(ui.available_width());
    let (r, _) = ui.allocate_exact_size(Vec2::new(w, 32.0), Sense::hover());
    ui.painter().add(nine_shape(r, "ribbon", 7.0, Color32::WHITE));
    ui.painter().text(r.center() + Vec2::new(1.0, 0.0), egui::Align2::CENTER_CENTER, text, FontId::new(12.0, press()), Color32::from_black_alpha(160));
    ui.painter().text(r.center() - Vec2::new(0.0, 1.0), egui::Align2::CENTER_CENTER, text, FontId::new(12.0, press()), Color32::WHITE);
    ui.add_space(SP_S);
}

/// Stone ledge behind a header / footer bar. Call first inside the panel's closure.
pub fn ledge(ui: &egui::Ui, top: bool) {
    let r = ui.clip_rect();
    let p = ui.painter();
    p.add(tile_shape(r, "wall", Color32::from_gray(92)));
    p.rect_filled(r, 0.0, Color32::from_rgba_unmultiplied(0x10, 0x0c, 0x1c, 120));
    let lip = 6.0 * SKIN_SCALE / 2.0;
    let y = if top { r.max.y - lip } else { r.min.y };
    let strip = Rect::from_min_size(Pos2::new(r.min.x, y), Vec2::new(r.width(), lip));
    p.add(tile_shape(strip, "ledge", Color32::from_gray(200)));
    let edge_y = if top { r.max.y - 1.0 } else { r.min.y };
    p.rect_filled(Rect::from_min_size(Pos2::new(r.min.x, edge_y), Vec2::new(r.width(), 2.0)), 0.0, EDGE);
}

/// A small stone button for secondary actions inside rows.
pub fn mini_btn(ui: &mut egui::Ui, label: &str, color: Color32) -> egui::Response {
    let w = 18.0 + label.chars().count() as f32 * 8.0;
    let (r, resp) = ui.allocate_exact_size(Vec2::new(w, 24.0), Sense::click());
    bevel(ui.painter(), r, if resp.hovered() { shade(color, 1.2) } else { color }, false);
    ui.painter().text(r.center(), egui::Align2::CENTER_CENTER, label, FontId::new(7.0, press()), Color32::WHITE);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A carved channel with a coloured fill.
pub fn bar(ui: &mut egui::Ui, frac: f32, color: Color32, h: f32) {
    let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), h), Sense::hover());
    let frac = ui.ctx().animate_value_with_time(egui::Id::unique(("bar", r.min.x as i32, r.min.y as i32 / 8)), frac, 0.25);
    let p = ui.painter();
    p.rect_filled(r.expand(4.0), 3.0, Color32::from_rgb(0x5a, 0x66, 0x7c));
    p.rect_filled(r.expand(2.0), 2.0, EDGE);
    p.rect_filled(r, 0.0, Color32::from_rgb(0x14, 0x12, 0x1e));
    let w = (r.width() * frac.clamp(0.0, 1.0)).floor();
    if w > 0.0 {
        let fr = Rect::from_min_size(r.min, Vec2::new(w, h));
        p.rect_filled(fr, 0.0, color);
        p.rect_filled(Rect::from_min_size(fr.min, Vec2::new(w, (h / 3.0).floor())), 0.0, shade(color, 1.35));
        p.rect_filled(Rect::from_min_size(Pos2::new(fr.min.x, fr.max.y - 2.0), Vec2::new(w, 2.0)), 0.0, shade(color, 0.7));
    }
    let mut x = r.min.x + 12.0;
    while x < r.max.x {
        p.rect_filled(Rect::from_min_size(Pos2::new(x, r.min.y), Vec2::new(2.0, h)), 0.0, Color32::from_black_alpha(70));
        x += 12.0;
    }
}

// course emblem: pixel shield tile with label and 3 level dots
pub fn emblem(ui: &mut egui::Ui, label: &str, color: Color32, size: f32) {
    let (r, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    draw_emblem(ui.painter(), r, label, color);
}
pub fn draw_emblem(p: &egui::Painter, r: Rect, label: &str, color: Color32) {
    let c = 6.0;
    p.rect_filled(r, 0.0, EDGE);
    p.rect_filled(r.shrink2(Vec2::new(0.0, c)), 0.0, EDGE);
    let inner = r.shrink(3.0);
    p.rect_filled(inner, 0.0, color);
    p.rect_filled(Rect::from_min_size(inner.min, Vec2::new(inner.width(), 4.0)), 0.0, shade(color, 1.3));
    p.rect_filled(Rect::from_min_size(Pos2::new(inner.min.x, inner.max.y - 5.0), Vec2::new(inner.width(), 5.0)), 0.0, shade(color, 0.7));
    for (dx, dy) in [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)] {
        let cp = Pos2::new(if dx == 0.0 { r.min.x } else { r.max.x - 4.0 }, if dy == 0.0 { r.min.y } else { r.max.y - 4.0 });
        p.rect_filled(Rect::from_min_size(cp, Vec2::splat(4.0)), 0.0, BG);
    }
    let fs = if label.len() > 3 { r.width() * 0.2 } else { r.width() * 0.3 };
    p.text(r.center() - Vec2::new(0.0, r.height() * 0.08), egui::Align2::CENTER_CENTER, label, FontId::new(fs.max(8.0), press()), Color32::WHITE);
    for i in 0..3 {
        let d = r.width() * 0.07;
        let cx = r.center().x + (i as f32 - 1.0) * d * 2.2;
        let cy = r.max.y - r.height() * 0.2;
        p.rect_filled(Rect::from_center_size(Pos2::new(cx, cy), Vec2::splat(d)), 0.0, if i == 0 { Color32::WHITE } else { shade(color, 0.6) });
    }
}

/// Dungeon wall behind every screen: dim stone bricks with a vignette towards the edges.
pub fn floor(p: &egui::Painter, r: Rect, scan: bool) {
    p.rect_filled(r, 0.0, BG);
    p.add(tile_shape(r, "wall", Color32::from_gray(58)));
    let v = Color32::from_black_alpha(90);
    p.rect_filled(Rect::from_min_size(r.min, Vec2::new(r.width(), 24.0)), 0.0, v);
    p.rect_filled(Rect::from_min_size(Pos2::new(r.min.x, r.max.y - 24.0), Vec2::new(r.width(), 24.0)), 0.0, v);
    if scan {
        let mut y = r.min.y;
        while y < r.max.y {
            p.rect_filled(Rect::from_min_size(Pos2::new(r.min.x, y), Vec2::new(r.width(), 1.0)), 0.0, Color32::from_black_alpha(28));
            y += 3.0;
        }
    }
}

pub fn panel_resp(ui: &mut egui::Ui, fill: Color32, edge: Color32, f: impl FnOnce(&mut egui::Ui)) -> egui::Response {
    let rect = framed(ui, fill, edge, f).response.rect;
    let r = ui.interact(rect, ui.scope_id().with(("pr", rect.min.x as i32, rect.min.y as i32)), Sense::click());
    if r.hovered() { ui.painter().rect_stroke(rect.shrink(1.0), 2.0, Stroke::new(1.0, Color32::from_white_alpha(40)), egui::StrokeKind::Inside); }
    r.on_hover_cursor(egui::CursorIcon::PointingHand)
}

// ---------- pixel art from Pixelrepo's free collection ----------
pub static TEX: OnceLock<HashMap<&'static str, (egui::TextureId, Vec2)>> = OnceLock::new();

/// Every embedded picture: Pixelrepo Free collection (credited in Settings and README).
pub const ART: &[(&str, &[u8])] = &[
    ("potion_red", include_bytes!("../assets/art/potion_red.png")),
    ("crystal_purple", include_bytes!("../assets/art/crystal_purple.png")),
    ("crystal_blue", include_bytes!("../assets/art/crystal_blue.png")),
    ("torch", include_bytes!("../assets/art/torch.png")),
    ("chest", include_bytes!("../assets/art/chest.png")),
    ("book", include_bytes!("../assets/art/book.png")),
    ("sword", include_bytes!("../assets/art/sword.png")),
    ("shield", include_bytes!("../assets/art/shield.png")),
    ("helmet", include_bytes!("../assets/art/helmet.png")),
    ("map", include_bytes!("../assets/art/map.png")),
    ("key", include_bytes!("../assets/art/key.png")),
    ("staff", include_bytes!("../assets/art/staff.png")),
    ("cauldron", include_bytes!("../assets/art/cauldron.png")),
    ("scroll_fire", include_bytes!("../assets/art/scroll_fire.png")),
    ("coin_gold", include_bytes!("../assets/art/coin_gold.png")),
    ("gem_green", include_bytes!("../assets/art/gem_green.png")),
    ("orb_red", include_bytes!("../assets/art/orb_red.png")),
    ("axe", include_bytes!("../assets/art/axe.png")),
    ("banner", include_bytes!("../assets/art/banner.png")),
    ("banner_royal", include_bytes!("../assets/art/banner_royal.png")),
    ("bow", include_bytes!("../assets/art/bow.png")),
    ("campfire", include_bytes!("../assets/art/campfire.png")),
    ("chalice", include_bytes!("../assets/art/chalice.png")),
    ("coins", include_bytes!("../assets/art/coins.png")),
    ("console", include_bytes!("../assets/art/console.png")),
    ("cpu", include_bytes!("../assets/art/cpu.png")),
    ("crown", include_bytes!("../assets/art/crown.png")),
    ("cup_silver", include_bytes!("../assets/art/cup_silver.png")),
    ("flame", include_bytes!("../assets/art/flame.png")),
    ("gear", include_bytes!("../assets/art/gear.png")),
    ("hammer", include_bytes!("../assets/art/hammer.png")),
    ("hat_wizard", include_bytes!("../assets/art/hat_wizard.png")),
    ("heart", include_bytes!("../assets/art/heart.png")),
    ("helmet_bronze", include_bytes!("../assets/art/helmet_bronze.png")),
    ("helmet_knight", include_bytes!("../assets/art/helmet_knight.png")),
    ("hourglass", include_bytes!("../assets/art/hourglass.png")),
    ("keyhole", include_bytes!("../assets/art/keyhole.png")),
    ("lantern", include_bytes!("../assets/art/lantern.png")),
    ("letter", include_bytes!("../assets/art/letter.png")),
    ("medallion", include_bytes!("../assets/art/medallion.png")),
    ("money_bag", include_bytes!("../assets/art/money_bag.png")),
    ("mystic_figure", include_bytes!("../assets/art/mystic_figure.png")),
    ("potion_magic", include_bytes!("../assets/art/potion_magic.png")),
    ("scroll_lightning", include_bytes!("../assets/art/scroll_lightning.png")),
    ("spellbook", include_bytes!("../assets/art/spellbook.png")),
    ("amulet", include_bytes!("../assets/art/chest2.png")),
];

/// Dungeon skin tiles (Kenney "Tiny Dungeon" and "UI Pack: Pixel Adventure", CC0). `true` = repeat-wrapped.
pub const SKIN: &[(&str, &[u8], bool)] = &[
    ("wall", include_bytes!("../assets/ui/wall.png"), true),
    ("wall2", include_bytes!("../assets/ui/wall2.png"), true),
    ("ledge", include_bytes!("../assets/ui/ledge.png"), true),
    ("frame_stone", include_bytes!("../assets/ui/frame_stone.png"), false),
    ("frame_wood", include_bytes!("../assets/ui/frame_wood.png"), false),
    ("frame_plain", include_bytes!("../assets/ui/frame_plain.png"), false),
    ("block", include_bytes!("../assets/ui/block.png"), false),
    ("parch", include_bytes!("../assets/ui/parch.png"), false),
    ("ribbon", include_bytes!("../assets/ui/ribbon.png"), false),
    ("door", include_bytes!("../assets/ui/door.png"), false),
    ("chest_closed", include_bytes!("../assets/ui/chest_closed.png"), false),
    ("chest_open", include_bytes!("../assets/ui/chest_open.png"), false),
    ("banner_wall", include_bytes!("../assets/ui/banner_wall.png"), false),
];

/// Badge and reward icons from the GTORAVERSE Achievement Icon Pack (credited in Settings and README).
pub const BADGES: &[(&str, &[u8])] = &[
    ("bronze_shield", include_bytes!("../assets/badges/bronze_shield.png")), ("silver_shield", include_bytes!("../assets/badges/silver_shield.png")), ("gold_shield", include_bytes!("../assets/badges/gold_shield.png")),
    ("bronze_star", include_bytes!("../assets/badges/bronze_star.png")), ("silver_star", include_bytes!("../assets/badges/silver_star.png")), ("gold_star", include_bytes!("../assets/badges/gold_star.png")),
    ("bronze_crown", include_bytes!("../assets/badges/bronze_crown.png")), ("silver_crown", include_bytes!("../assets/badges/silver_crown.png")), ("gold_crown", include_bytes!("../assets/badges/gold_crown.png")),
    ("bronze_trophy", include_bytes!("../assets/badges/bronze_trophy.png")), ("silver_trophy", include_bytes!("../assets/badges/silver_trophy.png")), ("gold_trophy", include_bytes!("../assets/badges/gold_trophy.png")),
    ("bronze_medal", include_bytes!("../assets/badges/bronze_medal.png")), ("silver_medal", include_bytes!("../assets/badges/silver_medal.png")), ("gold_medal", include_bytes!("../assets/badges/gold_medal.png")),
    ("bronze_pin", include_bytes!("../assets/badges/bronze_pin.png")), ("silver_pin", include_bytes!("../assets/badges/silver_pin.png")), ("gold_pin", include_bytes!("../assets/badges/gold_pin.png")),
    ("bronze_coin", include_bytes!("../assets/badges/bronze_coin.png")), ("silver_coin", include_bytes!("../assets/badges/silver_coin.png")), ("gold_coin", include_bytes!("../assets/badges/gold_coin.png")),
    ("bronze_heart", include_bytes!("../assets/badges/bronze_heart.png")), ("silver_heart", include_bytes!("../assets/badges/silver_heart.png")), ("gold_heart", include_bytes!("../assets/badges/gold_heart.png")),
    ("red_heart", include_bytes!("../assets/badges/red_heart.png")),
    ("diamond", include_bytes!("../assets/badges/diamond.png")), ("diamond_crystal", include_bytes!("../assets/badges/diamond_crystal.png")),
    ("brown_chest", include_bytes!("../assets/badges/brown_chest.png")), ("open_brown_chest", include_bytes!("../assets/badges/open_brown_chest.png")),
    ("silver_chest", include_bytes!("../assets/badges/silver_chest.png")), ("open_silver_chest", include_bytes!("../assets/badges/open_silver_chest.png")),
    ("red_gold_chest", include_bytes!("../assets/badges/red_gold_chest.png")), ("open_gold_red_chest", include_bytes!("../assets/badges/open_gold_red_chest.png")),
    ("gift_box_red", include_bytes!("../assets/badges/gift_box_red.png")), ("gift_box_blue", include_bytes!("../assets/badges/gift_box_blue.png")), ("gift_box_green", include_bytes!("../assets/badges/gift_box_green.png")),
];

#[cfg_attr(not(test), allow(dead_code))]
pub fn art_exists(name: &str) -> bool { ART.iter().any(|(n, _)| *n == name) || BADGES.iter().any(|(n, _)| *n == name) }

pub fn load_textures(ctx: &egui::Context) -> Vec<egui::TextureHandle> {
    let mut map = HashMap::new();
    let mut handles = vec![];
    let all = ART.iter().map(|(n, b)| (*n, *b, false)).chain(BADGES.iter().map(|(n, b)| (*n, *b, false))).chain(SKIN.iter().copied());
    for (name, bytes, repeat) in all {
        if let Ok(img) = image::load_from_memory(bytes) {
            let rgba = img.to_rgba8();
            let size = [rgba.width() as usize, rgba.height() as usize];
            let ci = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
            let opts = if repeat { egui::TextureOptions::NEAREST_REPEAT } else { egui::TextureOptions::NEAREST };
            let h = ctx.load_texture(name, ci, opts);
            map.insert(name, (h.id(), Vec2::new(size[0] as f32, size[1] as f32)));
            handles.push(h);
        }
    }
    let _ = TEX.set(map);
    handles
}

/// Draw a texture centred in `bx`, scaled by a whole number so pixels stay sharp.
pub fn draw_icon(p: &egui::Painter, bx: Rect, name: &str, tint: Color32) {
    let Some((id, sz)) = TEX.get().and_then(|m| m.get(name)) else { return };
    let k = (bx.width() / sz.x).min(bx.height() / sz.y).floor().max(1.0);
    let r = Rect::from_center_size(bx.center(), *sz * k);
    p.image(*id, r, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), tint);
}

pub fn icon(ui: &mut egui::Ui, name: &str, size: f32) {
    let (r, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    draw_icon(ui.painter(), r, name, Color32::WHITE);
}


#[cfg(test)]
mod tests {
    #[test]
    fn all_art_decodes() {
        for (n, b) in super::ART.iter().chain(super::BADGES) { assert!(image::load_from_memory(b).is_ok(), "{} does not decode", n); }
        for (n, b, _) in super::SKIN { assert!(image::load_from_memory(b).is_ok(), "skin {} does not decode", n); }
    }
}
