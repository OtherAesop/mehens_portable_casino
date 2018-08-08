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
use game_logic::scene_return_values::SceneReturn;
use game_logic::utility_functions::*;

//Ggez
use ggez::graphics::{FilterMode,Image, Point2, draw, set_default_filter};
use ggez::graphics::spritebatch::{SpriteBatch};

use ggez::event;
use ggez::event::{MouseButton, Keycode};

use ggez::audio::Source;

use ggez::{Context, GameResult};

/*
Here I define all the assets I will need to run a particular scene.
*/

pub struct DicecoinMPC {
    //Background image
    background_dc_mpc: SpriteBatch,
    //Sounds
    bad_boop: Source,
    good_boop: Source,
    //Enter buttons
    enter: SpriteBatch,
    enter_flipped: SpriteBatch,

}

#[allow(unused_variables)]
#[allow(dead_code)]
impl DicecoinMPC {
    //We should not worry about framerate limiting here since MainState handles calls
    pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        Ok(())
    }

    //We won't worry about clearing or presenting either since MainState handles that too
    //Also you MUST add params to your Spritebatch every draw call for it to render.
    pub fn draw(&mut self, ctx: &mut Context, _screen_center: &(f32,f32)) -> GameResult<()> {
        //Draws MPC on screen
        self.background_dc_mpc.add(make_def_param());
        draw(ctx,&self.background_dc_mpc, Point2::new(0.0, 0.0), 0.0)?;
        self.background_dc_mpc.clear();

        //Draws Enter button on screen

        //Draws EnterReverse button on screen

        Ok(())
    }

    pub fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32){}

    pub fn key_down_event(&mut self, _ctx: &mut Context, keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) -> SceneReturn{
        match keycode {
            Keycode::Return => SceneReturn::Finished,
            _               => SceneReturn::Good,
        }
    }

    pub fn key_up_event(&mut self, _ctx: &mut Context, _keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) {}
}

impl DicecoinMPC {
    pub fn new(ctx: &mut Context) -> GameResult<DicecoinMPC> {
        //Note we should set defaults in each module so we can guarantee proper behavior
        set_default_filter(ctx, FilterMode::Nearest);

        //Background allocation
        let bg = Image::new(ctx, "/Dicecoin GameBoard.png")?;
        let bg_spr = SpriteBatch::new(bg);

        //Enter button allocations
        let enter = Image::new(ctx, "/Enter.png")?;
        let enter_spr = SpriteBatch::new(bg);
        let enter_flipped = Image::new(ctx, "/EnterReverse.png")?;
        let enter_flipped_spr = SpriteBatch::new(bg);

        //Sound allocations
        let b_boop = Source::new(ctx, "/beep4.ogg")?;
        let g_boop = Source::new(ctx, "/Bleep Sound.wav")?;

        let x = DicecoinMPC {
            //Background
            background_dc_mpc: bg_spr,
            //Sounds
            bad_boop: b_boop,
            good_boop: g_boop,
            //Enter buttons
            enter: SpriteBatch,
            enter_flipped: SpriteBatch,
        };
        Ok(x)
    }
}
