use my_parser_kma_group_3_DB::*;
use std::env;

pub fn main() -> anyhow::Result<()> {
    let args : Vec<String> = env::args().collect();

    // parse args & get res

    let res = ParsedEmail::new();
    println!("{:?}", res);
    Ok(())
}