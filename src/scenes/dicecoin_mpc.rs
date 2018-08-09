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
use game_logic::turns::Turn;
use game_logic::phase::Phase;
use game_logic::player::Player;
//This holds the stuff that can be used to draw on the screen
use game_logic::player_assets::PlayerAssets;
use game_logic::utility_functions::*;
//Easy to handle things by enums
use gambling::dice_type::DiceType;

//Ggez
use ggez::graphics::{FilterMode,Image, Point2, Font, Color, draw, set_default_filter};
use ggez::graphics::spritebatch::{SpriteBatch};

use ggez::event;
use ggez::event::{MouseButton, Keycode};

use ggez::audio::Source;

use ggez::{Context, GameResult};

/*
Here I define all the assets I will need to run a particular scene.
*/

#[allow(unused)]
pub struct DicecoinMPC {
    //Background image
    background_dc_mpc: SpriteBatch,
    //Text
    cyan: Color,
    font: Font,
    //Sounds
    bad_boop: Source,
    good_boop: Source,
    //Enter buttons and offset variables
    enter: SpriteBatch,
    enter_offset: (f32,f32),
    go_up_enter: bool,
    enter_flip: SpriteBatch,
    enter_flip_offset: (f32,f32),
    go_up_enter_flip: bool,
    //Player variables
    p1: Player,
    p2: Player,
    player_assets: PlayerAssets,
    //Endgame variables
    quit_flag: bool, //ONLY assign to true when you want the game to end
    game_winner: Turn, //Just going to print the winner, could possibly use for unique endings
    //Environment variables
    turnphase: (Turn, Phase), // A way to measure transitions (P1 or P2 , Betting, Raising, or Rolling in that order)
    p1_end_ready: bool, //Signifies the player is allowed to end their turn
    p2_end_ready: bool,
    winner: Turn,
    //P1
    betting_phase_flag_p1: bool,
    raising_phase_flag_p1: bool,
    rolling_phase_flag_p1: bool,
    //P2
    betting_phase_flag_p2: bool,
    raising_phase_flag_p2: bool,
    rolling_phase_flag_p2: bool,
    highest_roller: Turn, //Turn works because the highest_roller must be a player. This decides who picks Coos or Pearls
}

#[allow(unused_variables)]
#[allow(dead_code)]
impl DicecoinMPC {
    //We should not worry about framerate limiting here since MainState handles calls
    pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        //Here we control the animation for the enter buttons, they will only animate when the player is allowed to end
        if self.p2_end_ready {
            let new_pos = float_animation(0.15, -0.15, 0.03, self.enter_offset.1, self.go_up_enter, ctx);
            self.enter_offset = (self.enter_offset.0, new_pos.1);
            self.go_up_enter = new_pos.0;
         } else { self.enter_offset.1 = 0.0; } //Keep button at starting pos if not ready to end

        if self.p1_end_ready {
            let new_pos_flip = float_animation(0.15, -0.15, 0.03, self.enter_flip_offset.1, self.go_up_enter_flip, ctx);
            self.enter_flip_offset = (self.enter_flip_offset.0, new_pos_flip.1);
            self.go_up_enter_flip = new_pos_flip.0;
        } else { self.enter_flip_offset.1 = 0.0; } //Keep button at starting pos if not ready to end

        //Update the dice count on each player here
        self.player_assets.update_var(&self.p1, &self.p2);

        //Updates first two advance conditions, the third is the result of a decision and thus must
        //be checked in event handling
        match self.turnphase.0 {
            Turn::Player1 => {
                //P1 env var updates
                let p1_state = check_advance_conditions(&self.p1); //Returns (betting_flag, raising_flag)
                self.betting_phase_flag_p1 = p1_state.0;
                self.raising_phase_flag_p1 = p1_state.1;

            }
            Turn::Player2 => {
                //P2 env var updates
                let p2_state = check_advance_conditions(&self.p2); //Returns (betting_flag, raising_flag)
                self.betting_phase_flag_p2 = p2_state.0;
                self.raising_phase_flag_p2 = p2_state.1;
            }
        }

