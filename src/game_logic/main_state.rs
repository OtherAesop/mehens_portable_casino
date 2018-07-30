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
The intention here is to define a MainState framework such that the game designer can add scenes
and progression conditions that allow them to insert game scenes as they please and as is necessary
to tell the story or whatever
*/

//My imports
use SceneType;
use game_logic::utility_functions::*;

//Ggez imports
use ggez::{Context};
use ggez::error::{GameResult/*,GameError*/};

use ggez::event;
use ggez::event::{MouseButton, Keycode};
//This is the core loop
#[allow(dead_code)]
pub struct MainState{
    scene_curr: SceneType,
    scene_buf: Vec<SceneType>,
}

impl event::EventHandler for MainState{
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {

        Ok(())
    }
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32){}

    fn key_down_event(&mut self, ctx: &mut Context, keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) {
        if keycode == Keycode::Return {
            match ctx.quit() {
                Err(_) => panic!("Couldn't exit normally"),
                _       => (),
            };
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) {}

}

impl MainState{

    //This loads the first scene and stores the rest into a buffer variable
    pub fn new (_cont: &mut Context) -> Self {
        let mut scenes: Vec<SceneType> = make_scenes();
        if scenes.is_empty(){
            panic!("Empty scene buffer.");
        }

        //Note that remove will not fail here since we have guaranteed it will have at least one
        //scene element to take out. Note that scene_buf CAN be empty
        MainState{ scene_curr: scenes.remove(0), scene_buf: scenes}

    }

}