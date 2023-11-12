# Day 12

Run

```console
cat input.txt | go run main.go
```

## Issues

- I had a bug with the instruction counter with jumps. It would increase IP before doing the jump, this caused to loop until the integer rolled over, ending in the wrong answer.
