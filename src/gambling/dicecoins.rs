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

//This file defines how a Dicecoin is made, and its functions.
//Note the spirit of Dicecoin is based off standard RPG dice
//but higher is allowed.

/*
Limitations:
-User may not have face values greater than 100,000
-User may not have more than 10,000 faces
-User may not roll more than 9,999 dice at a time
-User may not have 0 faced dice, this will return a single sided dice with a value of 1

Guarantees:
Maximum valid roll = 9,999 * 10,000 * 100,000 = 9.999e+12 = 9,999,000,000,000
Minimum valid roll = 0
A request beyond these constraints will return a 0 or a roll with the max value.
In cases like a request for 100,000 dice rolls, this will roll the dice 9,999 tunes

User Facing Functions:
new(mut face_vals: Vec<u32>) -> Self (Creates dice with above constraints)
roll_x(&self, x_dice: u32) -> u64 (Randomly rolls die * dice rolled)
roll_max_x(&self, x_dice: u32) -> u64 (Rolls the min value * dice rolled)
roll_min_x(&self, x_dice: u32) -> u64 (Rolls the max value * dice rolled)
*/

use rand::{thread_rng,Rng};

const MAX_FACE_VALUE: u32 = 100000;
const MAX_FACE_COUNT: u32 = 10000;
const MAX_ROLL_COUNT: u32 = 9999;


pub struct Dicecoins {
    face_count: u32,
    face_values: Vec<u32>,
    min_value: u32,
    max_value: u32,
    avg_value: u32,
}

#[allow(dead_code)] //Probably not going to use all of these
impl Dicecoins {
    pub fn new(mut face_vals: Vec<u32>) -> Self {
        let mut min = 1; //compiler bug, initializing these here but never reading them should throw warning
        let mut max = 1; //since it is possible that they will never be read
        let mut avg = 0;

        if face_vals.len() == 0 { //No 0 sided dice allowed
            return Dicecoins {face_count: 1, face_values: vec![1], min_value: 1, max_value: 1, avg_value: 1}
        }

        //User may not have values greater than 100,000
        //Index access is safe since we use the for _ in _
        for x in 0..face_vals.len() {
            //Enforce the limit
            if face_vals[x] > MAX_FACE_VALUE {
                face_vals[x] = MAX_FACE_VALUE;
            }
            //Initially the first element is both the biggest and the smallest
            if x == 0 {
                max = face_vals[x];
                min = face_vals[x];
            }
            //Finds max element
            if max < face_vals[x] {
                max = face_vals[x];
            }
            //Finds min element
            if min > face_vals[x] {
                min = face_vals[x];
            }
            avg += face_vals[x]; //the sum divided by the total is the expected value
        }

        avg = avg/face_vals.len() as u32; //the sum divided by the total is the expected value

        //User may not have more than 10,000 faces
        if face_vals.len() > MAX_FACE_COUNT as usize {
            let new_vals: Vec<u32> = face_vals.split_off(MAX_FACE_COUNT as usize);
            Dicecoins {face_count: MAX_FACE_COUNT, face_values: new_vals, min_value: min,
                max_value: max, avg_value: avg}
        }
        else { //Normal range
            Dicecoins {face_count: face_vals.len() as u32, face_values: face_vals, min_value: min,
                max_value: max, avg_value: avg }
            }
    }

    //Rolls the dice the indicated number of times and returns the integer sum
    pub fn roll_x(&self, x_dice: u32) -> u64 {
        let mut sum = 0;

        //Reject roll request greater than 9,999 so we can guarantee max return is 99,990,000
        if x_dice < MAX_ROLL_COUNT {
            for _x in 0..x_dice {
                sum += self.roll();
            }
        }
        else {
            for _x in 0..MAX_ROLL_COUNT { //Roll to the limit instead
                sum += self.roll();
            }
        }

        sum
    }

    //Rolls the dice the indicated number of times and returns the integer sum
    pub fn roll_max_x(&self, x_dice: u32) -> u64 {
        let sum;

        //Cap roll request greater than 9,999 so we can guarantee max return is 99,990,000
        if x_dice <= MAX_ROLL_COUNT {
            sum = self.max_value as u64 * x_dice as u64;
        }
        else {
            sum = self.max_value as u64 * MAX_ROLL_COUNT as u64;
        }

        sum
    }

