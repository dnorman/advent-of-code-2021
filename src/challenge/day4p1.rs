pub fn run() {
    let (input, mut _aoc) = super::get(4);
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";

    for line in input.lines() {
        println!("{}", line)
    }
}

#[derive(Clone, Debug)]
struct Square {
    number: u8,
    marked: bool,
}
struct Board {
    squares: [Square; 25],
}

impl Board {
    fn new<T>(squares: [T; 25]) -> Self
    where
        T: Into<Square>,
    {
        let squares = squares.map(|s| s.into());
        Board { squares }
    }
    fn is_winning(&self) -> bool {
        // rows
        self.squares[..]
            .chunks(5)
            .any(|row| row.iter().all(|sq| sq.marked))

        // columns
        || (0..5).any(|col| (0..5).all(|row| self.squares[(5 * row) + col].marked))

        // diagonals
        || [0, 6, 12, 18, 24].iter().all(|i| self.squares[*i].marked)
        || [4, 8, 12, 16, 20].iter().all(|i| self.squares[*i].marked)
    }
}

#[cfg(test)]
mod test {
    impl Into<Square> for u8 {
        fn into(self) -> Square {
            Square {
                number: 0,
                marked: self != 0,
            }
        }
    }

    use super::{Board, Square};
    #[test]
    fn test_boards() {
        let board = Board::new([
            1, 1, 1, 1, 0, //
            1, 1, 1, 0, 1, //
            1, 1, 0, 1, 1, //
            1, 0, 1, 1, 1, //
            0, 1, 1, 1, 1, //
        ]);
        assert_eq!(board.is_winning(), false);

        let board = Board::new([
            1, 1, 1, 1, 1, //
            0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, //
        ]);
        assert_eq!(board.is_winning(), true);

        let board = Board::new([
            0, 0, 0, 0, 0, //
            1, 1, 1, 1, 1, //
            0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, //
        ]);
        assert_eq!(board.is_winning(), true);

        let board = Board::new([
            1, 0, 0, 0, 0, //
            1, 0, 0, 0, 0, //
            1, 0, 0, 0, 0, //
            1, 0, 0, 0, 0, //
            1, 0, 0, 0, 0, //
        ]);
        assert_eq!(board.is_winning(), true);

        let board = Board::new([
            0, 1, 0, 0, 0, //
            0, 1, 0, 0, 0, //
            0, 1, 0, 0, 0, //
            0, 1, 0, 0, 0, //
            0, 1, 0, 0, 0, //
        ]);
        assert_eq!(board.is_winning(), true);

        let board = Board::new([
            1, 0, 0, 0, 0, //
            0, 1, 0, 0, 0, //
            0, 0, 1, 0, 0, //
            0, 0, 0, 1, 0, //
            0, 0, 0, 0, 1, //
        ]);
        assert_eq!(board.is_winning(), true);
        let board = Board::new([
            0, 0, 0, 0, 1, //
            0, 0, 0, 1, 0, //
            0, 0, 1, 0, 0, //
            0, 1, 0, 0, 0, //
            1, 0, 0, 0, 0, //
        ]);
        assert_eq!(board.is_winning(), true);
    }
}