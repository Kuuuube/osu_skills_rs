use crate::structs;
use crate::skill_calculation::utils;
use crate::pair_structs;

pub fn get_slider_pos(hit_object: &structs::HitObject, time: i64) -> pair_structs::Pairf64 {
    if utils::is_hit_object_type(&hit_object.hit_object_type, structs::HitObjectType::Slider) {
        let mut percent: f64;
        if time <= hit_object.time {
            percent = 0.0;
        } else if time > hit_object.end_time {
            percent = 1.0;
        } else {
            let time_length: i64 = time - hit_object.time;
            let repeats_done: i64 = time_length / hit_object.to_repeat_time;
            percent = (time_length - hit_object.to_repeat_time * repeats_done) as f64 / hit_object.to_repeat_time as f64;
            if repeats_done % 2 != 0 {
                percent = 1.0 - percent;
            }
        }

        let ncurve: usize = hit_object.ncurve as usize;
        let index_f: f64 = percent * ncurve as f64;
        let index: usize = index_f as usize;

        if index >= ncurve {
            return hit_object.lerp_points[ncurve];
        } else {
            let point = hit_object.lerp_points[index];
            let point2 = hit_object.lerp_points[index + 1];
            let t2: f64 = index_f - index as f64;
            return pair_structs::Pairf64{x: utils::lerp(point.x, point2.x, t2), y: utils::lerp(point.y, point2.y, t2)};
        }
    }
    return pair_structs::Pairf64{x: -1.0, y: -1.0};
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
            if timing_point_index == usize::MAX {
                i += 1;
                continue;
            }

            beatmap.hit_objects[i].to_repeat_time = (f64::round((((-600.0 / beatmap.timing_points[timing_point_index].bpm) * beatmap.hit_objects[i].pixel_length * beatmap.timing_points[timing_point_index].sm) / (100.0 * beatmap.sm)) as f64)) as i64;

            //stop some aspire maps from causing ooms by catching overflows, unnested operation in the below comment
            //beatmap.hit_objects[i].end_time = beatmap.hit_objects[i].time + beatmap.hit_objects[i].to_repeat_time * beatmap.hit_objects[i].repeat as i64;
            beatmap.hit_objects[i].end_time = match beatmap.hit_objects[i].to_repeat_time.checked_mul(beatmap.hit_objects[i].repeat as i64) {
                Some(some) => match beatmap.hit_objects[i].time.checked_add(some) {
                    Some(some) => some,
                    None => panic!("slider overflow detected, aborting calculation")
                },
                None => panic!("slider overflow detected, aborting calculation")
            };

            if beatmap.hit_objects[i].repeat > 1 {
                let mut j: i64 = beatmap.hit_objects[i].time;
                while j < beatmap.hit_objects[i].end_time {
                    if j > beatmap.hit_objects[i].end_time {
                        break;
                    }
                    beatmap.hit_objects[i].repeat_times.push(j);

                    j += beatmap.hit_objects[i].to_repeat_time;
                }
            }

            let tick_interval: i64 = (beat_lengths[timing_point_index] / beatmap.st) as i64;
            let err_interval: i64 = 10;
            let mut j: i64 = 1;

            let mut k: usize = (beatmap.hit_objects[i].time + tick_interval) as usize;
            while k < (beatmap.hit_objects[i].end_time - err_interval) as usize {
                if k > beatmap.hit_objects[i].end_time as usize {
                    break;
                }

                let tick_time: i64 = beatmap.hit_objects[i].time + tick_interval * j;
                if tick_time < 0 || tick_interval < 10 {
                    break;
                }

                beatmap.hit_objects[i].ticks.push(tick_time);

                k += tick_interval as usize;
                j += 1;
            }

            if i64::abs(beatmap.hit_objects[i].end_time as i64 - beatmap.hit_objects[i].time) < 100 && (beatmap.hit_objects[i].ticks.len() == 0) {
                let mut hit_object_new: structs::HitObject = Default::default();
                hit_object_new.curves = vec![pair_structs::Pairf64{x: beatmap.hit_objects[i].pos.x, y: beatmap.hit_objects[i].pos.y}, pair_structs::Pairf64{x: beatmap.hit_objects[i].pos.x + tick_interval as f64 / beatmap.st, y: beatmap.hit_objects[i].pos.y + tick_interval as f64 / beatmap.st}];
                hit_object_new.pos = beatmap.hit_objects[i].pos;
                hit_object_new.hit_object_type = beatmap.hit_objects[i].hit_object_type;
                hit_object_new.time = beatmap.hit_objects[i].time;
                hit_object_new.end_time = beatmap.hit_objects[i].time + 101;
                hit_object_new.to_repeat_time = beatmap.hit_objects[i].time + 101;
                hit_object_new.repeat = 1;
                hit_object_new.pixel_length = 100.0;
                hit_object_new.curve_type = structs::CurveType::LinearCurve;

                let slider: structs::Slider = slider_fn(hit_object_new.clone(), true);

                beatmap.hit_objects[i] = combine_slider_hit_object(hit_object_new, slider);
            }
        } else {
            beatmap.hit_objects[i].end_time = beatmap.hit_objects[i].time;
        }
        i += 1;
    }

    return beatmap;
}

