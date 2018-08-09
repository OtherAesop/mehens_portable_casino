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
use game_logic::player::Player;
use game_logic::utility_functions::{make_param};
//Easy to handle things by enums
use gambling::dice_type::DiceType;

//Ggez
use ggez::graphics::{Image, Font, TextCached, Color, Scale, Point2, WHITE, BLACK, draw, set_color};
use ggez::graphics::spritebatch::{SpriteBatch};


use ggez::{Context, GameResult};

const DICE_NUM_PRINT_SIZE: f32 = 25.0;
const SCORE_TEXT_PRINT_SIZE: f32 = 28.0;
const ROLLING_OFFSET: f32 = 60.0;
const ROLLING_OFFSET_NEG: f32 = -60.0;

#[allow(unused)]
pub struct PlayerAssets {
    //Dice Sprites
    d2: SpriteBatch,
    d4: SpriteBatch,
    d6: SpriteBatch,
    d8: SpriteBatch,
    d10: SpriteBatch,
    d10p: SpriteBatch,
    d12: SpriteBatch,
    d20: SpriteBatch,
    //Text
    cyan: Color,
    font: Font,
    //P1
    d2_text_p1: TextCached,
    d4_text_p1: TextCached,
    d6_text_p1: TextCached,
    d8_text_p1: TextCached,
    d10_text_p1: TextCached,
    d10p_text_p1: TextCached,
    d12_text_p1: TextCached,
    d20_text_p1: TextCached,
    roll_result_p1: TextCached,
    //P2
    d2_text_p2: TextCached,
    d4_text_p2: TextCached,
    d6_text_p2: TextCached,
    d8_text_p2: TextCached,
    d10_text_p2: TextCached,
    d10p_text_p2: TextCached,
    d12_text_p2: TextCached,
    d20_text_p2: TextCached,
    roll_result_p2: TextCached,

}

#[allow(unused)]
impl PlayerAssets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {

        //D2-D20 picture allocations
        let d2 = Image::new(ctx, "/D2blur.png")?;
        let d4 = Image::new(ctx, "/D4blur.png")?;
        let d6 = Image::new(ctx, "/D6blur.png")?;
        let d8 = Image::new(ctx, "/D8blur.png")?;
        let d10 = Image::new(ctx, "/D10blur.png")?;
        let d10p = Image::new(ctx, "/D10Pblur.png")?;
        let d12 = Image::new(ctx, "/D12blur.png")?;
        let d20 = Image::new(ctx, "/D20blur.png")?;
        //SpriteBatch alloc
        let d2_spr = SpriteBatch::new(d2);
        let d4_spr = SpriteBatch::new(d4);
        let d6_spr = SpriteBatch::new(d6);
        let d8_spr = SpriteBatch::new(d8);
        let d10_spr = SpriteBatch::new(d10);
        let d10p_spr = SpriteBatch::new(d10p);
        let d12_spr = SpriteBatch::new(d12);
        let d20_spr = SpriteBatch::new(d20);

        //Text allocations
        let pretty_color = Color::new(0.2, 0.60, 0.894, 1.0); //This is the cyan in the concept doc
        let press_start_2p = Font::new_glyph_font(ctx, "/PressStart2P-Regular.ttf")?; //Usage taken from https://github.com/ggez/ggez/blob/master/examples/text_cached.rs

        let mut d2t = TextCached::new("0")?;
        d2t.set_font(press_start_2p.clone(), Scale::uniform(DICE_NUM_PRINT_SIZE));
        let mut d4t = TextCached::new("0")?;
        d4t.set_font(press_start_2p.clone(), Scale::uniform(DICE_NUM_PRINT_SIZE));
        let mut d6t = TextCached::new("0")?;
        d6t.set_font(press_start_2p.clone(), Scale::uniform(DICE_NUM_PRINT_SIZE));
        let mut d8t= TextCached::new("0")?;
        d8t.set_font(press_start_2p.clone(), Scale::uniform(DICE_NUM_PRINT_SIZE));
        let mut d10t = TextCached::new("0")?;
        d10t.set_font(press_start_2p.clone(), Scale::uniform(DICE_NUM_PRINT_SIZE));
        let mut d10pt = TextCached::new("0")?;
        d10pt.set_font(press_start_2p.clone(), Scale::uniform(DICE_NUM_PRINT_SIZE));
        let mut d12t = TextCached::new("0")?;
        d12t.set_font(press_start_2p.clone(), Scale::uniform(DICE_NUM_PRINT_SIZE));
        let mut d20t = TextCached::new("0")?;
        d20t.set_font(press_start_2p.clone(), Scale::uniform(DICE_NUM_PRINT_SIZE));

        //Roll result
        let mut rollt = TextCached::new("0")?;
        rollt.set_font(press_start_2p.clone(), Scale::uniform(SCORE_TEXT_PRINT_SIZE));

