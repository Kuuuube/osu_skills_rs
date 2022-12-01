mod precision;
mod reaction;
mod utils;
mod structs;
mod vector2d;
mod patterns;

fn main() {
    let mut testmap: structs::Beatmap = Default::default();

    testmap.skills.precision = precision::calculate_precision(&testmap, 1000.0, 0.1, 20.0, 2.0);
    testmap.skills.reaction = reaction::calculate_reaction(&testmap, false);

    let precision = testmap.skills.precision.to_string();
    let reaction = testmap.skills.reaction.to_string();
    println!("precision: {precision}, reaction: {reaction}")
}