use std::{fs, io::Write};

mod algs;
mod classic_skill_calculation;
mod skill_calculation;
mod structs;
mod pair_structs;
mod osu_parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut osu_filepath: String = Default::default();
    let mut alg: String = Default::default();
    let mut mod_int: i32 = Default::default();
    let mut is_dir: String = Default::default();
    let mut output_file_string: String = Default::default();
    let mut output_type: String = "stdout".to_string();

    let mut i: usize = 1;
    while i < args.len() {
        let arg = match args.get(i) {
            Some(some) => some,
            None => ""
        };
        let split: Vec<&str> = arg.split("=").collect();
        match &split[0].to_lowercase() as &str {
            "--help" => { print!("osu!Skills rs\nUsage: osu_skills_rs [OPTION]...\n\nMandatory:\n     --file=FILE                 path to .osu file to parse\n\nOptional:\n     --alg=ALG                   calculation alg to use (`classic` or `default`)\n     --mods=MODS                 integer sum of all mod values to apply (`2`: EZ|`8`: HD|`16`: HR|`64`: DT|`256`: HT)\n     --is-dir=TYPE               set FILE to DIR or SUBDIR (recursive) and parse all .osu files in (DIR|SUBDIR)\n     --output-type=TYPE          output stream and type (stdout|file-txt|file-csv)\n     --out=FILE                  set output FILE (output-type must be file-txt or file-csv)\n"); return }
            "--file" => { osu_filepath = safe_get_string(split, 1) },
            "--alg" => { alg = safe_get_string(split, 1) },
            "--mods" => { mod_int = safe_parse_i32(safe_get_string(split, 1)) },
            "--is-dir" => { is_dir = safe_get_string(split, 1) },
            "--out" => { output_file_string = safe_get_string(split, 1) },
            "--output-type" => { output_type = safe_get_string(split, 1) },
            _ => { print!("osu!Skills rs: unknown option {}\nUsage: osu_skills_rs [OPTION]...\n\nTry `osu_skills_rs --help` for more options.\n", split[0]); return }
        }

        i += 1;
    }

    if osu_filepath.len() == 0 {
        print!("osu!Skills rs: missing .osu file path\nUsage: osu_skills_rs [OPTION]...\n\nTry `osu_skills_rs --help` for more options.\n");
        return;
    }

    let mut files: Vec<std::path::PathBuf> = Default::default();

    match &is_dir as &str {
        "dir" => {
            let paths = match fs::read_dir(osu_filepath) {
                Ok(ok) => ok,
                Err(_) => return
            };
            for path in paths {
                let unwrapped_path = path.unwrap().path();
                if fs::metadata(unwrapped_path.clone()).unwrap().is_file() {
                    files.push(unwrapped_path);
                }
            }
        },
        "subdir" => { print!("osu!Skills rs: subdir option currently unimplemented.\n"); return },
        _ => files.push(std::path::Path::new(&osu_filepath).to_path_buf())
    };

    match &output_type.to_lowercase() as &str {
        "stdout" => { output_stdout(mod_int, alg, files) },
        "file-txt" => { output_file_txt(mod_int, alg, files, output_file_string) },
        "file-csv" => { output_file_csv(mod_int, alg, files, output_file_string) },
        _ => {}
    }
}

fn output_stdout(mod_int: i32, alg: String, files: Vec<std::path::PathBuf>) {
    for osu_filepath in files {
        let beatmap: structs::Beatmap = match &alg.to_lowercase() as &str {
            "classic" => classic_process_beatmap(osu_filepath, mod_int),
            _ => process_beatmap(osu_filepath, mod_int)
        };

        if beatmap.skills != structs::Beatmap::default().skills {
            let formatted_string: String = match &alg as &str {
                "classic" => format!("BeatmapsetID: {}, BeatmapID: {}, Stamina: {}, Tenacity: {}, Agility: {}, Accuracy: {}, Precision: {}, Reaction: {}, Memory: {}\n", beatmap.beatmap_id, beatmap.beatmap_set_id, beatmap.skills.stamina, beatmap.skills.tenacity, beatmap.skills.agility, beatmap.skills.accuracy, beatmap.skills.precision, beatmap.skills.reaction, beatmap.skills.memory),
                _ => format!("BeatmapsetID: {}, BeatmapID: {}, Stamina: {}, Streams: {}, Aim: {}, Accuracy: {}, Precision: {}, Reaction: {}, Flashlight: {}\n", beatmap.beatmap_id, beatmap.beatmap_set_id, beatmap.skills.stamina, beatmap.skills.tenacity, beatmap.skills.agility, beatmap.skills.accuracy, beatmap.skills.precision, beatmap.skills.reaction, beatmap.skills.memory)
            };

            print!("{}", formatted_string);
        }
    }
}

