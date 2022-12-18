# Day 15: Beacon Exclusion Zone

## Part 1

Oh, now they are getting really difficult :( So, the grid is huge so we cannot calculate it all in memory. So we must programmatically calculate the row at a time. It's not much code but the logic is a bit tricky - basically for each x point (you are given the y) you must loop through all the sensors and calculate if their sensor range is your spot. If it is, you add a forbidden square. It does not run instantly.
Extra tricky bit is that some beacons are on this line, so you have to check for those too. If you are out by 1 or 2 that's probably why.

## Part 2

Ah crap, now we have to do the above lots of times, of course! As part 1 is so slow I've done some wrong thing.

~some time later~

Okay here's one optimisation - instead of looping through the first x we find up to the last x we find, we can loop through all of the individual sensor x ranges. Because we already know lots of the x values are not part of any sensor, so we only want to check the ones that are close to a sensor.
Now this is better but we have an opposite problem - we're sometimes checking an x value more than once. But then this doesn't really matter because checking each x coordinate at a time is silly, we can use this new range thing we just calculated. So let's store ranges of x coordinates instead of each individual point. This makes the calculations even trickier but it will be a lot faster. 
And then there's a new problem that we have to stitch all of these ranges together, and some of them overlap.

~some time later~

I had a chat with chatGPT and it told me about something called KDTrees or Quadtrees that are optimised for range queries. What we have at the minute is still really slow, I need to find a way for finding the gap in the ranges. I'll maybe do this later. At least the example works!


