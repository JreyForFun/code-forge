//! A tiny SQL engine for the SQL course: one-table SELECT over built-in sample tables.
//! Supports: columns or *, COUNT/SUM/MIN/MAX/AVG, WHERE <col> <op> <literal>, GROUP BY <col>,
//! ORDER BY <col> [ASC|DESC], LIMIT n. Keywords are case-insensitive.

#[derive(Clone, Debug, PartialEq)]
pub enum Val { N(f64), T(String) }

impl Val {
    pub fn show(&self) -> String {
        match self {
            Val::N(n) if n.fract() == 0.0 && n.abs() < 1e15 => format!("{}", *n as i64),
            Val::N(n) => format!("{:.2}", n),
            Val::T(t) => t.clone(),
        }
    }
    fn cmp(&self, o: &Val) -> std::cmp::Ordering {
        match (self, o) {
            (Val::N(a), Val::N(b)) => a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal),
            (Val::T(a), Val::T(b)) => a.to_lowercase().cmp(&b.to_lowercase()),
            (Val::N(_), Val::T(_)) => std::cmp::Ordering::Less,
            (Val::T(_), Val::N(_)) => std::cmp::Ordering::Greater,
        }
    }
}

pub struct Table { pub name: &'static str, pub cols: Vec<&'static str>, pub rows: Vec<Vec<Val>> }

