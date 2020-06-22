# stellaris-performance-test

A rough barebones simulation of calculating pops production in Stellaris using gpu compute.

This does have all the job modifiers Stellaris has, but of course in a much cleaner enviroment with less restrictions and I'm sure a bunch of stuff missing.

### Why

Stellaris performance annoys me, I've said this, I've got responses like 'well why don't you fix it?' or 'are you a programmer?', so I thought, you know what? I am, and I could atleast give it a go in a test of theory.

### Tests

#### async 14k test

See the video.

In this test we are running a relatively small galaxy (atleast by my standard) with a day after every 0.1 seconds (100ms) giving 10 days of grace for the production calculation (meaning the calculation starts on the 30th and finishes before the 10th of the next month). Using these setting although there is an intial stutter on the first few calculations, as the process continues into later days caching starts to take affect and the bump disspears. Effectively, a day passes every 0.1 seconds in a universe of 14k pops with no perceivable slowdown. This is similar to fastest at the beginning of the game. I'd say that's pretty good.

### Addendum

I'm not having a go, I'm just running an experiment.

(in comparison to https://www.youtube.com/watch?v=9SFE89Wj8go is only 21,724 pops)

Also worth mentioning this is very rough and despite that I imagine it will slow down with adding a few features future optimisation will speed it up more.
