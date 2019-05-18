use space_age_derive::Planet;

const EARTH_PERIOD: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration {
    val: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { val: s as f64 }
    }
}

pub trait Planet: Period {
    fn years_during(d: &Duration) -> f64;
}

pub trait Period {
    fn period() -> f64;
}

#[derive(Planet)]
pub struct Mercury;
#[derive(Planet)]
pub struct Venus;
#[derive(Planet)]
pub struct Earth;
#[derive(Planet)]
pub struct Mars;
#[derive(Planet)]
pub struct Jupiter;
#[derive(Planet)]
pub struct Saturn;
#[derive(Planet)]
pub struct Uranus;
#[derive(Planet)]
pub struct Neptune;

impl Period for Mercury {
    fn period() -> f64 {
        EARTH_PERIOD * 0.2408467
    }
}
impl Period for Venus {
    fn period() -> f64 {
        EARTH_PERIOD * 0.61519726
    }
}
impl Period for Earth {
    fn period() -> f64 {
        EARTH_PERIOD
    }
}
impl Period for Mars {
    fn period() -> f64 {
        EARTH_PERIOD * 1.8808158
    }
}
impl Period for Jupiter {
    fn period() -> f64 {
        EARTH_PERIOD * 11.862615
    }
}
impl Period for Saturn {
    fn period() -> f64 {
        EARTH_PERIOD * 29.447498
    }
}
impl Period for Uranus {
    fn period() -> f64 {
        EARTH_PERIOD * 84.016846
    }
}
impl Period for Neptune {
    fn period() -> f64 {
        EARTH_PERIOD * 164.79132
    }
}
