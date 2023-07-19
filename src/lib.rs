pub mod game {
    pub mod board;
    pub mod display;
}

mod controller{
    pub trait Controller {
        fn make_move(board: &crate::game::board::Board);
    }
    mod player;
    mod min_max_ai;
}
