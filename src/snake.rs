extern crate sdl2;

use std::arch::x86_64::_rdrand32_step;
use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use std::ptr::null;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use rand::{Rng, thread_rng};

const BLACK: Color = Color{ r: 0, g: 0, b: 0, a: 255 };
const GREEN: Color = Color{ r: 0, g: 255, b: 0, a: 255 };
const RED: Color = Color{ r: 255, g: 0, b: 0, a: 255 };

pub fn main() {
    let mut b = Board::new_default();
    'game_loop: loop {
        b.tick();
        if b.over {
            break
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10));
    }
}

struct Board {
    window: WindowCanvas,
    event: EventPump,
    food: Food,
    snake: Snake,
    size: i32,
    over: bool
}

impl Board {
    fn get_window(name: &str, width: u32, height: u32) -> (WindowCanvas, EventPump) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(name.borrow(), width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(BLACK);
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().unwrap();
        (canvas, event_pump)
    }
    fn new_default() -> Board {
        Board::new_size(20)
    }
    fn new_size(size: i32) -> Board {
        let (window, event) = Board::get_window("snake", 500, 500);
        let snake = Snake::new(Point{x: (size / 2) as i32, y: (size / 2) as i32 });
        Board {
            window,
            event,
            food: {
                let mut f: Option<Food> = None;
                while f.is_none() || f.as_ref().unwrap().point == *snake.body_segments.first().unwrap() {
                    f = Some(Food::random(size as i32));
                }
                f.unwrap()
            },
            snake,
            size,
            over: false
        }
    }

    fn tick(&mut self) {
        // reset
        self.window.set_draw_color(BLACK);
        self.window.clear();
        // check for events
        for event in self.event.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {self.over = true},
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {self.snake.direction = Direction::Left}
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {self.snake.direction = Direction::Right}
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {self.snake.direction = Direction::Up}
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {self.snake.direction = Direction::Down}
                _ => {}
            }
        }

        // update the snake
        let dead = self.snake.update();
        if dead {self.over = true}
        else if !self.in_bounds() {self.over=true}

        // check for overlapping with the food
        if *self.snake.body_segments.first().unwrap() == self.food.point {
            self.snake.grow();
            self.food = self.get_food();
        }

        for body in self.snake.body_segments.clone() {
            self.draw_point(body, GREEN);
        }
        self.draw_point(self.food.point.clone(), RED);

        // update drawing
        self.update()
    }
    fn get_food(&self) -> Food {
        let mut f: Food;
        loop {
            f = Food::random(self.size as i32);
            if self.snake.body_segments.iter().all(|p| *p!=f.point) {
                return f
            }
        }
    }
    fn in_bounds(&self) -> bool {
        let pt = self.snake.body_segments.first().unwrap();
        pt.x >=0 && pt.x < self.size && pt.y >= 0 && pt.y < self.size
    }

    fn draw_point(&mut self, point: Point, color: Color) {
        self.window.set_draw_color(color);
        let window_width = self.window.window().size().0;
        let box_width = window_width / self.size as u32;
        self.window.draw_rect(
            Rect::new(point.x*box_width as i32, point.y*box_width as i32, box_width, box_width)
        ).unwrap();
    }
    fn update(&mut self) {
        self.window.present();
    }
}
struct Food {
    point: Point
}
impl Food {
    fn new(x: i32, y:i32) -> Food{Food{point: Point{x, y}}}
    fn random(size: i32) -> Food {
        let x = thread_rng().gen_range(0..size);
        let y = thread_rng().gen_range(0..size);
        Food::new(x, y)
    }
}
#[derive(PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn left(&self) -> Point {Point{x: self.x-1, y: self.y}}
    fn right(&self) -> Point {Point{x: self.x+1, y: self.y}}
    fn up(&self) -> Point {Point{x: self.x, y: self.y-1}}
    fn down(&self) -> Point {Point{x: self.x, y: self.y+1}}
}
enum Direction {
    Up, Down, Left, Right
}
struct Snake {
    body_segments: Vec<Point>,
    direction: Direction
}

impl Snake {
    fn new(start_point: Point) -> Snake {
        Snake {
            body_segments: vec![start_point],
            direction: Direction::Right
        }
    }
    fn update(&mut self) -> bool {
        let mut flag = false;
        let new_head = match self.direction {
            Direction::Left => self.body_segments.first().unwrap().left(),
            Direction::Right => self.body_segments.first().unwrap().right(),
            Direction::Up => self.body_segments.first().unwrap().up(),
            _ => self.body_segments.first().unwrap().down()
        };
        for i in (1..self.body_segments.len()).rev() {
            self.body_segments[i] = self.body_segments[i-1].clone();
            if self.body_segments[i] == new_head {flag = true}
        }
        self.body_segments[0] = new_head;
        flag
    }
    fn grow(&mut self) {
        self.body_segments.push(self.body_segments.last().cloned().unwrap())
    }
}