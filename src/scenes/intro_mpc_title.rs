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
use game_logic::scene_type::SceneType;
use game_logic::utility_functions::{safe_quit, make_param};

//Ggez
//use ggez::graphics;
use ggez::graphics::{FilterMode,Image, Point2, /*DrawParam, Drawable,*/ draw, set_default_filter};
use ggez::graphics::spritebatch::{SpriteBatch};

use ggez::event;
use ggez::event::{MouseButton, Keycode};

//use ggez::audio;
use ggez::{Context, GameResult};

/*
Here I define all the assets I will need to run a particular scene. This creates everything I need
*/
#[allow(unused)]
pub struct IntroMPC {
    s_type: SceneType,
    bg_mpc: Image,
    background_mpc: SpriteBatch,
}

impl IntroMPC {
    //We should not worry about framerate limiting here since MainState handles calls
    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        Ok(())
    }

    //We won't worry about clearing or presenting either since MainState handles that too
    //Also you MUST add params to your Spritebatch every draw call for it to render.
    pub fn draw(&mut self, ctx: &mut Context, _screen_center: &(f32,f32)) -> GameResult<()> {
        self.background_mpc.add(make_param());
        draw(ctx,&self.background_mpc, Point2::new(0.0, 0.0), 0.0)?;
        Ok(())
    }

    pub fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32){}

    pub fn key_down_event(&mut self, ctx: &mut Context, keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) {
        if keycode == Keycode::Return {
            safe_quit(ctx);
        }
    }

    pub fn key_up_event(&mut self, _ctx: &mut Context, _keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) {}
}

impl IntroMPC {
    pub fn new(ctx: &mut Context) -> GameResult<IntroMPC> {
        set_default_filter(ctx, FilterMode::Nearest);

        let bg = Image::new(ctx, "/MPC1.png")?;
        let bg_spr = SpriteBatch::new(bg.clone());


        let x = IntroMPC {
            s_type: SceneType::Intro,
            bg_mpc: bg,
            background_mpc: bg_spr,
            //background_mpc_param: bg_param,
        };
        Ok(x)
    }
}