use std::ops::{Index, IndexMut};
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
    // The idiomatic way to do type conversions is with the `from` trait
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

// Now we can call roll.into(), and if type inference deems
// that we want a usize (or if we say so explicitly), the compiler
// will call this method to convert
impl From<Roll> for usize {
    fn from(roll: Roll) -> Self {
        match roll {
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
    pub fn new() -> Self {
        Board {
            board: Default::default(),
            score: 0,
        }
    }

    pub fn iter(&self) -> Iter<Column> {
        self.board.iter()
    }

    // Consider making return type a `Result`
    pub fn make_move(&mut self, roll: Roll, col: usize) {
        let column = &mut self[col];
        //unsafe bad? TODO?
        // Yeah we should probably match on it. what do we do if you can't add to the row?
        let index = column.iter().position(|opt| opt.is_none()).unwrap();

        // update column
        column[index] = Some(roll);

        // update score
        println!("updating score");
        self.update_score();
    }

    fn update_score(&mut self) {
        let mut score = 0;
        for col in self.board.iter() {
            // The reason you have to use vec is because the iterators don't necessarily have a statically
            // known size
            let col_int: Vec<usize> = col
                .iter()
                .map(|roll: &Option<Roll>| if let Some(r) = roll { r.to_usize() } else { 0 })
                .collect();

            for (i, n) in col_int.iter().enumerate() {
                if *n == 0 {
                    break;
                }

                // You can use take() here, which take a specified number of iterator elements
                // let count = col_int[0..i].iter().filter(|&x| *x == *n).count();
                let count = col_int.iter().take(i).filter(|&x| *x == *n).count();
                score += (1 + count * 2) * n
            }
        }
        self.score = score
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
