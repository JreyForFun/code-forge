//! SQL: seven chapters from the first SELECT to joins. Every answer runs against the sample
//! database (heroes, items, guilds, quests). Wording, data and projects are original.
use super::*;
use eframe::egui::Color32;

pub fn course() -> Course {
    Course {
        name: "SQL", tag: "SQL", slug: "sql", color: Color32::from_rgb(0x1f, 0x9d, 0x8a), lang: "sql", cat: "Data", level: "Beginner", hours: 16,
        desc: "Ask databases questions with SQL: query, filter, sort, total, change data, build tables and join them.",
        teaches: vec!["SELECT, DISTINCT and aliases", "Filtering with WHERE, LIKE, IN and NULL", "Sorting, limiting and calculated columns", "Totals with COUNT, SUM, AVG and GROUP BY", "INSERT, UPDATE, DELETE and CREATE TABLE", "Combining tables with JOIN"],
        snippet: r#"SELECT name FROM heroes;"#,
        badge: Badge { name: "Vault Keeper", art: "medal", desc: "Finished every SQL quest" },
        modules: vec![querying(), filtering(), sorting(), aggregates(), changing(), building(), joining()],
    }
}

fn querying() -> Module {
    ch("Querying", "What a database is, and your first questions with SELECT.", vec![
        l("What Is a Database?", &[
            (r#"A database stores data in TABLES.

A table is like a spreadsheet: columns are the fields, rows are the records."#, "book"),
            (r#"heroes
id | name  | class  | hp
1  | Aria  | mage   | 8
2  | Borin | knight | 15"#, "map"),
        ], vec![
            quiz("In a table, what is a row?", &["One record, like one hero", "One field, like hp", "The table's name"], 0, "Rows are records; columns are fields."),
            quiz("What does SQL stand for?", &["Structured Query Language", "Simple Question List", "Server Quick Link"], 0, "SQL is the language for asking databases questions."),
            fill("Show everything in heroes.", "SELECT * FROM ___;", &["heroes", "hero", "table"], 0, "Name the table after FROM."),
            fill("End the statement.", "SELECT * FROM items___", &[";", ".", ":"], 0, "Statements end with a semicolon."),
            copy("Type your first query.", "SELECT * FROM heroes;", "* means every column."),
            recall("From memory: show every column of the items table.", "SELECT * FROM items;", "SELECT * FROM items;"),
        ]).with(r##"## Data in tables
Almost every app you use keeps its data in a database: game saves, shop orders, messages, high scores. Most databases store data in tables, and you ask them questions in SQL (Structured Query Language, often said "sequel").

A table looks like a spreadsheet:

    heroes
    id | name  | class  | hp | level | guild_id
    1  | Aria  | mage   | 8  | 3     | 1
    2  | Borin | knight | 15 | 4     | 2

- Each column is a field with a name: name, class, hp.
- Each row is one record: one hero.
- The id column gives every row a unique number.

## Your practice database
Code Forge has four tables you can query: heroes, items, guilds and quests. The Spell Lab shows their columns when you pick SQL.

## Your first query
    SELECT * FROM heroes;

Read it as "select every column from the heroes table". SELECT says what to show, * means all columns, FROM names the table, and the semicolon ends the statement.

## SQL is forgiving about case
select and SELECT mean the same. Writing keywords in capitals is a habit that makes queries easy to read."##,
        &["Databases store data in tables of rows and columns.", "A row is a record; a column is a field.", "SQL is the language for asking a database questions.", "SELECT * FROM table; shows everything."]),
        l("SELECT", &[
            (r#"SELECT picks what to show. FROM picks the table.

SELECT * FROM heroes;

* means "all columns"."#, "scroll_fire"),
            (r#"Keywords can be any case, but CAPITALS are easier to read.

End each statement with ;"#, "keyhole"),
        ], vec![
            quiz("What does * mean in SELECT *?", &["Every column", "Multiply", "The first row"], 0, "* is shorthand for all columns."),
            quiz("Which keyword names the table?", &["FROM", "SELECT", "TABLE"], 0, "FROM table_name."),
            fill("Show every column.", "SELECT ___ FROM items;", &["*", "all", "%"], 0, "* means all columns."),
            fill("Pick the table.", "SELECT * ___ heroes;", &["FROM", "IN", "OF"], 0, "FROM names the table."),
            copy("Type this query.", "SELECT * FROM heroes;", "Shows every hero."),
            recall("From memory: show every column of the heroes table.", "SELECT * FROM heroes;", "SELECT * FROM heroes;"),
        ]).with(r##"## The shape of a query
Every question you ask with SQL starts with SELECT:

    SELECT * FROM heroes;

- SELECT: what to show (here *, every column).
- FROM: which table to look in.
- ; ends the statement, like a full stop.

## The result is a table
A query never changes the data. It builds a new, temporary table with the answer and shows it to you. You can run the same query a hundred times and nothing in the database moves.

## Spacing and lines
SQL ignores extra spaces and line breaks, so long queries are often split over several lines:

    SELECT *
    FROM heroes;

## Common mistakes
- Misspelling a keyword: SELCT or FORM.
- Misspelling the table: hero instead of heroes. The error message lists the tables that exist.
- Putting FROM before SELECT. The order of the parts is fixed, and you will meet more parts (WHERE, ORDER BY...) that each have their place.

## When to use *
* is perfect for exploring a table you do not know. In real apps, name the columns you need instead: it is faster and clearer (next lesson)."##,
        &["SELECT says what to show; FROM says which table.", "A query builds a new result table and never changes the data.", "SQL ignores extra spaces and line breaks.", "* is great for exploring; name columns in real queries."]),
        l("Choosing Columns", &[
            (r#"Name the columns you want, separated by commas:

SELECT name, hp FROM heroes;"#, "chest"),
            (r#"The result shows the columns in the order you wrote them."#, "map"),
        ], vec![
            quiz("What separates column names in SELECT?", &["Commas", "Spaces", "AND"], 0, "SELECT a, b, c."),
            quiz("SELECT hp, name shows which column first?", &["hp", "name", "Alphabetical order"], 0, "Columns appear in the order written."),
            fill("Show only names.", "SELECT ___ FROM heroes;", &["name", "*name", "names"], 0, "Write the column name."),
            fill("Show name and class.", "SELECT name___ class FROM heroes;", &[",", "AND", "+"], 0, "Commas separate columns."),
            copy("Type this query.", "SELECT name, price FROM items;", "Two columns."),
            recall("From memory: show the name and level of every hero.", "SELECT name, level FROM heroes;", "SELECT name, level FROM heroes;"),
        ]).with(r##"## Asking for just what you need
Instead of *, list the columns you want, separated by commas:

    SELECT name, hp FROM heroes;

The result has exactly those columns, in that order. SELECT hp, name would show hp first.

## Why not always use *?
- Clarity: a reader sees immediately which data the query uses.
- Speed: real tables can have dozens of columns and millions of rows; fetching only what you need is much faster.
- Stability: if someone adds a column to the table later, your query still returns the same shape.

## Errors
Asking for a column that does not exist gives an error that lists the real columns:

    SELECT power FROM heroes;

Check spelling first; column names must match.

## Commas
A comma goes BETWEEN names, never after the last one: SELECT name, hp, FROM heroes is a mistake.

## Exploring a new table
A good habit: run SELECT * FROM table LIMIT 3 (you will learn LIMIT soon) to see the column names, then write the real query with only the columns you need."##,
        &["List columns after SELECT, separated by commas.", "Columns appear in the order you write them.", "Naming columns is clearer and faster than *.", "No comma after the last column."]),
        l("DISTINCT and AS", &[
            (r#"DISTINCT removes duplicate rows:

SELECT DISTINCT class FROM heroes;"#, "crystal_blue"),
            (r#"AS renames a column in the result:

SELECT name AS hero FROM heroes;"#, "banner"),
        ], vec![
            quiz("What does DISTINCT do?", &["Removes duplicate rows from the result", "Sorts the rows", "Deletes data"], 0, "Each different value appears once."),
            quiz("Does AS change the table in the database?", &["No, only the result's column name", "Yes, it renames the column forever"], 0, "Aliases only affect the result."),
            fill("List each class once.", "SELECT ___ class FROM heroes;", &["DISTINCT", "UNIQUE", "ONCE"], 0, "SELECT DISTINCT column."),
            fill("Rename the column in the result.", "SELECT name ___ hero FROM heroes;", &["AS", "IS", "TO"], 0, "column AS new_name."),
            copy("Type this query.", "SELECT DISTINCT type FROM items;", "Each type once."),
            recall("From memory: show each hero's name renamed as hero_name.", "SELECT name AS hero_name FROM heroes;", "name AS hero_name."),
        ]).with(r##"## DISTINCT: each value once
Many rows can share a value. Two heroes are knights, two are mages. To see each class only once:

    SELECT DISTINCT class FROM heroes;

DISTINCT looks at whole result rows. With several columns, SELECT DISTINCT class, level removes only rows where BOTH class and level repeat.

DISTINCT is great for questions like "which item types do we sell?" or "which cities have guilds?"

## AS: friendlier column names
AS gives a column a new name in the result (an alias):

    SELECT name AS hero, hp AS health FROM heroes;

The header now says hero and health. The real table is unchanged; the alias exists only in this result.

Aliases matter more when columns are calculated (price * 2) or come from totals (COUNT(*)), because those headers are otherwise long and confusing. You will use AS a lot in later chapters.

## Names with spaces
An alias with a space needs quotes: AS 'hit points'. Most people use underscores instead: hit_points."##,
        &["SELECT DISTINCT shows each different row once.", "AS renames a column in the result only.", "Aliases make calculated columns readable.", "Use underscores (or quotes) for multi-word aliases."]),
        p("Project: Hero Roster", &[
            (r##"CHECKPOINT PROJECT

The guild master wants a tidy roster: names, classes and levels, with a friendly header for the name, plus a list of the classes in the guild."##, "helmet_knight"),
            (r##"SELECT name AS hero, class, level FROM heroes;
SELECT DISTINCT class FROM heroes;"##, "banner_royal"),
        ], vec![
            quiz("Which query lists every class once?", &["SELECT DISTINCT class FROM heroes;", "SELECT class FROM heroes;", "SELECT * FROM class;"], 0, "DISTINCT removes duplicates."),
            quiz("Can one answer contain two queries?", &["Yes, separated by semicolons", "No"], 0, "Each statement ends with ; and gets its own result."),
            fill("Give the name column a friendly header.", r##"SELECT name ___ hero, class, level FROM heroes;"##, &["AS", "=", "TO"], 0, "AS sets the header."),
            fill("Choose the roster columns.", r##"SELECT name AS hero, class, ___ FROM heroes;"##, &["level", "levels", "*"], 0, "Name the column exactly."),
            quiz("What is the header of the first column in the roster?", &["hero", "name", "heroes"], 0, "AS hero renames it in the result."),
            copy("Type both queries.", r##"SELECT name AS hero, class, level FROM heroes;
SELECT DISTINCT class FROM heroes;"##, "A roster and a list of classes."),
            recall("From memory: show each different guild_id in heroes once.", r##"SELECT DISTINCT guild_id FROM heroes;"##, "DISTINCT guild_id."),
        ]).with(r##"## Your first report
Reports are what databases are for: answering a real question for a real person. The guild master asks for two things:

1. A roster with each hero's name, class and level, with a friendly header for the name.
2. A list of the different classes in the guild.

## The queries
    SELECT name AS hero, class, level FROM heroes;
    SELECT DISTINCT class FROM heroes;

You can run both at once: each statement ends with a semicolon and gets its own result table.

## Think like a reporter
Before writing a query, say the question in plain words: "show me the name, class and level of every hero". Then map each word to SQL: "show" becomes SELECT, "every hero" becomes FROM heroes. This habit makes even big queries easier.

## Check
- Do the headers read nicely?
- Does the class list have no repeats?
- Did you name exactly the columns that were asked for, instead of SELECT *?"##,
        &["Say the question in plain words, then translate it to SQL.", "Name exactly the columns asked for.", "Use AS for friendly headers and DISTINCT for unique lists.", "Several statements can run one after another."]),
    ])
}

fn filtering() -> Module {
    ch("Filtering", "Pick only the rows you need with WHERE, AND/OR, LIKE, IN, BETWEEN and NULL checks.", vec![
        l("WHERE", &[
            (r#"WHERE keeps only rows that match a condition:

SELECT * FROM heroes WHERE hp > 10;"#, "sword"),
            (r#"Comparisons: =  !=  <  >  <=  >=

Text goes in single quotes:
WHERE class = 'mage'"#, "hat_wizard"),
        ], vec![
            quiz("Which keyword filters rows?", &["WHERE", "FILTER", "IF"], 0, "WHERE condition."),
            quiz("How do you write the text value mage in a condition?", &["'mage'", "mage", "(mage)"], 0, "Text values go in quotes."),
            fill("Only mages.", "SELECT * FROM heroes WHERE class = ___;", &["'mage'", "mage", "\"mage"], 0, "Quote text values."),
            fill("Heroes with more than 10 hp.", "SELECT name FROM heroes WHERE hp ___ 10;", &[">", "=>", "=="], 0, "> means greater than."),
            copy("Type this query.", "SELECT name FROM heroes WHERE level >= 3;", "Level 3 and up."),
            recall("From memory: show every item whose price is below 10.", "SELECT * FROM items WHERE price < 10;", "WHERE price < 10."),
        ]).with(r##"## Choosing rows
SELECT chooses columns; WHERE chooses rows. Only rows where the condition is true appear in the result:

    SELECT name, hp FROM heroes WHERE hp > 10;

## Comparison operators
    =    equal                 !=  or <>   not equal
    >    greater than          <           less than
    >=   greater or equal      <=          less or equal

Note that SQL uses a single = for comparing (unlike Python and JavaScript).

## Text values need quotes
    SELECT * FROM heroes WHERE class = 'knight';

Without quotes, SQL thinks knight is a column name and reports that no such column exists. Numbers never need quotes.

## Order of the parts
WHERE comes after FROM:

    SELECT columns FROM table WHERE condition;

## Filtering does not delete
WHERE only hides rows from this result. The rows are still in the table. (Deleting data is a different statement, coming later.)

## Edges
hp > 10 excludes a hero with exactly 10; hp >= 10 includes them. Think about the boundary every time."##,
        &["WHERE keeps only rows where the condition is true.", "SQL compares with a single =; != or <> means not equal.", "Text values go in single quotes; numbers do not.", "WHERE comes after FROM and never deletes data."]),
        l("AND / OR", &[
            (r#"Combine conditions:

AND  both must be true
OR   at least one true

WHERE class = 'mage' AND hp > 7"#, "keyhole"),
            (r#"Use brackets when mixing:

WHERE class = 'mage' AND (hp > 7 OR level > 2)"#, "key"),
        ], vec![
            quiz("WHERE class = 'mage' AND hp > 100 matches...", &["Mages with more than 100 hp", "All mages and all strong heroes", "Nothing ever"], 0, "AND needs both."),
            quiz("Which keyword needs only one condition to be true?", &["OR", "AND", "NOT"], 0, "OR needs at least one."),
            fill("Both must be true.", "SELECT * FROM heroes WHERE class = 'knight' ___ hp > 16;", &["AND", "OR", "&&"], 0, "SQL uses the word AND."),
            fill("Either class works.", "SELECT * FROM heroes WHERE class = 'rogue' ___ class = 'ranger';", &["OR", "AND", "||"], 0, "OR allows either."),
            copy("Type this query.", "SELECT name FROM heroes WHERE class = 'mage' AND level > 1;", "Both conditions."),
            recall("From memory: show items that are potions OR cost more than 20.", "SELECT * FROM items WHERE type = 'potion' OR price > 20;", "type = 'potion' OR price > 20."),
        ]).with(r##"## Combining conditions
- AND: a row must pass BOTH conditions.
- OR: a row must pass AT LEAST ONE.
- NOT: flips a condition.

    SELECT * FROM heroes WHERE class = 'knight' AND hp > 16;
    SELECT * FROM heroes WHERE class = 'rogue' OR class = 'ranger';
    SELECT * FROM heroes WHERE NOT class = 'mage';

## AND before OR
SQL evaluates AND before OR, like multiplication before addition. This query:

    WHERE class = 'mage' AND hp > 7 OR level > 4

means (mage with hp > 7) OR (anyone above level 4). If you meant "mages that have hp > 7 or level > 4", add brackets:

    WHERE class = 'mage' AND (hp > 7 OR level > 4)

When you mix AND and OR, always use brackets. It costs nothing and prevents surprises.

## A common mistake
    WHERE class = 'rogue' OR 'ranger'     -- wrong

Each side of OR must be a full condition: class = 'rogue' OR class = 'ranger'. (The next lessons show a shorter way with IN.)"##,
        &["AND needs both conditions; OR needs at least one.", "NOT flips a condition.", "AND is evaluated before OR; use brackets when mixing.", "Each side of OR must be a complete condition."]),
        l("LIKE", &[
            (r#"LIKE matches text patterns:

%  any number of characters
_  exactly one character

WHERE name LIKE 'D%'"#, "spellbook"),
            (r#"'%ion'   ends with ion
'%or%'   contains or
'_l%'    second letter is l"#, "scroll_lightning"),
        ], vec![
            quiz("What does % mean in a LIKE pattern?", &["Any number of characters", "Exactly one character", "A percentage"], 0, "% matches zero or more characters."),
            quiz("Which pattern finds names ending in 'n'?", &["'%n'", "'n%'", "'_n'"], 0, "% first, then n."),
            fill("Names starting with A.", "SELECT name FROM heroes WHERE name LIKE ___;", &["'A%'", "'%A'", "'A_'"], 0, "A then anything."),
            fill("Use pattern matching.", "SELECT * FROM items WHERE name ___ '%ion';", &["LIKE", "=", "IS"], 0, "LIKE matches patterns."),
            copy("Type this query.", "SELECT name FROM items WHERE name LIKE '%o%';", "Contains an o."),
            recall("From memory: show heroes whose name starts with B.", "SELECT * FROM heroes WHERE name LIKE 'B%';", "LIKE 'B%'."),
        ]).with(r##"## Searching inside text
= finds exact matches. LIKE finds patterns, using two wildcard characters:

- % stands for any number of characters (even none).
- _ stands for exactly one character.

    SELECT name FROM heroes WHERE name LIKE 'D%';    -- starts with D
    SELECT name FROM items WHERE name LIKE '%ion';   -- ends with ion
    SELECT name FROM items WHERE name LIKE '%or%';   -- contains or
    SELECT name FROM heroes WHERE name LIKE '_l%';   -- second letter l
    SELECT name FROM heroes WHERE name LIKE '____';  -- exactly 4 letters

## Search boxes
When you type in a shop's search box, the app often runs something like WHERE name LIKE '%sword%' behind the scenes.

## Case
In many databases (including Code Forge's), LIKE ignores upper and lower case for simple letters, so 'd%' also finds Dara.

## NOT LIKE
    SELECT name FROM items WHERE name NOT LIKE '%o%';

finds names without an o.

## Speed
Patterns that start with % make the database check every row. That is fine for small tables, but big apps use special search tools for this."##,
        &["LIKE matches text patterns.", "% means any number of characters; _ means exactly one.", "'A%' starts with A, '%a' ends with a, '%a%' contains a.", "NOT LIKE excludes matches."]),
        l("IN and BETWEEN", &[
            (r#"IN checks against a list:

WHERE class IN ('mage', 'rogue')"#, "chest"),
            (r#"BETWEEN checks a range (both ends included):

WHERE hp BETWEEN 10 AND 15"#, "hourglass"),
        ], vec![
            quiz("Is 15 included in BETWEEN 10 AND 15?", &["Yes", "No"], 0, "BETWEEN includes both ends."),
            quiz("class IN ('mage', 'rogue') is the same as...", &["class = 'mage' OR class = 'rogue'", "class = 'mage' AND class = 'rogue'", "class LIKE 'mage%'"], 0, "IN is a short OR."),
            fill("Match any of the listed types.", "SELECT * FROM items WHERE type ___ ('tool', 'armor');", &["IN", "LIKE", "BETWEEN"], 0, "IN checks a list."),
            fill("A price range.", "SELECT * FROM items WHERE price BETWEEN 5 ___ 12;", &["AND", "TO", "OR"], 0, "BETWEEN low AND high."),
            copy("Type this query.", "SELECT name FROM heroes WHERE level BETWEEN 2 AND 4;", "Levels 2, 3 and 4."),
            recall("From memory: show heroes whose class is knight or ranger, using IN.", "SELECT * FROM heroes WHERE class IN ('knight', 'ranger');", "IN with a list."),
        ]).with(r##"## IN: one of a list
Instead of chaining ORs:

    WHERE class = 'mage' OR class = 'rogue' OR class = 'ranger'

write the list once:

    WHERE class IN ('mage', 'rogue', 'ranger')

The list goes in brackets, with commas between values. Text values still need quotes; numbers do not: WHERE level IN (1, 3, 5).

NOT IN excludes the listed values: WHERE class NOT IN ('knight').

## BETWEEN: a range
    WHERE hp BETWEEN 10 AND 15

means hp >= 10 AND hp <= 15. Both ends are INCLUDED, so heroes with exactly 10 or 15 hp appear.

BETWEEN works with numbers, and also with text and dates in alphabetical or calendar order: WHERE name BETWEEN 'A' AND 'C'.

NOT BETWEEN finds values outside the range.

## Why use them?
IN and BETWEEN say exactly what you mean in fewer words, and fewer words means fewer mistakes. Reading WHERE price BETWEEN 5 AND 12 aloud sounds just like the question you were asked."##,
        &["IN ('a', 'b') matches any value in the list.", "BETWEEN low AND high includes both ends.", "NOT IN and NOT BETWEEN exclude.", "Both are shorter, clearer versions of OR and >=/<= checks."]),
        l("NULL Values", &[
            (r#"NULL means "no value": unknown or missing.

Finn has no guild: guild_id is NULL."#, "lantern"),
            (r#"Use IS NULL, not = NULL:

WHERE guild_id IS NULL
WHERE guild_id IS NOT NULL"#, "keyhole"),
        ], vec![
            quiz("What does NULL mean?", &["No value / unknown", "Zero", "Empty text"], 0, "NULL is the absence of a value."),
            quiz("How do you find rows with a missing guild_id?", &["WHERE guild_id IS NULL", "WHERE guild_id = NULL", "WHERE guild_id = 0"], 0, "Use IS NULL."),
            fill("Find heroes without a guild.", "SELECT name FROM heroes WHERE guild_id IS ___;", &["NULL", "0", "''"], 0, "IS NULL."),
            fill("Find quests that have a hero.", "SELECT title FROM quests WHERE hero_id IS ___ NULL;", &["NOT", "!", "NO"], 0, "IS NOT NULL."),
            copy("Type this query.", "SELECT title FROM quests WHERE hero_id IS NULL;", "Quests nobody has taken."),
            recall("From memory: show the names of heroes that DO have a guild_id.", "SELECT name FROM heroes WHERE guild_id IS NOT NULL;", "IS NOT NULL."),
        ]).with(r##"## Missing values
Sometimes a value is unknown or does not exist yet. Databases store this as NULL. In our data, Finn the ranger has no guild, so his guild_id is NULL, and the Haunted Mill quest has no hero yet.

NULL is NOT the same as 0 or empty text '': 0 is a number, '' is text with no letters, NULL is "no value at all".

## Testing for NULL
Comparisons with NULL never come out true, not even NULL = NULL, because "unknown equals unknown" is itself unknown. So this finds nothing:

    SELECT name FROM heroes WHERE guild_id = NULL;   -- wrong

Use IS NULL and IS NOT NULL:

    SELECT name FROM heroes WHERE guild_id IS NULL;
    SELECT name FROM heroes WHERE guild_id IS NOT NULL;

## NULL in totals
COUNT(guild_id) counts only rows that have a value, while COUNT(*) counts all rows. SUM and AVG skip NULLs too. You will see this in the Aggregates chapter.

## Why NULLs happen
New records often start with some fields unknown, optional fields are left blank, and LEFT JOINs (last chapter) fill in NULL when there is no match."##,
        &["NULL means no value: unknown or missing.", "NULL is not 0 and not empty text.", "Use IS NULL / IS NOT NULL, never = NULL.", "Totals like COUNT(column) skip NULLs."]),
        p("Project: Item Finder", &[
            (r##"CHECKPOINT PROJECT

Build the shop's best filter: cheap potions and tools, under 10 gold, with their prices."##, "potion_red"),
            (r##"SELECT name, price FROM items
WHERE type IN ('potion', 'tool') AND price < 10;"##, "money_bag"),
        ], vec![
            quiz("Which condition finds items that cost 5 to 12 (inclusive)?", &["price BETWEEN 5 AND 12", "price > 5 AND price < 12", "price IN (5, 12)"], 0, "BETWEEN includes both ends."),
            quiz("Why use brackets in type = 'tool' AND (price < 5 OR price > 20)?", &["AND runs before OR, so brackets group the OR", "Brackets are decoration", "SQL requires them always"], 0, "Brackets control the order."),
            fill("Names containing 'ion'.", r##"SELECT name FROM items WHERE name LIKE ___;"##, &["'%ion%'", "'ion'", "'_ion'"], 0, "% on both sides means contains."),
            fill("Potions or tools.", r##"SELECT name FROM items WHERE type ___ ('potion', 'tool');"##, &["IN", "=", "LIKE"], 0, "IN checks a list."),
            quiz("Which items does the project's filter return?", &["Potion, Map and Torch", "Potion and Elixir", "Every item"], 0, "Potions and tools under 10: Elixir costs 12, so it's out."),
            copy("Type the filter.", r##"SELECT name, price FROM items
WHERE type IN ('potion', 'tool') AND price < 10;"##, "Two conditions joined by AND."),
            recall("From memory: show the name of items whose price is between 8 and 30.", r##"SELECT name FROM items WHERE price BETWEEN 8 AND 30;"##, "BETWEEN 8 AND 30."),
        ]).with(r##"## Shop search filters
The shopkeeper's website needs filters, and every filter is a WHERE clause. This project combines everything from the Filtering chapter.

## Build the pieces first
1. Cheap items (under 10 gold):

    SELECT name, price FROM items WHERE price < 10;

2. Potions and tools only:

    SELECT name FROM items WHERE type IN ('potion', 'tool');

## The project: both at once
    SELECT name, price FROM items
    WHERE type IN ('potion', 'tool') AND price < 10;

AND keeps only rows where both conditions are true: Potion (5), Map (8) and Torch (3). The Elixir is a potion, but at 12 gold it's not cheap.

## More filters to try
- Search for a word: WHERE name LIKE '%or%'.
- A price range: WHERE price BETWEEN 8 AND 30.

## Check each filter
Look at the full table first (SELECT * FROM items) and predict which rows each filter should return, THEN run it. Comparing your prediction with the result is the fastest way to catch a wrong operator or a missing bracket."##,
        &["Every search filter is a WHERE condition.", "IN checks a list of values.", "AND keeps rows where both conditions are true.", "Predict the result before running the query."]),
    ])
}

fn sorting() -> Module {
    ch("Sorting", "Order results, keep only the top rows, and calculate new columns.", vec![
        l("ORDER BY", &[
            (r#"ORDER BY sorts the result:

SELECT name, hp FROM heroes ORDER BY hp;

ASC = smallest first (default)
DESC = largest first"#, "crown"),
            (r#"Sort by several columns:

ORDER BY class, hp DESC"#, "medallion"),
        ], vec![
            quiz("What is the default sort direction?", &["ASC (smallest first)", "DESC (largest first)", "Random"], 0, "ASC is the default."),
            quiz("Where does ORDER BY go?", &["After WHERE", "Before FROM", "Right after SELECT"], 0, "FROM, WHERE, then ORDER BY."),
            fill("Strongest first.", "SELECT name FROM heroes ORDER BY hp ___;", &["DESC", "ASC", "DOWN"], 0, "DESC is largest first."),
            fill("Sort by name.", "SELECT name FROM heroes ___ BY name;", &["ORDER", "SORT", "GROUP"], 0, "ORDER BY column."),
            copy("Type this query.", "SELECT name, price FROM items ORDER BY price DESC;", "Most expensive first."),
            recall("From memory: show hero names sorted by level, lowest first.", "SELECT name FROM heroes ORDER BY level;", "ORDER BY level (ASC is default)."),
        ]).with(r##"## Putting results in order
Without ORDER BY, a database may return rows in any order. When order matters, say so:

    SELECT name, hp FROM heroes ORDER BY hp;         -- weakest first
    SELECT name, hp FROM heroes ORDER BY hp DESC;    -- strongest first

- ASC (ascending) goes from small to large, A to Z. It is the default.
- DESC (descending) goes from large to small, Z to A.

## Sorting by several columns
    SELECT name, class, hp FROM heroes ORDER BY class, hp DESC;

First by class (A to Z); heroes of the same class are then sorted by hp, strongest first. Each column has its own direction.

## Where it goes
    SELECT ... FROM ... WHERE ... ORDER BY ...;

ORDER BY comes after WHERE: first pick the rows, then sort them.

## Sorting by something not shown
You can sort by a column you did not select: SELECT name FROM heroes ORDER BY level DESC; shows only names, but in level order.

## NULLs
Rows with NULL usually sort first in ascending order (they count as "smallest") in Code Forge's database. Other databases may differ."##,
        &["ORDER BY column sorts the result.", "ASC (default) is small to large; DESC is large to small.", "Several columns: ORDER BY a, b DESC.", "ORDER BY comes after WHERE."]),
        l("LIMIT", &[
            (r#"LIMIT keeps only the first rows:

SELECT * FROM heroes LIMIT 3;"#, "hourglass"),
            (r#"With ORDER BY: the top 3

SELECT name FROM heroes
ORDER BY hp DESC LIMIT 3;"#, "gold_trophy"),
        ], vec![
            quiz("What does LIMIT 2 do?", &["Keeps only 2 rows", "Skips 2 rows", "Shows 2 columns"], 0, "LIMIT caps the number of rows."),
            quiz("To get the 3 strongest heroes, you need...", &["ORDER BY hp DESC LIMIT 3", "LIMIT 3 only", "ORDER BY hp LIMIT 3"], 0, "Sort first, then cut."),
            fill("Keep only one row.", "SELECT name FROM heroes ORDER BY level DESC ___ 1;", &["LIMIT", "TOP", "FIRST"], 0, "LIMIT n."),
            fill("Cheapest item.", "SELECT name FROM items ORDER BY price ___ LIMIT 1;", &["ASC", "DESC", "UP"], 0, "Smallest price first."),
            copy("Type this query.", "SELECT name, price FROM items ORDER BY price DESC LIMIT 2;", "The two priciest items."),
            recall("From memory: show the name of the hero with the highest hp.", "SELECT name FROM heroes ORDER BY hp DESC LIMIT 1;", "Sort DESC, LIMIT 1."),
        ]).with(r##"## Only the first few rows
LIMIT caps how many rows come back:

    SELECT * FROM heroes LIMIT 3;

## Top lists
LIMIT is most useful after ORDER BY. Together they answer "top" and "bottom" questions:

    -- the three strongest heroes
    SELECT name, hp FROM heroes ORDER BY hp DESC LIMIT 3;

    -- the cheapest item
    SELECT name, price FROM items ORDER BY price LIMIT 1;

Without ORDER BY, LIMIT 3 gives "some" three rows, not the best three.

## The full order of parts
    SELECT columns
    FROM table
    WHERE condition
    ORDER BY column
    LIMIT n;

This order is fixed. Putting LIMIT before WHERE is an error, and Code Forge's message reminds you of the order.

## Previews
SELECT * FROM table LIMIT 5 is the classic way to peek at a huge table: you see the columns and a few example rows without waiting for millions of results.

## Ties
If two heroes tie for the top spot, LIMIT 1 shows only one of them. Add a second sort column (ORDER BY hp DESC, name) to make the result predictable."##,
        &["LIMIT n keeps only the first n rows.", "ORDER BY + LIMIT answers top/bottom questions.", "Order of parts: SELECT, FROM, WHERE, ORDER BY, LIMIT.", "Add a second sort column to break ties."]),
        l("Calculated Columns", &[
            (r#"SELECT can calculate:

SELECT name, price * 2 AS double_price
FROM items;"#, "coins"),
            (r#"Operators: + - * / %

Functions: UPPER(name), LOWER(name), LENGTH(name), ROUND(x, 1)"#, "gear"),
        ], vec![
            quiz("Does SELECT price * 2 change the stored prices?", &["No, it only calculates for the result", "Yes"], 0, "Queries never change data."),
            quiz("Why add AS to a calculation?", &["To give the column a readable header", "To save it", "It is required"], 0, "Otherwise the header is the formula."),
            fill("Show each price with a 10 gold tax added.", "SELECT name, price ___ 10 AS with_tax FROM items;", &["+", "&", "AND"], 0, "+ adds."),
            fill("Shout the names.", "SELECT ___(name) FROM heroes;", &["UPPER", "LOUD", "CAPS"], 0, "UPPER makes capitals."),
            copy("Type this query.", "SELECT name, hp * 10 AS hp_x10 FROM heroes;", "A calculated column."),
            recall("From memory: show each item's name and its price divided by 2 as half_price.", "SELECT name, price / 2 AS half_price FROM items;", "price / 2 AS half_price."),
        ]).with(r##"## Calculating in a query
SELECT can do more than copy columns. It can calculate new values for every row:

    SELECT name, price, price * 2 AS double_price FROM items;
    SELECT name, hp + level AS power FROM heroes;

The maths operators are + - * / and % (remainder). The stored data never changes; the new column exists only in the result. Use AS to give it a readable name.

## Text functions
    SELECT UPPER(name) FROM heroes;      -- ARIA
    SELECT LOWER(name) FROM heroes;      -- aria
    SELECT LENGTH(name) FROM heroes;     -- 4

## Rounding
    SELECT ROUND(price / 3, 1) AS third FROM items;

## Calculations in WHERE and ORDER BY
You can calculate anywhere a value is allowed:

    SELECT name FROM heroes WHERE hp * 2 > 25;
    SELECT name FROM heroes ORDER BY hp + level DESC;

## Joining text
|| glues text: SELECT name || ' the ' || class FROM heroes; gives "Aria the mage"."##,
        &["SELECT can calculate new columns with + - * / %.", "Calculations never change the stored data.", "Name calculated columns with AS.", "UPPER, LOWER, LENGTH and ROUND are handy functions."]),
        p("Project: Leaderboard", &[
            (r##"CHECKPOINT PROJECT

Build the guild leaderboard: a power score (hp + level * 5), sorted from best to worst, top 3 only."##, "gold_trophy"),
            (r##"SELECT name, hp + level * 5 AS power
FROM heroes
ORDER BY power DESC
LIMIT 3;"##, "banner_royal"),
        ], vec![
            quiz("In hp + level * 5, what happens first?", &["level * 5", "hp + level", "Left to right"], 0, "Multiplication before addition."),
            quiz("Can ORDER BY use the alias power?", &["Yes", "No"], 0, "ORDER BY can use output aliases."),
            fill("Best first.", r##"ORDER BY power ___"##, &["DESC", "ASC", "TOP"], 0, "DESC puts the largest first."),
            fill("Only the top 3.", r##"___ 3;"##, &["LIMIT", "TOP", "MAX"], 0, "LIMIT keeps the first rows."),
            quiz("Who tops the leaderboard?", &["Dara, with 43", "Borin, with 35", "Aria, with 23"], 0, "Dara: 18 + 5 * 5 = 43."),
            copy("Type the leaderboard.", r##"SELECT name, hp + level * 5 AS power
FROM heroes
ORDER BY power DESC
LIMIT 3;"##, "Calculate, sort, limit."),
            recall("From memory: show the 2 heroes with the highest level, name and level only.", r##"SELECT name, level FROM heroes ORDER BY level DESC LIMIT 2;"##, "ORDER BY level DESC LIMIT 2."),
        ]).with(r##"## A real leaderboard
Every game has a leaderboard. Yours ranks heroes by a power score that the database calculates.

## The rules
- power = hp + level * 5
- Show the name and power.
- Best first, top three only.

## The query
    SELECT name, hp + level * 5 AS power
    FROM heroes
    ORDER BY power DESC
    LIMIT 3;

## How it runs
Think of the database working in this order:
1. FROM heroes: take all rows.
2. SELECT: calculate power for each row.
3. ORDER BY power DESC: sort by the new column.
4. LIMIT 3: keep the first three.

That is why ORDER BY can use the alias power even though it is defined in SELECT.

## Check the maths
Pick one hero and calculate their power by hand. Dara: 18 + 5 * 5 = 43. Does the query agree?"##,
        &["Calculate a score in SELECT and name it with AS.", "ORDER BY can sort by that alias.", "LIMIT keeps the top rows after sorting.", "Check one row by hand to verify the formula."]),
    ])
}

fn aggregates() -> Module {
    ch("Aggregates", "Summarise many rows into totals with COUNT, SUM, AVG, MIN, MAX, GROUP BY and HAVING.", vec![
        l("COUNT and SUM", &[
            (r#"Aggregate functions turn many rows into one value:

SELECT COUNT(*) FROM heroes;
SELECT SUM(price) FROM items;"#, "coins"),
            (r#"COUNT(*) counts rows.
COUNT(column) counts non-NULL values.
SUM adds numbers."#, "money_bag"),
        ], vec![
            quiz("What does COUNT(*) return?", &["The number of rows", "The sum of all numbers", "The first row"], 0, "COUNT(*) counts rows."),
            quiz("COUNT(guild_id) skips which rows?", &["Rows where guild_id is NULL", "Rows with guild_id 1", "None"], 0, "COUNT(column) ignores NULL."),
            fill("Count the items.", "SELECT ___(*) FROM items;", &["COUNT", "SUM", "TOTAL"], 0, "COUNT counts rows."),
            fill("Total value of all items.", "SELECT ___(price) FROM items;", &["SUM", "COUNT", "ADD"], 0, "SUM adds."),
            copy("Type this query.", "SELECT COUNT(*) FROM heroes WHERE class = 'knight';", "How many knights."),
            recall("From memory: show the total reward of all quests.", "SELECT SUM(reward) FROM quests;", "SUM(reward)."),
        ]).with(r##"## From many rows to one answer
So far every query returned rows. Aggregate functions summarise many rows into a single value.

## COUNT
    SELECT COUNT(*) FROM heroes;            -- 6 heroes
    SELECT COUNT(guild_id) FROM heroes;     -- 5, Finn's NULL is skipped
    SELECT COUNT(DISTINCT class) FROM heroes;   -- 4 different classes

COUNT(*) counts rows; COUNT(column) counts rows where that column has a value.

## SUM
    SELECT SUM(price) FROM items;           -- total value of the shop
    SELECT SUM(reward) FROM quests;

SUM only works on numbers and skips NULLs.

## With WHERE
WHERE filters first, then the aggregate runs on what is left:

    SELECT COUNT(*) FROM heroes WHERE class = 'knight';   -- 2

## Name your totals
The header of COUNT(*) is literally "COUNT(*)". Use AS for a clearer result:

    SELECT COUNT(*) AS knights FROM heroes WHERE class = 'knight';

## Questions they answer
How many players signed up today? How much gold did the shop earn? How many quests are still open? Every dashboard is built from queries like these."##,
        &["Aggregates turn many rows into one value.", "COUNT(*) counts rows; COUNT(col) skips NULLs.", "SUM adds a numeric column.", "WHERE filters before the aggregate runs."]),
        l("AVG, MIN and MAX", &[
            (r#"AVG  average
MIN  smallest
MAX  largest

SELECT AVG(hp) FROM heroes;"#, "medallion"),
            (r#"Several at once:

SELECT MIN(price), MAX(price) FROM items;"#, "crystal_purple"),
        ], vec![
            quiz("Which function gives the average?", &["AVG", "MEAN", "MID"], 0, "AVG(column)."),
            quiz("MIN(name) on text gives...", &["The first name alphabetically", "An error", "The shortest name"], 0, "MIN/MAX work on text too."),
            fill("Most expensive price.", "SELECT ___(price) FROM items;", &["MAX", "TOP", "HIGH"], 0, "MAX is largest."),
            fill("Average level.", "SELECT ___(level) FROM heroes;", &["AVG", "AVERAGE", "SUM"], 0, "AVG."),
            copy("Type this query.", "SELECT MIN(hp), MAX(hp) FROM heroes;", "Weakest and strongest hp."),
            recall("From memory: show the average reward of quests as avg_reward.", "SELECT AVG(reward) AS avg_reward FROM quests;", "AVG with an alias."),
        ]).with(r##"## More summaries
    SELECT AVG(hp) FROM heroes;        -- average hit points
    SELECT MIN(price) FROM items;      -- cheapest price
    SELECT MAX(level) FROM heroes;     -- highest level

You can ask for several in one query:

    SELECT MIN(price) AS cheapest, MAX(price) AS priciest, AVG(price) AS average
    FROM items;

## Text and dates
MIN and MAX also work on text (alphabetical order) and dates (earliest/latest): MIN(name) gives Aria.

## Which row was it?
MAX(hp) tells you the highest hp, but not WHO has it. Adding name next to it is not allowed without GROUP BY (next lesson). To find the hero, use ORDER BY hp DESC LIMIT 1 instead.

## NULLs are skipped
AVG adds the non-NULL values and divides by how many there are. A missing value does not count as zero.

## Rounding averages
Averages often have long decimals; wrap them: ROUND(AVG(price), 1)."##,
        &["AVG gives the average; MIN and MAX the smallest and largest.", "Several aggregates can share one SELECT.", "MIN/MAX work on text too.", "To find WHICH row has the max, use ORDER BY ... LIMIT 1."]),
        l("GROUP BY", &[
            (r#"GROUP BY makes one total per group:

SELECT class, COUNT(*)
FROM heroes
GROUP BY class;"#, "banner"),
            (r#"Every column next to a total must be in GROUP BY."#, "shield"),
        ], vec![
            quiz("What does GROUP BY class do?", &["Makes one result row per class", "Sorts by class", "Deletes duplicate classes"], 0, "One group per value."),
            quiz("SELECT name, COUNT(*) FROM heroes GROUP BY class is...", &["An error: name is not grouped", "Fine", "The same as without GROUP BY"], 0, "Non-total columns must be grouped."),
            fill("One count per class.", "SELECT class, COUNT(*) FROM heroes GROUP ___ class;", &["BY", "ON", "IN"], 0, "GROUP BY column."),
            fill("Total price per type.", "SELECT type, ___(price) FROM items GROUP BY type;", &["SUM", "GROUP", "ALL"], 0, "SUM per group."),
            copy("Type this query.", "SELECT class, AVG(hp) FROM heroes GROUP BY class;", "Average hp per class."),
            recall("From memory: count the items of each type.", "SELECT type, COUNT(*) FROM items GROUP BY type;", "GROUP BY type."),
        ]).with(r##"## Totals per group
COUNT(*) alone gives one number for the whole table. GROUP BY splits the rows into groups first and gives one result row per group:

    SELECT class, COUNT(*) AS heroes
    FROM heroes
    GROUP BY class;

    class   | heroes
    knight  | 2
    mage    | 2
    rogue   | 1
    ranger  | 1

## How it works
1. Rows with the same class are put in the same pile.
2. The aggregate (COUNT, SUM, AVG...) runs on each pile.
3. Each pile becomes one row in the result.

## The golden rule
Every column in SELECT must either be inside an aggregate or listed in GROUP BY. SELECT name, COUNT(*) ... GROUP BY class fails, because one class has several names: which one should it show?

## Sorting groups
    SELECT type, SUM(price) AS total
    FROM items
    GROUP BY type
    ORDER BY total DESC;

## Order of parts
SELECT, FROM, WHERE, GROUP BY, ORDER BY, LIMIT. WHERE filters rows BEFORE they are grouped."##,
        &["GROUP BY column makes one result row per value.", "Aggregates run separately on each group.", "Every selected column must be aggregated or grouped.", "WHERE filters before grouping; ORDER BY sorts the groups."]),
        l("HAVING", &[
            (r#"HAVING filters groups (after totals):

SELECT class, COUNT(*)
FROM heroes
GROUP BY class
HAVING COUNT(*) > 1;"#, "keyhole"),
            (r#"WHERE filters rows before grouping.
HAVING filters groups after."#, "hourglass"),
        ], vec![
            quiz("Which filters groups by their totals?", &["HAVING", "WHERE", "LIMIT"], 0, "HAVING works after GROUP BY."),
            quiz("Can WHERE use COUNT(*)?", &["No, use HAVING", "Yes"], 0, "Totals don't exist yet at WHERE time."),
            fill("Only classes with more than one hero.", "SELECT class FROM heroes GROUP BY class ___ COUNT(*) > 1;", &["HAVING", "WHERE", "IF"], 0, "HAVING filters groups."),
            fill("Types whose total price is over 20.", "SELECT type, SUM(price) FROM items GROUP BY type HAVING ___(price) > 20;", &["SUM", "COUNT", "MAX"], 0, "Filter on the total."),
            copy("Type this query.", "SELECT type, COUNT(*) FROM items GROUP BY type HAVING COUNT(*) >= 2;", "Types with 2+ items."),
            recall("From memory: show classes whose average hp is above 10.", "SELECT class FROM heroes GROUP BY class HAVING AVG(hp) > 10;", "HAVING AVG(hp) > 10."),
        ]).with(r##"## Filtering groups
WHERE filters individual rows. But what if you want to filter by a total, such as "classes with more than one hero"? The total does not exist yet when WHERE runs. HAVING filters the groups AFTER the totals are calculated:

    SELECT class, COUNT(*) AS members
    FROM heroes
    GROUP BY class
    HAVING COUNT(*) > 1;

## WHERE and HAVING together
    SELECT type, AVG(price) AS avg_price
    FROM items
    WHERE price > 4
    GROUP BY type
    HAVING AVG(price) > 10;

1. WHERE drops rows with price 4 or less.
2. GROUP BY makes piles by type.
3. HAVING keeps only piles whose average is above 10.

## A simple rule
- Condition about one row (price > 4)? WHERE.
- Condition about a total (COUNT, SUM, AVG...)? HAVING.

Writing WHERE COUNT(*) > 1 is an error, and Code Forge's message points you to HAVING.

## Order of parts
SELECT, FROM, WHERE, GROUP BY, HAVING, ORDER BY, LIMIT."##,
        &["HAVING filters groups after the totals are calculated.", "WHERE filters rows before grouping.", "Conditions on totals go in HAVING.", "Order: WHERE, GROUP BY, HAVING, ORDER BY, LIMIT."]),
        p("Project: Guild Report", &[
            (r##"CHECKPOINT PROJECT

The guild council wants a report: how many heroes each class has, their average hp, and only classes with at least 2 members."##, "banner_royal"),
            (r##"SELECT class, COUNT(*) AS members, AVG(hp) AS avg_hp
FROM heroes
GROUP BY class
HAVING COUNT(*) >= 2;"##, "book"),
        ], vec![
            quiz("Where does the 'at least 2 members' rule go?", &["HAVING COUNT(*) >= 2", "WHERE COUNT(*) >= 2", "LIMIT 2"], 0, "It's about a group's total, so it goes in HAVING."),
            quiz("Why use AS members?", &["A readable header instead of COUNT(*)", "It counts faster", "It is required"], 0, "Readable reports."),
            fill("Group the heroes by class.", r##"___ BY class"##, &["GROUP", "ORDER", "SORT"], 0, "GROUP BY makes one group per class."),
            fill("Average hp per class.", r##"___(hp) AS avg_hp"##, &["AVG", "SUM", "MID"], 0, "AVG gives the average."),
            quiz("Which classes appear in the report?", &["mage and knight", "All four classes", "rogue and ranger"], 0, "Only mage and knight have 2 members."),
            copy("Type the report.", r##"SELECT class, COUNT(*) AS members, AVG(hp) AS avg_hp
FROM heroes
GROUP BY class
HAVING COUNT(*) >= 2;"##, "Group, total, filter."),
            recall("From memory: show each item type with its most expensive price as top_price.", r##"SELECT type, MAX(price) AS top_price FROM items GROUP BY type;"##, "MAX per type."),
        ]).with(r##"## A council report
Reports with totals per category are the heart of business data: sales per month, players per level, gold per guild. The council asks:

- For each class: how many heroes, and their average hp.
- Only classes with at least two members.

## The query
    SELECT class, COUNT(*) AS members, AVG(hp) AS avg_hp
    FROM heroes
    GROUP BY class
    HAVING COUNT(*) >= 2;

## Read it in running order
FROM heroes, then GROUP BY class, then COUNT and AVG are calculated for each group, then HAVING keeps the big groups, and finally SELECT shapes the columns.

## WHERE or HAVING?
WHERE filters single rows before grouping; HAVING filters whole groups after their totals are known. "At least 2 members" is about a group, so it needs HAVING.

## Extend it
- Sort the biggest classes first: ORDER BY members DESC.
- Add the strongest hp per class with MAX(hp).

## Check
Count the knights and mages yourself in SELECT * FROM heroes. Do the numbers match the report?"##,
        &["Reports usually group rows and total each group.", "HAVING filters groups by their totals.", "Aliases make reports readable.", "Verify totals against the raw data."]),
    ])
}

fn changing() -> Module {
    ch("Changing Data", "Add, change and remove rows with INSERT, UPDATE and DELETE.", vec![
        l("INSERT", &[
            (r#"INSERT adds a new row:

INSERT INTO items (name, type, price)
VALUES ('Rope', 'tool', 4);"#, "chest"),
            (r#"Columns and values match by position. Text in quotes."#, "map"),
        ], vec![
            quiz("Which statement adds rows?", &["INSERT", "UPDATE", "ADD"], 0, "INSERT INTO ... VALUES ..."),
            quiz("In INSERT INTO items (name, price) VALUES ('Rope', 4), what is price?", &["4", "Rope", "NULL"], 0, "Values match columns by position."),
            fill("Start the insert.", "___ INTO items (name, type, price) VALUES ('Rope', 'tool', 4);", &["INSERT", "ADD", "PUT"], 0, "INSERT INTO."),
            fill("Introduce the values.", "INSERT INTO items (name, type, price) ___ ('Bow', 'weapon', 20);", &["VALUES", "DATA", "WITH"], 0, "VALUES (...)."),
            copy("Type this.", "INSERT INTO items (name, type, price) VALUES ('Rope', 'tool', 4);", "One new item."),
            recall("From memory: add a guild named Sky Watch in the city Cloudreach (columns name, city).", "INSERT INTO guilds (name, city) VALUES ('Sky Watch', 'Cloudreach');", "Two columns, two values."),
        ]).with(r##"## Adding new rows
Querying reads data. INSERT writes new data:

    INSERT INTO items (name, type, price)
    VALUES ('Rope', 'tool', 4);

- INSERT INTO names the table.
- The brackets list the columns you are filling.
- VALUES gives the values, in the same order as the columns.
- Text needs quotes; numbers do not.

## Columns you skip
Columns you leave out get NULL (or a default value the table defines). Above, id is skipped, so it is NULL in Code Forge's simple database; real databases usually fill in the next id automatically.

## Many rows at once
    INSERT INTO items (name, type, price)
    VALUES ('Rope', 'tool', 4),
           ('Bow', 'weapon', 20);

## Without a column list
INSERT INTO guilds VALUES (4, 'Sky Watch', 'Cloudreach'); works only if you give a value for EVERY column in the table's order. Listing columns is safer and clearer.

## See the change
In Code Forge each run starts from a fresh copy of the database, so add a SELECT after your INSERT in the same answer to see the new row:

    INSERT INTO items (name, type, price) VALUES ('Rope', 'tool', 4);
    SELECT * FROM items;"##,
        &["INSERT INTO table (columns) VALUES (values) adds a row.", "Values match the columns by position.", "Skipped columns become NULL (or a default).", "Several rows can be inserted at once, separated by commas."]),
        l("UPDATE", &[
            (r#"UPDATE changes existing rows:

UPDATE items SET price = 6
WHERE name = 'Potion';"#, "hammer"),
            (r#"Without WHERE, EVERY row changes!

SET can use the old value:
SET price = price + 1"#, "flame"),
        ], vec![
            quiz("What happens to UPDATE items SET price = 0 without WHERE?", &["Every item's price becomes 0", "Nothing", "Only the first item changes"], 0, "Always check your WHERE."),
            quiz("Which keyword names the new values?", &["SET", "VALUES", "TO"], 0, "UPDATE table SET col = value."),
            fill("Change values.", "UPDATE heroes ___ hp = 20 WHERE name = 'Aria';", &["SET", "VALUES", "TO"], 0, "SET column = value."),
            fill("Raise every potion's price by 1.", "UPDATE items SET price = price ___ 1 WHERE type = 'potion';", &["+", "AND", "&"], 0, "Use the old value plus 1."),
            copy("Type this.", "UPDATE items SET price = 6 WHERE name = 'Potion';", "Only the Potion changes."),
            recall("From memory: set Elm's level to 2.", "UPDATE heroes SET level = 2 WHERE name = 'Elm';", "UPDATE ... SET ... WHERE."),
        ]).with(r##"## Changing existing rows
    UPDATE items
    SET price = 6
    WHERE name = 'Potion';

- UPDATE names the table.
- SET lists the new values: column = value.
- WHERE chooses which rows to change.

## The most dangerous missing line
Without WHERE, UPDATE changes EVERY row:

    UPDATE items SET price = 0;   -- every item is now free!

Professionals often run the WHERE part as a SELECT first (SELECT * FROM items WHERE name = 'Potion') to see exactly which rows will change, then turn it into the UPDATE.

## Using the old value
    UPDATE items SET price = price + 1 WHERE type = 'potion';
    UPDATE heroes SET hp = hp * 2, level = level + 1 WHERE name = 'Elm';

Separate several changes with commas.

## Picking exactly one row
Names can repeat; ids never do. Real apps usually update by id: WHERE id = 3.

## Check
Follow every UPDATE with a SELECT to confirm the change:

    UPDATE heroes SET level = 2 WHERE name = 'Elm';
    SELECT name, level FROM heroes;"##,
        &["UPDATE table SET col = value WHERE condition changes rows.", "Without WHERE, every row changes.", "SET can use old values: price = price + 1.", "Test the WHERE with a SELECT first; update by id when possible."]),
        l("DELETE", &[
            (r#"DELETE removes rows:

DELETE FROM items WHERE name = 'Torch';"#, "axe"),
            (r#"DELETE FROM items; with no WHERE removes EVERY row.

There is no undo button!"#, "flame"),
        ], vec![
            quiz("What does DELETE FROM heroes; do?", &["Removes every hero", "Removes the table's first row", "Nothing without WHERE"], 0, "No WHERE means all rows."),
            quiz("Does DELETE remove the table itself?", &["No, only rows", "Yes"], 0, "DROP TABLE removes tables."),
            fill("Remove rows.", "___ FROM items WHERE price < 4;", &["DELETE", "REMOVE", "DROP"], 0, "DELETE FROM."),
            fill("Only remove the Torch.", "DELETE FROM items ___ name = 'Torch';", &["WHERE", "IF", "SET"], 0, "WHERE picks the rows."),
            copy("Type this.", "DELETE FROM quests WHERE reward < 10;", "Removes small quests."),
            recall("From memory: delete the hero named Elm.", "DELETE FROM heroes WHERE name = 'Elm';", "DELETE FROM ... WHERE."),
        ]).with(r##"## Removing rows
    DELETE FROM items WHERE name = 'Torch';

- DELETE FROM names the table.
- WHERE chooses which rows disappear.

## No WHERE, no rows
    DELETE FROM items;

removes EVERY row. The table still exists, but it is empty. In a real database there is no undo, which is why careful teams keep backups and test their WHERE with a SELECT first.

## DELETE versus DROP
- DELETE removes rows; the table and its columns stay.
- DROP TABLE items; removes the whole table, structure and all.

## Soft deletes
Many apps never really delete. Instead they UPDATE a column like is_deleted = 1 and hide those rows with WHERE is_deleted = 0. That way, mistakes can be undone.

## Check your work
    DELETE FROM quests WHERE reward < 10;
    SELECT * FROM quests;

The Lost Cat quest (reward 5) should be gone."##,
        &["DELETE FROM table WHERE condition removes rows.", "Without WHERE, every row is removed.", "DELETE keeps the table; DROP TABLE removes it.", "Test with SELECT first; many apps use soft deletes."]),
        p("Project: Shop Restock", &[
            (r##"CHECKPOINT PROJECT

Restock the shop: add a lantern, raise tool prices, remove the torches nobody buys, then show the shop."##, "money_bag"),
            (r##"INSERT INTO items (name, type, price) VALUES ('Lantern', 'tool', 9);
UPDATE items SET price = price + 2 WHERE type = 'tool';
DELETE FROM items WHERE name = 'Torch';
SELECT * FROM items;"##, "lantern"),
        ], vec![
            quiz("In what order should you check your changes?", &["Run the change, then SELECT to see the result", "SELECT never helps", "Changes show automatically"], 0, "Always confirm with a SELECT."),
            quiz("Which statement would empty the whole shop by mistake?", &["DELETE FROM items;", "DELETE FROM items WHERE price > 100;", "SELECT * FROM items;"], 0, "No WHERE means every row."),
            fill("Add a new item.", r##"___ INTO items (name, type, price) VALUES ('Lantern', 'tool', 9);"##, &["INSERT", "UPDATE", "ADD"], 0, "INSERT adds rows."),
            fill("Raise tool prices.", r##"UPDATE items ___ price = price + 2 WHERE type = 'tool';"##, &["SET", "VALUES", "TO"], 0, "SET gives the new value."),
            quiz("What does the lantern cost at the end?", &["11", "9", "13"], 0, "It was inserted at 9 before tool prices went up by 2."),
            copy("Type the restock.", r##"INSERT INTO items (name, type, price) VALUES ('Lantern', 'tool', 9);
UPDATE items SET price = price + 2 WHERE type = 'tool';
DELETE FROM items WHERE name = 'Torch';
SELECT * FROM items;"##, "Insert, update, delete, then look."),
            recall("From memory: delete every item of type armor, then show all items.", r##"DELETE FROM items WHERE type = 'armor';
SELECT * FROM items;"##, "Delete, then select."),
        ]).with(r##"## Running the shop
The shopkeeper hands you a list of changes. Each change is one statement; together they restock the shop.

## The changes
    INSERT INTO items (name, type, price) VALUES ('Lantern', 'tool', 9);
    UPDATE items SET price = price + 2 WHERE type = 'tool';
    DELETE FROM items WHERE name = 'Torch';
    SELECT * FROM items;

1. New stock arrives: a lantern. The id is left out, so the database numbers it for you.
2. Tools are in demand, so every tool costs 2 more.
3. Nobody buys torches any more, so they go.
4. Show the shop to check the result.

## Order matters
Statements run one after another, each seeing the results of the ones before. The lantern was inserted BEFORE the price rise, so it costs 11, not 9. Swap the first two lines and it would stay at 9.

## Think before you change
For each UPDATE and DELETE, ask: "which rows will this touch?" Run the WHERE as a SELECT first if you are unsure. One missing WHERE can change every row."##,
        &["INSERT, UPDATE and DELETE each change data in one way.", "Statements run in order, each seeing earlier changes.", "Check which rows a WHERE touches before changing them.", "Finish with a SELECT to confirm the result."]),
    ])
}

fn building() -> Module {
    ch("Building Tables", "Design your own tables with CREATE TABLE, data types, keys and ALTER TABLE.", vec![
        l("CREATE TABLE", &[
            (r#"Make a new table:

CREATE TABLE pets (
  id INTEGER,
  name TEXT,
  age INTEGER
);"#, "cauldron"),
            (r#"Each column has a name and a type. Then INSERT rows into it."#, "potion_magic"),
        ], vec![
            quiz("What does CREATE TABLE make?", &["A new, empty table", "A new row", "A copy of heroes"], 0, "The table starts empty."),
            quiz("What separates column definitions?", &["Commas", "Semicolons", "AND"], 0, "Commas between columns."),
            fill("Make a table.", "___ TABLE pets (name TEXT, age INTEGER);", &["CREATE", "MAKE", "NEW"], 0, "CREATE TABLE."),
            fill("Give the column a type.", "CREATE TABLE pets (name ___);", &["TEXT", "'TEXT'", "WORDS"], 0, "TEXT stores words."),
            copy("Type this.", "CREATE TABLE pets (id INTEGER, name TEXT, age INTEGER);", "Three columns."),
            recall("From memory: create a table spells with columns name TEXT and power INTEGER.", "CREATE TABLE spells (name TEXT, power INTEGER);", "CREATE TABLE with two columns."),
        ]).with(r##"## Designing a table
Before data can be stored, someone has to create the table and decide its columns:

    CREATE TABLE pets (
      id INTEGER,
      name TEXT,
      species TEXT,
      age INTEGER
    );

- CREATE TABLE gives the new table a name.
- Inside the brackets, each column has a name and a data type.
- Columns are separated by commas.

The new table is empty. Fill it with INSERT:

    INSERT INTO pets VALUES (1, 'Pip', 'owl', 3);
    SELECT * FROM pets;

## Naming
Table names are usually plural nouns (pets, quests), column names are short and lowercase with underscores (owner_id). Choose names a stranger would understand.

## Plan first
Ask: what is one row? (One pet.) What do I need to know about each one? (Name, species, age.) Each answer becomes a column.

## Already exists
Creating a table whose name is taken is an error. Real projects use CREATE TABLE IF NOT EXISTS, or DROP TABLE first when rebuilding."##,
        &["CREATE TABLE name (column TYPE, ...) makes an empty table.", "Each column has a name and a type, separated by commas.", "Fill the table with INSERT, check it with SELECT.", "Plan: one row is one thing; each fact about it is a column."]),
        l("Data Types and Keys", &[
            (r#"Common types:

INTEGER  whole numbers
REAL     decimals
TEXT     words
BOOLEAN  true/false (often 1/0)"#, "gear"),
            (r#"A PRIMARY KEY identifies each row uniquely:

id INTEGER PRIMARY KEY

NOT NULL means a value is required."#, "key"),
        ], vec![
            quiz("Which type stores a price like 4.50?", &["REAL", "INTEGER", "TEXT"], 0, "REAL holds decimals."),
            quiz("What is a primary key for?", &["Identifying each row uniquely", "Sorting rows", "Encrypting data"], 0, "No two rows share a primary key."),
            fill("Make id the unique identifier.", "CREATE TABLE pets (id INTEGER PRIMARY ___, name TEXT);", &["KEY", "ID", "UNIQUE"], 0, "PRIMARY KEY."),
            fill("Require a name.", "CREATE TABLE pets (id INTEGER, name TEXT NOT ___);", &["NULL", "EMPTY", "NONE"], 0, "NOT NULL."),
            copy("Type this.", "CREATE TABLE pets (id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER);", "Key and required name."),
            recall("From memory: create a table towns with id INTEGER PRIMARY KEY and name TEXT.", "CREATE TABLE towns (id INTEGER PRIMARY KEY, name TEXT);", "A key column and a name."),
        ]).with(r##"## Data types
Each column stores one kind of value. Common types:

    INTEGER   whole numbers: 3, -10, 2026
    REAL      decimals: 4.5, 0.25
    TEXT      words and sentences
    BOOLEAN   true/false (many databases store 1 and 0)
    DATE      dates like '2026-11-17'

Choosing the right type helps the database sort correctly (numbers by value, not alphabetically) and catch mistakes.

## Primary keys
A primary key is a column whose value is unique for every row and never empty, usually id:

    CREATE TABLE pets (
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULL,
      age INTEGER
    );

Two pets can both be called Pip, but they will never share an id. Other tables point to rows by their id (heroes.guild_id points to guilds.id); that connection is what joins use in the last chapter. A column that points to another table's key is called a foreign key.

## NOT NULL
NOT NULL makes a column required: inserting a row without a name would be refused by a full database.

## Code Forge's database
Our practice engine reads these words but does not enforce every rule. Real databases like SQLite, PostgreSQL and MySQL enforce them strictly."##,
        &["Pick a type per column: INTEGER, REAL, TEXT, BOOLEAN, DATE.", "A PRIMARY KEY uniquely identifies each row.", "NOT NULL makes a value required.", "Columns that point at another table's key are foreign keys."]),
        l("ALTER TABLE", &[
            (r#"Add a column to an existing table:

ALTER TABLE items ADD COLUMN stock INTEGER;"#, "hammer"),
            (r#"Existing rows get NULL in the new column until you UPDATE them."#, "gear"),
        ], vec![
            quiz("What value do existing rows get in a newly added column?", &["NULL", "0", "The column name"], 0, "Until you update them."),
            quiz("Which statement changes a table's structure?", &["ALTER TABLE", "UPDATE", "INSERT"], 0, "ALTER changes the table; UPDATE changes rows."),
            fill("Change the table's structure.", "___ TABLE items ADD COLUMN stock INTEGER;", &["ALTER", "UPDATE", "CHANGE"], 0, "ALTER TABLE."),
            fill("Add a column.", "ALTER TABLE heroes ___ COLUMN title TEXT;", &["ADD", "NEW", "PUT"], 0, "ADD COLUMN."),
            copy("Type this.", "ALTER TABLE items ADD COLUMN stock INTEGER;", "A new column."),
            recall("From memory: add a column motto of type TEXT to guilds.", "ALTER TABLE guilds ADD COLUMN motto TEXT;", "ALTER TABLE ... ADD COLUMN ..."),
        ]).with(r##"## Changing a table's shape
Needs change. The shop now wants to track stock. Instead of rebuilding the table, add a column:

    ALTER TABLE items ADD COLUMN stock INTEGER;

The word COLUMN is optional in many databases: ALTER TABLE items ADD stock INTEGER also works.

## The new column starts empty
Every existing row gets NULL in the new column. Fill it in with UPDATE:

    ALTER TABLE items ADD COLUMN stock INTEGER;
    UPDATE items SET stock = 10;
    UPDATE items SET stock = 0 WHERE name = 'Elixir';
    SELECT name, stock FROM items;

## ALTER versus UPDATE
- ALTER TABLE changes the STRUCTURE (which columns exist).
- UPDATE changes the DATA (values in rows).

## Other ALTERs
Real databases can also rename tables or columns and remove columns (ALTER TABLE ... DROP COLUMN). These are rarer and some databases limit them, because existing apps may rely on the old shape.

## Plan ahead, but not too much
Nobody designs a perfect table on day one. Adding columns later is normal; good names and types make it painless."##,
        &["ALTER TABLE ... ADD COLUMN name TYPE adds a column.", "Existing rows get NULL in the new column.", "Fill new columns with UPDATE.", "ALTER changes structure; UPDATE changes data."]),
        p("Project: Pet Shelter", &[
            (r##"CHECKPOINT PROJECT

Build the shelter's database: create a pets table with a key, add two pets, then find the older ones."##, "potion_red"),
            (r##"CREATE TABLE pets (id INTEGER PRIMARY KEY, name TEXT, species TEXT, age INTEGER);
INSERT INTO pets VALUES (1, 'Pip', 'owl', 3), (2, 'Mog', 'cat', 7);
SELECT name FROM pets WHERE age > 5;"##, "chest"),
        ], vec![
            quiz("Why does pets need an id column?", &["To identify each pet uniquely", "To sort alphabetically", "SQL requires it"], 0, "Two pets might share a name."),
            quiz("After CREATE TABLE, how many rows does pets have?", &["0", "1", "It copies heroes"], 0, "New tables are empty."),
            fill("Create the table.", r##"___ TABLE pets (id INTEGER PRIMARY KEY, name TEXT, species TEXT, age INTEGER);"##, &["CREATE", "INSERT", "ALTER"], 0, "CREATE TABLE builds a new table."),
            fill("Add two pets in one statement.", r##"INSERT INTO pets VALUES (1, 'Pip', 'owl', 3)___ (2, 'Mog', 'cat', 7);"##, &[",", ";", "AND"], 0, "Commas separate rows."),
            quiz("Which pet does the last query return?", &["Mog", "Pip", "Both"], 0, "Only Mog is older than 5."),
            copy("Type the shelter.", r##"CREATE TABLE pets (id INTEGER PRIMARY KEY, name TEXT, species TEXT, age INTEGER);
INSERT INTO pets VALUES (1, 'Pip', 'owl', 3), (2, 'Mog', 'cat', 7);
SELECT name FROM pets WHERE age > 5;"##, "Create, insert, query."),
            recall("From memory: create table toys (name TEXT), insert 'Ball', then select all toys.", r##"CREATE TABLE toys (name TEXT);
INSERT INTO toys VALUES ('Ball');
SELECT * FROM toys;"##, "Three statements."),
        ]).with(r##"## A database from scratch
The village shelter wants to track its animals. You will design the table, fill it, and answer the keeper's question, all in one script.

## 1. Design
One row is one pet. Facts per pet: an id, a name, the species and the age.

    CREATE TABLE pets (id INTEGER PRIMARY KEY, name TEXT, species TEXT, age INTEGER);

## 2. Fill
    INSERT INTO pets VALUES (1, 'Pip', 'owl', 3), (2, 'Mog', 'cat', 7);

## 3. Ask
    SELECT name FROM pets WHERE age > 5;

The answer is Mog, the 7-year-old cat.

## Why the id?
Two pets could share a name, but never an id. PRIMARY KEY tells the database this column identifies each row, and if you leave it out of an INSERT, the database numbers the row for you.

## Level up
- Count the pets per species: SELECT species, COUNT(*) FROM pets GROUP BY species;
- Track adoptions: ALTER TABLE pets ADD adopted INTEGER, then UPDATE a pet to 1."##,
        &["Design: one row per thing, one column per fact, plus an id.", "CREATE the table, INSERT rows, then query.", "Commas separate rows in one INSERT.", "One script can build, fill and question a database."]),
    ])
}

fn joining() -> Module {
    ch("Joining Tables", "Combine related tables with INNER JOIN, aliases and LEFT JOIN.", vec![
        l("Why Join?", &[
            (r#"Data is split into tables to avoid repeating it.

heroes.guild_id points to guilds.id"#, "map"),
            (r#"Borin's guild_id is 2.
Guild 2 is Iron Wall.

A JOIN puts them back together."#, "key"),
        ], vec![
            quiz("Why store the guild in its own table?", &["So guild details are stored once, not repeated per hero", "Tables must be small", "It is faster to type"], 0, "No repetition means no contradictions."),
            quiz("heroes.guild_id refers to...", &["guilds.id", "heroes.id", "quests.id"], 0, "It points to a guild's id."),
            fill("Which column links a quest to its hero?", "SELECT title, ___ FROM quests;", &["hero_id", "id", "name"], 0, "hero_id points to heroes.id."),
            fill("Find guild 2's name.", "SELECT name FROM guilds WHERE id = ___;", &["2", "'Borin'", "guild_id"], 0, "Look the id up."),
            copy("Type this query.", "SELECT name, guild_id FROM heroes;", "See the links."),
            recall("From memory: show every column of the guilds table.", "SELECT * FROM guilds;", "SELECT * FROM guilds;"),
        ]).with(r##"## Data lives in several tables
Imagine storing each hero's guild name AND city in the heroes table. Two heroes in Iron Wall means writing "Iron Wall, Stonehold" twice. If the guild moves, you must change every copy, and if you miss one, the data contradicts itself.

Databases avoid this by splitting data into related tables:

    heroes                       guilds
    name  | guild_id             id | name        | city
    Borin | 2          ---->     2  | Iron Wall   | Stonehold
    Dara  | 2          ---->
    Aria  | 1          ---->     1  | Moon Circle | Silverpine

heroes.guild_id holds the id of a row in guilds. This is a foreign key pointing at a primary key.

## Our four tables
- heroes.guild_id → guilds.id
- quests.hero_id → heroes.id

## Joining
To answer "which city does each hero live in?" you need both tables at once. A JOIN combines rows from two tables wherever the link matches. That is the next lesson.

## Look before you join
Run SELECT * on each table first and find the matching columns. Every join starts with the question: "which column in table A points to which column in table B?""##,
        &["Data is split into tables so each fact is stored once.", "A foreign key (heroes.guild_id) points at a primary key (guilds.id).", "JOINs combine related tables back together.", "Find the matching columns before writing a join."]),
        l("INNER JOIN", &[
            (r#"SELECT heroes.name, guilds.name
FROM heroes
JOIN guilds ON heroes.guild_id = guilds.id;"#, "banner"),
            (r#"Write table.column when both tables have the same column name (like name or id)."#, "shield"),
        ], vec![
            quiz("What does ON say?", &["How rows from the two tables match", "Which columns to show", "How to sort"], 0, "ON is the matching rule."),
            quiz("Which heroes are missing from an INNER JOIN with guilds?", &["Heroes with no matching guild (Finn)", "None", "All mages"], 0, "INNER JOIN keeps only matches."),
            fill("Combine the tables.", "SELECT heroes.name, guilds.city FROM heroes ___ guilds ON heroes.guild_id = guilds.id;", &["JOIN", "AND", "WITH"], 0, "JOIN table ON ..."),
            fill("Give the matching rule.", "SELECT heroes.name FROM heroes JOIN guilds ___ heroes.guild_id = guilds.id;", &["ON", "WHERE", "BY"], 0, "ON condition."),
            copy("Type this query.", "SELECT heroes.name, guilds.name FROM heroes JOIN guilds ON heroes.guild_id = guilds.id;", "Hero and guild names."),
            recall("From memory: show each quest title with the name of its hero (join quests and heroes).", "SELECT quests.title, heroes.name FROM quests JOIN heroes ON quests.hero_id = heroes.id;", "Join on hero_id = id."),
        ]).with(r##"## Combining matching rows
    SELECT heroes.name, guilds.name, guilds.city
    FROM heroes
    JOIN guilds ON heroes.guild_id = guilds.id;

- FROM heroes is the first table.
- JOIN guilds adds the second table.
- ON gives the matching rule: a hero row and a guild row belong together when heroes.guild_id equals guilds.id.

JOIN and INNER JOIN mean the same thing.

## table.column
Both tables have columns called name and id. Writing just name is ambiguous, and Code Forge's message asks you to choose. Prefix the table: heroes.name, guilds.name.

## Only matches survive
An INNER JOIN keeps only rows that have a partner. Finn has no guild (NULL), so he disappears from the result. The Haunted Mill quest has no hero, so it disappears from a quests-heroes join. If you need those rows, use a LEFT JOIN (two lessons ahead).

## Everything else still works
WHERE, ORDER BY, GROUP BY and LIMIT all work on the joined result:

    SELECT guilds.name, COUNT(*) AS members
    FROM heroes
    JOIN guilds ON heroes.guild_id = guilds.id
    GROUP BY guilds.name;"##,
        &["JOIN table ON a.col = b.col combines matching rows.", "JOIN is the same as INNER JOIN.", "Use table.column when names clash.", "Rows without a match are left out of an INNER JOIN."]),
        l("Aliases", &[
            (r#"Short names for tables:

SELECT h.name, g.city
FROM heroes AS h
JOIN guilds AS g ON h.guild_id = g.id;"#, "hat_wizard"),
            (r#"AS is optional: FROM heroes h"#, "scroll_fire"),
        ], vec![
            quiz("In FROM heroes AS h, what is h?", &["A short name for heroes", "A column", "A new table"], 0, "Table alias."),
            quiz("Once you write FROM heroes h, how do you refer to its name column?", &["h.name", "heroes.h", "name.h"], 0, "alias.column."),
            fill("Give quests a short name.", "SELECT q.title FROM quests ___ q;", &["AS", "IS", "="], 0, "table AS alias."),
            fill("Use the alias.", "SELECT ___.name FROM heroes AS h;", &["h", "heroes.h", "AS"], 0, "alias.column."),
            copy("Type this query.", "SELECT h.name, g.city FROM heroes AS h JOIN guilds AS g ON h.guild_id = g.id;", "Short and clear."),
            recall("From memory: show q.title and q.reward from quests aliased as q, joined to heroes h on q.hero_id = h.id, only for knights.", "SELECT q.title, q.reward FROM quests AS q JOIN heroes AS h ON q.hero_id = h.id WHERE h.class = 'knight';", "Aliases plus WHERE."),
        ]).with(r##"## Shorter table names
Join queries repeat table names a lot. A table alias gives a table a short nickname for the rest of the query:

    SELECT h.name, g.name AS guild, g.city
    FROM heroes AS h
    JOIN guilds AS g ON h.guild_id = g.id;

- heroes AS h: inside this query, h means heroes.
- g.city: the city column of guilds.
- AS is optional for tables: FROM heroes h works too.

## Column aliases versus table aliases
- Column alias (g.name AS guild): renames a column in the RESULT.
- Table alias (guilds AS g): a nickname used INSIDE the query.

Both make long queries readable. Here the column alias also stops the result from having two columns both called name.

## Once aliased, use the alias
After FROM heroes h, write h.name, not heroes.name. Mixing them is a common source of errors.

## Good alias names
Use the first letter or a short word: h for heroes, q for quests, g for guilds. In big queries with many tables, slightly longer aliases (qst, hro) can be clearer."##,
        &["FROM table AS t gives the table a short alias.", "Refer to columns as alias.column.", "Table aliases work inside the query; column aliases rename result headers.", "After aliasing a table, use the alias consistently."]),
        l("LEFT JOIN", &[
            (r#"LEFT JOIN keeps every row from the left table, even without a match:

SELECT h.name, g.name
FROM heroes h
LEFT JOIN guilds g ON h.guild_id = g.id;"#, "lantern"),
            (r#"No match? The right side is NULL.

Finn appears with guild NULL."#, "keyhole"),
        ], vec![
            quiz("What does LEFT JOIN keep that INNER JOIN drops?", &["Left rows with no match", "Right rows with no match", "Duplicate rows"], 0, "Every left row stays."),
            quiz("What fills the right-side columns when there is no match?", &["NULL", "0", "The left values"], 0, "Missing matches become NULL."),
            fill("Keep heroes without a guild.", "SELECT h.name, g.name FROM heroes h ___ JOIN guilds g ON h.guild_id = g.id;", &["LEFT", "INNER", "OUTER RIGHT"], 0, "LEFT JOIN keeps all heroes."),
            fill("Find quests with no hero.", "SELECT q.title FROM quests q LEFT JOIN heroes h ON q.hero_id = h.id WHERE h.id IS ___;", &["NULL", "0", "EMPTY"], 0, "No match means NULL."),
            copy("Type this query.", "SELECT h.name, g.city FROM heroes AS h LEFT JOIN guilds AS g ON h.guild_id = g.id;", "All six heroes."),
            recall("From memory: show every quest title with its hero's name, keeping quests that have no hero (quests q, heroes h).", "SELECT q.title, h.name FROM quests AS q LEFT JOIN heroes AS h ON q.hero_id = h.id;", "LEFT JOIN from quests."),
        ]).with(r##"## Keeping rows without a partner
An INNER JOIN drops rows that have no match. Sometimes you need them: "list ALL heroes and their guild, if any".

    SELECT h.name, g.name AS guild
    FROM heroes AS h
    LEFT JOIN guilds AS g ON h.guild_id = g.id;

LEFT JOIN keeps every row from the LEFT table (the one after FROM). When there is no matching row on the right, the right-side columns are filled with NULL. Finn now appears, with guild NULL.

## Finding what is missing
Combining LEFT JOIN with IS NULL answers "which ones have no partner?":

    SELECT q.title
    FROM quests AS q
    LEFT JOIN heroes AS h ON q.hero_id = h.id
    WHERE h.id IS NULL;

This finds quests nobody has taken: the Haunted Mill.

## Which table is "left"?
The left table is the one written first (after FROM). Swapping the tables changes which side is kept. If you want all guilds, even empty ones, start FROM guilds.

## INNER or LEFT?
- Only complete pairs? INNER JOIN.
- Everything from the first table, matched where possible? LEFT JOIN."##,
        &["LEFT JOIN keeps every row of the first (left) table.", "Unmatched right-side columns become NULL.", "LEFT JOIN + WHERE right.id IS NULL finds rows with no partner.", "The table after FROM is the one that is kept."]),
        p("Final Project: Quest Board", &[
            (r##"FINAL PROJECT

Build the tavern's quest board: every quest, its reward, and the hero and guild taking it, with open quests shown too."##, "scroll_fire"),
            (r##"SELECT q.title, q.reward, h.name AS hero, g.name AS guild
FROM quests AS q
LEFT JOIN heroes AS h ON q.hero_id = h.id
LEFT JOIN guilds AS g ON h.guild_id = g.id
ORDER BY q.reward DESC;"##, "banner_royal"),
        ], vec![
            quiz("Why LEFT JOIN from quests?", &["So open quests with no hero still appear", "LEFT is faster", "INNER JOIN can't sort"], 0, "Every quest must be on the board."),
            quiz("Which shows only quests nobody has taken?", &["WHERE h.id IS NULL", "WHERE q.reward > 0", "LIMIT 1"], 0, "No matching hero means NULL."),
            fill("Join quests to heroes.", r##"LEFT JOIN heroes AS h ON q.hero_id = ___"##, &["h.id", "h.name", "q.id"], 0, "hero_id points to heroes.id."),
            fill("Join each hero to their guild.", r##"LEFT JOIN guilds AS g ON h.guild_id = ___"##, &["g.id", "g.name", "h.id"], 0, "guild_id points to guilds.id."),
            quiz("What do hero and guild show for the Haunted Mill?", &["NULL: nobody has taken it", "The first hero", "The query fails"], 0, "LEFT JOIN keeps the quest with empty hero and guild."),
            copy("Type the quest board.", r##"SELECT q.title, q.reward, h.name AS hero, g.name AS guild
FROM quests AS q
LEFT JOIN heroes AS h ON q.hero_id = h.id
LEFT JOIN guilds AS g ON h.guild_id = g.id
ORDER BY q.reward DESC;"##, "Three tables, richest quests first."),
            recall("From memory: show the total reward per hero name, joining quests q to heroes h (inner join), grouped by h.name.", r##"SELECT h.name, SUM(q.reward) FROM quests AS q JOIN heroes AS h ON q.hero_id = h.id GROUP BY h.name;"##, "Join, then group."),
        ]).with(r##"## The tavern quest board
Your final SQL project combines three tables to build a quest board for the tavern wall: every quest, its reward, the hero who took it and that hero's guild.

## The query
    SELECT q.title, q.reward, h.name AS hero, g.name AS guild
    FROM quests AS q
    LEFT JOIN heroes AS h ON q.hero_id = h.id
    LEFT JOIN guilds AS g ON h.guild_id = g.id
    ORDER BY q.reward DESC;

## How the joins chain
- quests is the starting table, so every quest appears.
- The first LEFT JOIN finds the hero whose id matches the quest's hero_id.
- The second LEFT JOIN finds that hero's guild through guild_id.
- An open quest (the Haunted Mill) has no hero, so hero and guild show NULL; LEFT JOIN keeps it on the board anyway.

## Level up: totals per guild
    SELECT g.name AS guild, SUM(q.reward) AS gold
    FROM quests AS q
    JOIN heroes AS h ON q.hero_id = h.id
    JOIN guilds AS g ON h.guild_id = g.id
    GROUP BY g.name;

## You made it
You can now query, filter, sort, total, change, create and join. These are the same skills data analysts and backend developers use every day. You have finished the SQL course!"##,
        &["LEFT JOIN keeps unmatched rows such as open quests.", "Joins chain: each JOIN adds a table.", "Aliases (q, h, g) keep long queries readable.", "You've used every core SQL skill."]),
    ])
}
