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
use SceneType;
#[allow(unused_imports)]
use game_logic::main_state;
/*
pub fn load_next(scene_curr: SceneType, scene_buf: Vec<SceneType>) -> bool {

    let mut scene_retval = Ok(false); //This will never get read
    let mut exit_flag = false;

    //Plays the next scene or detects the Exit scene and leaves
    while !exit_flag {
        //Note: I used if/else here because two match statements are a little ugly
        if scene_buf.is_empty() { //Here we know that there is only one scene so we replay it
            scene_retval = scene_curr.play();
        } else { //There is another scene ready to be played, unwrap will not fail since there will always be something to pick
            match scene_buf.first().unwrap() {
                SceneType::Cutscene => (), //I don't need anything special to be done for these
                SceneType::Game     => (),
                SceneType::Menu     => (),
                SceneType::Pause    => (),
                SceneType::Credits  => (),
                SceneType::Exit     => exit_flag = true, //If we encounter an exit then we know it is time
                //_                   => panic!("Undefined handling for scene type encountered when loading from {:?}.", self.scene_curr),
            }
        }

        if !exit_flag { //DEBUG: Can I avoid copying here?
            //println!("This scene is {:?}.", self.scene_curr);
            self.scene_buf.push(self.scene_curr); //put the current at the end of the array
            self.scene_curr = self.scene_buf.remove(0); // take the item at the front
            //println!("Next scene is {:?}.", self.scene_curr);
            //println!("-------------------------------------");
            scene_retval = self.scene_curr.play();

            match scene_retval {
                Ok(flag) => if flag { /*all good do nothing*/} else { exit_flag = true; }
                _        => exit_flag = true,
            }
        }
    }

    //This should never fail. I think this destroys the actual game context rather than the event
    //being run.
    self.ctx.quit()?;
    scene_retval
}
*/

pub fn make_scenes() -> Vec<SceneType> {
    //Test buffer to see if loop is working correctly
    let mut scenes: Vec<SceneType> = Vec::new();
    scenes.push(SceneType::Cutscene);
    scenes.push(SceneType::Game);
    scenes.push(SceneType::Menu);
    scenes.push(SceneType::Pause);
    scenes.push(SceneType::Credits);
    scenes.push(SceneType::Exit);

    scenes
}