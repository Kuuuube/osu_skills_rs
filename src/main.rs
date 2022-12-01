mod precision;
mod reaction;
mod utils;
mod structs;
mod pair_structs;
mod patterns;
mod stamina;

fn main() {
    let mut testmap: structs::Beatmap = Default::default();

    testmap.skills.precision = precision::calculate_precision(&testmap);
    testmap.skills.reaction = reaction::calculate_reaction(&testmap, false);
    testmap.skills.stamina = stamina::calculate_stamina(&testmap);

    let precision = testmap.skills.precision.to_string();
    let reaction = testmap.skills.reaction.to_string();
    let stamina = testmap.skills.stamina.to_string();
    println!("precision: {precision}, reaction: {reaction}, stamina: {stamina}")
}