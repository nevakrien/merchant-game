# merchant-game

This game is inspired by port royale FTL and other simple economics games.
The hope here is to actually simulate a market instead of runing mostly random numbers.

To this end the NPCs would have parts trained with RL models written in pytorch so that we can actually have smart behivior.

The main idea is that merchants only know what they can directly observe or learn from other merchants. They can travel between cities, maintain inventories, post trade offers, remember previous trades, and gossip about events they have seen.

Cities therefore do not have a single fixed "price" for an item. Instead merchants expose offers, other merchants decide whether those offers are worth taking, and prices emerge from the interactions between agents.

The simulation is written in Rust, while learned policies are intended to be trained in PyTorch. The game logic and policy interface are kept separate so that learned policies can be compared against simple scripted ones.