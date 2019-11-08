// mehens_portable_casino. A gambling game made using ggez and Dicecoin
// Copyright (C) 2018  Ian Gore
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
These are some colors that help make what is going on in the game a little more obvious without
cluttery text
*/

use ggez::error::{GameResult};
use ggez::graphics::{Color};

#[derive(Debug, Clone, Copy)]
pub struct ColorPalette{
    //Enter Key Colors
    pub p1_highlight: Color,
    pub p1_fade_highlight: Color,
    pub p2_highlight: Color,
    pub p2_fade_highlight: Color,
    //Roll result Colors
    pub p1_roll_result: Color,
    pub p2_roll_result: Color,
    //Dice Colors
    pub p1_dice: Color,
    pub p2_dice: Color,
}

impl ColorPalette {
    pub fn new () -> GameResult<Self> {
        let x = ColorPalette {
            //Enter Key Colors
            p1_highlight: Color::new(1.0, 0.0, 0.0, 1.0), //This is the highlight color to indicate something is p1
            p1_fade_highlight: Color::new(1.0, 0.0, 0.0, 0.5), //This is the highlight color to indicate something is p1
            p2_highlight: Color::new(0.0, 0.0, 1.0, 1.0), //This is the highlight color to indicate something is p2
            p2_fade_highlight: Color::new(0.0, 0.0, 1.0, 0.5), //This is the highlight color to indicate something is p2
            //Roll result Colors
            p1_roll_result: Color::new(0.5, 0.0, 0.0, 1.0), //This is the highlight color for P1's score text
            p2_roll_result: Color::new(0.0, 0.0, 1.0, 1.0), //This is the highlight color for P2's score text
            //Dice Colors
            p1_dice: Color::new(1.0, 0.5, 0.5, 1.0), //This is the highlight color for P1's dice
            p2_dice: Color::new(0.5, 0.5, 1.0, 1.0), //This is the highlight color for P2's dice
        };


        Ok(x)
    }
}