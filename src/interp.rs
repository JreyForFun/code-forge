// A small interpreter so the Code tab really runs Python-style and JavaScript-style snippets,
// and a tiny HTML previewer.
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum V { N(f64), S(String), B(bool) }

#[derive(Clone, Copy, PartialEq)]
pub enum Kind { Out, Err, H1, H2, P, Li, Btn }

fn show(v: &V) -> String {
    match v {
        V::N(n) => if n.fract() == 0.0 && n.abs() < 1e15 { format!("{}", *n as i64) } else { format!("{}", n) },
        V::S(s) => s.clone(),
        V::B(b) => b.to_string(), // python-style True/False handled by lang flag
    }
}

#[derive(Debug, Clone, PartialEq)]
enum T { Num(f64), Str(String), Id(String), Op(String) }

fn lex(s: &str) -> Result<Vec<T>, String> {
    let c: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut out = vec![];
    while i < c.len() {
        let ch = c[i];
        if ch.is_whitespace() { i += 1; }
        else if ch.is_ascii_digit() || (ch == '.' && i + 1 < c.len() && c[i + 1].is_ascii_digit()) {
            let st = i;
            while i < c.len() && (c[i].is_ascii_digit() || c[i] == '.') { i += 1; }
            out.push(T::Num(c[st..i].iter().collect::<String>().parse().map_err(|_| "bad number".to_string())?));
        } else if ch == '"' || ch == '\'' || ch == '`' {
            let q = ch;
            i += 1;
            let mut v = String::new();
            while i < c.len() && c[i] != q {
                if c[i] == '\\' && i + 1 < c.len() {
                    i += 1;
                    v.push(match c[i] { 'n' => '\n', 't' => '\t', x => x });
                } else { v.push(c[i]); }
                i += 1;
            }
            if i >= c.len() { return Err("missing closing quote".into()); }
            i += 1;
            out.push(T::Str(v));
        } else if ch.is_alphabetic() || ch == '_' || ch == '$' {
            let st = i;
            while i < c.len() && (c[i].is_alphanumeric() || c[i] == '_' || c[i] == '$' || c[i] == '.') { i += 1; }
            out.push(T::Id(c[st..i].iter().collect()));
        } else {
            let two: String = c[i..(i + 2).min(c.len())].iter().collect();
            let three: String = c[i..(i + 3).min(c.len())].iter().collect();
            if three == "===" || three == "!==" { out.push(T::Op(three)); i += 3; }
            else if ["==", "!=", "<=", ">=", "&&", "||", "**", "//"].contains(&two.as_str()) { out.push(T::Op(two)); i += 2; }
            else if "+-*/%()<>,![]".contains(ch) { out.push(T::Op(ch.to_string())); i += 1; }
            else { return Err(format!("unexpected '{}'", ch)); }
        }
    }
    Ok(out)
}

struct Ev<'a> { t: Vec<T>, i: usize, vars: &'a HashMap<String, V> }

