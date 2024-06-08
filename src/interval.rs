use crate::rtweekend::INFINITY;

#[derive(Clone, Copy)]
pub struct Interval {
  pub min: f32,
  pub max: f32,
}

impl Interval {
    // Constructor for default interval
    pub fn new() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY,
        }
    }

    // Constructor with specified min and max
    pub fn new_with_bounds(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    // Method to get the size of the interval
    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    // Method to check if the interval contains a value
    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    // Method to check if the interval surrounds a value (exclusive)
    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        return x;
    }

    // Static members
    pub const EMPTY: Interval = Interval {
        min: INFINITY,
        max: -INFINITY,
    };
    pub const UNIVERSE: Interval = Interval {
        min: -INFINITY,
        max: INFINITY,
    };
}