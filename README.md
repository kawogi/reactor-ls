# Reactor-Ls
Reactor-Ls racing game and engine - planning phase

## Preface and Motivation
I'm a big fan of zero-g racers such as Sony's Wipeout. Since Wipeout is only supported on the Playstation I switched to Redout by 34BigThings and I was totally excited about the overall sound design, physics and sense of speed. Sadly they never implemented a lot of features requested by their user base and - this is my main problem - it officially only runs under Windows while I migrated to Linux some years ago.

Even though the availability of Linux(-compatible) games is continuously increasing, I didn't find anything comparable to those aforementioned racing games. Being a developer with some random experience in computer games, I'm having this irrepressible itch to fix that. I've been collecting ideas over the recent years, sketched some game engines, even started to code parts of a game every now and then. By gaining more experience over time I had to admit that this is orders of magnitude big for a single person. And, without someone paying my bills, I won't be able to invest enough time.

So, am I asking for others to code my favorite game? Yes and no. I'm sure that what I have in mind is something for a bigger audience and has room for improvements - so it would be _our_ favorite game. Seeing that others are interested in the outcome will certainly boost my motivation to invest a bigger part of my spare time.

What about revenues? This won't make anyone rich and famous - but I wouldn't be exactly offended if that happens. First priority is that this game comes to life at all - at whatever state. The engine shall be open source and free in its heart but at the moment I think it's okay to allow commercial products based thereon (maybe mods, spin-offs, community servers).

## The Game

The project's outcome shall be a cross-platform racing game. The specific genre is often called "anti-gravity racer" or "pod-racer". That means the _ships_ are hovering above a given track rather than having wheels with friction. For the beginning I suggest the following games to get an idea:

* [random gameplay example for WipeOut](https://www.youtube.com/watch?v=0n2aLJWhwOs)
* [random gameplay example for Redout](https://www.youtube.com/watch?v=7WaLiguCIb8)

A main difference between those games is that Redout incorporates real-world physics. This is something that I declare mandatory for Reactor Ls. I have no strong opinion about whether the ships shall be weaponized right now. Let's move that question to a later stage. But we need jumps, loopings and crashes for sure!

## The Engine

While the primary goal of this project is to have a playable racing game, I think it's wise to separate the game engine from the actual game. Either right from the start or at an early stage. This way other games can be built based on that engine without having to incoporate too many styles and wishes into a single application. The development of the engine will mature in parallel with the game until it gets stabilized.

## Technology and Processing

The language of choice for this project is [Rust](https://www.rust-lang.org/). It's a perfomant systems programming language targeting all common platforms (Linux, Windows, MacOS, Android, WebAssembly). It's okay to add another language for scripting purposes but no other compiled language shall be used.

Reactor-Ls shall be be runnable on Linux, Windows and MacOS. I'd like to see a runnable version on ChromeBooks, Android devices and Webbrowsers, too, but their implementation depends on the compromises that have to be made. I don't have a lot experience in that area. Would be great to discuss this in an early stage.

As the basic game principle is quite simple, it should be possible to strip down the graphics to a year 2000-level to make it runnable on a recent Raspberry Pi. That will also be a question of the Graphics API to use. Speaking of that: I think we should start with WebGL 2.0 (or OpenGL ES 3.0) which is widely supported and enough to get started. I like to see a Vulkan version in the long run, though.

I'm used to design my software by addressing every possible enhancement. As this would result in another Star Citizen I'll have to slap my own hands in this case. There are just too many options. Instead I'll try to embrace another Rust-feature: _fearless refactoring_. Just make the code do its job and generalize on a need-by-need basis. In my recent projects that worked out better than expected. Rust's strict type system made it incredibly hard to break things.

## Stages

To have something to aim for I suggest some coarse milestones (no dates, just feature sets).

Other things like organization, deciding feature details, etc. shall be permanent background tasks.

### Stage 0: Make it runnable

This will be kind of a "Hello, World!" program.

* some simple 3D-shapes should be displayed and animated (working render loop)
* basic keyboard input shall be captured and maybe affect the 3D scene
* the program writes its logs into a file and/or terminal
* settings can be read from a config file

### Stage 1: Make it playbable

* implement basic game mechanics (controls, collision detection, inertia, hovering)
* add a rudimentary track to test the physics (jumps, loopings, maybe forks and joins)
* show the ship

### Stage 2: Make it enjoyable

This will be the most challenging stage as it defines the public perception of the game. This can take a long time and might require several iterations. The stage implies there is some kind of a level editor.

I'd be happy if it was possible to split this milestone.

* design a proper level with textures, effects, colors
* design a ship and cockpit
* add sounds (engine, collisions, announcements)
* add music
* add menus and settings
* add visual effects
* define one or more game modes (time trial, etc.)

### Stage 3: Make it competitive

Everything that makes it a multiplayer game.

* add network connectivity
* create some highscore tables
* create a dedicated (headless) server to host games
* add more game modes (race, last man standing, etc.)
* develop a driver A.I.

### Stage 4: Make it extendable

It shall be possible for non-programmers to improve the game or build their own ideas and mods.

* move the game rules into some script
* define workflow to create new tracks/maps/ships
* move the ship mechanics into a scripting language
* make it possible to write your own driver A.I.

## Enhancements and Wishlist

After the basic game is working I'd like to talk about adding some specials:

* In-game music shall react on the player's action
* Allow users to create their own ships within a set of fixed limitations (available energy, measures, etc.) they can add bigger and/or faster engines. This in turn makes the ship heavier and might influence the air friction. The framework may be chosen to be light but damageable or heavy but sturdy. It might be possible to steer by thrust and navigation engines or by panning/tilting the engines. Controls might be scriptable. The ultimate goal is that users can highly optimize their ships while still being able to fairly compete. Everything should be backed by real-world physics.
* Weapons
* Maybe the engine can be made flexible enough for races or missions in free space without being bound to a track.
* It might be necessary to decide what graphical style the engine can handle best. Wipeout tries to show a rather realistic environment while Redout just tries to make it look appealing without sacrificing too much performance. This often results in low-poly models and flat shading on purpose. I personally like the "cyperpunk" style. Neon lights, glass, reflections, unnecessarily futuristic stuff. Those are ideal to be procedurally generated. No need to fill gigabytes of textures and fine meshes with LOD-information. That simplifies the engine but makes it less general.

## About me

My primary language is German, so please give me a hint if something needs to be fixed or cleared up language-wise.

I have some experience in early demo programming and my math and physics skills are at least above average. I wrote some physical simulations, raytracers, sound and music synthesizers, did some image processing and I'm familiar with some assembly languages and low-level optimizations on general. I'm not an expert in any of those fields but it's enough to start some serious work.

I usually try to avoid external libraries as they tend to enforce a certain programming style and creep in unnecessary restrictions. Also it can be hard to get rid of them, once their development took a wrong turn or even stopped. But as [crates.io](https://crates.io) offers a lot of very generic little helpers, we're fine. I just don't want to build on an existing bloat-engine just to learn what we can _not_ accomplish because of some external design decisions. We don't need to re-invent the wheel but re-inventing the car is totally an option.

I have absolutely no experience in leading an open source project and I'm fairly new git and GitHub. This will be my personal learning curve.
