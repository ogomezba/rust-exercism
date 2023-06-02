use std::collections::HashMap;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let board = Matrix::new(minefield);

    Vec::new()
}

#[derive(Debug)]
struct Matrix {
    elements: HashMap<usize, HashMap<usize, BoardSquare>>,
}

#[derive(Debug)]
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
        let mut elements = HashMap::new();

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

    fn get(&self, i: &usize, j: &usize) -> &BoardSquare {
        self.elements.get(i).and_then(|row| row.get(j)).unwrap()
    }
}
