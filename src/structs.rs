use std::iter::Map;

pub struct Vector2F64 {
    pub x: f64,
    pub y: f64
}

pub struct Timing {
    pub time: i64,
    pub data: f64,
    pub key: i32,
    pub press: bool,

    pub pos: Vector2F64
}

pub enum Mods {
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

pub struct TimingPoint {
    pub offset: i32,
    pub beat_interval: f64,
    pub meter: i32,
    pub inherited: bool,
    pub sm: f64,
    pub bpm: f64
}

pub enum HitObjectType {
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

 pub enum CurveType {
    PerfectCurve,
    BezierCurve,
    LinearCurve,
    CatmullCurve
} 

pub struct HitObject {
    pub pos: Vector2F64,
    pub time: i32,
    pub hit_object_type: i32,

    pub curve_type: CurveType,
    pub curves: Vec<Vector2F64>,
    pub lerp_points: Vec<Vector2F64>,
    pub ncurve: i32,
    pub repeat: i32,
    pub repeat_times: Vec<i32>,
    pub pixel_length: f64,
    pub end_time: i32,
    pub to_repeat_time: i32,
    pub endpoint: Vector2F64,
    pub ticks: Vec<i32>
}

pub struct Burst {
    pub interval: i32,
    pub strain: f64
}

pub struct Stream {
    interval: i32,
    length: i32
}

pub enum AimPointTypes {
    AimPointNone,
    AimPointCircle,
    AimPointSlider,
    AimPointSliderreverse,
    AimPointSliderend
}

pub struct AimPoint {
    pub time: i32,
    pub pos: Vector2F64,
    pub aim_point_type: AimPointTypes
}

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

pub struct Beatmap {
    pub format: i32,

    pub artist: String,
    pub title: String,
    pub version: String,
    pub creator: String,
    pub name: String,

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

    //This might be incorrect. Original c++ code:
    //std::map<int, int> timeMapper;
    pub time_mapper: Map<i32, i32>,

    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub x_change: Vec<f64>,
    pub y_change: Vec<f64>,
    pub distances: Vec<f64>,
    pub aim_strains: Vec<f64>,
    pub angle_strains: Vec<f64>,
    
    pub angles: Vec<f64>,
    pub angle_bonuses: Vec<f64>,
    pub reaction_times: Vec<i32>,

    pub press_intervals: Vec<f64>,
    pub tap_strains: Vec<f64>,

    //This might be incorrect. Original c++ code:
    //std::map<int, std::vector<std::vector<int>>> streams;
    //std::map<int, std::vector<std::vector<int>>> bursts;
    pub streams: Map<i32, Vec<Vec<i32>>>,
    pub bursts: Map<i32, Vec<Vec<i32>>>,

    pub skills: Skills,

    pub mods: i32,
    pub mods_string: String,

    pub compressed_stream: Vec<i32>,
    pub stream: Vec<i32>,
    pub stack: Vec<i32>
}