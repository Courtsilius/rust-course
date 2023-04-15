use std::env;
use std::str::FromStr;

fn main() {
    let s = "1";

    let a = i32::from_str(s);
    let kek = env::args();
    for argument in kek {
        println!("{argument}");
    }
}

