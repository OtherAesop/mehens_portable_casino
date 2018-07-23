extern crate rand;

mod gambling;

use gambling::dicecoins::{DiceType, Dicecoins};

fn main() {
    let d2: DiceType = DiceType::D2;
    let d4 = Dicecoins::new(DiceType::D4, vec![1,2,3,4]);
    println!("Hello, world!");
}
