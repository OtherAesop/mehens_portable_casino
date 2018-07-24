//This file defines how a Dicecoin is made, and its functions.
//Note the spirit of Dicecoin is based off standard RPG dice
//but higher is allowed.

/*
Limitations:
-User may not have face values greater than 100,000
-User may not have more than 10,000 faces
-User may not roll more than 9,999 dice at a time

Guarantees:
Maximum valid roll = 9,999 * 10,000 * 100,000 = 9.999e+12 = 9,999,000,000,000
Minimum valid roll = 1
A request beyond these constraints will return a 0

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
        let mut min = 1;
        let mut max = 1;
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

        //Reject roll request greater than 9,999 so we can guarantee max return is 99,990,000
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

        //Reject roll request greater than 9,999 so we can guarantee max return is 99,990,000
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

        //Reject roll request greater than 9,999 so we can guarantee max return is 99,990,000
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

#[cfg(test)]
mod tests {
    use super::Dicecoins;
    #[test]
    fn dicecoin_declare() {
        assert_eq!(1, Dicecoins::new(vec![1,2,3,4]).roll_min_x(1)); //<----Why not in scope????
    }
}