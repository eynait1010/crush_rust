fn main() {
  let tup = (500, 6.4, 1);
  // pattern matching to destructure
  let (x, y, z) = tup;

  println!("{} {} {}", x,y,z);

  let five_hundred = tup.0;
  let six_point_four = tup.1;
  let one = tup.2;

  println!("{} {} {}",five_hundred,six_point_four,one);

  let _a = [1, 2, 3, 4, 5];
  let _b: [i32; 5] = [1, 2, 3, 4, 5];
  let _c = [3; 5];

  let first = _a[0];
  let second = _a[1];

  println!("{}{}",first,second)
}