        //Updates the advance condition by phase. Splitting the updates by both phase and turn
        //ensures correctness and continuity every frame
        match self.turnphase.1 {
            Phase::Betting => {
                self.p1_end_ready = self.betting_phase_flag_p1;
                self.p2_end_ready = self.betting_phase_flag_p2;
            }
            Phase::Raising => {
                self.p1_end_ready = self.raising_phase_flag_p1;
                self.p2_end_ready = self.raising_phase_flag_p2;
            }
            Phase::Rolling => {
                self.p1_end_ready = self.rolling_phase_flag_p1;
                self.p2_end_ready = self.rolling_phase_flag_p2;
            }
            //_             => panic!("Unhandled turnphase update in dicecoin_mpc's update");
        }

        //DEBUG block
        println!("Turn: {:?}, Phase {:?}", self.turnphase.0.clone(), self.turnphase.1.clone());

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
        self.enter.add(make_param((649.0,414.0), (1.0,1.0), 0.0, (0.0, self.enter_offset.1)));
        draw(ctx,&self.enter, Point2::new(0.0, 0.0), 0.0)?;
        self.enter.clear();

        //Draws EnterReverse button on screen
        self.enter_flip.add(make_param((36.0,34.0), (1.0,1.0), 0.0, (0.0, self.enter_flip_offset.1)));
        draw(ctx,&self.enter_flip, Point2::new(0.0, 0.0), 0.0)?;
        self.enter_flip.clear();

        //Prettier to have all static draws handled elsewhere
        self.player_assets.draw_var(ctx, &self.p1, &self.p2)?;

