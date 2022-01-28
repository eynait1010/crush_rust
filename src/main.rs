#[derive(Debug)]

enum IpAdd{
    V4(u8, u8, u8, u8),
    V6(String)
}
fn main (){
    let home = IpAdd::V4(127,0,0,1);
    let loopback = IpAdd::V6(String::from("::1"));
    println!("{:?}",home);
    let a = Some(5);
    let b:Option<i32> = None;
    let c:Option<i8> = Some(8);
    let d:Option<i8> = None;
}
  
