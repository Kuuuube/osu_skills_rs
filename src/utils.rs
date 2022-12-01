use crate::structs;
use crate::structs::HitObjectType;
use crate::pair_structs;

pub fn deg_to_rad(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.0;
}



pub fn btwn(lss: &i64, val: &i64, gtr: &i64) -> bool {
    return (&i64::min(*lss, *gtr) <= val) && (val <= &i64::max(*lss, *gtr));
}

pub fn get_dir_angle(a: pair_structs::Pairf64, b: pair_structs::Pairf64, c: pair_structs::Pairf64) -> f64 {
    let ab = pair_structs::Pairf64 {x: b.x - a.x, y: b.y - a.y};
    let cb = pair_structs::Pairf64 {x: b.x - c.x, y: b.y - c.y};

    let dot: f64 = ab.x * cb.x + ab.y * cb.y;
    let cross: f64 = ab.x * cb.y + ab.y * cb.x;

    let alpha = f64::atan2(cross, dot) * 180.0 / std::f64::consts::PI;

    return alpha;
}

pub fn get_angle(a: pair_structs::Pairf64, b: pair_structs::Pairf64, c: pair_structs::Pairf64) -> f64 {
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

pub fn get_weighted_value_2(vals: Vec<f64>, decay: f64) -> f64 {
    let mut result: f64 = 0.0;
    let mut i: usize = 0;
    while i < vals.len() {
        result += vals[i] * f64::powf(decay, i as f64);
        i += 1;
    }
    return result;
}

pub fn get_peak_vals(vals: &Vec<f64>) -> Vec<f64>{
    let mut output: Vec<f64> = Default::default();
    let mut i: usize = 1;
    if vals.len() >= 3 {
        while i < vals.len() - 1 {
            if vals[i] > vals[i - 1] && vals[i] > vals[i + 1] {
                output.push(vals[i]);
            }
            i += 1;
        }
        output.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
    return output;
}