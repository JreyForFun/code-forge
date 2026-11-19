//! A small SQL engine for the SQL course. Each run gets its own copy of the sample database, so
//! learners can change data freely.
//!
//! Statements: SELECT [DISTINCT] ... FROM t [AS a] [[INNER|LEFT] JOIN t2 [AS b] ON ...]
//! [WHERE ...] [GROUP BY ...] [HAVING ...] [ORDER BY ... [ASC|DESC]] [LIMIT n];
//! INSERT INTO t [(cols)] VALUES (...), (...); UPDATE t SET c = v, ... [WHERE ...];
//! DELETE FROM t [WHERE ...]; CREATE TABLE t (col TYPE, ...); ALTER TABLE t ADD [COLUMN] c TYPE;
//! DROP TABLE t.
//! Expressions: + - * / %, = != <> < <= > >=, AND OR NOT, LIKE, IN (...), BETWEEN, IS [NOT] NULL,
//! COUNT/SUM/AVG/MIN/MAX (with DISTINCT), UPPER, LOWER, LENGTH, ROUND. Keywords are case-insensitive.

#[derive(Clone, Debug, PartialEq)]
pub enum Val { N(f64), T(String), Null }

impl Val {
    pub fn show(&self) -> String {
        match self {
            Val::N(n) if n.fract() == 0.0 && n.abs() < 1e15 => format!("{}", *n as i64),
            Val::N(n) => format!("{:.2}", n),
            Val::T(t) => t.clone(),
            Val::Null => "NULL".into(),
        }
    }
    fn cmp(&self, o: &Val) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        match (self, o) {
            (Val::N(a), Val::N(b)) => a.partial_cmp(b).unwrap_or(Equal),
            (Val::T(a), Val::T(b)) => a.to_lowercase().cmp(&b.to_lowercase()),
            (Val::Null, Val::Null) => Equal,
            (Val::Null, _) => Less,
            (_, Val::Null) => Greater,
            (Val::N(_), Val::T(_)) => Less,
            (Val::T(_), Val::N(_)) => Greater,
        }
    }
    fn truthy(&self) -> bool { matches!(self, Val::N(n) if *n != 0.0) }
    fn num(&self) -> Option<f64> { match self { Val::N(n) => Some(*n), Val::T(t) => t.trim().parse().ok(), Val::Null => None } }
}

fn b(x: bool) -> Val { Val::N(if x { 1.0 } else { 0.0 }) }

#[derive(Clone)]
/// `key`: the INTEGER PRIMARY KEY column, numbered automatically when an INSERT leaves it out.
pub struct Table { pub name: String, pub cols: Vec<String>, pub rows: Vec<Vec<Val>>, pub key: Option<usize> }

pub fn sample_tables() -> Vec<Table> {
    let t = |s: &str| Val::T(s.into());
    let n = |x: f64| Val::N(x);
    let cols = |c: &[&str]| c.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    vec![
        Table { key: Some(0), name: "heroes".into(), cols: cols(&["id", "name", "class", "hp", "level", "guild_id"]), rows: vec![
            vec![n(1.0), t("Aria"), t("mage"), n(8.0), n(3.0), n(1.0)],
            vec![n(2.0), t("Borin"), t("knight"), n(15.0), n(4.0), n(2.0)],
            vec![n(3.0), t("Cael"), t("rogue"), n(11.0), n(2.0), n(1.0)],
            vec![n(4.0), t("Dara"), t("knight"), n(18.0), n(5.0), n(2.0)],
            vec![n(5.0), t("Elm"), t("mage"), n(6.0), n(1.0), n(3.0)],
            vec![n(6.0), t("Finn"), t("ranger"), n(12.0), n(3.0), Val::Null],
        ]},
        Table { key: Some(0), name: "items".into(), cols: cols(&["id", "name", "type", "price"]), rows: vec![
            vec![n(1.0), t("Potion"), t("potion"), n(5.0)],
            vec![n(2.0), t("Sword"), t("weapon"), n(30.0)],
            vec![n(3.0), t("Shield"), t("armor"), n(25.0)],
            vec![n(4.0), t("Map"), t("tool"), n(8.0)],
            vec![n(5.0), t("Torch"), t("tool"), n(3.0)],
            vec![n(6.0), t("Elixir"), t("potion"), n(12.0)],
        ]},
        Table { key: Some(0), name: "guilds".into(), cols: cols(&["id", "name", "city"]), rows: vec![
            vec![n(1.0), t("Moon Circle"), t("Silverpine")],
            vec![n(2.0), t("Iron Wall"), t("Stonehold")],
            vec![n(3.0), t("Green Path"), t("Silverpine")],
        ]},
        Table { key: Some(0), name: "quests".into(), cols: cols(&["id", "title", "reward", "hero_id"]), rows: vec![
            vec![n(1.0), t("Slime Cave"), n(20.0), n(3.0)],
            vec![n(2.0), t("Dragon Peak"), n(150.0), n(4.0)],
            vec![n(3.0), t("Lost Cat"), n(5.0), n(1.0)],
            vec![n(4.0), t("Bandit Camp"), n(60.0), n(2.0)],
            vec![n(5.0), t("Haunted Mill"), n(45.0), Val::Null],
        ]},
    ]
}

#[derive(Debug, Clone, PartialEq)]
enum Tok { Word(String), Num(f64), Str(String), Sym(String) }

