# Day 14 Parabolic Reflector Dish

## Part 1

This was fun! I used bitmaps to represent the rocks, because that's normally a nice way to manipulate rows without having to deal with coordinates.

## Part 2

I was expecting to just have to tilt in other directions, however it turns out we need to do a lot more processing! This was stupidly hard. I save the hashes of each map so we can look for cycles. When we find a cycle we can break out and simulate just the last bit of it. This took ages to debug.
