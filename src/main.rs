use anyhow::anyhow;

struct Board {
    grid: Vec<Vec<char>>,
}

impl Board {
    fn new() -> Self {
        Board {
            grid: vec![
                vec!['_', '_', '_'],
                vec!['_', '_', '_'],
                vec!['_', '_', '_'],
            ],
        }
    }

    fn print_board(&self) -> () {
        self.grid.iter().for_each(|row| println!("{:?}", row));
    }

    fn is_position_empty(&self, position: [usize; 2]) -> bool {
        match self.get_mark(position) {
            Ok('_') => true,
            _ => false,
        }
    }

    fn get_mark(&self, position: [usize; 2]) -> anyhow::Result<char> {
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

    fn is_position_valid(&self, position: [usize; 2]) -> bool {
        match self.get_mark(position) {
            Ok(_) => true,
            _ => false,
        }
    }

    fn place_mark(&mut self, mark: char, position: [usize; 2]) -> anyhow::Result<()> {
        let [row_position, column_position] = position;

        if self.is_position_valid(position) == false {
            return Err(anyhow!("The position is outside the board"));
        }

        if self.is_position_empty(position) == false {
            return Err(anyhow!("Position is already taken."));
        }

        self.grid[row_position][column_position] = mark;
        Ok(())
    }

    fn win_row(&self, mark: char) -> bool {
        let mut has_won = false;

        self.grid.iter().for_each(|row| {
            if row.iter().all(|m| m == &mark) {
                has_won = true
            }
        });

        has_won
    }

    fn win_col(&self, mark: char) -> bool {
        let mut has_won = false;

        for i in 0..=2 {
            let c1 = self.grid[0][i];
            let c2 = self.grid[1][i];
            let c3 = self.grid[2][i];
            if c1 == mark && c2 == mark && c3 == mark {
                has_won = true;
            }
        }

        has_won
    }
}

fn main() {
    let mut board = Board::new();
    board.print_board();

    match board.place_mark('0', [0, 0]) {
        Ok(value) => println!("Result: {:?}", value),
        Err(error) => eprintln!("Error: {:?}", error),
    }
    match board.place_mark('0', [1, 0]) {
        Ok(value) => println!("Result: {:?}", value),
        Err(error) => eprintln!("Error: {:?}", error),
    }
    match board.place_mark('0', [2, 0]) {
        Ok(value) => println!("Result: {:?}", value),
        Err(error) => eprintln!("Error: {:?}", error),
    }

    board.print_board();
    println!("{:?}", board.win_col('0'));

    // let mark = board.get_mark([0, 1]);
    // let is_empty = board.is_position_empty([0, 1]);
}