fn combine_slider_hit_object(hit_object: structs::HitObject, slider: structs::Slider) -> structs::HitObject {
    let new_hit_object = structs::HitObject { pos: hit_object.pos, time: hit_object.time, hit_object_type: hit_object.hit_object_type, curve_type: hit_object.curve_type, curves: hit_object.curves, 
        lerp_points: slider.curve, ncurve: slider.ncurve, //these two are the only slider values used
        repeat: hit_object.repeat, repeat_times: hit_object.repeat_times, pixel_length: hit_object.pixel_length, end_time: hit_object.end_time, to_repeat_time: hit_object.to_repeat_time, end_point: hit_object.end_point, ticks: hit_object.ticks };
    return new_hit_object;
}

pub fn slider_fn(mut hit_object: structs::HitObject, line: bool) -> structs::Slider {
    let mut slider: structs::Slider = Default::default();
    let mut beziers: Vec<structs::Bezier> = Default::default();

    let control_points: usize = hit_object.curves.len() + 1;
    let mut points: Vec<pair_structs::Pairf64> = Default::default();
    let mut last_point = pair_structs::Pairf64{x: -1.0, y: -1.0};

    let mut i: usize = 0;
    while i < hit_object.curves.len() {
        slider.slider_xy.push(pair_structs::Pairf64{x: hit_object.curves[i].x, y: hit_object.curves[i].y});
        i += 1;
    }

    slider.xy.x = hit_object.pos.x;
    slider.xy.y = hit_object.pos.y;

    i = 0;
    while i < control_points {
        let t_point: pair_structs::Pairf64;
        if i == 0 {
            t_point = pair_structs::Pairf64 {x: slider.xy.x, y: slider.xy.y};
        } else {
            t_point = slider.slider_xy[i - 1];
        }

        if line {
            if last_point != (pair_structs::Pairf64 {x: -1.0, y: -1.0}) {
                points.push(t_point);
                beziers.push(bezier_fn(&points));
                points.clear();
            }
        } else if last_point != (pair_structs::Pairf64 {x: -1.0, y: -1.0}) && t_point == last_point {
            if points.len() >= 2 {
                beziers.push(bezier_fn(&points));
            }
            points.clear();
        }
        points.push(t_point);
        last_point = t_point;
        
        i += 1;
    }

    if line || points.len() < 2 {
    } else {
        beziers.push(bezier_fn(&points));
        points.clear();
    }

    let mut curves_list = beziers;

    let curve_points_separation: f64 = 5.0;
    slider.ncurve = (hit_object.pixel_length / curve_points_separation) as i32;
    slider.curve.resize(slider.ncurve as usize + 1, Default::default());

    if curves_list.len() == 0 {
        let object_pos_vec: Vec<pair_structs::Pairf64> = vec![hit_object.pos];
        curves_list.push(bezier_fn(&object_pos_vec));
        hit_object.end_point = hit_object.pos;
    }

    let mut distance_at: f64 = 0.0;
    let mut curve_counter: usize = 0;
    let mut cur_point: i32 = 0;
    let mut cur_curve: &structs::Bezier = &curves_list[curve_counter];
    curve_counter += 1;
    let mut last_curve: pair_structs::Pairf64 = cur_curve.curve_points[0];
    let mut last_distance_at: f64 = 0.0;

    let pixel_length: f64 = hit_object.pixel_length;

    let mut i: usize = 0;
    while i < (slider.ncurve + 1) as usize {
        let pref_distance: i32 = (i as f64 * pixel_length / slider.ncurve as f64) as i32;
        while distance_at < pref_distance as f64 {
            last_distance_at = distance_at;
            last_curve = cur_curve.curve_points[cur_point as usize];
            cur_point += 1;

            if cur_point >= cur_curve.ncurve {
                if curve_counter < curves_list.len() {
                    cur_curve = &curves_list[curve_counter];
                    curve_counter += 1;
                    cur_point = 0;
                } else {
                    cur_point = cur_curve.ncurve - 1;

                    if last_distance_at == distance_at {
                        break;
                    }
                }
            }
            distance_at += cur_curve.curve_dist[cur_point as usize];
        }
        let this_curve: pair_structs::Pairf64 = cur_curve.curve_points[cur_point as usize];

        if distance_at - last_distance_at > 1.0 {
            let t: f64 = (pref_distance as f64 - last_distance_at) / (distance_at - last_distance_at);
            slider.curve[i] = pair_structs::Pairf64{x: utils::lerp(last_curve.x, this_curve.x, t), y: (utils::lerp(last_curve.y, this_curve.y, t))};
        } else {
            slider.curve[i] = this_curve;
        }
        
        i += 1;
    }

    return slider;
} 

