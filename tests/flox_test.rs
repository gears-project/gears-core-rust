extern crate xflow;
use xflow::*;

fn expect_ok_integer(input: &str, expected: i64) -> () {
    match flox::parse(input) {
        Err(_) => assert!(false),
        Ok(res) => {
            match res {
                flox::FloxResult::Integer(res) => assert_eq!(res, expected),
                _ => assert!(false),
            }
        }
    }
}

#[test]
fn test_flox_arithmetic() {
    expect_ok_integer("1+2", 3);
    expect_ok_integer("1 + 2", 3);
    // expect_ok_integer("1+2+3", 6);
    expect_ok_integer("1*2", 2);
    expect_ok_integer("1 * 2", 2);
    expect_ok_integer("11+255", 266);
    expect_ok_integer("11 + 255", 266);
}
