mod precision;

fn main() {
    let precision = precision::calculate_precision(223.217, 1000.0, 0.1, 10, 20.0, 2.0).to_string();
    println!("new: {precision}");
}