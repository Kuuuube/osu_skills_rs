use crate::algs::erf;
use crate::structs;
use crate::skill_calculation::utils;

pub fn calculate_accuracy(beatmap: &structs::Beatmap) -> f64 {
    let mut circles: f64 = 0.0;
    for obj in &beatmap.hit_objects {
        if utils::is_hit_object_type(&obj.hit_object_type, structs::HitObjectType::Normal) {
            circles += 1.0;
        }
    }

    let mut od_ms: f64 = utils::od_to_ms(beatmap.od);
    if utils::has_mod(&beatmap, structs::Mods::DT) {
        od_ms /= 1.5;
    } else if utils::has_mod(&beatmap, structs::Mods::HT) {
        od_ms /= 0.75;
    }

    let tapping: f64;
    let acc_scale: f64 = 0.01; //this value comes from osu skills config file "AccScale"
    if beatmap.skills.stamina == 0.0 {
        tapping = 1.0; //erf::erf(f64::INFINITY)
    } else {
        tapping = erf::erf(od_ms / (acc_scale * beatmap.skills.stamina * beatmap.skills.stamina));
    }

    let verscale: f64 = 0.3; //this value comes from osu skills config file "VerScale"
    let accuracy: f64 = -verscale * circles * f64::ln(tapping);

    return accuracy;
}