# Shoefull
A small unfinished 2D platformer

![alt text](https://i.imgur.com/Ykqcvyf.png)

Shoefull is a test project using a Rust library called ggez to "create a Good Game Easily."

Shoefull is displayed using the Rust canvas and only uses keyboard inputs to allow the user to navigate through the menu/play the game. The latest release only consists of a simple target practice mode where the player must collide with randomly generated targets while dashing in the fastest time possible to reach a score of 10.

# Compiling/Running

Install the Rust compiler toolchain via ![```rustup```](https://rustup.rs/) and then cd to the folder of this repo and run ```cargo run --release```.

It is essential that Shoefull is compiled with ```--release``` due to the undesirable results debug mode delivers, mainly being the slow animation of spritebatches in the ggez library.

# Controls

```W A S D```  MOVE/JUMP


```J```  DASH

```P```  PAUSE

# Screenshots
![alt text](https://i.imgur.com/fRplxM3.png)

![alt text](https://i.imgur.com/dO0d1GU.png)

![alt test](https://i.imgur.com/xBav025.png)

[```Click here for Game animations```](https://www.piskelapp.com/user/5510105074761728/public)

Credit to [```Simrat Bains```](https://github.com/Simratt) for game concept.
