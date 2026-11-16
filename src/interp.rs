//! A small, safe interpreter for beginner Python and JavaScript snippets, plus previews for HTML,
//! CSS and SQL. Supports variables, numbers, strings, booleans, lists, arithmetic, comparisons,
//! if/elif/else, while, for (range, list, C-style, for-of), functions with return, print/console.log.
//! Every run is bounded (steps, nesting, output size, source size), so it can never hang the app.
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum V { N(f64), S(String), B(bool), L(Vec<V>), Nil }

/// Output line kinds. `Table` holds rows joined by '\n' with cells split by '\x1f' (first row = header);
/// `Styled` holds a css::Style encoding.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Kind { Out, Err, H1, H2, P, Li, Btn, Img, Table, Styled }

const MAX_STEPS: u32 = 20_000;
const MAX_DEPTH: u32 = 48;
const MAX_OUT: usize = 500;
const MAX_SRC: usize = 20_000;
const MAX_LIST: usize = 10_000;

fn num_str(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 { format!("{}", n as i64) } else { format!("{}", n) }
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

fn truthy(v: &V) -> bool {
    match v { V::N(n) => *n != 0.0, V::S(s) => !s.is_empty(), V::B(b) => *b, V::L(l) => !l.is_empty(), V::Nil => false }
}
fn num(v: &V) -> Result<f64, String> {
    match v {
        V::N(n) => Ok(*n),
        V::B(b) => Ok(*b as i32 as f64),
        V::S(s) => s.trim().parse().map_err(|_| format!("'{}' is not a number", s)),
        V::L(_) => Err("a list is not a number".into()),
        V::Nil => Err("nothing (None) is not a number".into()),
    }
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
            while t.starts_with('}') { depth = depth.saturating_sub(1); t = t[1..].trim().to_string(); }
            if t.is_empty() { continue; }
            // one-line bodies: `if (x) { a; }` or `function f(n) { return n * 2; }`
            if let Some(open) = t.find('{') {
                if t.trim_end().ends_with('}') && !t.starts_with('{') {
                    let head = t[..open].trim().to_string();
                    let body = t[open + 1..t.rfind('}').unwrap_or(t.len())].trim().to_string();
                    out.push(Line { ind: depth, text: head, no: no + 1 });
                    for stmt in body.split(';').map(|x| x.trim()).filter(|x| !x.is_empty()) { out.push(Line { ind: depth + 1, text: stmt.to_string(), no: no + 1 }); }
                    continue;
                }
            }
            if t.ends_with('{') {
                t = t[..t.len() - 1].trim().to_string();
                out.push(Line { ind: depth, text: t, no: no + 1 });
                depth += 1;
                continue;
            }
            for stmt in t.split(';').map(|x| x.trim()).filter(|x| !x.is_empty()) { out.push(Line { ind: depth, text: stmt.to_string(), no: no + 1 }); }
        } else {
            let ind = raw.len() - raw.trim_start().len();
            out.push(Line { ind: ind / 2, text: t, no: no + 1 });
        }
    }
    out
}

struct Func { params: Vec<String>, from: usize, to: usize }

