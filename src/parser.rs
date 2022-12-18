//arg format:
//"name:collect1,stamina:1-100,tenacity:100-200;name:collect2,agility:50-90;name:collect1,memory:200-300;precision:900-1000"
//^                                             ^                           ^                            ^
//collection 1                                  collection 2                collection 1                 collection 3

use std::io::{BufRead, Write};

pub fn skill_file_parser(arg: String, input_filepath: String, output_filepath: String) {
    let mut collections: Vec<Collection> = read_arg(arg);
    let beatmaps: Vec<BeatmapData> = read_maps(input_filepath);

    let mut i: usize = 0;
    while i < beatmaps.len() {
        let mut j: usize = 0;
        while j < collections.len() {
            if collections[j].collection_filter.stamina_max >= beatmaps[i].stamina && collections[j].collection_filter.stamina_min <= beatmaps[i].stamina &&
            collections[j].collection_filter.tenacity_max >= beatmaps[i].tenacity && collections[j].collection_filter.tenacity_min <= beatmaps[i].tenacity &&
            collections[j].collection_filter.agility_max >= beatmaps[i].agility && collections[j].collection_filter.agility_min <= beatmaps[i].agility &&
            collections[j].collection_filter.accuracy_max >= beatmaps[i].accuracy && collections[j].collection_filter.accuracy_min <= beatmaps[i].accuracy &&
            collections[j].collection_filter.precision_max >= beatmaps[i].precision && collections[j].collection_filter.precision_min <= beatmaps[i].precision &&
            collections[j].collection_filter.reaction_max >= beatmaps[i].reaction && collections[j].collection_filter.reaction_min <= beatmaps[i].reaction &&
            collections[j].collection_filter.memory_max >= beatmaps[i].memory && collections[j].collection_filter.memory_min <= beatmaps[i].memory {
                collections[j].beatmap_data.push(beatmaps[i].clone());
            }
            j += 1;
        }
        i += 1;
    }

    write_collectionconverter_csv(collections, output_filepath);

}

fn write_collectionconverter_csv(collections: Vec<Collection>, output_filepath: String) {
    //format: "CollectionName","MapId","MapSetId","Md5","PlayMode","ArtistRoman","ArtistUnicode","TitleRoman","TitleUnicode","DiffName","StarsNomod"

    let mut output_file = match std::fs::OpenOptions::new().create(true).write(true).truncate(true).open(output_filepath) {
        Ok(ok) => ok,
        Err(error) => panic!("osu!Skills rs: couldn't open file. Error: {}", error)
    };

    match output_file.write(("\"CollectionName\",\"MapId\",\"MapSetId\",\"Md5\",\"PlayMode\",\"ArtistRoman\",\"ArtistUnicode\",\"TitleRoman\",\"TitleUnicode\",\"DiffName\",\"StarsNomod\"\n").as_bytes()) {
        Ok(_) => (),
        Err(error) => println!("osu!Skills rs: failed to write file. Error: {}\n\n", error)
    };

    for collection in collections {
        let name: String;
            if collection.collection_name == String::default() {
                name = collection.default_name;
            } else {
                name = collection.collection_name;
            }

        for beatmap in collection.beatmap_data {
            let formatted_string: String = format!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n", name, beatmap.beatmap_id, beatmap.beatmap_set_id, beatmap.beatmap_md5, "Osu", beatmap.artist, beatmap.artist_unicode, beatmap.title, beatmap.title_unicode, beatmap.version, "-1");
            match output_file.write(formatted_string.as_bytes()) {
                Ok(_) => (),
                Err(error) => println!("osu!Skills rs: failed to write file. Error: {}\n\n", error)
            };
        }
    }

}

