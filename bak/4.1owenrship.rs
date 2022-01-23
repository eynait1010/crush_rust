fn main() {
  // heap: move
  let s1 = String::from("hello");
  let s2 = s1;
  // println!("{}{}, world!", s1, s2);// borrow of moved value: `s1`

  // heap: clone
  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("s1 = {}, s2 = {}", s1, s2);
  
  // stack clone
  let x = 5;
  let y = x;

  println!("x = {}, y = {}", x, y);
}
