use rand::Rng;
use crate::cell::{Cell, create_cell};
use crate::{GRID_HEIGHT, GRID_WIDTH};

#[derive(Debug)]
pub struct Board {
    pub(crate) cells: Vec<Cell>,
}

impl Board {
    pub fn init_board() -> Board {
        let mut board = Board {
            cells: Vec::with_capacity((GRID_WIDTH * GRID_HEIGHT) as usize)
        };

        let mut rng = rand::thread_rng();

        for e in 0..(GRID_WIDTH * GRID_HEIGHT) {
            let random_nbr: i32 = rng.gen();

            if (random_nbr % 3) == 2 {
                board.cells.insert(e as usize, create_cell(true));
            } else {
                board.cells.insert(e as usize, create_cell(false));
            }
        }
        board
    }

    pub fn check_status(&self, x: i32, y: i32) -> bool {
        return if x < 0 || x >= GRID_WIDTH || y < 0 || y >= GRID_HEIGHT {
            false
        } else {
            self.cells[((y * GRID_WIDTH) + x) as usize].alive
        };
    }

    pub fn set_next_status(&mut self, x: i32, y: i32, status: bool) {
        self.cells[((y * GRID_WIDTH) + x) as usize].next_round = status;
    }

    fn alive_neighbours(&self, x: i32, y: i32) -> i32 {
        let mut count: i32 = 0;

        if self.check_status(x - 1, y - 1) {
            count += 1;
        }

        if self.check_status(x + 0, y - 1) {
            count += 1;
        }

        if self.check_status(x + 1, y - 1) {
            count += 1;
        }

        if self.check_status(x - 1, y + 0) {
            count += 1;
        }

        if self.check_status(x + 1, y + 0) {
            count += 1;
        }

        if self.check_status(x - 1, y + 1) {
            count += 1;
        }

        if self.check_status(x + 0, y + 1) {
            count += 1;
        }

        if self.check_status(x + 1, y + 1) {
            count += 1;
        }

        count
    }

    pub fn play_round(&mut self) {
        // go through the rules to see what happens to each cell
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                // rules

                let neighbours: i32 = self.alive_neighbours(x, y);
                let status: bool = self.check_status(x, y);

                if status {
                    // if the cell is alive
                    if neighbours < 2 || neighbours > 3 {
                        self.set_next_status(x, y, false);
                    } else if neighbours == 2 || neighbours == 3 {
                        self.set_next_status(x, y, true);
                    }
                } else {
                    // the cell is dead
                    if neighbours == 3 {
                        self.set_next_status(x, y, true);
                    }
                }
            }
        }

        // apply the outcome
        for i in 0..self.cells.len() {
            self.cells[i].alive = self.cells[i].next_round;
        }
    }
}
