# License

mehens_portable_casino. A gambling game made using ggez and Dicecoin

Copyright (C) 2018  Ian L. Gore

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

## Dicecoin License

The expression of the Dicecoin concept here is licensed under the GPL

# Mehen's Portable Casino

This is a small 2D gambling game made using the Good Game Easily (Ggez) engine that implements the Dicecoin Game as
described below. This game is designed to be short, small, but well implemented and hopefully enjoyable. 
I also intend this to be a showcase of a concept that I came up with of semi-randomized currency. 
Unfortunately the concept does not have any good economic usages, but was a fun project.

## Dicecoin Game

Please note these rules are currently in rough draft form and may be changed in the future.

##### Setup

Each player starts the game with a set of Dicecoins (a D2, D4, D6, D8, D10, D10P, D12, and D20.)

Please note in the game each player begins with 3 sets.

##### Betting Phase

At the beginning of a turn, moving counter-clockwise and ending with the dealer or house if one exists, each player will
select between 1 and 2 of their Dicecoins and put them into their playing field at the center of the table.

##### Raising Phase

Turns will repeat like this until each player (and the house if applicable) have had a turn. Each player, beginning with
the first player to take a turn and moving counter clockwise, will have the chance to bet any number of Dicecoins. After 
the last player raises or passes then whoever has bet the most dice will make one of the following calls.

1) Coos (low)
2) Pearls (high)

##### Rolling Phase

Each player then rolls the Dicecoin they put into the pool and adds the rolled numbers.

If Coos was selected then the player with the lowest rolled value wins.
If Pearls was selected then the player with the highest rolled value wins.

The winner then takes the entire pool of Dicecoins. In the event of a tie then the winners evenly split the bet, with any 
remainder going to the winner who is earliest in the playing order.

Please note in the video game this is decided randomly.

##### Purchase Phase

At this point each player may replenish their supply of Dicecoins by buying them at house prices but may never possess
more Dicecoins then they began the game with. A player with no Dicecoins to bet, or who cannot wager the minimum bet after
the Purchase Phase on a table loses. Players may choose to leave the table at this point with no penalty, leaving at any
other point forfeits their bet and all the Dicecoins they began the game with.

Please note this is not implemented in the game.

## Dicecoin Concept

While this program is licensed to you under the GPL the Dicecoin concept is not.

A Dicecoin is a polyhedral dice that has different values on each side. These values represent the worth, in DC of a coin
after it is rolled. Dice can really only be practical up to a certain number of faces in a physical form so the standard
set is the dice with 2, 4, 6, 8, 10, 12, and 20 faces plus one 10-sided dice that has standard 'D10' numbers 1-10 printed
on it.

These faces can have any value but generally speaking they should made so that as the number of faces gets smaller, the
possible maximum and minimum values become very far apart (such as 1 and 10,000 on a D2.) How you pick the faces is up
to you, but pick so that they resemble some sort of currency of your choosing or perhaps a whole system of your design.

When a Dicecoin is spent you roll it and it's value (in DC) permanently becomes the rolled value. DC can be spent to purchase new 
Dicecoins without rolled values at exchanges or transferred once like normal currency. Changing the purchase prices of
Dicecoins while requiring that once rolled they must be spent and then used to purchase new Dicecoins. Additionally you
can require that they be rolled at the time of purchase if that fits your implementation purposes.

### Dicecoin Canonical Definition

I modeled my version of Dicecoin such that the expected values of each dice are roughly equivalent to the various
values of Euros, excluding a few I had no need for. Note that I consider the percentile dice to be an obsolete
coin since its expected value is so low that I imagined they would be the rough equivalent of the US penny or Japanese ¥1
coin but have included them because why not?

Canonical Dicecoin Definitions (note D stands for dice and the number stands for the number of faces, and P stands for percentile)
1) D2 - 1, 4000                                                                 (Expected Value: 2000.5)
2) D4 - 1, 750, 1500, 2000                                                      (Expected Value: 1062.75)
3) D6 - 1, 200, 400, 600, 800, 1000                                             (Expected Value: 500.2)
4) D8 - 1, 15, 30, 45, 60, 75, 90, 500                                          (Expected Value: 102)
5) D10 - 1, 20, 30, 40, 50, 60, 70, 80, 90, 100                                 (Expected Value: 54.1)
6) D10P - 1, 2, 3, 4, 5, 6, 7, 8, 9, 10                                         (Expected Value: 5.5)
7) D12 - 1, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 50                            (Expected Value: 22.6)
8) D20 - 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20  (Expected Value: 10.5)

## Documentation

A problem I ran into when first starting with Rust as a tool for game development was that there were no well commented
examples available, so throughout the project code I have made the effort to document the code so that it is obvious for
beginners and as little effort as possible is needed to get up and running with something simple. I extended this practice
to pretty much every file so it is my hope that you can click on any file and know the following

1) At least a vague idea of what the syntax means

2) The intended purpose of the code

### Game Architecture

