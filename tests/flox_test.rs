extern crate xflow;
use xflow::*;

#[test]
fn test_flox_arithmetic() {

    match flox::parse("1+2") {
        Err(_) => assert!(true),
        Ok(res) => {
            match res {
                flox::FloxOutput::Integer(res) => assert_eq!(res, 3),
                _ => assert!(false),
            }
        }
    }

}
