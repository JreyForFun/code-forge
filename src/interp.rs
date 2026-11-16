//! A small, safe interpreter for beginner Python and JavaScript snippets, plus previews for HTML,
//! CSS and SQL. Supports variables, numbers, strings, booleans, lists/arrays, objects/dicts,
//! template literals and f-strings, arithmetic, comparisons, if/elif/else, while, for (range,
//! list, C-style, for-of), break/continue, functions (declarations, expressions, arrows) with
//! return, string and array methods, and a simulated web page for DOM lessons.
//! Every run is bounded (steps, nesting, output size, source size), so it can never hang the app.
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum V { N(f64), S(String), B(bool), L(Vec<V>), O(Vec<(String, V)>), Nil }

/// Output line kinds. `Table` holds rows joined by '\n' with cells split by '\x1f' (first row = header);
/// `Styled` holds a css::Style encoding; `Page` is a simulated web page change.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Kind { Out, Err, H1, H2, P, Li, Btn, Img, Table, Styled, Page }

const MAX_STEPS: u32 = 20_000;
const MAX_DEPTH: u32 = 48;
const MAX_OUT: usize = 500;
const MAX_SRC: usize = 20_000;
const MAX_LIST: usize = 10_000;
const EL: &str = "__el";
const STYLE: &str = "__style";

fn num_str(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 { format!("{}", n as i64) } else { let s = format!("{:.10}", n); s.trim_end_matches('0').trim_end_matches('.').to_string() }
}

#[derive(Debug, Clone, PartialEq)]
enum T { Num(f64), Str(String), Tpl(String, bool), Id(String), Op(String) }

fn lex(s: &str) -> Result<Vec<T>, String> {
    let c: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut out = vec![];
    let read_str = |i: &mut usize, q: char| -> Result<String, String> {
        *i += 1;
        let mut v = String::new();
        while *i < c.len() && c[*i] != q {
            if c[*i] == '\\' && *i + 1 < c.len() {
                *i += 1;
                v.push(match c[*i] { 'n' => '\n', 't' => '\t', x => x });
            } else { v.push(c[*i]); }
            *i += 1;
        }
        if *i >= c.len() { return Err("missing closing quote".into()); }
        *i += 1;
        Ok(v)
    };
    while i < c.len() {
        let ch = c[i];
        if ch.is_whitespace() { i += 1; }
        else if ch.is_ascii_digit() || (ch == '.' && i + 1 < c.len() && c[i + 1].is_ascii_digit()) {
            let st = i;
            while i < c.len() && (c[i].is_ascii_digit() || c[i] == '.') { i += 1; }
            out.push(T::Num(c[st..i].iter().collect::<String>().parse().map_err(|_| "bad number".to_string())?));
        } else if (ch == 'f' || ch == 'F') && i + 1 < c.len() && (c[i + 1] == '"' || c[i + 1] == '\'') && (i == 0 || !(c[i - 1].is_alphanumeric() || c[i - 1] == '_')) {
            i += 1;
            let q = c[i];
            out.push(T::Tpl(read_str(&mut i, q)?, true));
        } else if ch == '`' {
            out.push(T::Tpl(read_str(&mut i, '`')?, false));
        } else if ch == '"' || ch == '\'' {
            out.push(T::Str(read_str(&mut i, ch)?));
        } else if ch.is_alphabetic() || ch == '_' || ch == '$' {
            let st = i;
            while i < c.len() && (c[i].is_alphanumeric() || c[i] == '_' || c[i] == '$' || (c[i] == '.' && i + 1 < c.len() && (c[i + 1].is_alphabetic() || c[i + 1] == '_'))) { i += 1; }
            out.push(T::Id(c[st..i].iter().collect()));
        } else {
            let two: String = c[i..(i + 2).min(c.len())].iter().collect();
            let three: String = c[i..(i + 3).min(c.len())].iter().collect();
            if three == "===" || three == "!==" { out.push(T::Op(three)); i += 3; }
            else if ["==", "!=", "<=", ">=", "&&", "||", "**", "//", "=>"].contains(&two.as_str()) { out.push(T::Op(two)); i += 2; }
            else if "+-*/%()<>,![]{}:.?".contains(ch) { out.push(T::Op(ch.to_string())); i += 1; }
            else { return Err(format!("unexpected '{}'", ch)); }
        }
    }
    Ok(out)
}

fn truthy(v: &V) -> bool {
    match v { V::N(n) => *n != 0.0, V::S(s) => !s.is_empty(), V::B(b) => *b, V::L(l) => !l.is_empty(), V::O(_) => true, V::Nil => false }
}
fn num(v: &V) -> Result<f64, String> {
    match v {
        V::N(n) => Ok(*n),
        V::B(b) => Ok(*b as i32 as f64),
        V::S(s) => s.trim().parse().map_err(|_| format!("'{}' is not a number", s)),
        V::L(_) => Err("a list is not a number".into()),
        V::O(_) => Err("an object is not a number".into()),
        V::Nil => Err("nothing (None/undefined) is not a number".into()),
    }
}
fn field<'a>(o: &'a [(String, V)], k: &str) -> Option<&'a V> { o.iter().find(|(n, _)| n == k).map(|(_, v)| v) }
fn el_of(v: &V) -> Option<(String, bool)> {
    if let V::O(o) = v {
        if let Some(V::S(s)) = field(o, EL) { return Some((s.clone(), false)); }
        if let Some(V::S(s)) = field(o, STYLE) { return Some((s.clone(), true)); }
    }
    None
}

/// Split "a, b" at top-level commas.
fn split_args(s: &str) -> Vec<String> {
    let mut out = vec![];
    let mut depth = 0i32;
    let mut q: Option<char> = None;
    let mut cur = String::new();
    for ch in s.chars() {
        if let Some(qc) = q { cur.push(ch); if ch == qc { q = None; } continue; }
        match ch {
            '"' | '\'' | '`' => { q = Some(ch); cur.push(ch); }
            '(' | '[' | '{' => { depth += 1; cur.push(ch); }
            ')' | ']' | '}' => { depth -= 1; cur.push(ch); }
            ',' if depth == 0 => { out.push(cur.trim().to_string()); cur.clear(); }
            _ => cur.push(ch),
        }
    }
    if !cur.trim().is_empty() { out.push(cur.trim().to_string()); }
    out
}

/// Bracket balance ignoring strings: (paren+square+brace).
fn depth_of(s: &str) -> i32 {
    let mut d = 0;
    let mut q: Option<char> = None;
    for ch in s.chars() {
        if let Some(qc) = q { if ch == qc { q = None; } continue; }
        match ch { '"' | '\'' | '`' => q = Some(ch), '(' | '[' | '{' => d += 1, ')' | ']' | '}' => d -= 1, _ => {} }
    }
    d
}

/// Position of the last top-level '.' (not inside brackets or strings).
fn last_top_dot(s: &str) -> Option<usize> {
    let mut d = 0;
    let mut q: Option<char> = None;
    let mut last = None;
    for (i, ch) in s.char_indices() {
        if let Some(qc) = q { if ch == qc { q = None; } continue; }
        match ch { '"' | '\'' | '`' => q = Some(ch), '(' | '[' | '{' => d += 1, ')' | ']' | '}' => d -= 1, '.' if d == 0 => last = Some(i), _ => {} }
    }
    last
}

struct Line { ind: usize, text: String, no: usize }

/// Turn source into indented logical lines. JS braces become indentation; multi-line object/array
/// literals and calls are joined into one line.
fn normalize(src: &str, js: bool) -> Vec<Line> {
    let mut out = vec![];
    let mut depth: usize = 0;
    let raw: Vec<&str> = src.lines().collect();
    let mut k = 0;
    while k < raw.len() {
        let no = k + 1;
        let line = raw[k];
        k += 1;
        let mut t = line.trim().to_string();
        if t.is_empty() { continue; }
        t = strip_comment(&t, js);
        if t.is_empty() { continue; }
        if js {
            t = t.trim_end_matches(';').trim().to_string();
            while t.starts_with('}') { depth = depth.saturating_sub(1); t = t[1..].trim().trim_start_matches(')').trim_start_matches(';').trim().to_string(); }
            if t.is_empty() || t == ")" { continue; }
            // join a literal that continues on the next lines: `const car = {` ... `};`
            let opens_literal = |s: &str| (s.ends_with('{') && (s.contains('=') && !s.contains("=>") && !s.trim_end_matches('{').trim().ends_with(')') || s.starts_with("return {") || s.starts_with("console.log(")))
                || s.ends_with('[') || s.ends_with('(') || s.ends_with(',');
            if opens_literal(&t) {
                let mut joined = t.clone();
                while depth_of(&joined) > 0 && k < raw.len() {
                    let nxt = raw[k].trim();
                    k += 1;
                    if nxt.starts_with("//") { continue; }
                    joined.push(' ');
                    joined.push_str(nxt);
                }
                let j = joined.trim_end_matches(';').trim().to_string();
                if depth_of(&j) == 0 { out.push(Line { ind: depth, text: j, no }); continue; }
                t = j;
            }
            // one-line bodies: `if (x) { a; }` or `function f(n) { return n * 2; }`
            if let Some(open) = t.find('{') {
                let head = t[..open].trim();
                let is_block = head.ends_with(')') || head.ends_with("=>") || head == "else" || head.starts_with("else") || head.starts_with("function") || head.starts_with("for") || head.starts_with("while") || head.starts_with("if");
                if t.trim_end().ends_with('}') && is_block {
                    let body = t[open + 1..t.rfind('}').unwrap_or(t.len())].trim().to_string();
                    out.push(Line { ind: depth, text: head.to_string(), no });
                    for stmt in split_stmts(&body) { out.push(Line { ind: depth + 1, text: stmt, no }); }
                    continue;
                }
            }
            if t.ends_with('{') {
                t = t[..t.len() - 1].trim().to_string();
                out.push(Line { ind: depth, text: t, no });
                depth += 1;
                continue;
            }
            // brace-less one-liners: `if (x > 4) break;` / `else console.log(1);`
            if let Some((head, body)) = braceless(&t) {
                out.push(Line { ind: depth, text: head, no });
                for stmt in split_stmts(&body) { out.push(Line { ind: depth + 1, text: stmt, no }); }
                continue;
            }
            for stmt in split_stmts(&t) { out.push(Line { ind: depth, text: stmt, no }); }
        } else {
            let ind = line.len() - line.trim_start().len();
            let mut joined = t.clone();
            while depth_of(&joined) > 0 && k < raw.len() { joined.push(' '); joined.push_str(raw[k].trim()); k += 1; }
            out.push(Line { ind: ind / 2, text: joined, no });
        }
    }
    out
}

