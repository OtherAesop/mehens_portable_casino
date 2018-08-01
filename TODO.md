I just discovered that implementing things in a modular fashion using the ggez engine is pretty much impossible
becuase of the way event::run() and Context work in a thread unsafe fashion. It appears that my original idea to have
an easily expandable engine will need to be cut down to the following.
 
1) A set of enums which are loaded into a Vec<SceneType.>

2) The game will iterate its event::EventHandler traits over that Vec<...> and use potentially MASSIVE match statements to define game logic.
I don't know if it is possible to implement event::EventHandler for something besides the main trait but we will see.

3) This means that in practicum instead of having everything divided by reusable functions in separate files, everything will be handled inside of 
the main game loop and divided by match statements. Ugly I know.

4) I will cram as much as possible into functions to simplify it to the greatest extent possible 