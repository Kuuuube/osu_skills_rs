use crate::structs;

pub fn calculate_precision(beatmap: &structs::Beatmap) -> f64 {
    let agility_pow: f64 = beatmap.skill_calculation_vars.precision.agility_pow;
    let total_mult: f64 = beatmap.skill_calculation_vars.precision.total_mult;
    let total_pow: f64 = beatmap.skill_calculation_vars.precision.total_pow;
    let agility_subtract: f64 = beatmap.skill_calculation_vars.precision.agility_subtract;

    let scaled_agility: f64 = if beatmap.skills.agility > beatmap.skill_calculation_vars.precision.agility_limit {
        1.0
    } else {
        f64::powf(beatmap.skills.agility + 1.0, agility_pow) - agility_subtract
    };

    let precision: f64 =
        total_mult * f64::powf(scaled_agility * (beatmap.cs as i32 as f64), total_pow);
    return precision;
}