/// Remove a trailing `// ...` (JS) or `# ...` (Python) comment that is outside any string.
fn strip_comment(t: &str, js: bool) -> String {
    let c: Vec<char> = t.chars().collect();
    let mut q: Option<char> = None;
    let mut i = 0;
    while i < c.len() {
        if let Some(qc) = q {
            if c[i] == '\\' { i += 2; continue; }
            if c[i] == qc { q = None; }
            i += 1;
            continue;
        }
        match c[i] {
            '"' | '\'' | '`' => q = Some(c[i]),
            '/' if js && i + 1 < c.len() && c[i + 1] == '/' => return c[..i].iter().collect::<String>().trim().to_string(),
            '#' if !js => return c[..i].iter().collect::<String>().trim().to_string(),
            _ => {}
        }
        i += 1;
    }
    t.trim().to_string()
}

fn braceless(t: &str) -> Option<(String, String)> {
    if let Some(rest) = t.strip_prefix("else ") && !rest.starts_with("if") && !rest.trim().is_empty() { return Some(("else".into(), rest.trim().into())); }
    let kw = ["else if", "if", "while", "for"].into_iter().find(|k| t.starts_with(k) && t[k.len()..].trim_start().starts_with('('))?;
    let start = t.find('(')?;
    let mut d = 0;
    for (i, ch) in t.char_indices().skip(start) {
        if ch == '(' { d += 1; } else if ch == ')' { d -= 1; if d == 0 {
            let body = t[i + 1..].trim();
            if body.is_empty() || body.starts_with('{') { return None; }
            let _ = kw;
            return Some((t[..=i].trim().to_string(), body.to_string()));
        } }
    }
    None
}

fn split_stmts(s: &str) -> Vec<String> {
    let mut out = vec![];
    let mut d = 0;
    let mut q: Option<char> = None;
    let mut cur = String::new();
    for ch in s.chars() {
        if let Some(qc) = q { cur.push(ch); if ch == qc { q = None; } continue; }
        match ch {
            '"' | '\'' | '`' => { q = Some(ch); cur.push(ch); }
            '(' | '[' | '{' => { d += 1; cur.push(ch); }
            ')' | ']' | '}' => { d -= 1; cur.push(ch); }
            ';' if d == 0 => { if !cur.trim().is_empty() { out.push(cur.trim().to_string()); } cur.clear(); }
            _ => cur.push(ch),
        }
    }
    if !cur.trim().is_empty() { out.push(cur.trim().to_string()); }
    out
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Flow { Next, Return, Break, Continue }

#[derive(Clone)]
struct Func { params: Vec<String>, from: usize, to: usize, expr: Option<String> }

struct Run<'l> {
    lines: &'l [Line], vars: HashMap<String, V>, funcs: HashMap<String, Func>, out: Vec<(Kind, String)>,
    steps: u32, js: bool, depth: u32, ret: Option<V>, seed: u64, dom: HashMap<String, Vec<(String, V)>>,
}

/// Expression evaluator. Holds the runner so it can call user functions.
struct Ev<'a, 'l> { t: Vec<T>, i: usize, r: &'a mut Run<'l> }

impl<'a, 'l> Ev<'a, 'l> {
    fn peek(&self) -> Option<&T> { self.t.get(self.i) }
    fn op(&mut self, o: &str) -> bool {
        if matches!(self.peek(), Some(T::Op(x)) if x == o) { self.i += 1; true } else { false }
    }
    fn kw(&mut self, k: &str) -> bool {
        if matches!(self.peek(), Some(T::Id(x)) if x == k) { self.i += 1; true } else { false }
    }
    fn expr(&mut self) -> Result<V, String> {
        let c = self.or()?;
        if self.op("?") {
            let a = self.expr()?;
            if !self.op(":") { return Err("a ? b : c needs the ':' part".into()); }
            let b = self.expr()?;
            return Ok(if truthy(&c) { a } else { b });
        }
        Ok(c)
    }
    fn or(&mut self) -> Result<V, String> {
        let mut l = self.and()?;
        loop {
            if self.op("||") || self.kw("or") { let r = self.and()?; l = if self.r.js { if truthy(&l) { l } else { r } } else { V::B(truthy(&l) || truthy(&r)) }; } else { return Ok(l); }
        }
    }
    fn and(&mut self) -> Result<V, String> {
        let mut l = self.not()?;
        loop {
            if self.op("&&") || self.kw("and") { let r = self.not()?; l = V::B(truthy(&l) && truthy(&r)); } else { return Ok(l); }
        }
    }
    fn not(&mut self) -> Result<V, String> {
        if self.kw("not") { let v = self.not()?; return Ok(V::B(!truthy(&v))); }
        self.cmp()
    }
    fn cmp(&mut self) -> Result<V, String> {
        let l = self.add()?;
        if self.kw("in") {
            let r = self.add()?;
            return Ok(V::B(match &r { V::L(xs) => xs.contains(&l), V::S(s) => s.contains(&self.r.show(&l)), V::O(o) => field(o, &self.r.show(&l)).is_some(), _ => false }));
        }
        for o in ["===", "!==", "==", "!=", "<=", ">=", "<", ">"] {
            if self.op(o) {
                let r = self.add()?;
                let ord = match (&l, &r) {
                    (V::N(a), V::N(b)) => a.partial_cmp(b),
                    (V::S(a), V::S(b)) => a.partial_cmp(b),
                    (V::N(a), V::S(b)) | (V::S(b), V::N(a)) if self.r.js && (o == "<" || o == ">" || o == "<=" || o == ">=") => b.trim().parse::<f64>().ok().and_then(|bn| if matches!(l, V::N(_)) { a.partial_cmp(&bn) } else { bn.partial_cmp(a) }),
                    _ => None,
                };
                let loose = o == "==" || o == "!=";
                let eq = l == r || (self.r.js && loose && match (&l, &r) { (V::N(a), V::S(b)) | (V::S(b), V::N(a)) => b.trim().parse::<f64>().map(|x| x == *a).unwrap_or(false), _ => false });
                use std::cmp::Ordering::*;
                return Ok(V::B(match o {
                    "==" | "===" => eq,
                    "!=" | "!==" => !eq,
                    "<" => ord == Some(Less),
                    ">" => ord == Some(Greater),
                    "<=" => matches!(ord, Some(Less) | Some(Equal)),
                    _ => matches!(ord, Some(Greater) | Some(Equal)),
                }));
            }
        }
        Ok(l)
    }
    fn add(&mut self) -> Result<V, String> {
        let mut l = self.mul()?;
        loop {
            if self.op("+") {
                let r = self.mul()?;
                l = match (&l, &r) {
                    (V::N(a), V::N(b)) => V::N(a + b),
                    (V::L(a), V::L(b)) => V::L(a.iter().chain(b.iter()).cloned().collect()),
                    (V::S(_), _) | (_, V::S(_)) => {
                        if !self.r.js && !(matches!(l, V::S(_)) && matches!(r, V::S(_))) { return Err("can only join text with text (use str() to convert a number)".into()); }
                        V::S(format!("{}{}", self.r.show(&l), self.r.show(&r)))
                    }
                    _ => V::N(num(&l)? + num(&r)?),
                };
            } else if self.op("-") {
                let r = self.mul()?;
                l = V::N(num(&l)? - num(&r)?);
            } else { return Ok(l); }
        }
    }
    fn mul(&mut self) -> Result<V, String> {
        let mut l = self.un()?;
        loop {
            if self.op("**") { let r = self.un()?; l = V::N(num(&l)?.powf(num(&r)?)); }
            else if self.op("*") {
                let r = self.un()?;
                l = match (&l, &r) {
                    (V::S(s), V::N(n)) | (V::N(n), V::S(s)) if !self.r.js => V::S(s.repeat((n.max(0.0) as usize).min(MAX_LIST))),
                    _ => V::N(num(&l)? * num(&r)?),
                };
            } else if self.op("//") { let r = self.un()?; let d = num(&r)?; if d == 0.0 { return Err("division by zero".into()); } l = V::N((num(&l)? / d).floor()); }
            else if self.op("/") { let r = self.un()?; let d = num(&r)?; if d == 0.0 { if self.r.js { l = V::N(f64::INFINITY); continue; } return Err("division by zero".into()); } l = V::N(num(&l)? / d); }
            else if self.op("%") { let r = self.un()?; let d = num(&r)?; if d == 0.0 { return Err("division by zero".into()); } let a = num(&l)?; l = V::N(if self.r.js { a % d } else { a - d * (a / d).floor() }); }
            else { return Ok(l); }
        }
    }
    fn un(&mut self) -> Result<V, String> {
        if self.op("-") { let v = self.un()?; return Ok(V::N(-num(&v)?)); }
        if self.op("+") { let v = self.un()?; return Ok(V::N(num(&v)?)); }
        if self.op("!") { let v = self.un()?; return Ok(V::B(!truthy(&v))); }
        if self.kw("typeof") { let v = self.un()?; return Ok(V::S(self.r.type_name(&v))); }
        let mut v = self.prim()?;
        loop {
            if self.op("[") {
                let idx = self.expr()?;
                if !self.op("]") { return Err("missing ]".into()); }
                v = index(&v, &idx)?;
            } else if self.op(".") {
                let Some(T::Id(name)) = self.t.get(self.i).cloned() else { return Err("expected a name after '.'".into()) };
                self.i += 1;
                let mut cur = v;
                let parts: Vec<&str> = name.split('.').collect();
                for (k, p) in parts.iter().enumerate() {
                    if k + 1 == parts.len() && self.op("(") { let a = self.args()?; cur = self.r.method(cur, p, a, None)?; }
                    else { cur = self.r.member(&cur, p)?; }
                }
                v = cur;
            } else { break; }
        }
        Ok(v)
    }
    fn args(&mut self) -> Result<Vec<V>, String> {
        let mut a = vec![];
        if self.op(")") { return Ok(a); }
        loop {
            a.push(self.expr()?);
            if self.op(",") { continue; }
            if self.op(")") { return Ok(a); }
            return Err("expected ) or ,".into());
        }
    }
    fn template(&mut self, raw: &str, py: bool) -> Result<V, String> {
        let mut out = String::new();
        let c: Vec<char> = raw.chars().collect();
        let mut i = 0;
        while i < c.len() {
            let start = if py { c[i] == '{' && !(i + 1 < c.len() && c[i + 1] == '{') } else { c[i] == '$' && i + 1 < c.len() && c[i + 1] == '{' };
            if start {
                i += if py { 1 } else { 2 };
                let mut d = 1;
                let st = i;
                while i < c.len() { if c[i] == '{' { d += 1; } else if c[i] == '}' { d -= 1; if d == 0 { break; } } i += 1; }
                if i >= c.len() { return Err("a { in the text is never closed".into()); }
                let inner: String = c[st..i].iter().collect();
                let v = self.r.eval(&inner)?;
                out.push_str(&self.r.show(&v));
                i += 1;
            } else if py && c[i] == '{' { out.push('{'); i += 2; } else if py && c[i] == '}' && i + 1 < c.len() && c[i + 1] == '}' { out.push('}'); i += 2; } else { out.push(c[i]); i += 1; }
        }
        Ok(V::S(out))
    }
    fn prim(&mut self) -> Result<V, String> {
        let t = self.t.get(self.i).cloned().ok_or("unfinished expression")?;
        self.i += 1;
        match t {
            T::Num(n) => Ok(V::N(n)),
            T::Str(s) => Ok(V::S(s)),
            T::Tpl(s, py) => self.template(&s, py),
            T::Op(o) if o == "(" => { let v = self.expr()?; if !self.op(")") { return Err("missing )".into()); } Ok(v) }
            T::Op(o) if o == "[" => {
                let mut items = vec![];
                if self.op("]") { return Ok(V::L(items)); }
                loop {
                    items.push(self.expr()?);
                    if self.op(",") { if self.op("]") { break; } continue; }
                    if self.op("]") { break; }
                    return Err("expected ] or ,".into());
                }
                Ok(V::L(items))
            }
            T::Op(o) if o == "{" => {
                let mut items: Vec<(String, V)> = vec![];
                if self.op("}") { return Ok(V::O(items)); }
                loop {
                    let k = match self.t.get(self.i).cloned() { Some(T::Id(k)) | Some(T::Str(k)) => { self.i += 1; k } Some(T::Num(n)) => { self.i += 1; num_str(n) } _ => return Err("an object key must be a name".into()) };
                    if !self.op(":") { return Err(format!("expected ':' after the key '{}'", k)); }
                    let v = self.expr()?;
                    if let Some(e) = items.iter_mut().find(|(n, _)| *n == k) { e.1 = v; } else { items.push((k, v)); }
                    if self.op(",") { if self.op("}") { break; } continue; }
                    if self.op("}") { break; }
                    return Err("expected } or , in the object".into());
                }
                Ok(V::O(items))
            }
            T::Id(id) => {
                if self.op("(") {
                    let a = self.args()?;
                    return self.r.call(&id, a);
                }
                match id.as_str() {
                    "True" | "true" => return Ok(V::B(true)),
                    "False" | "false" => return Ok(V::B(false)),
                    "None" | "null" | "undefined" => return Ok(V::Nil),
                    "Math.PI" => return Ok(V::N(std::f64::consts::PI)),
                    "document" => return Ok(V::O(vec![("__document".into(), V::B(true))])),
                    _ => {}
                }
                self.r.get_path(&id)
            }
            T::Op(o) => Err(format!("unexpected '{}'", o)),
        }
    }
}

