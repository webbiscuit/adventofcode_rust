# Day X

## Part 1

1, 4, 7, 8 can be found using the number of segments in the input.

## Part 2

This is tougher.

With 1 4 and 7 we know cf segments.
The difference between 4 and 1 gives you the bd segment.

Then we can count the number of segments and check a few things to find what they are.

2, 3, 5 - (5 segs)
Does it have bd? Yes, it's 5
Does it contain 1? It's 3
Else it's 2


0, 6, 9 - (6 segs)
Does it contain 4? It's 9
Else does it contain 1? It's 0
Else it's 6 (or does it contain 5)