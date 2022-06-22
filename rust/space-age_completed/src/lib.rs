// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration
{
    seconds: u64
}

impl From<u64> for Duration
{
    fn from(s: u64) -> Self
    {
        Duration
        {
            seconds: s
        }
    }
}

pub trait Planet
{
    fn years_during(d: &Duration) -> f64;
}

macro_rules! planet
{
    ($planet_name: ident, $orbital_period: expr) =>
    {
        pub struct $planet_name;

        impl Planet for $planet_name
        {
            fn years_during(d: &Duration) -> f64
            {
                d.seconds as f64 / ($orbital_period * 31557600.)
            }
        }
    }
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.);
planet!(Mars,  1.8808158);
planet!(Jupiter, 11.862615);
planet!(Uranus, 84.016846);
planet!(Saturn, 29.447498);
planet!(Neptune, 164.79132);