fn lex(src: &str) -> Result<Vec<Tok>, String> {
    let c: Vec<char> = src.chars().collect();
    let mut i = 0;
    let mut out = vec![];
    while i < c.len() {
        let ch = c[i];
        if ch.is_whitespace() { i += 1; }
        else if ch == '-' && i + 1 < c.len() && c[i + 1] == '-' { while i < c.len() && c[i] != '\n' { i += 1; } }
        else if ch.is_ascii_digit() || (ch == '.' && i + 1 < c.len() && c[i + 1].is_ascii_digit()) {
            let s = i;
            while i < c.len() && (c[i].is_ascii_digit() || c[i] == '.') { i += 1; }
            out.push(Tok::Num(c[s..i].iter().collect::<String>().parse().map_err(|_| "That number looks wrong.".to_string())?));
        } else if ch == '\'' || ch == '"' {
            let q = ch; i += 1;
            let mut v = String::new();
            loop {
                if i >= c.len() { return Err("A text value is missing its closing quote.".into()); }
                if c[i] == q { if i + 1 < c.len() && c[i + 1] == q { v.push(q); i += 2; continue; } break; }
                v.push(c[i]); i += 1;
            }
            out.push(Tok::Str(v));
            i += 1;
        } else if ch.is_alphabetic() || ch == '_' {
            let s = i;
            while i < c.len() && (c[i].is_alphanumeric() || c[i] == '_') { i += 1; }
            out.push(Tok::Word(c[s..i].iter().collect()));
        } else {
            let two: String = c[i..(i + 2).min(c.len())].iter().collect();
            if ["<=", ">=", "!=", "<>", "||"].contains(&two.as_str()) { out.push(Tok::Sym(two)); i += 2; }
            else if "*,();=<>.+-/%".contains(ch) { out.push(Tok::Sym(ch.to_string())); i += 1; }
            else { return Err(format!("SQL does not understand '{}'.", ch)); }
        }
    }
    Ok(out)
}

fn tok_text(t: &Tok) -> String {
    match t { Tok::Word(w) => w.clone(), Tok::Num(n) => Val::N(*n).show(), Tok::Str(s) => format!("'{}'", s), Tok::Sym(s) => s.clone() }
}

#[derive(Debug, Clone)]
enum E {
    Lit(Val),
    Col(Option<String>, String),
    Bin(Box<E>, String, Box<E>),
    Not(Box<E>),
    Neg(Box<E>),
    Like(Box<E>, Box<E>, bool),
    In(Box<E>, Vec<E>, bool),
    Between(Box<E>, Box<E>, Box<E>, bool),
    IsNull(Box<E>, bool),
    Agg(String, Option<Box<E>>, bool),
    Func(String, Vec<E>),
}

impl E {
    fn has_agg(&self) -> bool {
        match self {
            E::Agg(..) => true,
            E::Lit(_) | E::Col(..) => false,
            E::Bin(a, _, c) => a.has_agg() || c.has_agg(),
            E::Not(a) | E::Neg(a) | E::IsNull(a, _) => a.has_agg(),
            E::Like(a, c, _) => a.has_agg() || c.has_agg(),
            E::In(a, l, _) => a.has_agg() || l.iter().any(|x| x.has_agg()),
            E::Between(a, l, h, _) => a.has_agg() || l.has_agg() || h.has_agg(),
            E::Func(_, a) => a.iter().any(|x| x.has_agg()),
        }
    }
}

const AGGS: [&str; 5] = ["COUNT", "SUM", "MIN", "MAX", "AVG"];
const FUNCS: [&str; 4] = ["UPPER", "LOWER", "LENGTH", "ROUND"];
const RESERVED: [&str; 24] = ["SELECT", "FROM", "WHERE", "GROUP", "ORDER", "BY", "LIMIT", "ASC", "DESC", "AND", "OR", "NOT", "HAVING", "JOIN", "INNER", "LEFT", "ON", "AS", "LIKE", "IN", "BETWEEN", "IS", "NULL", "DISTINCT"];

struct P { t: Vec<Tok>, i: usize }
impl P {
    fn peek(&self) -> Option<&Tok> { self.t.get(self.i) }
    fn peek_kw(&self, k: &str) -> bool { matches!(self.peek(), Some(Tok::Word(w)) if w.eq_ignore_ascii_case(k)) }
    fn kw(&mut self, k: &str) -> bool { if self.peek_kw(k) { self.i += 1; true } else { false } }
    fn sym(&mut self, s: &str) -> bool {
        if let Some(Tok::Sym(x)) = self.peek() && x == s { self.i += 1; return true; }
        false
    }
    fn need_kw(&mut self, k: &str, msg: &str) -> Result<(), String> { if self.kw(k) { Ok(()) } else { Err(msg.into()) } }
    fn need_sym(&mut self, s: &str, msg: &str) -> Result<(), String> { if self.sym(s) { Ok(()) } else { Err(msg.into()) } }
    fn name(&mut self, what: &str) -> Result<String, String> {
        match self.peek().cloned() {
            Some(Tok::Word(w)) if !RESERVED.contains(&w.to_uppercase().as_str()) => { self.i += 1; Ok(w) }
            Some(Tok::Word(w)) => Err(format!("Expected {} before {}.", what, w.to_uppercase())),
            _ => Err(format!("Expected {} here.", what)),
        }
    }
    /// Optional table alias: `heroes AS h` or `heroes h`; otherwise the table's own name.
    fn alias(&mut self, table: &str) -> Result<String, String> {
        let bare = matches!(self.peek(), Some(Tok::Word(w)) if !RESERVED.contains(&w.to_uppercase().as_str()));
        if self.kw("AS") || bare { self.name("a short name") } else { Ok(table.to_string()) }
    }
    fn text_since(&self, start: usize) -> String {
        let mut s = String::new();
        for (k, t) in self.t[start..self.i].iter().enumerate() {
            let tt = tok_text(t);
            let prev = if k > 0 { Some(&self.t[start + k - 1]) } else { None };
            let tight = matches!(t, Tok::Sym(x) if x == "(" || x == ")" || x == "." || x == ",") || matches!(prev, Some(Tok::Sym(x)) if x == "(" || x == ".");
            if !s.is_empty() && !tight { s.push(' '); }
            s.push_str(&tt);
        }
        s
    }

