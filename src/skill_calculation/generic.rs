use crate::structs;
use crate::pair_structs;
use crate::utils;
use crate::skill_calculation;

pub fn prepare_aim_data(mut beatmap: structs::Beatmap) -> structs::Beatmap {
    beatmap = calculate_movement_data(beatmap);
    beatmap = gather_target_points(beatmap);
    beatmap = gather_aim_points(beatmap);
    beatmap = calculate_angles(beatmap);
    return beatmap;
}

pub fn prepare_tap_data(mut beatmap: structs::Beatmap) -> structs::Beatmap {
    beatmap = calculate_press_intervals(beatmap);
    beatmap = gather_tap_patterns(beatmap);
    return beatmap;
}

fn calculate_movement_data(mut beatmap: structs::Beatmap) -> structs::Beatmap {
    let mut previous_pos: pair_structs::Pairf64 = Default::default();
    let mut previous_time: i64 = -1;

    let mut i: usize = 0;
    while i < beatmap.hit_objects.len() {
        if (utils::is_hit_object_type(&beatmap.hit_objects[i].hit_object_type, structs::HitObjectType::Normal) || utils::is_hit_object_type(&beatmap.hit_objects[i].hit_object_type, structs::HitObjectType::Slider)) && previous_time != -1 {
            let mut distance: f64 = pair_structs::get_distance_from(&beatmap.hit_objects[i].pos, &previous_pos);
            let rad_subtract: f64 = 2.0 * utils::cs_to_px(beatmap.cs as i32 as f64) as f64;
            let interval: f64 = (beatmap.hit_objects[i].time - previous_time) as f64;

            if distance >= rad_subtract {
                distance -= rad_subtract;
            } else {
                distance /= 2.0;
            }

            beatmap.distances.push(distance);
            let dist_xy: pair_structs::Pairf64 = pair_structs::Pairf64 { x: beatmap.hit_objects[i].pos.x - previous_pos.x, y: beatmap.hit_objects[i].pos.y - previous_pos.y };
            beatmap.velocities.push(pair_structs::Pairf64{x: dist_xy.x / interval, y: dist_xy.y / interval});
        }

        if utils::is_hit_object_type(&beatmap.hit_objects[i].hit_object_type, structs::HitObjectType::Normal) || utils::is_hit_object_type(&beatmap.hit_objects[i].hit_object_type, structs::HitObjectType::Slider) {
            previous_pos = beatmap.hit_objects[i].pos;
            previous_time = beatmap.hit_objects[i].time;
        }

        i += 1;
    }

    let mut old_vel: pair_structs::Pairf64 = Default::default();

    i = 0;
    while i < beatmap.velocities.len() {
        let vel: pair_structs::Pairf64 = pair_structs::Pairf64 { x: beatmap.velocities[i].x, y: beatmap.velocities[i].y };
        if i != 0 {
            beatmap.velocities_change.push(vel - old_vel);
        }
        old_vel = vel;

        i += 1;
    }

    return beatmap;
}

fn gather_target_points(mut beatmap: structs::Beatmap) -> structs::Beatmap {
    let mut target_point: structs::Timing;
    let mut i: i32 = 0;
    let mut prev_time: i64 = -1;

    for hit_obj in &beatmap.hit_objects {
        if i64::abs(hit_obj.time - prev_time) < 5 && i > 0 {
            continue;
        }
        prev_time = hit_obj.time;

        if utils::is_hit_object_type(&hit_obj.hit_object_type, structs::HitObjectType::Normal) {
            target_point = structs::Timing{time: hit_obj.time, pos: hit_obj.pos, key: i, press: false, data: Default::default()};

            beatmap.target_points.push(target_point);
        } else if utils::is_hit_object_type(&hit_obj.hit_object_type, structs::HitObjectType::Slider) {
            for tick in &hit_obj.ticks {
                target_point = structs::Timing{time: *tick as i64, pos: skill_calculation::slider::get_slider_pos(hit_obj, *tick), key: i, press: true, data: Default::default()};

                beatmap.target_points.push(target_point);
            }
        }
        i += 1;
    }
    return beatmap;
}

