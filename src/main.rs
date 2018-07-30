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
extern crate ggez;

//We need to import these package directories and then 'use' what we need out of them
mod gambling;
mod game_logic;

//import the needed namespaces
//use gambling::dicecoins::Dicecoins;
#[allow(unused_imports)]
use game_logic::main_state::*;
#[allow(unused_imports)]
use game_logic::scene_type::*;

//Ggez
use ggez::conf::{WindowSetup, WindowMode, NumSamples, FullscreenType};
#[allow(unused_imports)]
use ggez::event;
#[allow(unused_imports)]
use ggez::{ContextBuilder};
//use ggez::error::{GameResult, GameError};
//use ggez::graphics;
//use ggez::event::*;

//Std
use std::env;
use std::path;
//use std;
//use std::time;

fn main() {
    //We want to set the Window settings here. The user could be given
    //the option to set these
    let w_setup = WindowSetup {
        title: "Mehen's Portable Casino".to_string(),
        icon: "/placeholder_icon.png".to_string(),
        resizable: false,
        allow_highdpi: true,
        samples: NumSamples::One, //anti-aliasing
    };

    //Basic window settings, can be changed ingame
    let w_mode = WindowMode {
        width: 800,
        height: 600,
        borderless: false,
        fullscreen_type: FullscreenType::Off,
        vsync: true,
        min_width: 0, //Don't care about this unless we have resizeable
        max_width: 0,
        min_height: 0,
        max_height: 0,
    };

    let mut ctx_build = ContextBuilder::new("mehens_portable_casino", "ggez").
        window_setup(w_setup.clone()).
        window_mode(w_mode.clone());

    //We add the CARGO_MANIFEST_DIR/resources to the filesystems paths so
    //we look in the cargo project for files, but only when on the dev build
    //Note that x.expect panics with the message when x contains an Err(...), and does nothing when it contains an Ok(...)
    match env::var("CARGO_MANIFEST_DIR") {
        Ok(cargo_path) =>  {
            let mut dev_path = path::PathBuf::from(cargo_path);
            dev_path.push("assets/");
            //We have to rebuild if this is the case
            ctx_build = ContextBuilder::new("mehens_portable_casino", "ggez").
                window_setup(w_setup).
                window_mode(w_mode).
                add_resource_path(dev_path);
        }
        _            => (), //We know this is a distributed executable running right here
    }


    /*
    This returns a Result<Context, GameError>, and the match checks to see which it is. if there is
    an error then we want to cry at the user, because it is definitely their fault *sarcasm*.
    */
    let ctx;
    match ctx_build.build() {
        Ok(context)  => ctx = context,
        Err(e)       => panic!("Failed to build game context with err: {:?}.", e),
    }

    //Test buffer to see if loop is working correctly
    let mut test_buf: Vec<SceneType> = Vec::new();
    test_buf.push(SceneType::Cutscene);
    test_buf.push(SceneType::Game);
    test_buf.push(SceneType::Menu);
    test_buf.push(SceneType::Credits);
    //test_buf.push(SceneType::Exit);

    //Runs the game and returns a tuple that shows the exiting conditions
    let game = &mut MainState::new(ctx, test_buf).play();


    //Note our game returns a bool reference and thus we must use the * to get a correct comparison
    let retval; //We want this to return 0 on success
    match game {
        Ok(flag) =>{ if *flag {retval = 0} else {retval = -1;}},
        _e => eprintln!("{:?}",_e), //QUESTION: Will this print whatever value GameError's enum value is?
    }

}
