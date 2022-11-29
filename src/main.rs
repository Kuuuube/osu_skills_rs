mod precision;
mod structs;
mod vector2d;

fn main() {
    let testmap: structs::Beatmap = Default::default();

    let precision = precision::calculate_precision(testmap, 1000.0, 0.1, 20.0, 2.0).to_string();
    println!("new: {precision}");

    let test = vector2d::Vector2F64{ x: 100.0, y: 500.2};

    println!("{}, {}", test.x, test.y);
}