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

/*
Player Actions: these are the functions that can be called or fields that can be accessed, separated by category
###Dicecoin Bank###
- Player.dX_count: where x is the number of faces on the dice this will access the current count the player holds of that dice and allow it to be changed

###Dicecoins in betting pool###
pub fn bet_dice(dice: DiceType) -> bool - Subtracts a dice from the Dicecoin Bank and adds it to the Dicecoins gambled, returns true if possible and false if not
pub fn get_dice(dice_vec: &Vec<DiceType>) -> bool - Adds all dice in dice_vec to the players bank if possible and returns true if no overflow, false otherwise. Doesn't clear passed Vec for modularity
pub fn check_bet() -> &Vec<DiceType> - returns an immutable reference to the dice being bet
pub fn clear_bet() - empties the dice being bet

###Dicecoins in rolling pool###
pub fn roll_dice() -> bool -- attempts to roll all dice in rolling pool and sets roll_result to the total roll, this also clears the rolling pool. Returns true if no overflow, false otherwise
pub fn bet_rolling_dice(dice: DiceType) -> bool - Subtracts a dice from the Dicecoin Bank and adds it to the Dicecoins in the rolling pool, returns true if possible and false if not
pub fn check_rolling_dice() -> &Vec<DiceType> - returns an immutable reference to the dice in the rolling pool
pub fn clear_rolling_dice() - empties the dice in the rolling pool

###Dicecoin utility functions###
    pub fn check_dice_total(&self) -> &u32 - returns the total number of held dice
    fn update_dice_total (&mut self) - called when there is a change to dice numbers to update the dice total
    clear_roll_result(&mut self) - clears the roll result
*/

/*
This defines a player struct that simplifies the job of holding player specific information and defines
actions the player can take.
*/

//Need Dicecoin definitions here
use gambling::dice_type::DiceType;
use gambling::dicecoins::Dicecoins;

const MAX_GAMBLE_DICE: usize    = 8; //Need to compare to Vec's len() return result
const MAX_ROLL_DICE: usize      = 2;
const MAX_DICE_HELD: u32        = 9; //Need to compare to u32 values
const MAX_ROLL_VALUE: u64       = 9999; //Need to compare to Dicecoins roll return value which is u64

pub struct Player {
    //Dicecoin canonical definition (These are the dicecoins as defined by my project)
    d2: Dicecoins,
    d4: Dicecoins,
    d6: Dicecoins,
    d8: Dicecoins,
    d10: Dicecoins,
    d10p: Dicecoins,
    d12: Dicecoins,
    d20: Dicecoins,
    //Dicecoin bank (These are the total amount of various Dicecoins the player has)
    pub d2_count: u32,
    pub d4_count: u32,
    pub d6_count: u32,
    pub d8_count: u32,
    pub d10_count: u32,
    pub d10p_count: u32,
    pub d12_count: u32,
    pub d20_count: u32,
    pub total_dice: u32,
    //Dicecoin in rolling pool (is that a word?) (These are the Dicecoins being set to be rolled)
    rolling_dice: Vec<DiceType>,
    pub roll_result: u64,
    //Dicecoins gambled (as in like a cash bet) and roll result
    gambled_dice: Vec<DiceType>,
}

