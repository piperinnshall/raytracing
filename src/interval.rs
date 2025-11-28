#[derive(Clone, Copy)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    const EMPTY: Self = Self { min: f64::INFINITY, max: f64::NEG_INFINITY };
    const UNIVERSE: Self = Self{ min: f64::NEG_INFINITY, max: f64::INFINITY } ;

    pub fn new(min: f64, max: f64) -> Self { Self { min, max } } 

    pub fn size(&self) -> f64 { self.max - self.min } 
    pub fn contains(&self, n: f64) -> bool { self.min <= n && n <= self.max }
    pub fn surrounds(&self, n: f64) -> bool { self.min < n && n < self.max }

    pub fn clamp(&self, n:f64) -> f64 { 
        if n < self.min { self.min } 
        else if n > self.max { self.max } 
        else { n }
    }

    pub fn min(&self) -> f64 { self.min }
    pub fn max(&self) -> f64 { self.max }
}

impl Default for Interval {
    fn default() -> Self { Self::EMPTY }
}

