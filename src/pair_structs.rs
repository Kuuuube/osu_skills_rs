#[derive(Copy, Clone)]
pub struct Vector2f64 {
    pub x: f64,
    pub y: f64
}

impl Default for Vector2f64 {
    fn default() -> Self {Vector2f64 { x: (0.0), y: (0.0) }}
}

#[derive(Copy, Clone)]
pub struct Vector2i64 {
    pub x: i64,
    pub y: i64
}

impl Default for Vector2i64 {
    fn default() -> Self {Vector2i64 { x: (0), y: (0) }}
}

#[derive(Copy, Clone)]
pub struct Vector2i32Map {
    pub x: i32,
    pub y: i32
}

impl Default for Vector2i32Map {
    fn default() -> Self {Vector2i32Map { x: (0), y: (0) }}
}

#[derive(Clone)]
pub struct VectorVectori32Map {
    pub x: i32,
    pub y: Vec<Vec<i32>>
}

impl Default for VectorVectori32Map {
    fn default() -> Self {VectorVectori32Map { x: (0), y: (vec![vec![0]]) }}
}

fn get_length(vector2_0: Vector2f64) -> f64 {
    return f64::sqrt(vector2_0.x * vector2_0.x + vector2_0.y * vector2_0.y);
}

pub fn get_distance_from(vector2_0: &Vector2f64, vector2_1: &Vector2f64) -> f64 {
    return get_length(Vector2f64{ x: vector2_0.x - vector2_1.x, y: vector2_0.y - vector2_1.y });
}