fn index(v: &V, idx: &V) -> Result<V, String> {
    match (v, idx) {
        (V::O(o), k) => { let k = match k { V::S(s) => s.clone(), V::N(n) => num_str(*n), _ => return Err("bad key".into()) }; Ok(field(o, &k).cloned().unwrap_or(V::Nil)) }
        (V::L(l), _) => { let k = num(idx)?; let i = if k < 0.0 { l.len() as f64 + k } else { k }; l.get(i as usize).cloned().filter(|_| i >= 0.0 && i.fract() == 0.0).ok_or_else(|| "index out of range".to_string()) }
        (V::S(s), _) => { let k = num(idx)?; let ch: Vec<char> = s.chars().collect(); let i = if k < 0.0 { ch.len() as f64 + k } else { k }; ch.get(i as usize).filter(|_| i >= 0.0).map(|c| V::S(c.to_string())).ok_or_else(|| "index out of range".to_string()) }
        _ => Err("only lists, text and objects can be indexed".into()),
    }
}

impl<'l> Run<'l> {
    fn show(&self, v: &V) -> String { self.show_d(v, 0) }
    fn show_d(&self, v: &V, d: usize) -> String {
        let q = |x: &V| match x { V::S(s) if d < 8 => format!("'{}'", s), o => self.show_d(o, d + 1) };
        match v {
            V::N(n) => if n.is_infinite() { if *n > 0.0 { "Infinity".into() } else { "-Infinity".into() } } else { num_str(*n) },
            V::S(s) => s.clone(),
            V::B(b) => if self.js { b.to_string() } else if *b { "True".into() } else { "False".into() },
            V::Nil => if self.js { "undefined".into() } else { "None".into() },
            V::L(l) => {
                let items: Vec<String> = l.iter().map(q).collect();
                if self.js { if items.is_empty() { "[]".into() } else { format!("[ {} ]", items.join(", ")) } } else { format!("[{}]", items.join(", ")) }
            }
            V::O(o) => {
                if let Some((sel, _)) = el_of(v) { return format!("<element {}>", sel); }
                let items: Vec<String> = o.iter().map(|(k, x)| if self.js { format!("{}: {}", k, q(x)) } else { format!("'{}': {}", k, q(x)) }).collect();
                if items.is_empty() { "{}".into() } else if self.js { format!("{{ {} }}", items.join(", ")) } else { format!("{{{}}}", items.join(", ")) }
            }
        }
    }
    fn type_name(&self, v: &V) -> String {
        let js = self.js;
        match v { V::N(n) => if js { "number" } else if n.fract() == 0.0 { "int" } else { "float" }, V::S(_) => if js { "string" } else { "str" }, V::B(_) => if js { "boolean" } else { "bool" },
            V::L(_) => if js { "object" } else { "list" }, V::O(_) => if js { "object" } else { "dict" }, V::Nil => if js { "undefined" } else { "NoneType" } }.into()
    }
    fn random(&mut self) -> f64 {
        self.seed ^= self.seed << 13; self.seed ^= self.seed >> 7; self.seed ^= self.seed << 17;
        (self.seed >> 11) as f64 / (1u64 << 53) as f64
    }
    fn page(&mut self, t: String) -> Result<(), String> {
        if self.out.len() >= MAX_OUT { return Err("stopped: too much output".into()); }
        self.out.push((Kind::Page, t)); Ok(())
    }

    fn eval(&mut self, s: &str) -> Result<V, String> {
        let t = lex(s)?;
        let mut e = Ev { t, i: 0, r: self };
        let v = e.expr()?;
        if e.i < e.t.len() { return Err(format!("extra text after expression: {}", s.trim())); }
        Ok(v)
    }

    fn step(&mut self) -> Result<(), String> {
        self.steps += 1;
        if self.steps > MAX_STEPS { Err("stopped: too many steps (is there an endless loop?)".into()) } else { Ok(()) }
    }

