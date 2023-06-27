// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEAR_IN_SECONDS: u64 = 31557600;

#[derive(Debug)]
pub struct Duration {
    earth_year: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        let earth_year = s as f64 / EARTH_YEAR_IN_SECONDS as f64;
        Self { earth_year }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.earth_year
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.earth_year / 0.2408467
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.earth_year / 0.61519726
    }
}
impl Planet for Earth {}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.earth_year / 1.8808158
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.earth_year / 11.862615
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.earth_year / 29.447498
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.earth_year / 84.016846
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.earth_year / 164.79132
    }
}