struct Run<'l> { lines: &'l [Line], vars: HashMap<String, V>, funcs: HashMap<String, Func>, out: Vec<(Kind, String)>, steps: u32, js: bool, depth: u32, ret: Option<V> }

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
    fn or(&mut self) -> Result<V, String> {
        let mut l = self.and()?;
        loop {
            if self.op("||") || self.kw("or") { let r = self.and()?; l = V::B(truthy(&l) || truthy(&r)); } else { return Ok(l); }
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
        for o in ["===", "!==", "==", "!=", "<=", ">=", "<", ">"] {
            if self.op(o) {
                let r = self.add()?;
                let ord = match (&l, &r) {
                    (V::N(a), V::N(b)) => a.partial_cmp(b),
                    (V::S(a), V::S(b)) => a.partial_cmp(b),
                    _ => None,
                };
                let eq = l == r;
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
                    (V::S(s), V::N(n)) | (V::N(n), V::S(s)) => V::S(s.repeat((n.max(0.0) as usize).min(MAX_LIST))),
                    _ => V::N(num(&l)? * num(&r)?),
                };
            } else if self.op("//") { let r = self.un()?; let d = num(&r)?; if d == 0.0 { return Err("division by zero".into()); } l = V::N((num(&l)? / d).floor()); }
            else if self.op("/") { let r = self.un()?; let d = num(&r)?; if d == 0.0 { return Err("division by zero".into()); } l = V::N(num(&l)? / d); }
            else if self.op("%") { let r = self.un()?; let d = num(&r)?; if d == 0.0 { return Err("division by zero".into()); } let a = num(&l)?; l = V::N(if self.r.js { a % d } else { a - d * (a / d).floor() }); }
            else { return Ok(l); }
        }
    }
    fn un(&mut self) -> Result<V, String> {
        if self.op("-") { let v = self.un()?; return Ok(V::N(-num(&v)?)); }
        if self.op("!") { let v = self.un()?; return Ok(V::B(!truthy(&v))); }
        let mut v = self.prim()?;
        // postfix: indexing
        while self.op("[") {
            let idx = self.or()?;
            if !self.op("]") { return Err("missing ]".into()); }
            let k = num(&idx)?;
            v = match &v {
                V::L(l) => { let i = if k < 0.0 { l.len() as f64 + k } else { k }; l.get(i as usize).cloned().filter(|_| i >= 0.0 && i.fract() == 0.0).ok_or("index out of range")? }
                V::S(s) => { let ch: Vec<char> = s.chars().collect(); let i = if k < 0.0 { ch.len() as f64 + k } else { k }; ch.get(i as usize).filter(|_| i >= 0.0).map(|c| V::S(c.to_string())).ok_or("index out of range")? }
                _ => return Err("only lists and text can be indexed".into()),
            };
        }
        Ok(v)
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
            T::Op(o) if o == "[" => {
                let mut items = vec![];
                if self.op("]") { return Ok(V::L(items)); }
                loop {
                    items.push(self.or()?);
                    if self.op(",") { if self.op("]") { break; } continue; }
                    if self.op("]") { break; }
                    return Err("expected ] or ,".into());
                }
                Ok(V::L(items))
            }
            T::Id(id) => {
                if self.op("(") {
                    let a = self.args()?;
                    return self.r.call(&id, a);
                }
                if let Some(base) = id.strip_suffix(".length") {
                    return match self.r.vars.get(base) { Some(V::L(l)) => Ok(V::N(l.len() as f64)), Some(V::S(s)) => Ok(V::N(s.chars().count() as f64)), _ => Err(format!("'{}' is not defined", base)) };
                }
                match id.as_str() {
                    "True" | "true" => Ok(V::B(true)),
                    "False" | "false" => Ok(V::B(false)),
                    "None" | "null" | "undefined" => Ok(V::Nil),
                    "Math.PI" => Ok(V::N(std::f64::consts::PI)),
                    _ => self.r.vars.get(&id).cloned().ok_or(format!("'{}' is not defined", id)),
                }
            }
            T::Op(o) => Err(format!("unexpected '{}'", o)),
        }
    }
}

impl<'l> Run<'l> {
    fn show(&self, v: &V) -> String {
        match v {
            V::N(n) => num_str(*n),
            V::S(s) => s.clone(),
            V::B(b) => if self.js { b.to_string() } else if *b { "True".into() } else { "False".into() },
            V::Nil => if self.js { "undefined".into() } else { "None".into() },
            V::L(l) => {
                let items: Vec<String> = l.iter().map(|x| match x { V::S(s) => format!("'{}'", s), o => self.show(o) }).collect();
                if self.js { if items.is_empty() { "[]".into() } else { format!("[ {} ]", items.join(", ")) } } else { format!("[{}]", items.join(", ")) }
            }
        }
    }

    fn eval(&mut self, s: &str) -> Result<V, String> {
        let t = lex(s)?;
        let mut e = Ev { t, i: 0, r: self };
        let v = e.or()?;
        if e.i < e.t.len() { return Err("extra text after expression".into()); }
        Ok(v)
    }

