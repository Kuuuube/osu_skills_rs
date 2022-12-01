use crate::structs;
use crate::utils;
use crate::pair_structs;

pub fn get_visibility_times(obj: &structs::HitObject, ar: f64, hidden: bool, opacity_start: f64, opacity_end: f64) -> pair_structs::Pairi64 {
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