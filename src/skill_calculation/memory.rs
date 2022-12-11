use crate::utils;
use crate::structs;
use crate::pair_structs;

fn get_approach_relative_size(time: f64, hit_time: f64, ar: f64) -> f64 {
    let ar_ms: f64 = utils::ar_to_ms(ar);
    
    if hit_time < time {
        return 1.0;
    } else if hit_time - ar_ms > time {
        return 0.0;
    } else {
        return 1.0 + 3.0 * ((hit_time - time) / ar_ms);
    }
}

fn is_observable_from(obj: &structs::HitObject, distance: i32, from_pos: &pair_structs::Pairf64) -> bool {
    let dist: f64 = pair_structs::get_distance_from(&obj.pos, &from_pos);
    if dist < distance as f64 {
        return true;
    }
    return false;
}

pub fn calculate_memory(beatmap: &structs::Beatmap) -> f64 {
    if beatmap.hit_objects.len() <= 0 {
        return 0.0 
    }
    
    let mut total_mem_points: f64 = 0.0;
    let mut old: &structs::HitObject = &beatmap.hit_objects[0];
    let mut combo: i32 = 0;

    let mut i: usize = 1;
    while i < beatmap.hit_objects.len() {
        let cur: &structs::HitObject = &beatmap.hit_objects[i];
        let mut mem_points: f64 = 0.0;
        let observable_dist: i32;

        if combo < 100 {
            observable_dist = 160;
        } else if combo < 200 {
            observable_dist = 120;
        } else {
            observable_dist = 100;
        }

        let mut slider_bonus_factor: f64 = 1.0;

        if utils::is_hit_object_type(&old.hit_object_type, structs::HitObjectType::Slider) {
            slider_bonus_factor = 1.1; //this value comes from osu skills config file "SliderBuff"
        }

        let mut observable: bool = false;
        let mut help_pixels: i32;

        let mut j: usize = i - 1;
        while j > 0 {
            let prev: &structs::HitObject = &beatmap.hit_objects[j];
            if cur.time - prev.time > utils::ar_to_ms(beatmap.ar) as i64 {
                break;
            }
            if !utils::has_mod(&beatmap, structs::Mods::HD) {
                let size: f64 = get_approach_relative_size(prev.end_time as f64, cur.time as f64, beatmap.ar);
                help_pixels = (size * utils::cs_to_px(beatmap.cs as i32 as f64) as f64) as i32;
            } else {
                let observable_time: i64 = cur.time - (utils::ar_to_ms(beatmap.ar) * 0.3) as i64;
                if prev.time > observable_time {
                    j -= 1;
                    continue;
                }
                help_pixels = utils::cs_to_px(beatmap.cs as i32 as f64) as i32;
            }
            if is_observable_from(cur, observable_dist + help_pixels, &prev.pos) {
                observable = true;
                break;
            }
            j -= 1;
        }

        if !observable {
            if !utils::has_mod(&beatmap, structs::Mods::HD) {
                let size: f64 = get_approach_relative_size(old.end_time as f64, cur.time as f64, beatmap.ar);
                help_pixels = (size * utils::cs_to_px(beatmap.cs as i32 as f64) as f64) as i32;
            } else {
                help_pixels = utils::cs_to_px(beatmap.cs as i32 as f64) as i32;
            }

            if utils::is_hit_object_type(&cur.hit_object_type, structs::HitObjectType::NewCombo) || utils::is_hit_object_type(&cur.hit_object_type, structs::HitObjectType::ColourHax) {
                let dist: f64 = pair_structs::get_distance_from(&cur.pos, &old.end_point);
                if dist > (observable_dist + help_pixels) as f64 {
                    mem_points = slider_bonus_factor * (dist / (cur.time - old.time) as f64);
                }
            } else {
                let dist: f64 = pair_structs::get_distance_from(&cur.pos, &old.end_point);
                if dist > (observable_dist + help_pixels) as f64 {
                    let follow_points_nerf: f64 = 0.8; //this value comes from osu skills config file "FollowpointsNerf"
                    mem_points = slider_bonus_factor * follow_points_nerf * (dist / (cur.time - old.time) as f64);
                }
            }
        }

        if utils::is_hit_object_type(&cur.hit_object_type, structs::HitObjectType::Normal) || utils::is_hit_object_type(&cur.hit_object_type, structs::HitObjectType::Spinner) {
            combo += 1;
        } else if utils::is_hit_object_type(&cur.hit_object_type, structs::HitObjectType::Slider) {
            combo += (cur.ticks.len() + 2) as i32;
        }

        old = cur;
        total_mem_points += mem_points;

        i += 1;
    }

    let total_mult: f64 = 205.0; //this value comes from osu skills config file "TotalMult"
    let total_pow: f64 = 0.3; //this value comes from osu skills config file "TotalPow"
    let memory: f64 = total_mult * f64::powf(total_mem_points, total_pow);
    
    return memory;
}