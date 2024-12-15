mod game;
mod ui;

use game::Game;
use std::{fs, io};
use ui::Ui;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("../input.txt").expect("Failed to read input file");
    let game = Game::new(&input);
    let mut ui = Ui::new()?;
    ui.run(game)
}
