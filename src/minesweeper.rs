use std::collections::HashSet;
use rand::{Rng, thread_rng};
use rand::distributions::Slice;
use sdl2::event::{Event, EventType};
use sdl2::{EventPump, Sdl};
use sdl2::event::EventType::MouseButtonDown;
use sdl2::keyboard::Keycode;
use sdl2::libc::kevent;
use sdl2::mouse::{MouseState, MouseUtil};
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

fn main() {

}

const SIZE: usize = 30;
const WINDOW_SIZE: u32 = 600;
const WHITE: Color = Color {r: 255, g: 255, b: 255, a: 255};
const BLACK: Color = Color {r: 0, g: 0, b: 0, a: 255};

struct Graphics {
    board: Board,
    context: Sdl,
    window: WindowCanvas,
    over: bool,
}
impl Graphics {
    fn new(b: Board) -> Graphics {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("interactive minesweeper", WINDOW_SIZE, WINDOW_SIZE)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(WHITE);
        canvas.clear();
        canvas.present();
        Graphics {
            board: b,
            context: sdl_context,
            window: canvas,
            over: false
        }

    }

    fn tick(&mut self) {
        self.event_loop();
        // self.draw();
    }
    fn event_loop(&mut self) {
        for event in self.context.event_pump().expect("event pump failed").poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {self.over = true},
                Event::MouseButtonDown { .. } => {
                    let mouse = self.context.event_pump().unwrap().mouse_state();
                    let row = (mouse.y() / WINDOW_SIZE as i32) as usize;
                    let col = (mouse.x() / WINDOW_SIZE as i32) as usize;
                    let c = Cell(row, col);

                    // todo: reveal cell
                }
                _ => {}
            }
        }
    }
    fn draw(&mut self, revealed: Vec<&Cell>) {
        for cell in revealed {

        }
    }
    fn draw_full(&mut self) {
        self.draw_background();
    }
    fn draw_background(&mut self) {
        self.window.set_draw_color(WHITE);
        self.window.clear();
        let box_size = WINDOW_SIZE as i32 / SIZE as i32;
        self.window.set_draw_color(BLACK);
        for i in 1..SIZE as i32 {
            let new_val = i * box_size;
            let pts = vec!(Point::new(0, new_val), Point::new(SIZE as i32, new_val));
            self.window.draw_lines(pts.as_slice())
                .expect("Issue drawing horizontal gridlines");
            let pts = vec!(Point::new(new_val, 0), Point::new(new_val, SIZE as i32, ));
            self.window.draw_lines(pts.as_slice())
                .expect("Issue drawing vertical gridlines");
        }
    }
}
struct Board {
    over: bool,
    cells: [[i32; SIZE]; SIZE],
}
#[derive(Clone, Copy)]
struct Cell(usize, usize);
impl Board {
    fn new() -> Board {
        let mut b = Board {
            over: false,
            cells: [[0; SIZE]; SIZE],
        };
        b.place_bombs(30);
        b
    }
    fn place_bombs(&mut self, count: u32) {
        for _ in 0..count {
            let row = thread_rng().gen_range(0..SIZE);
            let col = thread_rng().gen_range(0..SIZE);
            self.cells[row][col] = -1;
            for neighbor in self.neighbors(row, col) {
                if self.cells[neighbor.0][neighbor.1] != -1 {
                    self.cells[neighbor.0][neighbor.1] += 1;
                }
            }
        }
    }

    fn uncover(&mut self, cell: Cell) -> Result<HashSet<Cell>, Cell> {
        let val = self.number(cell);
        if val == -1 {
            return Err(cell);
        }
        let returning: HashSet<Cell> = HashSet::new();
        Ok(returning)  // fixme
        // if val == 0 {
        // }
    }

    fn bomb(&self, row: usize, col: usize) -> bool {self.cells[row][col]==-1}
    fn safe(&self, row: usize, col: usize) -> bool {!self.bomb(row, col)}
    fn number(&self, cell: Cell) -> i32 {
        self.cells[cell.0][cell.1]
    }
    fn neighbor_cell(&self, cell: Cell) -> Vec<Cell> {
        self.neighbors(cell.0, cell.1)
    }
    fn neighbors(&self, row: usize, col: usize) -> Vec<Cell> {
        let r = row as i16;
        let c = col as i16;
        [
            (r-1, c-1), (r-1, c), (r-1, c+1),
            (r, c-1),             (r, c+1),
            (r+1, c-1), (r+1, c), (r+1, c+1),
        ]
            .iter()
            .filter(|(r1, c1)| self.in_board(*r1, *c1))
            .map(|(r1, c1)| Cell(*r1 as usize, *c1 as usize))
            .collect()
    }
    fn in_board(&self, row: i16, col: i16) -> bool {
        row >= 0 && row < SIZE as i16 && col >= 0 && col < SIZE as i16
    }
}


struct Solver {
    board: Board
}

impl Solver {

}