    // expression grammar: or > and > not > comparison > add > mul > unary > primary
    fn expr(&mut self) -> Result<E, String> {
        let mut l = self.and()?;
        while self.kw("OR") { let r = self.and()?; l = E::Bin(Box::new(l), "OR".into(), Box::new(r)); }
        Ok(l)
    }
    fn and(&mut self) -> Result<E, String> {
        let mut l = self.not()?;
        while self.kw("AND") { let r = self.not()?; l = E::Bin(Box::new(l), "AND".into(), Box::new(r)); }
        Ok(l)
    }
    fn not(&mut self) -> Result<E, String> {
        if self.kw("NOT") { return Ok(E::Not(Box::new(self.not()?))); }
        self.cmp()
    }
    fn cmp(&mut self) -> Result<E, String> {
        let l = self.add()?;
        if let Some(Tok::Sym(s)) = self.peek().cloned() && ["=", "!=", "<>", "<", "<=", ">", ">="].contains(&s.as_str()) {
            self.i += 1;
            let r = self.add()?;
            return Ok(E::Bin(Box::new(l), s, Box::new(r)));
        }
        if self.kw("IS") {
            let neg = self.kw("NOT");
            self.need_kw("NULL", "Write IS NULL or IS NOT NULL.")?;
            return Ok(E::IsNull(Box::new(l), neg));
        }
        let neg = self.kw("NOT");
        if self.kw("LIKE") { let r = self.add()?; return Ok(E::Like(Box::new(l), Box::new(r), neg)); }
        if self.kw("IN") {
            self.need_sym("(", "IN needs a list in brackets, like IN ('mage', 'rogue').")?;
            let mut items = vec![self.expr()?];
            while self.sym(",") { items.push(self.expr()?); }
            self.need_sym(")", "The IN list needs a closing ).")?;
            return Ok(E::In(Box::new(l), items, neg));
        }
        if self.kw("BETWEEN") {
            let lo = self.add()?;
            self.need_kw("AND", "Write BETWEEN low AND high.")?;
            let hi = self.add()?;
            return Ok(E::Between(Box::new(l), Box::new(lo), Box::new(hi), neg));
        }
        if neg { return Err("NOT here must be followed by LIKE, IN or BETWEEN.".into()); }
        Ok(l)
    }
    fn add(&mut self) -> Result<E, String> {
        let mut l = self.mul()?;
        loop {
            let op = if self.sym("+") { "+" } else if self.sym("-") { "-" } else if self.sym("||") { "||" } else { return Ok(l) };
            let r = self.mul()?;
            l = E::Bin(Box::new(l), op.into(), Box::new(r));
        }
    }
    fn mul(&mut self) -> Result<E, String> {
        let mut l = self.unary()?;
        loop {
            let op = if self.sym("*") { "*" } else if self.sym("/") { "/" } else if self.sym("%") { "%" } else { return Ok(l) };
            let r = self.unary()?;
            l = E::Bin(Box::new(l), op.into(), Box::new(r));
        }
    }
    fn unary(&mut self) -> Result<E, String> {
        if self.sym("-") { return Ok(E::Neg(Box::new(self.unary()?))); }
        self.primary()
    }
    fn primary(&mut self) -> Result<E, String> {
        match self.peek().cloned() {
            Some(Tok::Num(n)) => { self.i += 1; Ok(E::Lit(Val::N(n))) }
            Some(Tok::Str(s)) => { self.i += 1; Ok(E::Lit(Val::T(s))) }
            Some(Tok::Sym(s)) if s == "(" => { self.i += 1; let e = self.expr()?; self.need_sym(")", "A ( is missing its closing ).")?; Ok(e) }
            Some(Tok::Word(w)) => {
                let up = w.to_uppercase();
                if up == "NULL" { self.i += 1; return Ok(E::Lit(Val::Null)); }
                if up == "TRUE" || up == "FALSE" { self.i += 1; return Ok(E::Lit(b(up == "TRUE"))); }
                if AGGS.contains(&up.as_str()) && matches!(self.t.get(self.i + 1), Some(Tok::Sym(x)) if x == "(") {
                    self.i += 2;
                    let distinct = self.kw("DISTINCT");
                    let arg = if self.sym("*") { if up != "COUNT" { return Err(format!("{}(*) is not allowed; name a column.", up)); } None } else { Some(Box::new(self.expr()?)) };
                    self.need_sym(")", &format!("{}( needs a closing ).", up))?;
                    return Ok(E::Agg(up, arg, distinct));
                }
                if FUNCS.contains(&up.as_str()) && matches!(self.t.get(self.i + 1), Some(Tok::Sym(x)) if x == "(") {
                    self.i += 2;
                    let mut args = vec![self.expr()?];
                    while self.sym(",") { args.push(self.expr()?); }
                    self.need_sym(")", &format!("{}( needs a closing ).", up))?;
                    return Ok(E::Func(up, args));
                }
                if RESERVED.contains(&up.as_str()) { return Err(format!("Expected a value or column before {}.", up)); }
                self.i += 1;
                if self.sym(".") { let c = self.name("a column name after the dot")?; return Ok(E::Col(Some(w), c)); }
                Ok(E::Col(None, w))
            }
            Some(t) => Err(format!("Unexpected '{}'.", tok_text(&t))),
            None => Err("The statement ends too early.".into()),
        }
    }
}

/// A result row, the source row it came from, and its group (for ORDER BY on unselected columns).
type OutRow = (Vec<Val>, Vec<Val>, Vec<Vec<Val>>);

/// Column metadata for a (possibly joined) row: (table or alias, column name).
type Cols = Vec<(String, String)>;

fn find_col(cols: &Cols, q: &Option<String>, name: &str) -> Result<usize, String> {
    let hits: Vec<usize> = cols.iter().enumerate().filter(|(_, (t, c))| c.eq_ignore_ascii_case(name) && q.as_ref().is_none_or(|q| t.eq_ignore_ascii_case(q))).map(|(i, _)| i).collect();
    match hits.len() {
        1 => Ok(hits[0]),
        0 => {
            let list: Vec<String> = cols.iter().map(|(_, c)| c.clone()).collect();
            Err(match q { Some(q) => format!("No column '{}.{}'. Columns: {}.", q, name, list.join(", ")), None => format!("No column '{}'. Columns: {}. (If '{}' is a text value, put it in quotes: '{}'.)", name, list.join(", "), name, name) })
        }
        _ => Err(format!("'{}' is in more than one table; write it as table.{}.", name, name)),
    }
}

fn like(text: &str, pat: &str) -> bool {
    let t: Vec<char> = text.to_lowercase().chars().collect();
    let p: Vec<char> = pat.to_lowercase().chars().collect();
    fn go(t: &[char], p: &[char]) -> bool {
        match p.first() {
            None => t.is_empty(),
            Some('%') => (0..=t.len()).any(|k| go(&t[k..], &p[1..])),
            Some('_') => !t.is_empty() && go(&t[1..], &p[1..]),
            Some(c) => t.first() == Some(c) && go(&t[1..], &p[1..]),
        }
    }
    go(&t, &p)
}

