#[derive(Copy, Clone)]
pub struct Pairf64 {
    pub x: f64,
    pub y: f64
}

impl Default for Pairf64 {
    fn default() -> Self {Pairf64 { x: (0.0), y: (0.0) }}
}

#[derive(Copy, Clone)]
pub struct Pairi64 {
    pub x: i64,
    pub y: i64
}

impl Default for Pairi64 {
    fn default() -> Self {Pairi64 { x: (0), y: (0) }}
}

#[derive(Copy, Clone)]
pub struct Pairi32 {
    pub x: i32,
    pub y: i32
}

impl Default for Pairi32 {
    fn default() -> Self {Pairi32 { x: (0), y: (0) }}
}

#[derive(Clone)]
pub struct Pairi32VectorVectori32 {
    pub x: i32,
    pub y: Vec<Vec<i32>>
}

impl Default for Pairi32VectorVectori32 {
    fn default() -> Self {Pairi32VectorVectori32 { x: (0), y: (vec![vec![0]]) }}
}

fn get_length(vector2_0: Pairf64) -> f64 {
    return f64::sqrt(vector2_0.x * vector2_0.x + vector2_0.y * vector2_0.y);
}

pub fn get_distance_from(vector2_0: &Pairf64, vector2_1: &Pairf64) -> f64 {
    return get_length(Pairf64{ x: vector2_0.x - vector2_1.x, y: vector2_0.y - vector2_1.y });
}