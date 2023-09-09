#[cfg(feature = "buddy-alloc")]
mod alloc;
mod game;
mod palette;
mod snake;
mod wasm4;

use game::Game;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref SNAKE_GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[no_mangle]
fn start() {
    palette::set_palette([0xfbf7f3, 0xe5b083, 0x426e5d, 0x20283d])
}

#[no_mangle]
fn update() {
    SNAKE_GAME.lock().expect("game_state").update();
}