/// Evaluate `e` for one row, or for a group of rows when aggregates are involved.
fn eval(e: &E, cols: &Cols, row: &[Val], group: &[&Vec<Val>]) -> Result<Val, String> {
    let ev = |x: &E| eval(x, cols, row, group);
    Ok(match e {
        E::Lit(v) => v.clone(),
        E::Col(q, n) => row.get(find_col(cols, q, n)?).cloned().unwrap_or(Val::Null),
        E::Neg(a) => match ev(a)?.num() { Some(n) => Val::N(-n), None => Val::Null },
        E::Not(a) => { let v = ev(a)?; if v == Val::Null { Val::Null } else { b(!v.truthy()) } }
        E::IsNull(a, neg) => b((ev(a)? == Val::Null) != *neg),
        E::Like(a, p, neg) => match (ev(a)?, ev(p)?) { (Val::Null, _) | (_, Val::Null) => Val::Null, (x, y) => b(like(&x.show(), &y.show()) != *neg) },
        E::In(a, list, neg) => { let v = ev(a)?; if v == Val::Null { Val::Null } else { let mut hit = false; for x in list { if ev(x)?.cmp(&v).is_eq() { hit = true; } } b(hit != *neg) } }
        E::Between(a, lo, hi, neg) => { let (v, l, h) = (ev(a)?, ev(lo)?, ev(hi)?); if v == Val::Null { Val::Null } else { b((v.cmp(&l).is_ge() && v.cmp(&h).is_le()) != *neg) } }
        E::Bin(l, op, r) => {
            if op == "AND" { let a = ev(l)?; if a != Val::Null && !a.truthy() { return Ok(b(false)); } let c = ev(r)?; return Ok(if a == Val::Null || c == Val::Null { if c != Val::Null && !c.truthy() { b(false) } else { Val::Null } } else { b(c.truthy()) }); }
            if op == "OR" { let a = ev(l)?; if a.truthy() { return Ok(b(true)); } let c = ev(r)?; return Ok(if c.truthy() { b(true) } else if a == Val::Null || c == Val::Null { Val::Null } else { b(false) }); }
            let (a, c) = (ev(l)?, ev(r)?);
            if a == Val::Null || c == Val::Null { return Ok(Val::Null); }
            match op.as_str() {
                "=" => b(a.cmp(&c).is_eq()), "!=" | "<>" => b(a.cmp(&c).is_ne()),
                "<" => b(a.cmp(&c).is_lt()), "<=" => b(a.cmp(&c).is_le()), ">" => b(a.cmp(&c).is_gt()), ">=" => b(a.cmp(&c).is_ge()),
                "||" => Val::T(format!("{}{}", a.show(), c.show())),
                _ => {
                    let (Some(x), Some(y)) = (a.num(), c.num()) else { return Err(format!("{} needs numbers.", op)) };
                    match op.as_str() {
                        "+" => Val::N(x + y), "-" => Val::N(x - y), "*" => Val::N(x * y),
                        "/" => if y == 0.0 { Val::Null } else { Val::N(x / y) },
                        _ => if y == 0.0 { Val::Null } else { Val::N(x % y) },
                    }
                }
            }
        }
        E::Func(f, args) => {
            let v = ev(&args[0])?;
            if v == Val::Null { return Ok(Val::Null); }
            match f.as_str() {
                "UPPER" => Val::T(v.show().to_uppercase()),
                "LOWER" => Val::T(v.show().to_lowercase()),
                "LENGTH" => Val::N(v.show().chars().count() as f64),
                _ => { let d = match args.get(1) { Some(x) => ev(x)?.num().unwrap_or(0.0), None => 0.0 }; let p = 10f64.powi(d as i32); Val::N((v.num().unwrap_or(0.0) * p).round() / p) }
            }
        }
        E::Agg(f, arg, distinct) => {
            let mut vals: Vec<Val> = vec![];
            for r in group {
                let v = match arg { Some(a) => eval(a, cols, r, &[])?, None => Val::N(1.0) };
                if v != Val::Null && !(*distinct && vals.iter().any(|x| x.cmp(&v).is_eq())) { vals.push(v); }
            }
            let nums: Vec<f64> = vals.iter().filter_map(|v| v.num()).collect();
            match f.as_str() {
                "COUNT" => Val::N(vals.len() as f64),
                "SUM" => if nums.is_empty() { Val::Null } else { Val::N(nums.iter().sum()) },
                "AVG" => if nums.is_empty() { Val::Null } else { Val::N(nums.iter().sum::<f64>() / nums.len() as f64) },
                "MIN" => vals.iter().min_by(|a, b| a.cmp(b)).cloned().unwrap_or(Val::Null),
                _ => vals.iter().max_by(|a, b| a.cmp(b)).cloned().unwrap_or(Val::Null),
            }
        }
    })
}

/// What a statement produced: rows to show (header first) or a message.
pub enum Outcome { Rows(Vec<Vec<String>>), Done(String) }

/// A database for one run: starts as the sample tables; statements may change it.
pub struct Db { pub tables: Vec<Table> }

impl Default for Db { fn default() -> Self { Db { tables: sample_tables() } } }

impl Db {
    fn table_idx(&self, name: &str) -> Result<usize, String> {
        self.tables.iter().position(|t| t.name.eq_ignore_ascii_case(name)).ok_or_else(|| {
            let names: Vec<&str> = self.tables.iter().map(|t| t.name.as_str()).collect();
            format!("No table named '{}'. Tables: {}.", name, names.join(", "))
        })
    }

