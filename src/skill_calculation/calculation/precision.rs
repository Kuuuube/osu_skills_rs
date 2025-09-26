use crate::{structs, vars::SKILL_CALCULATION_VARS};

pub fn calculate_precision(beatmap: &structs::Beatmap) -> f64 {
    let agility_limit: f64 = SKILL_CALCULATION_VARS.precision.agility_limit;
    let agility_pow: f64 = SKILL_CALCULATION_VARS.precision.agility_pow;
    let total_mult: f64 = SKILL_CALCULATION_VARS.precision.total_mult;
    let total_pow: f64 = SKILL_CALCULATION_VARS.precision.total_pow;
    let agility_subtract: f64 = f64::powf(agility_limit, 0.1) - 1.0; //this value originally came from osu skills config but it is better to calculate it
    let agility_shift: f64 = f64::powf(agility_subtract, 10.0);

    let scaled_agility: f64;

    if beatmap.skills.agility > agility_limit {
        scaled_agility = 1.0;
    } else {
        scaled_agility =
            f64::powf(beatmap.skills.agility + agility_shift, agility_pow) - agility_subtract;
    }

    let precision: f64 = total_mult * f64::powf(scaled_agility * beatmap.cs, total_pow);
    return precision;
}
