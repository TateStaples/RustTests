mod tic_tac_toe;
mod connect4;
mod minesweeper;
mod leetcode;
mod snake;
mod wordle;

use std::mem;

// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn main() {

    leetcode::main()
}

