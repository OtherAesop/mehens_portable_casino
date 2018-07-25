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

# Mehen's Portable Casino

This is a small 2D gambling game made using the Good Game Easily (Ggez) engine that implements the Dicecoin Game as
described below. This game is designed to be short, small, but well implemented and hopefully enjoyable. 
I also intend this to be a showcase of a concept that I came up with of semi-randomized currency. 
Unfortunately the concept does not have any good economic usages, but was a fun project.

## Dicecoin Game

Please note these rules are currently in rough draft form and may be changed in the future.

##### Setup

Each player starts the game with a pool of 25,000 DC and a set of Dicecoins (a D2, D4, D6, D8, D10, D10P, D12, and D20.) 
Any computer characters will start with a pool of 50,000 DC and a set of Dicecoins. 

##### Betting Phase

At the beginning of a turn, moving counter-clockwise and ending with the dealer or house if one exists, each player will
select between 1 and 4 of their Dicecoins and put them into their playing field at the center of the table with a bet in DC.

##### Raising Phase

Turns will repeat like this until each player (and the house if applicable) have had a turn. Each player, beginning with
the first player to take a turn and moving counter clockwise, will have the chance to raise their DC bet or pass. After 
the last player raises or passes then whoever has the highest bet will select one of these options.

1) Coos (low)
2) Pearls (high)

##### Rolling Phase

Each player then rolls the Dicecoin they put into the pool and adds the rolled numbers.

If Coos was selected then the player with the lowest rolled value wins.
If Pearls was selected then the player with the highest rolled value wins.

The winner then takes the entire pool of DC. In the event of a tie then the winners evenly split the bet, with any 
remainder going to the winner who is earliest in the playing order.

##### Purchase Phase

At this point each player may replenish their supply of Dicecoins by buying them at house prices but may never possess
more Dicecoins then they began the game with. A player with no Dicecoins to bet, or who cannot wager the minimum bet after
the Purchase Phase on a table loses. Players may choose to leave the table at this point with no penalty, leaving at any
other point forfeits their bet and all the Dicecoins they began the game with.

## Dicecoin Concept

Dicecoin is licensed to you under the GNU General Public License as stated in the source file.

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

## Documentation

A problem I ran into when first starting with Rust as a tool for game development was that there were no well commented
examples available, so throughout the project code I have made the effort to document the code so that it is obvious for
beginners and as little effort as possible is needed to get up and running with something simple.

## Planned Features

-Dicecoin gambling game

-Dicecoin STL file upload

