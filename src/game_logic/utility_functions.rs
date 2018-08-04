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

//My imports
use SceneType;
#[allow(unused_imports)]
use game_logic::main_state;

//Ggez
use ggez::conf::{WindowSetup, WindowMode};
use ggez::{ContextBuilder, Context};
use ggez::graphics::{/*Image,*/ Point2, DrawParam};
use ggez::timer;

//Std
use std::env::var;
use std::path::PathBuf;

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
//to the parameters to do its work
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
/*
    scenes.push(SceneType::Intro);
    scenes.push(SceneType::Cutscene);
    scenes.push(SceneType::Game);
    scenes.push(SceneType::Menu);
    scenes.push(SceneType::Pause);
    scenes.push(SceneType::Credits);
    scenes.push(SceneType::Exit);
*/
    scenes.push(SceneType::Intro);
    scenes.push(SceneType::Exit);

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