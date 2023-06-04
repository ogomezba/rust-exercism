use std::{char::from_digit, collections::HashMap, fmt::Debug};

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let board_without_numbers = build_matrix(minefield);
    let mut final_minefield = Vec::with_capacity(board_without_numbers.size.0);

    for col_iter in board_without_numbers.row_iter() {
        let mut row = String::with_capacity(board_without_numbers.size.1);

        for bs in col_iter {
            row.push(match bs.square_type {
                BoardType::Bomb => '*',
                BoardType::NoBomb => {
                    let adjacent_bombs = board_without_numbers
                        .adjacents((bs.i, bs.j))
                        .filter(|bs| bs.square_type == BoardType::Bomb)
                        .count();

                    match adjacent_bombs {
                        0 => ' ',
                        n => from_digit(n as u32, 10).unwrap(),
                    }
                }
            });
        }

        final_minefield.push(row);
    }

    return final_minefield;
}

fn build_matrix(rows: &[&str]) -> Matrix<BoardSquare> {
    if rows.len() == 0 {
        return Matrix::new((0, 0));
    }

    let mut board = Matrix::new((rows.len(), rows.get(0).unwrap().len()));

    for (i, row) in rows.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            board.put(
                (i, j),
                BoardSquare {
                    i,
                    j,
                    square_type: BoardType::from_char(c),
                },
            );
        }
    }

    return board;
}

struct Matrix<T> {
    size: (usize, usize),
    elements: HashMap<usize, HashMap<usize, T>>,
}

#[derive(Debug, PartialEq, Clone)]
enum BoardType {
    Bomb,
    NoBomb,
}

impl BoardType {
    fn from_char(c: char) -> Self {
        match c {
            '*' => BoardType::Bomb,
            _ => BoardType::NoBomb,
        }
    }
}

#[derive(Debug, Clone)]
struct BoardSquare {
    i: usize,
    j: usize,
    square_type: BoardType,
}

impl<T> Matrix<T> {
    fn new((i, j): (usize, usize)) -> Matrix<T> {
        if i == 0 {
            return Matrix {
                elements: HashMap::new(),
                size: (0, 0),
            };
        }

        let mut elements = HashMap::with_capacity(i);

        for idx in 0..i {
            elements.insert(idx, HashMap::with_capacity(j));
        }

        Matrix {
            elements,
            size: (i, j),
        }
    }

    fn get(&self, (i, j): (usize, usize)) -> &T {
        self.elements.get(&i).and_then(|row| row.get(&j)).unwrap()
    }

    fn rows(&self) -> usize {
        self.size.0
    }

    fn cols(&self) -> usize {
        self.size.1
    }

    fn row_iter(&self) -> RowIter<T> {
        RowIter { matrix: self, i: 0 }
    }

    fn put(&mut self, (i, j): (usize, usize), element: T) {
        self.elements.get_mut(&i).unwrap().insert(j, element);
    }

    fn adjacents(&self, element: (usize, usize)) -> AdjacentElementsIter<T> {
        AdjacentElementsIter {
            matrix: self,
            element,
            current_adjacent: Direction::TL,
        }
    }
}

struct RowIter<'a, T> {
    matrix: &'a Matrix<T>,
    i: usize,
}

struct ColumnIter<'a, T> {
    matrix: &'a Matrix<T>,
    i: usize,
    j: usize,
}

struct AdjacentElementsIter<'a, T> {
    matrix: &'a Matrix<T>,
    element: (usize, usize),
    current_adjacent: Direction,
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = ColumnIter<'a, T>;

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

impl<'a, T> Iterator for ColumnIter<'a, T> {
    type Item = &'a T;

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

impl<'a, T> Iterator for AdjacentElementsIter<'a, T> {
    type Item = &'a T;

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
