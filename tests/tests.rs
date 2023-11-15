use my_parser_kma_group_3_DB::*;

#[test]
pub fn good_email_test() {
    let got = parse_email("d.burliai@ukma.edu.ua").unwrap();
    dbg!(got);
}

#[test]
#[should_panic]
pub fn bad_email_no_name_test() {
    parse_email(".burliai@ukma.edu.ua").unwrap();
}