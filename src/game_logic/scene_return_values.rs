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
These are the various return values that scenes can have.
1) Good means execution was a success and iteration should continue (i.e. repeat)
2) Finished means execution was a success and the next thing should be loaded/executed
3) Err means there was a fatal error and the String should contain a helpful hint
*/

//This allows us to format the SceneType with {:?} in println!(...)
#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum SceneReturn {
    Good,
    Finished,
    Err(String),
}