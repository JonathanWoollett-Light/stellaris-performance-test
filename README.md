# stellaris-performance-test

### What?

A rough simulation of calculating pops production and optimising jobs in Stellaris using gpu compute.

This does have all the job modifiers Stellaris has, but of course in a much cleaner enviroment with less restrictions and I'm sure a bunch of stuff missing.

### Why?

Stellaris performance annoys me, I've said this, I've got responses like 'well why don't you fix it?' or 'are you a programmer?', so I thought, you know what? I am, and I could atleast give it a go in a test of theory.

### How?

### Production

Production is very simple, component-wise multipling a bunch of vectors.

Only `species.count` in the equation is not a vector. It is an integer scalar.

All vectors are of floating point values and of the length of the number of resources in the game (an unmodded game has 11 resources).

![production equation image](https://github.com/JonathanWoollett-Light/stellaris-performance-test/blob/master/images/production.PNG)

While it is quite simple and the system I've done is very fast, there is still more to do, mostly concerning changing from iterating over vectors to using matricies.

### Optimization

Optimisation is a lot more complicated than production.

Right now I feature intra-planetary optimisation, this means pops can be perfectly (I think) optimised (moved between jobs) within planets to maximise production.

The first point of optimisation is what value we use to judge how well optimised our system is (what value are we trying to maximise).

I use the overall production of all resources component-wise multiplied by their market values then summed.

This effectively can convert the resource production of anything (empire/planet/job/worker) into a single scalar value.

Lets say a certain pop working a job produces 2 minerals, 1 energy and 0.5 exotic gases and 0.25 dark matter.

Thus we have a production vector (`...` is just to emit the values we don't need here so the example is clearer, the length of all vectors in this example are the number of resources (11)):

![prod_vec](https://github.com/JonathanWoollett-Light/stellaris-performance-test/blob/master/images/optimization/prod_vec.PNG)

We also have our makert values for our resources:

![market vec](https://github.com/JonathanWoollett-Light/stellaris-performance-test/blob/master/images/optimization/market_vec.PNG)

We component-wise multiply these 2 together:

![mul_vec](https://github.com/JonathanWoollett-Light/stellaris-performance-test/blob/master/images/optimization/mul_vec.PNG)

And we sum the resultant vector to get our scalar production value:

![sum_vec](https://github.com/JonathanWoollett-Light/stellaris-performance-test/blob/master/images/optimization/sum_vec.PNG)

### Tests

#### async 14k test

See the video.

In this test we are running a relatively small galaxy (atleast by my standard) with a day after every 0.1 seconds (100ms) giving 10 days of grace for the production calculation (meaning the calculation starts on the 30th and finishes before the 10th of the next month). Using these setting although there is an intial stutter on the first few calculations, as the process continues into later days caching starts to take affect and the bump disappears. Effectively, a day passes every 0.1 seconds in a galaxy of 14k pops with no perceivable slowdown. This is similar to fastest at the beginning of the game. I'd say that's pretty good (although the 10 day grace period is pretty huuuuuge I'll admit that, although its really only 1 second at 0.1 seconds per day).

### Addendum

I'm not having a go, I'm just running an experiment.

(in comparison to https://www.youtube.com/watch?v=9SFE89Wj8go 21,724 pops)

Also worth mentioning this is very rough and despite that I imagine it will slow down with adding a few features future optimisation will speed it up more.
