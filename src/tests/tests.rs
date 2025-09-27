use std::collections::HashMap;

use crate::structs::Skills;

#[test]
fn calculate_maps_default() {
    let expected_default_data = get_default_data();
    for osu_filepath in std::fs::read_dir("src/tests/testmaps").unwrap() {
        let filepath = osu_filepath.unwrap();
        let pathbuf = filepath.path();
        let filename = filepath.file_name().into_string().unwrap();
        if !filename.contains(".osu") {
            continue;
        }
        let processed_beatmap = crate::output::process_beatmap(
            pathbuf,
            0,
            crate::structs::CalculationAlgorithm::Default,
        );
        dbg!(&filename, &processed_beatmap.skills);
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
        if !filename.contains(".osu") {
            continue;
        }
        let processed_beatmap = crate::output::process_beatmap(
            pathbuf,
            0,
            crate::structs::CalculationAlgorithm::Classic,
        );
        dbg!(&filename, &processed_beatmap.skills);
        assert!(*expected_default_data.get(&filename).unwrap() == processed_beatmap.skills);
    }
}

#[test]
fn calculate_maps_rebalance_1() {
    let expected_default_data = get_rebalance_1_data();
    for osu_filepath in std::fs::read_dir("src/tests/testmaps").unwrap() {
        let filepath = osu_filepath.unwrap();
        let pathbuf = filepath.path();
        let filename = filepath.file_name().into_string().unwrap();
        if !filename.contains(".osu") {
            continue;
        }
        let processed_beatmap = crate::output::process_beatmap(
            pathbuf,
            0,
            crate::structs::CalculationAlgorithm::Rebalance1,
        );
        dbg!(&filename, &processed_beatmap.skills);
        assert!(*expected_default_data.get(&filename).unwrap() == processed_beatmap.skills);
    }
}

