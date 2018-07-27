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
use game_logic::main_state::*;
use game_logic::scene_type::*;

//Ggez
use ggez::conf::{WindowSetup, WindowMode, NumSamples, FullscreenType};
use ggez::event;
use ggez::{ContextBuilder, GameResult};
//use ggez::graphics;
//use ggez::event::*;

//Std
use std::env;
use std::path;
//use std::time;

fn main() {
    /*
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("helloworld", "ggez", c).unwrap();
    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
// We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
// we we look in the cargo project for files.
  if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
      let mut path = path::PathBuf::from(manifest_dir);
      path.push("assets");
      println!("{:?}", path);
  }
*/
    //We want to set the Window settings here. The user could be given
    //the option to set these
    let w_setup = WindowSetup {
        title: "ee".to_string(),
        icon: "assets/placeholder_icon.png".to_string(),
        resizable: false,
        allow_highdpi: true,
        samples: NumSamples::One,
    }; //anti-aliasing

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
    /*
        let c = conf::Conf::new();
        let ctx = &mut Context::load_from_conf("helloworld", "ggez", c).unwrap();
        graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
        */

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    // Note that x.expect panics with the message when x contains an Err(...), and does nothing when it contains an Ok(...)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR");
    let mut my_path = path::PathBuf::from(manifest_dir.expect("Cannot mount resource folder."));
    my_path.push("assets");


    let ctx_build = ContextBuilder::new("mehens_portable_casino", "Mushu").
                                window_setup(w_setup).
                                window_mode(w_mode).
                                add_resource_path(my_path);

    /*
    This returns a Result<Context, GameError>, and the match checks to see which it is. if there is
    an error then we want to cry at the user, because it is definitely their fault (sarcasm.)
    */
    let ctx;
    match ctx_build.build() {
        Ok(context)  => ctx = context,
        Err(E)       => panic!("Failed to build game context."),
    }


/*
  let state = &mut MainState::new(ctx).unwrap();
  if let Err(e) = event::run(ctx, state) {
      println!("Error encountered: {}", e);
  } else {
      println!("Game exited cleanly.");
  }
*/
}
