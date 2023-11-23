# roocker's rust - `echo`
- reads and outputs input
- does not allocate Data (e.g. to a vector) to do so, 
- no external crates
- consumes Input Iterator once 
- uses RefCell to work around Rust borrowing rules when matching option flags

# Issues:
- `rr_echo -e test1\\ntest2` - does not output newline, since i check word by word.

# Options:
```
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
```

