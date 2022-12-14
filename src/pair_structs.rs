#[derive(Copy, Clone, PartialEq)]
pub struct Pairf64 {
    pub x: f64,
    pub y: f64
}

impl Default for Pairf64 {
    fn default() -> Self {Pairf64 { x: (0.0), y: (0.0) }}
}

impl std::ops::Sub for Pairf64 {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self::Output {
        Pairf64 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Add for Pairf64 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self::Output {
        Pairf64 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Div for Pairf64 {
    type Output = Self;
    
    fn div(self, other: Self) -> Self::Output {
        Pairf64 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl std::ops::Mul for Pairf64 {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self::Output {
        Pairf64 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl std::ops::AddAssign for Pairf64 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::ops::SubAssign for Pairf64 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl std::ops::MulAssign for Pairf64 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}

impl std::ops::DivAssign for Pairf64 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Pairi64 {
    pub x: i64,
    pub y: i64
}

impl Default for Pairi64 {
    fn default() -> Self {Pairi64 { x: (0), y: (0) }}
}

impl std::ops::Sub for Pairi64 {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self::Output {
        Pairi64 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Add for Pairi64 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self::Output {
        Pairi64 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Div for Pairi64 {
    type Output = Self;
    
    fn div(self, other: Self) -> Self::Output {
        Pairi64 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl std::ops::Mul for Pairi64 {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self::Output {
        Pairi64 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl std::ops::AddAssign for Pairi64 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::ops::SubAssign for Pairi64 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl std::ops::MulAssign for Pairi64 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}

impl std::ops::DivAssign for Pairi64 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Pairi32 {
    pub x: i32,
    pub y: i32
}

impl Default for Pairi32 {
    fn default() -> Self {Pairi32 { x: (0), y: (0) }}
}

impl std::ops::Sub for Pairi32 {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self::Output {
        Pairi32 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Add for Pairi32 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self::Output {
        Pairi32 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Div for Pairi32 {
    type Output = Self;
    
    fn div(self, other: Self) -> Self::Output {
        Pairi32 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl std::ops::Mul for Pairi32 {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self::Output {
        Pairi32 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl std::ops::AddAssign for Pairi32 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::ops::SubAssign for Pairi32 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl std::ops::MulAssign for Pairi32 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}

impl std::ops::DivAssign for Pairi32 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }
}

#[derive(Clone)]
pub struct Pairi32VectorVectori32 {
    pub x: i32,
    pub y: Vec<Vec<i32>>
}

impl Default for Pairi32VectorVectori32 {
    fn default() -> Self {Pairi32VectorVectori32 { x: (Default::default()), y: (Default::default()) }}
}

impl PartialEq for Pairi32VectorVectori32 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl Eq for Pairi32VectorVectori32 {}

impl PartialOrd for Pairi32VectorVectori32 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl Ord for Pairi32VectorVectori32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x)
    }
}

#[derive(Clone)]
pub struct Pairi32VectorPairf64 {
    pub x: i32,
    pub y: Vec<Pairf64>
}

impl Default for Pairi32VectorPairf64 {
    fn default() -> Self {Pairi32VectorPairf64 { x: (0), y: (Default::default()) }}
}

pub fn get_length(pairf64_0: Pairf64) -> f64 {
    return f64::sqrt(pairf64_0.x * pairf64_0.x + pairf64_0.y * pairf64_0.y);
}

pub fn get_distance_from(pairf64_0: &Pairf64, pairf64_1: &Pairf64) -> f64 {
    return get_length(Pairf64{ x: pairf64_0.x - pairf64_1.x, y: pairf64_0.y - pairf64_1.y });
}

pub fn mid_point(pairf64_0: &Pairf64, pairf64_1: &Pairf64) -> Pairf64 {
    return (*pairf64_0 + *pairf64_1) / Pairf64 { x: 2.0, y: 2.0 };
}

pub fn nor(pairf64_0: &Pairf64) -> Pairf64 {
    return Pairf64 { x: -pairf64_0.y, y: pairf64_0.x };
}