fn get_default_data() -> HashMap<String, Skills> {
    return HashMap::from([
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
            "v4.osu".to_string(),
            Skills {
                stamina: 176.16576775841932,
                tenacity: 142.95602750485276,
                agility: 350.5245510773862,
                precision: 546.9365392945944,
                memory: 284.46232996693453,
                accuracy: 114.92656196479895,
                reaction: 131.78090251589242,
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
    ]);
}

fn get_classic_data() -> HashMap<String, Skills> {
    let mut classic_data = get_default_data();

    if let Some(x) = classic_data.get_mut("v3.osu") {
        x.precision = 175.09985094746898;
        x.memory = 139.20714779380998;
        x.accuracy = 49.55097527132471;
        x.reaction = 131.77927931172076;
    }

    if let Some(x) = classic_data.get_mut("v4.osu") {
        x.precision = 462.80032007711304;
        x.memory = 284.46232996693453;
        x.accuracy = 114.92656196479895;
        x.reaction = 131.78090251589242;
    }

    if let Some(x) = classic_data.get_mut("v5.osu") {
        x.precision = 310.04219912706;
        x.memory = 387.3171533017515;
        x.accuracy = 195.9269738180805;
        x.reaction = 165.43659219815865;
    }

    if let Some(x) = classic_data.get_mut("v6.osu") {
        x.precision = 322.06721578405796;
        x.memory = 260.98208810344596;
        x.accuracy = 34.54115687227016;
        x.reaction = 108.28217969591446;
    }

    if let Some(x) = classic_data.get_mut("v7.osu") {
        x.precision = 225.3533338253617;
        x.memory = 651.0522602352947;
        x.accuracy = 1896.435469189151;
        x.reaction = 108.44975733037401;
    }

    if let Some(x) = classic_data.get_mut("v8.osu") {
        x.precision = 295.2087233643707;
        x.memory = 313.2980197451202;
        x.accuracy = 168.51320985019865;
        x.reaction = 216.59037258230052;
    }

    if let Some(x) = classic_data.get_mut("v9.osu") {
        x.precision = 174.67372486433274;
        x.memory = 177.2708223937021;
        x.accuracy = 0.002865057307830196;
        x.reaction = 106.79992589807796;
    }

    if let Some(x) = classic_data.get_mut("v10.osu") {
        x.precision = 89.72850249232434;
        x.memory = 0.0;
        x.accuracy = -0.0;
        x.reaction = 82.52950868921036;
    }

    if let Some(x) = classic_data.get_mut("v11.osu") {
        x.precision = 191.49709910907885;
        x.memory = 151.13989204337966;
        x.accuracy = 38.8800665091224;
        x.reaction = 165.77799264498856;
    }

    if let Some(x) = classic_data.get_mut("v12.osu") {
        x.precision = 234.5629439087768;
        x.memory = 574.7652986988444;
        x.accuracy = 632.8695343293567;
        x.reaction = 303.33320791173594;
    }

    if let Some(x) = classic_data.get_mut("v13.osu") {
        x.precision = 119.04899914815111;
        x.memory = 351.0729224629957;
        x.accuracy = 82.42058878089013;
        x.reaction = 188.03154311388158;
    }

    if let Some(x) = classic_data.get_mut("v14.osu") {
        x.precision = 362.4905668436316;
        x.memory = 829.5990418379729;
        x.accuracy = 421.68128341615466;
        x.reaction = 426.98476412851824;
    }

    return classic_data;
}

fn get_rebalance_1_data() -> HashMap<String, Skills> {
    return HashMap::from([
        (
            "v3.osu".to_string(),
            Skills {
                stamina: 188.09170758989,
                tenacity: 139.28519391586053,
                agility: 209.14886636067754,
                precision: 310.81301545739717,
                memory: 233.86574847804496,
                accuracy: 357.8583216675231,
                reaction: 152.78330617647686,
            },
        ),
        (
            "v4.osu".to_string(),
            Skills {
                stamina: 260.3729506723779,
                tenacity: 142.95602750485276,
                agility: 252.59978233383907,
                precision: 513.7299058707363,
                memory: 359.074493959281,
                accuracy: 450.09688691515197,
                reaction: 152.78507104993895,
            },
        ),
        (
            "v5.osu".to_string(),
            Skills {
                stamina: 251.05024432844877,
                tenacity: 160.67530937571166,
                agility: 242.00120786477834,
                precision: 416.5511432115917,
                memory: 432.12534380116387,
                accuracy: 464.2055117084483,
                reaction: 188.42493075827534,
            },
        ),
        (
            "v6.osu".to_string(),
            Skills {
                stamina: 209.07368187829078,
                tenacity: 626.9147772161582,
                agility: 253.23876455299225,
                precision: 423.8601454471603,
                memory: 340.9856639044052,
                accuracy: 361.01623023311475,
                reaction: 127.48293623478614,
            },
        ),
        (
            "v7.osu".to_string(),
            Skills {
                stamina: 401.11208240579106,
                tenacity: 486.3285928938134,
                agility: 282.1880067610648,
                precision: 348.67355277372457,
                memory: 590.1184729571977,
                accuracy: 621.2766981959237,
                reaction: 127.66482838689143,
            },
        ),
        (
            "v8.osu".to_string(),
            Skills {
                stamina: 243.03260416728966,
                tenacity: 173.04490247887657,
                agility: 228.412233516846,
                precision: 407.39511318515434,
                memory: 380.4909129835611,
                accuracy: 473.30197200529255,
                reaction: 241.53616041588077,
            },
        ),
        (
            "v9.osu".to_string(),
            Skills {
                stamina: 111.98352968828767,
                tenacity: 626.9147772161582,
                agility: 208.5655985504184,
                precision: 310.4765518520394,
                memory: 270.3659870702836,
                accuracy: 227.9638401258083,
                reaction: 125.85086511639216,
            },
        ),
        (
            "v10.osu".to_string(),
            Skills {
                stamina: 72.67911677120951,
                tenacity: 626.9147772161582,
                agility: 188.21139825888014,
                precision: 219.90250743886074,
                memory: 0.0,
                accuracy: 154.5860524170765,
                reaction: 99.23809751072656,
            },
        ),
        (
            "v11.osu".to_string(),
            Skills {
                stamina: 204.71573967058555,
                tenacity: 48.267256654776354,
                agility: 232.0519835031802,
                precision: 323.54571014679016,
                memory: 245.6954783055098,
                accuracy: 373.9366709618941,
                reaction: 188.7834401190493,
            },
        ),
        (
            "v12.osu".to_string(),
            Skills {
                stamina: 400.2959207155216,
                tenacity: 272.30560909758174,
                agility: 296.4932563491454,
                precision: 355.274257743274,
                memory: 547.6007360950356,
                accuracy: 648.2648638459365,
                reaction: 329.48522764394033,
            },
        ),
        (
            "v13.osu".to_string(),
            Skills {
                stamina: 234.669547746693,
                tenacity: 626.9147772161582,
                agility: 261.42079145446485,
                precision: 249.6555380205757,
                memory: 407.38787594804546,
                accuracy: 430.17875374101976,
                reaction: 212.02551486770398,
            },
        ),
        (
            "v14.osu".to_string(),
            Skills {
                stamina: 462.7150235628645,
                tenacity: 508.40291043593504,
                agility: 525.0614403978884,
                precision: 439.45902669313944,
                memory: 682.4811243347966,
                accuracy: 805.5293432314736,
                reaction: 451.5345549663724,
            },
        ),
    ]);
}
