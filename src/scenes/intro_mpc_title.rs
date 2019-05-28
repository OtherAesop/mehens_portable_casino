// mehens_portable_casino. A gambling game made using ggez and Dicecoin
// Copyright (C) 2018  MushuYoWushu
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

use ggez::{Context, GameResult};

use ggez::audio::Source;

/*
Here I define all the assets I will need to run a particular scene. This creates everything I need
*/

pub struct IntroMPC {
    //Background image
    background_mpc: SpriteBatch,
    //Sounds
    bad_boop: Source,
    good_boop: Source,
    //Enter button variables
    enter: SpriteBatch,
    enter_offset: (f32,f32),
    go_up_enter: bool,
    //Title text variables
    mpc_title: SpriteBatch,
    //mpc_font: Font,
    mpc_offset: (f32,f32),
    go_up_mpc: bool,
}

impl IntroMPC {
    //We should not worry about framerate limiting here since MainState handles calls
    pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        //Here we control the animation for the enter button
        let new_pos = float_animation(0.2, -0.2, 0.03, self.enter_offset.1, self.go_up_enter, ctx);
        self.enter_offset = (self.enter_offset.0, new_pos.1);
        self.go_up_enter = new_pos.0;

        //Here we control the title text/picture animation
        let new_pos_title = float_animation(0.1, -0.1, 0.02, self.mpc_offset.1, self.go_up_mpc, ctx);
        self.mpc_offset = (self.mpc_offset.0, new_pos_title.1);
        self.go_up_mpc = new_pos_title.0;

        Ok(())
    }

    //We won't worry about clearing or presenting either since MainState handles that too
    //Also you MUST add params to your Spritebatch every draw call for it to render.
    pub fn draw(&mut self, ctx: &mut Context, _screen_center: &(f32,f32)) -> GameResult<()> {
        //Draws MPC on screen
        self.background_mpc.add(make_def_param());
        draw(ctx,&self.background_mpc, Point2::new(0.0, 0.0), 0.0)?;
        self.background_mpc.clear();

        //Draws Enter button
        self.enter.add(make_param((655.0,440.0), (0.55,0.5), 0.0, (0.0, self.enter_offset.1)));
        draw(ctx,&self.enter, Point2::new(0.0, 0.0), 0.0)?;
        self.enter.clear();

        //Draws 'Mehen's Portable Casino' on screen
        self.mpc_title.add(make_param((32.0,140.0), (1.0,1.0), 0.0, (0.0, self.mpc_offset.1)));
        draw(ctx,&self.mpc_title, Point2::new(0.0, 0.0), 0.0)?;
        self.mpc_title.clear();
        Ok(())
    }

    pub fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32){}

    pub fn key_down_event(&mut self, _ctx: &mut Context, keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) -> SceneReturn{
        match keycode {
            Keycode::Return => {
                safe_play(&self.good_boop);
                SceneReturn::Finished
            },
            _               => {
                safe_play(&self.bad_boop);
                SceneReturn::Good
            },
        }
    }

    pub fn key_up_event(&mut self, _ctx: &mut Context, _keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) {}
}

impl IntroMPC {
    pub fn new(ctx: &mut Context) -> GameResult<IntroMPC> {
        //Note we should set defaults in each module so we can guarantee proper behavior
        set_default_filter(ctx, FilterMode::Nearest);

        let bg = Image::new(ctx, "/MPC1.png")?;
        let bg_spr = SpriteBatch::new(bg);

        let enter = Image::new(ctx, "/Enter.png")?;
        let enter_spr = SpriteBatch::new(enter);

        let title = Image::new(ctx, "/Mehens_Portable_Casino.png")?;
        let title_spr = SpriteBatch::new(title);

        let b_boop = Source::new(ctx, "/beep4.ogg")?;
        let g_boop = Source::new(ctx, "/Bleep Sound.wav")?;

        let x = IntroMPC {
            background_mpc: bg_spr,
            bad_boop: b_boop,
            good_boop: g_boop,
            //Enter button variables
            enter: enter_spr,
            enter_offset: (0.0,0.0),
            go_up_enter: true,
            //Title text variables
            mpc_title: title_spr,
            mpc_offset: (0.0,0.0),
            go_up_mpc: false,
        };
        Ok(x)
    }
}