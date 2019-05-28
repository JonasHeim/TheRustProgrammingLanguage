enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    
    println!("Penny value in cents: {}", value_in_cents(Coin::Penny));
    println!("Nickel value in cents: {}", value_in_cents(Coin::Nickel));
    println!("Dime value in cents: {}", value_in_cents(Coin::Dime));
    println!("Quarter value in cents: {}", value_in_cents(Coin::Quarter));

}
