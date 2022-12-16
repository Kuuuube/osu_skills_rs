use std::{fs, io::Write};
use std::panic;

use crate::osu_parser;
use crate::structs;
use crate::classic_skill_calculation;
use crate::skill_calculation;

pub fn output_stdout(mod_int: i32, alg: String, files: Vec<std::path::PathBuf>) {
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

pub fn output_file_txt(mod_int: i32, alg: String, files: Vec<std::path::PathBuf>, output_file_string: String) {
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

pub fn output_file_csv(mod_int: i32, alg: String, files: Vec<std::path::PathBuf>, output_file_string: String) {
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
    let result = panic::catch_unwind(|| {
        let mut beatmap: structs::Beatmap = osu_parser::parse_beatmap(osu_filepath.clone());
        beatmap.mods = mod_int;

        if beatmap.hit_objects.len() >= 2 && beatmap.mode == 0 {
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
    });

    match result {
        Ok(ok) => return ok,
        Err(_) => { print!("osu!Skills rs: failed to process map `{}`\n", osu_filepath.display().to_string()); return structs::Beatmap::default()}
    }
}

fn classic_process_beatmap(osu_filepath: std::path::PathBuf, mod_int: i32) -> structs::Beatmap {
    let result = panic::catch_unwind(|| {
        let mut beatmap: structs::Beatmap = osu_parser::parse_beatmap(osu_filepath.clone());
        beatmap.mods = mod_int;

        if beatmap.hit_objects.len() >= 2 && beatmap.mode == 0 {
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
    });

    match result {
        Ok(ok) => return ok,
        Err(_) => { print!("osu!Skills rs: failed to process .osu file. Path: `{}`\n", osu_filepath.display().to_string()); return structs::Beatmap::default()}
    }
}