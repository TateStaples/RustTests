use std::io;
use std::ptr::hash;

pub(crate) fn main() {
    let mut board = Board::empty();
    while(!board.over()) {
        board = human_play(board);
        if board.over() {continue;}
        // board.print();
        board = smart_ai_play(board);
        board.print();
    }
    println!("Team {} won the game", board.score())
}

pub (crate) fn user_input(prompt: String) -> String {
    println!("{}", prompt);
    // stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Did not enter a correct string");
    input.trim().to_string()
}

fn get_play() -> u8 {
    let mut input: String;
    loop {
        input = user_input(String::from("Test: "));
        let answer = input.parse::<u8>();
        if !answer.is_err() {
            let val: u8 = answer.unwrap();
            if val < 9 {return val;}
            else {println!("Index outside of board size");}
        }
        else {println!("Invalid input. Should be 0-9");}
    }
}

fn human_play(mut b: Board) -> Board {
    let index = get_play();
    b.place(true, index as usize);
    b
}

fn smart_ai_play(mut b: Board) -> Board {
    let depth = b.values.iter().filter(|&x| *x == 0).count() as u8;
    // eprintln!("Depth = {}", depth);
    let best_move = min_max(b.clone(), false, depth, depth);
    // eprintln!("Ai moves to {}", best_move);
    b.place(false, best_move as usize);
    b
}

fn min_max(board: Board, maxing: bool, depth: u8, intial_depth: u8) -> i8 {
    let current_score = board.score();
    if current_score != 0 || depth == 0 { return current_score; }
    let mut best_move = 0;
    let mut best_score = if maxing {-100} else {100};  // fixme
    for index in 0usize..9 {
        if board.values[index] == 0 {  // if this is an open move
            // eprintln!("move found");
            let mut copy = board.clone();  // make a copy for this new branch
            copy.place(maxing, index);
            let score = min_max(copy, !maxing, depth-1, intial_depth);
            if depth == intial_depth {println!("Score of {} for going @ {}", score, index)}
            if (maxing && score > best_score) || (!maxing && score < best_score) {
                best_score = score;
                best_move = index;
            }
        }
    }
    if depth == intial_depth {  // the issue is here
        println!("The best move is {} with a score of {}", best_move, best_score);
        best_move as i8
    } else {best_score}  // at the top of the recursion return move instead of score
}

struct Board {
    values: [i8; 9],
}
impl Board {
    // empty is a static method
    fn empty() -> Board {
        Board {values: [0;9]}
    }
    fn example() -> Board { Board {values: [
        1, 1, 0,
        -1, 0, 0,
        1, -1, -1
    ]}}
    fn over(&self) -> bool {
        self.score() != 0
    }  // fixme: add draw condition
    fn score(&self) -> i8 {
        let mut sum = 0;
        for row in 0..3 {
            sum = self.values[row*3]+self.values[row*3+1]+self.values[row*3+2];
            if sum == 3 {return 1;}
            if sum == -3 {return -1;}
        }
        for col in 0..3 {
            sum = self.values[0+col]+self.values[3+col]+self.values[6+col];
            if sum == 3 {return 1;}
            if sum == -3 {return -1;}
        }
        sum = self.values[0]+self.values[4]+self.values[8];
        if sum == 3 {return 1;}
        if sum == -3 {return -1;}
        sum = self.values[2]+self.values[4]+self.values[6];
        if sum == 3 {return 1;}
        if sum == -3 {return -1;}
        0
    }
    fn place(&mut self, player: bool, index: usize) {
        let play_value: i8 = if player { 1 } else { -1 };
        self.values[index] = play_value;
    }
    fn print(&self) {
        println!("{}\t|\t{}\t|\t{}", self.values[0], self.values[1],self.values[2]);
        println!("----------------------------------");
        println!("{}\t|\t{}\t|\t{}", self.values[3], self.values[4],self.values[5]);
        println!("----------------------------------");
        println!("{}\t|\t{}\t|\t{}", self.values[6], self.values[7],self.values[8]);
        println!();
        println!();
        println!();
        println!();
    }
    fn clone(&self) -> Board {
        Board {values: self.values.clone() }
    }
}