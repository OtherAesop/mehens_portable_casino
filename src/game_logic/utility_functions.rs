// mehens_portable_casino. A gambling game made using ggez and Dicecoin
// Copyright (C) 2018  Ian Gore
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

//My imports
use SceneType;
use gambling::dice_type::DiceType;
use game_logic::turns::Turn;
use game_logic::phase::Phase;
use game_logic::player::Player;

//Ggez
use ggez::conf::{WindowSetup, WindowMode};
use ggez::{ContextBuilder, Context};
use ggez::graphics::{Point2, DrawParam};
use ggez::timer;
use ggez::audio::Source;

//Std
use std::env::var;
use std::path::PathBuf;

//Rand
use rand::{thread_rng,Rng};

//Distributes the winnings to the winner, returns false if there was an overflow error
#[allow(unused)] // retval really isn't useful here at all but we always want to have the option to check return values
pub fn win(p1: &mut Player, p2: &mut Player, winner: &Turn) -> bool{
    let mut retval = true;

    //Clones p1 and p2's dice info so we don't need to mutably borrow it
    //Ugly but functional for now
    let mut p1_temp_dice: Vec<DiceType> = Vec::new();
    for x in p1.check_bet().iter() {
        match x {
            DiceType::D2    => p1_temp_dice.push(DiceType::D2),
            DiceType::D4    => p1_temp_dice.push(DiceType::D4),
            DiceType::D6    => p1_temp_dice.push(DiceType::D6),
            DiceType::D8    => p1_temp_dice.push(DiceType::D8),
            DiceType::D10   => p1_temp_dice.push(DiceType::D10),
            DiceType::D10p  => p1_temp_dice.push(DiceType::D10p),
            DiceType::D12   => p1_temp_dice.push(DiceType::D12),
            DiceType::D20   => p1_temp_dice.push(DiceType::D20),
            _             => panic!("Unhandled DiceType in winner - p1"),
        }
    }
    p1.clear_bet();

    for x in p1.check_rolling_dice().iter() {
        match x {
            DiceType::D2    => p1_temp_dice.push(DiceType::D2),
            DiceType::D4    => p1_temp_dice.push(DiceType::D4),
            DiceType::D6    => p1_temp_dice.push(DiceType::D6),
            DiceType::D8    => p1_temp_dice.push(DiceType::D8),
            DiceType::D10   => p1_temp_dice.push(DiceType::D10),
            DiceType::D10p  => p1_temp_dice.push(DiceType::D10p),
            DiceType::D12   => p1_temp_dice.push(DiceType::D12),
            DiceType::D20   => p1_temp_dice.push(DiceType::D20),
            _             => panic!("Unhandled DiceType in winner - p1"),
        }
    }
    p1.clear_rolling_dice();

    let mut p2_temp_dice: Vec<DiceType> = Vec::new();;
    for y in p2.check_bet().iter() {
        match y {
            DiceType::D2    => p2_temp_dice.push(DiceType::D2),
            DiceType::D4    => p2_temp_dice.push(DiceType::D4),
            DiceType::D6    => p2_temp_dice.push(DiceType::D6),
            DiceType::D8    => p2_temp_dice.push(DiceType::D8),
            DiceType::D10   => p2_temp_dice.push(DiceType::D10),
            DiceType::D10p  => p2_temp_dice.push(DiceType::D10p),
            DiceType::D12   => p2_temp_dice.push(DiceType::D12),
            DiceType::D20   => p2_temp_dice.push(DiceType::D20),
            _             => panic!("Unhandled DiceType in winner - p1"),
        }
    }
    p2.clear_bet();

    for y in p2.check_rolling_dice().iter() {
        match y {
            DiceType::D2    => p2_temp_dice.push(DiceType::D2),
            DiceType::D4    => p2_temp_dice.push(DiceType::D4),
            DiceType::D6    => p2_temp_dice.push(DiceType::D6),
            DiceType::D8    => p2_temp_dice.push(DiceType::D8),
            DiceType::D10   => p2_temp_dice.push(DiceType::D10),
            DiceType::D10p  => p2_temp_dice.push(DiceType::D10p),
            DiceType::D12   => p2_temp_dice.push(DiceType::D12),
            DiceType::D20   => p2_temp_dice.push(DiceType::D20),
            _             => panic!("Unhandled DiceType in winner - p1"),
        }
    }
    p2.clear_rolling_dice();


    //Due to guards deeper below, an overflow in score (which is impossible anyway) will only
    //cause the score to stay at the max value. Values checked because good programming.
    match winner {
        Turn::Player1 => {
            //Add P1's dice back to their bank
            retval = p1.get_dice(&p1_temp_dice);
            //Add P2's dice to P1's bank
            retval = p1.get_dice(&p2_temp_dice);
        },
        Turn::Player2 => {
            //Add P2's dice back to their bank
            retval = p2.get_dice(&p2_temp_dice);
            //Add P1's dice to P2's bank
            retval = p2.get_dice(&p1_temp_dice);
        },
        _             => panic!("Unhandled Turn in winner - winner match statement"),
    }

    retval
}

