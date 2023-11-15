use anyhow::anyhow;
//use my_parser_kma_group_3_DB::*;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
struct Grammar;

pub fn main() -> anyhow::Result<()> {
    //println!("{:?}", list_parser::list("[1,1,2,3,5,8]"));

    Ok(())
}

#[test]
pub fn basic_test() -> anyhow::Result<()> {
    let got = Grammar::parse(Rule::field, "131.13")?.next().ok_or_else(|| anyhow!("no pair"))?;
    dbg!(got);

    Ok(())
}