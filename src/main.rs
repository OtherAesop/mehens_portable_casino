// mehens_portable_casino. A gambling game made using ggez and Dicecoin
// Copyright (C) 2018  Ian L. Gore
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//Go find the crate via what is specified in Cargo.toml
extern crate rand;

//We need to import the gambling package
mod gambling;
//import the Dicecoins namespace
use gambling::dicecoins::Dicecoins;

fn main() {

    let d4 = Dicecoins::new(vec![1,2,3,4]);
    let d2 = Dicecoins::new(vec![4,2,1,3]);

    println!("Hello, world! Roll a standard d{}... result: {}", d4.face_count(), d4.roll_x(1));
    println!("Hello, world! Roll a standard d{}... max result: {}", d4.face_count(), d4.roll_max_x(1));
    println!("Hello, world! Roll a standard d{}... avg result: {}", d2.face_count(), d2.roll_avg_x(1));
    println!("Hello, world! Roll a standard d{}... min result: {}", d4.face_count(), d4.roll_min_x(1));
}
