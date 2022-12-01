use crate::structs;
use crate::utils;
use crate::vector2d;

pub fn get_slider_pos(hit_object: &structs::HitObject, time: i32) -> vector2d::Vector2F64 {
    if utils::is_hit_object_type(&hit_object.hit_object_type, structs::HitObjectType::Slider) {
        let mut percent: f64;
        if time as i64 <= hit_object.time {
            percent = 0.0;
        } else if time > hit_object.end_time {
            percent = 1.0;
        } else {
            let time_length: i32 = time - hit_object.time as i32;
            let repeats_done: i32 = time_length / hit_object.to_repeat_time;
            percent = ((time_length - hit_object.to_repeat_time * repeats_done) / hit_object.to_repeat_time) as f64;
            if repeats_done % 2 != 0 {
                percent = 1.0 - percent;
            }
        }

        let index_f = percent * hit_object.ncurve as f64;
        let index: i32 = index_f as i32;

        if index >= hit_object.ncurve {
            return hit_object.lerp_points[hit_object.ncurve as usize];
        } else {
            let point = hit_object.lerp_points[index as usize];
            let point2 = hit_object.lerp_points[(index + 1) as usize];
            let t2 = index_f - index as f64;
            return vector2d::Vector2F64{x: utils::lerp(point.x, point2.x, t2), y: utils::lerp(point.y, point2.y, t2)};
        } 
    } else {
        return vector2d::Vector2F64{x: -1.0, y: -1.0};
    }
}