    /// Read `a.b.c` (variables, object fields, `.length`, element properties).
    fn get_path(&mut self, path: &str) -> Result<V, String> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut cur = match self.vars.get(parts[0]) { Some(v) => v.clone(), None => {
            if self.funcs.contains_key(parts[0]) { return Ok(V::S(format!("[function {}]", parts[0]))); }
            return Err(format!("'{}' is not defined", parts[0]));
        } };
        for p in &parts[1..] { cur = self.member(&cur, p)?; }
        Ok(cur)
    }
    fn member(&mut self, v: &V, p: &str) -> Result<V, String> {
        if let Some((sel, is_style)) = el_of(v) {
            if !is_style && p == "style" { return Ok(V::O(vec![(STYLE.into(), V::S(sel))])); }
            let key = if is_style { format!("style.{}", p) } else { p.to_string() };
            return Ok(self.dom.get(&sel).and_then(|o| field(o, &key).cloned()).unwrap_or(V::S(String::new())));
        }
        match (v, p) {
            (V::L(l), "length") => Ok(V::N(l.len() as f64)),
            (V::S(s), "length") => Ok(V::N(s.chars().count() as f64)),
            (V::O(o), k) => Ok(field(o, k).cloned().unwrap_or(V::Nil)),
            _ => Err(format!("'{}' has no property '{}'", self.show(v), p)),
        }
    }
    /// Write `a.b.c = v` into a variable (objects nest).
    fn set_path(&mut self, path: &str, val: V) -> Result<(), String> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.len() == 1 { self.vars.insert(path.into(), val); return Ok(()); }
        // element targets: el.textContent / el.style.color
        let base = parts[..parts.len() - 1].join(".");
        let base_v = self.get_path(&base)?;
        if let Some((sel, is_style)) = el_of(&base_v) { return self.set_dom(&sel, is_style, parts[parts.len() - 1], val); }
        let root = self.vars.get_mut(parts[0]).ok_or(format!("'{}' is not defined", parts[0]))?;
        let mut cur = root;
        for (k, p) in parts[1..].iter().enumerate() {
            let last = k + 2 == parts.len();
            match cur {
                V::O(o) => {
                    if last { if let Some(e) = o.iter_mut().find(|(n, _)| n == p) { e.1 = val; } else { o.push((p.to_string(), val)); } return Ok(()); }
                    if !o.iter().any(|(n, _)| n == p) { return Err(format!("'{}' has no property '{}'", parts[..=k].join("."), p)); }
                    cur = &mut o.iter_mut().find(|(n, _)| n == p).unwrap().1;
                }
                _ => return Err(format!("can't set '{}' - '{}' is not an object", p, parts[..=k].join("."))),
            }
        }
        Ok(())
    }
    fn set_dom(&mut self, sel: &str, is_style: bool, prop: &str, val: V) -> Result<(), String> {
        let shown = self.show(&val);
        let key = if is_style { format!("style.{}", prop) } else { prop.to_string() };
        let e = self.dom.entry(sel.to_string()).or_default();
        if let Some(x) = e.iter_mut().find(|(n, _)| *n == key) { x.1 = val; } else { e.push((key, val)); }
        let msg = match (is_style, prop) {
            (true, p) => format!("{} style {} = {}", sel, kebab(p), shown),
            (false, "textContent" | "innerText") => format!("{} now shows: {}", sel, shown),
            (false, "innerHTML") => format!("{} HTML set to: {}", sel, shown),
            (false, "value") => format!("{} input value = {}", sel, shown),
            (false, "src") => format!("{} picture changed to {}", sel, shown),
            (false, p) => format!("{} {} = {}", sel, p, shown),
        };
        self.page(msg)
    }

    fn call(&mut self, f: &str, a: Vec<V>) -> Result<V, String> {
        if let Some(func) = self.funcs.get(f).cloned() { return self.call_user(f, &func, a); }
        // callable stored in a variable that names a function: const f = greet
        let alias = match self.vars.get(f) { Some(V::S(s)) => s.strip_prefix("[function ").and_then(|x| x.strip_suffix(']')).map(|x| x.to_string()), _ => None };
        if let Some(name) = alias && let Some(func) = self.funcs.get(&name).cloned() { return self.call_user(&name, &func, a); }
        let first = a.first().cloned();
        let arg0 = || first.clone().ok_or_else(|| format!("{}() needs a value", f));
        let n = |i: usize| -> Result<f64, String> { num(a.get(i).ok_or("missing argument")?) };
        Ok(match f {
            "len" => match arg0()? { V::L(l) => V::N(l.len() as f64), V::S(s) => V::N(s.chars().count() as f64), V::O(o) => V::N(o.len() as f64), _ => return Err("len() needs text or a list".into()) },
            "str" | "String" => V::S(self.show(&arg0()?)),
            "int" | "parseInt" | "Math.trunc" => V::N(n(0)?.trunc()),
            "Math.floor" => V::N(n(0)?.floor()),
            "float" | "Number" | "parseFloat" => V::N(n(0)?),
            "abs" | "Math.abs" => V::N(n(0)?.abs()),
            "round" | "Math.round" => V::N(n(0)?.round()),
            "Math.sqrt" => V::N(n(0)?.sqrt()),
            "Math.ceil" => V::N(n(0)?.ceil()),
            "Math.pow" | "pow" => V::N(n(0)?.powf(n(1)?)),
            "Math.random" => V::N(self.random()),
            "max" | "Math.max" | "min" | "Math.min" => {
                let vals: Vec<f64> = match a.as_slice() { [V::L(l)] => l.iter().map(num).collect::<Result<_, _>>()?, _ => a.iter().map(num).collect::<Result<_, _>>()? };
                if vals.is_empty() { return Err(format!("{}() needs values", f)); }
                V::N(if f.ends_with("max") { vals.iter().cloned().fold(f64::MIN, f64::max) } else { vals.iter().cloned().fold(f64::MAX, f64::min) })
            }
            "sum" => match arg0()? { V::L(l) => V::N(l.iter().map(num).sum::<Result<f64, String>>()?), _ => return Err("sum() needs a list".into()) },
            "type" => V::S(self.type_name(&arg0()?)),
            "list" => match arg0()? { V::S(s) => V::L(s.chars().map(|c| V::S(c.to_string())).collect()), V::L(l) => V::L(l), _ => return Err("list() needs text or a list".into()) },
            "range" => { let (s, e) = if a.len() >= 2 { (n(0)?, n(1)?) } else { (0.0, n(0)?) }; V::L((0..((e - s).max(0.0) as usize).min(MAX_LIST)).map(|k| V::N(s + k as f64)).collect()) }
            "alert" => { let t = self.show(&arg0()?); self.page(format!("[alert] {}", t))?; V::Nil }
            "input" | "prompt" => V::S(String::new()),
            "Object.keys" => match arg0()? { V::O(o) => V::L(o.iter().filter(|(k, _)| !k.starts_with("__")).map(|(k, _)| V::S(k.clone())).collect()), _ => return Err("Object.keys() needs an object".into()) },
            "Object.values" => match arg0()? { V::O(o) => V::L(o.iter().filter(|(k, _)| !k.starts_with("__")).map(|(_, v)| v.clone()).collect()), _ => return Err("Object.values() needs an object".into()) },
            "Array.isArray" => V::B(matches!(arg0()?, V::L(_))),
            "document.getElementById" => { let id = self.show(&arg0()?); V::O(vec![(EL.into(), V::S(format!("#{}", id)))]) }
            "document.querySelector" => V::O(vec![(EL.into(), V::S(self.show(&arg0()?)))]),
            "document.write" => { let t = self.show(&arg0()?); self.page(format!("page: {}", t))?; V::Nil }
            _ => {
                if let Some(dot) = f.rfind('.') {
                    let (base, m) = (&f[..dot], &f[dot + 1..]);
                    let bv = self.get_path(base)?;
                    return self.method(bv, m, a, Some(base));
                }
                return Err(format!("'{}' is not defined", f));
            }
        })
    }

    fn call_user(&mut self, name: &str, func: &Func, a: Vec<V>) -> Result<V, String> {
        if a.len() > func.params.len() || (a.len() < func.params.len() && !self.js) { return Err(format!("{}() needs {} value(s), got {}", name, func.params.len(), a.len())); }
        let saved = self.vars.clone();
        for (k, p) in func.params.iter().enumerate() { self.vars.insert(p.clone(), a.get(k).cloned().unwrap_or(V::Nil)); }
        let r = if let Some(e) = &func.expr { self.eval(e).map(|v| { self.ret = Some(v); Flow::Return }) } else { self.exec(func.from, func.to) };
        let ret = self.ret.take().unwrap_or(V::Nil);
        // locals vanish after the call; changes to globals inside functions are kept for objects/lists
        let mut restored = saved;
        for (k, v) in self.vars.drain() { if restored.contains_key(&k) && !func.params.contains(&k) { restored.insert(k, v); } }
        self.vars = restored;
        r?;
        Ok(ret)
    }

    /// Methods on values. `path` is the variable path when the receiver can be mutated.
    fn method(&mut self, v: V, m: &str, a: Vec<V>, path: Option<&str>) -> Result<V, String> {
        let arg = |i: usize| a.get(i).cloned().ok_or_else(|| format!(".{}() needs a value", m));
        if let Some((sel, _)) = el_of(&v) {
            return match m {
                "addEventListener" => {
                    let evt = self.show(&arg(0)?);
                    let handler = a.get(1).map(|h| self.show(h)).unwrap_or_default();
                    self.page(format!("{} listens for '{}'", sel, evt))?;
                    if let Some(name) = handler.strip_prefix("[function ").and_then(|x| x.strip_suffix(']'))
                        && let Some(func) = self.funcs.get(name).cloned() { self.page(format!("(simulating a {} on {})", evt, sel))?; self.call_user(name, &func, vec![])?; }
                    Ok(V::Nil)
                }
                "classList.add" | "add" => { let c = self.show(&arg(0)?); self.page(format!("{} gets class '{}'", sel, c))?; Ok(V::Nil) }
                "remove" => { self.page(format!("{} removed from the page", sel))?; Ok(V::Nil) }
                "click" => { self.page(format!("{} clicked", sel))?; Ok(V::Nil) }
                _ => Err(format!("elements here don't support .{}()", m)),
            };
        }
        if let V::O(o) = &v && field(o, "__document").is_some() { return self.call(&format!("document.{}", m), a); }
        match (&v, m) {
            (V::S(s), "toUpperCase" | "upper") => Ok(V::S(s.to_uppercase())),
            (V::S(s), "toLowerCase" | "lower") => Ok(V::S(s.to_lowercase())),
            (V::S(s), "trim" | "strip") => Ok(V::S(s.trim().into())),
            (V::S(s), "includes") => Ok(V::B(s.contains(&self.show(&arg(0)?)))),
            (V::S(s), "startsWith" | "startswith") => Ok(V::B(s.starts_with(&self.show(&arg(0)?)))),
            (V::S(s), "endsWith" | "endswith") => Ok(V::B(s.ends_with(&self.show(&arg(0)?)))),
            (V::S(s), "indexOf" | "find") => { let t = self.show(&arg(0)?); Ok(V::N(s.find(&t).map(|i| s[..i].chars().count() as f64).unwrap_or(-1.0))) }
            (V::S(s), "replace") => { let (x, y) = (self.show(&arg(0)?), self.show(&arg(1)?)); Ok(V::S(if self.js { s.replacen(&x, &y, 1) } else { s.replace(&x, &y) })) }
            (V::S(s), "repeat") => Ok(V::S(s.repeat((num(&arg(0)?)?.max(0.0) as usize).min(MAX_LIST)))),
            (V::S(s), "split") => { let sep = a.first().map(|x| self.show(x)); Ok(V::L(match sep.as_deref() { Some("") => s.chars().map(|c| V::S(c.to_string())).collect(), Some(p) => s.split(p).map(|x| V::S(x.into())).collect(), None => s.split_whitespace().map(|x| V::S(x.into())).collect() })) }
            (V::S(s), "charAt") => index(&v, &arg(0)?).or(Ok(V::S(String::new()))).map(|x| if s.is_empty() { V::S(String::new()) } else { x }),
            (V::S(s), "slice" | "substring") => { let ch: Vec<char> = s.chars().collect(); let st = (num(&arg(0)?)? as isize).clamp(0, ch.len() as isize) as usize; let en = a.get(1).map(num).transpose()?.map(|e| (e as isize).clamp(0, ch.len() as isize) as usize).unwrap_or(ch.len()); Ok(V::S(if st < en { ch[st..en].iter().collect() } else { String::new() })) }
            (V::N(n), "toFixed") => { let d = a.first().map(num).transpose()?.unwrap_or(0.0) as usize; Ok(V::S(format!("{:.*}", d.min(10), n))) }
            (V::N(n), "toString") => Ok(V::S(num_str(*n))),
            (V::L(l), "includes") => Ok(V::B(l.contains(&arg(0)?))),
            (V::L(l), "indexOf" | "index") => { let x = arg(0)?; Ok(V::N(l.iter().position(|y| *y == x).map(|i| i as f64).unwrap_or(-1.0))) }
            (V::L(l), "join") => { let sep = a.first().map(|x| self.show(x)).unwrap_or_else(|| ",".into()); Ok(V::S(l.iter().map(|x| self.show(x)).collect::<Vec<_>>().join(&sep))) }
            (V::L(l), "slice") => { let st = (num(&arg(0)?)? as usize).min(l.len()); let en = a.get(1).map(num).transpose()?.map(|e| (e as usize).min(l.len())).unwrap_or(l.len()); Ok(V::L(if st < en { l[st..en].to_vec() } else { vec![] })) }
            (V::L(_), "push" | "append" | "pop" | "shift" | "unshift" | "reverse" | "sort") => {
                let p = path.ok_or_else(|| format!(".{}() needs a list stored in a variable", m))?.to_string();
                let V::L(mut l) = v else { unreachable!() };
                let ret = match m {
                    "push" | "append" => { if l.len() >= MAX_LIST { return Err("list is too long".into()); } l.extend(a.iter().cloned()); if self.js { V::N(l.len() as f64) } else { V::Nil } }
                    "unshift" => { for x in a.iter().rev() { l.insert(0, x.clone()); } V::N(l.len() as f64) }
                    "pop" => l.pop().unwrap_or(V::Nil),
                    "shift" => if l.is_empty() { V::Nil } else { l.remove(0) },
                    "reverse" => { l.reverse(); V::Nil }
                    _ => { l.sort_by(|x, y| match (x, y) { (V::N(a), V::N(b)) => a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal), _ => self.show(x).cmp(&self.show(y)) }); V::Nil }
                };
                let out = if m == "reverse" || m == "sort" { V::L(l.clone()) } else { ret };
                self.set_path(&p, V::L(l))?;
                Ok(if self.js || matches!(m, "pop") { out } else { V::Nil })
            }
            (V::O(o), "keys") => Ok(V::L(o.iter().map(|(k, _)| V::S(k.clone())).collect())),
            (V::O(o), "values") => Ok(V::L(o.iter().map(|(_, v)| v.clone()).collect())),
            (V::O(o), "get") => { let k = self.show(&arg(0)?); Ok(field(o, &k).cloned().unwrap_or_else(|| a.get(1).cloned().unwrap_or(V::Nil))) }
            (V::O(o), name) => {
                // an object property holding a function name: obj.greet()
                match field(o, name) { Some(V::S(s)) if s.starts_with("[function ") => { let fname = s[10..s.len() - 1].to_string(); self.call(&fname, a) } _ => Err(format!("'{}' has no method '{}'", self.show(&v), name)) }
            }
            _ => Err(format!("{} has no method '{}'", self.type_name(&v), m)),
        }
    }

    fn block_end(&self, i: usize, to: usize) -> usize {
        let base = self.lines[i].ind;
        let mut j = i + 1;
        while j < to && self.lines[j].ind > base { j += 1; }
        j
    }

    fn exec(&mut self, from: usize, to: usize) -> Result<Flow, String> {
        self.depth += 1;
        if self.depth > MAX_DEPTH { self.depth -= 1; return Err("stopped: blocks are nested too deeply".into()); }
        let r = self.exec_inner(from, to);
        self.depth -= 1;
        r
    }

    fn exec_inner(&mut self, from: usize, to: usize) -> Result<Flow, String> {
        let mut i = from;
        while i < to {
            self.step()?;
            let no = self.lines[i].no;
            let e = |m: String| if m.starts_with("line ") || m.starts_with("stopped") { m } else { format!("line {}: {}", no, m) };
            let t = self.lines[i].text.clone();
            let t = t.as_str();
            let end = self.block_end(i, to);
            let hdr = |t: &str, kw: &str| -> Option<String> {
                let r = t.strip_prefix(kw)?;
                if !(r.starts_with(' ') || r.starts_with('(')) { return None; }
                let r = r.trim().trim_end_matches(':').trim();
                let r = if r.starts_with('(') && r.ends_with(')') && balanced(&r[1..r.len() - 1]) { &r[1..r.len() - 1] } else { r };
                Some(r.to_string())
            };
            if t == "break" { return Ok(Flow::Break); }
            if t == "continue" { return Ok(Flow::Continue); }
            if t == "pass" { i += 1; continue; }
            if let Some(args) = t.strip_prefix("print(").or_else(|| t.strip_prefix("console.log(")).filter(|a| depth_of(a) == -1 && a.ends_with(')')) {
                let inner = &args[..args.len() - 1];
                let mut parts = vec![];
                for a in split_args(inner) { let v = self.eval(&a).map_err(e)?; parts.push(self.show(&v)); }
                if self.out.len() >= MAX_OUT { return Err("stopped: too much output".into()); }
                self.out.push((Kind::Out, parts.join(" ")));
                i += 1;
            } else if let Some((name, params, expr)) = func_header(t) {
                if expr.is_none() && end == i + 1 { return Err(e("this function has no body".into())); }
                self.funcs.insert(name.clone(), Func { params, from: i + 1, to: end, expr });
                self.vars.remove(&name);
                i = end;
            } else if let Some((target, evt, params)) = listener_header(t) {
                // el.addEventListener("click", () => { ... })  -> register and simulate one event
                let el = self.eval(&target).map_err(e)?;
                let Some((sel, _)) = el_of(&el) else { return Err(e(format!("'{}' is not a page element", target))) };
                self.page(format!("{} listens for '{}'", sel, evt))?;
                self.page(format!("(simulating a {} on {})", evt, sel))?;
                for p in params { self.vars.insert(p, V::O(vec![("type".into(), V::S(evt.clone()))])); }
                if self.exec(i + 1, end)? == Flow::Return { self.ret = None; }
                i = end;
            } else if t == "return" || t.starts_with("return ") {
                let expr = t["return".len()..].trim();
                self.ret = Some(if expr.is_empty() { V::Nil } else { self.eval(expr).map_err(e)? });
                return Ok(Flow::Return);
            } else if let Some(c) = hdr(t, "if") {
                let ind = self.lines[i].ind;
                let mut branches = vec![(Some(c), i + 1, end)];
                let mut j = end;
                while j < to && self.lines[j].ind == ind {
                    let lt = self.lines[j].text.clone();
                    let be = self.block_end(j, to);
                    if let Some(c2) = hdr(&lt, "elif").or_else(|| hdr(&lt, "else if")) { branches.push((Some(c2), j + 1, be)); j = be; }
                    else if lt == "else" || lt == "else:" { branches.push((None, j + 1, be)); j = be; break; }
                    else { break; }
                }
                for (cond, a, b) in branches {
                    let go = match cond { Some(c) => { let v = self.eval(&c).map_err(e)?; truthy(&v) } None => true };
                    if go { let f = self.exec(a, b)?; if f != Flow::Next { return Ok(f); } break; }
                }
                i = j;
            } else if let Some(c) = hdr(t, "while") {
                loop {
                    let v = self.eval(&c).map_err(e)?;
                    if !truthy(&v) { break; }
                    match self.exec(i + 1, end)? { Flow::Return => return Ok(Flow::Return), Flow::Break => break, _ => {} }
                    self.step()?;
                }
                i = end;
            } else if let Some(c) = hdr(t, "for") {
                let c = c.trim_start_matches("let ").trim_start_matches("const ").trim_start_matches("var ").to_string();
                let in_of = c.split_once(" in ").or_else(|| c.split_once(" of "));
                if let (Some((v, rest)), false) = (in_of, c.contains(';')) {
                    let rest = rest.trim();
                    let items: Vec<V> = if let Some(r) = rest.strip_prefix("range(").and_then(|x| x.strip_suffix(')')) {
                        let mut a = vec![];
                        for x in split_args(r) { let v = self.eval(&x).map_err(e)?; a.push(num(&v).map_err(e)?); }
                        let (s, en, st) = match a.len() { 0 => return Err(e("range() needs a number".into())), 1 => (0.0, a[0], 1.0), 2 => (a[0], a[1], 1.0), _ => (a[0], a[1], a[2]) };
                        if st == 0.0 { return Err(e("range() step can't be 0".into())); }
                        let count = (((en - s) / st).ceil().max(0.0) as usize).min(MAX_STEPS as usize + 1);
                        (0..count).map(|k| V::N(s + k as f64 * st)).collect()
                    } else {
                        let is_in = c.contains(" in ") && self.js;
                        match self.eval(rest).map_err(e)? {
                            V::L(l) => if is_in { (0..l.len()).map(|k| V::S(k.to_string())).collect() } else { l },
                            V::S(s) => s.chars().map(|c| V::S(c.to_string())).collect(),
                            V::O(o) => o.iter().filter(|(k, _)| !k.starts_with("__")).map(|(k, _)| V::S(k.clone())).collect(),
                            _ => return Err(e("can only loop over range(), a list, text or an object".into())),
                        }
                    };
                    for it in items {
                        self.vars.insert(v.trim().to_string(), it);
                        match self.exec(i + 1, end)? { Flow::Return => return Ok(Flow::Return), Flow::Break => break, _ => {} }
                        self.step()?;
                    }
                } else {
                    let p: Vec<&str> = c.split(';').collect();
                    if p.len() != 3 { return Err(e("unsupported for loop".into())); }
                    self.assign(p[0].trim()).map_err(e)?;
                    loop {
                        let v = self.eval(p[1].trim()).map_err(e)?;
                        if !truthy(&v) { break; }
                        match self.exec(i + 1, end)? { Flow::Return => return Ok(Flow::Return), Flow::Break => break, _ => {} }
                        self.assign(p[2].trim()).map_err(e)?;
                        self.step()?;
                    }
                }
                i = end;
            } else {
                self.assign(t).map_err(e)?;
                i += 1;
            }
        }
        Ok(Flow::Next)
    }

    fn assign(&mut self, t: &str) -> Result<(), String> {
        let t = t.trim_end_matches(';').trim();
        let t = ["let ", "const ", "var "].iter().find_map(|k| t.strip_prefix(k)).unwrap_or(t);
        let is_path = |n: &str| !n.is_empty() && n.split('.').all(|p| !p.is_empty() && p.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '$') && !p.chars().next().unwrap().is_ascii_digit());
        if let Some(n) = t.strip_suffix("++").or_else(|| t.strip_suffix("--")) {
            let n = n.trim();
            let cur = num(&self.get_path(n)?)?;
            return self.set_path(n, V::N(cur + if t.ends_with("++") { 1.0 } else { -1.0 }));
        }
        for o in ["+=", "-=", "*=", "/=", "%="] {
            if let Some((n, r)) = t.split_once(o) {
                let n = n.trim();
                if !is_path(n) { break; }
                let cur = self.get_path(n)?;
                let rv = self.eval(r)?;
                let nv = match (o, &cur, &rv) {
                    ("+=", V::N(a), V::N(b)) => V::N(a + b),
                    ("+=", V::L(a), V::L(b)) => V::L(a.iter().chain(b).cloned().collect()),
                    ("+=", _, _) => V::S(format!("{}{}", self.show(&cur), self.show(&rv))),
                    ("-=", _, _) => V::N(num(&cur)? - num(&rv)?),
                    ("*=", _, _) => V::N(num(&cur)? * num(&rv)?),
                    ("%=", _, _) => V::N(num(&cur)? % num(&rv)?),
                    _ => { let d = num(&rv)?; if d == 0.0 { return Err("division by zero".into()); } V::N(num(&cur)? / d) }
                };
                return self.set_path(n, nv);
            }
        }
        // find a top-level single '=' (not ==, <=, >=, !=, =>)
        let b: Vec<char> = t.chars().collect();
        let mut d = 0; let mut q: Option<char> = None; let mut eq = None;
        for (i, &ch) in b.iter().enumerate() {
            if let Some(qc) = q { if ch == qc { q = None; } continue; }
            match ch {
                '"' | '\'' | '`' => q = Some(ch), '(' | '[' | '{' => d += 1, ')' | ']' | '}' => d -= 1,
                '=' if d == 0 => {
                    let prev = if i > 0 { b[i - 1] } else { ' ' };
                    let next = b.get(i + 1).copied().unwrap_or(' ');
                    if next != '=' && next != '>' && !"=!<>".contains(prev) { eq = Some(i); break; }
                }
                _ => {}
            }
        }
        if let Some(i) = eq {
            let lhs: String = b[..i].iter().collect::<String>().trim().to_string();
            let rhs: String = b[i + 1..].iter().collect();
            let v = self.eval(&rhs)?;
            if is_path(&lhs) { return self.set_path(&lhs, v); }
            // arr[0] = v / obj["k"] = v
            if let Some(open) = lhs.rfind('[').filter(|_| lhs.ends_with(']')) {
                let base = lhs[..open].trim();
                let idx = self.eval(&lhs[open + 1..lhs.len() - 1])?;
                let mut cur = self.get_path(base)?;
                match (&mut cur, &idx) {
                    (V::L(l), _) => { let k = num(&idx)? as usize; if k < l.len() { l[k] = v; } else if k == l.len() { l.push(v); } else { return Err("index out of range".into()); } }
                    (V::O(o), V::S(k)) => { if let Some(e) = o.iter_mut().find(|(n, _)| n == k) { e.1 = v; } else { o.push((k.clone(), v)); } }
                    _ => return Err(format!("can't set an item of {}", base)),
                }
                return self.set_path(base, cur);
            }
            // document.getElementById("x").textContent = v
            if let Some(dot) = last_top_dot(&lhs) {
                let base = self.eval(&lhs[..dot])?;
                if let Some((sel, st)) = el_of(&base) { return self.set_dom(&sel, st, &lhs[dot + 1..], v); }
            }
            return Err(format!("can't assign to {}", lhs));
        }
        // bare expression: a function or method call
        if t.contains('(') {
            // mutating method on a variable: arr.push(x)
            if let Some(open) = t.find('(') {
                let callee = &t[..open];
                if t.ends_with(')') && is_path(callee) && depth_of(&t[open..]) == 0 && t[open..].matches('(').count() >= 1 && split_balanced_call(t) {
                    let inner = &t[open + 1..t.len() - 1];
                    let mut a = vec![];
                    for x in split_args(inner) { a.push(self.eval(&x)?); }
                    self.call(callee, a)?;
                    return Ok(());
                }
            }
            self.eval(t)?;
            return Ok(());
        }
        if is_path(t) { self.get_path(t)?; return Ok(()); }
        Err(format!("cannot understand: {}", t))
    }
}