        Ok(())
    }

    pub fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32){}

    pub fn key_down_event(&mut self, ctx: &mut Context, keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) -> SceneReturn{
        let msg = match self.turnphase.0 {
            //Check far below for player controller functions
            Turn::Player1   => self.p1_controller(ctx, keycode),
            Turn::Player2   => self.p2_controller(ctx, keycode),
            //_               => SceneReturn::Err("Unhandled player encountered in dicecoin_mpc's key_down_event".to_string()),
        };

        let retval = match msg {
            SceneReturn::Good => { //A player made a move and has not ended their turn, they can go again
                SceneReturn::Good
            }
            SceneReturn::Finished => { //A player has ended their turn, transition to the next turnphase (pairing of turn and phase)
                self.turnphase = transition_turnphase(&self.turnphase.0, &self.turnphase.1);
                //We can guarantee that after a transition there must be a player action so all progression conditions are implied not met.
                //P1
                self.betting_phase_flag_p1 = false;
                self.raising_phase_flag_p1 = false;
                self.rolling_phase_flag_p1 = false;
                //P2
                self.betting_phase_flag_p2 = false;
                self.raising_phase_flag_p2 = false;
                self.rolling_phase_flag_p2 = false;
                //Animation controller
                self.p1_end_ready = false;
                self.p2_end_ready = false;
                //We need to know which player gets to pick Coos or Pearls
                if self.turnphase.0 == Turn::Player1 && self.turnphase.1 == Phase::Rolling{ //true when turnphase == (Player1, Rolling), I.E. start of rolling phase
                    self.highest_roller = high_roller(&self.p1, &self.p2); //compares total dice bet and randomly decides roller if even bet
                }
                //We want to check for victory conditions when we get back to the first scene
                if self.turnphase.0 == Turn::Player1 && self.turnphase.1 == Phase::Betting{ // This will execute at the top of every round
                    if *self.p1.check_dice_total() == 0 { //Player 1 loses
                        self.game_winner = Turn::Player2;
                        self.quit_flag = true;
                    } else if *self.p2.check_dice_total() == 0 { //Player 2 loses
                        self.game_winner = Turn::Player1;
                        self.quit_flag = true;
                    }
                }
                SceneReturn::Good //This signifies that the next player may go
            }
            SceneReturn::Err(s) => { //Something wrong, scream bloody murder and terminate
                SceneReturn::Err(s)
            }
        };

        //User beware: returning SceneReturn::Finished here indicates that for whatever reason you
        //want to end the game and go to the next scene
        retval
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
        let enter = Image::new(ctx, "/EnterAdjusted.png")?;
        let enter_spr = SpriteBatch::new(enter);
        let enter_flipped = Image::new(ctx, "/EnterReverse.png")?;
        let enter_flipped_spr = SpriteBatch::new(enter_flipped);

        //Font alloc
        let font1 = Font::new_glyph_font(ctx, "/PressStart2P-Regular.ttf")?;

        //Sound allocations
        let b_boop = Source::new(ctx, "/beep4.ogg")?;
        let g_boop = Source::new(ctx, "/Bleep Sound.wav")?;

        let x = DicecoinMPC {
            //Background
            background_dc_mpc: bg_spr,
            //Text
            cyan: Color::new(95.0, 205.0, 228.0, 255.0), //This is the cyan in the concept doc,
            font: font1,
            //Sounds
            bad_boop: b_boop,
            good_boop: g_boop,
            //Enter buttons and environment variables
            enter: enter_spr,
            enter_offset: (0.0,0.0),
            go_up_enter: true,
            enter_flip: enter_flipped_spr,
            enter_flip_offset: (0.0,0.0),
            go_up_enter_flip: false, //It looks better to have them travelling in opposite directions methinks
            //Player variables
            p1: Player::new(),
            p2: Player::new(),
            player_assets: PlayerAssets::new(ctx)?,
            //Endgame variables
            quit_flag: false, //ONLY assign to true when you want the game to end
            game_winner: Turn::Player1, //Just going to print the winner, could possibly use for unique endings
            //Environment variables
            turnphase: (Turn::Player1, Phase::Betting), // A way to measure transitions (P1 or P2 , Betting, Raising, or Rolling in that order)
            p1_end_ready: false, //Signifies the player is allowed to end their turn
            p2_end_ready: false,
            winner: Turn::Player1, //Should never actually be checked before it is overwritten
            //P1
            betting_phase_flag_p1: false,
            raising_phase_flag_p1: false,
            rolling_phase_flag_p1: false,
            //P2
            betting_phase_flag_p2: false,
            raising_phase_flag_p2: false,
            rolling_phase_flag_p2: false,
            highest_roller: Turn::Player1, //Should never actually be checked before it is overwritten
        };
        Ok(x)
    }
}

//Player controllers are put down here to make this more readable

