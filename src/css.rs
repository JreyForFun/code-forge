//! CSS preview for the CSS course: parses `selector { prop: value; }` rules and builds a style
//! for a sample line of text.

#[derive(Clone, Debug, PartialEq)]
pub struct Rule { pub selector: String, pub decls: Vec<(String, String)> }

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Style { pub color: Option<[u8; 3]>, pub bg: Option<[u8; 3]>, pub size: Option<f32>, pub align: Option<String>, pub bold: bool }

impl Style {
    /// Compact text encoding used to pass the style through the interpreter's output list.
    pub fn encode(&self, text: &str) -> String {
        let c = |x: Option<[u8; 3]>| x.map(|v| format!("{:02x}{:02x}{:02x}", v[0], v[1], v[2])).unwrap_or_default();
        format!("{}|{}|{}|{}|{}|{}", c(self.color), c(self.bg), self.size.map(|s| s.to_string()).unwrap_or_default(), self.align.clone().unwrap_or_default(), self.bold as u8, text)
    }
    pub fn decode(s: &str) -> (Style, String) {
        let p: Vec<&str> = s.splitn(6, '|').collect();
        if p.len() < 6 { return (Style::default(), s.to_string()); }
        let hex = |h: &str| if h.len() == 6 { u32::from_str_radix(h, 16).ok().map(|v| [(v >> 16) as u8, (v >> 8) as u8, v as u8]) } else { None };
        (Style { color: hex(p[0]), bg: hex(p[1]), size: p[2].parse().ok(), align: if p[3].is_empty() { None } else { Some(p[3].into()) }, bold: p[4] == "1" }, p[5].to_string())
    }
}

pub fn named(c: &str) -> Option<[u8; 3]> {
    Some(match c {
        "black" => [0, 0, 0], "white" => [255, 255, 255], "red" => [255, 0, 0], "green" => [0, 128, 0], "lime" => [0, 255, 0],
        "blue" => [0, 0, 255], "yellow" => [255, 255, 0], "gold" => [255, 215, 0], "orange" => [255, 165, 0], "purple" => [128, 0, 128],
        "pink" => [255, 192, 203], "gray" | "grey" => [128, 128, 128], "silver" => [192, 192, 192], "navy" => [0, 0, 128], "teal" => [0, 128, 128],
        "aqua" | "cyan" => [0, 255, 255], "maroon" => [128, 0, 0], "olive" => [128, 128, 0], "fuchsia" | "magenta" => [255, 0, 255],
        "brown" => [165, 42, 42], "crimson" => [220, 20, 60], "coral" => [255, 127, 80], "tomato" => [255, 99, 71], "violet" => [238, 130, 238],
        _ => return None,
    })
}

pub fn color(v: &str) -> Option<[u8; 3]> {
    let v = v.trim().to_lowercase();
    if let Some(h) = v.strip_prefix('#') {
        let h: String = if h.len() == 3 { h.chars().flat_map(|c| [c, c]).collect() } else { h.to_string() };
        if h.len() != 6 { return None; }
        let n = u32::from_str_radix(&h, 16).ok()?;
        return Some([(n >> 16) as u8, (n >> 8) as u8, n as u8]);
    }
    named(&v)
}

pub fn parse(src: &str) -> Result<Vec<Rule>, String> {
    // strip /* comments */
    let mut s = String::new();
    let mut rest = src;
    while let Some(a) = rest.find("/*") {
        s.push_str(&rest[..a]);
        match rest[a..].find("*/") { Some(b) => rest = &rest[a + b + 2..], None => return Err("A comment is missing its closing */.".into()) }
    }
    s.push_str(rest);
    let mut rules = vec![];
    let mut rest = s.as_str();
    loop {
        let t = rest.trim();
        if t.is_empty() { break; }
        let Some(open) = t.find('{') else { return Err(format!("Missing {{ after '{}'.", t.chars().take(20).collect::<String>())) };
        let selector = t[..open].trim().to_string();
        if selector.is_empty() { return Err("A rule needs a selector before {, like p or .big.".into()); }
        let Some(close) = t[open..].find('}') else { return Err(format!("The rule for '{}' is missing its closing }}.", selector)) };
        let body = &t[open + 1..open + close];
        if body.contains('{') { return Err(format!("The rule for '{}' is missing its closing }}.", selector)); }
        let mut decls = vec![];
        for d in body.split(';') {
            let d = d.trim();
            if d.is_empty() { continue; }
            let Some((k, v)) = d.split_once(':') else { return Err(format!("'{}' needs a colon: property: value;", d)) };
            let (k, v) = (k.trim().to_lowercase(), v.trim().to_string());
            if k.is_empty() || v.is_empty() { return Err(format!("'{}' needs both a property and a value.", d)); }
            decls.push((k, v));
        }
        rules.push(Rule { selector, decls });
        rest = &t[open + close + 1..];
    }
    if rules.is_empty() { return Err("Write a rule, for example: p { color: red; }".into()); }
    Ok(rules)
}

pub fn style_of(rule: &Rule) -> Style {
    let mut st = Style::default();
    for (k, v) in &rule.decls {
        match k.as_str() {
            "color" => st.color = color(v),
            "background-color" | "background" => st.bg = color(v),
            "font-size" => st.size = v.trim_end_matches("px").trim().parse::<f32>().ok().map(|s| s.clamp(8.0, 40.0)),
            "text-align" => st.align = Some(v.to_lowercase()),
            "font-weight" => st.bold = v == "bold" || v.parse::<u32>().map(|w| w >= 600).unwrap_or(false),
            _ => {}
        }
    }
    st
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_rules_and_styles() {
        let r = parse("p { color: red; }").unwrap();
        assert_eq!(r[0].selector, "p");
        assert_eq!(r[0].decls, vec![("color".to_string(), "red".to_string())]);
        assert_eq!(style_of(&r[0]).color, Some([255, 0, 0]));
        let r = parse("body { background-color: #020617; }\n.big { font-size: 24px; text-align: center }").unwrap();
        assert_eq!(r.len(), 2);
        assert_eq!(style_of(&r[0]).bg, Some([2, 6, 23]));
        let st = style_of(&r[1]);
        assert_eq!(st.size, Some(24.0));
        assert_eq!(st.align.as_deref(), Some("center"));
    }

    #[test]
    fn colors() {
        assert_eq!(color("#fff"), Some([255, 255, 255]));
        assert_eq!(color("Gold"), Some([255, 215, 0]));
        assert_eq!(color("#12"), None);
        assert_eq!(color("sparkly"), None);
    }

    #[test]
    fn errors() {
        assert!(parse("p color: red;").is_err());
        assert!(parse("p { color: red;").is_err());
        assert!(parse("p { color red; }").unwrap_err().contains("colon"));
        assert!(parse("{ color: red; }").is_err());
        assert!(parse("").is_err());
        assert!(parse("/* hi */ p { color: red; }").is_ok());
    }

    #[test]
    fn style_encoding_round_trips() {
        let st = Style { color: Some([1, 2, 3]), bg: None, size: Some(20.0), align: Some("center".into()), bold: true };
        let (d, t) = Style::decode(&st.encode("a | b"));
        assert_eq!(d, st);
        assert_eq!(t, "a | b");
    }
}