pub fn sample_tables() -> Vec<Table> {
    let t = |s: &str| Val::T(s.into());
    let n = |x: f64| Val::N(x);
    vec![
        Table { name: "heroes", cols: vec!["name", "class", "hp", "level"], rows: vec![
            vec![t("Aria"), t("mage"), n(8.0), n(3.0)],
            vec![t("Borin"), t("knight"), n(15.0), n(4.0)],
            vec![t("Cael"), t("rogue"), n(11.0), n(2.0)],
            vec![t("Dara"), t("knight"), n(18.0), n(5.0)],
            vec![t("Elm"), t("mage"), n(6.0), n(1.0)],
            vec![t("Finn"), t("ranger"), n(12.0), n(3.0)],
        ]},
        Table { name: "items", cols: vec!["name", "type", "price"], rows: vec![
            vec![t("Potion"), t("potion"), n(5.0)],
            vec![t("Sword"), t("weapon"), n(30.0)],
            vec![t("Shield"), t("armor"), n(25.0)],
            vec![t("Map"), t("tool"), n(8.0)],
            vec![t("Torch"), t("tool"), n(3.0)],
            vec![t("Elixir"), t("potion"), n(12.0)],
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
        else if ch.is_ascii_digit() {
            let s = i;
            while i < c.len() && (c[i].is_ascii_digit() || c[i] == '.') { i += 1; }
            out.push(Tok::Num(c[s..i].iter().collect::<String>().parse().map_err(|_| "That number looks wrong.".to_string())?));
        } else if ch == '\'' || ch == '"' {
            let q = ch; i += 1;
            let s = i;
            while i < c.len() && c[i] != q { i += 1; }
            if i >= c.len() { return Err("A text value is missing its closing quote.".into()); }
            out.push(Tok::Str(c[s..i].iter().collect()));
            i += 1;
        } else if ch.is_alphabetic() || ch == '_' {
            let s = i;
            while i < c.len() && (c[i].is_alphanumeric() || c[i] == '_') { i += 1; }
            out.push(Tok::Word(c[s..i].iter().collect()));
        } else {
            let two: String = c[i..(i + 2).min(c.len())].iter().collect();
            if ["<=", ">=", "!=", "<>"].contains(&two.as_str()) { out.push(Tok::Sym(two)); i += 2; }
            else if "*,();=<>".contains(ch) { out.push(Tok::Sym(ch.to_string())); i += 1; }
            else { return Err(format!("SQL does not understand '{}'.", ch)); }
        }
    }
    Ok(out)
}

#[derive(Debug, Clone)]
enum Item { All, Col(String), Agg(String, Option<String>) }

struct P { t: Vec<Tok>, i: usize }
impl P {
    fn peek(&self) -> Option<&Tok> { self.t.get(self.i) }
    fn kw(&mut self, k: &str) -> bool {
        if let Some(Tok::Word(w)) = self.peek() && w.eq_ignore_ascii_case(k) { self.i += 1; return true; }
        false
    }
    fn sym(&mut self, s: &str) -> bool {
        if let Some(Tok::Sym(x)) = self.peek() && x == s { self.i += 1; return true; }
        false
    }
    fn word(&mut self, what: &str) -> Result<String, String> {
        match self.peek().cloned() { Some(Tok::Word(w)) => { self.i += 1; Ok(w) } _ => Err(format!("Expected {} here.", what)) }
    }
}

const AGGS: [&str; 5] = ["COUNT", "SUM", "MIN", "MAX", "AVG"];
const KEYWORDS: [&str; 11] = ["SELECT", "FROM", "WHERE", "GROUP", "ORDER", "BY", "LIMIT", "ASC", "DESC", "AND", "OR"];

/// Run a query. Ok(rows) where rows[0] is the header.
pub fn query(src: &str) -> Result<Vec<Vec<String>>, String> {
    let toks = lex(src)?;
    if toks.is_empty() { return Err("Type a query, for example: SELECT * FROM heroes;".into()); }
    let mut p = P { t: toks, i: 0 };
    if !p.kw("SELECT") { return Err("A query starts with SELECT.".into()); }
    let mut items = vec![];
    loop {
        if p.sym("*") { items.push(Item::All); }
        else {
            let w = p.word("a column name")?;
            let up = w.to_uppercase();
            if AGGS.contains(&up.as_str()) && p.sym("(") {
                let arg = if p.sym("*") { None } else { Some(p.word("a column name")?) };
                if !p.sym(")") { return Err(format!("{}( needs a closing ).", up)); }
                if arg.is_none() && up != "COUNT" { return Err(format!("{}(*) is not allowed; name a column.", up)); }
                items.push(Item::Agg(up, arg));
            } else if KEYWORDS.contains(&up.as_str()) {
                return Err(format!("Expected a column name before {}.", up));
            } else { items.push(Item::Col(w)); }
        }
        if !p.sym(",") { break; }
    }
    if !p.kw("FROM") { return Err("Missing FROM <table>.".into()); }
    let tname = p.word("a table name")?;
    let tables = sample_tables();
    let table = tables.iter().find(|t| t.name.eq_ignore_ascii_case(&tname))
        .ok_or_else(|| format!("No table named '{}'. Try heroes or items.", tname))?;
    let col = |name: &str| -> Result<usize, String> {
        table.cols.iter().position(|c| c.eq_ignore_ascii_case(name))
            .ok_or_else(|| format!("Table {} has no column '{}'. Columns: {}.", table.name, name, table.cols.join(", ")))
    };
    let mut rows: Vec<&Vec<Val>> = table.rows.iter().collect();
    if p.kw("WHERE") {
        // conditions joined by AND / OR; AND binds tighter (an OR of AND-groups)
        let mut groups: Vec<Vec<(usize, String, Val)>> = vec![vec![]];
        loop {
            let c = col(&p.word("a column name")?)?;
            let op = match p.peek().cloned() { Some(Tok::Sym(s)) if ["=", "!=", "<>", "<", "<=", ">", ">="].contains(&s.as_str()) => { p.i += 1; s } _ => return Err("WHERE needs a comparison like = or >.".into()) };
            let lit = match p.peek().cloned() {
                Some(Tok::Num(n)) => { p.i += 1; Val::N(n) }
                Some(Tok::Str(s)) => { p.i += 1; Val::T(s) }
                Some(Tok::Word(w)) => return Err(format!("Put text values in quotes: '{}'.", w)),
                _ => return Err("WHERE needs a value to compare with.".into()),
            };
            groups.last_mut().unwrap().push((c, op, lit));
            if p.kw("AND") { continue; }
            if p.kw("OR") { groups.push(vec![]); continue; }
            break;
        }
        let test = |r: &Vec<Val>, (c, op, lit): &(usize, String, Val)| {
            let o = r[*c].cmp(lit);
            match op.as_str() { "=" => o.is_eq(), "!=" | "<>" => o.is_ne(), "<" => o.is_lt(), "<=" => o.is_le(), ">" => o.is_gt(), _ => o.is_ge() }
        };
        rows.retain(|r| groups.iter().any(|g| g.iter().all(|cond| test(r, cond))));
    }
    let mut group = None;
    if p.kw("GROUP") {
        if !p.kw("BY") { return Err("Write GROUP BY <column>.".into()); }
        group = Some(col(&p.word("a column name")?)?);
    }
    let mut order: Option<(String, bool)> = None;
    if p.kw("ORDER") {
        if !p.kw("BY") { return Err("Write ORDER BY <column>.".into()); }
        let c = p.word("a column name")?;
        let desc = if p.kw("DESC") { true } else { p.kw("ASC"); false };
        order = Some((c, desc));
    }
    let mut limit = None;
    if p.kw("LIMIT") {
        match p.peek().cloned() { Some(Tok::Num(n)) => { p.i += 1; limit = Some(n as usize); } _ => return Err("LIMIT needs a number.".into()) }
    }
    p.sym(";");
    if let Some(t) = p.peek() {
        let s = match t { Tok::Word(w) => w.clone(), Tok::Num(n) => n.to_string(), Tok::Str(s) => format!("'{}'", s), Tok::Sym(s) => s.clone() };
        return Err(format!("Unexpected '{}' at the end. Check the order: WHERE, GROUP BY, ORDER BY, LIMIT.", s));
    }

    // header
    let mut header = vec![];
    for it in &items {
        match it {
            Item::All => header.extend(table.cols.iter().map(|c| c.to_string())),
            Item::Col(c) => { header.push(table.cols[col(c)?].to_string()); }
            Item::Agg(f, a) => { if let Some(a) = a { col(a)?; } header.push(format!("{}({})", f, a.clone().unwrap_or("*".into()))); }
        }
    }
    let aggregate = group.is_some() || items.iter().any(|i| matches!(i, Item::Agg(..)));
    let mut out: Vec<Vec<Val>> = vec![];
    if aggregate {
        if items.iter().any(|i| matches!(i, Item::All)) { return Err("SELECT * can't be mixed with totals; name the columns.".into()); }
        let mut groups: Vec<(Option<Val>, Vec<&Vec<Val>>)> = vec![];
        match group {
            Some(g) => for r in &rows {
                if let Some(e) = groups.iter_mut().find(|(k, _)| k.as_ref() == Some(&r[g])) { e.1.push(r); } else { groups.push((Some(r[g].clone()), vec![r])); }
            },
            None => groups.push((None, rows.clone())),
        }
        for (_, rs) in &groups {
            let mut line = vec![];
            for it in &items {
                match it {
                    Item::Col(c) => {
                        let ci = col(c)?;
                        if group != Some(ci) { return Err(format!("'{}' must be in GROUP BY to show next to a total.", c)); }
                        line.push(rs.first().map(|r| r[ci].clone()).unwrap_or(Val::T(String::new())));
                    }
                    Item::Agg(f, a) => {
                        let vals: Vec<&Val> = match a { Some(a) => { let ci = col(a)?; rs.iter().map(|r| &r[ci]).collect() } None => rs.iter().map(|r| &r[0]).collect() };
                        let nums: Vec<f64> = vals.iter().filter_map(|v| if let Val::N(n) = v { Some(*n) } else { None }).collect();
                        line.push(match f.as_str() {
                            "COUNT" => Val::N(vals.len() as f64),
                            "SUM" => Val::N(nums.iter().sum()),
                            "AVG" => if nums.is_empty() { Val::T("NULL".into()) } else { Val::N(nums.iter().sum::<f64>() / nums.len() as f64) },
                            "MIN" => vals.iter().min_by(|a, b| a.cmp(b)).map(|v| (*v).clone()).unwrap_or(Val::T("NULL".into())),
                            _ => vals.iter().max_by(|a, b| a.cmp(b)).map(|v| (*v).clone()).unwrap_or(Val::T("NULL".into())),
                        });
                    }
                    Item::All => {}
                }
            }
            out.push(line);
        }
        if let Some((oc, desc)) = &order {
            let hi = header.iter().position(|h| h.eq_ignore_ascii_case(oc)).ok_or_else(|| format!("ORDER BY {} must be one of the selected columns.", oc))?;
            out.sort_by(|a, b| { let o = a[hi].cmp(&b[hi]); if *desc { o.reverse() } else { o } });
        }
    } else {
        if let Some((oc, desc)) = &order {
            let ci = col(oc)?;
            rows.sort_by(|a, b| { let o = a[ci].cmp(&b[ci]); if *desc { o.reverse() } else { o } });
        }
        for r in &rows {
            let mut line = vec![];
            for it in &items {
                match it { Item::All => line.extend(r.iter().cloned()), Item::Col(c) => line.push(r[col(c)?].clone()), Item::Agg(..) => {} }
            }
            out.push(line);
        }
    }
    if let Some(n) = limit { out.truncate(n); }
    let mut res = vec![header];
    res.extend(out.iter().map(|l| l.iter().map(|v| v.show()).collect()));
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn col0(q: &str) -> Vec<String> { query(q).unwrap().iter().skip(1).map(|r| r[0].clone()).collect() }

    #[test]
    fn select_star_and_columns() {
        let r = query("SELECT * FROM heroes;").unwrap();
        assert_eq!(r[0], vec!["name", "class", "hp", "level"]);
        assert_eq!(r.len(), 7);
        let r = query("select name, hp from HEROES").unwrap();
        assert_eq!(r[0], vec!["name", "hp"]);
        assert_eq!(r[1], vec!["Aria", "8"]);
        assert_eq!(query("SELECT * FROM items;").unwrap().len(), 7);
    }

    #[test]
    fn where_filters() {
        assert_eq!(col0("SELECT name FROM heroes WHERE hp > 10;"), vec!["Borin", "Cael", "Dara", "Finn"]);
        assert_eq!(col0("SELECT * FROM heroes WHERE class = 'mage';"), vec!["Aria", "Elm"]);
        assert_eq!(col0("SELECT name FROM heroes WHERE class != 'knight'").len(), 4);
        assert!(query("SELECT * FROM heroes WHERE class = mage").unwrap_err().contains("quotes"));
        assert_eq!(col0("SELECT * FROM heroes WHERE class = 'mage' AND hp > 5;"), vec!["Aria", "Elm"]);
        assert_eq!(col0("SELECT * FROM heroes WHERE class = 'mage' AND hp > 7;"), vec!["Aria"]);
        assert_eq!(col0("SELECT * FROM heroes WHERE class = 'rogue' OR hp > 15;"), vec!["Cael", "Dara"]);
    }

    #[test]
    fn order_and_limit() {
        assert_eq!(col0("SELECT name FROM heroes ORDER BY hp DESC LIMIT 1;"), vec!["Dara"]);
        assert_eq!(col0("SELECT * FROM heroes ORDER BY name;")[0], "Aria");
        assert_eq!(col0("SELECT * FROM heroes LIMIT 3;").len(), 3);
    }

    #[test]
    fn aggregates_and_groups() {
        assert_eq!(query("SELECT COUNT(*) FROM heroes;").unwrap()[1], vec!["6"]);
        assert_eq!(query("SELECT MAX(hp) FROM heroes;").unwrap()[1], vec!["18"]);
        assert_eq!(query("SELECT AVG(price) FROM items").unwrap()[1], vec!["13.83"]);
        let g = query("SELECT class, COUNT(*) FROM heroes GROUP BY class;").unwrap();
        assert_eq!(g[0], vec!["class", "COUNT(*)"]);
        assert!(g.contains(&vec!["knight".to_string(), "2".to_string()]));
        let s = query("SELECT class, SUM(hp) FROM heroes GROUP BY class ORDER BY class;").unwrap();
        assert_eq!(s[1], vec!["knight", "33"]);
    }

    #[test]
    fn friendly_errors() {
        assert!(query("").is_err());
        assert!(query("SELEC * FROM heroes").unwrap_err().contains("SELECT"));
        assert!(query("SELECT * FROM dragons").unwrap_err().contains("heroes"));
        assert!(query("SELECT power FROM heroes").unwrap_err().contains("Columns"));
        assert!(query("SELECT * heroes").is_err());
        assert!(query("SELECT * FROM heroes LIMIT 2 WHERE hp > 1").unwrap_err().contains("order"));
        assert!(query("SELECT name, COUNT(*) FROM heroes").unwrap_err().contains("GROUP BY"));
        assert!(query("SELECT * FROM heroes WHERE name = 'x").unwrap_err().contains("quote"));
    }
}
