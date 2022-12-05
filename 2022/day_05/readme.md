# Day 5 Supply stacks

## Part 1

In which we play with stacks and do some tricky parsing. This isn't too tricky if you keep track of the stacks correctly - nice classes help. But the parsing is a bit nasty, because the crates are read in top first, but the stack is bottom first. We could reverse the list but that's a bit annoying, so the way I've done it is find the bottom of the crate diagram first and build up. The instruction parsing should probably be done with regex but can be done in an ugly way instead.

## Part 2

This is much like part 2 except the stacking strategy is changed to multiple items instead of popping/pushing one at a time.