use std::{cell::RefCell, env, error::Error, iter::FilterMap};

/**
# roocker's rust `echo`
- reads and outputs input

# todo
- [ ] accept `-e` arg for backslash interpretation output

Echo the STRING(s) to standard output.

       -n     do not output the trailing newline

       -e     enable interpretation of backslash escapes

       -E     disable interpretation of backslash escapes (default)

       --help display this help and exit

       --version
              output version information and exit

       If -e is in effect, the following sequences are recognized:

       \\     backslash

       \a     alert (BEL)

       \b     backspace

       \c     produce no further output

       \e     escape

       \f     form feed

       \n     new line

       \r     carriage return

       \t     horizontal tab

       \v     vertical tab

       \0NNN  byte with octal value NNN (1 to 3 digits)

       \xHH   byte with hexadecimal value HH (1 to 2 digits)

       NOTE: your shell may have its own version of echo, which usually
       supersedes the version described here.  Please refer to your
       shell's documentation for details about the options it supports.

       NOTE: printf(1) is a preferred alternative, which does not have
       issues outputting option-like strings.

*/

#[derive(Debug)]
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
        &'a mut self,
        input: impl Iterator<Item = String> + 'a,
    ) -> impl Iterator<Item = String> + 'a
// where
    //     T: Iterator<Item = String>,
    //     f: FnMut(String),
    {
        let input = input.filter_map(|word| match word.as_str() {
            "-e" => {
                *self.backslash_escapes.borrow_mut() = true;
                None
            }
            "-n" => {
                *self.trailing_newline.borrow_mut() = false;
                None
            }
            _ => Some(word),
        });
        input
    }
}

fn main() {
    // code a gnu echo command in rust use input variable as input
    // println!("{}", std::env::args().skip(1).format(" "));

    // define standard config
    let mut config = Config::new();

    // read input
    let input = env::args().skip(1);
    // config.find_flags(&input);

    let content = config.find_flags(input);

    let bs_e = *config.backslash_escapes.borrow();

    if let Some(first_word) = content.next() {
        // first = format!("{}", word);
        if bs_e {
            print!("{}", replace_escapes(first_word));
        } else {
            print!("{}", first_word);
        }
        content.for_each(|word| {
            if let Some(word) = Some(word) {
                if bs_e {
                    print!(" {}", replace_escapes(word));
                } else {
                    print!(" {}", word);
                }
            }
        });
    };

    println!("\nDDD");

    fn replace_escapes(word: String) -> String {
        let mut output = String::new();
        let mut chars = word.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                if let Some(c) = chars.next() {
                    match c {
                        'n' => output.push('\n'),
                        't' => output.push('\t'),
                        _ => output.push(c),
                    }
                }
            } else {
                output.push(c);
            }
        }
        output
    }
}