fn gather_aim_points(mut beatmap: structs::Beatmap) -> structs::Beatmap {
    for hit_obj in &beatmap.hit_objects {
        if utils::is_hit_object_type(&hit_obj.hit_object_type, structs::HitObjectType::Normal) {
            beatmap.aim_points.push(structs::AimPoint { time: hit_obj.time as i32, pos: hit_obj.pos, aim_point_type: structs::AimPointTypes::AimPointCircle });
        } else if utils::is_hit_object_type(&hit_obj.hit_object_type, structs::HitObjectType::Slider) {
            beatmap.aim_points.push(structs::AimPoint { time: hit_obj.time as i32, pos: hit_obj.pos, aim_point_type: structs::AimPointTypes::AimPointSlider });

            let end_time: i32 = utils::get_last_tick_time(hit_obj);
            let end_pos: pair_structs::Pairf64 = skill_calculation::slider::get_slider_pos(hit_obj, end_time);

            if hit_obj.ticks.len() != 0 || pair_structs::get_distance_from(&hit_obj.pos, &end_pos) > 2.0 * utils::cs_to_px(beatmap.cs as i32 as f64) as f64 {
                beatmap.aim_points.push(structs::AimPoint { time: end_time, pos: end_pos, aim_point_type: structs::AimPointTypes::AimPointSliderend });
            }
        }
    }
    return beatmap;
}

fn calculate_angles(mut beatmap: structs::Beatmap) -> structs::Beatmap {
    let mut i: usize = 0;
    while i + 2 < beatmap.aim_points.len() {
        let angle: f64 = utils::get_dir_angle(beatmap.aim_points[i].pos, beatmap.aim_points[i + 1].pos, beatmap.aim_points[i + 2].pos);
        beatmap.angles.push(angle);

        i += 1;
    }

    if beatmap.angles.len() > 0 {
        i = 0;
        let mut old_angle: f64 = beatmap.angles[0] - 2.0 * beatmap.angles[0];
        for angle in &beatmap.angles {
            let angle_deref = *angle;
            let bonus: f64;
            let absd: f64 = f64::abs(angle_deref);
            if utils::sign(angle_deref) == utils::sign(old_angle) {
                if absd < 90.0 {
                    bonus = f64::sin(utils::deg_to_rad(absd) * 0.784 + 0.339837);
                } else {
                    bonus = f64::sin(utils::deg_to_rad(absd));
                }
            } else {
                if absd < 90.0 {
                    bonus = f64::sin(utils::deg_to_rad(absd) * 0.536 + 0.72972);
                } else {
                    bonus = f64::sin(utils::deg_to_rad(absd)) / 2.0;
                }
            }
            beatmap.angle_bonuses.push(bonus);
            old_angle = angle_deref;

            i += 1;
        }
    }

    return beatmap;
}

pub fn prepare_timing_points(mut beatmap: structs::Beatmap) -> structs::Beatmap {
    beatmap.bpm_min = 10000.0;
    beatmap.bpm_max = 0.0;

    let mut bpm: f64 = 0.0;
    let mut slider_mult: f64;
    let mut old_beat: f64 = -100.0;

    let mut i: usize = 0;
    while i < beatmap.timing_points.len() {
        if beatmap.timing_points[i].inherited {
            if beatmap.timing_points[i].beat_interval <= 0.0 {
                slider_mult = beatmap.timing_points[i].beat_interval;
                old_beat = beatmap.timing_points[i].beat_interval;
            } else {
                slider_mult = old_beat;
            }
        } else {
            slider_mult = -100.0;
            bpm = 60000.0 / beatmap.timing_points[i].beat_interval;

            if beatmap.bpm_min > bpm {
                beatmap.bpm_min = bpm;
            }
            if beatmap.bpm_max < bpm {
                beatmap.bpm_max = bpm;
            }
        }
        beatmap.timing_points[i].bpm = bpm;
        beatmap.timing_points[i].sm = slider_mult;

        i += 1;
    }

    return beatmap;
}