impl<'a> Ev<'a> {
    fn peek(&self) -> Option<&T> { self.t.get(self.i) }
    fn op(&mut self, o: &str) -> bool {
        if matches!(self.peek(), Some(T::Op(x)) if x == o) { self.i += 1; true } else { false }
    }
    fn or(&mut self) -> Result<V, String> {
        let mut l = self.and()?;
        loop {
            if self.op("||") || self.kw("or") { let r = self.and()?; l = V::B(truthy(&l) || truthy(&r)); } else { return Ok(l); }
        }
    }
    fn kw(&mut self, k: &str) -> bool {
        if matches!(self.peek(), Some(T::Id(x)) if x == k) { self.i += 1; true } else { false }
    }
    fn and(&mut self) -> Result<V, String> {
        let mut l = self.cmp()?;
        loop {
            if self.op("&&") || self.kw("and") { let r = self.cmp()?; l = V::B(truthy(&l) && truthy(&r)); } else { return Ok(l); }
        }
    }
    fn cmp(&mut self) -> Result<V, String> {
        let l = self.add()?;
        for o in ["===", "!==", "==", "!=", "<=", ">=", "<", ">"] {
            if self.op(o) {
                let r = self.add()?;
                let ord = match (&l, &r) {
                    (V::N(a), V::N(b)) => a.partial_cmp(b),
                    _ => show(&l).partial_cmp(&show(&r)),
                };
                let eq = l == r || show(&l) == show(&r) && matches!((&l, &r), (V::S(_), V::S(_)));
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
                    _ => V::S(format!("{}{}", show(&l), show(&r))),
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
                    (V::S(s), V::N(n)) | (V::N(n), V::S(s)) => V::S(s.repeat(n.max(0.0) as usize)),
                    _ => V::N(num(&l)? * num(&r)?),
                };
            } else if self.op("//") { let r = self.un()?; let d = num(&r)?; if d == 0.0 { return Err("division by zero".into()); } l = V::N((num(&l)? / d).floor()); }
            else if self.op("/") { let r = self.un()?; let d = num(&r)?; if d == 0.0 { return Err("division by zero".into()); } l = V::N(num(&l)? / d); }
            else if self.op("%") { let r = self.un()?; let d = num(&r)?; if d == 0.0 { return Err("division by zero".into()); } l = V::N(num(&l)?.rem_euclid(d)); }
            else { return Ok(l); }
        }
    }
    fn un(&mut self) -> Result<V, String> {
        if self.op("-") { let v = self.un()?; return Ok(V::N(-num(&v)?)); }
        if self.op("!") || self.kw("not") { let v = self.un()?; return Ok(V::B(!truthy(&v))); }
        self.prim()
    }
    fn args(&mut self) -> Result<Vec<V>, String> {
        let mut a = vec![];
        if self.op(")") { return Ok(a); }
        loop {
            a.push(self.or()?);
            if self.op(",") { continue; }
            if self.op(")") { return Ok(a); }
            return Err("expected ) or ,".into());
        }
    }
    fn prim(&mut self) -> Result<V, String> {
        let t = self.t.get(self.i).cloned().ok_or("unfinished expression")?;
        self.i += 1;
        match t {
            T::Num(n) => Ok(V::N(n)),
            T::Str(s) => Ok(V::S(s)),
            T::Op(o) if o == "(" => { let v = self.or()?; if !self.op(")") { return Err("missing )".into()); } Ok(v) }
            T::Id(id) => {
                if self.op("(") {
                    let a = self.args()?;
                    return call(&id, &a);
                }
                match id.as_str() {
                    "True" | "true" => Ok(V::B(true)),
                    "False" | "false" => Ok(V::B(false)),
                    "None" | "null" | "undefined" => Ok(V::S("None".into())),
                    "Math.PI" => Ok(V::N(std::f64::consts::PI)),
                    _ => self.vars.get(&id).cloned().ok_or(format!("'{}' is not defined", id)),
                }
            }
            T::Op(o) => Err(format!("unexpected '{}'", o)),
        }
    }
}

fn truthy(v: &V) -> bool {
    match v { V::N(n) => *n != 0.0, V::S(s) => !s.is_empty() && s != "None", V::B(b) => *b }
}
fn num(v: &V) -> Result<f64, String> {
    match v { V::N(n) => Ok(*n), V::B(b) => Ok(*b as i32 as f64), V::S(s) => s.trim().parse().map_err(|_| format!("'{}' is not a number", s)) }
}

fn call(f: &str, a: &[V]) -> Result<V, String> {
    let n = |i: usize| -> Result<f64, String> { num(a.get(i).ok_or("missing argument")?) };
    Ok(match f {
        "len" => V::N(show(a.first().ok_or("missing argument")?).chars().count() as f64),
        "str" | "String" => V::S(show(a.first().ok_or("missing argument")?)),
        "int" | "parseInt" | "Math.floor" => V::N(n(0)?.floor()),
        "float" | "Number" | "parseFloat" => V::N(n(0)?),
        "abs" | "Math.abs" => V::N(n(0)?.abs()),
        "round" | "Math.round" => V::N(n(0)?.round()),
        "Math.sqrt" => V::N(n(0)?.sqrt()),
        "Math.ceil" => V::N(n(0)?.ceil()),
        "max" | "Math.max" => V::N(n(0)?.max(n(1)?)),
        "min" | "Math.min" => V::N(n(0)?.min(n(1)?)),
        "upper" => V::S(show(&a[0]).to_uppercase()),
        _ => return Err(format!("unknown function '{}'", f)),
    })
}

fn eval(s: &str, vars: &HashMap<String, V>) -> Result<V, String> {
    let mut e = Ev { t: lex(s)?, i: 0, vars };
    let v = e.or()?;
    if e.i < e.t.len() { return Err("extra text after expression".into()); }
    Ok(v)
}

// split "a, b" at top-level commas
fn split_args(s: &str) -> Vec<String> {
    let mut out = vec![];
    let mut depth = 0;
    let mut q: Option<char> = None;
    let mut cur = String::new();
    for ch in s.chars() {
        if let Some(qc) = q { cur.push(ch); if ch == qc { q = None; } continue; }
        match ch {
            '"' | '\'' | '`' => { q = Some(ch); cur.push(ch); }
            '(' | '[' => { depth += 1; cur.push(ch); }
            ')' | ']' => { depth -= 1; cur.push(ch); }
            ',' if depth == 0 => { out.push(cur.trim().to_string()); cur.clear(); }
            _ => cur.push(ch),
        }
    }
    if !cur.trim().is_empty() { out.push(cur.trim().to_string()); }
    out
}

