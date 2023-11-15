### Danylo Burliai Parser

My parser for educational purposes
It can parse corporate user entry, in format Name Surname, n.surname@domain.com

![some img](parser.png)

```rust
for arg in args {
match arg.as_str() {
"--file" => { is_file = true; },
"--email" => { is_user = false; },
_ => { s = arg; },
}
}
```
Grammar: 
```pest
lower_word = _{ ASCII_ALPHA_LOWER+ }
email_domain = { lower_word ~ ("." ~ lower_word){1, 2}}
email_name = { lower_word ~ "." ~ lower_word }
email = { email_name ~ "@" ~ email_domain}
just_email = { SOI ~ email_name ~ "@" ~ email_domain ~ EOI }

capital_letter_word = _{ ASCII_ALPHA_UPPER ~ (ASCII_ALPHA_LOWER)* }
name = { capital_letter_word }
surname = { capital_letter_word }
user = { name ~ WHITE_SPACE ~ surname ~ ", " ~ email}

file_email = { SOI ~ (email ~ NEWLINE)* ~ (email ~ NEWLINE?) ~ EOI }
file_user = { SOI ~ (user ~ NEWLINE)* ~ (user ~ NEWLINE?) ~ EOI }
```