use std::collections::HashMap;

use crate::structs::Skills;

#[test]
fn calculate_maps_default() {
    let expected_default_data = get_default_data();
    for osu_filepath in std::fs::read_dir("src/tests/testmaps").unwrap() {
        let filepath = osu_filepath.unwrap();
        let pathbuf = filepath.path();
        let filename = filepath.file_name().into_string().unwrap();
        let processed_beatmap = crate::output::process_beatmap(pathbuf, 0, crate::structs::CalculationAlgorithm::Default);
        assert!(*expected_default_data.get(&filename).unwrap() == processed_beatmap.skills);
    }
}

#[test]
fn calculate_maps_classic() {
    let expected_default_data = get_classic_data();
    for osu_filepath in std::fs::read_dir("src/tests/testmaps").unwrap() {
        let filepath = osu_filepath.unwrap();
        let pathbuf = filepath.path();
        let filename = filepath.file_name().into_string().unwrap();
        let processed_beatmap = crate::output::process_beatmap(pathbuf, 0, crate::structs::CalculationAlgorithm::Classic);
        assert!(*expected_default_data.get(&filename).unwrap() == processed_beatmap.skills);
    }
}

fn get_default_data() -> HashMap<String, Skills> {
    return HashMap::from([
        (
            "v4.osu".to_string(),
            Skills {
                stamina: 176.16576775841932,
                tenacity: 140.92325122626224,
                agility: 350.5245510773862,
                precision: 546.9365392945944,
                memory: 284.46232996693453,
                accuracy: 114.92656196479895,
                reaction: 131.78090251589242,
            },
        ),
        (
            "v6.osu".to_string(),
            Skills {
                stamina: 147.0282836790383,
                tenacity: 626.9147772161582,
                agility: 352.18152915187204,
                precision: 380.55519933735013,
                memory: 260.98208810344596,
                accuracy: 34.54115687227016,
                reaction: 108.28217969591446,
            },
        ),
        (
            "v8.osu".to_string(),
            Skills {
                stamina: 167.71225986813994,
                tenacity: 173.04490247887657,
                agility: 290.48231726801816,
                precision: 351.26523342492897,
                memory: 313.2980197451202,
                accuracy: 168.51320985019865,
                reaction: 216.59037258230052,
            },
        ),
        (
            "v13.osu".to_string(),
            Skills {
                stamina: 173.0772011186,
                tenacity: 626.9147772161582,
                agility: 373.7189566248563,
                precision: 191.06949880213207,
                memory: 382.4943440533611,
                accuracy: 82.42058878089013,
                reaction: 188.03154311405692,
            },
        ),
        (
            "v11.osu".to_string(),
            Skills {
                stamina: 120.69827452556522,
                tenacity: 48.267256654776354,
                agility: 299.1824351189411,
                precision: 227.6099116796273,
                memory: 151.13989204337966,
                accuracy: 38.8800665091224,
                reaction: 165.77799264498856,
            },
        ),
        (
            "v3.osu".to_string(),
            Skills {
                stamina: 115.3475215472077,
                tenacity: 139.28519391586053,
                agility: 246.43015788347833,
                precision: 209.66882636003643,
                memory: 139.20714779380998,
                accuracy: 49.55097527132471,
                reaction: 131.77927931172076,
            },
        ),
        (
            "v10.osu".to_string(),
            Skills {
                stamina: 18.463681165860347,
                tenacity: 626.9147772161582,
                agility: 202.38700033379715,
                precision: 108.30758087951872,
                memory: 0.0,
                accuracy: 0.0,
                reaction: 82.52950868921036,
            },
        ),
        (
            "v14.osu".to_string(),
            Skills {
                stamina: 463.1850614416184,
                tenacity: 508.40291043593504,
                agility: 1373.73341387273,
                precision: 320.0,
                memory: 829.5990418379729,
                accuracy: 433.9212189210708,
                reaction: 426.98476412851824,
            },
        ),
        (
            "v12.osu".to_string(),
            Skills {
                stamina: 388.7012531966348,
                tenacity: 272.30560909758174,
                agility: 472.72063233208667,
                precision: 274.4278826941878,
                memory: 574.7652986988444,
                accuracy: 632.8695343293567,
                reaction: 303.33320791173594,
            },
        ),
        (
            "v7.osu".to_string(),
            Skills {
                stamina: 389.27821085939286,
                tenacity: 486.3285928938134,
                agility: 431.03796562285726,
                precision: 264.448213514689,
                memory: 651.0522602352947,
                accuracy: 1896.435469189151,
                reaction: 108.44975733037401,
            },
        ),
        (
            "v5.osu".to_string(),
            Skills {
                stamina: 179.49853612935894,
                tenacity: 160.67530937571166,
                agility: 323.5710241174351,
                precision: 367.4552367402495,
                memory: 387.3171533017515,
                accuracy: 195.9269738180805,
                reaction: 165.43659219815865,
            },
        ),
        (
            "v9.osu".to_string(),
            Skills {
                stamina: 45.31959678674479,
                tenacity: 626.9147772161582,
                agility: 245.14886649064286,
                precision: 209.2015862101831,
                memory: 177.2708223937021,
                accuracy: 0.002865057307830196,
                reaction: 106.79992589807796,
            },
        ),
    ]);
}

