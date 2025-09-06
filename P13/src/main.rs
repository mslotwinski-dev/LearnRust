mod game;

use crate::game::Game;

const CHAOS: bool = false;

fn main() {
    Game::new(70, 20, 0.25, CHAOS);
}
