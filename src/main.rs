// 猜数游戏
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
   println!("Guess the number!");
   let secret_num = rand::thread_rng().gen_range(1,101);
   loop {
        println!("Please input your guess.");
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Fail to read line");
        let num:u32 = match num.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("your num is: {}",num);
        match num.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("Win");
                break;
            },
        }
   }
}