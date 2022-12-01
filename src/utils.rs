use crate::structs;
use crate::structs::HitObjectType;
use crate::vector2d;

pub fn deg_to_rad(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.0;
}



pub fn btwn(lss: &i64, val: &i64, gtr: &i64) -> bool {
    return (&i64::min(*lss, *gtr) <= val) && (val <= &i64::max(*lss, *gtr));
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

pub fn is_hit_object_type(hit_object: &i32, object_type: HitObjectType) -> bool {
    return (hit_object & object_type as i32) > 0;
}

pub fn ar_to_ms(ar: f64) -> f64 {
    if ar <= 5.0 {
        return 1800.0 - 120.0 * ar;
    } else {
        return 1950.0 - 150.0 * ar;
    }

}

pub fn find_timing_at(timings: &Vec<structs::Timing>, time: i64) -> i32 {
    let mut start: i32 = 0;
    let mut end: i32 = timings.len() as i32 - 2;
    let mut mid: i32;

    if end < 0 {
        return 0;
    }

    while start <= end {
        mid = (start + end) / 2;

        if btwn(&timings[mid as usize].time, &time, &timings[(mid + 1) as usize].time) {
            return mid + 1;
        }

        if time < timings[mid as usize].time {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    if time < timings[0].time {
        return i32::min_value();
    }

    if time > timings[timings.len() - 1].time {
        return i32::max_value();
    }

    return i32::min_value(); //original code returns NAN here
}

pub fn get_value(min: f64, max: f64, percent: f64) -> f64 {
    return f64::max(max, min) - (1.0 - percent) * (f64::max(max, min) - f64::min(max, min));
}

pub fn cs_to_px(cs: f64) -> f64 {
    return 54.5 - (4.5 * cs);
}