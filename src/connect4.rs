use std::io;
use std::fmt::format;
use std::ops::Index;
use crate::tic_tac_toe::user_input;

pub(crate) fn main() {
    let mut g = Game::new();
    while(!g.over()) {
        human_play(&mut g);
        dumb_ai(&mut g);
        g.print();
    }
    g.print();
    println!("Final score: {}", g.score())
}

fn get_play() -> u8 {
    let mut input: String;
    loop {
        input = user_input(String::from("Pick a drop location (0-6): "));
        let answer = input.parse::<u8>();
        if !answer.is_err() {
            let val: u8 = answer.unwrap();
            if val < 7 {return val;}
            else {println!("Index outside of board size");}
        }
        else {println!("Invalid input. Should be 0-9");}
    }
}

fn human_play(game: &mut Game) {
    let available = game.available();
    loop {
        let choice = get_play() as usize;
        if available.contains(&choice) {
            game.drop( true, choice);
            return;
        }
        else {
            println!("The selected row, {}, is already full. Try again", choice)
        }
    }
}
fn dumb_ai(game: &mut Game) {
    let available = game.available();
    let choice = available.first().unwrap();
    game.drop(false, *choice);
}
fn smart_ai_play(mut game: Game) -> Game {
    let best_move = min_max(game.clone(), false, 0, 0);
    game.drop(false, best_move as usize);
    game
}


fn min_max(game: Game, maxing: bool, depth: u8, intial_depth: u8) -> i8 {
    if game.over() { return game.score(); } // todo: add heuristic
    let mut best_move = 0;
    let mut best_score = if maxing {-100} else {100};  // fixme
    for choice in game.available() {
        // eprintln!("move found");
        let mut copy = game.clone();  // make a copy for this new branch
        copy.drop(maxing, choice);
        let score = min_max(copy, !maxing, depth+1, intial_depth);
        if depth == intial_depth {println!("Score of {} for going @ {}", score, choice)}
        if (maxing && score > best_score) || (!maxing && score < best_score) {
            best_score = score;
            best_move = choice;
        }
    }
    if depth == intial_depth {  // the issue is here
        println!("The best move is {} with a score of {}", best_move, best_score);
        best_move as i8
    } else {best_score}  // at the top of the recursion return move instead of score
}

struct Column {
    fill: [i8; 6]
}
impl Column {
    fn new() -> Column { Column{ fill: [0; 6]} }
    fn increment(&mut self, player: bool) { self.fill[self.size()] = if player { 1 } else { -1 }; }
    fn full(&self) -> bool {self.fill[5] != 0}
    fn size(&self) -> usize { self.fill.iter().position(|&x| x == 0).unwrap_or(5) }
    fn get(&self, index: usize) -> i8 { self.fill[index] }
    fn safe_get(&self, index:usize) -> i8 { if index < 6 {self.get(index)} else {0}}
    fn clone(&self) -> Column { Column { fill: self.fill.clone() } }
}

struct Game {
    cols: [Column; 7]
} impl Game {
    fn new() -> Game {
        let list = [Column::new(), Column::new(), Column::new(), Column::new(), Column::new(), Column::new(), Column::new()];
        // let list = arr![Column::new(); 7];
        // list.fill_with(Column::new);
        Game {cols: list}
    }
    fn drop(&mut self, player: bool, column: usize) -> bool {
        let mut col = &mut self.cols[column];
        if col.full() {return false;}
        col.increment(player);
        return true;
    }
    fn score(&self) -> i8 {
        for col in &self.cols {
            // check vert - kinda opt method
            if col.get(3) != 0 {
                let center = col.get(3);

                // count down
                let mut up:u8 = 0;
                for i in 1..3 {
                    if col.get((3 + i) as usize) == center { up += 1 }
                    else {continue}
                }
                // count up
                let mut down:u8 = 0;
                for i in 1..4 {
                    if col.get((3 - i) as usize) == center {down += 1}
                    else {continue}
                }
                // sum and check
                if (1 + down + up) >= 4 {
                    eprintln!("Team {} one by vert in col #{}", center, &self.cols.iter().position(|x| x.fill == col.fill).unwrap());
                    eprintln!("down: {}, up: {}", down, up);
                    return center;
                }
            }
        }
        // check ho and diagonal
        let center_col = self.cols.get(3).unwrap();
        let patterns: [(i8, i8); 3] = [(1, 0), (1, 1), (1, -1)];
        let mut row_index: i8 = 0;
        for slot in center_col.fill {
            if slot == 0 {return 0}
            for pattern in patterns {
                let (dx, dy) = pattern;
                // count down
                let mut down = 0;
                for i in 1..4 {
                    let col: &Column = &self.cols.get((3 + dx * i) as usize).unwrap();
                    if col.safe_get((row_index + i * dy) as usize) == slot {down += 1}
                    else {continue}
                }
                // count up
                let mut up = 0;
                for i in 1..4 {
                    let col: &Column = &self.cols.get((3 + dx * -i) as usize).unwrap();
                    if col.safe_get((row_index + -i * dy) as usize) == slot {up += 1}
                    else {continue}
                }
                println!("Team {} one by ({}, {}) in center row #{}", slot, dx, dy, row_index);
                if (1 + down + up) >= 4 { return slot; }
            }
        }
        0
    }
    fn over(&self) -> bool { self.score() != 0 || self.available().is_empty() }
    fn full(&self, col: usize) -> bool { self.cols[col].full() }
    fn available(&self) -> Vec<usize> {
        let default_actions: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];
        default_actions
            .iter()
            .filter(|&&x| !self.full(x))
            .map(|&x| x)
            .collect()
    }

    fn print(&self) {
        for row in 0..6 {
            let mut row_string = String::from("|");
            for col in &self.cols {
                let value = col.get(5 - row);
                let sym = if value == 0 {' '} else if value == 1 {'X'} else {'O'};
                row_string += &*format!("\t{sym}\t|");
            }
            println!("{}", row_string);
        }
        println!();
        println!();
        println!();
        println!();
    }
    fn clone(&self) -> Game {
        // Game { cols: self.cols.map(|x| x.clone()) }
        Game::new() // fixme
    }
}