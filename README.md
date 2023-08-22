# Sudoku Solver
This repository hosts a rendition of Peter's Norbig Sudoku solver, implemented using a DFS search and contraint propagation to reduce the search space.

## Run it
Feed to `stdin` text input defining one Sudoku per line. For each Sudoku, `[1..9]` represents a filled in cell, and `[.0]` an empty cell. Any other characters are ignored. For example:

```bash
cat data/top95.txt | cargo run
cat data/hardest.txt | cargo run
```
