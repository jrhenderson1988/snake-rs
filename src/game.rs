use crate::snake::Snake;
use crate::point::Point;
use crate::direction::Direction;
use std::io::Stdout;
use crossterm::ExecutableCommand;
use crossterm::terminal::{Clear, ClearType, size, SetSize};
use crossterm::style::{SetForegroundColor, Print, ResetColor, Color};
use std::time::{Duration, Instant};
use crossterm::cursor::{Show, MoveTo, Hide};
use crossterm::event::{poll, read, Event, KeyCode};
use crate::command::Command;
use rand::Rng;

const BASE_INTERVAL_MS: u64 = 1000;
const MAX_SPEED: u64 = 20;

#[derive(Debug)]
pub struct Game {
    stdout: Stdout,
    original_terminal_size: (u16, u16),
    width: u16,
    height: u16,
    food: Option<Point>,
    snake: Snake,
    speed: u8,
}

impl Game {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        let original_terminal_size: (u16, u16) = size().unwrap();
        Self {
            stdout,
            original_terminal_size,
            width,
            height,
            food: None,
            snake: Snake::new(
                Point::new(width / 2, height / 2),
                3,
                match rand::thread_rng().gen_range(0, 4) {
                    0 => Direction::Up,
                    1 => Direction::Right,
                    2 => Direction::Down,
                    _ => Direction::Left
                }
            ),
            speed: 10,
        }
    }

    pub fn run(&mut self) {
        self.prepare_ui();
        self.place_food();

        let mut done = false;
        while !done {
            let now = Instant::now();
            let interval = self.calculate_interval();
            let direction = self.snake.get_direction();

            loop {
                let elapsed = now.elapsed();
                if elapsed >= interval {
                    break;
                }

                if let Some(command) = self.get_command(interval - elapsed) {
                    match command {
                        Command::Quit => {
                            done = true;
                            break;
                        }
                        Command::Turn(towards) => {
                            if direction != towards && direction.opposite() != towards {
                                self.snake.set_direction(towards);
                            }
                        }
                    }
                }
            }

            if self.has_collided_with_wall() || self.has_bitten_itself() {
                done = true;
            } else {
                self.snake.slither();

                if let Some(food_point) = self.food {
                    if self.snake.get_head_point() == food_point {
                        self.snake.grow();
                        self.place_food();
                    }
                }

                self.render();
            }
        }

        self.restore_ui();

        println!("Game Over! Your score is {}", self.snake.len());
    }

    pub fn render(&mut self) {
        self.draw_borders();
        self.draw_background();
        self.draw_food();
        self.draw_snake();
    }

    fn calculate_interval(&self) -> Duration {
        Duration::from_millis(
            (BASE_INTERVAL_MS / MAX_SPEED) * (MAX_SPEED - (self.speed as u64))
        )
    }

    fn get_command(&self, wait_for: Duration) -> Option<Command> {
        if let Ok(occurred) = poll(wait_for) {
            if occurred {
                if let Ok(event) = read() {
                    if let Event::Key(event) = event {
                        return match event.code {
                            KeyCode::Char('Q') => Some(Command::Quit),
                            KeyCode::Char('q') => Some(Command::Quit),
                            KeyCode::Esc => Some(Command::Quit),
                            KeyCode::Up => Some(Command::Turn(Direction::Up)),
                            KeyCode::Right => Some(Command::Turn(Direction::Right)),
                            KeyCode::Down => Some(Command::Turn(Direction::Down)),
                            KeyCode::Left => Some(Command::Turn(Direction::Left)),
                            _ => None
                        };
                    }
                }
            }
        }

        None
    }

    fn has_collided_with_wall(&self) -> bool {
        let head_point = self.snake.get_head_point();

        match self.snake.get_direction() {
            Direction::Up => head_point.y == 0,
            Direction::Right => head_point.x == self.width - 1,
            Direction::Down => head_point.y == self.height - 1,
            Direction::Left => head_point.x == 0,
        }
    }

    fn has_bitten_itself(&self) -> bool {
        let delta = self.snake.get_direction().delta();
        let next_head_point = self.snake.get_head_point().apply_delta(delta);
        let mut body_points = self.snake.get_body_points().clone();
        body_points.remove(body_points.len() - 1);
        body_points.remove(0);

        body_points.contains(&next_head_point)
    }

    fn place_food(&mut self) {
        loop {
            let random_x = rand::thread_rng().gen_range(0, self.width);
            let random_y = rand::thread_rng().gen_range(0, self.height);
            let point = Point::new(random_x, random_y);
            if !self.snake.contains_point(&point) {
                self.food = Some(point);
                break;
            }
        }
    }

    fn prepare_ui(&mut self) {
        self.stdout
            .execute(SetSize(self.width + 3, self.height + 3)).unwrap()
            .execute(Clear(ClearType::All)).unwrap()
            .execute(Hide).unwrap();
    }

    fn restore_ui(&mut self) {
        let (cols, rows) = self.original_terminal_size;
        self.stdout
            .execute(SetSize(cols, rows)).unwrap()
            .execute(Clear(ClearType::All)).unwrap()
            .execute(Show).unwrap()
            .execute(ResetColor).unwrap();
    }

    fn draw_snake(&mut self) {
        self.stdout.execute(SetForegroundColor(Color::Green)).unwrap();

        for body in self.snake.get_body_points() {
            self.stdout
                .execute(MoveTo(body.x + 1, body.y + 1)).unwrap()
                .execute(Print("o")).unwrap();
        }

        let head_point = self.snake.get_head_point();
        self.stdout
            .execute(MoveTo(head_point.x + 1, head_point.y + 1)).unwrap()
            .execute(
                Print(
                    "O"
                    // match self.snake.get_direction() {
                    //     Direction::North => "v".to_string(),
                    //     Direction::East => "<".to_string(),
                    //     Direction::South => "^".to_string(),
                    //     Direction::West => ">".to_string(),
                    // }
                )
            ).unwrap();
    }

    fn draw_food(&mut self) {
        self.stdout.execute(SetForegroundColor(Color::Red)).unwrap();

        for food in self.food.iter() {
            self.stdout
                .execute(MoveTo(food.x + 1, food.y + 1)).unwrap()
                .execute(Print("*")).unwrap();
        }
    }

    fn draw_background(&mut self) {
        self.stdout.execute(ResetColor).unwrap();

        for y in 1..self.height + 1 {
            for x in 1..self.width + 1 {
                self.stdout
                    .execute(MoveTo(x, y)).unwrap()
                    .execute(Print(" ")).unwrap();
            }
        }
    }

    fn draw_borders(&mut self) {
        self.stdout.execute(SetForegroundColor(Color::DarkGrey)).unwrap();

        for y in 0..self.height + 2 {
            self.stdout
                .execute(MoveTo(0, y)).unwrap()
                .execute(Print("#")).unwrap()
                .execute(MoveTo(self.width + 1, y)).unwrap()
                .execute(Print("#")).unwrap();
        }

        for x in 0..self.width + 2 {
            self.stdout
                .execute(MoveTo(x, 0)).unwrap()
                .execute(Print("#")).unwrap()
                .execute(MoveTo(x, self.height + 1)).unwrap()
                .execute(Print("#")).unwrap();
        }

        self.stdout
            .execute(MoveTo(0, 0)).unwrap()
            .execute(Print("+")).unwrap()
            .execute(MoveTo(self.width + 1, self.height + 1)).unwrap()
            .execute(Print("+")).unwrap()
            .execute(MoveTo(self.width + 1, 0)).unwrap()
            .execute(Print("+")).unwrap()
            .execute(MoveTo(0, self.height + 1)).unwrap()
            .execute(Print("+")).unwrap();
    }
}