    fn step(&mut self) -> Result<(), String> {
        self.steps += 1;
        if self.steps > MAX_STEPS { Err("stopped: too many steps (is there an endless loop?)".into()) } else { Ok(()) }
    }

    fn call(&mut self, f: &str, a: Vec<V>) -> Result<V, String> {
        if let Some(func) = self.funcs.get(f) {
            let (params, from, to) = (func.params.clone(), func.from, func.to);
            if a.len() != params.len() { return Err(format!("{}() needs {} value(s), got {}", f, params.len(), a.len())); }
            let saved = self.vars.clone();
            for (p, v) in params.iter().zip(a) { self.vars.insert(p.clone(), v); }
            let r = self.exec(from, to);
            let ret = self.ret.take().unwrap_or(V::Nil);
            // locals vanish after the call; changes to globals inside functions are not kept (beginner model)
            self.vars = saved;
            r?;
            return Ok(ret);
        }
        let first = a.first().cloned();
        let arg0 = || first.clone().ok_or_else(|| format!("{}() needs a value", f));
        let n = |i: usize| -> Result<f64, String> { num(a.get(i).ok_or("missing argument")?) };
        Ok(match f {
            "len" => match arg0()? { V::L(l) => V::N(l.len() as f64), V::S(s) => V::N(s.chars().count() as f64), _ => return Err("len() needs text or a list".into()) },
            "str" | "String" => V::S(self.show(&arg0()?)),
            "int" | "parseInt" | "Math.trunc" => V::N(n(0)?.trunc()),
            "Math.floor" => V::N(n(0)?.floor()),
            "float" | "Number" | "parseFloat" => V::N(n(0)?),
            "abs" | "Math.abs" => V::N(n(0)?.abs()),
            "round" | "Math.round" => V::N(n(0)?.round()),
            "Math.sqrt" => V::N(n(0)?.sqrt()),
            "Math.ceil" => V::N(n(0)?.ceil()),
            "max" | "Math.max" | "min" | "Math.min" => {
                let vals: Vec<f64> = match a.as_slice() { [V::L(l)] => l.iter().map(num).collect::<Result<_, _>>()?, _ => a.iter().map(num).collect::<Result<_, _>>()? };
                if vals.is_empty() { return Err(format!("{}() needs values", f)); }
                V::N(if f.ends_with("max") { vals.iter().cloned().fold(f64::MIN, f64::max) } else { vals.iter().cloned().fold(f64::MAX, f64::min) })
            }
            "sum" => match arg0()? { V::L(l) => V::N(l.iter().map(num).sum::<Result<f64, String>>()?), _ => return Err("sum() needs a list".into()) },
            "upper" => V::S(self.show(&arg0()?).to_uppercase()),
            "type" | "typeof" => V::S(match arg0()? { V::N(_) => if self.js { "number" } else { "int" }, V::S(_) => if self.js { "string" } else { "str" }, V::B(_) => if self.js { "boolean" } else { "bool" }, V::L(_) => if self.js { "object" } else { "list" }, V::Nil => "None" }.into()),
            _ => {
                if let Some((base, m)) = f.rsplit_once('.') {
                    if let Some(V::S(s)) = self.vars.get(base) {
                        return Ok(match m { "upper" | "toUpperCase" => V::S(s.to_uppercase()), "lower" | "toLowerCase" => V::S(s.to_lowercase()), "strip" | "trim" => V::S(s.trim().into()), _ => return Err(format!("unknown method '{}'", m)) });
                    }
                }
                return Err(format!("unknown function '{}'", f));
            }
        })
    }

    fn block_end(&self, i: usize, to: usize) -> usize {
        let base = self.lines[i].ind;
        let mut j = i + 1;
        while j < to && self.lines[j].ind > base { j += 1; }
        j
    }