struct Line { ind: usize, text: String, no: usize }

fn normalize(src: &str, js: bool) -> Vec<Line> {
    let mut out = vec![];
    let mut depth: usize = 0;
    for (no, raw) in src.lines().enumerate() {
        let mut t = raw.trim().to_string();
        if t.is_empty() || t.starts_with('#') || t.starts_with("//") { continue; }
        if js {
            t = t.trim_end_matches(';').trim().to_string();
            let mut closed = false;
            while t.starts_with('}') { depth = depth.saturating_sub(1); t = t[1..].trim().to_string(); closed = true; }
            let _ = closed;
            if t.is_empty() { continue; }
            if t.ends_with('{') {
                t = t[..t.len() - 1].trim().to_string();
                out.push(Line { ind: depth, text: t, no: no + 1 });
                depth += 1;
                continue;
            }
            out.push(Line { ind: depth, text: t, no: no + 1 });
        } else {
            let ind = raw.len() - raw.trim_start().len();
            out.push(Line { ind: ind / 2, text: t, no: no + 1 });
        }
    }
    out
}

struct Run { vars: HashMap<String, V>, out: Vec<(Kind, String)>, steps: u32, js: bool }

impl Run {
    fn block_end(lines: &[Line], i: usize) -> usize {
        let base = lines[i].ind;
        let mut j = i + 1;
        while j < lines.len() && lines[j].ind > base { j += 1; }
        j
    }
    fn exec(&mut self, lines: &[Line], from: usize, to: usize) -> Result<(), String> {
        let mut i = from;
        while i < to {
            self.steps += 1;
            if self.steps > 20000 { return Err("stopped: too many steps (infinite loop?)".into()); }
            let l = &lines[i];
            let e = |m: String| format!("line {}: {}", l.no, m);
            let t = l.text.as_str();
            let end = Self::block_end(lines, i);
            let hdr = |t: &str, kw: &str| -> Option<String> {
                let r = t.strip_prefix(kw)?;
                if !(r.starts_with(' ') || r.starts_with('(')) { return None; }
                let r = r.trim().trim_end_matches(':').trim();
                let r = if r.starts_with('(') && r.ends_with(')') { &r[1..r.len() - 1] } else { r };
                Some(r.to_string())
            };
            if let Some(args) = t.strip_prefix("print(").or_else(|| t.strip_prefix("console.log(")) {
                let inner = args.strip_suffix(')').ok_or(e("missing )".into()))?;
                let parts: Vec<String> = split_args(inner).iter().map(|a| eval(a, &self.vars).map(|v| self.fmt(&v))).collect::<Result<_, _>>().map_err(&e)?;
                self.out.push((Kind::Out, parts.join(" ")));
                i += 1;
            } else if let Some(c) = hdr(t, "if") {
                let ok = truthy(&eval(&c, &self.vars).map_err(&e)?);
                let has_else = end < to && (lines[end].text == "else" || lines[end].text == "else:") && lines[end].ind == l.ind;
                let end2 = if has_else { Self::block_end(lines, end) } else { end };
                if ok { self.exec(lines, i + 1, end)?; } else if has_else { self.exec(lines, end + 1, end2)?; }
                i = end2;
            } else if let Some(c) = hdr(t, "while") {
                while truthy(&eval(&c, &self.vars).map_err(&e)?) {
                    self.exec(lines, i + 1, end)?;
                    self.steps += 1;
                    if self.steps > 20000 { return Err("stopped: too many steps (infinite loop?)".into()); }
                }
                i = end;
            } else if let Some(c) = hdr(t, "for") {
                if let Some((v, rest)) = c.split_once(" in ") {
                    let r = rest.trim();
                    let r = r.strip_prefix("range(").and_then(|x| x.strip_suffix(')')).ok_or(e("only range(...) loops are supported".into()))?;
                    let a: Vec<f64> = split_args(r).iter().map(|x| eval(x, &self.vars).and_then(|v| num(&v))).collect::<Result<_, _>>().map_err(&e)?;
                    let (s, en) = if a.len() >= 2 { (a[0], a[1]) } else { (0.0, *a.first().unwrap_or(&0.0)) };
                    let mut k = s;
                    while k < en {
                        self.vars.insert(v.trim().to_string(), V::N(k));
                        self.exec(lines, i + 1, end)?;
                        k += 1.0;
                    }
                } else {
                    // js: let i = a; i < b; i++
                    let p: Vec<&str> = c.split(';').collect();
                    if p.len() != 3 { return Err(e("unsupported for loop".into())); }
                    self.assign(p[0].trim()).map_err(&e)?;
                    while truthy(&eval(p[1].trim(), &self.vars).map_err(&e)?) {
                        self.exec(lines, i + 1, end)?;
                        let inc = p[2].trim();
                        if let Some(n) = inc.strip_suffix("++") { let cur = self.vars.get(n).map(|v| num(v)).transpose().map_err(&e)?.unwrap_or(0.0); self.vars.insert(n.into(), V::N(cur + 1.0)); }
                        else { self.assign(inc).map_err(&e)?; }
                        self.steps += 1;
                        if self.steps > 20000 { return Err("stopped: too many steps (infinite loop?)".into()); }
                    }
                }
                i = end;
            } else {
                self.assign(t).map_err(&e)?;
                i += 1;
            }
        }
        Ok(())
    }
    fn fmt(&self, v: &V) -> String {
        match v { V::B(b) if !self.js => if *b { "True".into() } else { "False".into() }, _ => show(v) }
    }
    fn assign(&mut self, t: &str) -> Result<(), String> {
        let t = t.trim_end_matches(';');
        let t = ["let ", "const ", "var "].iter().find_map(|k| t.strip_prefix(k)).unwrap_or(t);
        for o in ["+=", "-=", "*=", "/="] {
            if let Some((n, r)) = t.split_once(o) {
                let n = n.trim();
                let cur = self.vars.get(n).cloned().ok_or(format!("'{}' is not defined", n))?;
                let rv = eval(r, &self.vars)?;
                let nv = match (o, &cur, &rv) {
                    ("+=", V::N(a), V::N(b)) => V::N(a + b),
                    ("+=", _, _) => V::S(format!("{}{}", show(&cur), show(&rv))),
                    ("-=", _, _) => V::N(num(&cur)? - num(&rv)?),
                    ("*=", _, _) => V::N(num(&cur)? * num(&rv)?),
                    _ => V::N(num(&cur)? / num(&rv)?),
                };
                self.vars.insert(n.into(), nv);
                return Ok(());
            }
        }
        if let Some((n, r)) = t.split_once('=') {
            let n = n.trim();
            if !r.starts_with('=') && !n.is_empty() && n.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '$') {
                let v = eval(r, &self.vars)?;
                self.vars.insert(n.into(), v);
                return Ok(());
            }
        }
        Err(format!("cannot understand: {}", t))
    }
}