    pub fn exec(&mut self, src: &str) -> Result<Outcome, String> {
        let toks = lex(src)?;
        if toks.is_empty() { return Err("Type a query, for example: SELECT * FROM heroes;".into()); }
        let mut p = P { t: toks, i: 0 };
        let out = if p.peek_kw("SELECT") { Outcome::Rows(self.select(&mut p)?) }
            else if p.kw("INSERT") { self.insert(&mut p)? }
            else if p.kw("UPDATE") { self.update(&mut p)? }
            else if p.kw("DELETE") { self.delete(&mut p)? }
            else if p.kw("CREATE") { self.create(&mut p)? }
            else if p.kw("ALTER") { self.alter(&mut p)? }
            else if p.kw("DROP") { p.need_kw("TABLE", "Write DROP TABLE name.")?; let n = p.name("a table name")?; let i = self.table_idx(&n)?; self.tables.remove(i); Outcome::Done(format!("Table {} dropped", n)) }
            else { return Err(format!("SQL statements start with SELECT, INSERT, UPDATE, DELETE or CREATE, not '{}'.", tok_text(&p.t[0]))); };
        p.sym(";");
        if let Some(t) = p.peek() {
            return Err(format!("Unexpected '{}' at the end. Check the order: FROM, JOIN, WHERE, GROUP BY, HAVING, ORDER BY, LIMIT.", tok_text(t)));
        }
        Ok(out)
    }

    fn select(&mut self, p: &mut P) -> Result<Vec<Vec<String>>, String> {
        p.need_kw("SELECT", "A query starts with SELECT.")?;
        let distinct = p.kw("DISTINCT");
        // select list: (expr or *, header text)
        let mut items: Vec<(Option<E>, String)> = vec![];
        loop {
            if p.sym("*") { items.push((None, "*".into())); }
            else {
                let st = p.i;
                let e = p.expr()?;
                let mut label = p.text_since(st);
                if let E::Col(_, c) = &e { label = c.clone(); }
                if p.kw("AS") { label = match p.peek().cloned() { Some(Tok::Str(s)) => { p.i += 1; s } _ => p.name("a name after AS")? }; }
                items.push((Some(e), label));
            }
            if !p.sym(",") { break; }
        }
        p.need_kw("FROM", "Missing FROM <table>.")?;
        let t1 = p.name("a table name")?;
        let a1 = p.alias(&t1)?;
        let base = self.tables[self.table_idx(&t1)?].clone();
        let mut cols: Cols = base.cols.iter().map(|c| (a1.clone(), c.clone())).collect();
        let mut rows: Vec<Vec<Val>> = base.rows.clone();
        // joins
        loop {
            let left = if p.kw("LEFT") { p.kw("OUTER"); true } else { p.kw("INNER"); false };
            if !p.kw("JOIN") { if left { return Err("Write LEFT JOIN table ON ...".into()); } break; }
            let t2 = p.name("a table name after JOIN")?;
            let a2 = p.alias(&t2)?;
            let other = self.tables[self.table_idx(&t2)?].clone();
            p.need_kw("ON", "A JOIN needs ON, for example ON heroes.guild_id = guilds.id.")?;
            let on = p.expr()?;
            let mut jcols = cols.clone();
            jcols.extend(other.cols.iter().map(|c| (a2.clone(), c.clone())));
            let mut joined = vec![];
            for r in &rows {
                let mut matched = false;
                for o in &other.rows {
                    let mut jr = r.clone();
                    jr.extend(o.iter().cloned());
                    if eval(&on, &jcols, &jr, &[])?.truthy() { joined.push(jr); matched = true; }
                }
                if left && !matched { let mut jr = r.clone(); jr.extend(std::iter::repeat_n(Val::Null, other.cols.len())); joined.push(jr); }
            }
            cols = jcols;
            rows = joined;
        }
        if p.kw("WHERE") {
            let w = p.expr()?;
            if w.has_agg() { return Err("Totals like COUNT() can't go in WHERE; use HAVING after GROUP BY.".into()); }
            let mut keep = vec![];
            for r in rows { if eval(&w, &cols, &r, &[])?.truthy() { keep.push(r); } }
            rows = keep;
        }
        let mut group_by: Vec<E> = vec![];
        if p.kw("GROUP") {
            p.need_kw("BY", "Write GROUP BY <column>.")?;
            loop { group_by.push(p.expr()?); if !p.sym(",") { break; } }
        }
        let having = if p.kw("HAVING") { Some(p.expr()?) } else { None };
        let mut order: Vec<(E, bool)> = vec![];
        if p.kw("ORDER") {
            p.need_kw("BY", "Write ORDER BY <column>.")?;
            loop {
                let e = p.expr()?;
                let desc = if p.kw("DESC") { true } else { p.kw("ASC"); false };
                order.push((e, desc));
                if !p.sym(",") { break; }
            }
        }
        let mut limit = None;
        if p.kw("LIMIT") {
            match p.peek().cloned() { Some(Tok::Num(n)) => { p.i += 1; limit = Some(n as usize); } _ => return Err("LIMIT needs a number.".into()) }
        }

        // header
        let mut header = vec![];
        for (e, label) in &items {
            if e.is_none() { header.extend(cols.iter().map(|(_, c)| c.clone())); } else { header.push(label.clone()); }
        }
        let aggregate = !group_by.is_empty() || having.is_some() || items.iter().any(|(e, _)| e.as_ref().is_some_and(|e| e.has_agg()));
        // each output row keeps its source row(s) for ORDER BY on columns that are not selected
        let mut out: Vec<OutRow> = vec![];
        if aggregate {
            if items.iter().any(|(e, _)| e.is_none()) { return Err("SELECT * can't be mixed with totals; name the columns.".into()); }
            let gtext: Vec<String> = group_by.iter().map(|g| format!("{:?}", g)).collect();
            for (e, label) in &items {
                let e = e.as_ref().unwrap();
                if !e.has_agg() && !gtext.contains(&format!("{:?}", e)) && !matches!(e, E::Lit(_)) {
                    return Err(format!("'{}' must be in GROUP BY to show next to a total.", label));
                }
            }
            let mut groups: Vec<(Vec<Val>, Vec<Vec<Val>>)> = vec![];
            for r in rows {
                let mut key = vec![];
                for g in &group_by { key.push(eval(g, &cols, &r, &[])?); }
                match groups.iter_mut().find(|(k, _)| k.len() == key.len() && k.iter().zip(&key).all(|(a, b)| a.cmp(b).is_eq())) {
                    Some(g) => g.1.push(r),
                    None => groups.push((key, vec![r])),
                }
            }
            if groups.is_empty() && group_by.is_empty() { groups.push((vec![], vec![])); }
            for (_, rs) in &groups {
                let refs: Vec<&Vec<Val>> = rs.iter().collect();
                let first = rs.first().cloned().unwrap_or_else(|| vec![Val::Null; cols.len()]);
                if let Some(h) = &having && !eval(h, &cols, &first, &refs)?.truthy() { continue; }
                let mut line = vec![];
                for (e, _) in &items { line.push(eval(e.as_ref().unwrap(), &cols, &first, &refs)?); }
                out.push((line, first, rs.clone()));
            }
        } else {
            for r in rows {
                let mut line = vec![];
                for (e, _) in &items {
                    match e { None => line.extend(r.iter().cloned()), Some(e) => line.push(eval(e, &cols, &r, &[])?) }
                }
                out.push((line, r, vec![]));
            }
        }
        if distinct {
            let mut seen: Vec<Vec<Val>> = vec![];
            out.retain(|(l, _, _)| { if seen.iter().any(|s| s.iter().zip(l).all(|(a, b)| a.cmp(b).is_eq())) { false } else { seen.push(l.clone()); true } });
        }
        if !order.is_empty() {
            // sort keys: an output column (by alias/label or position) or any expression on the source row
            let mut keyed = vec![];
            for (line, src, grp) in out {
                let mut keys = vec![];
                for (e, _) in &order {
                    let by_label = if let E::Col(None, n) = e { header.iter().position(|h| h.eq_ignore_ascii_case(n)) } else { None };
                    let k = match (e, by_label) {
                        (E::Lit(Val::N(n)), _) if *n >= 1.0 && (*n as usize) <= line.len() => line[*n as usize - 1].clone(),
                        (_, Some(i)) if aggregate || find_col(&cols, &None, &header[i]).is_err() => line[i].clone(),
                        _ => { let refs: Vec<&Vec<Val>> = grp.iter().collect(); eval(e, &cols, &src, &refs)? }
                    };
                    keys.push(k);
                }
                keyed.push((keys, line));
            }
            keyed.sort_by(|a, b| {
                for (k, (_, desc)) in order.iter().enumerate() {
                    let o = a.0[k].cmp(&b.0[k]);
                    let o = if *desc { o.reverse() } else { o };
                    if o.is_ne() { return o; }
                }
                std::cmp::Ordering::Equal
            });
            out = keyed.into_iter().map(|(_, l)| (l, vec![], vec![])).collect();
        }
        if let Some(n) = limit { out.truncate(n); }
        let mut res = vec![header];
        res.extend(out.iter().map(|(l, _, _)| l.iter().map(|v| v.show()).collect()));
        Ok(res)
    }

