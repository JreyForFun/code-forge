//! All course content. Every lesson follows the same three steps:
//!   1. LEARN    - short slides with a picture
//!   2. COMPLETE - pick or drag the missing code into the gap (plus concept checks)
//!   3. CODE     - type the code (first copy it, then repeat it from memory)
//!
//! Every chapter ends with a checkpoint project. The HTML, CSS and JavaScript chapter topics follow
//! the beginner path used by popular free curricula (e.g. Codédex). All wording, examples and
//! project themes are original to Code Forge.
pub use crate::data::*;

mod css;
mod css_deep;
mod html;
mod html_deep;
mod js;
mod js_deep;
mod python;
mod sql;

pub type S = (&'static str, &'static str);

pub fn fill(prompt: &'static str, code: &'static str, opts: &[&'static str], right: usize, why: &'static str) -> Q {
    Q::Fill { prompt, code, opts: opts.to_vec(), right, why }
}
pub fn copy(prompt: &'static str, answer: &'static str, why: &'static str) -> Q { Q::Type { prompt, answer, why, guide: true } }
pub fn recall(prompt: &'static str, answer: &'static str, why: &'static str) -> Q { Q::Type { prompt, answer, why, guide: false } }
pub fn quiz(q: &'static str, opts: &[&'static str], right: usize, why: &'static str) -> Q { Q::Mc { q, opts: opts.to_vec(), right, why } }
/// A regular lesson.
pub fn l(title: &'static str, slides: &[S], qs: Vec<Q>) -> Lesson { Lesson { title, slides: slides.to_vec(), qs, project: false, explain: "", summary: vec![] } }
/// A checkpoint project that closes a chapter.
pub fn p(title: &'static str, slides: &[S], qs: Vec<Q>) -> Lesson { Lesson { title, slides: slides.to_vec(), qs, project: true, explain: "", summary: vec![] } }

impl Lesson {
    /// Add the long explanation and the key points to a lesson written in one place.
    pub fn with(mut self, explain: &'static str, summary: &[&'static str]) -> Self { self.explain = explain; self.summary = summary.to_vec(); self }
}

/// The deep part of a lesson, kept in a per-course table next to the lesson list:
/// a long explanation, the key points, and extra practice.
pub struct Deep { pub title: &'static str, pub explain: &'static str, pub summary: &'static [&'static str], pub qs: Vec<Q> }
pub fn d(title: &'static str, explain: &'static str, summary: &'static [&'static str], qs: Vec<Q>) -> Deep { Deep { title, explain, summary, qs } }

/// Attach deep content to the lessons with the same title. Extra concept and fill questions go
/// before the typing questions, so a lesson still ends with "type it" and "from memory".
pub fn attach(mut c: Course, deep: Vec<Deep>) -> Course {
    for dp in deep {
        let Some(ls) = c.modules.iter_mut().flat_map(|m| m.lessons.iter_mut()).find(|l| l.title == dp.title) else {
            if cfg!(test) { panic!("{}: deep content for unknown lesson '{}'", c.name, dp.title) } else { continue }
        };
        ls.explain = dp.explain;
        ls.summary = dp.summary.to_vec();
        let at = ls.qs.iter().position(|q| matches!(q, Q::Type { .. })).unwrap_or(ls.qs.len());
        let (typing, extra): (Vec<Q>, Vec<Q>) = dp.qs.into_iter().partition(|q| matches!(q, Q::Type { .. }));
        let tail = ls.qs.split_off(at);
        ls.qs.extend(extra);
        ls.qs.extend(tail);
        ls.qs.extend(typing);
    }
    c
}
/// A chapter with a one-line blurb shown in the course preview.
pub fn ch(name: &'static str, desc: &'static str, lessons: Vec<Lesson>) -> Module { Module { name, desc, lessons } }

pub fn all() -> Vec<Course> {
    vec![attach(html::course(), html_deep::deep()), attach(css::course(), css_deep::deep()), attach(js::course(), js_deep::deep()), python::course(), sql::course()]
}

/// 1.0 saves keyed lessons by position (`course.chapter.lesson`). Map them to stable ids.
/// (old key, course slug, lesson title in the current content)
const OLD_KEYS: &[(&str, &str, &str)] = &[
    ("0.0.0", "html", "Your First Tags"), ("0.0.1", "html", "Headings"), ("0.1.0", "html", "Lists"), ("0.1.1", "html", "Links"),
    ("0.2.0", "html", "Page Skeleton"), ("0.2.1", "html", "Comments and Attributes"), ("0.3.0", "html", "Your First Form"), ("0.3.1", "html", "Why Semantics?"),
    ("1.0.0", "css", "What Is CSS?"), ("1.0.1", "css", "Class and ID Selectors"), ("1.1.0", "css", "Colors"), ("1.1.1", "css", "Fonts"),
    ("1.2.0", "css", "Padding"), ("1.2.1", "css", "Borders and Corners"), ("1.3.0", "css", "Display Flex"), ("1.3.1", "css", "Every Element Is a Box"),
    ("2.0.0", "javascript", "Hello, Console"), ("2.0.1", "javascript", "let and const"), ("2.1.0", "javascript", "Arithmetic"), ("2.1.1", "javascript", "Comparison Operators"),
    ("2.2.0", "javascript", "else"), ("2.2.1", "javascript", "for Loops"), ("2.3.0", "javascript", "Return Values"), ("2.3.1", "javascript", "Making Arrays"),
    ("3.0.0", "python", "Printing"), ("3.0.1", "python", "Strings and Comments"), ("3.1.0", "python", "Variables"), ("3.1.1", "python", "Math"),
    ("3.2.0", "python", "If Statements"), ("3.2.1", "python", "Else"), ("3.3.0", "python", "For Loops"), ("3.3.1", "python", "While Loops"),
    ("3.4.0", "python", "Lists"), ("3.4.1", "python", "Functions"),
    ("4.0.0", "sql", "SELECT"), ("4.0.1", "sql", "Choosing Columns"), ("4.1.0", "sql", "WHERE"), ("4.1.1", "sql", "AND / OR"),
    ("4.2.0", "sql", "ORDER BY"), ("4.2.1", "sql", "LIMIT"), ("4.3.0", "sql", "COUNT and SUM"), ("4.3.1", "sql", "GROUP BY"),
];

fn is_old_key(k: &str) -> bool { let p: Vec<&str> = k.split('.').collect(); p.len() == 3 && p.iter().all(|x| !x.is_empty() && x.chars().all(|c| c.is_ascii_digit())) }

/// Convert position keys from a 1.0 save to stable lesson ids. Unknown old keys are dropped.
pub fn migrate(s: &mut crate::save::Save) {
    let map = |k: &str| OLD_KEYS.iter().find(|(o, _, _)| *o == k).map(|(_, c, t)| format!("{}/{}", c, slugify(t)));
    let done: Vec<String> = s.done.iter().cloned().collect();
    for k in done { if is_old_key(&k) { s.done.remove(&k); if let Some(n) = map(&k) { s.done.insert(n); } } }
    let best: Vec<(String, (u32, u32))> = s.best.iter().map(|(k, v)| (k.clone(), *v)).collect();
    for (k, v) in best { if is_old_key(&k) { s.best.remove(&k); if let Some(n) = map(&k) { s.best.insert(n, v); } } }
    s.qdone.retain(|k, _| !is_old_key(k));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interp::{run, Kind};
    use std::collections::HashSet;

    #[test]
    fn content_is_well_formed() {
        let cs = all();
        assert_eq!(cs.len(), 5);
        let mut slugs = HashSet::new();
        for c in &cs {
            assert!(slugs.insert(c.slug), "duplicate course slug {}", c.slug);
            assert!(!c.modules.is_empty(), "{} has no chapters", c.name);
            for m in ["bronze", "silver", "gold"] { assert!(crate::art::art_exists(&format!("{}_{}", m, c.badge.art)), "{}: no {} badge icon for {}", c.name, m, c.badge.art); }
            assert!((3..=6).contains(&c.teaches.len()), "{}: teaches needs 3-6 items", c.name);
            let mut ids = HashSet::new();
            for (mi, m) in c.modules.iter().enumerate() {
                assert!(!m.lessons.is_empty(), "{} / {} has no lessons", c.name, m.name);
                for (li, l) in m.lessons.iter().enumerate() {
                    let at = format!("{} / {} / {}", c.name, m.name, l.title);
                    assert!(ids.insert(c.lesson_id(mi, li)), "{at}: duplicate lesson id");
                    assert!(!l.slides.is_empty(), "{at}: no slides");
                    assert!(l.qs.len() >= 2, "{at}: needs at least 2 questions");
                    for (_, img) in &l.slides { assert!(crate::art::art_exists(img), "{at}: unknown picture '{img}'"); }
                    for q in &l.qs {
                        match q {
                            Q::Mc { opts, right, .. } => { assert!(*right < opts.len(), "{at}: bad answer index"); assert!(opts.len() >= 2, "{at}: needs 2+ options"); }
                            Q::Fill { code, opts, right, .. } => {
                                assert!(*right < opts.len(), "{at}: bad answer index");
                                assert_eq!(code.matches("___").count(), 1, "{at}: fill question needs exactly one gap");
                                let set: HashSet<_> = opts.iter().collect();
                                assert_eq!(set.len(), opts.len(), "{at}: duplicate options");
                            }
                            Q::Type { answer, .. } => assert!(!answer.trim().is_empty(), "{at}: empty answer"),
                        }
                    }
                }
            }
        }
    }

    /// Codédex-depth web courses: chapter counts and a checkpoint project closing every chapter.
    #[test]
    fn web_courses_have_depth_and_checkpoints() {
        let cs = all();
        for (slug, min_ch) in [("html", 4), ("css", 5), ("javascript", 8)] {
            let c = cs.iter().find(|c| c.slug == slug).unwrap();
            assert!(c.modules.len() >= min_ch, "{} needs {} chapters", slug, min_ch);
            for m in &c.modules {
                assert!(m.lessons.last().unwrap().project, "{} / {} must end with a checkpoint project", slug, m.name);
                assert!(!m.desc.is_empty(), "{} / {} needs a blurb", slug, m.name);
            }
        }
        let web: usize = cs.iter().filter(|c| ["html", "css", "javascript"].contains(&c.slug)).map(|c| c.lesson_count()).sum();
        assert!(web >= 80, "HTML+CSS+JS have only {} lessons", web);
    }

    /// 003 FR-1/FR-2: every lesson is deep: a long explanation, a summary and enough practice.
    #[test]
    fn every_lesson_is_deep() {
        let mut short = vec![];
        for c in all() {
            for m in &c.modules {
                for l in &m.lessons {
                    let words = l.explain.split_whitespace().count();
                    if words < 120 { short.push(format!("{} / {}: explanation has {} words (needs 120)", c.name, l.title, words)); }
                    if !(3..=6).contains(&l.summary.len()) { short.push(format!("{} / {}: summary has {} points (needs 3-6)", c.name, l.title, l.summary.len())); }
                    if l.qs.len() < 6 { short.push(format!("{} / {}: {} questions (needs 6)", c.name, l.title, l.qs.len())); }
                    if !matches!(l.qs.last(), Some(Q::Type { .. })) { short.push(format!("{} / {}: must end with a typing step", c.name, l.title)); }
                }
            }
        }
        assert!(short.is_empty(), "{} problems:\n{}", short.len(), short.join("\n"));
    }

    /// SC-002: typing the expected answer in any course never produces an error.
    #[test]
    fn content_answers_run_clean() {
        let mut failures = vec![];
        for c in all() {
            for m in &c.modules {
                for l in &m.lessons {
                    for q in &l.qs {
                        if let Q::Type { answer, .. } = q {
                            let out = run(c.lang, answer);
                            if let Some((_, e)) = out.iter().find(|(k, _)| *k == Kind::Err) {
                                failures.push(format!("{} / {}: `{}` -> {}", c.name, l.title, answer.replace('\n', "⏎"), e));
                            }
                        }
                    }
                }
            }
        }
        assert!(failures.is_empty(), "{} answers fail:\n{}", failures.len(), failures.join("\n"));
    }

    /// Filling the gap with the right option gives code that runs (for runnable languages).
    #[test]
    fn filled_gaps_run_clean() {
        let mut failures = vec![];
        for c in all() {
            if !["javascript", "python"].contains(&c.lang) { continue; }
            for m in &c.modules {
                for l in &m.lessons {
                    for q in &l.qs {
                        if let Q::Fill { code, opts, right, .. } = q {
                            let src = code.replace("___", opts[*right]);
                            if !src.contains("console.log") && !src.contains("print(") { continue; }
                            // gaps are fragments, so names defined elsewhere in the lesson are fine; syntax must be valid
                            if let Some((_, e)) = run(c.lang, &src).iter().find(|(k, e)| *k == Kind::Err && !e.contains("is not defined")) { failures.push(format!("{} / {}: `{}` -> {}", c.name, l.title, src.replace('\n', "⏎"), e)); }
                        }
                    }
                }
            }
        }
        assert!(failures.is_empty(), "{} filled gaps fail:\n{}", failures.len(), failures.join("\n"));
    }

    #[test]
    fn migration_targets_exist_and_map() {
        let cs = all();
        let ids: HashSet<String> = cs.iter().flat_map(|c| (0..c.modules.len()).flat_map(move |m| (0..c.modules[m].lessons.len()).map(move |l| c.lesson_id(m, l)))).collect();
        for (k, c, t) in OLD_KEYS { assert!(ids.contains(&format!("{}/{}", c, slugify(t))), "old key {} -> {}/{} has no lesson", k, c, t); }
        let mut s = crate::save::Save::new();
        s.done.insert("0.0.0".into()); s.done.insert("9.9.9".into()); s.done.insert("css/colors".into());
        s.best.insert("2.0.0".into(), (90, 2));
        s.qdone.insert("0.0.1".into(), vec![1]);
        migrate(&mut s);
        assert!(s.done.contains("html/your-first-tags"));
        assert!(s.done.contains("css/colors"));
        assert!(!s.done.iter().any(|k| is_old_key(k)));
        assert_eq!(s.best.get("javascript/hello-console"), Some(&(90, 2)));
        assert!(s.qdone.is_empty());
    }

    /// Every checkpoint project tells ONE story: the code on its slide is exactly what the learner
    /// types, the explanation shows that same code, the questions don't repeat each other, and the
    /// code runs cleanly.
    #[test]
    fn checkpoint_projects_tell_one_story() {
        let mut bad = vec![];
        for c in all() {
            for m in &c.modules {
                for l in m.lessons.iter().filter(|l| l.project) {
                    let id = format!("{} / {}", c.name, l.title);
                    let head = if l.title.starts_with("Final") { "FINAL PROJECT" } else { "CHECKPOINT PROJECT" };
                    if !l.slides.first().is_some_and(|s| s.0.starts_with(head)) { bad.push(format!("{}: first slide must start with {}", id, head)); }
                    if l.qs.len() != 7 { bad.push(format!("{}: {} questions, expected 7", id, l.qs.len())); }
                    if l.summary.len() != 4 || l.explain.len() < 600 { bad.push(format!("{}: needs a full explanation and 4 summary points", id)); }
                    let program = l.slides.last().map(|s| s.0).unwrap_or("");
                    let copies: Vec<&str> = l.qs.iter().filter_map(|q| match q { Q::Type { answer, guide: true, .. } => Some(*answer), _ => None }).collect();
                    match copies.as_slice() {
                        [a] if crate::lesson::norm_code(a, c.lang) == crate::lesson::norm_code(program, c.lang) => {}
                        _ => bad.push(format!("{}: the typed project must be exactly the code on the slide", id)),
                    }
                    for line in program.lines().map(str::trim).filter(|x| !x.is_empty()) {
                        if !l.explain.contains(line) { bad.push(format!("{}: explanation is missing the line `{}`", id, line)); }
                    }
                    if run(c.lang, program).iter().any(|(k, _)| *k == Kind::Err) { bad.push(format!("{}: the project code doesn't run", id)); }
                    let mut prompts = HashSet::new();
                    let mut fills = HashSet::new();
                    for q in &l.qs {
                        let (p, fill_answer) = match q { Q::Mc { q, .. } => (*q, None), Q::Fill { prompt, opts, right, .. } => (*prompt, opts.get(*right).copied()), Q::Type { prompt, .. } => (*prompt, None) };
                        if !prompts.insert(p) { bad.push(format!("{}: question asked twice: {}", id, p)); }
                        if let Some(a) = fill_answer && !fills.insert(a) { bad.push(format!("{}: two gaps with the same answer `{}`", id, a)); }
                    }
                }
            }
        }
        assert!(bad.is_empty(), "{} project problems:\n{}", bad.len(), bad.join("\n"));
    }
}
