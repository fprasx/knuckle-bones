mod lib;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_board() {
        let mut board = lib::Board::new();
        board.make_move(lib::Roll::One, 0);
        board.make_move(lib::Roll::One, 0);
        board.make_move(lib::Roll::One, 0);
        println!("{:?}", board);

        assert_eq!(board.score, 9);

        let mut board = lib::Board::new();
        board.make_move(lib::Roll::One, 0);
        board.make_move(lib::Roll::One, 0);
        board.make_move(lib::Roll::Two, 0);
        board.make_move(lib::Roll::Two, 1);
        board.make_move(lib::Roll::Two, 1);
        println!("{:?}", board);

        assert_eq!(board.score, 14);

    } 
}
