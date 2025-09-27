use std::collections::HashMap;

use crate::pair_structs;

#[derive(Clone, Copy)]
pub enum ConfigPreset {
    Classic,
    Rebalance1,
}

#[derive(Default, Clone, Copy)]
pub enum CalculationAlgorithm {
    #[default]
    Default,
    Classic,
    Rebalance1,
}

#[derive(Default, Clone, Copy)]
pub enum OutputType {
    #[default]
    Stdout,
    Txt,
    Csv,
}

impl std::fmt::Display for OutputType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OutputType::Stdout => write!(f, "stdout"),
            OutputType::Txt => write!(f, "txt"),
            OutputType::Csv => write!(f, "csv"),
        }
    }
}

#[derive(Default, PartialEq, Debug)]
pub struct Timing {
    pub time: i64,
    pub data: f64,
    pub key: i32,
    pub press: bool,

    pub pos: pair_structs::Pairf64,
}

#[derive(Copy, Clone)]
pub enum Mods {
    NM = 0,
    //NF = 1,
    EZ = 2,
    HD = 8,
    HR = 16,
    //SD = 32,
    DT = 64,
    //RL = 128,
    HT = 256,
    //FL = 1024,
    //AU = 2048,
    //SO = 4096,
    //AP = 8192
}

impl Default for Mods {
    fn default() -> Self {
        Mods::NM
    }
}

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct TimingPoint {
    pub offset: f64,
    pub beat_interval: f64,
    pub meter: f64,
    pub inherited: bool,
    pub sm: f64,
    pub bpm: f64,
}

#[derive(Clone, PartialEq)]

pub enum HitObjectType {
    None = 0,
    Normal = 1,
    Slider = 2,
    NewCombo = 4,
    //NormalNewCombo = 5,
    //SliderNewCombo = 6,
    Spinner = 8,
    ColourHax = 112,
    //Hold = 128,
    //ManiaLong = 128
}