//Evaluates the game on the Coos condition, returns the winner
pub fn evaluate_coos (p1: &Player, p2: &Player) -> Turn {
    let retval;

    if p1.roll_result > p2.roll_result {
        retval = Turn::Player2;
    } else if p1.roll_result < p2.roll_result {
        retval = Turn::Player1;
    } else {
        //[0,1], randomly determines winner since they are equal
        retval = match thread_rng().gen_range(0, 2) {
            0 => Turn::Player1,
            1 => Turn::Player2,
            _ => panic!("Unhandled player case in high_roller"),
        }
    }

    retval
}

//Evaluates the game on the Pearls condition, returns the winner
pub fn evaluate_pearls (p1: &Player, p2: &Player) -> Turn {
    let retval;

    if p1.roll_result > p2.roll_result {
        retval = Turn::Player1;
    } else if p1.roll_result < p2.roll_result {
        retval = Turn::Player2;
    } else {
        //[0,1], randomly determines winner since they are equal
        retval = match thread_rng().gen_range(0, 2) {
            0 => Turn::Player1,
            1 => Turn::Player2,
            _ => panic!("Unhandled player case in high_roller"),
        }
    }

    retval

}

//Checks players for conditions required to advance a players turn. True when ready to advance
//Note since the third phase can end only after a player makes a choice this can only check
//for two conditions
pub fn check_advance_conditions (p1: &Player) -> (bool,bool) {
    let mut betting_flag = false;
    let mut raising_flag = false;
    let rolling_dice_num = p1.check_rolling_dice().len(); // Checks number of dice in rolling pool
    let betted_dice_num = p1.check_bet().len(); //Checks number of dice betted

    //DEBUG: println!("# of rolling dice {}", rolling_dice_num.clone());
    //DEBUG: println!("# of betted dice {}", betted_dice_num.clone());
    if rolling_dice_num > 0 && rolling_dice_num < 3 { // [1,2]
        betting_flag = true;
    }
    if betted_dice_num < 9 {
        raising_flag = true;
    }

    (betting_flag, raising_flag)
}

//Determines who the highest better is between two players or decides randomly if even
//Again, turns are used because they are an exhaustive list of all possible players
pub fn high_roller(p1: &Player, p2: &Player) -> Turn {
    let p1_dice_count = if p1.check_bet().is_empty() { 0 } else { p1.check_bet().len() };
    let p2_dice_count = if p2.check_bet().is_empty() { 0 } else { p2.check_bet().len() };
    let retval;

    if p1_dice_count > p2_dice_count {
        retval = Turn::Player1;
    } else if p1_dice_count < p2_dice_count{
        retval = Turn::Player2;
    } else {
        //[0,1], randomly determines high roller
        retval = match thread_rng().gen_range(0, 2) {
            0 => Turn::Player1,
            1 => Turn::Player2,
            _ => panic!("Unhandled player case in high_roller"),
        }
    }

    retval
}

//Transitions turnphases appropriately and returns new turnphase
#[allow(unreachable_patterns)]
pub fn transition_turnphase (turn: &Turn, phase: &Phase) -> (Turn, Phase){
    let turnphase = (turn, phase);

    match turnphase {
        (Turn::Player1, Phase::Betting)   => (Turn::Player2, Phase::Betting),
        (Turn::Player2, Phase::Betting)   => (Turn::Player1, Phase::Raising),
        (Turn::Player1, Phase::Raising)   => (Turn::Player2, Phase::Raising),
        (Turn::Player2, Phase::Raising)   => (Turn::Player1, Phase::Rolling),
        (Turn::Player1, Phase::Rolling)   => (Turn::Player2, Phase::Rolling),
        (Turn::Player2, Phase::Rolling)   => (Turn::Player1, Phase::Betting),
        _               => panic!("Unhandled turnphase in transition_turnphase."),
    }
}

