# Day 11 Monkey in the Middle

## Part 1

In which we do loads of regex and iterate through our monkeys and chuck stuff. This took ages! I finally got the regex library out and it took a while to get all the bits right (notice that some of the multiplies can multiply self, not just other numbers). Iterating and mutating the list of monkeys isn't really allowed in Rust so that needed a bit of shaking into shape too. Not proud of today's code.

## Part 2

This is like the first part but with much higher numbers so it overflows as it is. One way to not make it overflow is find the a common factor that all the modulo operations can divide into (the LCM).