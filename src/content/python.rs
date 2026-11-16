use super::*;
use eframe::egui::Color32;

pub fn course() -> Course {
    Course { slug: "python", hours: 20, teaches: vec!["Variables, numbers and text", "if / elif / else decisions", "for and while loops", "Lists and functions"], snippet: r#"print("Hello, hero!")"#, badge: Badge { name: "Serpent Sage", art: "spellbook", desc: "Finished every Python quest" }, name: "Python", tag: "PY", color: Color32::from_rgb(0x2f, 0x6f, 0xd6), lang: "python", cat: "Code", level: "Beginner", desc: "Learn programming fundamentals such as variables, control flow, and loops", modules: vec![
        m("Hello World", vec![
            l("Printing", &[
                (r#"print() shows text on the screen.

Text goes between quotes. Anything between quotes is a STRING."#, "scroll_fire"),
                (r#"print("Hello, World!")

Code runs top to bottom, one line at a time."#, "torch"),
            ], vec![
                fill("Show a message.", r#"___("Hello, World!")"#, &["print", "echo", "say"], 0, "print() writes to the screen."),
                fill("Close the text.", r#"print("Hello___"#, &["\")", ")", "'"], 0, "Close the quote, then the bracket."),
                copy("Type this line.", r#"print("Hello, World!")"#, "print(\"text\")"),
                recall("From memory: print Hi.", r#"print("Hi")"#, r#"print("Hi")"#),
            ]),
            l("Strings and Comments", &[
                (r#"Print several lines to draw a pattern. Each print() starts a new line.

A line starting with # is a comment: Python ignores it."#, "book"),
                (r#"# draw a tower
print("*")
print("**")
print("***")"#, "chest"),
            ], vec![
                fill("Write a comment.", "___ this is a note", &["#", "//", "--"], 0, "Python comments start with #."),
                fill("Print two stars.", r#"print("___")"#, &["**", "2", "*+*"], 0, "Whatever is in the quotes is printed."),
                copy("Type this pattern.", "print(\"*\")\nprint(\"**\")", "Two prints, two lines."),
                recall("From memory: print A then B on two lines.", "print(\"A\")\nprint(\"B\")", "Two prints."),
            ]),
        ]),
        m("Variables", vec![
            l("Variables", &[
                (r#"A variable stores a value under a name. No keyword needed.

lives = 3
name = "Pix""#, "potion_red"),
                (r#"lives = 3
print(lives)  # 3"#, "orb_red"),
            ], vec![
                fill("Store 3 in lives.", "lives ___ 3", &["=", "==", ":"], 0, "A single = stores a value."),
                fill("Show the variable.", "lives = 3\n___(lives)", &["print", "show", "lives"], 0, "print(lives) shows 3."),
                copy("Type this and see 3.", "lives = 3\nprint(lives)", "Prints 3."),
                recall("From memory: gold is 7, print gold.", "gold = 7\nprint(gold)", "gold = 7; print(gold)"),
            ]),
            l("Math", &[
                (r#"Python does math: + - * / and % (remainder) and ** (power). Strings join with +."#, "coin_gold"),
                (r#"print(2 + 3 * 2)   # 8
print(7 % 4)       # 3
print("Hi " + "hero")"#, "gem_green"),
            ], vec![
                fill("Get the remainder.", "print(7 ___ 4)", &["%", "/", "//"], 0, "% is the remainder: 3."),
                fill("Join strings.", r#"print("a" ___ "b")"#, &["+", "&", "."], 0, "+ joins strings."),
                copy("Type this line.", "print(2 + 3 * 2)", "Multiplication first: 8."),
                recall("From memory: print 2 to the power 3.", "print(2 ** 3)", "print(2 ** 3)"),
            ]),
        ]),
        m("Control Flow", vec![
            l("If Statements", &[
                (r#"if runs code only when its condition is True. The line ends with a colon, and the code inside is indented."#, "key"),
                (r#"hp = 5
if hp > 0:
  print("alive")"#, "potion_red"),
            ], vec![
                fill("End the if line.", "if hp > 0___\n  print(\"alive\")", &[":", ";", "."], 0, "An if line ends with a colon."),
                fill("Test hp is above zero.", "if hp ___ 0:\n  print(\"alive\")", &[">", "<", "=="], 0, "> means greater than."),
                copy("Type this and see alive.", "hp = 5\nif hp > 0:\n  print(\"alive\")", "5 > 0 is True."),
                recall("From memory: if 2 > 1 print yes.", "if 2 > 1:\n  print(\"yes\")", "if 2 > 1:"),
            ]),
            l("Else", &[
                (r#"else runs when the condition is False. == checks equality."#, "shield"),
                (r#"hp = 0
if hp > 0:
  print("alive")
else:
  print("fallen")"#, "sword"),
            ], vec![
                fill("Run this otherwise.", "if hp > 0:\n  print(\"alive\")\n___:\n  print(\"fallen\")", &["else", "elif", "otherwise"], 0, "else:"),
                fill("Check equality.", "if hp ___ 0:\n  print(\"zero\")", &["==", "=", "=>"], 0, "== compares values."),
                copy("Type this and see fallen.", "hp = 0\nif hp > 0:\n  print(\"alive\")\nelse:\n  print(\"fallen\")", "0 > 0 is False."),
                recall("From memory: if 1 > 2 print a, else print b.", "if 1 > 2:\n  print(\"a\")\nelse:\n  print(\"b\")", "else branch"),
            ]),
        ]),
        m("Loops", vec![
            l("For Loops", &[
                (r#"for repeats code. range(3) gives 0, 1, 2 - it starts at 0 and stops before 3."#, "orb_red"),
                (r#"for i in range(3):
  print(i)"#, "torch"),
            ], vec![
                fill("Count three times.", "for i in ___(3):\n  print(i)", &["range", "count", "loop"], 0, "range()."),
                fill("Loop over the variable i.", "for ___ in range(3):\n  print(i)", &["i", "range", "print"], 0, "The loop variable is i."),
                copy("Type this loop.", "for i in range(3):\n  print(i)", "Prints 0, 1, 2."),
                recall("From memory: loop twice printing hi.", "for i in range(2):\n  print(\"hi\")", "range(2)"),
            ]),
            l("While Loops", &[
                (r#"while repeats as long as its condition is True. Change the variable inside or it never stops!"#, "cauldron"),
                (r#"n = 3
while n > 0:
  print(n)
  n -= 1"#, "potion_red"),
            ], vec![
                fill("Keep going while n is above 0.", "while n ___ 0:\n  print(n)\n  n -= 1", &[">", "<", "=="], 0, "Loop while n > 0."),
                fill("Count down by one.", "while n > 0:\n  print(n)\n  n ___ 1", &["-=", "+=", "=="], 0, "-= subtracts."),
                copy("Type this loop.", "n = 3\nwhile n > 0:\n  print(n)\n  n -= 1", "Prints 3, 2, 1."),
                recall("From memory: n = 2, loop while n > 0 printing n, then subtract 1.", "n = 2\nwhile n > 0:\n  print(n)\n  n -= 1", "Prints 2, 1."),
            ]),
        ]),
        m("Lists and Functions", vec![
            l("Lists", &[
                (r#"A list holds many values. Counting starts at 0, so loot[0] is the first item. len() counts items."#, "chest"),
                (r#"loot = ["gem", "coin", "key"]
print(loot[0])
print(len(loot))"#, "key"),
            ], vec![
                fill("Make a list.", "loot = ___\"gem\", \"coin\"]", &["[", "(", "{"], 0, "Lists use square brackets."),
                fill("Count the items.", "print(___(loot))", &["len", "count", "size"], 0, "len() counts items."),
                copy("Type this list.", "loot = [\"gem\", \"coin\"]\nprint(len(loot))", "Prints 2."),
                recall("From memory: nums holds 1 and 2, print how many.", "nums = [1, 2]\nprint(len(nums))", "len(nums) is 2."),
            ]),
            l("Functions", &[
                (r#"A function is a reusable spell. def starts it, return hands back a result."#, "staff"),
                (r#"def heal(hp):
  return hp + 10

print(heal(5))  # 15"#, "potion_red"),
            ], vec![
                fill("Define a function.", "___ heal(hp):\n  return hp + 10", &["def", "func", "make"], 0, "def starts a function."),
                fill("Send back a result.", "def heal(hp):\n  ___ hp + 10", &["return", "give", "print"], 0, "return."),
                copy("Type this and see 15.", "def heal(hp):\n  return hp + 10\n\nprint(heal(5))", "5 + 10 = 15."),
                recall("From memory: double(n) returns n * 2.", "def double(n):\n  return n * 2", "def double(n):"),
            ]),
        ]),
    ]}
}
