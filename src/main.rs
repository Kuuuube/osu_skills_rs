use std::fs;
use std::panic;

mod algs;
mod classic_skill_calculation;
mod skill_calculation;
mod structs;
mod pair_structs;
mod osu_parser;
mod output;
mod parser;

fn main() {
    panic::set_hook(Box::new(|_info| {
        print!("{_info}\n")
    }));

    let args: Vec<String> = std::env::args().collect();

    let mut input_filepath: String = Default::default();
    let mut alg: String = Default::default();
    let mut mod_int: i32 = Default::default();
    let mut is_dir: String = Default::default();
    let mut output_filepath: String = Default::default();
    let mut output_type: String = "stdout".to_string();
    let mut no_ext: bool = false;
    let mut parser_mode: bool = false;
    let mut parser_arg: String = Default::default();

    let mut i: usize = 1;
    while i < args.len() {
        let arg = match args.get(i) {
            Some(some) => some,
            None => ""
        };
        let split: Vec<&str> = arg.split("=").collect();
        match &split[0].to_lowercase() as &str {
            "--help" => { print!("
            osu!Skills rs\nUsage: osu_skills_rs [OPTION]...\n\nSkill Calculator:\n  Mandatory:\n     --in=FILE                   path to .osu file to parse\n\n  Optional:\n     --alg=ALG                   calculation alg to use (classic|default)\n     --mods=MODS                 integer sum of all mod values to apply (`2`: EZ|`8`: HD|`16`: HR|`64`: DT|`256`: HT)\n     --is-dir=TYPE               set FILE to DIR or SUBDIR (recursive) and parse all .osu files in (DIR|SUBDIR)\n     --output-type=TYPE          output stream and type (stdout|file-txt|file-csv)\n     --out=FILE                  set output FILE (output-type must be file-txt or file-csv)\n     --no-ext                    removes file extension check for .osu files\n\nSkill File Parser:\n  Mandatory:\n     --parser=ARGS               args for the parser in the following format:\n                                 collections separated by `;`, filters separated by `,`\n                                 fiters separated from values by `:`, min and max values separated by `-`\n                                 example: \"stamina:1-100,tenacity:100-200;precision:900-1000\"\n     --in=FILE                   input file\n     --out=FILE                  output file\n"); return }
            "--in" => { input_filepath = safe_get_string(split, 1) },
            "--alg" => { alg = safe_get_string(split, 1) },
            "--mods" => { mod_int = safe_parse_i32(safe_get_string(split, 1)) },
            "--is-dir" => { is_dir = safe_get_string(split, 1) },
            "--out" => { output_filepath = safe_get_string(split, 1) },
            "--output-type" => { output_type = safe_get_string(split, 1) },
            "--no-ext" => { no_ext = true },
            "--parser" => { parser_mode = true; parser_arg = safe_get_string(split, 1)},
            _ => { print!("osu!Skills rs: unknown option {}\nUsage: osu_skills_rs [OPTION]...\n\nTry `osu_skills_rs --help` for more options.\n", split[0]); return }
        }

        i += 1;
    }

    if parser_mode {
        parser::skill_file_parser(parser_arg, input_filepath, output_filepath);
        return;
    }

    if input_filepath.len() == 0 {
        print!("osu!Skills rs: missing .osu file path\nUsage: osu_skills_rs [OPTION]...\n\nTry `osu_skills_rs --help` for more options.\n");
        return;
    }

    let mut files: Vec<std::path::PathBuf> = Default::default();

    print!("Collecting filenames from `{}`\n", input_filepath);
    match &is_dir.to_lowercase() as &str {
        "dir" => {
            let paths = match fs::read_dir(input_filepath) {
                Ok(ok) => ok,
                Err(_) => return
            };
            for path in paths {
                let unwrapped_path = path.unwrap().path();
                let path_as_string = unwrapped_path.to_string_lossy().to_string();
                if !no_ext && path_as_string.len() > 3 {
                    if path_as_string.split(".").last().unwrap().to_lowercase() != "osu".to_string() {
                        continue;
                    }
                }
                if fs::metadata(unwrapped_path.clone()).unwrap().is_file() {
                    files.push(unwrapped_path);
                }
            }
        },
        "subdir" => {
            let mut dirs: Vec<std::path::PathBuf> = Default::default();
            let paths = match fs::read_dir(input_filepath) {
                Ok(ok) => ok,
                Err(_) => return
            };
            for path in paths {
                let unwrapped_path = path.unwrap().path();
                if fs::metadata(unwrapped_path.clone()).unwrap().is_file() {
                    let path_as_string = unwrapped_path.to_string_lossy().to_string();
                    if !no_ext && path_as_string.len() > 3 {
                        if path_as_string.split(".").last().unwrap().to_lowercase() != "osu".to_string() {
                            continue;
                        }
                    }
                    files.push(unwrapped_path);
                } else if fs::metadata(unwrapped_path.clone()).unwrap().is_dir() {
                    dirs.push(unwrapped_path);
                }
            }
            let mut i: usize = 0;
            while i < dirs.len() {
                let rec_paths = match fs::read_dir(dirs[i].clone()) {
                    Ok(ok) => ok,
                    Err(_) => return
                };

                for rec_path in rec_paths {
                    let unwrapped_rec_path = rec_path.unwrap().path();
                    if fs::metadata(unwrapped_rec_path.clone()).unwrap().is_file() {
                        let path_as_string = unwrapped_rec_path.to_string_lossy().to_string();
                        if !no_ext && path_as_string.len() > 3 {
                            if path_as_string.split(".").last().unwrap().to_lowercase() != "osu".to_string() {
                                continue;
                            }
                        }
                        files.push(unwrapped_rec_path);
                    } else if fs::metadata(unwrapped_rec_path.clone()).unwrap().is_dir() {
                        dirs.push(unwrapped_rec_path);
                    }
                }
                i += 1;
            }
        },
        _ => files.push(std::path::Path::new(&input_filepath).to_path_buf())
    };

    print!("Starting calculation of `{}` maps\n", files.len());
    match &output_type.to_lowercase() as &str {
        "stdout" => { output::output_stdout(mod_int, alg, files) },
        "file-txt" => { output::output_file_txt(mod_int, alg, files, output_filepath) },
        "file-csv" => { output::output_file_csv(mod_int, alg, files, output_filepath) },
        _ => {}
    }

    print!("Finished calculation of all maps to `{}`\n", output_type);
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