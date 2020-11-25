# Reactor-Ls
Reactor-Ls racing game and engine

## Preface and Motivation
I'm a big fan of zero-g racers such as Sony's Wipe'Out. Since Wipe'Out is only supported on the playstation I switched to Redout by 34BigThings and I was totally excited about the overall sound design, physics and sense of speed. Sadly they never implemented a lot of features requested by their user base and - this is my main problem - it officially only runs under Windows while I migrated to Linux some years ago.

Even though the availability of Linux(-compatible) games is continuously increasing, I didn't find anything comparable to those aforementioned racing games. Being a developer with some random experience in computer games, I'm having this irrepressible itch to fix that. I've been collecting ideas over the recent years, sketched some game engines, even started to code parts of a game every now and then. By gaining more experience over time I had to admit that this is orders of magnitude big for a single person. And, without someone paying my bills, I won't be able to invest enough time.

So, am I asking for others to code my favorite game? Yes and no. I'm sure that what I have in mind is something for a bigger audience and has room for improvements - so it would be _our_ favorite game. Seeing that others are interested in the outcome will certainly boost my motivation to invest a bigger part of my spare time.

What about revenues? This won't make anyone rich and famous - but I wouldn't be exactly offended if that happens. First priority is that this game comes to life at all - at whatever state. The engine shall be open source and free in its heart but at the moment I think it's okay to allow commercial products based thereon (maybe mods, spin-offs, community servers).

## The Game

The projects outcome shall be a cross-platform racing game. The specific genre is often called "anti-gravity racer" or "pod-racer". That means the _ships_ are hovering above a given track rather than having wheels with friction. For the beginning I suggest the following games as orientation:

* [random gameplay example for WipeOut](https://www.youtube.com/watch?v=0n2aLJWhwOs)
* [random gameplay example for Redout](https://www.youtube.com/watch?v=7WaLiguCIb8)

A main difference between those games is that Redout incorporates real-world physics. This is something that I declare mandatory for Reactor Ls. I have no strong opinion about whether the ships shall be weaponized right now. Let's move that question to a later stage.

## The Engine

While the primary goal of this project is to have a playable racing game, I think it's wise to separate the game engine from the actual game. Either right from the start or at an early stage. This way other games can be built based on that engine without having to incoporate too many styles and wishes into a single application. The development of the engine will mature in parallel with the game until it gets stabilized.

## Technology and Processing

The language of choice for this project is [Rust](https://www.rust-lang.org/). It's a perfomant systems programming language targeting all common platforms (Linux, Windows, MacOS, Android, WebAssembly). It's okay to add another language for scripting purposes but no other compiled language shall be used.

Reactor-Ls shall be be runnable on Linux, Windows and MacOS. I'd like to see a runnable version on ChromeBooks, Android devices and Webbrowsers, too, but their implementation depends on the compromises that have to be made. I don't have a lot experience in that area. Would be great to discuss this in an early stage.

As the basic game principle is quite simple, it should be possible to strip down the graphices to a year 2000-level to make it runnable on a recent Raspberry Pi. That will also be a question of the Graphics API to use. Speaking of that: I think we should start with WebGL 2.0 (or OpenGL ES 3.0) which is widely supported and enough to get started. I like to see a Vulkan version in the long run, though.

I'm used to design my software by addressing every possible enhancement. As this would result in another Star Citizen I'll have to slap my own hands in this case. There are just too many options. Instead I'll try to embrace another Rust-feature: _fearless refactoring_. Just make the code do its job and generalize on a need-by-need basis. In my recent projects that worked out better than expected. Rust's strict type system made it incredibly hard to break things.

## Stages

### Stage 0: Make it runnable

### Stage 1: Make it playbable

### Stage 2: Make it enjoyable

### Stage 3: Make it competitive

### Stage 4: Make it extendable


## Enhancements and Wishlist

After the basic game is working I'd like to talks about adding some specials:

* In-game music. 
* Allow users to create their own ships within a set of fixed limitations (available energy, measures, etc.) they can add bigger and/or faster engines. This in turn makes the ship 
* Weapons
