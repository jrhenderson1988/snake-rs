use crate::direction::Direction;
use crate::point::Point;

#[derive(Debug)]
pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
    digesting: bool,
}

impl Snake {
    pub fn new(start: Point, length: u16, direction: Direction) -> Self {
        let delta = direction.opposite().delta();
        let body: Vec<Point> = (0..length)
            .into_iter()
            .map(|i| start.apply_delta((delta.0 * (i as i16), delta.1 * (i as i16))))
            .collect();

        Self { body, direction, digesting: false }
    }

    pub fn get_head_point(&self) -> Point {
        self.body.first().unwrap().clone()
    }

    pub fn get_body_points(&self) -> Vec<Point> {
        self.body.clone()
    }

    pub fn get_direction(&self) -> Direction {
        self.direction.clone()
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        self.body.contains(point)
    }

    pub fn slither(&mut self) {
        self.body.insert(0, self.body.first().unwrap().apply_delta(self.direction.delta()));
        if !self.digesting {
            self.body.remove(self.body.len() - 1);
        } else {
            self.digesting = false;
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if direction != self.direction.opposite() {
            self.direction = direction;
        }
    }

    pub fn grow(&mut self) {
        self.digesting = true;
    }
}