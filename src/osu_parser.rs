use std::io::BufRead;
use crate::{structs, pair_structs, algs::md5};

pub fn parse_beatmap(file_path: std::path::PathBuf) -> structs::Beatmap {
    let mut found: Found = Found::FoundNone;
    let mut is_header: bool;
    let mut beatmap_data: structs::Beatmap = Default::default();

    let file_data: Vec<u8> = match std::fs::read(&file_path) {
        Ok(ok) => ok,
        Err(_) => vec![]
    };

    if file_data.len() >= 3 {
        let magic_number = file_data[0..3].to_vec();
        if magic_number != vec![0xEF, 0xBB, 0xBF] /* UTF-8 */ && magic_number != vec![0x6F, 0x73, 0x75] /* osu */ {
            return beatmap_data;
        }
    }

    beatmap_data.beatmap_md5 = format!("{:x}", md5::compute(file_data));

    let file = match std::fs::File::open(&file_path) {
        Ok(x) => x,
        Err(_) => return beatmap_data
    };

    let reader = std::io::BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line_unwrap = match line {
            Ok(ok) => ok,
            Err(error) => { print!("osu!Skills rs: failed to parse .osu file. Error: {}. Path: `{}`\n\n", error, file_path.display().to_string()); return beatmap_data }
        };
        
        if beatmap_data.format == "" {
            beatmap_data.format = line_unwrap;
            continue;
        }

        match &line_unwrap as &str {
            "[General]" => { found = Found::FoundGeneral; is_header = true; },
            "[Editor]" => { found = Found::FoundEditor; is_header = true; },
            "[Metadata]" => { found = Found::FoundMetadata; is_header = true; },
            "[Difficulty]" => { found = Found::FoundDifficulty; is_header = true; },
            "[Events]" => { found = Found::FoundEvents; is_header = true; },
            "[TimingPoints]" => { found = Found::FoundTimingPoints; is_header = true; },
            "[Colours]" => { found = Found::FoundColours; is_header = true; },
            "[HitObjects]" => { found = Found::FoundHitobjects; is_header = true; },
            _ => { is_header = false; }
        }

        if !is_header {
            match found {
                Found::FoundGeneral => {beatmap_data = general_parser(beatmap_data, line_unwrap)},
                Found::FoundEditor => {},
                Found::FoundMetadata => {beatmap_data = metadata_parser(beatmap_data, line_unwrap)},
                Found::FoundDifficulty => {beatmap_data = difficulty_parser(beatmap_data, line_unwrap)},
                Found::FoundEvents => {},
                Found::FoundTimingPoints => {beatmap_data = timing_points_parser(beatmap_data, line_unwrap)},
                Found::FoundColours => {},
                Found::FoundHitobjects => {beatmap_data = hit_objects_parser(beatmap_data, line_unwrap)},
                Found::FoundNone => {},
            };
        }
    }

    return beatmap_data;
}

fn general_parser(mut beatmap: structs::Beatmap, line: String) -> structs::Beatmap {
    let split: Vec<&str> = trim_str_vec(line.split(":").collect());

    if split.len() >= 2 {
        match split[0] {
            "Mode" => beatmap.mode = safe_parse_i32(split[1]),
            _ => ()
        }
    }

    return beatmap;
}

fn metadata_parser(mut beatmap: structs::Beatmap, line: String) -> structs::Beatmap {
    let split: Vec<&str> = trim_str_vec(line.split(":").collect());

    if split.len() >= 2 {
        match split[0] {
            "Title" => beatmap.title = split[1].to_string(),
            "TitleUnicode" => beatmap.title_unicode = split[1].to_string(),
            "Artist" => beatmap.artist = split[1].to_string(),
            "ArtistUnicode" => beatmap.artist_unicode = split[1].to_string(),
            "Creator" => beatmap.creator = split[1].to_string(),
            "Version" => beatmap.version = split[1].to_string(),
            "Source" => beatmap.source = split[1].to_string(),
            "Tags" => beatmap.tags = split[1].to_string(),
            "BeatmapID" => beatmap.beatmap_id = split[1].to_string(),
            "BeatmapSetID" => beatmap.beatmap_set_id = split[1].to_string(),
            _ => ()
        }
    }
    return beatmap;
}

