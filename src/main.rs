use my_corp_parser::*;
use std::{fs, env};

fn read_file(path: &str) -> String {
    let data = fs::read_to_string(path).unwrap();
    data
}

fn main() -> anyhow::Result<()> {
    let args : Vec<String> = env::args().collect();

    //println!("{}", args.len()); idk about windows, but my mac shows 2 cli by default if launched via cargo run, so i'm not checking len

    if args.len() <= 2 {
        println!("To use cli write text or filename as last arg. Use flag --file to parse file content or --email to parse email instead of user. Use --credits to display credits");
        return Ok(());
    }

    let mut is_file = false;
    let mut is_user = true;
    let mut s = String::new();

    for arg in args {
        match arg.as_str() {
            "--file" => { is_file = true; },
            "--email" => { is_user = false; },
            "--credits" => {
                println!("My_Corp_Parser. Parsing your organization's data in no time!");
                println!("Version 1.0.1");
                println!("This parser was made by Danylo Burliai for educational purposes at kma rust course");
                return Ok(());
            },
            _ => { s = arg; },
        }
    }

    if is_file {
        if is_user {
            println!("{:?}", parse_file_user(read_file(s.as_str()).as_str()));
        } else {
            println!("{:?}", parse_file_email(read_file(s.as_str()).as_str()));
        }
    } else {
        if is_user {
            println!("{:?}", parse_user(s.as_str()));
        } else {
            println!("{:?}", parse_email(s.as_str()));
        }
    }

    Ok(())
}