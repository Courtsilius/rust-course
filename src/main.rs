use std::str::FromStr;

fn boo() -> i32 {
    std::env::args()
        .nth(1)
        .map(|s: String| {
            i32::from_str(&s)
        })
        .unwrap_or(Ok(0))
        .unwrap_or_else(|err| {
            eprintln!("Error while parsing: {}", err);
            0
        })
}

fn main() {
    let x = boo();
    println!("{x}");

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

