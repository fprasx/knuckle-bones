mod lib;
use lib::{Board, Roll};

struct Game {
    b1: Board,
    b2: Board,
    turn: usize,
    heuristic: Option<Fn(&Game) -> usize>,
}


impl Game {
    fn new(b1: Board, b2: Board) {
        Game(b1, b2, 0, None)
    }

    fn make_move(&mut self, roll: Roll, col: usize) {

        let (board, otherboard): &mut Board;
        if self.turn % 2 == 0 {
            board = &mut self.b1;
            otherboard = &mut self.b2;
        }
        else {
            board = &mut self.b2;
            otherboard = &mut self.b1;
        }

        let column = &mut board[col];
        let othercolumn = &mut otherboard[col];

        //make sure move is legal
        if column[2].is_some() { panic!("Tried to add roll to filled column!") }

        //get index to place roll
        let index = column.iter().position(|opt| opt.is_none()).unwrap();

        // update column
        column[index] = Some(roll);

        //remove dice of same number in opposite column
        for i in [0..3] {
            match othercolumn[i] {
                None => break,
                Some(roll) => othercolumn[i] == None,
            }
        }

        //settle opposite dice
        let open_index: usize = 0;
        for i in [0..3] {
            match othercolumn[i] {
                None => continue,
                r @ Some => {
                    othercolumn[open_index] = r;
                    open_index += 1;
                }
            }
        }

        // update score
        self.b1.update_score();
        self.b2.update_score();
    }

    fn get_best_move(&self) {
        //
    }


}




