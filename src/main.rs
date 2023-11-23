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

/**
# roocker's rust - `echo`
- reads and outputs input
- does not allocate Data (e.g. to a vector) to do so,
- no external crates
- consumes Input Iterator once
- uses RefCell to work around Rust borrowing rules when matching option flags

# Issues:
- `rr_echo -e test1\\ntest2` - does not output newline, since i check word by word.

# Options:
Echo the STRING(s) to standard output.

       -n     do not output the trailing newline
       -e     enable interpretation of backslash escapes
       -E     disable interpretation of backslash escapes (default)

       If -e is in effect, the following sequences are recognized:
       \\     backslash
       \a     alert (BEL)
       \b     backspace
       \e     escape
       \f     form feed
       \n     new line
       \r     carriage return
       \t     horizontal tab
       \v     vertical tab

    ## not implemented

       --help display this help and exit
       --version
              output version information and exit
       \c          produce no further output
       \0NNN       byte with octal value NNN (1 to 3 digits)
       \xHH        byte with hexadecimal value HH (1 to 2 digits)
       \uHHHH      the Unicode character whose value is the hexadecimal value HHHH.
                   HHHH can be one to four hex digits.
       \UHHHHHHHH  the Unicode character whose value is the hexadecimal value
                   HHHHHHHH. HHHHHHHH can be one to eight hex digits.

*/
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