fn read_arg(arg: String) -> Vec<Collection> {
    let mut char_vec: Vec<char> = Default::default();
    let illegal_chars: Vec<char> = vec!['\"', '\''];
    for char in arg.chars() {
        if !illegal_chars.contains(&char) {
            char_vec.push(char);
        }
    }
    let arg_trimmed: String = char_vec.into_iter().collect();
    let args_per_collection: Vec<&str> = arg_trimmed.split(";").collect();

    let mut collections: Vec<Collection> = vec![Collection::default(); args_per_collection.len()];

    let mut i: usize = 0;
    while i < args_per_collection.len() {
        let args_per_type: Vec<&str> = args_per_collection[i].split(",").collect();
        
        for arg_type in args_per_type {
            let args_per_value: Vec<&str> = arg_type.split(":").collect();
            let value_1: String = safe_get_slice(&args_per_value, 1);
            let split_min_max: Vec<&str> = value_1.split("-").collect();
            match &args_per_value[0].to_lowercase() as &str {
                "name" => { collections[i].collection_name = safe_get_slice(&args_per_value, 1) },
                "stamina" => {
                    collections[i].collection_filter.stamina_min = safe_parse_f64(safe_get_slice(&split_min_max, 0));
                    collections[i].collection_filter.stamina_max = safe_parse_f64(safe_get_slice(&split_min_max, 1));
                    if collections[i].default_name == String::default() {
                        collections[i].default_name = format!("{}_{}-{}", args_per_value[0], collections[i].collection_filter.stamina_min, collections[i].collection_filter.stamina_max);
                    } else {
                        collections[i].default_name = format!("{}_{}_{}-{}", collections[i].default_name, args_per_value[0], collections[i].collection_filter.stamina_min, collections[i].collection_filter.stamina_max);
                    }
                },
                "tenacity"|"streams" => {
                    collections[i].collection_filter.tenacity_min = safe_parse_f64(safe_get_slice(&split_min_max, 0));
                    collections[i].collection_filter.tenacity_max = safe_parse_f64(safe_get_slice(&split_min_max, 1));
                    if collections[i].default_name == String::default() {
                        collections[i].default_name = format!("{}_{}-{}", args_per_value[0], collections[i].collection_filter.tenacity_min, collections[i].collection_filter.tenacity_max);
                    } else {
                        collections[i].default_name = format!("{}_{}_{}-{}", collections[i].default_name, args_per_value[0], collections[i].collection_filter.tenacity_min, collections[i].collection_filter.tenacity_max);
                    }
                },
                "agility"|"aim" => {
                    collections[i].collection_filter.agility_min = safe_parse_f64(safe_get_slice(&split_min_max, 0));
                    collections[i].collection_filter.agility_max = safe_parse_f64(safe_get_slice(&split_min_max, 1));
                    if collections[i].default_name == String::default() {
                        collections[i].default_name = format!("{}_{}-{}", args_per_value[0], collections[i].collection_filter.agility_min, collections[i].collection_filter.agility_max);
                    } else {
                        collections[i].default_name = format!("{}_{}_{}-{}", collections[i].default_name, args_per_value[0], collections[i].collection_filter.agility_min, collections[i].collection_filter.agility_max);
                    }
                },
                "accuracy" => {
                    collections[i].collection_filter.accuracy_min = safe_parse_f64(safe_get_slice(&split_min_max, 0));
                    collections[i].collection_filter.accuracy_max = safe_parse_f64(safe_get_slice(&split_min_max, 1));
                    if collections[i].default_name == String::default() {
                        collections[i].default_name = format!("{}_{}-{}", args_per_value[0], collections[i].collection_filter.accuracy_min, collections[i].collection_filter.accuracy_max);
                    } else {
                        collections[i].default_name = format!("{}_{}_{}-{}", collections[i].default_name, args_per_value[0], collections[i].collection_filter.accuracy_min, collections[i].collection_filter.accuracy_max);
                    }
                },
                "precision" => {
                    collections[i].collection_filter.precision_min = safe_parse_f64(safe_get_slice(&split_min_max, 0));
                    collections[i].collection_filter.precision_max = safe_parse_f64(safe_get_slice(&split_min_max, 1));
                    if collections[i].default_name == String::default() {
                        collections[i].default_name = format!("{}_{}-{}", args_per_value[0], collections[i].collection_filter.precision_min, collections[i].collection_filter.precision_max);
                    } else {
                        collections[i].default_name = format!("{}_{}_{}-{}", collections[i].default_name, args_per_value[0], collections[i].collection_filter.precision_min, collections[i].collection_filter.precision_max);
                    }
                },
                "reaction" => {
                    collections[i].collection_filter.reaction_min = safe_parse_f64(safe_get_slice(&split_min_max, 0));
                    collections[i].collection_filter.reaction_max = safe_parse_f64(safe_get_slice(&split_min_max, 1));
                    if collections[i].default_name == String::default() {
                        collections[i].default_name = format!("{}_{}-{}", args_per_value[0], collections[i].collection_filter.reaction_min, collections[i].collection_filter.reaction_max);
                    } else {
                        collections[i].default_name = format!("{}_{}_{}-{}", collections[i].default_name, args_per_value[0], collections[i].collection_filter.reaction_min, collections[i].collection_filter.reaction_max);
                    }
                },
                "memory"|"flashlight" => {
                    collections[i].collection_filter.memory_min = safe_parse_f64(safe_get_slice(&split_min_max, 0));
                    collections[i].collection_filter.memory_max = safe_parse_f64(safe_get_slice(&split_min_max, 1));
                    if collections[i].default_name == String::default() {
                        collections[i].default_name = format!("{}_{}-{}", args_per_value[0], collections[i].collection_filter.memory_min, collections[i].collection_filter.memory_max);
                    } else {
                        collections[i].default_name = format!("{}_{}_{}-{}", collections[i].default_name, args_per_value[0], collections[i].collection_filter.memory_min, collections[i].collection_filter.memory_max);
                    }
                },
                _ => {}
            }
        }
        i += 1;
    }
    
    return collections;
}

