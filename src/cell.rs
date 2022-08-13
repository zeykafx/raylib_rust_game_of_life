#[derive(Debug)]
pub struct Cell {
    pub(crate) alive: bool,
    pub(crate) next_round: bool
}

pub fn create_cell(alive: bool) -> Cell {
    Cell{
        alive,
        next_round: false
    }
}