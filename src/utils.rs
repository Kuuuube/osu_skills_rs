use crate::structs;
use crate::structs::HitObjectType;
use crate::vector2d;

pub fn deg_to_rad(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.0;
}

pub fn rad_to_deg(radians: f64) -> f64 {
    return radians * 180.0 / std::f64::consts::PI;
}

pub fn btwn(lss: &i64, val: &i64, gtr: &i64) -> bool {
    return (&i64::min(*lss, *gtr) <= val) && (val <= &i64::max(*lss, *gtr));
}

pub fn ms_to_bpm(ms: i32) -> i32{
    if ms == 0 {
        return ms;
    }
    return 60 * 1000 / (ms * 4);
}

pub fn ms_to_time_string(ms: i32) -> String {
    let min = ms / 60000;
    let sec = (ms - (min * 60000)) / 1000;
    let msec = sec % 1000;

    return [min.to_string(), ":".to_string(), sec.to_string(), ":".to_string(), msec.to_string()].join("");
}

pub fn get_dir_angle(a: vector2d::Vector2F64, b: vector2d::Vector2F64, c: vector2d::Vector2F64) -> f64 {
    let ab = vector2d::Vector2F64 {x: b.x - a.x, y: b.y - a.y};
    let cb = vector2d::Vector2F64 {x: b.x - c.x, y: b.y - c.y};

    let dot: f64 = ab.x * cb.x + ab.y * cb.y;
    let cross: f64 = ab.x * cb.y + ab.y * cb.x;

    let alpha = f64::atan2(cross, dot) * 180.0 / std::f64::consts::PI;

    return alpha;
}

pub fn get_angle(a: vector2d::Vector2F64, b: vector2d::Vector2F64, c: vector2d::Vector2F64) -> f64 {
    return deg_to_rad(get_dir_angle(a, b, c)).abs();
}

pub fn find_hit_object_at(hit_objects: &Vec<structs::HitObject>, time: i64, dir: bool) -> u32 {
    let mut start: u32 = 0;
    let mut end: u32 = hit_objects.len().try_into().unwrap();
    let mut mid: u32;

    while start <= end {
        mid = (start + end) / 2;

         if btwn(&(hit_objects[mid as usize].time as i64), &time, &(hit_objects[mid as usize].end_time as i64)) {
            if btwn(&(hit_objects[(mid + 1) as usize].time as i64), &time, &(hit_objects[(mid) as usize].end_time as i64)) {
                return mid + 1;
            } else {
                return mid;
            }
        }

        if btwn(&(hit_objects[mid as usize].time as i64), &time, &(hit_objects[(mid + 1) as usize].end_time as i64)) {
            return mid + dir as u32;
        }

        if time < hit_objects[mid as usize].time as i64 {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    let final_val: u32 = hit_objects.len().try_into().unwrap();
    return final_val - 1;
} 

pub fn is_hit_object_type(hit_object: &i32, object_type: HitObjectType) -> bool {
    return (hit_object & object_type as i32) > 0;
}