/// True when `f(...)` is one call whose argument list closes at the very end.
fn split_balanced_call(t: &str) -> bool {
    let Some(open) = t.find('(') else { return false };
    let mut d = 0; let mut q: Option<char> = None;
    for (i, ch) in t.char_indices().skip(open) {
        if let Some(qc) = q { if ch == qc { q = None; } continue; }
        match ch { '"' | '\'' | '`' => q = Some(ch), '(' => d += 1, ')' => { d -= 1; if d == 0 { return i == t.len() - 1; } } _ => {} }
    }
    false
}

fn kebab(p: &str) -> String {
    let mut o = String::new();
    for c in p.chars() { if c.is_uppercase() { o.push('-'); o.extend(c.to_lowercase()); } else { o.push(c); } }
    o
}

fn is_ident(s: &str) -> bool { !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '$') && !s.chars().next().unwrap().is_ascii_digit() }

fn params_of(s: &str) -> Option<Vec<String>> {
    let s = s.trim();
    let inner = if s.starts_with('(') && s.ends_with(')') { &s[1..s.len() - 1] } else if is_ident(s) { s } else { return None };
    let ps: Vec<String> = inner.split(',').map(|p| p.trim().split('=').next().unwrap_or("").trim().to_string()).filter(|p| !p.is_empty()).collect();
    if ps.iter().all(|p| is_ident(p)) { Some(ps) } else { None }
}

