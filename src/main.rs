extern crate rand;

mod gambling;

use gambling::dicecoins::Dicecoins;

fn main() {

    let d4 = Dicecoins::new(vec![1,2,3,4]);
    let d2 = Dicecoins::new(vec![4,2,1,3]);

    println!("Hello, world! Roll a standard d{}... result: {}", d4.face_count(), d4.roll_x(1));
    println!("Hello, world! Roll a standard d{}... max result: {}", d4.face_count(), d4.roll_max_x(1));
    println!("Hello, world! Roll a standard d{}... avg result: {}", d2.face_count(), d2.roll_avg_x(1));
    println!("Hello, world! Roll a standard d{}... min result: {}", d4.face_count(), d4.roll_min_x(1));
}
