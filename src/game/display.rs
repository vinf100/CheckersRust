use crate::game::board::Board;
use crate::game::board::CellType;
use crate::game::board::Colour;
use colored::Colorize;

pub fn text_display (board: Board){
    for i in 0..8{
        for j in 0..8 {
            let tile = board.get_cell(j, i);
            let text = match tile {
                CellType::REGULAR(_) => "X",
                CellType::KING(_) => "K",
                CellType::EMPTY => " "
            };
            match tile {
                CellType::REGULAR(c) | CellType::KING(c) => {
                    match c {
                        Colour::WHITE => text.white(),
                        Colour::BLACK => text.black(),
                    };
                },
                _ => {},
            };
            
            print!("{}", if (i + j) % 2 == 0 {
                text.on_bright_white()
            } else {
                text.on_bright_black()
            });
            
        }
        print!("\n");
    }
}