/// def f(a): / function f(a) / const f = function(a) / const f = (a) => / const f = (a) => a * 2
fn func_header(t: &str) -> Option<(String, Vec<String>, Option<String>)> {
    let t = t.trim().trim_end_matches(':').trim();
    for kw in ["def ", "function ", "async function "] {
        if let Some(r) = t.strip_prefix(kw) {
            let (name, ps) = r.split_once('(')?;
            return Some((name.trim().to_string(), params_of(&format!("({}", ps))?, None));
        }
    }
    let r = ["const ", "let ", "var "].iter().find_map(|k| t.strip_prefix(k))?;
    let (name, rhs) = r.split_once('=')?;
    let (name, rhs) = (name.trim(), rhs.trim());
    if !is_ident(name) { return None; }
    if let Some(ps) = rhs.strip_prefix("function") { return Some((name.into(), params_of(ps.trim())?, None)); }
    let (ps, body) = rhs.split_once("=>")?;
    let ps = params_of(ps)?;
    let body = body.trim();
    if body.is_empty() { Some((name.into(), ps, None)) } else { Some((name.into(), ps, Some(body.to_string()))) }
}

/// el.addEventListener("click", function() / () => / (event) =>
fn listener_header(t: &str) -> Option<(String, String, Vec<String>)> {
    let pos = t.find(".addEventListener(")?;
    let target = t[..pos].to_string();
    let rest = &t[pos + ".addEventListener(".len()..];
    let (evt, cb) = rest.split_once(',')?;
    let evt = evt.trim().trim_matches(|c| c == '"' || c == '\'').to_string();
    let cb = cb.trim();
    let ps = if let Some(p) = cb.strip_prefix("function") { params_of(p.trim())? } else { params_of(cb.strip_suffix("=>")?.trim())? };
    Some((target, evt, ps))
}

fn balanced(s: &str) -> bool {
    let mut d = 0i32;
    for c in s.chars() { if c == '(' { d += 1; } else if c == ')' { d -= 1; if d < 0 { return false; } } }
    d == 0
}

