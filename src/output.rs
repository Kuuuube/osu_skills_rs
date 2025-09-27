use std::panic;
use std::{fs, io::Write};

use crate::skill_calculation;
use crate::structs::{self, CalculationAlgorithm, OutputType};
use crate::vars::{SKILL_CALCULATION_VARS, SKILL_CALCULATION_VARS_REBALANCE_1};
use crate::{calculation_utils, osu_parser};
use crate::{classic_skill_calculation, rebalance_1};

pub fn output(
    mod_int: i32,
    alg: CalculationAlgorithm,
    output_type: OutputType,
    files: Vec<std::path::PathBuf>,
    output_file_string: String,
) {
    match output_type {
        OutputType::Stdout => {
            for osu_filepath in files {
                let beatmap: structs::Beatmap = process_beatmap(osu_filepath, mod_int, alg);

                if beatmap.skills != structs::Beatmap::default().skills {
                    let formatted_string: String = match alg {
                        CalculationAlgorithm::Classic => format!("BeatmapsetID: {}, BeatmapID: {}, Stamina: {}, Tenacity: {}, Agility: {}, Accuracy: {}, Precision: {}, Reaction: {}, Memory: {}\n", beatmap.beatmap_id, beatmap.beatmap_set_id, beatmap.skills.stamina, beatmap.skills.tenacity, beatmap.skills.agility, beatmap.skills.accuracy, beatmap.skills.precision, beatmap.skills.reaction, beatmap.skills.memory),
                        _ => format!("BeatmapsetID: {}, BeatmapID: {}, Stamina: {}, Streams: {}, Aim: {}, Accuracy: {}, Precision: {}, Reaction: {}, Flashlight: {}\n", beatmap.beatmap_id, beatmap.beatmap_set_id, beatmap.skills.stamina, beatmap.skills.tenacity, beatmap.skills.agility, beatmap.skills.accuracy, beatmap.skills.precision, beatmap.skills.reaction, beatmap.skills.memory)
                    };

                    print!("{}", formatted_string);
                }
            }
        }
        OutputType::Txt => {
            let mut output_file = match fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(output_file_string)
            {
                Ok(ok) => ok,
                Err(error) => panic!("osu!Skills rs: couldn't open file. Error: {}", error),
            };

            for osu_filepath in files {
                let beatmap: structs::Beatmap = process_beatmap(osu_filepath, mod_int, alg);
                if beatmap.skills != structs::Beatmap::default().skills {
                    let formatted_string: String = match alg {
                        CalculationAlgorithm::Classic => format!("BeatmapID: {}, BeatmapsetID: {}, Md5: {}, Stamina: {}, Tenacity: {}, Agility: {}, Accuracy: {}, Precision: {}, Reaction: {}, Memory: {}\n", beatmap.beatmap_id, beatmap.beatmap_set_id, beatmap.beatmap_md5, beatmap.skills.stamina, beatmap.skills.tenacity, beatmap.skills.agility, beatmap.skills.accuracy, beatmap.skills.precision, beatmap.skills.reaction, beatmap.skills.memory),
                        _ => format!("BeatmapsetID: {}, BeatmapID: {}, Md5: {}, Stamina: {}, Streams: {}, Aim: {}, Accuracy: {}, Precision: {}, Reaction: {}, Flashlight: {}\n", beatmap.beatmap_id, beatmap.beatmap_set_id, beatmap.beatmap_md5, beatmap.skills.stamina, beatmap.skills.tenacity, beatmap.skills.agility, beatmap.skills.accuracy, beatmap.skills.precision, beatmap.skills.reaction, beatmap.skills.memory)
                    };

                    match output_file.write(formatted_string.as_bytes()) {
                        Ok(_) => (),
                        Err(error) => {
                            println!("osu!Skills rs: failed to write file. Error: {}\n\n", error)
                        }
                    };
                }
            }
        }
        OutputType::Csv => {
            let mut output_file = match fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(output_file_string)
            {
                Ok(ok) => ok,
                Err(error) => panic!("osu!Skills rs: couldn't open file. Error: {}", error),
            };

            let header: &str = match alg {
                CalculationAlgorithm::Classic => "\"BeatmapID\",\"BeatmapsetID\",\"Md5\",\"Stamina\",\"Tenacity\",Agility\",\"Accuracy\",\"Precision\",\"Reaction\",\"Memory\",\"Mods\",\"Mode\",\"Artist\",\"ArtistUnicode\",\"Title\",\"TitleUnicode\",\"Version\"\n",
                _ => "\"BeatmapID\",\"BeatmapsetID\",\"Md5\",\"Stamina\",\"Streams\",\"Aim\",\"Accuracy\",\"Precision\",\"Reaction\",\"Flashlight\",\"Mods\",\"Mode\",\"Artist\",\"ArtistUnicode\",\"Title\",\"TitleUnicode\",\"Version\"\n"
            };

            match output_file.write(header.as_bytes()) {
                Ok(_) => (),
                Err(error) => println!("osu!Skills rs: failed to write file. Error: {}\n\n", error),
            };

            for osu_filepath in files {
                let beatmap: structs::Beatmap = process_beatmap(osu_filepath, mod_int, alg);
                if beatmap.skills != structs::Beatmap::default().skills {
                    let formatted_string: String = format!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n", beatmap.beatmap_id, beatmap.beatmap_set_id, beatmap.beatmap_md5, beatmap.skills.stamina, beatmap.skills.tenacity, beatmap.skills.agility, beatmap.skills.accuracy, beatmap.skills.precision, beatmap.skills.reaction, beatmap.skills.memory, beatmap.mods, "Osu", beatmap.artist, beatmap.artist_unicode, beatmap.title, beatmap.title_unicode, beatmap.version);

                    match output_file.write(formatted_string.as_bytes()) {
                        Ok(_) => (),
                        Err(error) => {
                            println!("osu!Skills rs: failed to write file. Error: {}\n\n", error)
                        }
                    };
                }
            }
        }
    }
}

