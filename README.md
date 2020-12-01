# 📢 Notice

I have currently given up becuase:

1. [Serde](https://serde.rs/) is amazingly difficult to use for data formats with indentation ([my stackoverflow question asking how to do it](https://stackoverflow.com/questions/64267249/serde-how-to-implement-multiple-custom-sequences?noredirect=1#comment113652330_64267249)).
2. The paradox save file format is awful.

# 🏎️ Stellaris performance test

## Intro

### What?

A rough simulation of calculating pops production and optimising jobs in Stellaris using gpu compute.

This does have all the job modifiers Stellaris has, but of course in a much cleaner enviroment with less restrictions and I'm sure a bunch of stuff missing.

### Why?

Stellaris performance annoys me, I've said this, I've got responses like 'well why don't you fix it?' or 'are you a programmer?', so I thought, you know what? I am, and I could atleast give it a go in a test of theory.

### How? Pls see [explanation.pdf](https://github.com/JonathanWoollett-Light/stellaris-performance-test/blob/master/explanation.pdf)
(yeah thats the best way I can do it, I am amazed github lacks LaTeX support)

## Tests

In general, its fast, in my simulations of 10s of thousands you don't notice either optimization or production running when time is pasisng at 0.5s per day.

Will give better illustrations of performance in a bit, got to make it able to load a Stellaris game save next (which is real annoying since they use some proprietary format for there savegame files).

### async 14k test

See the video.

In this test we are running a relatively small galaxy (atleast by my standard) with a day after every 0.1 seconds (100ms) giving 10 days of grace for the production calculation (meaning the calculation starts on the 30th and finishes before the 10th of the next month). Using these setting although there is an intial stutter on the first few calculations, as the process continues into later days caching starts to take affect and the bump disappears. Effectively, a day passes every 0.1 seconds in a galaxy of 14k pops with no perceivable slowdown. This is similar to fastest at the beginning of the game. I'd say that's pretty good (although the 10 day grace period is pretty huuuuuge I'll admit that, although its really only 1 second at 0.1 seconds per day).

Vs current performance: https://www.youtube.com/watch?v=9SFE89Wj8go 21,724 pops

## TODO

Add importing/exporting of a save game.

## Addendum

I'm not having a go, I'm just running an experiment.

Also worth mentioning this is very rough and despite that I imagine it will slow down with adding a few features future optimisation will speed it up more.
