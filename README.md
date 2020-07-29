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

# Further Notes

Game design is a field I am a complete stranger to and because I decided to pick up a language I've never used (let alone heard of) it became extremely challenging to familiarize myself with the whole concept really. Shoefull is my first ever (functioning) game and is made as a test to figure out the intricacies of designing gui/game sprites and coding the physics of a 2D platformer. I had an easy time figuring out the look of the game using an online spritemaker, but most—if not all—of the challenges I faced came from a lack of experience in the resources I was using and the number of hours I'd slept in my Physics class. Once I was able to bring the graphics to life, all that had to be done was figure out how I wanted the game to feel in terms of movement. This took hours of moving back and forth between horizontal/vertical acceleration values and adjusting sprite positions in accordance to the Rust canvas which became a harder task than I expected it to be. Eventually I was able to create passable movement physics and all that needed to be done was create an actual game out of it. At this point I was very content with what I had already created so I settled to make my first release of the game a reaction test target collider and that is what you see here.

# WIP

* Use WASM and js to make shoefull playable in your browser

* Add different shoes (powerups) that the player can test out in target practice mode

* Create a story mode



Credit to [```Simrat Bains```](https://github.com/Simratt) for game concept.
