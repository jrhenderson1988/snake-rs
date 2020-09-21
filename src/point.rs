#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Point { x, y }
    }

    pub fn apply_delta(&self, delta: (i16, i16)) -> Self {
        Point::new(
            Point::apply_delta_to_value(self.x, delta.0),
            Point::apply_delta_to_value(self.y, delta.1)
        )
    }

    fn apply_delta_to_value(value: u16, delta: i16) -> u16 {
        if delta.is_negative() && delta.abs() as u16 > value {
            panic!("Applying delta {} to value {} would result in a negative number", delta, value);
        } else {
            (value as i16 + delta) as u16
        }
    }
}