impl DicecoinMPC {
    //The nature of the game makes separate controllers neater
    fn p1_controller (&mut self, _ctx: &mut Context, keycode: event::Keycode) -> SceneReturn {

        let retval = match self.turnphase.1 {
            Phase::Betting => {
                match keycode {
                    Keycode::Q      => { if self.p1.bet_rolling_dice(DiceType::D2)   {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::W      => { if self.p1.bet_rolling_dice(DiceType::D4)   {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::E      => { if self.p1.bet_rolling_dice(DiceType::D6)   {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::R      => { if self.p1.bet_rolling_dice(DiceType::D8)   {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::A      => { if self.p1.bet_rolling_dice(DiceType::D10)  {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::S      => { if self.p1.bet_rolling_dice(DiceType::D10p) {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::D      => { if self.p1.bet_rolling_dice(DiceType::D12)  {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::F      => { if self.p1.bet_rolling_dice(DiceType::D20)  {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::Return => {
                        if self.p1_end_ready { //Player is allowed to end their turn
                            safe_play(&self.good_boop);
                            self.p1_end_ready = false;
                            SceneReturn::Finished
                        } else { //Player may not end their turn
                            safe_play(&self.bad_boop);
                            SceneReturn::Good
                        }
                    }
                    _               => { safe_play(&self.bad_boop);  SceneReturn::Good}
                }
            }
            Phase::Raising => {
                match keycode {
                    Keycode::Q      => { if self.p1.bet_dice(DiceType::D2)   {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::W      => { if self.p1.bet_dice(DiceType::D4)   {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::E      => { if self.p1.bet_dice(DiceType::D6)   {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::R      => { if self.p1.bet_dice(DiceType::D8)   {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::A      => { if self.p1.bet_dice(DiceType::D10)  {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::S      => { if self.p1.bet_dice(DiceType::D10p) {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::D      => { if self.p1.bet_dice(DiceType::D12)  {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::F      => { if self.p1.bet_dice(DiceType::D20)  {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::Return => {
                        if self.p1_end_ready { //Player is allowed to end their turn
                            safe_play(&self.good_boop);
                            self.p1_end_ready = false;
                            SceneReturn::Finished
                        } else { //Player may not end their turn
                            safe_play(&self.bad_boop);
                            SceneReturn::Good
                        }
                    }
                    _               => { safe_play(&self.bad_boop);  SceneReturn::Good}
                }
            }
            Phase::Rolling => {
                //This player gets their turn skipped
                //because they were not the highest roller
                if self.highest_roller != Turn::Player1 {
                    SceneReturn::Finished
                } else { //This player is the highest roller and gets to go
                    match keycode {
                        Keycode::C => {
                            if !self.rolling_phase_flag_p1 { //Have not yet rolled dice
                                safe_play(&self.good_boop);
                                //Evaluate game here
                                self.p1.roll_dice();
                                self.p2.roll_dice();
                                //We know the winner so it is ok to assign winnings after player ends turn
                                self.winner = evaluate_coos(&self.p1, &self.p2);
                                //A decision was made so the game can advance
                                self.rolling_phase_flag_p1 = true;
                                SceneReturn::Good
                            } else {
                                safe_play(&self.bad_boop);
                                SceneReturn::Good
                            }
                        }
                        Keycode::P => {
                            if !self.rolling_phase_flag_p1 { //Have not yet rolled dice
                                safe_play(&self.good_boop);
                                //Evaluate game here
                                self.p1.roll_dice();
                                self.p2.roll_dice();
                                //We know the winner so it is ok to assign winnings after player ends turn
                                self.winner = evaluate_pearls(&self.p1, &self.p2);
                                //A decision was made so the game can advance
                                self.rolling_phase_flag_p1 = true;
                                SceneReturn::Good
                            } else {
                                safe_play(&self.bad_boop);
                                SceneReturn::Good
                            }
                        }
                        Keycode::Return => {
                            if self.p1_end_ready { //Player is allowed to end their turn
                                safe_play(&self.good_boop);
                                if !win(&mut self.p1, &mut self.p2, &self.winner) {
                                    println!("Overflow occurred, but guards prevented bad game flow. Check your design.");
                                }
                                self.p1.clear_roll_result();
                                self.p2.clear_roll_result();
                                self.p1_end_ready = false;
                                SceneReturn::Finished
                            } else { //Player may not end their turn
                                safe_play(&self.bad_boop);
                                SceneReturn::Good
                            }
                        }
                        _ => {
                            safe_play(&self.bad_boop);
                            SceneReturn::Good
                        }
                    }
                }
            }
        };

        retval
    }

    fn p2_controller (&mut self, _ctx: &mut Context, keycode: event::Keycode) -> SceneReturn {
        let retval = match self.turnphase.1 {
            Phase::Betting => {
                match keycode {
                    Keycode::Y      => { if self.p2.bet_rolling_dice(DiceType::D2)   {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::U      => { if self.p2.bet_rolling_dice(DiceType::D4)   {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::I      => { if self.p2.bet_rolling_dice(DiceType::D6)   {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::O      => { if self.p2.bet_rolling_dice(DiceType::D8)   {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::H      => { if self.p2.bet_rolling_dice(DiceType::D10)  {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::J      => { if self.p2.bet_rolling_dice(DiceType::D10p) {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::K      => { if self.p2.bet_rolling_dice(DiceType::D12)  {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::L      => { if self.p2.bet_rolling_dice(DiceType::D20)  {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::Return => {
                        if self.p2_end_ready { //Player is allowed to end their turn
                            safe_play(&self.good_boop);
                            self.p2_end_ready = false;
                            SceneReturn::Finished
                        } else { //Player may not end their turn
                            safe_play(&self.bad_boop);
                            SceneReturn::Good
                        }
                    }
                    _               => { safe_play(&self.bad_boop);  SceneReturn::Good}
                }
            }
            Phase::Raising => {
                match keycode {
                    Keycode::Y      => { if self.p2.bet_dice(DiceType::D2)   {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::U      => { if self.p2.bet_dice(DiceType::D4)   {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::I      => { if self.p2.bet_dice(DiceType::D6)   {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::O      => { if self.p2.bet_dice(DiceType::D8)   {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::H      => { if self.p2.bet_dice(DiceType::D10)  {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::J      => { if self.p2.bet_dice(DiceType::D10p) {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::K      => { if self.p2.bet_dice(DiceType::D12)  {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::L      => { if self.p2.bet_dice(DiceType::D20)  {safe_play(&self.good_boop);} else {safe_play(&self.bad_boop);} SceneReturn::Good }
                    Keycode::Return => {
                        if self.p2_end_ready { //Player is allowed to end their turn
                            safe_play(&self.good_boop);
                            self.p2_end_ready = false;
                            SceneReturn::Finished
                        } else { //Player may not end their turn
                            safe_play(&self.bad_boop);
                            SceneReturn::Good
                        }
                    }
                    _               => { safe_play(&self.bad_boop);  SceneReturn::Good}
                }
            }
            Phase::Rolling => {
                //This player gets their turn skipped
                //because they were not the highest roller
                if self.highest_roller != Turn::Player2 {
                    SceneReturn::Finished
                } else { //This player is the highest roller and gets to go
                    match keycode {
                        Keycode::C => {
                            safe_play(&self.good_boop);
                            //Evaluate game here
                            self.p1.roll_dice();
                            self.p2.roll_dice();
                            //We know the winner so it is ok to assign winnings after player ends turn
                            self.winner = evaluate_coos(&self.p1, &self.p2);
                            //A decision was made so the game can advance
                            self.rolling_phase_flag_p2 = true;
                            SceneReturn::Finished
                        }
                        Keycode::P => {
                            safe_play(&self.good_boop);
                            //Evaluate game here
                            self.p1.roll_dice();
                            self.p2.roll_dice();
                            //We know the winner so it is ok to assign winnings after player ends turn
                            self.winner = evaluate_pearls(&self.p1, &self.p2);
                            //A decision was made so the game can advance
                            self.rolling_phase_flag_p2 = true;
                            SceneReturn::Finished
                        }
                        Keycode::Return => {
                            if self.p2_end_ready { //Player is allowed to end their turn
                                safe_play(&self.good_boop);
                                if !win(&mut self.p1, &mut self.p2, &self.winner) {
                                    println!("Overflow occurred, but guards prevented bad game flow. Check your design.");
                                }
                                self.p1.clear_roll_result();
                                self.p2.clear_roll_result();
                                self.p2_end_ready = false;
                                SceneReturn::Finished
                            } else { //Player may not end their turn
                                safe_play(&self.bad_boop);
                                SceneReturn::Good
                            }
                        }
                        _ => {
                            safe_play(&self.bad_boop);
                            SceneReturn::Good
                        }
                    }
                }
            }
        };

        retval
    }

}
