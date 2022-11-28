pub fn calculate_precision(agility: f64, agility_limit: f64, agility_pow: f64, agility_subtract: f64, beatmap_cs: i8, total_mult: f64, total_pow: f64) -> f64 {
        let scaled_agility;
    
        if agility > agility_limit {
            scaled_agility = 1.0;
        } else {
            scaled_agility = f64::powf(agility + 1.0, agility_pow) - agility_subtract;
        }
    
        let precision: f64 = total_mult * f64::powf(scaled_agility * beatmap_cs as f64, total_pow);
        return precision;
    }