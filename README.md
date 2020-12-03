# faern's Advent of Code 2020!!

Previous years I have been trying to be "generic" and overengineering.
I have tried creating frameworks so you can run the entire AoC with something
like `./aoc --day 5 --input ./input5`. That was extremely messy due to how
very different the input and outputs were supposed to be on different days etc.

This year I'm focusing on simplicity. Every day and sub-problem is a separate binary.
Copy pasting of code galore and error handling = panics. Simplicity and focusing
on just getting the result.

Run with:
```
cargo run --release --bin dayX_Y input/dayX
```
