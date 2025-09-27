use crate::pair_structs;
use crate::structs;
use crate::structs::SkillCalculationAgilityVars;

pub fn calculate_tap_strains(mut beatmap: structs::Beatmap) -> structs::Beatmap {
    let mut c: i32 = 0;
    let mut old_bonus: f64 = 0.0;
    let mut strain: f64 = 0.0;

    for interval in &beatmap.press_intervals {
        let interval_deref: i32 = *interval;
        if c == 0 {
            let largest_interval: f64 = beatmap.skill_calculation_vars.stamina.largest_interval;
            if interval_deref >= largest_interval as i32 {
                strain = 0.0;
            } else {
                let scale: f64 = beatmap.skill_calculation_vars.stamina.scale;
                let pow: f64 = beatmap.skill_calculation_vars.stamina.pow;
                let mult: f64 = beatmap.skill_calculation_vars.stamina.mult;
                strain = scale
                    / f64::powf(
                        interval_deref as f64,
                        f64::powf(interval_deref as f64, pow) * mult,
                    );
            }
            beatmap.tap_strains.push(strain);
        } else {
            let largest_interval: f64 = beatmap.skill_calculation_vars.stamina.largest_interval;
            if interval_deref >= largest_interval as i32 {
                let decay_max: f64 = beatmap.skill_calculation_vars.stamina.decay_max;
                strain *= decay_max;
            } else {
                if interval_deref <= 15 {
                    continue;
                }
                let scale: f64 = beatmap.skill_calculation_vars.stamina.scale;
                let pow: f64 = beatmap.skill_calculation_vars.stamina.pow;
                let mult: f64 = beatmap.skill_calculation_vars.stamina.mult;
                strain = scale
                    / f64::powf(
                        interval_deref as f64,
                        f64::powf(interval_deref as f64, pow) * mult,
                    );
                let decay: f64 = beatmap.skill_calculation_vars.stamina.decay;
                strain += old_bonus * decay;
            }
            beatmap.tap_strains.push(strain);
        }
        old_bonus = strain;

        c += 1;
    }

    return beatmap;
}

fn get_weighted_aim_distance(distance: f64, agility_vars: &SkillCalculationAgilityVars) -> f64 {
    let dist_mult: f64 = agility_vars.dist_mult;
    let dist_pow: f64 = agility_vars.dist_pow;
    let dist_div: f64 = agility_vars.dist_divisor;
    let distance_bonus: f64 = f64::powf(1.0 + (distance * dist_mult), dist_pow) / dist_div;
    return distance * distance_bonus;
}

fn get_weighted_aim_time(time: f64, agility_vars: &SkillCalculationAgilityVars) -> f64 {
    let time_mult: f64 = agility_vars.time_mult;
    let time_pow: f64 = agility_vars.time_pow;
    let time_bonus: f64 = f64::powf(time * time_mult, time_pow);
    return time * time_bonus;
}

pub fn calculate_aim_strains(mut beatmap: structs::Beatmap) -> structs::Beatmap {
    let mut old_strain: f64 = 0.0;

    let mut i: usize = 0;
    while i < beatmap.aim_points.len() {
        let mut strain = 0.0;

        if i != 0 {
            let distance: f64 = get_weighted_aim_distance(
                pair_structs::get_distance_from(
                    &beatmap.aim_points[i].pos,
                    &beatmap.aim_points[i - 1].pos,
                ),
                &beatmap.skill_calculation_vars.agility,
            );
            let interval: i64 = beatmap.aim_points[i].time - beatmap.aim_points[i - 1].time;
            let time: f64 =
                get_weighted_aim_time(interval as f64, &beatmap.skill_calculation_vars.agility);
            let mut angle_bonus: f64 = 1.0;

            if i > 1 {
                let angle_mult: f64 = beatmap.skill_calculation_vars.agility.angle_mult;
                angle_bonus = 1.0 + angle_mult * beatmap.angle_bonuses[i - 2];
            }

            if time > 0.0 {
                strain = distance / time * angle_bonus;
            } else {
                i += 1;
                continue;
            }

            if beatmap.aim_points[i].aim_point_type == structs::AimPointTypes::AimPointSliderend
                || beatmap.aim_points[i - 1].aim_point_type
                    == structs::AimPointTypes::AimPointSliderend
            {
                let slider_strain_decay: f64 =
                    beatmap.skill_calculation_vars.agility.slider_strain_decay;
                strain *= slider_strain_decay;
            }

            let strain_decay: f64 = beatmap.skill_calculation_vars.agility.strain_decay;
            old_strain -= strain_decay * interval as f64;
            if old_strain < 0.0 {
                old_strain = 0.0;
            }

            strain += old_strain;
        }
        beatmap.aim_strains.push(strain);
        old_strain = strain;

        i += 1;
    }

    return beatmap;
}