fn difficulty_parser(mut beatmap: structs::Beatmap, line: String) -> structs::Beatmap {
    let split: Vec<&str> = trim_str_vec(line.split(":").collect());

    fn osu_file_format_3_7(split: Vec<&str>, mut beatmap: structs::Beatmap) -> structs::Beatmap {
        match split[0] {
            "HPDrainRate" => beatmap.hp = safe_parse_f64(split[1]),
            "CircleSize" => beatmap.cs = safe_parse_f64(split[1]),
            "OverallDifficulty" => { beatmap.od = safe_parse_f64(split[1]); beatmap.ar = safe_parse_f64(split[1]); },
            "SliderMultiplier" => beatmap.sm = safe_parse_f64(split[1]),
            "SliderTickRate" => beatmap.st = safe_parse_f64(split[1]),
            _ => ()
        }
        return beatmap;
    }

    fn osu_file_format_8_14(split: Vec<&str>, mut beatmap: structs::Beatmap) -> structs::Beatmap {
        match split[0] {
            "HPDrainRate" => beatmap.hp = safe_parse_f64(split[1]),
            "CircleSize" => beatmap.cs = safe_parse_f64(split[1]),
            "OverallDifficulty" => beatmap.od = safe_parse_f64(split[1]), 
            "ApproachRate" => beatmap.ar = safe_parse_f64(split[1]),
            "SliderMultiplier" => beatmap.sm = safe_parse_f64(split[1]),
            "SliderTickRate" => beatmap.st = safe_parse_f64(split[1]),
            _ => ()
        }
        return beatmap;
    }

    if split.len() >= 2 {
        match &beatmap.format as &str {
            "osu file format v3" => beatmap = osu_file_format_3_7(split, beatmap),
            "osu file format v4" => beatmap = osu_file_format_3_7(split, beatmap),
            "osu file format v5" => beatmap = osu_file_format_3_7(split, beatmap),
            "osu file format v6" => beatmap = osu_file_format_3_7(split, beatmap),
            "osu file format v7" => beatmap = osu_file_format_3_7(split, beatmap),
            "osu file format v8" => beatmap = osu_file_format_8_14(split, beatmap),
            "osu file format v9" => beatmap = osu_file_format_8_14(split, beatmap),
            "osu file format v10" => beatmap = osu_file_format_8_14(split, beatmap),
            "osu file format v11" => beatmap = osu_file_format_8_14(split, beatmap),
            "osu file format v12" => beatmap = osu_file_format_8_14(split, beatmap),
            "osu file format v13" => beatmap = osu_file_format_8_14(split, beatmap),
            "osu file format v14" => beatmap = osu_file_format_8_14(split, beatmap),
            _ => beatmap = osu_file_format_8_14(split, beatmap),
        }
    }
    return beatmap;
}

fn timing_points_parser(mut beatmap: structs::Beatmap, line: String) -> structs::Beatmap {
    let split: Vec<&str> = trim_str_vec(line.split(",").collect());

    let mut timing_point: structs::TimingPoint = Default::default();

    match split.len() {
        2 => { /* osu file format v3 */
            timing_point.offset = safe_parse_f64(split[0]);
            timing_point.beat_interval = safe_parse_f64(split[1]);
            timing_point.meter = 4.0;
            beatmap.timing_points.push(timing_point);
        },
        5|6 => { /* osu file format v4-5_1 */
            timing_point.offset = safe_parse_f64(split[0]);
            timing_point.beat_interval = safe_parse_f64(split[1]);
            timing_point.meter = safe_parse_f64(split[2]);
            beatmap.timing_points.push(timing_point);
        },
        7 => { /* osu file format v5_2 */
            timing_point.offset = safe_parse_f64(split[0]);
            timing_point.beat_interval = safe_parse_f64(split[1]);
            timing_point.meter = safe_parse_f64(split[2]);
            timing_point.inherited = safe_parse_f64(split[6]) == 0.0;

            //fixes parsing for some aspire maps that can cause heavy lag and oom panics
            if timing_point.inherited == true && beatmap.timing_points.len() == 0 {
                let placeholder_time: structs::TimingPoint = Default::default();
                beatmap.timing_points.push(placeholder_time);
            }

            beatmap.timing_points.push(timing_point);
        },
        8 => { /* osu file format v6-14 */
            timing_point.offset = safe_parse_f64(split[0]);
            timing_point.beat_interval = safe_parse_f64(split[1]);
            timing_point.meter = safe_parse_f64(split[2]);
            timing_point.inherited = safe_parse_f64(split[6]) == 0.0;

            //fixes parsing for some aspire maps that can cause heavy lag and oom panics
            if timing_point.inherited == true && beatmap.timing_points.len() == 0 {
                let placeholder_time: structs::TimingPoint = Default::default();
                beatmap.timing_points.push(placeholder_time);
            }

            beatmap.timing_points.push(timing_point);
        },
        _ => (),
    }
    return beatmap;
}

