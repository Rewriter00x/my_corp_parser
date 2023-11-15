use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct EmailParser;

#[derive(Debug, Default)]
pub struct ParsedEmail {
    pub name: String,
    pub domain: String,
}

impl ParsedEmail {
    fn default() -> ParsedEmail {
        ParsedEmail {
            name: String::new(),
            domain: String::new()
        }
    }

    pub fn new() -> ParsedEmail {
        ParsedEmail{ ..Default::default() }
    }

    pub fn from_pairs(pairs: Pairs<Rule>) -> ParsedEmail {
        let mut res = ParsedEmail::new();
        for pair in pairs {
            let rule = pair.as_rule();
            let s = pair.as_str();
            match rule {
                Rule::email_domain => { res.domain = s.to_string(); }
                Rule::email_name => { res.name = s.to_string(); }
                _ => {
                    println!("Found unexpected rule {:?}", rule);
                }
            }
        }
        res
    }
}

#[derive(Debug, Default)]
pub struct ParsedUser {
    pub name: String,
    pub surname: String,
    pub email: ParsedEmail,
}

impl ParsedUser {
    fn default() -> ParsedUser {
        ParsedUser {
            name: String::new(),
            surname: String::new(),
            email: ParsedEmail::new(),
        }
    }

    pub fn new() -> ParsedUser {
        ParsedUser{ ..Default::default() }
    }

    pub fn from_pairs(pairs: Pairs<Rule>) -> ParsedUser {
        let mut res = ParsedUser::new();
        for pair in pairs {
            let rule = pair.as_rule();
            let s = pair.as_str();
            match rule {
                Rule::email => { res.email = ParsedEmail::from_pairs(pair.into_inner()); }
                Rule::name => { res.name = s.to_string(); }
                Rule::surname => { res.surname = s.to_string(); }
                _ => {
                    println!("Found unexpected rule {:?}", rule);
                }
            }
        }
        res
    }
}

pub fn parse_email(s: &str) -> anyhow::Result<ParsedEmail> {
    let mut got = EmailParser::parse(Rule::email, s).unwrap();
    Ok(ParsedEmail::from_pairs(got.next().unwrap().into_inner()))
}

pub fn parse_file(s: &str) -> anyhow::Result<Vec<ParsedEmail>> {
    let mut res: Vec<ParsedEmail> = vec!();
    let got = EmailParser::parse(Rule::file, s).unwrap();
    for elem in got {
        res.push(ParsedEmail::from_pairs(elem.into_inner()));
    }
    Ok(res)
}