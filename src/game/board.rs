type Loc = (usize, usize);
pub struct Board {
    cells: [[CellType; 8]; 8],
    displayMode: DisplayMode,
}

pub enum DisplayMode {
    TEXT, GUI,
}

#[derive(Copy, Clone)]
pub enum Colour {
    BLACK, WHITE
}
#[derive(Copy, Clone)]
pub enum CellType {
    REGULAR(Colour), KING(Colour), EMPTY
}

impl Board {
    pub fn new() -> Self {
        let mut cells  = [[CellType::EMPTY; 8]; 8];
        for i in [0, 2, 5, 7]{
            for j in [0, 2, 4, 6]{
                if i > 2 {
                    cells[i][j] = CellType::REGULAR(Colour::WHITE);
                } else {
                    cells[i][j] = CellType::REGULAR(Colour::BLACK);
                }
            }
        }
        for i in [1, 6]{
            for j in [1, 3, 5, 7]{
                if i > 2 {
                    cells[i][j] =  CellType::REGULAR(Colour::WHITE);
                } else {
                    cells[i][j] =  CellType::REGULAR(Colour::BLACK);
                }
            }
        }
        Board{cells}
    }
    pub fn get_cell(&self, x: usize, y: usize) -> CellType {
        self.cells[y][x]
    }
    pub fn set_cell(&mut self, x: usize, y: usize, content: CellType){
        self.cells[y][x] = content;
    }
    pub fn make_move(&mut self, src: Loc, dst: Loc, capt: Option<Loc>){
        assert!(matches!(self.cells[dst.1][dst.0], CellType::EMPTY));
        self.cells[dst.1][dst.0] = self.cells[src.1][src.0];
        self.cells[src.1][src.0] = CellType::EMPTY;
        if let Some(location) = capt {
            self.cells[location.1][location.0] = CellType::EMPTY;
        }
    }
    pub fn get_display_mode(&self) -> DisplayMode {
        return self.displayMode;
    }
    pub fn set_display_mode(&mut self, displayMode: DisplayMode){
        self.displayMode = displayMode;
    }
}





