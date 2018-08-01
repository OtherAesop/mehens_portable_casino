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
use ggez::graphics::spritebatch::{SpriteBatch, /*SpriteIdx*/};
use ggez::graphics::{present, clear, Image, Point2};

use ggez::event;
use ggez::event::{EventHandler, Keycode, Mod};

use ggez::timer::check_update_time; //Gotta control FPS!
use ggez::error::GameResult;
use ggez::Context;

pub struct Credits <'a>{
    fps: u32,
    ctx: &'a mut Context,
    background: [SpriteBatch; 2],
    screen_center: Point2,
    quit: bool,
}

/*
The lifetime specifier here basically says the reference to the Context must stay valid throughout
the entire span of the execution of Credits
*/

impl<'a> Credits<'a>{
    pub fn new (&mut self, cont: &'a mut Context) -> GameResult<Self> {

        let img_path1 = "/placeholder_bob1.png";
        let img_path2 = "/placeholder_bob2.png";
        let img1 = Image::new(self.ctx, img_path1)?;
        let img2 = Image::new(self.ctx, img_path1)?;

        let retval = Credits{
            fps: 30,
            ctx: cont,
            background: [SpriteBatch::new(img1), SpriteBatch::new(img2)],
            screen_center: Point2::new((self.ctx.conf.window_mode.width as f32)/(2 as f32), (self.ctx.conf.window_mode.height as f32)/(2 as f32)),
            quit: false,
        };

        Ok(retval)
    }

    pub fn play (&mut self) -> GameResult<()> {
        event::run(self.ctx, self)
    }
}

impl<'a> event::EventHandler for Credits<'a> {

    fn update(&mut self, cont: &mut Context) -> GameResult<()> {
        while check_update_time(self.ctx, self.fps) {
            // runs x frames per second
            if self.quit {
                self.ctx.quit();
            }
        }

        Ok(())
    }

    fn draw(&mut self, cont: &mut Context) -> GameResult<()> {
        clear(cont); //Clear the screen to background color!



        present(cont); //Begin drawing!

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key: Keycode, _keymod: Mod, _repeat: bool){
        match key {
            Keycode::Return => self.quit = true, //All this needs to do is progress when the user is ready
            _               => ()
        }
        ()
    }




}
*/