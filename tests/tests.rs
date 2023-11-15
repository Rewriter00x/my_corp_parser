use std::fs;
use my_parser_kma_group_3_DB::*;

#[test]
pub fn good_email_test() {
    let got = parse_email("d.burliai@gmail.com").unwrap();
    assert_eq!(got.name, "d.burliai");
    assert_eq!(got.domain, "gmail.com");
}

#[test]
pub fn good_just_email_test() {
    let got = parse_just_email("d.burliai@gmail.com").unwrap();
    assert_eq!(got.name, "d.burliai");
    assert_eq!(got.domain, "gmail.com");
}

#[test]
pub fn good_email_long_test() {
    let got = parse_email("d.burliai@ukma.edu.ua").unwrap();
    assert_eq!(got.name, "d.burliai");
    assert_eq!(got.domain, "ukma.edu.ua");
}

#[test]
pub fn bad_email_no_name_test() {
    let got = parse_email(".burliai@ukma.edu.ua");
    assert!(got.is_err());
}

#[test]
pub fn bad_email_too_long_test() {
    let got = parse_email("d.burliai@ukma.edu.ua.net").unwrap();
    assert_eq!(got.name, "d.burliai");
    assert_eq!(got.domain, "ukma.edu.ua");
}

#[test]
pub fn bad_just_email_too_long_test() {
    let got = parse_just_email("d.burliai@ukma.edu.ua.net");
    assert!(got.is_err())
}

#[test]
pub fn bad_email_domain_test() {
    let got = parse_email("d.burliai@ukma");
    assert!(got.is_err());
}

#[test]
pub fn good_user_test() {
    let got = parse_user("Danylo Burliai, d.burliai@hello.world").unwrap();
    assert_eq!(got.name, "Danylo");
    assert_eq!(got.surname, "Burliai");
    assert_eq!(got.email, ParsedEmail{ name: "d.burliai".to_string(), domain: "hello.world".to_string() });
}

#[test]
pub fn bad_user_no_surname_test() {
    let got = parse_user("Danylo, d.burliai@hello.world");
    assert!(got.is_err());
}

#[test]
pub fn bad_user_just_email_test() {
    let got = parse_user("d.burliai@hello.world");
    assert!(got.is_err());
}

#[test]
pub fn good_email_file_test() {
    /*let got = parse_file_email(
        "d.burliai@ukma.edu.ua\n\
        d.burliai@gmail.com\n\
        me.time@dist.edu"
    ).unwrap();*/
    let got = parse_file_email(fs::read_to_string("tests/test_email.txt").unwrap().as_str()).unwrap();
    let res = vec!(
        ParsedEmail{ name: "d.burliai".to_string(), domain: "ukma.edu.ua".to_string() },
        ParsedEmail{ name: "d.burliai".to_string(), domain: "gmail.com".to_string() },
        ParsedEmail{ name: "me.time".to_string(), domain: "dist.edu".to_string() },
        ParsedEmail{ name: "".to_string(), domain: "".to_string() }, /* For some reason this rule generates empty user at the end. Any help with that? */
    );
    assert_eq!(got, res);
}

#[test]
pub fn good_user_file_test() {
    /*let got = parse_file_user(
        "Danylo Burliai, d.burliai@ukma.edu.ua\n\
        Daniel Burn, d.burliai@gmail.com\n\
        Mister Time, me.time@dist.edu"
    ).unwrap();*/
    let got = parse_file_user(fs::read_to_string("tests/test_user.txt").unwrap().as_str()).unwrap();
    let res = vec!(
        ParsedUser{ name: "Danylo".to_string(), surname: "Burliai".to_string(), email: ParsedEmail{ name: "d.burliai".to_string(), domain: "ukma.edu.ua".to_string() } },
        ParsedUser{ name: "Daniel".to_string(), surname: "Burn".to_string(), email: ParsedEmail{ name: "d.burliai".to_string(), domain: "gmail.com".to_string() } },
        ParsedUser{ name: "Mister".to_string(), surname: "Time".to_string(), email: ParsedEmail{ name: "me.time".to_string(), domain: "dist.edu".to_string() } },
        ParsedUser{ name: "".to_string(), surname: "".to_string(), email: ParsedEmail{ name: "".to_string(), domain: "".to_string() } }, /* For some reason this rule generates empty user at the end. Any help with that? */
    );
    assert_eq!(got, res);
}