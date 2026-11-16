//! All course content. Every lesson follows the same three steps:
//!   1. LEARN    - short slides with a picture
//!   2. COMPLETE - pick or drag the missing code into the gap (plus concept checks)
//!   3. CODE     - type the code (first copy it, then repeat it from memory)
//!
//! Every chapter ends with a checkpoint project. The HTML, CSS and JavaScript chapter topics follow
//! the beginner path used by popular free curricula (e.g. Codédex). All wording, examples and
//! project themes are original to Pixel Learn.
pub use crate::data::*;

mod css;
mod html;
mod js;
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
pub fn l(title: &'static str, slides: &[S], qs: Vec<Q>) -> Lesson { Lesson { title, slides: slides.to_vec(), qs, project: false } }
/// A checkpoint project that closes a chapter.
pub fn p(title: &'static str, slides: &[S], qs: Vec<Q>) -> Lesson { Lesson { title, slides: slides.to_vec(), qs, project: true } }
/// A chapter without a blurb (older courses).
pub fn m(name: &'static str, lessons: Vec<Lesson>) -> Module { Module { name, desc: "", lessons } }
/// A chapter with a one-line blurb shown in the course preview.
pub fn ch(name: &'static str, desc: &'static str, lessons: Vec<Lesson>) -> Module { Module { name, desc, lessons } }

pub fn all() -> Vec<Course> { vec![html::course(), css::course(), js::course(), python::course(), sql::course()] }

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
            assert!(crate::art::art_exists(c.badge.art), "{}: unknown badge art", c.name);
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
}