    fn insert(&mut self, p: &mut P) -> Result<Outcome, String> {
        p.need_kw("INTO", "Write INSERT INTO table ...")?;
        let tn = p.name("a table name")?;
        let ti = self.table_idx(&tn)?;
        let mut targets: Vec<usize> = (0..self.tables[ti].cols.len()).collect();
        if p.sym("(") {
            targets.clear();
            loop {
                let c = p.name("a column name")?;
                let k = self.tables[ti].cols.iter().position(|x| x.eq_ignore_ascii_case(&c)).ok_or_else(|| format!("Table {} has no column '{}'.", tn, c))?;
                targets.push(k);
                if !p.sym(",") { break; }
            }
            p.need_sym(")", "The column list needs a closing ).")?;
        }
        p.need_kw("VALUES", "Write VALUES (...) after the table name.")?;
        let mut n = 0;
        loop {
            p.need_sym("(", "Each row of VALUES goes in brackets.")?;
            let mut vals = vec![];
            loop { vals.push(eval(&p.expr()?, &vec![], &[], &[])?); if !p.sym(",") { break; } }
            p.need_sym(")", "A VALUES row needs a closing ).")?;
            if vals.len() != targets.len() { return Err(format!("{} values for {} columns.", vals.len(), targets.len())); }
            let mut row = vec![Val::Null; self.tables[ti].cols.len()];
            for (k, v) in targets.iter().zip(vals) { row[*k] = v; }
            // a key left out gets the next number, as in real databases
            if let Some(k) = self.tables[ti].key && row[k] == Val::Null {
                let next = self.tables[ti].rows.iter().filter_map(|r| r[k].num()).fold(0.0, f64::max) + 1.0;
                row[k] = Val::N(next);
            }
            self.tables[ti].rows.push(row);
            n += 1;
            if !p.sym(",") { break; }
        }
        Ok(Outcome::Done(format!("{} row{} inserted into {}", n, if n == 1 { "" } else { "s" }, tn)))
    }

    fn where_mask(&self, p: &mut P, ti: usize) -> Result<Vec<bool>, String> {
        let t = &self.tables[ti];
        if !p.kw("WHERE") { return Ok(vec![true; t.rows.len()]); }
        let w = p.expr()?;
        let cols: Cols = t.cols.iter().map(|c| (t.name.clone(), c.clone())).collect();
        t.rows.iter().map(|r| eval(&w, &cols, r, &[]).map(|v| v.truthy())).collect()
    }

    fn update(&mut self, p: &mut P) -> Result<Outcome, String> {
        let tn = p.name("a table name")?;
        let ti = self.table_idx(&tn)?;
        p.need_kw("SET", "Write UPDATE table SET column = value.")?;
        let mut sets = vec![];
        loop {
            let c = p.name("a column name")?;
            let k = self.tables[ti].cols.iter().position(|x| x.eq_ignore_ascii_case(&c)).ok_or_else(|| format!("Table {} has no column '{}'.", tn, c))?;
            p.need_sym("=", "SET needs column = value.")?;
            sets.push((k, p.expr()?));
            if !p.sym(",") { break; }
        }
        let mask = self.where_mask(p, ti)?;
        let t = &self.tables[ti];
        let cols: Cols = t.cols.iter().map(|c| (t.name.clone(), c.clone())).collect();
        let mut changed = vec![];
        for (r, m) in t.rows.iter().zip(&mask) {
            if !*m { changed.push(None); continue; }
            let mut nr = r.clone();
            for (k, e) in &sets { nr[*k] = eval(e, &cols, r, &[])?; }
            changed.push(Some(nr));
        }
        let n = changed.iter().filter(|c| c.is_some()).count();
        for (r, c) in self.tables[ti].rows.iter_mut().zip(changed) { if let Some(c) = c { *r = c; } }
        Ok(Outcome::Done(format!("{} row{} updated in {}", n, if n == 1 { "" } else { "s" }, tn)))
    }

