use crate::structs;

pub fn calculate_stamina(beatmap: &structs::Beatmap) -> f64 {
    let max: f64 = beatmap.tap_strains.iter().copied().fold(0.0, f64::max);

    let total_mult: f64 = 4.6;
    let total_pow: f64 = 0.75;
    let stamina: f64 = total_mult * f64::powf(max, total_pow);
    return stamina;
}