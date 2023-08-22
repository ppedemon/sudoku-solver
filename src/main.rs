use bitvector::BitVector;
use core::fmt;
use std::collections::HashSet;
use std::io;

struct SudokuBoard {
    units: Vec<Vec<usize>>,
    units_for: Vec<Vec<usize>>,
    neighbors: Vec<HashSet<usize>>,
}

impl SudokuBoard {
    fn new() -> SudokuBoard {
        let mut units: Vec<Vec<usize>> = vec![Vec::new(); 27];
        let mut units_for: Vec<Vec<usize>> = vec![Vec::new(); 81];
        let mut neighbors: Vec<HashSet<usize>> = vec![HashSet::new(); 81];

        for i in 0..9 {
            for j in 0..9 {
                let p = i * 9 + j;
                let x = [i, 9 + j, 18 + (i / 3) * 3 + j / 3];
                for k in 0..3 {
                    units[x[k]].push(p);
                    units_for[p].push(x[k]);
                }
            }
        }
        for k in 0..neighbors.len() {
            for i in 0..units_for[k].len() {
                for j in 0..9 {
                    let u: usize = units_for[k][i];
                    let v = units[u][j];
                    if v != k {
                        neighbors[k].insert(v);
                    }
                }
            }
        }

        SudokuBoard {
            units,
            units_for,
            neighbors,
        }
    }
}

#[derive(Clone)]
struct Sudoku<'a> {
    board: &'a SudokuBoard,
    cells: Vec<BitVector>,
}

impl<'a> Sudoku<'a> {
    fn new(board: &'a SudokuBoard) -> Sudoku<'a> {
        let mut cells: Vec<BitVector> = vec![BitVector::new(10); 81];
        cells.iter_mut().for_each(|c| {
            for i in 1..=9 {
                c.insert(i);
            }
        });
        Sudoku { board, cells }
    }

    fn from(s: &str, board: &'a SudokuBoard) -> Sudoku<'a> {
        let mut sudoku = Sudoku::new(board);
        for (k, c) in s.chars().filter(|c| (*c >= '1' && *c <= '9') || *c == '0' || *c == '.').enumerate() {
            if c >= '1' && c <= '9' {
                let v: usize = c.to_digit(10).unwrap() as usize;
                if !sudoku.assign(k, v) {
                    panic!("Invalid Sudoku")
                }
            }
        }
        sudoku
    }

    fn assign(&mut self, k: usize, v: usize) -> bool {
        (1..=9).into_iter().filter(|i| *i != v).all(|i| self.eliminate(k, i))
    }

    fn eliminate(&mut self, k: usize, v: usize) -> bool {
        if !self.cells[k].contains(v) {
            true
        } else {
            self.cells[k].remove(v);
            match self.cells[k].len() {
                0 => false,
                1 => {
                    let val = self.uniq_val(k);
                    self.board.neighbors[k].iter().all(|n| self.eliminate(*n, val))    
                }
                _ =>
                    self.board.units_for[k]
                        .iter()
                        .map(|i| &self.board.units[*i])
                        .all(|u| {
                            let ps: Vec<&usize> = u.iter().filter(|p| self.cells[**p].contains(v)).collect();
                            match ps.len() {
                                0 => false,
                                1 => self.assign(*ps[0], v),
                                _ => true
                            }
                        })
            }
        }
    }

    #[inline]
    fn uniq_val(&self, k: usize) -> usize {
        assert_eq!(self.cells[k].len(), 1);
        self.cells[k].iter().nth(0).unwrap()
    }

    #[inline]
    fn is_solved(&self) -> bool {
        self.cells.iter().all(|k| k.len() == 1)
    }

    #[inline]
    fn smaller_cell(&self) -> usize {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, k)| k.len() > 1)
            .min_by_key(|(_, k)| k.len())
            .map(|(i, _)| i)
            .unwrap()
    }

    fn solve(&self) -> Option<Sudoku<'a>> {
        if self.is_solved() {
            Some(self.clone())
        } else {
            let k = self.smaller_cell();
            self.cells[k]
                .iter()
                .fold(None, |acc, v| match acc {
                    Some(_) => acc,
                    None => {
                        let mut s = self.clone();
                        s.assign(k, v).then(|| s.solve()).flatten()
                    }
                })
        }
    }
}

impl<'a> fmt::Display for Sudoku<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn bitvector_to_str(b: &BitVector) -> String {
            let n = b.iter().fold(0, |acc, n| acc*10 + n);
            n.to_string()
        }
    
        let width = self.cells.iter().map(|c| c.len() + 1).max().unwrap();
        let sep = "-".repeat(3 * width);
        for i in 0..9 {
            if i == 3 || i == 6 {
                writeln!(f, "{}+-{}+-{}", sep, sep, sep)?;
            }
            for j in 0..9 {
                if j == 3 || j == 6 {
                    write!(f, "| ")?;
                }
                write!(f, "{:width$}", bitvector_to_str(&self.cells[i*9 + j]), width = width)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "")
    }
} 

fn main() {
    let board = SudokuBoard::new();
    let lines = io::stdin().lines();
    for line in lines {
        let s = line.unwrap();
        let sudoku = Sudoku::from(&s, &board);
        let solved = sudoku.solve();
        println!("{}", solved.expect("No solution"));
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::SudokuBoard;

    #[test]
    fn init_works() {
        let s = SudokuBoard::new();
        assert_eq!(s.units.len(), 27);
        assert!((0..81).map(|i| s.units_for[i].len()).all(|n| n == 3));
        assert!((0..81).map(|i| s.neighbors[i].len()).all(|n| n == 20));
        assert_eq!(s.units_for[19], vec![2, 10, 18]);
        assert_eq!(s.units[18], vec![0, 1, 2, 9, 10, 11, 18, 19, 20]);
        assert_eq!(
            s.neighbors[19],
            HashSet::from_iter(vec![
                18, 20, 21, 22, 23, 24, 25, 26, 01, 10, 28, 37, 46, 55, 64, 73, 00, 02, 09, 11,
            ])
        );
    }
}
