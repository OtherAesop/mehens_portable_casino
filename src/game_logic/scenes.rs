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
These are the details of the types of scenes we can implement. These are designed to be inside the enum
and used to provide frameworks for different kinds of functionality.
*/

use ggez::GameResult;

pub struct Cutscene {



}

pub struct Game {



}

pub struct Menu {



}

pub struct Pause {



}

pub struct Credits {



}

pub struct Exit {
    //This is designed to trigger an immediate quit event and thus needs no data
}

trait Playable {
    fn play() -> GameResult<bool>;
}