pub fn run(lang: &str, src: &str) -> Vec<(Kind, String)> {
    if src.len() > MAX_SRC { return vec![(Kind::Err, format!("This program is too long to run ({} characters, limit {}).", src.len(), MAX_SRC))]; }
    match lang {
        "html" => return html(src),
        "css" => return css_out(src),
        "sql" => return sql_out(src),
        _ => {}
    }
    let js = lang == "javascript";
    let lines = normalize(src, js);
    let n = lines.len();
    let seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(42) | 1;
    let mut r = Run { lines: &lines, vars: HashMap::new(), funcs: HashMap::new(), out: vec![], steps: 0, js, depth: 0, ret: None, seed, dom: HashMap::new() };
    match r.exec(0, n) {
        Err(e) => r.out.push((Kind::Err, e)),
        Ok(Flow::Return) => r.out.push((Kind::Err, "'return' only works inside a function".into())),
        Ok(Flow::Break) | Ok(Flow::Continue) => r.out.push((Kind::Err, "'break' and 'continue' only work inside a loop".into())),
        Ok(Flow::Next) => {}
    }
    if r.out.is_empty() {
        let hint = if !r.funcs.is_empty() { "(function defined - call it to see a result)".to_string() }
            else if !r.vars.is_empty() { let mut ks: Vec<_> = r.vars.keys().cloned().collect(); ks.sort(); format!("(no output - stored {}. Print it to see it)", ks.join(", ")) }
            else if js { "(no output - try console.log(\"hi\"))".into() } else { "(no output - try print(\"hi\"))".into() };
        r.out.push((Kind::Out, hint));
    }
    r.out
}

fn html(src: &str) -> Vec<(Kind, String)> {
    let mut out = vec![];
    let mut rest = src;
    while let Some(a) = rest.find('<') {
        let Some(b) = rest[a..].find('>') else { break };
        let tag = rest[a + 1..a + b].trim().to_string();
        let lower = tag.to_lowercase();
        let after = &rest[a + b + 1..];
        if lower.starts_with('/') || lower.starts_with('!') { rest = after; continue; }
        let name = lower.split_whitespace().next().unwrap_or("").trim_end_matches('/').to_string();
        let close = format!("</{}>", name);
        if let Some(e) = after.to_lowercase().find(&close) {
            let inner = &after[..e];
            let text = strip(inner);
            let k = match name.as_str() { "h1" => Some(Kind::H1), "h2" | "h3" | "h4" | "h5" | "h6" => Some(Kind::H2), "p" | "span" | "b" | "i" | "a" | "strong" | "em" | "label" => Some(Kind::P), "li" => Some(Kind::Li), "button" => Some(Kind::Btn), _ => None };
            if let Some(k) = k { if !text.is_empty() { out.push((k, text)); } rest = &after[e + close.len()..]; continue; }
            if name == "title" { out.push((Kind::H2, format!("Tab title: {}", text))); rest = &after[e + close.len()..]; continue; }
            if name == "textarea" { out.push((Kind::Btn, format!("[ {} ]", if text.is_empty() { attr(&tag, "placeholder").unwrap_or_else(|| "text area".into()) } else { text }))); rest = &after[e + close.len()..]; continue; }
            if name == "select" {
                let first = inner.to_lowercase().find("<option").and_then(|o| inner[o..].find('>').map(|g| strip(inner[o + g + 1..].split('<').next().unwrap_or("")))).unwrap_or_default();
                let n = inner.to_lowercase().matches("<option").count();
                out.push((Kind::Btn, format!("[ {} v ]  ({} choice{})", first, n, if n == 1 { "" } else { "s" })));
                rest = &after[e + close.len()..]; continue;
            }
            // containers (body, header, footer, div, main, section...) holding plain text
            if !inner.contains('<') && !text.is_empty() { out.push((Kind::P, text)); rest = &after[e + close.len()..]; continue; }
        }
        match name.as_str() {
            "img" => out.push((Kind::Img, attr(&tag, "alt").or_else(|| attr(&tag, "src")).unwrap_or_else(|| "picture".into()))),
            "input" => {
                let ty = attr(&tag, "type").unwrap_or_else(|| "text".into()).to_lowercase();
                let val = attr(&tag, "value");
                let req = if lower.contains("required") { " *" } else { "" };
                out.push(match ty.as_str() {
                    "submit" | "button" | "reset" => (Kind::Btn, val.unwrap_or_else(|| if ty == "reset" { "Reset".into() } else { "Submit".into() })),
                    "checkbox" => (Kind::P, format!("[{}] {}{}", if lower.contains("checked") { "x" } else { " " }, val.or_else(|| attr(&tag, "name")).unwrap_or_default(), req)),
                    "radio" => (Kind::P, format!("({}) {}{}", if lower.contains("checked") { "o" } else { " " }, val.or_else(|| attr(&tag, "name")).unwrap_or_default(), req)),
                    _ => (Kind::Btn, format!("[ {}{} ]", attr(&tag, "placeholder").or(val).unwrap_or_else(|| format!("{} box", ty)), req)),
                });
            }
            "hr" => out.push((Kind::P, "----------------".into())),
            _ => {}
        }
        rest = after;
    }
    if out.is_empty() {
        if !src.contains('<') && !src.trim().is_empty() { out.push((Kind::P, src.trim().to_string())); }
        else { out.push((Kind::Err, "Nothing to show. Try <h1>Hello</h1>".into())); }
    }
    out
}

fn attr(tag: &str, name: &str) -> Option<String> {
    let lower = tag.to_lowercase();
    let i = lower.find(&format!("{}=", name))?;
    let r = &tag[i + name.len() + 1..];
    let q = r.chars().next()?;
    if q == '"' || q == '\'' { r[1..].split(q).next().map(|x| x.to_string()) } else { r.split_whitespace().next().map(|x| x.trim_end_matches('/').to_string()) }
}

fn css_out(src: &str) -> Vec<(Kind, String)> {
    match crate::css::parse(src) {
        Err(e) => vec![(Kind::Err, e)],
        Ok(rules) => {
            let mut out = vec![];
            for r in &rules {
                let decls: Vec<String> = r.decls.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                out.push((Kind::Out, format!("{}  ->  {}", r.selector, if decls.is_empty() { "(no properties)".into() } else { decls.join("; ") })));
                out.push((Kind::Styled, crate::css::style_of(r).encode("Hello, hero!")));
            }
            out
        }
    }
}

fn sql_out(src: &str) -> Vec<(Kind, String)> {
    let mut out = vec![];
    for stmt in src.split(';').map(|s| s.trim()).filter(|s| !s.is_empty()) {
        match crate::sql::query(stmt) {
            Err(e) => { out.push((Kind::Err, e)); break; }
            Ok(rows) => {
                let n = rows.len() - 1;
                out.push((Kind::Table, rows.iter().map(|r| r.join("\u{1f}")).collect::<Vec<_>>().join("\n")));
                out.push((Kind::Out, format!("{} row{}", n, if n == 1 { "" } else { "s" })));
            }
        }
    }
    if out.is_empty() { out.push((Kind::Err, "Type a query, for example: SELECT * FROM heroes;".into())); }
    out
}

fn strip(s: &str) -> String {
    let mut o = String::new();
    let mut inside = false;
    for c in s.chars() { match c { '<' => inside = true, '>' => inside = false, _ if !inside => o.push(c), _ => {} } }
    o.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn outs(lang: &str, src: &str) -> Vec<String> { run(lang, src).into_iter().map(|(k, t)| format!("{}{}", if k == Kind::Err { "!" } else { "" }, t)).collect() }

    #[test]
    fn python_basics() {
        assert_eq!(outs("python", "x = 2\ny = 3\nprint(x * y)"), vec!["6"]);
        assert_eq!(outs("python", "print(\"Hi\", 5 > 3)"), vec!["Hi True"]);
        assert_eq!(outs("python", "for i in range(3):\n  print(i)"), vec!["0", "1", "2"]);
        assert_eq!(outs("python", "for i in range(1, 10, 4):\n  print(i)"), vec!["1", "5", "9"]);
        assert_eq!(outs("python", "n = 3\nif n > 5:\n  print(\"big\")\nelse:\n  print(\"small\")"), vec!["small"]);
        assert_eq!(outs("python", "n = 5\nif n > 5:\n  print(\"big\")\nelif n == 5:\n  print(\"five\")\nelse:\n  print(\"small\")"), vec!["five"]);
        assert_eq!(outs("python", "print(7 // 2, 7 % 3, -5 % 3)"), vec!["3 1 1"]);
        assert_eq!(outs("python", "print(\"a\" + \"b\", \"ha\" * 3)"), vec!["ab hahaha"]);
        assert!(outs("python", "print(\"hp: \" + 5)")[0].starts_with('!'));
        assert_eq!(outs("python", "print(None, not True)"), vec!["None False"]);
    }

    #[test]
    fn python_lists_and_functions() {
        assert_eq!(outs("python", "loot = [\"gem\", \"coin\"]\nprint(loot[0])\nprint(len(loot))\nprint(loot)"), vec!["gem", "2", "['gem', 'coin']"]);
        assert_eq!(outs("python", "nums = [1, 2]\nnums.append(3)\nprint(nums, nums[-1], sum(nums))"), vec!["[1, 2, 3] 3 6"]);
        assert_eq!(outs("python", "for item in [\"a\", \"b\"]:\n  print(item)"), vec!["a", "b"]);
        assert_eq!(outs("python", "def heal(hp):\n  return hp + 10\n\nprint(heal(5))"), vec!["15"]);
        assert_eq!(outs("python", "def double(n):\n  return n * 2"), vec!["(function defined - call it to see a result)"]);
        assert_eq!(outs("python", "def fact(n):\n  if n <= 1:\n    return 1\n  return n * fact(n - 1)\nprint(fact(5))"), vec!["120"]);
        assert_eq!(outs("python", "def hi(name):\n  print(\"Hi\", name)\nhi(\"Ada\")"), vec!["Hi Ada"]);
        assert!(outs("python", "print([1][5])")[0].contains("out of range"));
        assert!(outs("python", "def f(a):\n  return a\nprint(f())")[0].contains("needs 1"));
        assert!(outs("python", "return 5")[0].contains("inside a function"));
    }

    #[test]
    fn javascript_basics() {
        assert_eq!(outs("javascript", "let a = 12;\nlet b = 5;\nconsole.log(\"Damage:\", a - b);"), vec!["Damage: 7"]);
        assert_eq!(outs("javascript", "for (let i = 3; i > 0; i--) {\n  console.log(i);\n}\nconsole.log(\"Cast!\");"), vec!["3", "2", "1", "Cast!"]);
        assert_eq!(outs("javascript", "console.log(true, \"hp: \" + 5)"), vec!["true hp: 5"]);
        assert_eq!(outs("javascript", "let x = 4;\nif (x > 5) {\n  console.log(\"big\");\n} else if (x > 3) {\n  console.log(\"mid\");\n} else {\n  console.log(\"small\");\n}"), vec!["mid"]);
    }

    #[test]
    fn javascript_arrays_and_functions() {
        assert_eq!(outs("javascript", "let loot = [\"gem\", \"coin\"];\nconsole.log(loot[0]);\nconsole.log(loot.length);\nconsole.log(loot);"), vec!["gem", "2", "[ 'gem', 'coin' ]"]);
        assert_eq!(outs("javascript", "function heal(hp) {\n  return hp + 10;\n}\nconsole.log(heal(5));"), vec!["15"]);
        assert_eq!(outs("javascript", "function double(n) { return n * 2; }\nconsole.log(double(4));"), vec!["8"]);
        assert_eq!(outs("javascript", "let n = [1];\nn.push(2);\nfor (const x of n) {\n  console.log(x);\n}"), vec!["1", "2"]);
    }

    #[test]
    fn endless_loops_and_deep_nesting_stop_safely() {
        assert!(outs("python", "while True:\n  x = 1").last().unwrap().contains("too many steps"));
        assert!(outs("javascript", "while (true) {\n}").last().unwrap().starts_with('!'));
        let mut deep = String::new();
        for d in 0..60 { deep.push_str(&format!("{}if True:\n", "  ".repeat(d))); }
        deep.push_str(&format!("{}print(1)\n", "  ".repeat(60)));
        assert!(outs("python", &deep).last().unwrap().contains("nested"));
        assert!(outs("python", &"x = 1\n".repeat(5000))[0].contains("too long"));
        assert!(outs("python", "while True:\n  print(1)").last().unwrap().starts_with('!'));
        assert!(outs("python", "def f(n):\n  return f(n + 1)\nprint(f(1))").last().unwrap().starts_with('!'));
        assert!(outs("python", "for i in range(100000000):\n  x = i").last().unwrap().contains("too many steps"));
        assert!(outs("python", "x = \"a\" * 100000000\nprint(len(x))")[0].len() < 10);
    }

    #[test]
    fn html_preview() {
        let o = run("html", "<h1>Hi</h1><ul><li>Sword</li></ul><img src=\"hero.png\" alt=\"Hero\">");
        assert_eq!(o[0], (Kind::H1, "Hi".to_string()));
        assert_eq!(o[1], (Kind::Li, "Sword".to_string()));
        assert_eq!(o[2], (Kind::Img, "Hero".to_string()));
        assert_eq!(run("html", "<body>Hello</body>")[0], (Kind::P, "Hello".to_string()));
        assert_eq!(run("html", "<footer>End</footer>")[0], (Kind::P, "End".to_string()));
    }

    #[test]
    fn css_and_sql_dispatch() {
        let o = run("css", "p { color: red; }");
        assert_eq!(o[0].0, Kind::Out);
        assert_eq!(o[1].0, Kind::Styled);
        let o = run("sql", "SELECT name FROM heroes LIMIT 2;");
        assert_eq!(o[0], (Kind::Table, "name\nAria\nBorin".to_string()));
        assert_eq!(o[1].1, "2 rows");
        assert_eq!(run("sql", "SELECT nope FROM heroes")[0].0, Kind::Err);
    }
}

