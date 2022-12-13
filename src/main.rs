mod algs;
mod classic_skill_calculation;
mod skill_calculation;
mod structs;
mod pair_structs;
mod osu_parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut filepath: String = Default::default();
    let mut alg: String = Default::default();
    let mut mod_int: i32 = Default::default();
    let mut help: bool = false;

    for arg in args {
        let split: Vec<&str> = arg.split("=").collect();
        match &split[0].to_lowercase() as &str {
            "--help" => { help = true; print!("osu!Skills rs\nUsage: osu_skills_rs [OPTION]...\n\nMandatory:\n     --file=FILE                 path to .osu file to parse\n\nOptional:\n     --alg=ALG                   calculation alg to use (`classic` or `default`)\n     --mod-int=MODS              sum of all mod values to apply (`2`: EZ, `8`: HD, `16`: HR, `64`: DT, `256`: HT)\n") }
            "--file" => { filepath = split[1].to_string() },
            "--alg" => { alg = split[1].to_string() },
            "--mod-int" => { mod_int = safe_parse_i32(split[1]) },
            _ => {}
        }
    }

    if filepath.len() == 0 && !help {
        print!("osu!Skills rs: missing .osu file path\nUsage: osu_skills_rs [OPTION]...\n\nTry `osu_skills_rs --help` for more options.\n")
    } else if !help {
        match &alg.to_lowercase() as &str {
            "classic" => classic_results(classic_process_beatmap(&filepath, mod_int)),
            _ => results(process_beatmap(&filepath, mod_int))
        };
    }
}

fn process_beatmap(filepath_str: &str, mod_int: i32) -> structs::Beatmap {
    let mut beatmap: structs::Beatmap = osu_parser::parse_beatmap(filepath_str);
    beatmap.mods = mod_int;

    if beatmap.hit_objects.len() >= 2 {
        beatmap = skill_calculation::utils::apply_mods(beatmap);

        beatmap = skill_calculation::calculation::generic::prepare_timing_points(beatmap);
        beatmap = skill_calculation::calculation::slider::approximate_slider_points(beatmap);
        beatmap = skill_calculation::calculation::generic::bake_slider_data(beatmap);

        beatmap = skill_calculation::calculation::generic::prepare_aim_data(beatmap);
        beatmap = skill_calculation::calculation::generic::prepare_tap_data(beatmap);

        beatmap = skill_calculation::calculation::strains::calculate_aim_strains(beatmap);
        beatmap = skill_calculation::calculation::strains::calculate_tap_strains(beatmap);

        beatmap.skills.reaction = skill_calculation::calculation::reaction::calculate_reaction(&beatmap);
        beatmap.skills.stamina = skill_calculation::calculation::stamina::calculate_stamina(&beatmap);
        beatmap.skills.tenacity = skill_calculation::calculation::tenacity::calculate_tenacity(&beatmap);
        beatmap.skills.agility = skill_calculation::calculation::agility::calculate_agility(&beatmap);
        beatmap.skills.precision = skill_calculation::calculation::precision::calculate_precision(&beatmap);
        beatmap.skills.accuracy = skill_calculation::calculation::accuracy::calculate_accuracy(&beatmap);
        beatmap.skills.memory = skill_calculation::calculation::memory::calculate_memory(&beatmap);
    }
    return beatmap;
}

fn classic_process_beatmap(filepath_str: &str, mod_int: i32) -> structs::Beatmap {
    let mut beatmap: structs::Beatmap = osu_parser::parse_beatmap(filepath_str);
    beatmap.mods = mod_int;

    if beatmap.hit_objects.len() >= 2 {
        beatmap = classic_skill_calculation::utils::apply_mods(beatmap);

        beatmap = classic_skill_calculation::calculation::generic::prepare_timing_points(beatmap);
        beatmap = classic_skill_calculation::calculation::slider::approximate_slider_points(beatmap);
        beatmap = classic_skill_calculation::calculation::generic::bake_slider_data(beatmap);

        beatmap = classic_skill_calculation::calculation::generic::prepare_aim_data(beatmap);
        beatmap = classic_skill_calculation::calculation::generic::prepare_tap_data(beatmap);

        beatmap = classic_skill_calculation::calculation::strains::calculate_aim_strains(beatmap);
        beatmap = classic_skill_calculation::calculation::strains::calculate_tap_strains(beatmap);

        beatmap.skills.reaction = classic_skill_calculation::calculation::reaction::calculate_reaction(&beatmap);
        beatmap.skills.stamina = classic_skill_calculation::calculation::stamina::calculate_stamina(&beatmap);
        beatmap.skills.tenacity = classic_skill_calculation::calculation::tenacity::calculate_tenacity(&beatmap);
        beatmap.skills.agility = classic_skill_calculation::calculation::agility::calculate_agility(&beatmap);
        beatmap.skills.precision = classic_skill_calculation::calculation::precision::calculate_precision(&beatmap);
        beatmap.skills.accuracy = classic_skill_calculation::calculation::accuracy::calculate_accuracy(&beatmap);
        beatmap.skills.memory = classic_skill_calculation::calculation::memory::calculate_memory(&beatmap);
    }
    return beatmap;
}


fn classic_results(beatmap: structs::Beatmap) {
    println!("Stamina: {}, Tenacity: {}, Agility: {}, Accuracy: {}, Precision: {}, Reaction: {}, Memory: {}", beatmap.skills.stamina, beatmap.skills.tenacity, beatmap.skills.agility, beatmap.skills.accuracy, beatmap.skills.precision, beatmap.skills.reaction, beatmap.skills.memory);
}

fn results(beatmap: structs::Beatmap) {
    println!("Stamina: {}, Streams: {}, Aim: {}, Accuracy: {}, Precision: {}, Reaction: {}, Flashlight: {}", beatmap.skills.stamina, beatmap.skills.tenacity, beatmap.skills.agility, beatmap.skills.accuracy, beatmap.skills.precision, beatmap.skills.reaction, beatmap.skills.memory);
}

fn safe_parse_i32(input: &str) -> i32 {
    let output = match input.parse::<i32>() {
        Ok(ok) => ok,
        Err(error) => { println!("Failed to parse mod arg. Error: {error}: `{input}`"); 0 }
    };
    return output;
}