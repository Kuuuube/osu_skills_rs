use crate::structs;
use crate::pair_structs;

fn get_longest_stream(streams: &Vec<pair_structs::Pairi32VectorVectori32>) -> structs::Stream {
    let mut max: usize = 1;
    let mut interval: i32 = 0;

    for stream in streams {
        interval = stream.x;
        max = 1;

        for j in &stream.y {
            let length: usize = j.len() + 1;
            if length > max {
                max = length;
            }
        }
        if max > 1 {
            break;
        }
    }

    return structs::Stream{interval: {interval}, length: {max as i32}};
}

pub fn calculate_tenacity(beatmap: &structs::Beatmap) -> f64 {
    let longest_stream: structs::Stream = get_longest_stream(&beatmap.streams);

    let interval_mult: f64 = 0.37; //this value comes from osu skills config file "IntervalMult"
    let interval_mult2: f64 = 13000.0; //this value comes from osu skills config file "IntervalMult2"
    let interval_pow: f64 = 0.143; //this value comes from osu skills config file "IntervalPow"
    let length_divisor: f64 = 0.08; //this value comes from osu skills config file "LengthDivisor"
    let length_mult: f64 = 15.0; //this value comes from osu skills config file "LengthMult"
    let total_mult: f64 = 5.0; //this value comes from osu skills config file "TotalMult"
    let total_pow: f64 = 0.75; //this value comes from osu skills config file "TotalPow"

    let interval_scaled: f64 = 1.0 / f64::powf(longest_stream.interval as f64, f64::powf(longest_stream.interval as f64, interval_pow) * interval_mult) * interval_mult2;
    let length_scaled: f64 = f64::powf(length_divisor / longest_stream.length as f64, length_divisor / longest_stream.length as f64 * length_mult);
    let tenacity: f64 = total_mult * f64::powf(interval_scaled * length_scaled, total_pow);
    return tenacity;
}