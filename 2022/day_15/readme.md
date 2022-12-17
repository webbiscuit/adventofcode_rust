# Day 15: Beacon Exclusion Zone

## Part 1

Oh, now they are getting really difficult :( So, the grid is huge so we cannot calculate it all in memory. So we must programmatically calculate the row at a time. It's not much code but the logic is a bit tricky - basically for each x point (you are given the y) you must loop through all the sensors and calculate if their sensor range is your spot. If it is, you add a forbidden square. It does not run instantly.
Extra tricky bit is that some beacons are on this line, so you have to check for those too. If you are out by 1 or 2 that's probably why.

## Part 2
