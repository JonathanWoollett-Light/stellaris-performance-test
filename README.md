# stellaris-performance-test

## What?

A rough simulation of calculating pops production and optimising jobs in Stellaris using gpu compute.

This does have all the job modifiers Stellaris has, but of course in a much cleaner enviroment with less restrictions and I'm sure a bunch of stuff missing.

## Why?

Stellaris performance annoys me, I've said this, I've got responses like 'well why don't you fix it?' or 'are you a programmer?', so I thought, you know what? I am, and I could atleast give it a go in a test of theory.

## How?

### Production

Production is very simple, component-wise multipling a bunch of vectors.

Only `species.count` in the equation is not a vector. It is an integer scalar.

All vectors are of floating point values and of the length of the number of resources in the game (an unmodded game has 11 resources).

![production equation image](https://github.com/JonathanWoollett-Light/stellaris-performance-test/blob/master/images/production.PNG)

While it is quite simple and the system I've done is very fast, there is still more to do, mostly concerning changing from iterating over vectors to using matricies.

### Optimization

Optimisation is a lot more complicated than production.

Right now I feature intra-planetary optimisation, this means pops can be perfectly (I think) optimised (moved between jobs) within planets to maximise production.

#### What are we maximising?

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


#### How do we maximise this value?

Now this is the tricky bit.

For ease here we will be looking at calculating for one empire.

In ours calculations here our end goal is to calculate the value of having each species in each job (job priorities)(the production value of 1 member of a species working said job).

##### Step 0: Definitions

So first off lets declare a few things (I'm skipping over how these things are created as its mostly standard programming tedium):

1. `jp`: Job Productions: A matrix of the productions of all jobs.
2. `jm`: Job Modifiers: A matrix of the empire modifiers of all jobs (you may get empire wide affects which modify the productions of certain jobs).
3. `sm`: Species Modifiers: A matrix of species modifiers for production (if a trait gives +20% minerals, then in the row for the species and value representing minerals would be 1.2).
4. `sem`: Species Empire Modifiers: A matrix of empire species modifiers for production (species policy can add modifiers, this is that).
4. `semp`: Species Employability: A matrix of species employability (1:employable,0:unemplyoable, certain species may not be able to work certain jobs (think traits like nerve stapled)).
5. `seemp`: Species Empire Employability: A matrix of empire species employability (1:employable,0:unemplyoable, certain empire specific policies may prevent species working certain jobs, think a species being slaves).
6. `mv`: Market Values: Market value of resources.
7. `em`: Empire Modifier: The empire wide modifier to all production (think mining guidsl giving +10% all minerals per month).

##### Step 1: Calculating intermediary vectors

Now we descend into the algorithm...

(`*` here means component-wise multiplication)

1. `ja = jp * jm`: Job Adjusted: This calculate the standard production of any job in this empire.
2. `csm = sm * sem`: Compressed Species Modifiers: This calculate the modifiers for a species in this specific empire.
3. `sem = sem * seemp`: Species Employability Mask: Represents what jobs species can work in this specific empire.
4. `emv = mv * em`: Empire Market Values: Relative market values of resources accounting for empire modifiers.

##### Step 2: Calculating job priorities

So... I'm no math whizz so it's kinda difficult to explain this since it's a relatively complex operation.

I'll try my best.

1. `ma`: Market Adjusted: Each row of `ja` is component-wise multiplied by `emv`.
2. `jprior`: Job Priorities: For each row, we transpose the row in `ma`, then matrix multiplied by the row in `csm` and the sum the resultant vector (this is carried out by [the SGEMM blas operation](http://www.netlib.org/lapack/explore-html/d4/de2/sgemm_8f.html))
3. `ejprior = jprior * sem`: Employable Job Priorities: `jprior` represents the production values of each species working each job, by multiplying `sem` over we set all values where the species cannot work to `0`.
4. `ids`: Ids: A (flattened) matrix with each element containing a job and a species id representing what what job & species the value in same location in `ejprior` refers to.
5 `pejprior = ejprior * pm`: Planet Employable Job Priorities: For each planet we multipy its modifier to get the job priorities for this specific planet (`pm`: Planet Modifier).
6. We flatten `pejprior` and zip it with `ids` such that we have a vector where each element contains a job priorites (floating point value) and the job id and species id for what the priority refers to. We define this vector as `ids_pejprior`.
7. We sort `ids_pejprior` in descending order by the priority values.
8. We iterate over `ids_pejprior` inorder assigning species to ideal jobs (this involves a little more programmatic complexity, but I don't think you need to understand this to understand how this algorithm works).

##### Step 3: Done

And that's it, jobs optimized perfectly (I think).

A couple points worth mentioning:
1. This same approach could be applied inter-planetary. That is it could optimize across your whole empire moving pops between worlds to acheive ideal production (not implement it yet though, although it is effectively the same algorithm).
2. Handling pop growth while optimisation is being calculated: There are 2 solutions here:
    1. This optimisation is some sort of edict the player must enact which temporarily halts pop growth (I prefer this idea as its far more thematic, people don't perfectly sort themselves into their ideal jobs on their own). Pops when grown simply get put in best aviable job for them and never move, unless this societal optimization edict is enacted  (I also prefer this thematically, like a family finding what they good at then it becoming a tradition they do forever even if they eventually stop being best).
    2. When pops are grown during the otpimization, when it completes they are made unemployed then assigned to the best jobs available to them not taken up by the optimisation.
3. Optimization can be initated 2 ways:
    1. Regularly on some interval for whole galaxy.
    2. By some edict. This algorithm works both inter-planetary and intra-planetary so the edict could be planetary or imeperial.
4. Using market values to judge the value of resources can be tricked. Especially if it optimizes on a regular interval a player could sell or buy a load of stuff from the market at once and massively change the market value just before the algorithm takes its snapshot of the market values. This could massively throw off how it assigns jobs. Fortunately in this case a solution is fairly easy: take an average of market values across time.

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
