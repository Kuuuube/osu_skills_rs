mod precision;
mod reaction;
mod utils;
mod structs;
mod pair_structs;
mod patterns;
mod stamina;
mod tenacity;
mod agility;

fn main() {
    let mut testmap: structs::Beatmap = Default::default();

    testmap.skills.precision = precision::calculate_precision(&testmap);
    testmap.skills.reaction = reaction::calculate_reaction(&testmap, false);
    testmap.skills.stamina = stamina::calculate_stamina(&testmap);
    testmap.skills.tenacity = tenacity::calculate_tenacity(&testmap);
    testmap.skills.agility = agility::calculate_agility(&testmap);

    let precision = testmap.skills.precision.to_string();
    let reaction = testmap.skills.reaction.to_string();
    let stamina = testmap.skills.stamina.to_string();
    let tenacity = testmap.skills.tenacity.to_string();
    let agility = testmap.skills.agility.to_string();
    println!("stamina: {stamina}, tenacity: {tenacity}, agility: {agility}, accuracy: {{accuracy}}, precision: {precision}, reaction: {reaction}, memory: {{memory}}")
}