impl Default for HitObjectType {
    fn default() -> Self {
        HitObjectType::None
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CurveType {
    None,
    PerfectCurve,
    BezierCurve,
    LinearCurve,
    CatmullCurve,
}

impl Default for CurveType {
    fn default() -> Self {
        CurveType::None
    }
}

#[derive(Default)]
pub struct Bezier {
    pub points: Vec<pair_structs::Pairf64>,
    pub curve_points: Vec<pair_structs::Pairf64>,
    pub curve_dist: Vec<f64>,
    pub ncurve: i32,
    pub total_distance: f64,
}

#[derive(Default)]
pub struct CircumscribedCircle {
    pub curve: Vec<pair_structs::Pairf64>,
    pub ncurve: i32,
    pub circle_center: pair_structs::Pairf64,
    pub radius: f64,
    pub start: pair_structs::Pairf64,
    pub mid: pair_structs::Pairf64,
    pub end: pair_structs::Pairf64,
    pub start_ang: f64,
    pub end_ang: f64,
    pub mid_ang: f64,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct HitObject {
    pub pos: pair_structs::Pairf64,
    pub time: i64,
    pub hit_object_type: i32,

    pub curve_type: CurveType,
    pub curves: Vec<pair_structs::Pairf64>,
    pub lerp_points: Vec<pair_structs::Pairf64>,
    pub ncurve: i32,
    pub repeat: i32,
    pub repeat_times: Vec<i64>,
    pub pixel_length: f64,
    pub end_time: i64,
    pub to_repeat_time: i64,
    pub end_point: pair_structs::Pairf64,
    pub ticks: Vec<i64>,
}

#[derive(Default)]
pub struct Slider {
    pub curve: Vec<pair_structs::Pairf64>,
    // pub curve_points_separation: i32,
    // pub start_angle: f64,
    // pub end_angle: f64,
    pub xy: pair_structs::Pairf64,
    pub slider_xy: Vec<pair_structs::Pairf64>,
    pub ncurve: i32,
}

// #[derive(Default, PartialEq)]
// pub struct Burst {
//     pub interval: i32,
//     pub strain: f64,
// }

#[derive(Default, PartialEq)]
pub struct Stream {
    pub interval: i32,
    pub length: i32,
}

#[derive(PartialEq, Debug)]
pub enum AimPointTypes {
    AimPointNone,
    AimPointCircle,
    AimPointSlider,
    //AimPointSliderreverse,
    AimPointSliderend,
}

impl Default for AimPointTypes {
    fn default() -> Self {
        AimPointTypes::AimPointNone
    }
}

#[derive(PartialEq, Debug)]
pub struct AimPoint {
    pub time: i64,
    pub pos: pair_structs::Pairf64,
    pub aim_point_type: AimPointTypes,
}

#[derive(Default, PartialEq, Debug)]
pub struct Skills {
    pub stamina: f64,
    pub tenacity: f64,
    pub agility: f64,
    pub precision: f64,
    pub memory: f64,
    pub accuracy: f64,
    pub reaction: f64,
}

#[derive(Default, PartialEq, Debug)]
pub struct Beatmap {
    pub format: String,

    pub title: String,
    pub title_unicode: String,
    pub artist: String,
    pub artist_unicode: String,
    pub creator: String,
    pub version: String,
    pub source: String,
    pub tags: String,
    pub mode: i32,
    pub beatmap_id: String,
    pub beatmap_set_id: String,

    pub beatmap_md5: String,

    pub hp: f64,
    pub cs: f64,
    pub od: f64,
    pub ar: f64,
    pub sm: f64,
    pub st: f64,
    pub bpm_min: f64,
    pub bpm_max: f64,

    pub timing_points: Vec<TimingPoint>,
    pub hit_objects: Vec<HitObject>,
    pub aim_points: Vec<AimPoint>,
    pub target_points: Vec<Timing>,

    pub spinners: i32,

    pub velocities: Vec<pair_structs::Pairf64>,
    pub velocities_change: Vec<pair_structs::Pairf64>,
    pub distances: Vec<f64>,
    pub aim_strains: Vec<f64>,
    pub angle_strains: Vec<f64>,

    pub angles: Vec<f64>,
    pub angle_bonuses: Vec<f64>,
    pub reaction_times: Vec<i32>,

    pub press_intervals: Vec<i32>,
    pub tap_strains: Vec<f64>,

    pub streams: HashMap<i32, Vec<Vec<i32>>>,

    pub skills: Skills,

    pub mods: i32,

    pub compressed_stream: Vec<i32>,
    pub stream: Vec<i32>,
    pub stack: Vec<i32>,

    pub skill_calculation_vars: SkillCalculationVars,
}

#[derive(Default, PartialEq, Debug)]
pub struct SkillCalculationVars {
    pub reaction: SkillCalculationReactionVars,
    pub precision: SkillCalculationPrecisionVars,
    pub accuracy: SkillCalculationAccuracyVars,
    pub agility: SkillCalculationAgilityVars,
    pub tenacity: SkillCalculationTenacityVars,
    pub memory: SkillCalculationMemoryVars,
    pub stamina: SkillCalculationStaminaVars,
}

#[derive(Default, PartialEq, Debug)]
pub struct SkillCalculationReactionVars {
    pub curve_exp: f64,
    pub fade_in_percent: f64,
    pub ver_scale: f64,
    pub pattern_damping: f64,
    pub avg_weighting: f64,
}

#[derive(Default, PartialEq, Debug)]
pub struct SkillCalculationPrecisionVars {
    pub total_pow: f64,
    pub total_mult: f64,
    pub agility_subtract: f64,
    pub agility_pow: f64,
    pub agility_limit: f64,
}

#[derive(Default, PartialEq, Debug)]
pub struct SkillCalculationAccuracyVars {
    pub total_pow: f64,
    pub total_mult: f64,
    pub ver_scale: f64,
    pub acc_scale: f64,
}

#[derive(Default, PartialEq, Debug)]
pub struct SkillCalculationAgilityVars {
    pub total_mult: f64,
    pub weighting: f64,
    pub total_pow: f64,
    pub slider_strain_decay: f64,
    pub angle_mult: f64,
    pub time_pow: f64,
    pub time_mult: f64,
    pub dist_divisor: f64,
    pub strain_decay: f64,
    pub dist_pow: f64,
    pub dist_mult: f64,
}

#[derive(Default, PartialEq, Debug)]
pub struct SkillCalculationTenacityVars {
    pub total_pow: f64,
    pub total_mult: f64,
    pub length_mult: f64,
    pub length_divisor: f64,
    pub interval_mult2: f64,
    pub interval_pow: f64,
    pub interval_mult: f64,
}

#[derive(Default, PartialEq, Debug)]
pub struct SkillCalculationMemoryVars {
    pub total_mult: f64,
    pub sliderbuff: f64,
    pub total_pow: f64,
    pub followpoints_nerf: f64,
}

#[derive(Default, PartialEq, Debug)]
pub struct SkillCalculationStaminaVars {
    pub total_pow: f64,
    pub largest_interval: f64,
    pub pow: f64,
    pub total_mult: f64,
    pub scale: f64,
    pub mult: f64,
    pub decay_max: f64,
    pub decay: f64,
}
