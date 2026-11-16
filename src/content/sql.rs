use super::*;
use eframe::egui::Color32;

pub fn course() -> Course {
    Course { slug: "sql", hours: 12, teaches: vec!["SELECT and choosing columns", "Filtering rows with WHERE", "Sorting and limiting results", "Totals with COUNT, SUM and GROUP BY"], snippet: r#"SELECT name FROM heroes;"#, badge: Badge { name: "Vault Keeper", art: "keyhole", desc: "Finished every SQL quest" }, name: "SQL", tag: "SQL", color: Color32::from_rgb(0x1f, 0x9d, 0x8a), lang: "sql", cat: "Data", level: "Beginner", desc: "Learn database basics, queries, filtering and totals", modules: vec![
        m("Querying", vec![
            l("SELECT", &[
                (r#"SQL talks to databases. SELECT reads rows from a table.

SELECT * FROM heroes;

* means every column."#, "book"),
                (r#"A table is like a spreadsheet: columns are fields, rows are records. heroes might hold name, hp and class."#, "chest"),
            ], vec![
                fill("Read from the table.", "___ * FROM heroes;", &["SELECT", "GET", "READ"], 0, "SELECT reads rows."),
                fill("Name the table.", "SELECT * ___ heroes;", &["FROM", "IN", "AT"], 0, "FROM names the table."),
                copy("Type this query.", "SELECT * FROM heroes;", "End with a semicolon."),
                recall("From memory: select everything from items.", "SELECT * FROM items;", "SELECT * FROM items;"),
            ]),
            l("Choosing Columns", &[
                (r#"List the columns you want instead of *, separated by commas."#, "map"),
                (r#"SELECT name, hp FROM heroes;"#, "scroll_fire"),
            ], vec![
                fill("Pick two columns.", "SELECT name___ hp FROM heroes;", &[",", ".", ";"], 0, "Separate columns with commas."),
                fill("Pick the column hp.", "SELECT ___ FROM heroes;", &["hp", "*", "FROM"], 0, "Name the column."),
                copy("Type this query.", "SELECT name, hp FROM heroes;", "Columns then table."),
                recall("From memory: select name from heroes.", "SELECT name FROM heroes;", "SELECT name FROM heroes;"),
            ]),
        ]),
        m("Filtering", vec![
            l("WHERE", &[
                (r#"WHERE keeps only the rows that match a condition."#, "key"),
                (r#"SELECT * FROM heroes
WHERE hp > 10;"#, "shield"),
            ], vec![
                fill("Filter the rows.", "SELECT * FROM heroes ___ hp > 10;", &["WHERE", "IF", "HAVING"], 0, "WHERE filters rows."),
                fill("Only strong heroes.", "SELECT * FROM heroes WHERE hp ___ 10;", &[">", "<", "=>"], 0, "> means greater than."),
                copy("Type this query.", "SELECT * FROM heroes WHERE hp > 10;", "WHERE then a condition."),
                recall("From memory: heroes with hp above 5.", "SELECT * FROM heroes WHERE hp > 5;", "WHERE hp > 5"),
            ]),
            l("AND / OR", &[
                (r#"Combine conditions: AND needs both to be true, OR needs at least one. Text values use single quotes."#, "sword"),
                (r#"SELECT * FROM heroes
WHERE class = 'mage' AND hp > 5;"#, "staff"),
            ], vec![
                fill("Need both conditions.", "SELECT * FROM heroes WHERE class = 'mage' ___ hp > 5;", &["AND", "OR", "ALSO"], 0, "AND needs both."),
                fill("Match text.", "SELECT * FROM heroes WHERE class = ___mage';", &["'", "\"", "`"], 0, "SQL text uses single quotes."),
                copy("Type this query.", "SELECT * FROM heroes WHERE class = 'mage';", "Single quotes around text."),
                recall("From memory: heroes where class is 'knight'.", "SELECT * FROM heroes WHERE class = 'knight';", "class = 'knight'"),
            ]),
        ]),
        m("Sorting", vec![
            l("ORDER BY", &[
                (r#"ORDER BY sorts the results. ASC goes low to high (default), DESC goes high to low."#, "crystal_blue"),
                (r#"SELECT * FROM heroes
ORDER BY hp DESC;"#, "coin_gold"),
            ], vec![
                fill("Sort the rows.", "SELECT * FROM heroes ___ BY hp DESC;", &["ORDER", "SORT", "GROUP"], 0, "ORDER BY."),
                fill("Highest first.", "SELECT * FROM heroes ORDER BY hp ___;", &["DESC", "ASC", "TOP"], 0, "DESC is high to low."),
                copy("Type this query.", "SELECT * FROM heroes ORDER BY hp DESC;", "ORDER BY column direction."),
                recall("From memory: order heroes by name.", "SELECT * FROM heroes ORDER BY name;", "ORDER BY name"),
            ]),
            l("LIMIT", &[
                (r#"LIMIT caps how many rows come back. Combine it with ORDER BY for a top list."#, "chest"),
                (r#"SELECT * FROM heroes
ORDER BY hp DESC
LIMIT 3;"#, "shield"),
            ], vec![
                fill("Only three rows.", "SELECT * FROM heroes ___ 3;", &["LIMIT", "TOP", "MAX"], 0, "LIMIT n."),
                fill("Build a top list.", "SELECT * FROM heroes ___ BY hp DESC LIMIT 3;", &["ORDER", "GROUP", "WHERE"], 0, "Sort, then limit."),
                copy("Type this query.", "SELECT * FROM heroes LIMIT 3;", "LIMIT goes last."),
                recall("From memory: the top 1 hero by hp.", "SELECT * FROM heroes ORDER BY hp DESC LIMIT 1;", "ORDER BY hp DESC LIMIT 1"),
            ]),
        ]),
        m("Totals", vec![
            l("COUNT and SUM", &[
                (r#"Aggregate functions summarise many rows into one value: COUNT, SUM, AVG, MAX and MIN."#, "coin_gold"),
                (r#"SELECT COUNT(*) FROM heroes;
SELECT SUM(hp) FROM heroes;"#, "gem_green"),
            ], vec![
                fill("Count the rows.", "SELECT ___(*) FROM heroes;", &["COUNT", "TOTAL", "NUM"], 0, "COUNT(*) counts rows."),
                fill("Add up the hp.", "SELECT ___(hp) FROM heroes;", &["SUM", "ADD", "PLUS"], 0, "SUM adds values."),
                copy("Type this query.", "SELECT COUNT(*) FROM heroes;", "COUNT(*)."),
                recall("From memory: the largest hp.", "SELECT MAX(hp) FROM heroes;", "MAX(hp)"),
            ]),
            l("GROUP BY", &[
                (r#"GROUP BY makes one result row per value, so you can total each group."#, "cauldron"),
                (r#"SELECT class, COUNT(*)
FROM heroes
GROUP BY class;"#, "map"),
            ], vec![
                fill("One row per class.", "SELECT class, COUNT(*) FROM heroes ___ BY class;", &["GROUP", "ORDER", "SORT"], 0, "GROUP BY."),
                fill("Count each group.", "SELECT class, ___(*) FROM heroes GROUP BY class;", &["COUNT", "SUM", "ALL"], 0, "COUNT(*) per group."),
                copy("Type this query.", "SELECT class, COUNT(*) FROM heroes GROUP BY class;", "GROUP BY goes last."),
                recall("From memory: total hp per class.", "SELECT class, SUM(hp) FROM heroes GROUP BY class;", "SUM(hp) GROUP BY class"),
            ]),
        ]),
    ]}
}
