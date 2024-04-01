extern crate ncurses;

use ncurses::*;
use std::collections::VecDeque;

const WIDTH: i32 = 20;
const HEIGHT: i32 = 20;
const GAME_SPEED: u64 = 100;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: VecDeque<i32, i32>,
    direction: Direction,
}

impl Snake {
    fn new() -> Snake {
        let mut body = VecDeque::new();
        body.push_back((WIDTH / 2, HEIGHT / 2));
        Snake {
            body,
            direction: Direction::Right,
        }
    }

    fn move_forward(&mut self) {
        let (x, y) = *self.body.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
        self.body.push_front(new_head);
        self.body.pop_back();
    }

    fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn grow(&mut self) {
        let (x ,y) = *self.body.back().unwrap();
        self.body.push_back((x, y));
    }
}

fn init_ncurses() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);
    timeout(0);
}

fn main() {
    init_ncurses();
    let mut snake = Snake::new();
    let mut food = (10, 10);
    let mut score = 0;

    loop {
        clear();
        for &(x, y) in &snake.body {
            mvprintw(y, x * 2, "0");
        }
        mvprintw(food.1, food.0 * 2, "X");
        mvprintw(HEIGHT + 1, 0, &format!("Score: {}", score));

        let ch = getch();
        match ch {
            KEY_UP if snake.direction != Direction::Down => snake.change_direction(Direction::Up),
            KEY_DOWN if snake.direction != Direction::Up => snake.change_direction(Direction::Down),
            KEY_LEFT if snake.direction != Direction::Right => snake.change_direction(Direction::Left),
            KEY_RIGHT if snake.direction != Direction::Left => snake.change_direction(Direction::Right),
            _ => {}
        }

        snake.move_forward();
        if snake.body.front() == Some(&food) {
            snake.grow();
            score += 10;
            // TBD random food locations
        }

        // TBD collisions

        refresh();
        std::thread::sleep(std::time::Duration::from_millis(GAME_SPEED));
    }

    endwin();
}