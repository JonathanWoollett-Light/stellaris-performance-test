# stellaris-performance-test

A rough barebones simulation of calculating pops production in Stellaris using gpu compute.

This does have all the job modifiers Stellaris has, but of course in a much cleaner enviroment with less restrictions and I'm sure a bunch of stuff missing.

### Why

Stellaris performance annoys me, I've said this, I've got responses like 'well why don't you fix it?' or 'are you a programmer?', so I thought, you know what? I am, and I could atleast give it a go in a test of theory.

### Tests

#### Test 1: 36k

From a test of 36,030 pops with 10 empires, 10 resources, 30 species and 50 jobs (and a couple other parameters which are ranges) these are current times (iteration : s : ms) (iteration is basically the month number) for calculating pop production:

pop_sum: 36,030
1. : 05:655
2. : 03:418
3. : 02:812
4. : 02:789
5. : 02:819
6. : 02:808
7. : 02:791
8. : 02:799
9. : 02:806
10. : 02:816

total : 31:518

#### Test 2: 99k

In a test with 99,150 pops, 20 empires, 10 resources, 30 species and 50 jobs (and a couple other parameters which are ranges (and have been increased)) these are the timings:

pop_sum: 99,150
1. : 11:464
2. : 08:889
3. : 09:448
4. : 09:780
5. : 11:329
6. : 11:923
7. : 11:119
8. : 11:093
9. : 12:926
10. : 16:067

total : 114:044

### Addendum

I'm not having a go, I'm just running an experiment.

(in comparison to https://www.youtube.com/watch?v=9SFE89Wj8go is only 21,724 pops)

Also worth mentioning this is very rough and despite that I imagine it will slow down with adding a few features future optimisation will speed it up more.