pub fn bake_slider_data(mut beatmap: structs::Beatmap) -> structs::Beatmap {
    let mut i: usize = 0;
    while i < beatmap.hit_objects.len() {
        if utils::is_hit_object_type(&beatmap.hit_objects[i].hit_object_type, structs::HitObjectType::Slider) {

            match beatmap.hit_objects[i].curve_type {
                structs::CurveType::BezierCurve => {
                    let slider_data: structs::Slider = skill_calculation::slider::slider_fn(beatmap.hit_objects[i].clone(), false);
                    beatmap.hit_objects[i].lerp_points.resize(slider_data.curve.len(), Default::default());
                    beatmap.hit_objects[i].lerp_points = slider_data.curve;
                    beatmap.hit_objects[i].ncurve = slider_data.ncurve;
                },
                structs::CurveType::PerfectCurve => {
                    if beatmap.hit_objects[i].curves.len() == 2 {
                        let circle_data: structs::CircumscribedCircle = skill_calculation::slider::circumscribed_circle(beatmap.hit_objects[i].clone());
                        beatmap.hit_objects[i].lerp_points.resize(circle_data.curve.len(), Default::default());
                        beatmap.hit_objects[i].lerp_points = circle_data.curve;
                        beatmap.hit_objects[i].ncurve = circle_data.ncurve;
                    } else {
                        let slider_data: structs::Slider = skill_calculation::slider::slider_fn(beatmap.hit_objects[i].clone(), false);
                        beatmap.hit_objects[i].lerp_points.resize(slider_data.curve.len(), Default::default());
                        beatmap.hit_objects[i].lerp_points = slider_data.curve;
                        beatmap.hit_objects[i].ncurve = slider_data.ncurve;
                    }
                },
                structs::CurveType::LinearCurve => {
                    let slider_data: structs::Slider = skill_calculation::slider::slider_fn(beatmap.hit_objects[i].clone(), true);
                    beatmap.hit_objects[i].lerp_points.resize(slider_data.curve.len(), Default::default());
                    beatmap.hit_objects[i].lerp_points = slider_data.curve;
                    beatmap.hit_objects[i].ncurve = slider_data.ncurve;
                },
                structs::CurveType::CatmullCurve => {
                    let slider_data: structs::Slider = skill_calculation::slider::slider_fn(beatmap.hit_objects[i].clone(), true);
                    beatmap.hit_objects[i].lerp_points.resize(slider_data.curve.len(), Default::default());
                    beatmap.hit_objects[i].lerp_points = slider_data.curve;
                    beatmap.hit_objects[i].ncurve = slider_data.ncurve;
                },
                _ => ()
            }

            if (beatmap.hit_objects[i].repeat % 2) != 0 {
                beatmap.hit_objects[i].end_point = match beatmap.hit_objects[i].lerp_points.last() {
                    Some(some) => *some,
                    None => Default::default()
                };
            } else {
                beatmap.hit_objects[i].end_point = match beatmap.hit_objects[i].lerp_points.first() {
                    Some(some) => *some,
                    None => Default::default()
                };
            }
        }
        
        i += 1;
    }
    return beatmap;
}

fn calculate_press_intervals(mut beatmap: structs::Beatmap) -> structs::Beatmap {
    let mut previous_time: i64 = -1;
    let mut i: usize = 0;
    while i < beatmap.hit_objects.len() {
        if utils::is_hit_object_type(&beatmap.hit_objects[i].hit_object_type, structs::HitObjectType::Normal) || utils::is_hit_object_type(&beatmap.hit_objects[i].hit_object_type, structs::HitObjectType::Slider) {
            if i > 0 {
                beatmap.press_intervals.push((beatmap.hit_objects[i].time - previous_time) as i32);
            }
            previous_time = beatmap.hit_objects[i].time;
        }

        i += 1;
    }
    return beatmap;
}

fn gather_tap_patterns(mut beatmap: structs::Beatmap) -> structs::Beatmap {
    let mut old: i32 = 0;
    let mut tmp: Vec<i32> = Default::default();
    let mut i: usize = 0;
    let offset_max_displacement: i32 = 2;

    while i < beatmap.press_intervals.len() {
        if i32::abs(beatmap.press_intervals[i] - old) > offset_max_displacement {
            if tmp.len() > 6 {
                beatmap.streams.push(pair_structs::Pairi32VectorVectori32{x: old, y: vec![tmp.clone()]})
            } else if tmp.len() > 1 {
                beatmap.bursts.push(pair_structs::Pairi32VectorVectori32{x: old, y: vec![tmp.clone()]})
            }
            tmp.clear();
        }
        tmp.push(beatmap.hit_objects[i].time as i32);
        old = beatmap.press_intervals[i];
        i += 1;
    }

    if tmp.len() > 6 {
        beatmap.streams.push(pair_structs::Pairi32VectorVectori32{x: old, y: vec![tmp.clone()]});
    } else if tmp.len() > 1 {
        beatmap.streams.push(pair_structs::Pairi32VectorVectori32{x: old, y: vec![tmp.clone()]});
    }

    beatmap.streams.sort();
    beatmap.bursts.sort();

    return beatmap;
}