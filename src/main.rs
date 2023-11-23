// cargo run -- -e test1 \\n test2 \\n test3: \\a test4 \\b test5 \\c test6 \\e test7 \\f test8 \\n \\n test9 \\r \\r test10 \\t test11 \\t test12  \\v test13 \\v test13

use std::{cell::RefCell, env};
struct Config {
    backslash_escapes: RefCell<bool>,
    trailing_newline: RefCell<bool>,
}

impl Config {
    fn new() -> Self {
        Config {
            backslash_escapes: RefCell::new(false),
            trailing_newline: RefCell::new(true),
        }
    }

    fn find_flags<'a>(
        &'a self,
        input: impl Iterator<Item = String> + 'a,
    ) -> impl Iterator<Item = String> + 'a {
        let input = input.filter_map(|word| match word.as_str() {
            "-e" => {
                *self.backslash_escapes.borrow_mut() = true;
                None
            }
            "-n" => {
                *self.trailing_newline.borrow_mut() = false;
                None
            }
            "-en" => {
                *self.backslash_escapes.borrow_mut() = true;
                *self.trailing_newline.borrow_mut() = false;
                None
            }
            "-ne" => {
                *self.backslash_escapes.borrow_mut() = true;
                *self.trailing_newline.borrow_mut() = false;
                None
            }
            "-E" => {
                *self.backslash_escapes.borrow_mut() = false;
                None
            }
            _ => Some(word),
        });
        input
    }
}

fn main() {
    // println!("{}", std::env::args().skip(1).format(" ")); // most simple version?

    let config = Config::new();
    let input = env::args().skip(1);

    // does defining content here increase overhead?
    let mut content = config.find_flags(input);

    // i dont like that i have to use replace_escapes for first_word seperatly here.
    // i can try to map over the whole content first and replace escapes words (w/o consuming the Iterator!)?
    if let Some(first_word) = &content.next() {
        if *config.backslash_escapes.borrow() {
            print!("{}", replace_escapes(first_word));
        } else {
            print!("{}", first_word);
        }
        content.for_each(|word| {
            if let Some(word) = Some(word) {
                if *config.backslash_escapes.borrow() {
                    print!(" {}", replace_escapes(&word));
                } else {
                    print!(" {}", word);
                }
            }
        });
        if *config.trailing_newline.borrow() {
            println!("");
        }
    };

    fn replace_escapes(word: &String) -> &str {
        match word.as_str() {
            "\\a" => "\x07",       // alert (BEL)
            "\\b" => "\x08",       // backspace
            "\\c" => "(STOPPPP!)", // produce no further output
            "\\e" => "\x1b",       // escape
            "\\f" => "\x0c",       // form feed
            "\\n" => "\n",         // newline
            "\\r" => "\r",         // carriage return
            "\\t" => "\t",         // horizontal tab
            "\\v" => "\x0B",       // vertical tab
            _ => word,
        }
    }
}
