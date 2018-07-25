#Mehen's Portable Casino

This is a small 2D gambling game made using the Good Game Easily (Ggez) engine. This game is designed to be short, small,
but well implemented and hopefully enjoyable. I also intend this to be a showcase of a concept that I came up with of
semi-randomized currency. Unfortunately the concept does not have any good economic usages, but was a fun project.

##Dicecoin

Dicecoin is licensed to you under the GNU General Public License as noted in the source file.

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

##Documentation

A problem I ran into when first starting with Rust as a tool for game development was that there were no well commented
examples available, so throughout the project code I have made the effort to document the code so that it is obvious for
beginners and as little effort as possible is needed to get up and running with something simple.

##Planned Features

-Dicecoin gambling game

