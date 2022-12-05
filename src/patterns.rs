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

pub fn get_slider_pos(hit_object: &structs::HitObject, time: i32) -> pair_structs::Pairf64 {
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
            return pair_structs::Pairf64{x: utils::lerp(point.x, point2.x, t2), y: utils::lerp(point.y, point2.y, t2)};
        } 
    } else {
        return pair_structs::Pairf64{x: -1.0, y: -1.0};
    }
}

pub fn approximate_slider_points(mut beatmap: structs::Beatmap) -> structs::Beatmap {
    let mut timing_point_offsets: Vec<f64> = Default::default();
    let mut beat_lengths: Vec<f64> = Default::default();
    let mut base: f64 = Default::default();

    let mut i: usize = 0;
    while i < beatmap.timing_points.len() {
        timing_point_offsets.push(beatmap.timing_points[i].offset as f64);

        if beatmap.timing_points[i].inherited {
            beat_lengths.push(base);
        } else {
            beat_lengths.push(beatmap.timing_points[i].beat_interval);
            base = beatmap.timing_points[i].beat_interval;
        }
        i += 1;
    }

    i = 0;
    while i < beatmap.hit_objects.len() {
        if utils::is_hit_object_type(&beatmap.hit_objects[i].hit_object_type, structs::HitObjectType::Slider) {
            let timing_point_index: usize = utils::get_value_pos(&timing_point_offsets, &(beatmap.hit_objects[i].time as f64), true);

            beatmap.hit_objects[i].to_repeat_time = (((((-600.0 / beatmap.timing_points[timing_point_index].bpm) * beatmap.hit_objects[i].pixel_length * beatmap.timing_points[timing_point_index].sm) / (100.0 * beatmap.sm)) as f64)) as i32;
            beatmap.hit_objects[i].end_time = beatmap.hit_objects[i].time as i32 + beatmap.hit_objects[i].to_repeat_time * beatmap.hit_objects[i].repeat;

            if beatmap.hit_objects[i].repeat > 1 {
                let mut j: i32 = beatmap.hit_objects[i].time as i32;
                while j < beatmap.hit_objects[i].end_time {
                    if j > beatmap.hit_objects[i].end_time {
                        break;
                    }
                    beatmap.hit_objects[i].repeat_times.push(j);

                    j += beatmap.hit_objects[i].to_repeat_time;
                }
            }

            let tick_interval: f64 = beat_lengths[timing_point_index] / beatmap.st;
            let err_interval: i32 = 10;
            let mut j: f64 = 1.0;

            let mut k: usize = (beatmap.hit_objects[i].time + tick_interval as i64) as usize;
            while k < (beatmap.hit_objects[i].end_time - err_interval) as usize {
                if k > beatmap.hit_objects[i].end_time as usize {
                    break;
                }

                let tick_time: i64 = beatmap.hit_objects[i].time + (tick_interval * j) as i64;
                if tick_time < 0 {
                    break;
                }

                beatmap.hit_objects[i].ticks.push(tick_time as i32);

                k += tick_interval as usize;
                j += 1.0;
            }

            if i64::abs(beatmap.hit_objects[i].end_time as i64 - beatmap.hit_objects[i].time) < 100 && (beatmap.hit_objects[i].ticks.len() == 0) {
                beatmap.hit_objects[i].curves = vec![pair_structs::Pairf64{x: beatmap.hit_objects[i].pos.x, y: beatmap.hit_objects[i].pos.y}, pair_structs::Pairf64{x: beatmap.hit_objects[i].pos.x + tick_interval / beatmap.st, y: beatmap.hit_objects[i].pos.y + tick_interval / beatmap.st}];
                beatmap.hit_objects[i].end_time = beatmap.hit_objects[i].end_time + 101;
                beatmap.hit_objects[i].to_repeat_time = beatmap.hit_objects[i].to_repeat_time + 101;
                beatmap.hit_objects[i].repeat = 1;
                beatmap.hit_objects[i].pixel_length = 100.0;
                beatmap.hit_objects[i].curve_type = structs::CurveType::LinearCurve;

                beatmap.hit_objects[i] = slider(beatmap.hit_objects[i].clone(), true);
            }
        } else {
            beatmap.hit_objects[i].end_time = beatmap.hit_objects[i].time as i32;
        }
        i += 1;
    }

    return beatmap;
}

 fn slider(hit_object: structs::HitObject, line: bool) -> structs::HitObject {
    let mut slider: structs::Slider = Default::default();
    let beziers: Vec<pair_structs::Pairf64>;

    let control_points: usize = hit_object.curves.len() + 1;
    let points: Vec<pair_structs::Pairf64>;
    let last_point = pair_structs::Pairf64{x: -1.0, y: -1.0};

    let mut i: usize = 0;
    while i < hit_object.curves.len() {
        slider.slider.push(pair_structs::Pairf64{x: hit_object.curves[i].x, y: hit_object.curves[i].y});
        i += 1;
    }

    slider.x = hit_object.pos.x;
    slider.y = hit_object.pos.y;

    i = 0;
    while i < control_points {
        // left off here ---------------------------------------------------------------------------------------------
        
        i += 1;
    }

    return hit_object;
} 

fn bezier_fn(points: Vec<pair_structs::Pairf64>) {
    let mut bezier: structs::Bezier = Default::default();
    let mut i: usize = 0;
    while i < points.len() {
        bezier.approx_length += pair_structs::get_distance_from(&points[i], &points[i + 1]);

        i += 1;
    }

    bezier_init(bezier);
}

fn bezier_init(mut bezier: structs::Bezier) -> structs::Bezier {
    bezier.ncurve = (bezier.approx_length / 4.0 + 2.0) as i32;
    let mut i: usize = 0;
    while i < bezier.ncurve as usize {
        bezier.curve_points.push(point_at(i as f64 / (bezier.ncurve - 1) as f64, &bezier));
        
        i += 1;
    }

    return bezier;
}

fn point_at(t: f64, bezier: &structs::Bezier) -> pair_structs::Pairf64 {
    let mut c: pair_structs::Pairf64 = Default::default();
    let n: usize = bezier.points.len() - 1;
    let mut i: usize = 0;
    while i <= n {
        let b: f64 = utils::bernstien(i as i64, n as i64, t);
        c += pair_structs::Pairf64{x: bezier.points[i].x * b, y: bezier.points[i].y * b};
        i += 1;
    }
    return c;
}