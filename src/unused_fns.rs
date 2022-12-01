//reaction.rs
fn is_hit_object_at(hit_objects: &Vec<structs::HitObject>, previous_time: i64, current_time: i64) -> bool {
    let i = utils::find_hit_object_at(&hit_objects, current_time, false);

    if utils::btwn(&previous_time, &(hit_objects[i as usize].time as i64), &current_time) {
        return true;
    }

    if utils::btwn(&(hit_objects[i as usize].time as i64), &current_time, &(hit_objects[i as usize].end_time as i64)) {
        return true;
    }

    return false;
}

//reaction.rs
fn get_next_tick_point(hit_objects: Vec<structs::HitObject>, time: i64) -> structs::Timing {
    let mut time_mut = time;
    let mut tick_point: structs::Timing = Default::default();
    let i: u32 = utils::find_hit_object_at(&hit_objects, time, true);

    if i >= (hit_objects.len() as u32 - 1) {
        return structs::Timing{time: 0, data: -1.0, key: Default::default(), press: Default::default(), pos: Default::default()};
    }

    if !is_hit_object_at(&hit_objects, time - 1, time) {
        time_mut = hit_objects[(i + 1) as usize].time;
        let pos: &vector2d::Vector2F64 = &hit_objects[(i + 1) as usize].pos;

        tick_point.pos = vector2d::Vector2F64{x: pos.x, y: pos.y};
        tick_point.time = time_mut;
        tick_point.data = 0.0;
        tick_point.press = false;
        return tick_point;
    } else {
        if utils::is_hit_object_type(&hit_objects[i as usize].hit_object_type, structs::HitObjectType::Slider) {
            let ticks: &Vec<i32> = &hit_objects[i as usize].ticks;
            let mut tick = 1;
            for _tick_index in ticks {
                if utils::btwn(&(ticks[tick - 1] as i64), &time_mut, &(ticks[tick] as i64)) {
                    time_mut = ticks[tick as usize] as i64;
                    let pos: vector2d::Vector2F64 = slider::get_slider_pos(&hit_objects[i as usize], ticks[tick]);

                    tick_point.pos = vector2d::Vector2F64{x: pos.x, y: pos.y};
                    tick_point.time = time_mut;
                    tick_point.data = 0.0;
                    tick_point.press = true;
                    return tick_point;
                }
                tick += 1;
            }
        }
        time_mut = hit_objects[(i + 1) as usize].time;
        let pos = hit_objects[(i + 1) as usize].pos;

        tick_point.pos = vector2d::Vector2F64{x: pos.x, y: pos.y};
        tick_point.time = time_mut;
        tick_point.data = 0.0;
        tick_point.press = false;
        return tick_point;
    }
}

//patterns.rs
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

//utils.rs
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    return a * (1.0 - t) + b * t;
}

//utils.rs
pub fn find_hit_object_at(hit_objects: &Vec<structs::HitObject>, time: i64, dir: bool) -> u32 {
    let mut start: u32 = 0;
    let mut end: u32 = hit_objects.len() as u32;
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

    let final_val: u32 = hit_objects.len() as u32;
    return final_val - 1;
} 

//utils.rs
pub fn rad_to_deg(radians: f64) -> f64 {
    return radians * 180.0 / std::f64::consts::PI;
}

//utils.rs
pub fn ms_to_bpm(ms: i32) -> i32{
    if ms == 0 {
        return ms;
    }
    return 60 * 1000 / (ms * 4);
}

//utils.rs
pub fn ms_to_time_string(ms: i32) -> String {
    let min = ms / 60000;
    let sec = (ms - (min * 60000)) / 1000;
    let msec = sec % 1000;

    return [min.to_string(), ":".to_string(), sec.to_string(), ":".to_string(), msec.to_string()].join("");
}