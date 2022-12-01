use crate::structs;

pub fn calculate_precision(beatmap: &structs::Beatmap, agility_limit: f64, agility_pow: f64, total_mult: f64, total_pow: f64) -> f64 {
    let scaled_agility;
    let agility_subtract = f64::powf(agility_limit, 0.1) - 1.0;
    let agility_shift = f64::powf(agility_subtract, 10.0);

    if beatmap.skills.agility > agility_limit {
        scaled_agility = 1.0;
    } else {
        scaled_agility = f64::powf(beatmap.skills.agility + agility_shift, agility_pow) - agility_subtract;
    }

    let precision: f64 = total_mult * f64::powf(scaled_agility * beatmap.cs, total_pow);
    return precision;
}