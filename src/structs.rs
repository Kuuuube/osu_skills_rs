use crate::pair_structs;

#[derive(Default)]
pub struct Timing {
    pub time: i64,
    pub data: f64,
    pub key: i32,
    pub press: bool,

    pub pos: pair_structs::Pairf64
}

#[derive(Copy, Clone)]
pub enum Mods {
    NM = 0,
    NF = 1,
    EZ = 2,
    HD = 8,
    HR = 16,
    SD = 32,
    DT = 64,
    RL = 128,
    HT = 256,
    FL = 1024,
    AU = 2048,
    SO = 4096,
    AP = 8192
}

impl Default for Mods {
    fn default() -> Self {Mods::NM}
}

#[derive(Default, Copy, Clone)]
pub struct TimingPoint {
    pub offset: i32,
    pub beat_interval: f64,
    pub meter: i32,
    pub inherited: bool,
    pub sm: f64,
    pub bpm: f64
}

#[derive(Clone, PartialEq)]

pub enum HitObjectType {
    None = 0,
    Normal = 1,
    Slider = 2,
    NewCombo = 4,
    NormalNewCombo = 5,
    SliderNewCombo = 6,
    Spinner = 8,
    ColourHax = 112,
    Hold = 128,
    //ManiaLong = 128
}

impl Default for HitObjectType {
    fn default() -> Self {HitObjectType::None}
}

#[derive(Copy, Clone)]
pub enum CurveType {
    None,
    PerfectCurve,
    BezierCurve,
    LinearCurve,
    CatmullCurve
}

impl Default for CurveType {
    fn default() -> Self {CurveType::None}
}

#[derive(Default)]
pub struct Bezier {
    pub points: Vec<pair_structs::Pairf64>,
    pub curve_points: Vec<pair_structs::Pairf64>,
    pub curve_dist: Vec<f64>,
    pub ncurve: i32,
    pub total_distance: f64,
    pub approx_length: f64
}

#[derive(Default, Clone)]
pub struct HitObject {
    pub pos: pair_structs::Pairf64,
    pub time: i64,
    pub hit_object_type: i32,

    pub curve_type: CurveType,
    pub curves: Vec<pair_structs::Pairf64>,
    pub lerp_points: Vec<pair_structs::Pairf64>,
    pub ncurve: i32,
    pub repeat: i32,
    pub repeat_times: Vec<i32>,
    pub pixel_length: f64,
    pub end_time: i32,
    pub to_repeat_time: i32,
    pub end_point: pair_structs::Pairf64,
    pub ticks: Vec<i32>
}

#[derive(Default)]
pub struct Slider {
    pub curve_points_separation: i32,
    pub start_angle: f64,
    pub end_angle: f64,
    pub x: f64,
    pub y: f64,
    pub slider: Vec<pair_structs::Pairf64>
}

#[derive(Default)]
pub struct Burst {
    pub interval: i32,
    pub strain: f64
}

#[derive(Default)]
pub struct Stream {
    pub interval: i32,
    pub length: i32
}

#[derive(PartialEq)]
pub enum AimPointTypes {
    AimPointNone,
    AimPointCircle,
    AimPointSlider,
    AimPointSliderreverse,
    AimPointSliderend
}

impl Default for AimPointTypes {
    fn default() -> Self {AimPointTypes::AimPointNone}
}

pub struct AimPoint {
    pub time: i32,
    pub pos: pair_structs::Pairf64,
    pub aim_point_type: AimPointTypes
}

#[derive(Default)]
pub struct Skills {
    pub stamina: f64,
    pub tenacity: f64,
    pub agility: f64,
    pub precision: f64,
    pub reading: f64,
    pub memory: f64,
    pub accuracy: f64,
    pub reaction: f64,
}

#[derive(Default)]
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
    pub beatmap_id: String,
    pub beatmap_set_id: String,

    pub name: String, //beatmap.artist + " - " + beatmap.title + " (" + beatmap.creator + ") [" + beatmap.version + "]"

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

    pub time_mapper: Vec<pair_structs::Pairi32>,

    pub velocities: Vec<pair_structs::Pairf64>,
    pub velocities_change: Vec<pair_structs::Pairf64>,
    pub distances: Vec<f64>,
    pub aim_strains: Vec<f64>,
    pub angle_strains: Vec<f64>,
    
    pub angles: Vec<f64>,
    pub angle_bonuses: Vec<f64>,
    pub reaction_times: Vec<i32>,

    pub press_intervals: Vec<f64>,
    pub tap_strains: Vec<f64>,

    pub test: Vec<Vec<i32>>,

    pub streams: Vec<pair_structs::Pairi32VectorVectori32>,
    pub bursts: Vec<pair_structs::Pairi32VectorVectori32>,

    pub skills: Skills,

    pub mods: Mods,
    pub mods_string: String,

    pub compressed_stream: Vec<i32>,
    pub stream: Vec<i32>,
    pub stack: Vec<i32>
}