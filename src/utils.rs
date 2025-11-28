use rand::Rng;

// Utility Functions

pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

// Random double in [0,1)
pub fn random_double() -> f64 {
    let mut rng = rand::rng();
    rng.random()
}

// Random double in [min,max)
pub fn random_double_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}