fn get_classic_data() -> HashMap<String, Skills> {
    return HashMap::from([
        (
            "v4.osu".to_string(),
            Skills {
                stamina: 176.16576775841932,
                tenacity: 140.92325122626224,
                agility: 350.5245510773862,
                precision: 462.80032007711304,
                memory: 284.46232996693453,
                accuracy: 114.92656196479895,
                reaction: 131.78090251589242,
            },
        ),
        (
            "v6.osu".to_string(),
            Skills {
                stamina: 147.0282836790383,
                tenacity: 626.9147772161582,
                agility: 352.18152915187204,
                precision: 322.06721578405796,
                memory: 260.98208810344596,
                accuracy: 34.54115687227016,
                reaction: 108.28217969591446,
            },
        ),
        (
            "v8.osu".to_string(),
            Skills {
                stamina: 167.71225986813994,
                tenacity: 173.04490247887657,
                agility: 290.48231726801816,
                precision: 295.2087233643707,
                memory: 313.2980197451202,
                accuracy: 168.51320985019865,
                reaction: 216.59037258230052,
            },
        ),
        (
            "v13.osu".to_string(),
            Skills {
                stamina: 173.0772011186,
                tenacity: 626.9147772161582,
                agility: 373.7189566248563,
                precision: 119.04899914815111,
                memory: 351.0729224629957,
                accuracy: 82.42058878089013,
                reaction: 188.03154311388158,
            },
        ),
        (
            "v11.osu".to_string(),
            Skills {
                stamina: 120.69827452556522,
                tenacity: 48.267256654776354,
                agility: 299.1824351189411,
                precision: 191.49709910907885,
                memory: 151.13989204337966,
                accuracy: 38.8800665091224,
                reaction: 165.77799264498856,
            },
        ),
        (
            "v3.osu".to_string(),
            Skills {
                stamina: 115.3475215472077,
                tenacity: 139.28519391586053,
                agility: 246.43015788347833,
                precision: 175.09985094746898,
                memory: 139.20714779380998,
                accuracy: 49.55097527132471,
                reaction: 131.77927931172076,
            },
        ),
        (
            "v10.osu".to_string(),
            Skills {
                stamina: 18.463681165860347,
                tenacity: 626.9147772161582,
                agility: 202.38700033379715,
                precision: 89.72850249232434,
                memory: 0.0,
                accuracy: -0.0,
                reaction: 82.52950868921036,
            },
        ),
        (
            "v14.osu".to_string(),
            Skills {
                stamina: 463.1850614416184,
                tenacity: 508.40291043593504,
                agility: 1373.73341387273,
                precision: 362.4905668436316,
                memory: 829.5990418379729,
                accuracy: 421.68128341615466,
                reaction: 426.98476412851824,
            },
        ),
        (
            "v12.osu".to_string(),
            Skills {
                stamina: 388.7012531966348,
                tenacity: 272.30560909758174,
                agility: 472.72063233208667,
                precision: 234.5629439087768,
                memory: 574.7652986988444,
                accuracy: 632.8695343293567,
                reaction: 303.33320791173594,
            },
        ),
        (
            "v7.osu".to_string(),
            Skills {
                stamina: 389.27821085939286,
                tenacity: 486.3285928938134,
                agility: 431.03796562285726,
                precision: 225.3533338253617,
                memory: 651.0522602352947,
                accuracy: 1896.435469189151,
                reaction: 108.44975733037401,
            },
        ),
        (
            "v5.osu".to_string(),
            Skills {
                stamina: 179.49853612935894,
                tenacity: 160.67530937571166,
                agility: 323.5710241174351,
                precision: 310.04219912706,
                memory: 387.3171533017515,
                accuracy: 195.9269738180805,
                reaction: 165.43659219815865,
            },
        ),
        (
            "v9.osu".to_string(),
            Skills {
                stamina: 45.31959678674479,
                tenacity: 626.9147772161582,
                agility: 245.14886649064286,
                precision: 174.67372486433274,
                memory: 177.2708223937021,
                accuracy: 0.002865057307830196,
                reaction: 106.79992589807796,
            },
        ),
    ]);
}
