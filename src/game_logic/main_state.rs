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

//Ggez imports
use ggez::Context;

//This is the core loop
pub struct MainState{
    ctx: Context,
    scene_curr: SceneType,
    scene_buf: Vec<SceneType>,
}

impl MainState{

    //This loads the first scene and stores the rest into a buffer variable
    pub fn new (context: Context, mut scenes: Vec<SceneType>) -> Self {

        if scenes.is_empty(){
            panic!("Empty scene buffer passed to MainState.");
        }

        //Note that remove will not fail here since we have guaranteed it will have at least one
        //scene element to take out. Note that scene_buf CAN be empty
        MainState{ctx: context, scene_curr: scenes.remove(0), scene_buf: scenes}

    }

    //Here we play scenes in a loop until one of them comes back crying for any reason and then we
    //decide it just isn't worth it anymore and terminate
    pub fn play(&mut self) -> bool {

        let mut success_flag = true;
        let mut exit_flag = false;

        while success_flag && !exit_flag {
            //Note: I used if/else here because two match statements are a little ugly
            if self.scene_buf.is_empty() { //Here we know that there is only one scene so we replay it
                //replay current scene
                success_flag = true;
            } else { //There is another scene ready to be played
                match self.scene_buf.remove(0) {
                    SceneType::Cutscene => success_flag = true, //I don't need anything special to be done for these
                    SceneType::Game     => success_flag = true,
                    SceneType::Menu     => success_flag = true,
                    SceneType::Pause    => success_flag = true,
                    SceneType::Credits  => success_flag = true,
                    SceneType::Exit     => exit_flag = true, //If we encounter an exit then we know it is time
                    //_                   => panic!("Undefined handling for scene type encountered when loading from {:?}.", self.scene_curr),
                }
            }

            if success_flag && !exit_flag {
                self.scene_buf.push(self.scene_curr); //put the current at the end of the array
                self.scene_curr = self.scene_buf.remove(0); // take the item at the front
                success_flag = false; //TODO: replace this with playing the Scene
            }
        }

        success_flag
    }

}




