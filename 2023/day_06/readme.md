# Day 6 Wait For It

## Part 1

Oh god, I think this is just maths. The equation is
`(x - n) * (x - (x - n))`
Where x is the number of milliseconds we have and n is how long we hold the button down for. Notice the results have a nice symmetry to them.
We can simplify this to
`xn - n^2`

Then we have to solve where it intersects at the record, so
`xn - n^2 = r`
or
`n^2 - xn + r = 0`
For the first example this is
`n^2 - 7n + 9 = 0`

Erm so at this point I look up how to quadratic equations...

## Part 2

Pretty much the same but with a bigger number. Switched to 64bit versions of ints and floats.
