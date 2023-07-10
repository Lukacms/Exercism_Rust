// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);

static DIVISORS: &[f64] = &[
    0.2408467, 0.61519726, 1.0, 1.8808158, 11.862615, 29.447498, 84.016846, 164.79132,
];

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self((s as f64) / 31557600.0)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! impl_Planet {
    ($i:ident, $p:expr) => {
        pub struct $i;
        impl Planet for $i {
            fn years_during(d: &Duration) -> f64 {
                d.0 / $p
            }
        }
    };
}

impl_Planet!(Mercury, DIVISORS[0]);
impl_Planet!(Venus, DIVISORS[1]);
impl_Planet!(Earth, DIVISORS[2]);
impl_Planet!(Mars, DIVISORS[3]);
impl_Planet!(Jupiter, DIVISORS[4]);
impl_Planet!(Saturn, DIVISORS[5]);
impl_Planet!(Uranus, DIVISORS[6]);
impl_Planet!(Neptune, DIVISORS[7]);