fn bezier_fn(points: &Vec<pair_structs::Pairf64>) -> structs::Bezier {
    let mut bezier: structs::Bezier = Default::default();
    bezier.points = points.clone();

    let mut approx_length: f64 = 0.0;
    let mut i: usize = 0;
    while i < points.len() - 1 {
        approx_length += pair_structs::get_distance_from(&points[i], &points[i + 1]);

        i += 1;
    }

    bezier.ncurve = (approx_length / 4.0) as i32 + 2;
    i = 0;
    while i < bezier.ncurve as usize {
        let mut c: pair_structs::Pairf64 = Default::default();
        let n: usize = points.len() - 1;
        let t: f64 = i as f64 / (bezier.ncurve - 1) as f64;
        let mut j: usize = 0;
        while j <= n {
            let b: f64 = utils::bernstien(j as i64, n as i64, t);
            c += pair_structs::Pairf64{x: bezier.points[j].x * b, y: bezier.points[j].y * b};
            j += 1;
        }
        bezier.curve_points.push(c);
        
        i += 1;
    }

    bezier.total_distance = 0.0;
    i = 0;
    while i < bezier.ncurve as usize {
        if i == 0 {
            bezier.curve_dist.push(0.0);
        } else {
            bezier.curve_dist.push(pair_structs::get_distance_from(&bezier.curve_points[i], &bezier.curve_points[i - 1]));
        }
        bezier.total_distance += bezier.curve_dist[i];

        i += 1;
    }

    return bezier;
}