    /// Run lines[from..to]. Ok(true) means a `return` happened.
    fn exec(&mut self, from: usize, to: usize) -> Result<bool, String> {
        self.depth += 1;
        if self.depth > MAX_DEPTH { self.depth -= 1; return Err("stopped: blocks are nested too deeply".into()); }
        let r = self.exec_inner(from, to);
        self.depth -= 1;
        r
    }

    fn exec_inner(&mut self, from: usize, to: usize) -> Result<bool, String> {
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
            if let Some(args) = t.strip_prefix("print(").or_else(|| t.strip_prefix("console.log(")) {
                let inner = args.strip_suffix(')').ok_or_else(|| e("missing )".into()))?;
                let mut parts = vec![];
                for a in split_args(inner) { let v = self.eval(&a).map_err(&e)?; parts.push(self.show(&v)); }
                if self.out.len() >= MAX_OUT { return Err("stopped: too much output".into()); }
                self.out.push((Kind::Out, parts.join(" ")));
                i += 1;
            } else if let Some(sig) = hdr(t, "def").or_else(|| hdr(t, "function")) {
                let (name, params) = sig.split_once('(').ok_or_else(|| e("a function needs ( ) after its name".into()))?;
                let params: Vec<String> = params.trim_end_matches(')').split(',').map(|p| p.trim().to_string()).filter(|p| !p.is_empty()).collect();
                if end == i + 1 { return Err(e("this function has no body".into())); }
                self.funcs.insert(name.trim().to_string(), Func { params, from: i + 1, to: end });
                i = end;
            } else if t == "return" || t.starts_with("return ") {
                let expr = t["return".len()..].trim();
                self.ret = Some(if expr.is_empty() { V::Nil } else { self.eval(expr).map_err(&e)? });
                return Ok(true);
            } else if let Some(c) = hdr(t, "if") {
                // collect the chain: if / elif / else if / else at the same indent
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
                    let go = match cond { Some(c) => { let v = self.eval(&c).map_err(&e)?; truthy(&v) } None => true };
                    if go { if self.exec(a, b)? { return Ok(true); } break; }
                }
                i = j;
            } else if let Some(c) = hdr(t, "while") {
                loop {
                    let v = self.eval(&c).map_err(&e)?;
                    if !truthy(&v) { break; }
                    if self.exec(i + 1, end)? { return Ok(true); }
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
                        for x in split_args(r) { let v = self.eval(&x).map_err(&e)?; a.push(num(&v).map_err(&e)?); }
                        let (s, en, st) = match a.len() { 0 => return Err(e("range() needs a number".into())), 1 => (0.0, a[0], 1.0), 2 => (a[0], a[1], 1.0), _ => (a[0], a[1], a[2]) };
                        if st == 0.0 { return Err(e("range() step can't be 0".into())); }
                        let count = (((en - s) / st).ceil().max(0.0) as usize).min(MAX_STEPS as usize + 1);
                        (0..count).map(|k| V::N(s + k as f64 * st)).collect()
                    } else {
                        match self.eval(rest).map_err(&e)? { V::L(l) => l, V::S(s) => s.chars().map(|c| V::S(c.to_string())).collect(), _ => return Err(e("can only loop over range(), a list or text".into())) }
                    };
                    for it in items {
                        self.vars.insert(v.trim().to_string(), it);
                        if self.exec(i + 1, end)? { return Ok(true); }
                        self.step()?;
                    }
                } else {
                    let p: Vec<&str> = c.split(';').collect();
                    if p.len() != 3 { return Err(e("unsupported for loop".into())); }
                    self.assign(p[0].trim()).map_err(&e)?;
                    loop {
                        let v = self.eval(p[1].trim()).map_err(&e)?;
                        if !truthy(&v) { break; }
                        if self.exec(i + 1, end)? { return Ok(true); }
                        self.assign(p[2].trim()).map_err(&e)?;
                        self.step()?;
                    }
                }
                i = end;
            } else {
                self.assign(t).map_err(&e)?;
                i += 1;
            }
        }
        Ok(false)
    }

    fn assign(&mut self, t: &str) -> Result<(), String> {
        let t = t.trim_end_matches(';').trim();
        let t = ["let ", "const ", "var "].iter().find_map(|k| t.strip_prefix(k)).unwrap_or(t);
        let is_name = |n: &str| !n.is_empty() && n.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '$') && !n.chars().next().unwrap().is_ascii_digit();
        if let Some(n) = t.strip_suffix("++").or_else(|| t.strip_suffix("--")) {
            let n = n.trim();
            let cur = num(self.vars.get(n).ok_or(format!("'{}' is not defined", n))?)?;
            self.vars.insert(n.into(), V::N(cur + if t.ends_with("++") { 1.0 } else { -1.0 }));
            return Ok(());
        }
        // list.append(x) / list.push(x)
        for m in [".append(", ".push("] {
            if let Some(pos) = t.find(m) {
                let n = &t[..pos];
                if is_name(n) && t.ends_with(')') {
                    let v = self.eval(&t[pos + m.len()..t.len() - 1])?;
                    match self.vars.get_mut(n) {
                        Some(V::L(l)) => { if l.len() >= MAX_LIST { return Err("list is too long".into()); } l.push(v); return Ok(()); }
                        Some(_) => return Err(format!("'{}' is not a list", n)),
                        None => return Err(format!("'{}' is not defined", n)),
                    }
                }
            }
        }
        for o in ["+=", "-=", "*=", "/="] {
            if let Some((n, r)) = t.split_once(o) {
                let n = n.trim();
                let cur = self.vars.get(n).cloned().ok_or(format!("'{}' is not defined", n))?;
                let rv = self.eval(r)?;
                let nv = match (o, &cur, &rv) {
                    ("+=", V::N(a), V::N(b)) => V::N(a + b),
                    ("+=", V::L(a), V::L(b)) => V::L(a.iter().chain(b).cloned().collect()),
                    ("+=", _, _) => V::S(format!("{}{}", self.show(&cur), self.show(&rv))),
                    ("-=", _, _) => V::N(num(&cur)? - num(&rv)?),
                    ("*=", _, _) => V::N(num(&cur)? * num(&rv)?),
                    _ => { let d = num(&rv)?; if d == 0.0 { return Err("division by zero".into()); } V::N(num(&cur)? / d) }
                };
                self.vars.insert(n.into(), nv);
                return Ok(());
            }
        }
        if let Some((n, r)) = t.split_once('=') {
            let n = n.trim();
            if !r.starts_with('=') && !n.ends_with(['!', '<', '>']) && is_name(n) {
                let v = self.eval(r)?;
                self.vars.insert(n.into(), v);
                return Ok(());
            }
        }
        // a bare expression, e.g. a function call
        if t.contains('(') { self.eval(t)?; return Ok(()); }
        Err(format!("cannot understand: {}", t))
    }
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
    let mut r = Run { lines: &lines, vars: HashMap::new(), funcs: HashMap::new(), out: vec![], steps: 0, js, depth: 0, ret: None };
    match r.exec(0, n) {
        Err(e) => r.out.push((Kind::Err, e)),
        Ok(true) => r.out.push((Kind::Err, "'return' only works inside a function".into())),
        Ok(false) => {}
    }
    if r.out.is_empty() {
        let hint = if r.funcs.is_empty() { if js { "(no output - try console.log(\"hi\"))" } else { "(no output - try print(\"hi\"))" } } else { "(function defined - call it to see a result)" };
        r.out.push((Kind::Out, hint.into()));
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
            // containers (body, header, footer, div, main, section...) holding plain text
            if !inner.contains('<') && !text.is_empty() { out.push((Kind::P, text)); rest = &after[e + close.len()..]; continue; }
        }
        match name.as_str() {
            "img" => out.push((Kind::Img, attr(&tag, "alt").or_else(|| attr(&tag, "src")).unwrap_or_else(|| "picture".into()))),
            "input" => out.push((Kind::Btn, format!("[ {} ]", attr(&tag, "placeholder").or_else(|| attr(&tag, "type")).unwrap_or_else(|| "text box".into())))),
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
                out.push((Kind::Styled, crate::css::style_of(r).encode(&format!("Sample text styled by {}", r.selector))));
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
