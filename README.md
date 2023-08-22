# Sudoku Solver
This repository hosts a [Rust](https://www.rust-lang.org) rendition of [Peter Norvig's Sudoku solver](https://norvig.com/sudoku.html), implemented using a DFS search and constraint propagation to reduce the search space.

## Run it
Feed to `stdin` text input defining one Sudoku per line. For each Sudoku, `[1..9]` represents a filled in cell, and `[.0]` an empty cell. Any other characters are ignored. For example:

```bash
cat data/top95.txt | cargo run
cat data/hardest.txt | cargo run
```
