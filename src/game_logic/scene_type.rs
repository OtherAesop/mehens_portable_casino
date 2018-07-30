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
These are the various types of scenes we can implement. The purpose of these is to direct the flow
of draw and update statements so it simulates having separate scenes in the game
*/

/*
Each scene here must match a SPECIFIC implementation defined elsewhere
a more modular approach looks to be impossible due to the way event::run() works,
attempt at your own risk
*/

//This allows us to format the SceneType with {:?} in println!(...)
#[derive(Debug, Copy, Clone)]
pub enum SceneType {
    Cutscene,
    Game,
    Menu,
    Pause,
    Credits,
    Exit,
}

