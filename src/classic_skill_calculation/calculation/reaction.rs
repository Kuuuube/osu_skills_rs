use crate::structs;
use crate::classic_skill_calculation::utils;
use crate::pair_structs;

fn get_visibility_times(obj: &structs::HitObject, ar: f64, hidden: bool, opacity_start: f64, opacity_end: f64) -> pair_structs::Pairi64 {
    let ar_ms: f64 = utils::ar_to_ms(ar);
    let preamp_time: f64 = obj.time as f64 - ar_ms;
    let mut times: pair_structs::Pairi64 = Default::default();

    if hidden {
        let fade_in_duration: f64 = 0.4 * ar_ms;
        let fade_in_time_end: f64 = preamp_time + fade_in_duration;

        times.x = utils::get_value(preamp_time, fade_in_time_end, opacity_start) as i64;

        if utils::is_hit_object_type(&obj.hit_object_type, structs::HitObjectType::Slider) {
            let fade_out_duration: f64 = obj.end_time as f64 - fade_in_time_end;
            let fade_out_time_end: f64 = fade_in_time_end + fade_out_duration;
            times.y = utils::get_value(fade_in_time_end, fade_out_time_end, 1.0 - opacity_end) as i64;

            return times;
        } else {
            let fade_out_duration: f64 = 0.7 * (obj.end_time as f64 - fade_in_time_end);
            let fade_out_time_end: f64 = fade_in_time_end + fade_out_duration;
            times.y = utils::get_value(fade_in_time_end, fade_out_time_end, 1.0 - opacity_start) as i64;
            return times;
        }
    } else {
        let fade_in_duration: f64 = f64::min(utils::ar_to_ms(ar), 400.0);
        let fade_in_time_end: f64 = preamp_time + fade_in_duration;

        times.x = utils::get_value(preamp_time, fade_in_time_end, opacity_start) as i64;

        if utils::is_hit_object_type(&obj.hit_object_type, structs::HitObjectType::Slider) {
            times.y = obj.end_time as i64;
            return times;
        } else {
            times.y = obj.time;
            return times;
        }
    }
}

fn pattern_req(p1: &structs::Timing, p2: &structs::Timing, p3: &structs::Timing, cspx: f64) -> f64 {
    let point1 = pair_structs::Pairf64 { x: p1.pos.x, y: p1.pos.y };
    let point2 = pair_structs::Pairf64 { x: p2.pos.x, y: p2.pos.y };
    let point3 = pair_structs::Pairf64 { x: p3.pos.x, y: p3.pos.y };

    let dist_12: f64 = pair_structs::get_distance_from(&point1, &point2);
    let dist_23: f64 = pair_structs::get_distance_from(&point2, &point3);
    let dist: f64 = dist_12 + dist_23;

    let angle: f64 = utils::get_angle(point1, point2, point3);
    
    let mut time = (p3.time - p1.time).abs();

    if time < 16 {
        time = 16;
    }

    return time as f64 / ((dist / (2.0 * cspx)) * ((std::f64::consts::PI - angle) / std::f64::consts::PI));
}

fn pattern_to_reaction(p1: &structs::Timing, p2: &structs::Timing, p3: &structs::Timing, ar_ms: f64, cs_px: f64) -> f64{
    let damping: f64 = 0.15; //this value comes from osu skills config file "PatternDamping"
    let curve_steepness = damping;
    let pattern_requirement = pattern_req(p1, p2, p3, cs_px);

    return ar_ms - ar_ms * f64::powf(std::f64::consts::E, -curve_steepness * pattern_requirement);
}

fn react_to_skill(time_to_react: f64) -> f64 {
    let a: f64 = f64::powf(2.0, f64::ln(78608.0 / 15625.0) / f64::ln(34.0 / 25.0)) * f64::powf(125.0, f64::ln(68.0 / 25.0) / f64::ln(34.0 / 25.0));
    let b: f64 = f64::ln(2.0) / (f64::ln(2.0) - 2.0 * f64::ln(5.0) + f64::ln(17.0));
    return a / f64::powf(time_to_react, b);
}

fn get_reaction_skill_at(target_points: &Vec<structs::Timing>, target_point: &structs::Timing, hit_objects: &Vec<structs::HitObject>, cs: f64, ar: f64, hidden: bool) -> f64 {
    let mut time_to_react = 0.0;
    let fade_in_react_req = 0.1; //this value comes from osu skills config file "FadeinPercent"
    let index: i32 = utils::find_timing_at(&target_points, target_point.time);

    if index >= (target_points.len() as i32) - 2 {
        time_to_react = utils::ar_to_ms(ar);
    } else if index < 3 {
        let visibility_times: pair_structs::Pairi64 = get_visibility_times(&hit_objects[0], ar, hidden, fade_in_react_req, 1.0);
        time_to_react = (hit_objects[0].time - visibility_times.x) as f64;
    } else {
        let t1: &structs::Timing = &target_points[index as usize];
        let t2: &structs::Timing = &target_points[(index + 1) as usize];
        let t3: &structs::Timing = &target_points[(index + 2) as usize];

        let mut time_since_start: f64 = 0.0;

        if target_point.press == true {
            time_since_start = f64::abs((target_point.time - hit_objects[target_point.key as usize].time) as f64);
        }

        let visibility_times: pair_structs::Pairi64 = get_visibility_times(&hit_objects[0], ar, hidden, fade_in_react_req, 1.0);
        let actual_ar_time: f64 = ((hit_objects[0].time - visibility_times.x) as f64) + time_since_start;

        let result: f64 = pattern_to_reaction(t1, t2, t3, actual_ar_time, utils::cs_to_px(cs) as f64);
        time_to_react = f64::sqrt(time_to_react * time_to_react + result * result);
    }

    let ver_scale: f64 = 12.2; //this value comes from osu skills config file "VerScale"
    let curve_exp: f64 = 0.64; //this value comes from osu skills config file "CurveExp"
    return ver_scale * f64::powf(react_to_skill(time_to_react), curve_exp);
}

pub fn calculate_reaction(beatmap: &structs::Beatmap) -> f64{
    let hidden: bool = utils::has_mod(beatmap, structs::Mods::HD);
    let mut max: f64 = 0.0;
    let mut avg: f64 = 0.0;
    let weight: f64 = 0.7; //this value comes from osu skills config file "AvgWeighting"

    for tick in &beatmap.target_points {
        let val: f64 = get_reaction_skill_at(&beatmap.target_points, &tick, &beatmap.hit_objects, beatmap.cs as i32 as f64, beatmap.ar, hidden);

        if val > max {
            max = val;
        }
        if val > max / 2.0 {
            avg = weight * val + (1.0 - weight) * avg;
        }
    }

    return (max + avg) / 2.0;
}