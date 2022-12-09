mod utils;
mod structs;
mod pair_structs;
mod algs;
mod skill_calculation;
mod osu_parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let get_path = match args.get(1) {
        Some(some) => some,
        None => { panic!("Invalid .osu file path specified."); }
    };

    let beatmap: structs::Beatmap = process_beatmap(get_path);

    handle_results(beatmap);
}

fn process_beatmap(filepath_str: &str) -> structs::Beatmap {
    let mut beatmap: structs::Beatmap = osu_parser::parse_beatmap(filepath_str);

    if beatmap.hit_objects.len() >= 2 {
        beatmap = utils::apply_mods(beatmap);

        beatmap = skill_calculation::generic::prepare_timing_points(beatmap);
        beatmap = skill_calculation::slider::approximate_slider_points(beatmap);
        beatmap = skill_calculation::generic::bake_slider_data(beatmap);

        beatmap = skill_calculation::generic::prepare_aim_data(beatmap);
        beatmap = skill_calculation::generic::prepare_tap_data(beatmap);

        beatmap = skill_calculation::strains::calculate_aim_strains(beatmap);
        beatmap = skill_calculation::strains::calculate_tap_strains(beatmap);

        beatmap.skills.reaction = skill_calculation::reaction::calculate_reaction(&beatmap);
        beatmap.skills.stamina = skill_calculation::stamina::calculate_stamina(&beatmap);
        beatmap.skills.tenacity = skill_calculation::tenacity::calculate_tenacity(&beatmap);
        beatmap.skills.agility = skill_calculation::agility::calculate_agility(&beatmap);
        beatmap.skills.precision = skill_calculation::precision::calculate_classic_precision(&beatmap);
        beatmap.skills.accuracy = skill_calculation::accuracy::calculate_accuracy(&beatmap);
        beatmap.skills.memory = skill_calculation::memory::calculate_memory(&beatmap);
    }
    return beatmap;
}

fn handle_results(beatmap: structs::Beatmap) {
    println!("stamina: {}, tenacity: {}, agility: {}, accuracy: {}, precision: {}, reaction: {}, memory: {}", beatmap.skills.stamina, beatmap.skills.tenacity, beatmap.skills.accuracy, beatmap.skills.agility, beatmap.skills.precision, beatmap.skills.reaction, beatmap.skills.memory);
}