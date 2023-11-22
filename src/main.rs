use std::env;

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

struct Config {
    backslash_escapes: bool,
    trailing_newline: bool,
}

impl Config {
    fn new() -> Self {
        Config {
            backslash_escapes: false,
            trailing_newline: true,
        }
    }
    fn find_flags<I>(&mut self, input: I)
    where
        I: Iterator<Item = String>,
    {
    }
}
fn main() {
    // code a gnu echo command in rust use input variable as input
    let mut config = Config::new();
    let input = env::args().skip(1);
    config.find_flags();
    // println!("input:{:?}", input);
    // println!("{:?}", input);

    // println!("after skip1:{:?}", input);

    let mut backslash_escapes = false;
    let mut trailing_newline = true;

    let mut input = input.map(|word| match word.as_str() {
        "-e" => {
            backslash_escapes = true;
            None
        }
        "-n" => {
            trailing_newline = false;
            None
        }
        _ => Some(word),
    });

    // let mut first = String::new();
    if let Some(first_word) = input.find_map(|word| word) {
        // first = format!("{}", word);
        print!("{}", first_word);
    };

    // println!("\nafter first word:{:?} \n", input);

    input.for_each(|word| {
        if let Some(word) = word {
            if backslash_escapes {
                print!(" {}", replace_escapes(word));
            } else {
                print!(" {}", word);
            }
        }
    });

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