        let x = PlayerAssets {
            //Dice Sprites
            d2: d2_spr,
            d4: d4_spr,
            d6: d6_spr,
            d8: d8_spr,
            d10: d10_spr,
            d10p: d10p_spr,
            d12: d12_spr,
            d20: d20_spr,
            //Text
            cyan: pretty_color,
            font: press_start_2p,
            //Number of banked Dicecoins
            //P1
            d2_text_p1: d2t.clone(), //Would end up basically doing this anyway
            d4_text_p1: d4t.clone(),
            d6_text_p1: d6t.clone(),
            d8_text_p1: d8t.clone(),
            d10_text_p1: d10t.clone(),
            d10p_text_p1: d10pt.clone(),
            d12_text_p1: d12t.clone(),
            d20_text_p1: d20t.clone(),
            roll_result_p1: rollt.clone(),
            //P2
            d2_text_p2: d2t,
            d4_text_p2: d4t,
            d6_text_p2: d6t,
            d8_text_p2: d8t,
            d10_text_p2: d10t,
            d10p_text_p2: d10pt,
            d12_text_p2: d12t,
            d20_text_p2: d20t,
            roll_result_p2: rollt,
        };

        Ok(x)
    }

    //Takes player data and updates text objects
    pub fn update_var(&mut self, p1: &Player, p2: &Player) {

        //We know that 0 is always the first index and there should really only be one index
        //P1
        self.d2_text_p1.replace_fragment(0,p1.d2_count.to_string());
        self.d4_text_p1.replace_fragment(0,p1.d4_count.to_string());
        self.d6_text_p1.replace_fragment(0,p1.d6_count.to_string());
        self.d8_text_p1.replace_fragment(0,p1.d8_count.to_string());
        self.d10_text_p1.replace_fragment(0,p1.d10_count.to_string());
        self.d10p_text_p1.replace_fragment(0,p1.d10p_count.to_string());
        self.d12_text_p1.replace_fragment(0,p1.d12_count.to_string());
        self.d20_text_p1.replace_fragment(0,p1.d20_count.to_string());
        self.roll_result_p1.replace_fragment(0,p1.roll_result.to_string());
        //P2
        self.d2_text_p2.replace_fragment(0,p2.d2_count.to_string());
        self.d4_text_p2.replace_fragment(0,p2.d4_count.to_string());
        self.d6_text_p2.replace_fragment(0,p2.d6_count.to_string());
        self.d8_text_p2.replace_fragment(0,p2.d8_count.to_string());
        self.d10_text_p2.replace_fragment(0,p2.d10_count.to_string());
        self.d10p_text_p2.replace_fragment(0,p2.d10p_count.to_string());
        self.d12_text_p2.replace_fragment(0,p2.d12_count.to_string());
        self.d20_text_p2.replace_fragment(0,p2.d20_count.to_string());
        self.roll_result_p2.replace_fragment(0,p2.roll_result.to_string());
    }

    //Draws player stuff on screen. This is ok because it is all fixed location and is not animated
    pub fn draw_var(&mut self, ctx: &mut Context, p1: &Player, p2: &Player, p1_highlight: &Color, p2_highlight: &Color) -> GameResult<()> {
        let p1_rolling_dice = p1.check_rolling_dice();
        let p1_bet = p1.check_bet();
        let p2_rolling_dice = p2.check_rolling_dice();
        let p2_bet = p2.check_bet();
        let p1_bet_coordinates = [(654.0,40.0),(716.0,40.0),(654.0,100.0),(716.0,100.0),(654.0,160.0),(716.0,160.0),(654.0,225.0),(716.0,225.0)];  //[d2,d4,d6,d8,d10,d10p,d12,d20] coordinates
        let p2_bet_coordinates = [(33.0,343.0),(97.0,343.0),(33.0,405.0),(97.0,405.0),(33.0,468.0),(97.0,468.0),(33.0,531.0),(97.0,531.0)];  //[d2,d4,d6,d8,d10,d10p,d12,d20] coordinates

        //Roll Result
        set_color(ctx, *p1_highlight); //By setting this we can add color on top of whatever we are drawing
        draw(ctx, &self.roll_result_p1, Point2::new(659.0,322.0), 0.0);
        set_color(ctx, *p2_highlight);
        draw(ctx, &self.roll_result_p2, Point2::new(39.0,256.0), 0.0);
        set_color(ctx, WHITE); //Need to reset to default color after done  to avoid coloring something else

        //Draw the rest of text
        set_color(ctx, BLACK); //By setting this we can add color on top of whatever we are drawing
        //P1
        draw(ctx, &self.d2_text_p1, Point2::new(237.0,62.0), 0.0);
        draw(ctx, &self.d4_text_p1, Point2::new(340.0,62.0), 0.0);
        draw(ctx, &self.d6_text_p1, Point2::new(441.0,61.0), 0.0);
        draw(ctx, &self.d8_text_p1, Point2::new(543.0,60.0), 0.0);
        draw(ctx, &self.d10_text_p1, Point2::new(240.0,175.0), 0.0);
        draw(ctx, &self.d10p_text_p1, Point2::new(341.0,175.0), 0.0);
        draw(ctx, &self.d12_text_p1, Point2::new(441.0,174.0), 0.0);
        draw(ctx, &self.d20_text_p1, Point2::new(541.0,175.0), 0.0);
        //P2
        draw(ctx, &self.d2_text_p2, Point2::new(237.0,405.0), 0.0);
        draw(ctx, &self.d4_text_p2, Point2::new(340.0,405.0), 0.0);
        draw(ctx, &self.d6_text_p2, Point2::new(441.0,404.0), 0.0);
        draw(ctx, &self.d8_text_p2, Point2::new(543.0,403.0), 0.0);
        draw(ctx, &self.d10_text_p2, Point2::new(240.0,518.0), 0.0);
        draw(ctx, &self.d10p_text_p2, Point2::new(341.0,518.0), 0.0);
        draw(ctx, &self.d12_text_p2, Point2::new(441.0,517.0), 0.0);
        draw(ctx, &self.d20_text_p2, Point2::new(541.0,518.0), 0.0);
        set_color(ctx, WHITE); //Need to reset to default color after done to avoid coloring something else

        //Draws the rolling dice on screen
        //We use a positive offset so the direction goes right, towards the center
        let mut counter: f32 = 0.0;
        for dice in p2_rolling_dice.iter(){
             let mut draw_me = match dice {
                DiceType::D2 => &mut self.d2,
                DiceType::D4 => &mut self.d4,
                DiceType::D6 => &mut self.d6,
                DiceType::D8 => &mut self.d8,
                DiceType::D10 => &mut self.d10,
                DiceType::D10p => &mut self.d10p,
                DiceType::D12 => &mut self.d12,
                DiceType::D20 => &mut self.d20,
                 _               => panic!("Unhandled DiceType in draw_var"),
            };
            draw_me.add(make_param((211.0 + (counter * ROLLING_OFFSET),281.0), (1.0,1.0), 0.0, (0.0, 0.0)));
            draw(ctx, draw_me, Point2::new(0.0,0.0), 0.0);
            draw_me.clear();

            counter += 1.0;
        }

        //We use a negative offset so it goes right, towards the center
        counter = 0.0; //reset counter
        for dice in p1_rolling_dice.iter(){
            let mut draw_me = match dice {
                DiceType::D2 => &mut self.d2,
                DiceType::D4 => &mut self.d4,
                DiceType::D6 => &mut self.d6,
                DiceType::D8 => &mut self.d8,
                DiceType::D10 => &mut self.d10,
                DiceType::D10p => &mut self.d10p,
                DiceType::D12 => &mut self.d12,
                DiceType::D20 => &mut self.d20,
                _               => panic!("Unhandled DiceType in draw_var"),
            };
            draw_me.add(make_param((540.0 + (counter * ROLLING_OFFSET_NEG),281.0), (1.0,1.0), 0.0, (0.0, 0.0)));
            draw(ctx, draw_me, Point2::new(0.0, 0.0), 0.0);
            draw_me.clear();

            counter += 1.0;
        } //Remember to reset counter if you want to use it again

        //Draws the betted dice on screen
        //Remember that SpriteBatches have that 3 line sequence we need
        let mut counter2: usize = 0; //reset counter
        for dice in p1_bet.iter() {
            let mut draw_me = match dice {
                DiceType::D2 => &mut self.d2,
                DiceType::D4 => &mut self.d4,
                DiceType::D6 => &mut self.d6,
                DiceType::D8 => &mut self.d8,
                DiceType::D10 => &mut self.d10,
                DiceType::D10p => &mut self.d10p,
                DiceType::D12 => &mut self.d12,
                DiceType::D20 => &mut self.d20,
                _               => panic!("Unhandled DiceType in draw_var"),
            };
            draw_me.add(make_param((p1_bet_coordinates[counter2].0,p1_bet_coordinates[counter2].1), (1.0,1.0), 0.0, (0.0,0.0)));
            draw(ctx, draw_me, Point2::new(0.0,0.0), 0.0);
            draw_me.clear();

            counter2 += 1;
        }

        counter2 = 0; //reset counter
        for dice in p2_bet.iter() {
            let mut draw_me = match dice {
                DiceType::D2 => &mut self.d2,
                DiceType::D4 => &mut self.d4,
                DiceType::D6 => &mut self.d6,
                DiceType::D8 => &mut self.d8,
                DiceType::D10 => &mut self.d10,
                DiceType::D10p => &mut self.d10p,
                DiceType::D12 => &mut self.d12,
                DiceType::D20 => &mut self.d20,
                _               => panic!("Unhandled DiceType in draw_var"),
            };
            draw_me.add(make_param((p2_bet_coordinates[counter2].0,p2_bet_coordinates[counter2].1), (1.0,1.0), 0.0, (0.0,0.0)));
            draw(ctx, draw_me, Point2::new(0.0,0.0), 0.0);
            draw_me.clear();

            counter2 += 1;
        } //Remember to reset counter2 if you want to use it again


        Ok(())
    }
}