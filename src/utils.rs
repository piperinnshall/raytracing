use rand::Rng;

// Utility Functions

pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

// Random f64 in [0,1)
pub fn random_f64() -> f64 {
    let mut rng = rand::rng();
    rng.random()
}

// Random f64 in [min,max)
pub fn random_range_f64(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}