    fn delete(&mut self, p: &mut P) -> Result<Outcome, String> {
        p.need_kw("FROM", "Write DELETE FROM table.")?;
        let tn = p.name("a table name")?;
        let ti = self.table_idx(&tn)?;
        let mask = self.where_mask(p, ti)?;
        let before = self.tables[ti].rows.len();
        let mut it = mask.iter();
        self.tables[ti].rows.retain(|_| !*it.next().unwrap_or(&false));
        let n = before - self.tables[ti].rows.len();
        Ok(Outcome::Done(format!("{} row{} deleted from {}", n, if n == 1 { "" } else { "s" }, tn)))
    }

    /// Skip a column's type and constraints (INTEGER, TEXT, VARCHAR(20), PRIMARY KEY, NOT NULL...).
    /// Returns true when the column is a PRIMARY KEY.
    fn skip_type(p: &mut P) -> bool {
        let mut depth = 0;
        let mut key = false;
        while let Some(t) = p.peek() {
            if tok_text(t).eq_ignore_ascii_case("PRIMARY") { key = true; }
            match t { Tok::Sym(s) if s == "(" => depth += 1, Tok::Sym(s) if s == ")" => { if depth == 0 { break; } depth -= 1; } Tok::Sym(s) if s == "," && depth == 0 => break, Tok::Sym(s) if s == ";" => break, _ => {} }
            p.i += 1;
        }
        key
    }

    fn create(&mut self, p: &mut P) -> Result<Outcome, String> {
        p.need_kw("TABLE", "Write CREATE TABLE name (columns).")?;
        let tn = p.name("a table name")?;
        if self.table_idx(&tn).is_ok() { return Err(format!("A table named {} already exists.", tn)); }
        p.need_sym("(", "List the columns in brackets: CREATE TABLE pets (name TEXT, age INTEGER).")?;
        let mut cols = vec![];
        let mut key = None;
        loop {
            cols.push(p.name("a column name")?);
            if Self::skip_type(p) && key.is_none() { key = Some(cols.len() - 1); }
            if !p.sym(",") { break; }
        }
        p.need_sym(")", "The column list needs a closing ).")?;
        self.tables.push(Table { name: tn.clone(), cols: cols.clone(), rows: vec![], key });
        Ok(Outcome::Done(format!("Table {} created with columns {}", tn, cols.join(", "))))
    }

    fn alter(&mut self, p: &mut P) -> Result<Outcome, String> {
        p.need_kw("TABLE", "Write ALTER TABLE name ADD column TYPE.")?;
        let tn = p.name("a table name")?;
        let ti = self.table_idx(&tn)?;
        p.need_kw("ADD", "Write ALTER TABLE name ADD column TYPE.")?;
        p.kw("COLUMN");
        let c = p.name("a column name")?;
        let _ = Self::skip_type(p);
        let t = &mut self.tables[ti];
        if t.cols.iter().any(|x| x.eq_ignore_ascii_case(&c)) { return Err(format!("{} already has a column {}.", tn, c)); }
        t.cols.push(c.clone());
        for r in &mut t.rows { r.push(Val::Null); }
        Ok(Outcome::Done(format!("Column {} added to {}", c, tn)))
    }
}

