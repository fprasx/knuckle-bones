use std::ops::{Deref, Index, IndexMut};

fn roll() -> Roll {
    match fastrand::usize(1..7) {
        1 => Roll::One,
        2 => Roll::Two,
        3 => Roll::Three,
        4 => Roll::Four,
        5 => Roll::Five,
        6 => Roll::Six,
        _ => unreachable!()
    }
}

enum Roll {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}
struct Board([[Option<Roll>; 3]; 3]);

impl Deref for Board {
    type Target = [[Option<Roll>; 3]; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Index<usize> for Board {
    type Output = [Option<Roll>; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &(self.0)[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut (self.0)[index]
    }
}

impl Board {
    fn new() -> Self {
        Board([[None, None, None], [None, None, None], [None, None, None]])
    }

    fn score(&self) -> usize {
        todo!()
    }

}
