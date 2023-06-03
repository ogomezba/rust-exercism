use std::collections::HashMap;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }

    let board = Matrix::new(minefield);
    let mut annotated_board = Vec::with_capacity(minefield.len());

    board.rows_iter().enumerate().for_each(|(i, row)| {
        let mut row_str = String::new();

        row.enumerate().for_each(|(j, bs)| {
            println!("i: {i}, j: {j}");
            row_str.push(match bs {
                BoardSquare::Bomb => '*',
                BoardSquare::NoBomb => {
                    match char::from_digit(
                        board
                            .adjacents((i, j))
                            .filter(|bs| **bs == BoardSquare::Bomb)
                            .count() as u32,
                        10,
                    )
                    .unwrap()
                    {
                        '0' => ' ',
                        n => n,
                    }
                }
            });
        });

        annotated_board.push(row_str);
    });

    return annotated_board;
}

#[derive(Debug)]
struct Matrix {
    elements: HashMap<usize, HashMap<usize, BoardSquare>>,
}

#[derive(Debug, PartialEq)]
enum BoardSquare {
    Bomb,
    NoBomb,
}

impl BoardSquare {
    fn from_string(c: char) -> BoardSquare {
        match c {
            '*' => BoardSquare::Bomb,
            _ => BoardSquare::NoBomb,
        }
    }
}

impl Matrix {
    fn new(rows: &[&str]) -> Matrix {
        let mut elements: HashMap<usize, HashMap<usize, BoardSquare>> = HashMap::new();

        rows.iter().enumerate().for_each(|(row_idx, row)| {
            let mut board_row = HashMap::new();
            row.chars()
                .map(BoardSquare::from_string)
                .enumerate()
                .for_each(|(col_idx, bs)| {
                    board_row.insert(col_idx, bs);
                });

            elements.insert(row_idx, board_row);
        });

        Matrix { elements }
    }

    fn get(&self, (i, j): (usize, usize)) -> &BoardSquare {
        self.elements.get(&i).and_then(|row| row.get(&j)).unwrap()
    }

    fn rows(&self) -> usize {
        self.elements.len()
    }

    fn cols(&self) -> usize {
        self.elements.get(&0).map(|row| row.len()).unwrap()
    }

    fn rows_iter(&self) -> RowIter {
        RowIter { matrix: self, i: 0 }
    }

    fn adjacents(&self, element: (usize, usize)) -> AdjacentElementsIter {
        AdjacentElementsIter {
            matrix: self,
            element,
            current_adjacent: Direction::TL,
        }
    }
}

struct RowIter<'a> {
    matrix: &'a Matrix,
    i: usize,
}

struct ColumnIter<'a> {
    matrix: &'a Matrix,
    i: usize,
    j: usize,
}

struct AdjacentElementsIter<'a> {
    matrix: &'a Matrix,
    element: (usize, usize),
    current_adjacent: Direction,
}

impl<'a> Iterator for AdjacentElementsIter<'a> {
    type Item = &'a BoardSquare;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_adjacent {
            Direction::TL => {
                if self.element.0 == 0 {
                    self.current_adjacent = Direction::R;
                    return self.next();
                } else if self.element.1 == 0 {
                    self.current_adjacent = Direction::T;
                    return self.next();
                } else {
                    self.current_adjacent = Direction::T;
                    return Some(self.matrix.get((self.element.0 - 1, self.element.1 - 1)));
                }
            }
            Direction::T => {
                self.current_adjacent = Direction::TR;
                return Some(self.matrix.get((self.element.0 - 1, self.element.1)));
            }
            Direction::TR => {
                if self.element.1 == self.matrix.cols() - 1 {
                    self.current_adjacent = Direction::B;
                    return self.next();
                } else {
                    self.current_adjacent = Direction::R;
                    return Some(self.matrix.get((self.element.0 - 1, self.element.1 + 1)));
                }
            }

            Direction::R => {
                if self.element.1 == self.matrix.cols() - 1 {
                    self.current_adjacent = Direction::B;
                    return self.next();
                } else {
                    self.current_adjacent = Direction::BR;
                    return Some(self.matrix.get((self.element.0, self.element.1 + 1)));
                }
            }

            Direction::BR => {
                if self.element.0 == self.matrix.rows() - 1 {
                    self.current_adjacent = Direction::L;
                    return self.next();
                } else {
                    self.current_adjacent = Direction::B;
                    return Some(self.matrix.get((self.element.0 + 1, self.element.1 + 1)));
                }
            }

            Direction::B => {
                if self.element.0 == self.matrix.rows() - 1 {
                    self.current_adjacent = Direction::L;
                    return self.next();
                } else {
                    self.current_adjacent = Direction::BL;
                    return Some(self.matrix.get((self.element.0 + 1, self.element.1)));
                }
            }

            Direction::BL => {
                if self.element.1 == 0 {
                    self.current_adjacent = Direction::END;
                    return self.next();
                } else {
                    self.current_adjacent = Direction::L;
                    return Some(self.matrix.get((self.element.0 + 1, self.element.1 - 1)));
                }
            }

            Direction::L => {
                self.current_adjacent = Direction::END;
                if self.element.1 == 0 {
                    return self.next();
                } else {
                    return Some(self.matrix.get((self.element.0, self.element.1 - 1)));
                }
            }

            Direction::END => return None,
        }
    }
}

impl<'a> Iterator for RowIter<'a> {
    type Item = ColumnIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.i {
            i if i == self.matrix.rows() => None,
            i => {
                self.i += 1;
                Some(ColumnIter {
                    matrix: self.matrix,
                    j: 0,
                    i,
                })
            }
        }
    }
}

impl<'a> Iterator for ColumnIter<'a> {
    type Item = &'a BoardSquare;

    fn next(&mut self) -> Option<Self::Item> {
        match self.j {
            j if j == self.matrix.cols() => None,
            j => {
                self.j += 1;
                Some(self.matrix.get((self.i, j)))
            }
        }
    }
}

enum Direction {
    TL,
    T,
    TR,
    R,
    BR,
    B,
    BL,
    L,
    END,
}
