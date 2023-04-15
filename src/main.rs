use std::str::FromStr;

fn boo(args: Vec<String>) -> i32 {
    args
        .iter()
        .nth(1)
        .map(|s: &String| {
            i32::from_str(&s)
        })
        .unwrap_or(Ok(0))
        .unwrap_or_else(|err| {
            eprintln!("Error while parsing: {}", err);
            0
        })
}

fn programm(args: Vec<String>) {
    let x = boo(args);
    println!("{x}")
}

fn main() {
    let args = std::env::args().collect();
    programm(args);
    //let mut kek = std::env::args();
    //let first = kek.nth(1).unwrap_or_else(|| {
    //    String::from("0")
    //});
    // println!("{first}");

    /*


    let f = std::env::args()
        .nth(1)
        .map(|s: String| {
            i32::from_str(&s)
        })
        .unwrap_or(Ok(0))
        .unwrap_or_else(|err| {
            eprintln!("Error while parsing: {}", err);
            0
        });

    println!("{f}");

     */

    /*
    //list
    let mut list = Vec::new();
    list.push(1);

    for elem in list.iter() {
        println!("{elem}");
    }
    */
}

