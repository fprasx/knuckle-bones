use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::slice::Iter;

pub fn roll() -> Roll {
    match fastrand::usize(1..7) {
        1 => Roll::One,
        2 => Roll::Two,
        3 => Roll::Three,
        4 => Roll::Four,
        5 => Roll::Five,
        6 => Roll::Six,
        _ => unreachable!(),
    }
}

#[derive(Debug)]
pub enum Roll {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Roll {
    fn to_usize(&self) -> usize {
        match self {
            Roll::One => 1,
            Roll::Two => 2,
            Roll::Three => 3,
            Roll::Four => 4,
            Roll::Five => 5,
            Roll::Six => 6,
        }
    }
}

pub type Column = [Option<Roll>; 3];

#[derive(Debug)]
pub struct Board {
    pub board: [Column; 3],
    pub score: usize,
}

impl Index<usize> for Board {
    type Output = Column;
    fn index(&self, index: usize) -> &Self::Output {
        &(self.board)[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut (self.board)[index]
    }
}

impl Board {
    pub fn new() -> Self { Board { board: Default::default(), score: 0 } }
    
    pub fn iter(&self) -> Iter<Column> { self.board.iter() }

    pub fn make_move(&mut self, roll: Roll, col: usize) {
        let column = &mut self[col];
        let index = column.iter().position(|opt| opt.is_none()).unwrap(); //unsafe bad? TODO?

        // update column
        column[index] = Some(roll);

        // update score
        println!("updating score");
        self.update_score();
    }

    fn update_score(&mut self) {
        let mut score = 0;
        for col in self.board.iter() {
            let col_int: Vec<usize> = col.iter().map(|roll: &Option<Roll>| if let Some(r) = roll { r.to_usize() } else { 0 }).collect::<Vec<usize>>();

            for (i, n) in col_int.iter().enumerate() {
                if *n == 0 { break }

                let count = col_int[0..i].iter().filter(|&x| *x == *n).count();
                score += (1 + count * 2) * n
            }   
        }
        self.score = score
    }

}