fn output_file_txt(mod_int: i32, alg: String, files: Vec<std::path::PathBuf>, output_file_string: String) {
    let mut output_file = match fs::OpenOptions::new().create(true).write(true).truncate(true).open(output_file_string) {
        Ok(ok) => ok,
        Err(error) => panic!("osu!Skills rs: couldn't open file. Error: {}", error)
    };

    for osu_filepath in files {
        let beatmap: structs::Beatmap = match &alg.to_lowercase() as &str {
            "classic" => classic_process_beatmap(osu_filepath, mod_int),
            _ => process_beatmap(osu_filepath, mod_int)
        };
        if beatmap.skills != structs::Beatmap::default().skills {
            let formatted_string: String = match &alg as &str {
                "classic" => format!("BeatmapID: {}, BeatmapsetID: {}, Md5: {}, Stamina: {}, Tenacity: {}, Agility: {}, Accuracy: {}, Precision: {}, Reaction: {}, Memory: {}\n", beatmap.beatmap_id, beatmap.beatmap_set_id, beatmap.beatmap_md5, beatmap.skills.stamina, beatmap.skills.tenacity, beatmap.skills.agility, beatmap.skills.accuracy, beatmap.skills.precision, beatmap.skills.reaction, beatmap.skills.memory),
                _ => format!("BeatmapsetID: {}, BeatmapID: {}, Md5: {}, Stamina: {}, Streams: {}, Aim: {}, Accuracy: {}, Precision: {}, Reaction: {}, Flashlight: {}\n", beatmap.beatmap_id, beatmap.beatmap_set_id, beatmap.beatmap_md5, beatmap.skills.stamina, beatmap.skills.tenacity, beatmap.skills.agility, beatmap.skills.accuracy, beatmap.skills.precision, beatmap.skills.reaction, beatmap.skills.memory)
            };

            match output_file.write(formatted_string.as_bytes()) {
                Ok(_) => (),
                Err(error) => println!("osu!Skills rs: failed to write file. Error: {}\n\n", error)
            };
        }
    }
}

fn output_file_csv(mod_int: i32, alg: String, files: Vec<std::path::PathBuf>, output_file_string: String) {
    let mut output_file = match fs::OpenOptions::new().create(true).write(true).truncate(true).open(output_file_string) {
        Ok(ok) => ok,
        Err(error) => panic!("osu!Skills rs: couldn't open file. Error: {}", error)
    };

    let header: &str = match &alg as &str {
        "classic" => "BeatmapID,BeatmapsetID,Md5,Stamina,Tenacity,Agility,Accuracy,Precision,Reaction,Memory\n",
        _ => "BeatmapID,BeatmapsetID,Md5,Stamina,Streams,Aim,Accuracy,Precision,Reaction,Flashlight\n"
    };

    match output_file.write(header.as_bytes()) {
        Ok(_) => (),
        Err(error) => println!("osu!Skills rs: failed to write file. Error: {}\n\n", error)
    };

    for osu_filepath in files {
        let beatmap: structs::Beatmap = match &alg.to_lowercase() as &str {
            "classic" => classic_process_beatmap(osu_filepath, mod_int),
            _ => process_beatmap(osu_filepath, mod_int)
        };
        if beatmap.skills != structs::Beatmap::default().skills {
            let formatted_string: String = format!("{},{},{},{},{},{},{},{},{},{}\n", beatmap.beatmap_id, beatmap.beatmap_set_id, beatmap.beatmap_md5, beatmap.skills.stamina, beatmap.skills.tenacity, beatmap.skills.agility, beatmap.skills.accuracy, beatmap.skills.precision, beatmap.skills.reaction, beatmap.skills.memory);
            
            match output_file.write(formatted_string.as_bytes()) {
                Ok(_) => (),
                Err(error) => println!("osu!Skills rs: failed to write file. Error: {}\n\n", error)
            };
        }
    }
}

fn process_beatmap(osu_filepath: std::path::PathBuf, mod_int: i32) -> structs::Beatmap {
    let mut beatmap: structs::Beatmap = osu_parser::parse_beatmap(osu_filepath);
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

fn classic_process_beatmap(osu_filepath: std::path::PathBuf, mod_int: i32) -> structs::Beatmap {
    let mut beatmap: structs::Beatmap = osu_parser::parse_beatmap(osu_filepath);
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

fn safe_parse_i32(input: String) -> i32 {
    let output = match input.parse::<i32>() {
        Ok(ok) => ok,
        Err(error) => { print!("osu!Skills rs: failed to parse --mods. Error: {}: `{}`\n\n", error, input); 0 }
    };
    return output;
}

fn safe_get_string(input: Vec<&str>, index: usize) -> String {
    let output = match input.get(index) {
        Some(some) => some.to_string(),
        None => "".to_string()
    };
    return output;
}