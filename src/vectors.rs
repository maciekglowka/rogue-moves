use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

// Vector2

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Vector2Int {
    pub x: i32,
    pub y: i32
}

impl Vector2Int {
    pub fn new(x: i32, y: i32) -> Vector2Int {
        Vector2Int{x: x, y: y}
    }
    pub fn from_world(x: f32, y: f32) -> Vector2Int {
        Vector2Int{x: (x / crate::graphics::TILE_SIZE) as i32, y: (y / crate::graphics::TILE_SIZE) as i32}
    }
    pub fn len(&self) -> f32 {
        ((self.x * self.x + self.y * self.y) as f32).sqrt()
    }
    pub fn dist(&self, other: Vector2Int) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx*dx + dy*dy) as f32).sqrt()
    }
}

impl Add for Vector2Int {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Vector2Int::new(self.x + other.x, self.y + other.y);
    }
}

impl AddAssign for Vector2Int {
    fn add_assign(&mut self, other: Self) {
        *self = Self{x: self.x + other.x, y: self.y + other.y};
    }
}

impl Sub for Vector2Int {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        return Vector2Int::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Vector2Int {
    fn sub_assign(&mut self, other: Self) {
        *self = Self{x: self.x - other.x, y: self.y - other.y};
    }
}

impl Div<i32> for Vector2Int {
    type Output = Self;

    fn div(self, other: i32) -> Self {
        return Vector2Int::new(self.x / other, self.y / other)
    }
}

impl Mul<i32> for Vector2Int {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        return Vector2Int::new(self.x * other, self.y * other)
    }
}

pub const ORTHO_DIRECTIONS: [Vector2Int; 4] = [
    Vector2Int{x:1, y:0}, Vector2Int{x:-1, y:0},
    Vector2Int{x:0, y:1}, Vector2Int{x:0, y:-1}
];

pub const DIAGONAL_DIRECTIONS: [Vector2Int; 4] = [
    Vector2Int{x:1, y:1}, Vector2Int{x:-1, y:1},
    Vector2Int{x:-1, y:-1}, Vector2Int{x:1, y:-1}
];

pub fn vector_line(a: Vector2Int, b: Vector2Int) -> Vec<Vector2Int> {
    let dx = b.x - a.x;
    let dy = b.y - a.y;

    let d = std::cmp::max(dx.abs(), dy.abs());

    let mut output = Vec::new();
    if d == 0 { return output; }

    for step in 0..=d {
        let t = step as f32 / d as f32;
        output.push(
            Vector2Int::new(
                lerp(a.x as f32, b.x as f32, t) as i32,
                lerp(a.y as f32, b.y as f32, t) as i32
            )
        );
    }
    output
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}