fn hit_objects_parser(mut beatmap: structs::Beatmap, line: String) -> structs::Beatmap {
    let split: Vec<&str> = trim_str_vec(line.split(",").collect());

    let mut hit_object: structs::HitObject = Default::default();

    if split.len() >= 5 {
        hit_object.pixel_length = 0.0;
        hit_object.repeat = 1;
        hit_object.ncurve = 0;
        hit_object.to_repeat_time = 0;

        hit_object.pos.x = safe_parse_f64(split[0]);
        hit_object.pos.y = safe_parse_f64(split[1]);
        hit_object.time = safe_parse_i64(split[2]);
        hit_object.hit_object_type = safe_parse_i32(split[3]);
        hit_object.end_time = hit_object.time;

        let match_hit_object_type = hit_object_type_checker(hit_object.hit_object_type);

        match match_hit_object_type {
            structs::HitObjectType::Normal => {
                hit_object.end_point = hit_object.pos;
                beatmap.hit_objects.push(hit_object);
            },
            structs::HitObjectType::Slider => {
                let slider_split: Vec<&str> = trim_str_vec(split[5].split("|").collect());
                
                //removes some types of aspire sliders and sliders with slidertick art that can cause heavy lag and oom panics
                if slider_split.len() > 200 {
                    return beatmap;
                }

                hit_object.curve_type = safe_parse_curve_type(slider_split[0]);

                let mut i: usize = 1;
                while i < slider_split.len() {
                    let curve_split: Vec<&str> = trim_str_vec(slider_split[i].split(":").collect());
                    let curve = pair_structs::Pairf64 {x: safe_parse_f64(curve_split[0]), y: safe_parse_f64(curve_split[1])};
                    hit_object.curves.push(curve);

                    i += 1;
                }

                //removes some types of aspire sliders that can cause heavy lag and oom panics
                hit_object.repeat = safe_parse_i32(split[6]);
                if hit_object.repeat > 1000 {
                    return beatmap;
                }
                hit_object.pixel_length = safe_parse_f64(split[7]);
                if hit_object.pixel_length > 1000.0 {
                    return beatmap;
                }

                //removes some types of aspire sliders that have negative lengths that cause oom panics
                if hit_object.pixel_length < 0.0 {
                    return beatmap;
                }

                beatmap.hit_objects.push(hit_object);
            },
            structs::HitObjectType::Spinner => {
                beatmap.spinners += 1;
            },
            _ => ()
        }
    }
    return beatmap;
}

fn trim_str_vec(mut input: Vec<&str>) -> Vec<&str> {
    
    let mut i: usize = 0;
    while i < input.len() {
        input[i] = input[i].trim();
        i += 1;
    }

    return input;
}

fn safe_parse_f64(input: &str) -> f64 {
    let output = match input.parse::<f64>() {
        Ok(ok) => ok,
        Err(error) => { print!("osu!Skills rs: failed to parse f64 in .osu file. Error: {}: `{}`\n\n", error, input); -1.0 }
    };
    //simple hack to stop aspire maps from causing overflows
    if output > f32::MAX as f64 || output < f32::MIN as f64 {
        return 0.0;
    }
    return output;
}

fn safe_parse_i32(input: &str) -> i32 {
    let output = match input.parse::<i32>() {
        Ok(ok) => ok,
        Err(error) => { print!("osu!Skills rs: failed to parse i32 in .osu file. Error: {}: `{}`\n\n", error, input); -1 }
    };
    //simple hack to stop aspire maps from causing overflows
    if output > i16::MAX as i32 || output < i16::MIN as i32 {
        return 0;
    }
    return output;
}

fn safe_parse_i64(input: &str) -> i64 {
    let output = match input.parse::<i64>() {
        Ok(ok) => ok,
        Err(error) => { print!("osu!Skills rs: failed to parse i64 in .osu file. Error: {}: `{}`\n\n", error, input); -1 }
    };
    //simple hack to stop aspire maps from causing overflows
    if output > i32::MAX as i64 || output < i32::MIN as i64 {
        return 0;
    }
    return output;
}

fn hit_object_type_checker(input: i32) -> structs::HitObjectType {
    if input & (structs::HitObjectType::Normal as i32) > 0 {
        return structs::HitObjectType::Normal;
    } else if input & (structs::HitObjectType::Slider as i32) > 0 {
        return structs::HitObjectType::Slider;
    } else if input & (structs::HitObjectType::Spinner as i32) > 0 {
        return structs::HitObjectType::Spinner;
    }
    return structs::HitObjectType::None;
}

fn safe_parse_curve_type(input: &str) -> structs::CurveType {
    let output = match input {
        "P" => structs::CurveType::PerfectCurve,
        "B" => structs::CurveType::BezierCurve,
        "L" => structs::CurveType::LinearCurve,
        "C" => structs::CurveType::CatmullCurve,
        _ => structs::CurveType::None
    };
    return output;
}

#[derive(PartialEq)]
enum Found {
    FoundNone,
    FoundGeneral,
    FoundEditor,
    FoundMetadata,
    FoundDifficulty,
    FoundEvents,
    FoundTimingPoints,
    FoundColours,
    FoundHitobjects
}