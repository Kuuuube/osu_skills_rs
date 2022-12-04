use std::io::BufRead;
use crate::structs;

pub fn parse_beatmap(file_path: &str) -> structs::Beatmap {
    let mut found: Found;
    let mut beatmap_data: structs::Beatmap = Default::default();
    
    let file = match std::fs::File::open(file_path) {
        Ok(x) => x,
        Err(_) => return beatmap_data
    };

    let reader = std::io::BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line_unwrap = line.unwrap();
        println!("{}. {}", index + 1, line_unwrap);
        let mut is_header: bool = false;

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
            _ => { found = Found::FoundNone; is_header = false; }
        }

        if !is_header {
            match found {
                FoundGeneral => (),
                FoundEditor => (),
                FoundMetadata => (),
                FoundDifficulty => (),
                FoundEvents => (),
                FoundTimingPoints => (),
                FoundColours => (),
                FoundHitobjects => (),
                FoundNone => ()
            }
        }
    }

    return beatmap_data;
}

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