    //Rolls the dice the indicated number of times and returns the integer sum
    pub fn roll_avg_x(&self, x_dice: u32) -> u64 {
        let sum;

        //Cap roll request greater than 9,999 so we can guarantee max return is 99,990,000
        if x_dice <= MAX_ROLL_COUNT {
            sum = self.avg_value as u64  * x_dice as u64;
        }
        else {
            sum = self.avg_value as u64 * MAX_ROLL_COUNT as u64;
        }
        sum
    }

    //Rolls the dice the indicated number of times and returns the integer sum
    pub fn roll_min_x(&self, x_dice: u32) -> u64 {
        let sum;

        //Cap roll request greater than 9,999 so we can guarantee max return is 99,990,000
        if x_dice <= MAX_ROLL_COUNT {
            sum = self.min_value as u64 * x_dice as u64;
        }
        else {
            sum = self.min_value as u64 * MAX_ROLL_COUNT as u64;
        }

        sum
    }

    //Get number of faces
    pub fn face_count(&self) -> u32 {
        self.face_count
    }

    //Rolls a dicecoin
    fn roll(&self) -> u64 {
        let index; //this is used to pick a face

        //This cast is ok because face_count will never be higher than 10,000
        index = thread_rng().gen_range(0, self.face_count) as usize; //Which face was rolled?
        //u32 to u64 is always ok
        self.face_values[index] as u64
    }
}

//Tests on binary file should always be in the source file and not in a tests directory. That is for
//libraries. I really wish somebody had told me this.
#[cfg(test)]
mod tests {
    use super::Dicecoins;
    #[test]
    fn dicecoin_min() {
        assert_eq!(1, Dicecoins::new(vec![3,2,1,4]).roll_min_x(1));
    }

    #[test]
    #[ignore] //Use 'test -- --ignored' when calling to call all tests with #[ignore]
    fn dicecoin_max() {
        assert_eq!(4, Dicecoins::new(vec![3,4,1,2]).roll_max_x(1));
    }

    #[test]
    #[ignore]
    fn dicecoin_avg() {
        assert_eq!(2, Dicecoins::new(vec![3,4,1,2]).roll_avg_x(1));
    }

    #[test]
    #[ignore]
    fn dicecoin_roll_max() {
        assert_eq!(9999, Dicecoins::new(vec![1]).roll_x(10000000));
    }

    #[test]
    #[ignore]
    fn dicecoin_max_roll_max() {
        assert_eq!(9999, Dicecoins::new(vec![1]).roll_max_x(10000000));
    }

    #[test]
    #[ignore]
    fn dicecoin_min_roll_max() {
        assert_eq!(9999, Dicecoins::new(vec![1]).roll_min_x(10000000));
    }

    #[test]
    #[ignore]
    fn dicecoin_avg_roll_max() {
        assert_eq!(9999, Dicecoins::new(vec![1]).roll_avg_x(10000000));
    }

    #[test]
    #[ignore]
    fn dicecoin_roll_0() {
        assert_eq!(0, Dicecoins::new(vec![1]).roll_x(0));
    }

    #[test]
    #[ignore]
    fn dicecoin_max_roll_0() {
        assert_eq!(0, Dicecoins::new(vec![1]).roll_max_x(0));
    }

    #[test]
    #[ignore]
    fn dicecoin_min_roll_0() {
        assert_eq!(0, Dicecoins::new(vec![1]).roll_min_x(0));
    }

    #[test]
    #[ignore]
    fn dicecoin_avg_roll_0() {
        assert_eq!(0, Dicecoins::new(vec![1]).roll_avg_x(0));
    }

    #[test]
    #[ignore]
    fn dicecoin_no_face() {
        assert_eq!(1, Dicecoins::new(vec![]).face_count());
    }

    #[test]
    #[ignore]
    fn dicecoin_max_faces() {
        //We need to load up a vector with over MAX_FACE_COUNT (10,000) elements
        //I don't want to figure out how to make that accessible here so this should be changed to be
        //the same as the above reference plus at least 1
        let mut test_vec: Vec<u32> = Vec::new();

        for _x in 0..10101 {
            test_vec.push(1);
        }
        assert_eq!(10000, Dicecoins::new(test_vec).face_count());
    }

    #[test]
    #[ignore]
    fn dicecoin_max_face_val() {
        assert_eq!(100000, Dicecoins::new(vec![900001]).roll_min_x(1));
    }

    #[test]
    #[ignore]
    fn dicecoin_min_face_val() {
        assert_eq!(0, Dicecoins::new(vec![0]).roll_min_x(1));
    }
}