Most of the example projects I ran into in Ggez were messy assemblies of match statements with little to no documentation. Not
only is this not expandable, it is ugly. To get past these hurdles I designed my game in such a way that with minimal fuss
you can expand, add in scenes, delete scenes, and change execution ordering by using enum handles (somewhat like the way Unity allows you to build
your game scenes together.)

To support this the game is divided into four distinct sections

1) game_logic - This is where all the code that controls the flow of scenes and accessory structs is located. By using the SceneType enum handle,
we are able to define a Vec<SceneType> such that MainState will cycle through them infinitely until it encounters a SceneType::Exit
or you just exit out.

2) scenes - This is where most of Ggez is actually called and used to implement the game scenes. Supporting logic files are found
in game_logic

3) gambling - This defines a dice rolling suite that can be used to create Dicecoins with documented guards and limitations

4) assets - This contains all of the game design concept art, design photos, etc (except the Dicecoin STLs which are in the release
binary) in the hopes that it is useful in some manner.

The game essentially functions like a loop within a loop. MainState operates and controls the game scene flow loop while the games themselves have loops
within them that allow you to actually *do* something. Adding in new stuff should basically boil down to these steps.

1) Creating a scene struct that implements 'event::EventHandler', though you can change the parameters since this is not an actual trait implementation. Make sure
you have some way for the game to terminate! The default method to do this is through 'key_down_event', but you can expand MainState and the game structs to handle
more termination conditions as you need by expanding the return values of the functions called in MainState.

2) Adding the relevant function calls (and initializing statements) to the scene struct into MainState inside of 'event::EventHandler' trait implementation's match statements with whatever
enum SceneType handle you think is most appropriate.

3) Adding your new scene into the game loop execution by adding it into the scenes created in 'make_scenes' inside of 'game_logic::utility_functions' 

By default MainState only checks for transitions on key_down_event, but you can expand this to trigger on any of the provided event::EventHandler
functions by adding in the return values as needed to update or draw and then handling them appropriately. 

### Gameplay Instructions

This project is meant as a showcase and thus does not implement the Setup phase or the Purchase phase. I make minimal use
of text in this game due so it is important to pay attention to the Shift Keys, and which are highlighted. Please note that
in the event of a tie in bet or rolled result, the winner of that contest will be decided randomly.

Player 1 is on the top half of the screen and has the color red while Player 2 is on the lower half and has the color blue 

#### Controls Player 1

Q, W, E, R, A, S, D, F - Press these to bet or put up a corresponding Dicecoin during the appropriate phase (Player 1)

Left Shift - Press this to end your turn, this will be greyed out when you are not allowed to end (Player 1)

#### Controls Player 2

Y, U, I, O, H, J, K, L - Press these to bet or put up a corresponding Dicecoin during the appropriate phase (Player 2)

Right Shift - Press this to end your turn, this will be greyed out when you are not allowed to end (Player 2)

#### Controls Coos/Pearls

C - Press this to gamble on Coos (The shift key of the player who has the right to make this move will be colored)
P - Press this to gamble on Pearls (The shift key of the player who has the right to make this move will be colored)

### Installation

#### Windows

Clone the project into an IDE of your choice that supports Rust and run the cargo command 'run --release' however your IDE 
(which is probably IntelliJ or Clion) supports it. If you want to play the debug build then use the command 
'cargo run' however this is unrecommended since there are known issues on the debug build.

I recommend IntelliJ Community Edition or CLion. Visual Studio is untested.

Download the precompiled executable release [here](https://github.com/MushuYoWushu/mehens_portable_casino/releases "Mehen's Portable Casino Release Download") and run it, being careful that all the files are in the same folder.
Please note the Dicecoin STL files must be downloaded through that zip file in order to reduce the download cost of cloning a repo

#### Mac

I imagine the process is similar to Windows but I never use Mac's, so until I get a platform to test on or somebody is kind
enough to do a test run for me, this is unsupported.

#### Linux

The same IDE instructions as for Windows apply.

Follow [these beautiful instructions](https://services.github.com/on-demand/github-cli/clone-repo-cli "How to Clone a Repo in Linux")
courtesy of GitHub to clone the repository and then type 'cargo run --release' inside the correct directory.

#### SDL2

Note that this should run fine on most non-jurassic period computers, but if you try and run the executable directly from the target directory
it will expect that the directory contains a folder named 'resources' with all the games assets and an SDL2.dll which is downloadable from the Windows
release page inside of the game's zip.

### Inspiration

These two projects, this standard example, and an 8 month long Dnd campaign were inspirations and references for this project
and for ggez syntax tutorials.

The Wizzer of Oz

https://github.com/maccam912/thewizzerofoz

Llamassacre

https://github.com/rap2hpoutre/llamassacre

Text-Cached Example

https://github.com/ggez/ggez/blob/master/examples/text_cached.rs

## Completed Features

-Dicecoin Game

-Dicecoin STL

## Planned Features

N/A