//Removing this can cause a bunch of unused warnings because the unused warning is applied transitively
//I.E. if a function is unused in something else here, everything in it is unused as well and triggers a warning
//See here for more https://github.com/rust-lang/rust/issues/18618#issuecomment-61709955
#[allow(unused)]
impl Player {
    pub fn new() -> Self {
        Player {
            //Dicecoin canonical definition (These are the dicecoins as defined by my project)
            d2: Dicecoins::new(vec![1, 4000]),
            d4: Dicecoins::new(vec![1, 750, 1500, 2000]),
            d6: Dicecoins::new(vec![1, 200, 400, 600, 800, 1000]),
            d8: Dicecoins::new(vec![1, 15, 30, 45, 60, 75, 90, 500]),
            d10: Dicecoins::new(vec![1, 20, 30, 40, 50, 60, 70, 80, 90, 100]),
            d10p: Dicecoins::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            d12: Dicecoins::new(vec![1, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 50]),
            d20: Dicecoins::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]),
            //Dicecoin bank (These are the total amount of various Dicecoins the player has)
            d2_count: 0,
            d4_count: 0,
            d6_count: 0,
            d8_count: 0,
            d10_count: 0,
            d10p_count: 1,
            d12_count: 1,
            d20_count: 1,
            total_dice: 3,
            //Dicecoin in rolling pool (is that a word?) (These are the Dicecoins being set to be rolled)
            //and roll result
            rolling_dice: Vec::<DiceType>::new(),
            roll_result: 0,
            //Dicecoins gambled (as in like a cash bet)
            gambled_dice: Vec::<DiceType>::new(),

        }
    }
    //###Dicecoins in betting pool###
    //Subtracts a dice from the Dicecoin Bank and adds it to the Dicecoins gambled, returns true if possible and false if not
    pub fn bet_dice(&mut self, dice: DiceType) -> bool {
        let retval = match dice {
            DiceType::D2 => {
                if self.d2_count > 0 && self.gambled_dice.len() < MAX_GAMBLE_DICE {
                    self.d2_count -= 1;
                    self.gambled_dice.push(DiceType::D2);
                    true
                } else {
                    false
                }
            },
            DiceType::D4 => {
                if self.d4_count > 0 && self.gambled_dice.len() < MAX_GAMBLE_DICE {
                    self.d4_count -= 1;
                    self.gambled_dice.push(DiceType::D4);
                    true
                } else {
                    false
                }
            },
            DiceType::D6 => {
                if self.d6_count > 0 && self.gambled_dice.len() < MAX_GAMBLE_DICE {
                    self.d6_count -= 1;
                    self.gambled_dice.push(DiceType::D6);
                    true
                } else {
                    false
                }
            },
            DiceType::D8 => {
                if self.d8_count > 0 && self.gambled_dice.len() < MAX_GAMBLE_DICE {
                    self.d8_count -= 1;
                    self.gambled_dice.push(DiceType::D8);
                    true
                } else {
                    false
                }
            },
            DiceType::D10 => {
                if self.d10_count > 0 && self.gambled_dice.len() < MAX_GAMBLE_DICE {
                    self.d10_count -= 1;
                    self.gambled_dice.push(DiceType::D10);
                    true
                } else {
                    false
                }
            },
            DiceType::D10p => {
                if self.d10p_count > 0 && self.gambled_dice.len() < MAX_GAMBLE_DICE {
                    self.d10p_count -= 1;
                    self.gambled_dice.push(DiceType::D10p);
                    true
                } else {
                    false
                }
            },
            DiceType::D12 => {
                if self.d12_count > 0 && self.gambled_dice.len() < MAX_GAMBLE_DICE {
                    self.d12_count -= 1;
                    self.gambled_dice.push(DiceType::D12);
                    true
                } else {
                    false
                }
            },
            DiceType::D20 => {
                if self.d20_count > 0 && self.gambled_dice.len() < MAX_GAMBLE_DICE {
                    self.d20_count -= 1;
                    self.gambled_dice.push(DiceType::D20);
                    true
                } else {
                    false
                }
            },
            //_               => panic!("Unhandled DiceType in bet_dice"),
        };

        //Need to keep track of dice total
        self.update_dice_total();

        retval
    }

    //Adds all dice in dice_vec to the players bank if possible and returns true if no overflow, false otherwise
    //We don't clear dice_vec here because clearing other players variables is weird
    pub fn get_dice(&mut self, dice_vec: &Vec<DiceType>) -> bool {
        let mut retval = true;

        //DEBUG
        //if !dice_vec.is_empty() {
        for dice in dice_vec.iter() {
            match dice {
                DiceType::D2    => { if self.d2_count < MAX_DICE_HELD { self.d2_count += 1;} else { retval = false; break; } }
                DiceType::D4    => { if self.d4_count < MAX_DICE_HELD { self.d4_count += 1;} else { retval = false; break; } }
                DiceType::D6    => { if self.d6_count < MAX_DICE_HELD { self.d6_count += 1;} else { retval = false; break; } }
                DiceType::D8    => { if self.d8_count < MAX_DICE_HELD { self.d8_count += 1;} else { retval = false; break; } }
                DiceType::D10   => { if self.d10_count < MAX_DICE_HELD { self.d10_count += 1;} else { retval = false; break; } }
                DiceType::D10p  => { if self.d12_count < MAX_DICE_HELD { self.d12_count += 1;} else { retval = false; break; } }
                DiceType::D12   => { if self.d10p_count < MAX_DICE_HELD { self.d10p_count += 1;} else { retval = false; break; } }
                DiceType::D20   => { if self.d20_count < MAX_DICE_HELD { self.d20_count += 1;} else { retval = false; break; } }
                _               => panic!("Unhandled DiceType in get_dice"),
            }
        }
        //}

        //Need to keep track of dice total
        self.update_dice_total();

        retval
    }

    //Returns immutable reference to gambled dice
    pub fn check_bet(&self) -> &Vec<DiceType> { &self.gambled_dice }

    //Clears gambled dice
    pub fn clear_bet(&mut self) { self.gambled_dice.clear(); }

    //###Dicecoins in rolling pool###

    //Attempts to roll all dice in rolling pool and sets roll_result to the total roll,
    //this also clears the rolling pool. Returns true if no overflow on max roll value, sets roll_result to MAX_ROLL_VALUE
    pub fn roll_dice(&mut self) -> bool {
        let mut retval = true;

        //clear the previous result
        self.roll_result = 0;

        for dice in self.rolling_dice.iter() {
            let temp_result = match dice {
                DiceType::D2    => { self.d2.roll_x(1) }
                DiceType::D4    => { self.d4.roll_x(1) }
                DiceType::D6    => { self.d6.roll_x(1) }
                DiceType::D8    => { self.d8.roll_x(1) }
                DiceType::D10   => { self.d10.roll_x(1) }
                DiceType::D10p  => { self.d10p.roll_x(1) }
                DiceType::D12   => { self.d12.roll_x(1) }
                DiceType::D20   => { self.d20.roll_x(1) }
                _               => panic!("Unhandled DiceType in roll_dice"),
            };

            if temp_result + self.roll_result > MAX_ROLL_VALUE {
                self.roll_result = MAX_ROLL_VALUE as u64; //u32 -> u64
                retval = false;
                break;
            } else {
                self.roll_result += temp_result;
            }
        }

        self.rolling_dice.clear();

        retval
    }


    pub fn bet_rolling_dice(&mut self, dice: DiceType) -> bool {
        let retval = match dice {
            DiceType::D2 => {
                if self.d2_count > 0 && self.rolling_dice.len() < MAX_ROLL_DICE {
                    self.d2_count -= 1;
                    self.rolling_dice.push(DiceType::D2);
                    true
                } else {
                    false
                }
            },
            DiceType::D4 => {
                if self.d4_count > 0 && self.rolling_dice.len() < MAX_ROLL_DICE {
                    self.d4_count -= 1;
                    self.rolling_dice.push(DiceType::D4);
                    true
                } else {
                    false
                }
            },
            DiceType::D6 => {
                if self.d6_count > 0 && self.rolling_dice.len() < MAX_ROLL_DICE {
                    self.d6_count -= 1;
                    self.rolling_dice.push(DiceType::D6);
                    true
                } else {
                    false
                }
            },
            DiceType::D8 => {
                if self.d8_count > 0 && self.rolling_dice.len() < MAX_ROLL_DICE {
                    self.d8_count -= 1;
                    self.rolling_dice.push(DiceType::D8);
                    true
                } else {
                    false
                }
            },
            DiceType::D10 => {
                if self.d10_count > 0 && self.rolling_dice.len() < MAX_ROLL_DICE {
                    self.d10_count -= 1;
                    self.rolling_dice.push(DiceType::D10);
                    true
                } else {
                    false
                }
            },
            DiceType::D10p => {
                if self.d10p_count > 0 && self.rolling_dice.len() < MAX_ROLL_DICE {
                    self.d10p_count -= 1;
                    self.rolling_dice.push(DiceType::D10p);
                    true
                } else {
                    false
                }
            },
            DiceType::D12 => {
                if self.d12_count > 0 && self.rolling_dice.len() < MAX_ROLL_DICE {
                    self.d12_count -= 1;
                    self.rolling_dice.push(DiceType::D12);
                    true
                } else {
                    false
                }
            },
            DiceType::D20 => {
                if self.d20_count > 0 && self.rolling_dice.len() < MAX_ROLL_DICE {
                    self.d20_count -= 1;
                    self.rolling_dice.push(DiceType::D20);
                    true
                } else {
                    false
                }
            },
            //_               => panic!("Unhandled DiceType in bet_dice"),
        };

        //Need to keep track of dice total
        self.update_dice_total();

        retval
    }
    pub fn check_rolling_dice(&self) -> &Vec<DiceType> { &self.rolling_dice }
    pub fn clear_rolling_dice(&mut self) { self.rolling_dice.clear(); }

    pub fn check_dice_total(&self) -> &u32 {
        &self.total_dice
    }

    fn update_dice_total (&mut self) {
        self.total_dice = self.d2_count + self.d4_count + self.d6_count + self.d8_count + self.d10_count + self.d10p_count + self.d12_count + self.d20_count;
    }

    pub fn clear_roll_result (&mut self) {
        self.roll_result = 0;
    }
}