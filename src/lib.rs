use anyhow::anyhow;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct UserParser;

#[derive(Debug, Default, PartialEq)]
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

#[derive(Debug, Default, PartialEq)]
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
    let got = UserParser::parse(Rule::email, s);
    if got.is_err() {
        return Err(anyhow!("Parsing failed"));
    }
    Ok(ParsedEmail::from_pairs(got.unwrap().next().unwrap().into_inner()))
}

pub fn parse_just_email(s: &str) -> anyhow::Result<ParsedEmail> {
    let got = UserParser::parse(Rule::just_email, s);
    if got.is_err() {
        return Err(anyhow!("Parsing failed"));
    }
    Ok(ParsedEmail::from_pairs(got.unwrap().next().unwrap().into_inner()))
}

pub fn parse_file_email(s: &str) -> anyhow::Result<Vec<ParsedEmail>> {
    let mut res: Vec<ParsedEmail> = vec!();
    let got = UserParser::parse(Rule::file_email, s);
    if got.is_err() {
        return Err(anyhow!("Parsing failed"));
    }
    for elem in got.unwrap().next().unwrap().into_inner() {
        res.push(ParsedEmail::from_pairs(elem.into_inner()));
    }
    Ok(res)
}

pub fn parse_user(s: &str) -> anyhow::Result<ParsedUser> {
    let got = UserParser::parse(Rule::user, s);
    if got.is_err() {
        return Err(anyhow!("Parsing failed"));
    }
    Ok(ParsedUser::from_pairs(got.unwrap().next().unwrap().into_inner()))
}

pub fn parse_file_user(s: &str) -> anyhow::Result<Vec<ParsedUser>> {
    let mut res: Vec<ParsedUser> = vec!();
    let got = UserParser::parse(Rule::file_user, s);
    if got.is_err() {
        return Err(anyhow!("Parsing failed"));
    }
    for elem in got.unwrap().next().unwrap().into_inner() {
        res.push(ParsedUser::from_pairs(elem.into_inner()));
    }
    Ok(res)
}