use crate::structs;
use crate::utils;

pub fn calculate_agility(beatmap: &structs::Beatmap) -> f64 {
    let total_mult: f64 = 30.0; //this value comes from osu skills config file "TotalMult"
    let total_pow: f64 = 0.28; //this value comes from osu skills config file "TotalPow"
    let weighting: f64 = 0.78; //this value comes from osu skills config file "Weighting"

    let top_weights: Vec<f64> = utils::get_peak_vals(&beatmap.aim_strains);

    let agility: f64 = total_mult * f64::powf(utils::get_weighted_value_2(top_weights, weighting), total_pow);
    return agility;
}