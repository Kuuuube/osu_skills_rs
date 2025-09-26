use crate::algs::erf::erf_inv;
use crate::calculation_utils;
use crate::structs;

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

    let cert: f64 = 0.1;
    let mut ss_ur: f64 = (5.0 * f64::sqrt(2.0) * calculation_utils::od_to_ms(beatmap.od))
        / erf_inv(f64::powf(cert, 1.0 / circles));
    if calculation_utils::has_mod(&beatmap, structs::Mods::DT) {
        ss_ur /= 1.5;
    } else if calculation_utils::has_mod(&beatmap, structs::Mods::HT) {
        ss_ur /= 0.75;
    }

    let ver_scale: f64 = 2.5; //this value comes from osu skills config file "VerScale"
    let total_mult: f64 = 42.2505; //this value comes from osu skills config file "VerScale"
    let total_pow: f64 = 0.27; //this value comes from osu skills config file "VerScale"
    let accuracy: f64 = total_mult
        * f64::powf(
            f64::powf(beatmap.skills.stamina, ver_scale) / ss_ur,
            total_pow,
        );

    return accuracy;
}
