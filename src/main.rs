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
//We only need the topmost directory. We can 'use' out the rest anywhere else.
mod gambling;
mod game_logic;
mod scenes;

//import the needed namespaces
//DEBUG: these are included so the compiler goes over them.
#[allow(unused_imports)]
use game_logic::main_state::*;
#[allow(unused_imports)]
use game_logic::scene_type::*;
use game_logic::utility_functions::*;

//Ggez
use ggez::conf::{WindowSetup, WindowMode, NumSamples, FullscreenType};
#[allow(unused_imports)]
use ggez::event;
#[allow(unused_imports)]
use ggez::{ContextBuilder, Context};
//use ggez::error::{GameResult, GameError};
//use ggez::graphics;
//use ggez::event::*;

//Std
//use std::slice::Iter;
//use std::iter::{Cycle};
//use std::env;
//use std::path;
//use std;
//use std::time;



fn main() {
    //We need to store the scene Vec somewhere so we can make a circular iterator reference to it
    let scenes: Vec<SceneType> = make_scenes();
    let scene_cycle = &mut scenes.iter().cycle();

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
        width: 1024,
        height: 768,
        borderless: false,
        fullscreen_type: FullscreenType::Off,
        vsync: true,
        min_width: 0, //Don't care about this unless we have resizeable
        max_width: 0,
        min_height: 0,
        max_height: 0,
    };

    //Correctly mounts the resource folders
    let ctx_build = mount_resources(w_setup, w_mode);

    /*
    This returns a Result<Context, GameError>, and the match checks to see which it is. if there is
    an error then we want to cry at the user, because it is definitely their fault *sarcasm*.
    */
    let ctx_temp= &mut ctx_build.build();
    let ctx;
    match ctx_temp {
        Ok(context)  => ctx = context,
        Err(e)       => panic!("Failed to build game context with err: {:?}.", e),
    }

    //Creates then runs the game and returns a tuple that shows the exiting conditions.
    //make_scenes
    let game = &mut MainState::new(ctx, scene_cycle);
    let result = event::run(ctx, game);

    //Note our game returns a bool reference and thus we must use the * to get a correct comparison

    if let Err(_) = result {
        println!("Fatal error encountered: {:?}", result);
    } else{
        println!("Game exited successfully");
    }

}
