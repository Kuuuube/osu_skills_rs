use crate::{structs, vars::SKILL_CALCULATION_VARS};

pub fn calculate_stamina(beatmap: &structs::Beatmap) -> f64 {
    let max: f64 = beatmap.tap_strains.iter().copied().fold(0.0, f64::max); //finds largest in Vec<f64>, returns 0.0 if nothing is found

    let total_mult: f64 = SKILL_CALCULATION_VARS.stamina.total_mult;
    let total_pow: f64 = SKILL_CALCULATION_VARS.stamina.total_pow;
    let stamina: f64 = total_mult * f64::powf(max, total_pow);
    return stamina;
}
