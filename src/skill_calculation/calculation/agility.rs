use crate::calculation_utils;
use crate::structs;
use crate::vars::SKILL_CALCULATION_VARS;

pub fn calculate_agility(beatmap: &structs::Beatmap) -> f64 {
    let total_mult: f64 = SKILL_CALCULATION_VARS.agility.total_mult;
    let total_pow: f64 = SKILL_CALCULATION_VARS.agility.total_pow;
    let weighting: f64 = SKILL_CALCULATION_VARS.agility.weighting;

    let top_weights: Vec<f64> = calculation_utils::get_peak_vals(&beatmap.aim_strains);

    let agility: f64 = total_mult
        * f64::powf(
            calculation_utils::get_weighted_value_2(top_weights, weighting),
            total_pow,
        );
    return agility;
}