pub fn process_beatmap(
    osu_filepath: std::path::PathBuf,
    mod_int: i32,
    alg: CalculationAlgorithm,
) -> structs::Beatmap {
    let result = panic::catch_unwind(|| {
        let mut beatmap: structs::Beatmap = osu_parser::parse_beatmap(osu_filepath.clone());
        beatmap.skill_calculation_vars = match alg {
            CalculationAlgorithm::Default | CalculationAlgorithm::Classic => SKILL_CALCULATION_VARS,
            CalculationAlgorithm::Rebalance1 => SKILL_CALCULATION_VARS_REBALANCE_1,
        };
        beatmap.mods = mod_int;

        if beatmap.hit_objects.len() >= 2 && beatmap.mode == 0 {
            beatmap = calculation_utils::apply_mods(beatmap);

            beatmap = skill_calculation::calculation::generic::prepare_timing_points(beatmap);
            beatmap = skill_calculation::calculation::slider::approximate_slider_points(beatmap);
            beatmap = skill_calculation::calculation::generic::bake_slider_data(beatmap);

            beatmap = skill_calculation::calculation::generic::prepare_aim_data(beatmap);
            beatmap = skill_calculation::calculation::generic::prepare_tap_data(beatmap);

            beatmap = skill_calculation::calculation::strains::calculate_aim_strains(beatmap);
            beatmap = skill_calculation::calculation::strains::calculate_tap_strains(beatmap);

            beatmap.skills.stamina =
                skill_calculation::calculation::stamina::calculate_stamina(&beatmap);
            beatmap.skills.tenacity =
                skill_calculation::calculation::tenacity::calculate_tenacity(&beatmap);
            beatmap.skills.agility =
                skill_calculation::calculation::agility::calculate_agility(&beatmap);

            match alg {
                CalculationAlgorithm::Default => {
                    beatmap.skills.accuracy =
                        skill_calculation::calculation::accuracy::calculate_accuracy(&beatmap);
                }
                CalculationAlgorithm::Classic => {
                    beatmap.skills.accuracy =
                        classic_skill_calculation::calculation::accuracy::calculate_accuracy(
                            &beatmap,
                        );
                }
                CalculationAlgorithm::Rebalance1 => {
                    beatmap.skills.accuracy =
                        rebalance_1::calculation::accuracy::calculate_accuracy(&beatmap);
                }
            }

            match alg {
                CalculationAlgorithm::Classic | CalculationAlgorithm::Rebalance1 => {
                    beatmap.skills.reaction =
                        classic_skill_calculation::calculation::reaction::calculate_reaction(
                            &beatmap,
                        );
                    beatmap.skills.precision =
                        classic_skill_calculation::calculation::precision::calculate_precision(
                            &beatmap,
                        );
                    beatmap.skills.memory =
                        classic_skill_calculation::calculation::memory::calculate_memory(&beatmap);
                }
                _ => {
                    beatmap.skills.reaction =
                        skill_calculation::calculation::reaction::calculate_reaction(&beatmap);
                    beatmap.skills.precision =
                        skill_calculation::calculation::precision::calculate_precision(&beatmap);
                    beatmap.skills.memory =
                        skill_calculation::calculation::memory::calculate_memory(&beatmap);
                }
            }
        }
        return beatmap;
    });

    match result {
        Ok(ok) => return ok,
        Err(_) => {
            print!(
                "osu!Skills rs: failed to process map `{}`\n",
                osu_filepath.display().to_string()
            );
            return structs::Beatmap::default();
        }
    }
}
