fn main() {
    let a = "hello world";
    let b = String::from("Hello World");
   
    // before
    let World = first_word(&b);
    println!("{}",World);
    // after
     let world = first_word_after(a);
     // ? TODO:
     let world2 = first_word_after(&a[..]);
    let World =first_word_after(&b[..]);
    println!("{}{}{}",world,World,world2)
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    // iter is a method that returns each element in a collection  
    // enumerate wraps the result of iter and returns each element as part of a tuple instead. 
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]

}
fn first_word_after(s: &str) -> &str {
    let bytes = s.as_bytes();
    // iter is a method that returns each element in a collection  
    // enumerate wraps the result of iter and returns each element as part of a tuple instead. 
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]

}