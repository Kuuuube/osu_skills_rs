use crate::algs::erf;
use crate::calculation_utils;
use crate::structs;
use crate::vars::SKILL_CALCULATION_VARS;

pub fn calculate_accuracy(beatmap: &structs::Beatmap) -> f64 {
    let mut circles: f64 = 0.0;
    for obj in &beatmap.hit_objects {
        if calculation_utils::is_hit_object_type(
            &obj.hit_object_type,
            structs::HitObjectType::Normal,
        ) {
            circles += 1.0;
        }
    }

    let mut od_ms: f64 = calculation_utils::od_to_ms(beatmap.od as i32 as f64);
    if calculation_utils::has_mod(&beatmap, structs::Mods::DT) {
        od_ms /= 1.5;
    } else if calculation_utils::has_mod(&beatmap, structs::Mods::HT) {
        od_ms /= 0.75;
    }

    let tapping: f64;
    let acc_scale: f64 = SKILL_CALCULATION_VARS.accuracy.acc_scale;
    if beatmap.skills.stamina == 0.0 {
        tapping = 1.0; //erf::erf(f64::INFINITY)
    } else {
        tapping = erf::erf(od_ms / (acc_scale * beatmap.skills.stamina * beatmap.skills.stamina));
    }

    let verscale: f64 = SKILL_CALCULATION_VARS.accuracy.ver_scale;
    let accuracy: f64 = -verscale * circles * f64::ln(tapping);

    return accuracy;
}