fn read_maps(input_filepath: String) -> Vec<BeatmapData> {
    let mut beatmaps_parsed: Vec<BeatmapData> = Default::default();

    let file = match std::fs::File::open(&input_filepath) {
        Ok(ok) => ok,
        Err(_) => return vec![BeatmapData::default()]
    };

    let reader = std::io::BufReader::new(file);

    let mut first_line: bool = true;
    for (_index, line) in reader.lines().enumerate() {
        let mut line_unwrap = match line {
            Ok(ok) => ok,
            Err(error) => { print!("osu!Skills rs: failed to parse file. Error: {}. Path: `{}`\n\n", error, input_filepath); return vec![BeatmapData::default()] }
        };

        if line_unwrap.contains(":") {
            //not csv, each line's formatting must be removed
            let line_headers: Vec<&str> = vec!["BeatmapID:","BeatmapsetID:","Md5:","Stamina:","Tenacity:","Agility:","Accuracy:","Precision:","Reaction:","Memory:","Streams:","Aim:","Flashlight:"];
            for header in line_headers {
                line_unwrap = line_unwrap.replace(header, "");
            }

            let mut char_vec: Vec<char> = Default::default();
            for char in line_unwrap.chars() {
                if char != ' ' {
                    char_vec.push(char);
                }
            }
            line_unwrap = char_vec.into_iter().collect();

        } else if first_line == true {
            //is csv, first line must be removed
            first_line = false;
            continue;
        }

        let mut chars_vec: Vec<char> = Default::default();
        let line_chars: Vec<char> = line_unwrap.chars().collect();
        let mut line_split: Vec<String> = Default::default();
        let mut found_quote: bool = false;
        let mut i: usize = 0;
        while i < line_chars.len() {
            let current_char = safe_get_char(&line_chars, i);
            let next_char = safe_get_char(&line_chars, i + 1);
            match (current_char, next_char, found_quote) {
                (',', _, false) => {
                    line_split.push(chars_vec.into_iter().collect());
                    chars_vec = Default::default();
                },
                ('"', '"', true) => {
                    chars_vec.push(current_char);
                    i += 1;
                },
                ('"', _, false) => {
                    found_quote = true;
                },
                ('"', _, true) => {
                    found_quote = false;
                },
                _ => { chars_vec.push(current_char) }
            }
            i += 1;
            if i == line_chars.len() {
                line_split.push(chars_vec.into_iter().collect());
                chars_vec = Default::default();
            }
        }

        let beatmap: BeatmapData = BeatmapData {
            beatmap_id: safe_get_string(&line_split, 0),
            beatmap_set_id: safe_get_string(&line_split, 1),
            beatmap_md5: safe_get_string(&line_split, 2),
            stamina: safe_parse_f64(safe_get_string(&line_split, 3)),
            tenacity: safe_parse_f64(safe_get_string(&line_split, 4)),
            agility: safe_parse_f64(safe_get_string(&line_split, 5)),
            accuracy: safe_parse_f64(safe_get_string(&line_split, 6)),
            precision: safe_parse_f64(safe_get_string(&line_split, 7)),
            reaction: safe_parse_f64(safe_get_string(&line_split, 8)),
            memory: safe_parse_f64(safe_get_string(&line_split, 9)),
            //mods - index 10
            //mode - index 11
            artist: safe_get_string(&line_split, 12),
            artist_unicode: safe_get_string(&line_split, 13),
            title: safe_get_string(&line_split, 14),
            title_unicode: safe_get_string(&line_split, 15),
            version: safe_get_string(&line_split, 16),
        };
        
        beatmaps_parsed.push(beatmap);
    }

    return beatmaps_parsed;
}

