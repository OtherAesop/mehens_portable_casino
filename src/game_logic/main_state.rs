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

/*
TODO NOTE: self.scene_curr = self.scene_circle_iter.next().unwrap(); selects the next scene
*/

//My imports
use SceneType;
use game_logic::utility_functions::{check_flags};
use scenes::intro_mpc_title::IntroMPC;

//Ggez imports
use ggez::{Context};
use ggez::error::{GameResult};

use ggez::timer::check_update_time; //Gotta control FPS!
use ggez::audio::Source;

use ggez::event;
use ggez::event::{MouseButton};

use ggez::graphics::spritebatch::{SpriteBatch};
use ggez::graphics::{Image,clear,present};

//Std imports
use std::slice::Iter;
use std::iter::{Cycle, Iterator};

//This is the core loop
//Note that due to the way resources are loaded
//all image data MUST be accessible here
#[allow(unused)]
pub struct MainState<'a>{
    //Scene Data
    scene_curr: &'a SceneType,
    scene_circle_iter: &'a mut Cycle<Iter<'a,SceneType>>,
    //Environment Variables
    screen_center_xy: (f32,f32),
    fps_target: u32,
    quit_flag: bool,
    music_played: bool,
    //Game Song
    bg_music: Source,
    //Intro Screen
    mpc_intro: IntroMPC,
    test_bg: SpriteBatch,
}

impl<'a> event::EventHandler for MainState<'a>{
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while check_update_time(ctx, self.fps_target) {
            //Run at x frames per second
            //Here we sort into the correct call, you can add extra logic to sort through multiple scenes
            //of different types or just add in a new enum. The second option is probably easier
            let msg = match self.scene_curr {
                SceneType::Intro    => self.mpc_intro.update(ctx),
                SceneType::Cutscene => Ok(()),/*TODO add in call to the correct scene and fn*/
                SceneType::Game     => Ok(()),/*TODO add in call to the correct scene and fn*/
                SceneType::Menu     => Ok(()),/*TODO add in call to the correct scene and fn*/
                SceneType::Pause    => Ok(()),/*TODO add in call to the correct scene and fn*/
                SceneType::Credits  => Ok(()),/*TODO add in call to the correct scene and fn*/
                SceneType::Exit     => {self.quit_flag = true; Ok(())},
            };

            if let Err(_) = msg {
                panic!("Error in MainState update call: {:?}", msg);
            }

            if !self.music_played {
                self.bg_music.play()?;
                self.bg_music.set_repeat(true);
                self.music_played = true;
            }

            check_flags(ctx, &self.quit_flag);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx); //Clears everything
        //Here we sort into the correct call, you can add extra logic to sort through multiple scenes
        //of different types or just add in a new enum. The second option is probably easier
        let msg = match self.scene_curr {
            SceneType::Intro    => self.mpc_intro.draw(ctx, &self.screen_center_xy),
            SceneType::Cutscene => Ok(()),/*TODO add in call to the correct scene and fn*/
            SceneType::Game     => Ok(()),/*TODO add in call to the correct scene and fn*/
            SceneType::Menu     => Ok(()),/*TODO add in call to the correct scene and fn*/
            SceneType::Pause    => Ok(()),/*TODO add in call to the correct scene and fn*/
            SceneType::Credits  => Ok(()),/*TODO add in call to the correct scene and fn*/
            SceneType::Exit     => {self.quit_flag = true; Ok(())},
        };

        if let Err(_) = msg {
            panic!("Error in MainState draw call: {:?}", msg);
        }

        present(ctx); //Actually draws your draw calls
        Ok(())
    }
    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: i32, y: i32){
        match self.scene_curr {
            SceneType::Intro    => self.mpc_intro.mouse_button_down_event(ctx, button, x, y ),
            SceneType::Cutscene => (),/*TODO add in call to the correct scene and fn*/
            SceneType::Game     => (),/*TODO add in call to the correct scene and fn*/
            SceneType::Menu     => (),/*TODO add in call to the correct scene and fn*/
            SceneType::Pause    => (),/*TODO add in call to the correct scene and fn*/
            SceneType::Credits  => (),/*TODO add in call to the correct scene and fn*/
            SceneType::Exit     => {self.quit_flag = true; ()},
        }
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: event::Keycode, keymod: event::Mod, repeat: bool) {
        //Here we sort into the correct call, you can add extra logic to sort through multiple scenes
        //of different types or just add in a new enum. The second option is probably easier
        match self.scene_curr {
            SceneType::Intro    => self.mpc_intro.key_down_event(ctx, keycode, keymod, repeat),
            SceneType::Cutscene => (),/*TODO add in call to the correct scene and fn*/
            SceneType::Game     => (),/*TODO add in call to the correct scene and fn*/
            SceneType::Menu     => (),/*TODO add in call to the correct scene and fn*/
            SceneType::Pause    => (),/*TODO add in call to the correct scene and fn*/
            SceneType::Credits  => (),/*TODO add in call to the correct scene and fn*/
            SceneType::Exit     => {self.quit_flag = true; ()},
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: event::Keycode, keymod: event::Mod, repeat: bool) {
        //Here we sort into the correct call, you can add extra logic to sort through multiple scenes
        //of different types or just add in a new enum. The second option is probably easier
        match self.scene_curr {
            SceneType::Intro    => self.mpc_intro.key_up_event(ctx, keycode, keymod, repeat),
            SceneType::Cutscene => (),/*TODO add in call to the correct scene and fn*/
            SceneType::Game     => (),/*TODO add in call to the correct scene and fn*/
            SceneType::Menu     => (),/*TODO add in call to the correct scene and fn*/
            SceneType::Pause    => (),/*TODO add in call to the correct scene and fn*/
            SceneType::Credits  => (),/*TODO add in call to the correct scene and fn*/
            SceneType::Exit     => {self.quit_flag = true; ()},
        }
    }

}

impl<'a> MainState<'a>{

    //This loads the first scene and stores the rest into a buffer variable
    pub fn new (ctx: &mut Context, scene_buf: &'a mut Cycle<Iter<'a,SceneType>>) -> Self {

        let bg = Image::new(ctx, "/MPC1.png").expect("ahhhh");
        let bg_spr = SpriteBatch::new(bg.clone());

        //Scene allocations
        let mpc1 = IntroMPC::new(ctx).expect("Cannot load IntroMPC");

        //Load background music
        let bg_mus = Source::new(ctx, "/8bit Dungeon Level.ogg").expect("Cannot load background music.");

        //Checking first scene also moves iterator
        let first_scene = scene_buf.next();
        if first_scene == None {
            panic!("Passed empty scene buffer to MainState, check utility_functions.");
        }

        //Note that remove will not fail here since we have guaranteed it will have at least one
        //scene element to take out. Note that scene_buf CANNOT be empty since it is an iterator
         let retval = MainState {
            scene_curr: first_scene.unwrap(),
            scene_circle_iter: scene_buf,
            screen_center_xy: (ctx.conf.window_mode.width as f32 / 2.0, ctx.conf.window_mode.height as f32 / 2.0),
            fps_target: 60,
            quit_flag: false,
            music_played: false,
            bg_music: bg_mus,
            mpc_intro: mpc1,
            test_bg: bg_spr,
        };

         retval
    }

}