#[cfg(test)]
mod tests_v2 {
    use super::*;
    fn o(lang: &str, src: &str) -> Vec<String> { run(lang, src).into_iter().map(|(k, t)| format!("{}{}", match k { Kind::Err => "!", Kind::Page => "@", _ => "" }, t)).collect() }

    #[test]
    fn js_objects() {
        let src = "const hero = {\n  name: \"Ada\",\n  hp: 10,\n  bag: [\"map\"]\n};\nconsole.log(hero.name);\nhero.hp -= 3;\nhero.level = 2;\nhero.bag.push(\"key\");\nconsole.log(hero.hp, hero.level, hero.bag.length);\nconsole.log(hero);";
        assert_eq!(o("javascript", src), vec!["Ada", "7 2 2", "{ name: 'Ada', hp: 7, bag: [ 'map', 'key' ], level: 2 }"]);
        assert_eq!(o("javascript", "const car = { color: \"red\" };\nconsole.log(car[\"color\"], typeof car, Object.keys(car));"), vec!["red object [ 'color' ]"]);
        assert_eq!(o("javascript", "const p = { a: 1, b: 2 };\nfor (const k in p) {\n  console.log(k, p[k]);\n}"), vec!["a 1", "b 2"]);
    }

    #[test]
    fn js_templates_ternary_typeof_methods() {
        assert_eq!(o("javascript", "let name = \"Ada\";\nlet hp = 7;\nconsole.log(`${name} has ${hp * 2} hp`);"), vec!["Ada has 14 hp"]);
        assert_eq!(o("javascript", "let x = 5;\nconsole.log(x > 3 ? \"big\" : \"small\", typeof x, typeof \"a\");"), vec!["big number string"]);
        assert_eq!(o("javascript", "let w = \"Hello\";\nconsole.log(w.toUpperCase(), w.length, w.includes(\"ell\"), \"a-b\".split(\"-\"));"), vec!["HELLO 5 true [ 'a', 'b' ]"]);
        assert_eq!(o("javascript", "let n = 3.14159;\nconsole.log(n.toFixed(2), [1, 2, 3].join(\"+\"));"), vec!["3.14 1+2+3"]);
        let r = o("javascript", "let roll = Math.floor(Math.random() * 6) + 1;\nconsole.log(roll >= 1 && roll <= 6);");
        assert_eq!(r, vec!["true"]);
    }

    #[test]
    fn js_functions_all_forms_and_break() {
        assert_eq!(o("javascript", "const add = (a, b) => a + b;\nconsole.log(add(2, 3));"), vec!["5"]);
        assert_eq!(o("javascript", "const greet = function(name) {\n  return \"Hi \" + name;\n};\nconsole.log(greet(\"Bo\"));"), vec!["Hi Bo"]);
        assert_eq!(o("javascript", "const sq = (n) => {\n  return n * n;\n};\nconsole.log(sq(4));"), vec!["16"]);
        assert_eq!(o("javascript", "for (let i = 0; i < 10; i++) {\n  if (i === 3) {\n    break;\n  }\n  if (i === 1) {\n    continue;\n  }\n  console.log(i);\n}"), vec!["0", "2"]);
        assert_eq!(o("javascript", "let n = 0;\nwhile (true) {\n  n++;\n  if (n > 4) break;\n}\nconsole.log(n);"), vec!["5"]);
    }

    #[test]
    fn js_dom_simulation() {
        let src = "const title = document.getElementById(\"title\");\ntitle.textContent = \"Welcome!\";\ntitle.style.color = \"gold\";\nconsole.log(title.textContent);";
        assert_eq!(o("javascript", src), vec!["@#title now shows: Welcome!", "@#title style color = gold", "Welcome!"]);
        let src = "const btn = document.querySelector(\"#go\");\nbtn.addEventListener(\"click\", () => {\n  alert(\"Go!\");\n});";
        assert_eq!(o("javascript", src), vec!["@#go listens for 'click'", "@(simulating a click on #go)", "@[alert] Go!"]);
        let src = "function shout() {\n  console.log(\"HEY\");\n}\nconst b = document.getElementById(\"b\");\nb.addEventListener(\"click\", shout);";
        assert_eq!(o("javascript", src), vec!["@#b listens for 'click'", "@(simulating a click on #b)", "HEY"]);
        assert_eq!(o("javascript", "document.getElementById(\"x\").innerHTML = \"<b>hi</b>\";"), vec!["@#x HTML set to: <b>hi</b>"]);
        assert_eq!(o("javascript", "document.getElementById(\"box\").style.backgroundColor = \"red\";"), vec!["@#box style background-color = red"]);
    }

    #[test]
    fn python_dicts_fstrings() {
        assert_eq!(o("python", "hero = {\"name\": \"Ada\", \"hp\": 10}\nhero[\"hp\"] = 7\nprint(hero[\"name\"], hero[\"hp\"])\nprint(hero)"), vec!["Ada 7", "{'name': 'Ada', 'hp': 7}"]);
        assert_eq!(o("python", "name = \"Ada\"\nprint(f\"Hi {name}, {2 + 3}!\")"), vec!["Hi Ada, 5!"]);
        assert_eq!(o("python", "for i in range(10):\n  if i == 2:\n    break\n  print(i)"), vec!["0", "1"]);
        assert_eq!(o("python", "x = 3  # three\nprint(\"a # b\", x)"), vec!["a # b 3"]);
        assert_eq!(o("javascript", "console.log(\"Hi\"); // say hi\nconsole.log(\"http://x\");"), vec!["Hi", "http://x"]);
    }

    #[test]
    fn html_forms_and_semantics() {
        let out = run("html", "<form>\n<label for=\"n\">Name</label>\n<input type=\"text\" id=\"n\" placeholder=\"Your name\" required>\n<input type=\"checkbox\" value=\"Mage\">\n<input type=\"radio\" name=\"c\" value=\"Knight\">\n<select><option>Sword</option><option>Bow</option></select>\n<textarea placeholder=\"Bio\"></textarea>\n<input type=\"submit\" value=\"Join\">\n</form>");
        let txt: Vec<String> = out.iter().map(|(_, t)| t.clone()).collect();
        assert_eq!(txt, vec!["Name", "[ Your name * ]", "[ ] Mage", "( ) Knight", "[ Sword v ]  (2 choices)", "[ Bio ]", "Join"]);
        let out = run("html", "<header><h1>Log</h1><nav><a href=\"#\">Home</a></nav></header><main><article><h2>Day 1</h2><p>Hi</p></article></main><footer>(c) me</footer>");
        assert_eq!(out.len(), 5);
        assert!(out.iter().all(|(k, _)| *k != Kind::Err));
    }
}
