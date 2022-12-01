use std::vec;

mod utils;
mod structs;
mod pair_structs;
mod patterns;
mod erf;
mod skill_calculation;

fn main() {
    let mut testmap: structs::Beatmap = Default::default();
 
/*     let hit_object_test: structs::HitObject = Default::default();
    testmap.hit_objects = vec![hit_object_test];  */

    testmap.skills.precision = skill_calculation::precision::calculate_precision(&testmap);
    testmap.skills.reaction = skill_calculation::reaction::calculate_reaction(&testmap);
    testmap.skills.stamina = skill_calculation::stamina::calculate_stamina(&testmap);
    testmap.skills.tenacity = skill_calculation::tenacity::calculate_tenacity(&testmap);
    testmap.skills.agility = skill_calculation::agility::calculate_agility(&testmap);
    testmap.skills.accuracy = skill_calculation::accuracy::calculate_accuracy(&testmap);
    testmap.skills.memory = skill_calculation::memory::calculate_memory(&testmap);

    let precision = testmap.skills.precision.to_string();
    let reaction = testmap.skills.reaction.to_string();
    let stamina = testmap.skills.stamina.to_string();
    let tenacity = testmap.skills.tenacity.to_string();
    let agility = testmap.skills.agility.to_string();
    let accuracy = testmap.skills.accuracy.to_string();
    let memory = testmap.skills.memory.to_string();
    println!("stamina: {stamina}, tenacity: {tenacity}, agility: {agility}, accuracy: {accuracy}, precision: {precision}, reaction: {reaction}, memory: {memory}")
}