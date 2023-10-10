use anyhow::anyhow;

pub struct Board {
    grid: Vec<Vec<char>>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let row = vec!['_'; size];

        Board {
            grid: vec![row; size],
        }
    }

    pub fn print_board(&self) -> () {
        self.grid.iter().for_each(|row| println!("{:?}", row));
    }

    pub fn is_position_empty(&self, position: [usize; 2]) -> bool {
        match self.get_mark(position) {
            Ok('_') => true,
            _ => false,
        }
    }

    pub fn get_mark(&self, position: [usize; 2]) -> anyhow::Result<char> {
        let [row_position, column_position] = position;

        let row = self
            .grid
            .get(row_position)
            .ok_or(anyhow!("Selected position is out of bounds"))?;
        let mark = row
            .get(column_position)
            .ok_or(anyhow!("Selected position is out of bounds"))?;

        Ok(*mark)
    }

    pub fn is_position_valid(&self, position: [usize; 2]) -> bool {
        match self.get_mark(position) {
            Ok(_) => true,
            _ => false,
        }
    }

    pub fn place_mark(&mut self, mark: char, position: [usize; 2]) -> anyhow::Result<()> {
        let [row_position, column_position] = position;

        if !self.is_position_valid(position) {
            return Err(anyhow!("The position is outside the board"));
        }

        if !self.is_position_empty(position) {
            return Err(anyhow!("Position is already taken."));
        }

        self.grid[row_position][column_position] = mark;
        Ok(())
    }

    pub fn win_row(&self, mark: char) -> bool {
        self.grid.iter().any(|row| row.iter().all(|m| m == &mark))
    }

    pub fn win_col(&self, mark: char) -> bool {
        for i in 0..self.grid.len() {
            if self.grid.iter().all(|row| row[i] == mark) {
                return true;
            }
        }
        false
    }

    pub fn win_diagonal(&self, mark: char) -> bool {
        let first_diagonal = (0..self.grid.len()).all(|i| self.grid[i][i] == mark);
        let second_diagonal =
            (0..self.grid.len()).all(|i| self.grid[i][self.grid.len() - 1 - i] == mark);

        first_diagonal || second_diagonal
    }

    pub fn has_won(&self, mark: char) -> bool {
        match (
            self.win_row(mark),
            self.win_col(mark),
            self.win_diagonal(mark),
        ) {
            (true, _, _) | (_, true, _) | (_, _, true) => true,
            _ => false,
        }
    }

    pub fn get_has_empty_positions(&self) -> bool {
        self.grid.concat().iter().any(|position| position == &'_')
    }
}
