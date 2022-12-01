mod precision;
mod reaction;
mod utils;
mod structs;
mod pair_structs;
mod patterns;
mod stamina;
mod tenacity;

fn main() {
    let mut testmap: structs::Beatmap = Default::default();

    testmap.skills.precision = precision::calculate_precision(&testmap);
    testmap.skills.reaction = reaction::calculate_reaction(&testmap, false);
    testmap.skills.stamina = stamina::calculate_stamina(&testmap);
    testmap.skills.tenacity = tenacity::calculate_tenacity(&testmap);

    let precision = testmap.skills.precision.to_string();
    let reaction = testmap.skills.reaction.to_string();
    let stamina = testmap.skills.stamina.to_string();
    let tenacity = testmap.skills.tenacity.to_string();
    println!("precision: {precision}, reaction: {reaction}, stamina: {stamina}, tenacity: {tenacity}")
}