//Prevents the player from queueing up noises when an audio source is played with this
pub fn safe_play (sound: &Source) {
    if !sound.playing() {
        sound.play().expect("Cannot play sound inside safe_play");
    }
}

//Controls the floating animation
//go_up = true means go upwards, false means go downwards
//Delta correction from Line 210 of https://github.com/maccam912/thewizzerofoz/blob/master/src/main.rs
pub fn float_animation(max_x: f32, min_x: f32, speed: f32, offset: f32, go_up: bool, ctx: &Context) -> (bool,f32) {
    let mut retval: (bool, f32) = (go_up, offset);

    if go_up {
        if offset >= max_x { retval.0 = false;}
        retval.1 = offset + speed * timer::get_delta(ctx).subsec_nanos() as f32/1e8;
    } else {
        if offset <= min_x { retval.0 = true;}
        retval.1 = offset - speed * timer::get_delta(ctx).subsec_nanos() as f32/1e8;
    }

    retval

}

//Makes the parameters for drawing in the top left corner of the screen
pub fn make_def_param() -> DrawParam {
    //Adapted from Line 305 - 311 of https://github.com/maccam912/thewizzerofoz/blob/master/src/main.rs
    DrawParam {
        dest: Point2::new(0.0,0.0),
        scale: Point2::new(1.0,1.0),
        rotation: 0.0,
        offset: Point2::new(0.0,0.0),
        ..Default::default()
    }
}

//Makes any given parameters
pub fn make_param(dest: (f32,f32), scale: (f32,f32), rot: f32, offset: (f32,f32),) -> DrawParam {
    //Adapted from Line 305 - 311 of https://github.com/maccam912/thewizzerofoz/blob/master/src/main.rs
    DrawParam {
        dest: Point2::new(dest.0,dest.1),
        scale: Point2::new(scale.0,scale.1),
        rotation: rot,
        offset: Point2::new(offset.0,offset.1),
        ..Default::default()
    }
}

//This is here to do stuff to the context based on flags set. Add whatever flags and info you need
//to the parameters to do its work. Probably not a good idea to pass in too much as it will get messy
pub fn check_flags(ctx: &mut Context, quit: &bool) {
    if *quit {
        safe_quit(ctx);
    }
}

//This just makes the code look a little prettier and lets the reader know that errors are checked
pub fn safe_quit(ctx: &mut Context) {
    match ctx.quit() {
        Err(_) => panic!("Couldn't exit normally"),
        _       => (),
    };
}

//Defines an ordering for the game to execute the game scene functions
//This should never change so it is constant
pub fn make_scenes() -> Vec<SceneType> {
    //Creates scene ordering for MainState to use to determine what to do
    let mut scenes: Vec<SceneType> = Vec::new();

    scenes.push(SceneType::Intro);
    scenes.push(SceneType::Game);
    //scenes.push(SceneType::Exit);

    scenes
}

//Mounts system resources so they can be found
pub fn mount_resources( window_s: WindowSetup, window_m: WindowMode) -> ContextBuilder{
    //This method is pretty damn messy. How could this be improved?
    let mut ctx_b = ContextBuilder::new("mehens_portable_casino", "ggez").
        window_setup(window_s.clone()).
        window_mode(window_m.clone());

    //We add the CARGO_MANIFEST_DIR/resources to the filesystems paths so
    //we look in the cargo project for files, but only when on the dev build
    //Note that x.expect panics with the message when x contains an Err(...), and does nothing when it contains an Ok(...)
    match var("CARGO_MANIFEST_DIR") {
        Ok(cargo_path) =>  {
            let mut dev_path = PathBuf::from(cargo_path);
            dev_path.push("assets/");
            //We have to rebuild if this is the case
            ctx_b = ContextBuilder::new("mehens_portable_casino", "ggez").
                window_setup(window_s).
                window_mode(window_m).
                add_resource_path(dev_path);
        }
        _            => (), //We know this is a distributed executable running right here
    }

    ctx_b
}