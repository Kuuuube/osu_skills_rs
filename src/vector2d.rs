pub struct Vector2F64 {
    pub x: f64,
    pub y: f64
}

impl Default for Vector2F64 {
    fn default() -> Self {Vector2F64 { x: (0.0), y: (0.0) }}
}

fn get_length(vector2_0: Vector2F64) -> f64 {
    return f64::sqrt(vector2_0.x * vector2_0.x + vector2_0.y * vector2_0.y);
}

pub fn get_distance_from(vector2_0: &Vector2F64, vector2_1: &Vector2F64) -> f64 {
    return get_length(Vector2F64{ x: vector2_0.x - vector2_1.x, y: vector2_0.y - vector2_1.y });
}