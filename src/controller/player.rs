use crate::controller::Controller;
use crate::game::board::*;
use text_io;
struct PlayerController{
    colour: COLOUR;
}

impl Controller for PlayerController {
    fn make_move(board: &Board){
        let display = board.get_display_mode();
        print!("Select a piece to move: ");
        get_tile(board, display);
        match CellType(c) {
            
        }
    }
    fn get_tile(board: Board, display: DisplayMode) -> CellType{
        match display {
            TEXT => {
                let row: i8 = read!();
                let column: i8 = read!();
                return board.get_tile(row, column);

            }
        }
    }
}
