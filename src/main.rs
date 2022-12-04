use std::vec;

mod utils;
mod structs;
mod pair_structs;
mod patterns;
mod algs;
mod skill_calculation;
mod osu_parser;

fn main() {
    let mut beatmap: structs::Beatmap = osu_parser::parse_beatmap("./testmaps/v3.osu");

    let format = &beatmap.sm.to_string();
    println!("{format}");
 
    let hit_object_test: structs::HitObject = Default::default();
    beatmap.hit_objects = vec![hit_object_test];

    beatmap = utils::apply_mods(beatmap);

    beatmap = skill_calculation::strains::calculate_aim_strains(beatmap);
    beatmap = skill_calculation::strains::calculate_tap_strains(beatmap);

    beatmap = skill_calculation::generic::prepare_timing_points(beatmap);

    beatmap = skill_calculation::generic::prepare_aim_data(beatmap);

    beatmap.skills.reaction = skill_calculation::reaction::calculate_reaction(&beatmap);
    beatmap.skills.stamina = skill_calculation::stamina::calculate_stamina(&beatmap);
    beatmap.skills.tenacity = skill_calculation::tenacity::calculate_tenacity(&beatmap);
    beatmap.skills.agility = skill_calculation::agility::calculate_agility(&beatmap);
    beatmap.skills.precision = skill_calculation::precision::calculate_precision(&beatmap);
    beatmap.skills.accuracy = skill_calculation::accuracy::calculate_accuracy(&beatmap);
    beatmap.skills.memory = skill_calculation::memory::calculate_memory(&beatmap);

    let precision = beatmap.skills.precision.to_string();
    let reaction = beatmap.skills.reaction.to_string();
    let stamina = beatmap.skills.stamina.to_string();
    let tenacity = beatmap.skills.tenacity.to_string();
    let agility = beatmap.skills.agility.to_string();
    let accuracy = beatmap.skills.accuracy.to_string();
    let memory = beatmap.skills.memory.to_string();
    
    println!("stamina: {stamina}, tenacity: {tenacity}, agility: {agility}, accuracy: {accuracy}, precision: {precision}, reaction: {reaction}, memory: {memory}")
}