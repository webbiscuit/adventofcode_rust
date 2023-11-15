# Day 18: Boiling Boulders

## Part 1

Here we check all the neighbours of a cube and add up how many surfaces are exposed. This didn't feel too bad so I suspect part 2 will be horrific.
There was a little niggle with Rust not liking me checking -1 (it was all unsigned) but I put in a set and checked for itself in the neighbour set and all was good.

## Part 2

Oh yay. So this is like part 1, but removing all of the internal cubes. I think to do this we have to check each surface to see which way it is facing, and then somehow know it's facing inwards or outwards. This sounds like ray casting which is hard.