pub fn run(lang: &str, src: &str) -> Vec<(Kind, String)> {
    if lang == "html" { return html(src); }
    let js = lang == "javascript";
    let lines = normalize(src, js);
    let mut r = Run { vars: HashMap::new(), out: vec![], steps: 0, js };
    let n = lines.len();
    if let Err(e) = r.exec(&lines, 0, n) { r.out.push((Kind::Err, e)); }
    if r.out.is_empty() { r.out.push((Kind::Out, "(no output - try print(\"hi\"))".into())); }
    r.out
}

fn html(src: &str) -> Vec<(Kind, String)> {
    let mut out = vec![];
    let mut rest = src;
    while let Some(a) = rest.find('<') {
        let Some(b) = rest[a..].find('>') else { break };
        let tag = rest[a + 1..a + b].trim().to_lowercase();
        let after = &rest[a + b + 1..];
        if tag.starts_with('/') || tag.starts_with('!') { rest = after; continue; }
        let name = tag.split_whitespace().next().unwrap_or("").to_string();
        let close = format!("</{}>", name);
        if let Some(e) = after.to_lowercase().find(&close) {
            let text = strip(&after[..e]);
            let k = match name.as_str() { "h1" => Some(Kind::H1), "h2" | "h3" => Some(Kind::H2), "p" | "span" | "div" | "b" | "i" | "a" => Some(Kind::P), "li" => Some(Kind::Li), "button" => Some(Kind::Btn), _ => None };
            if let Some(k) = k { if !text.is_empty() { out.push((k, text)); } rest = &after[e + close.len()..]; continue; }
        }
        rest = after;
    }
    if out.is_empty() { out.push((Kind::Err, "Nothing to show. Try <h1>Hello</h1>".into())); }
    out
}
fn strip(s: &str) -> String {
    let mut o = String::new();
    let mut inside = false;
    for c in s.chars() { match c { '<' => inside = true, '>' => inside = false, _ if !inside => o.push(c), _ => {} } }
    o.trim().to_string()
}
