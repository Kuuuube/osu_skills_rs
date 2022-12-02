use crate::structs;
use crate::pair_structs;
use crate::utils;

fn calculate_movement_data(mut beatmap: structs::Beatmap) {
    let mut previous_pos: pair_structs::Pairf64 = Default::default();
    let mut previous_time: i64 = -1;

    let mut i: usize = 0;
    while i < beatmap.hit_objects.len() {
        if (utils::is_hit_object_type(&beatmap.hit_objects[i].hit_object_type, structs::HitObjectType::Normal) || utils::is_hit_object_type(&beatmap.hit_objects[i].hit_object_type, structs::HitObjectType::Slider)) && previous_time != -1 {
            let mut distance: f64 = pair_structs::get_distance_from(&beatmap.hit_objects[i].pos, &previous_pos);
            let rad_subtract: f64 = 2.0 * utils::cs_to_px(beatmap.cs);
            let interval: f64 = beatmap.hit_objects[i].time as f64 - previous_time as f64;

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
        let vel: pair_structs::Pairf64 = pair_structs::Pairf64 { x: (i as f64), y: (i as f64) };
        if i != 0 {
            beatmap.velocities_change.push(vel - old_vel);
        }
        old_vel = vel;

        i += 1;
    }
}