#[derive(Default, Clone)]
struct BeatmapData {
    pub beatmap_id: String,
    pub beatmap_set_id: String,
    pub beatmap_md5: String,
    pub title: String,
    pub title_unicode: String,
    pub artist: String,
    pub artist_unicode: String,
    pub version: String,

    pub stamina: f64,
    pub tenacity: f64,
    pub agility: f64,
    pub precision: f64,
    pub memory: f64,
    pub accuracy: f64,
    pub reaction: f64
}

#[derive(Default, Clone)]
struct Collection {
    pub collection_name: String,
    pub default_name: String,
    pub beatmap_data: Vec<BeatmapData>,
    pub collection_filter: Filter
}

#[derive(Clone)]
struct Filter {
    pub stamina_min: f64,
    pub tenacity_min: f64,
    pub agility_min: f64,
    pub precision_min: f64,
    pub memory_min: f64,
    pub accuracy_min: f64,
    pub reaction_min: f64,

    pub stamina_max: f64,
    pub tenacity_max: f64,
    pub agility_max: f64,
    pub precision_max: f64,
    pub memory_max: f64,
    pub accuracy_max: f64,
    pub reaction_max: f64
}

impl Default for Filter {
    fn default() -> Self {
        Filter { 
            stamina_min: (f64::MIN), tenacity_min: (f64::MIN), agility_min: (f64::MIN), precision_min: (f64::MIN), memory_min: (f64::MIN), accuracy_min: (f64::MIN), reaction_min: (f64::MIN), 
            stamina_max: (f64::MAX), tenacity_max: (f64::MAX), agility_max: (f64::MAX), precision_max: (f64::MAX), memory_max: (f64::MAX), accuracy_max: (f64::MAX), reaction_max: (f64::MAX) 
        }
    }
}

fn safe_get_slice(input: &Vec<&str>, index: usize) -> String {
    let output = match input.get(index) {
        Some(some) => some.to_string(),
        None => "".to_string()
    };
    return output;
}

fn safe_get_string(input: &Vec<String>, index: usize) -> String {
    let output = match input.get(index) {
        Some(some) => some.to_string(),
        None => "".to_string()
    };
    return output;
}

fn safe_get_char(input: &Vec<char>, index: usize) -> char {
    let output = match input.get(index) {
        Some(some) => some.to_owned(),
        None => char::default().to_owned()
    };
    return output;
}

fn safe_parse_f64(input: String) -> f64 {
    let output = match input.parse::<f64>() {
        Ok(ok) => ok,
        Err(error) => { print!("osu!Skills rs: failed to parse f64. Error: {}: `{}`\n\n", error, input); 0.0 }
    };
    return output;
}