pub fn circumscribed_circle(hit_object: structs::HitObject) -> structs::CircumscribedCircle {
    let mut slider: structs::Slider = Default::default();
    let mut circle: structs::CircumscribedCircle = Default::default();
    let curve_points_separation: i32 = 5;
    
    let mut i: usize = 0;
    while i < hit_object.curves.len()  {
        slider.slider_xy.push(hit_object.curves[i]);
        i += 1;
    }
    slider.xy = hit_object.pos;

    circle.start = slider.xy;
    circle.mid = slider.slider_xy[0];
    circle.end = slider.slider_xy[1];

    let mida: pair_structs::Pairf64 = pair_structs::mid_point(&circle.start, &circle.mid);
    let midb: pair_structs::Pairf64 = pair_structs::mid_point(&circle.end, &circle.mid);
    let nora = pair_structs::nor(&(circle.mid - circle.start));
    let norb = pair_structs::nor(&(circle.mid - circle.end));

    circle.circle_center = circumscribed_circle_intersect(mida, nora, midb, norb);
    if circle.circle_center == (pair_structs::Pairf64{x: -1.0, y: -1.0}) {
        slider = slider_fn(hit_object, true);
        circle.curve.resize(slider.curve.len(), Default::default());
        circle.curve = slider.curve;
        circle.ncurve = slider.ncurve;
        return circle;
    }

    let start_ang_point: pair_structs::Pairf64 = circle.start - circle.circle_center;
    let mid_ang_point: pair_structs::Pairf64 = circle.mid - circle.circle_center;
    let end_ang_point: pair_structs::Pairf64 = circle.end - circle.circle_center;
    
    circle.start_ang = f64::atan2(start_ang_point.y, start_ang_point.x);
    circle.mid_ang = f64::atan2(mid_ang_point.y, mid_ang_point.x);
    circle.end_ang = f64::atan2(end_ang_point.y, end_ang_point.x);

    let two_pi: f64 = std::f64::consts::PI * 2.0;

    if !circumscribed_circle_is_in(circle.start_ang, circle.mid_ang, circle.end_ang) {
        if f64::abs(circle.start_ang + two_pi - circle.end_ang) < two_pi && circumscribed_circle_is_in(circle.start_ang + two_pi, circle.mid_ang, circle.end_ang) {
            circle.start_ang += two_pi; 
        } else if f64::abs(circle.start_ang - (circle.end_ang + two_pi)) < two_pi && circumscribed_circle_is_in(circle.start_ang, circle.mid_ang, circle.end_ang + two_pi) {
            circle.end_ang += two_pi;
        } else if f64::abs(circle.start_ang - two_pi - circle.end_ang) < two_pi && circumscribed_circle_is_in(circle.start_ang - two_pi, circle.mid_ang, circle.end_ang) {
            circle.start_ang -= two_pi;
        } else if f64::abs(circle.start_ang - (circle.end_ang - two_pi)) < two_pi && circumscribed_circle_is_in(circle.start_ang, circle.mid_ang, circle.end_ang - two_pi) {
            circle.end_ang -= two_pi;
        } else {
            return circle;
        }
    }

    circle.radius = pair_structs::get_length(start_ang_point);
    let pixel_length: f64 = hit_object.pixel_length;
    let arc_ang: f64 = pixel_length / circle.radius;

    if circle.end_ang > circle.start_ang {
        circle.end_ang = circle.start_ang + arc_ang;
    } else {
        circle.end_ang = circle.start_ang - arc_ang;
    }

    let step: f64 = hit_object.pixel_length / curve_points_separation as f64;
    circle.ncurve = step as i32;
    let len: usize = step as usize + 1;
    let mut i: usize = 0;
    while i < len {
        let ang: f64 = utils::lerp(circle.start_ang, circle.end_ang, i as f64 / step);
        circle.curve.push(pair_structs::Pairf64 { x: f64::cos(ang) * circle.radius + circle.circle_center.x, y: f64::sin(ang) * circle.radius + circle.circle_center.y });
        
        i += 1;
    }

    return circle;
}

fn circumscribed_circle_intersect(a: pair_structs::Pairf64, ta: pair_structs::Pairf64, b: pair_structs::Pairf64, tb: pair_structs::Pairf64) -> pair_structs::Pairf64 {
    let des: f64 = tb.x * ta.y - tb.y * ta.x;
    if f64::abs(des) < 0.00001 {
        return pair_structs::Pairf64{x: -1.0, y: -1.0};
    }

    let u: f64 = ((b.y - a.y) * ta.x + (a.x - b.x) * ta.y) / des;
    let b_new = pair_structs::Pairf64{x: b.x + (tb.x * u), y: b.y + (tb.y * u)};

    return b_new;
}

fn circumscribed_circle_is_in(a: f64, b: f64, c: f64) -> bool {
    return (b > a && b < c) || (b < a && b > c);
}