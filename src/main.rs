fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    let mut s2 = String::from("hello");

    {
        let r4 = &mut s2;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r5 = &mut s2;
    println!("{}",  r5);

    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
}
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
