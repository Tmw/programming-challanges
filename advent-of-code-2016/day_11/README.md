# Day11

This took waaay to long to solve 😳

```bash
iex -S mix

iex(1)> Day11.solve
Found answer to part_a in 752 ms. Answer is = 31 steps
Found answer to part_b in 12303 ms. Answer is = 55 steps
:ok
```

## Issues

- Had a bug in my hashing function which caused many collisions to happen. Many valid states would be skipped because of it, and the algorithm would terminate without any result.
- BFS got me the answer to part A but it took a mere 20 minutes without any optimalisations. For part B we'll try A\*.
- Running part A with the A\* algorithm definitively sped up (~ 10 min). Part B, however, ran for 8+ hours and counting, with no answer thus far. After spending some time reading up on BFS, A\* and other search algorithm, and how to optimize them, I gave up and read up on some tips over at the (Advent Of Code Subreddit)[https://www.reddit.com/r/adventofcode/comments/5hoia9/2016_day_11_solutions/].
- After implementing the optimalization where we don't try to move items down a level when that floor is already cleared. Even after this optimalization it took over 8 hours to find the solution.
- After tweaking the hashing algorithm so that the microchip- and generator pairs are interchangeable, it ran much, much quicker. It finds the answer to part B in seconds!