#[cfg_attr(not(test), allow(dead_code))]
/// Run one SELECT on a fresh copy of the sample database. Ok(rows) where rows[0] is the header.
pub fn query(src: &str) -> Result<Vec<Vec<String>>, String> {
    match Db::default().exec(src)? {
        Outcome::Rows(r) => Ok(r),
        Outcome::Done(m) => Err(format!("That statement changes data ({}); queries start with SELECT.", m)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn col0(q: &str) -> Vec<String> { query(q).unwrap().iter().skip(1).map(|r| r[0].clone()).collect() }
    fn run_all(src: &str) -> Vec<String> {
        let mut db = Db::default();
        let mut out = vec![];
        for s in src.split(';').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            match db.exec(s) { Ok(Outcome::Rows(r)) => out.push(r.iter().skip(1).map(|l| l.join("|")).collect::<Vec<_>>().join(",")), Ok(Outcome::Done(m)) => out.push(m), Err(e) => { out.push(format!("!{}", e)); break; } }
        }
        out
    }

    #[test]
    fn select_star_and_columns() {
        let r = query("SELECT * FROM heroes;").unwrap();
        assert_eq!(r[0], vec!["id", "name", "class", "hp", "level", "guild_id"]);
        assert_eq!(r.len(), 7);
        let r = query("select name, hp from HEROES").unwrap();
        assert_eq!(r[0], vec!["name", "hp"]);
        assert_eq!(r[1], vec!["Aria", "8"]);
        assert_eq!(query("SELECT name, price * 2 AS double_price FROM items LIMIT 1").unwrap()[1], vec!["Potion", "10"]);
        assert_eq!(query("SELECT name, price * 2 AS double_price FROM items LIMIT 1").unwrap()[0][1], "double_price");
        assert_eq!(col0("SELECT DISTINCT class FROM heroes ORDER BY class"), vec!["knight", "mage", "ranger", "rogue"]);
    }

    #[test]
    fn where_filters() {
        assert_eq!(col0("SELECT name FROM heroes WHERE hp > 10;"), vec!["Borin", "Cael", "Dara", "Finn"]);
        assert_eq!(col0("SELECT name FROM heroes WHERE class = 'mage';"), vec!["Aria", "Elm"]);
        assert!(query("SELECT * FROM heroes WHERE class = mage").unwrap_err().contains("mage"));
        assert_eq!(col0("SELECT name FROM heroes WHERE class = 'mage' AND hp > 7;"), vec!["Aria"]);
        assert_eq!(col0("SELECT name FROM heroes WHERE class = 'rogue' OR hp > 15;"), vec!["Cael", "Dara"]);
        assert_eq!(col0("SELECT name FROM heroes WHERE name LIKE 'D%'"), vec!["Dara"]);
        assert_eq!(col0("SELECT name FROM heroes WHERE name LIKE '_l%'"), vec!["Elm"]);
        assert_eq!(col0("SELECT name FROM heroes WHERE class IN ('mage', 'rogue')"), vec!["Aria", "Cael", "Elm"]);
        assert_eq!(col0("SELECT name FROM heroes WHERE hp BETWEEN 10 AND 15"), vec!["Borin", "Cael", "Finn"]);
        assert_eq!(col0("SELECT name FROM heroes WHERE guild_id IS NULL"), vec!["Finn"]);
        assert_eq!(col0("SELECT name FROM heroes WHERE NOT class = 'knight' AND (hp > 10 OR level = 1)"), vec!["Cael", "Elm", "Finn"]);
    }

    #[test]
    fn order_and_limit() {
        assert_eq!(col0("SELECT name FROM heroes ORDER BY hp DESC LIMIT 1;"), vec!["Dara"]);
        assert_eq!(col0("SELECT name FROM heroes ORDER BY class, hp DESC"), vec!["Dara", "Borin", "Aria", "Elm", "Finn", "Cael"]);
        assert_eq!(col0("SELECT * FROM heroes LIMIT 3;").len(), 3);
    }

    #[test]
    fn aggregates_groups_having() {
        assert_eq!(query("SELECT COUNT(*) FROM heroes;").unwrap()[1], vec!["6"]);
        assert_eq!(query("SELECT COUNT(guild_id) FROM heroes;").unwrap()[1], vec!["5"]);
        assert_eq!(query("SELECT AVG(price) FROM items").unwrap()[1], vec!["13.83"]);
        assert_eq!(query("SELECT COUNT(DISTINCT class) FROM heroes").unwrap()[1], vec!["4"]);
        let g = query("SELECT class, COUNT(*) AS n FROM heroes GROUP BY class ORDER BY n DESC, class;").unwrap();
        assert_eq!(g[0], vec!["class", "n"]);
        assert_eq!(g[1], vec!["knight", "2"]);
        assert_eq!(col0("SELECT class FROM heroes GROUP BY class HAVING COUNT(*) > 1 ORDER BY class"), vec!["knight", "mage"]);
        assert!(query("SELECT name, COUNT(*) FROM heroes").unwrap_err().contains("GROUP BY"));
        assert!(query("SELECT class FROM heroes WHERE COUNT(*) > 1 GROUP BY class").unwrap_err().contains("HAVING"));
    }

    #[test]
    fn joins() {
        let r = query("SELECT heroes.name, guilds.name FROM heroes JOIN guilds ON heroes.guild_id = guilds.id ORDER BY heroes.name").unwrap();
        assert_eq!(r.len(), 6);
        assert_eq!(r[1], vec!["Aria", "Moon Circle"]);
        let l = query("SELECT h.name, g.city FROM heroes AS h LEFT JOIN guilds AS g ON h.guild_id = g.id WHERE g.city IS NULL").unwrap();
        assert_eq!(l[1], vec!["Finn", "NULL"]);
        assert!(query("SELECT name FROM heroes JOIN guilds ON heroes.guild_id = guilds.id").unwrap_err().contains("more than one"));
        assert_eq!(col0("SELECT q.title FROM quests q JOIN heroes h ON q.hero_id = h.id WHERE h.class = 'knight' ORDER BY q.reward DESC"), vec!["Dragon Peak", "Bandit Camp"]);
    }

    #[test]
    fn changing_data() {
        assert_eq!(run_all("INSERT INTO items (name, type, price) VALUES ('Rope', 'tool', 4); SELECT name FROM items WHERE type = 'tool' ORDER BY name"), vec!["1 row inserted into items", "Map,Rope,Torch"]);
        assert_eq!(run_all("UPDATE items SET price = price + 1 WHERE type = 'potion'; SELECT price FROM items WHERE type = 'potion'"), vec!["2 rows updated in items", "6,13"]);
        assert_eq!(run_all("DELETE FROM heroes WHERE hp < 10; SELECT COUNT(*) FROM heroes"), vec!["2 rows deleted from heroes", "4"]);
        assert_eq!(run_all("CREATE TABLE pets (id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER); INSERT INTO pets VALUES (1, 'Owl', 3), (2, 'Cat', 5); SELECT name FROM pets WHERE age > 4"), vec!["Table pets created with columns id, name, age", "2 rows inserted into pets", "Cat"]);
        assert_eq!(run_all("ALTER TABLE items ADD COLUMN stock INTEGER; SELECT stock FROM items LIMIT 1"), vec!["Column stock added to items", "NULL"]);
        assert!(run_all("INSERT INTO items (name) VALUES ('a', 'b')")[0].contains("2 values for 1"));
    }

    #[test]
    fn friendly_errors() {
        assert!(query("").is_err());
        assert!(query("SELEC * FROM heroes").unwrap_err().contains("SELECT"));
        assert!(query("SELECT * FROM dragons").unwrap_err().contains("heroes"));
        assert!(query("SELECT power FROM heroes").unwrap_err().contains("Columns"));
        assert!(query("SELECT * heroes").is_err());
        assert!(query("SELECT * FROM heroes LIMIT 2 WHERE hp > 1").unwrap_err().contains("order"));
        assert!(query("SELECT * FROM heroes WHERE name = 'x").unwrap_err().contains("quote"));
    }

    #[test]
    fn primary_keys_are_numbered_automatically() {
        assert_eq!(run_all("INSERT INTO items (name, type, price) VALUES ('Lantern', 'tool', 9); SELECT id FROM items WHERE name = 'Lantern'"), vec!["1 row inserted into items", "7"]);
        assert_eq!(run_all("CREATE TABLE pets (id INTEGER PRIMARY KEY, name TEXT); INSERT INTO pets (name) VALUES ('Pip'), ('Mog'); SELECT id FROM pets"), vec!["Table pets created with columns id, name", "2 rows inserted into pets", "1,2"]);
        assert_eq!(run_all("CREATE TABLE toys (name TEXT); INSERT INTO toys VALUES ('Ball'); SELECT name FROM toys"), vec!["Table toys created with columns name", "1 row inserted into toys", "Ball"], "tables without a key are unchanged");
    }
}
