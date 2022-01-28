#[derive(Debug)]

enum Coin {
    Penny,
    Nickel,
    Quarter(String)
}
fn main (){
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Quarter(String::from("test")));
    plus_one(Some(5));
    plus_one(None);
}

fn value_in_cents(coin:Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => {
            println!("nickel");
            5
        },
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(num)=> Some(num+1)
    }
}