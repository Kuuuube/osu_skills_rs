use crate::structs;

pub fn calculate_precision(beatmap: &structs::Beatmap) -> f64 {
    let agility_pow: f64 = 0.1; //this value comes from osu skills config file "AgilityPow"
    let total_mult: f64 = 20.0; //this value comes from osu skills config file "TotalMult"
    let total_pow: f64 = 2.0; //this value comes from osu skills config file "TotalPow"
    let agility_subtract: f64 = 0.995462; //this value comes from osu skills config file "AgilitySubtract"

    let scaled_agility: f64 = f64::powf(beatmap.skills.agility + 1.0, agility_pow) - agility_subtract;

    let precision: f64 = total_mult * f64::powf(scaled_agility * (beatmap.cs as i32 as f64), total_pow);
    return precision;
}