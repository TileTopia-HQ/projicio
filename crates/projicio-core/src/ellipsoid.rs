/// Reference ellipsoid parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipsoid {
    /// Semi-major axis (equatorial radius) in meters.
    pub a: f64,
    /// Flattening (1/f).
    pub f: f64,
}

impl Ellipsoid {
    pub const fn new(a: f64, f: f64) -> Self {
        Self { a, f }
    }

    /// Semi-minor axis (polar radius).
    pub fn b(&self) -> f64 {
        self.a * (1.0 - self.f)
    }

    /// First eccentricity squared.
    pub fn e2(&self) -> f64 {
        2.0 * self.f - self.f * self.f
    }

    /// First eccentricity.
    pub fn e(&self) -> f64 {
        self.e2().sqrt()
    }

    /// WGS84 ellipsoid.
    pub const WGS84: Self = Self::new(6_378_137.0, 1.0 / 298.257_223_563);

    /// GRS80 ellipsoid (used by NAD83, ETRS89).
    pub const GRS80: Self = Self::new(6_378_137.0, 1.0 / 298.257_222_101);

    /// Clarke 1866 ellipsoid (used by NAD27).
    pub const CLARKE_1866: Self = Self::new(6_378_206.4, 1.0 / 294.978_698_2);

    /// International 1924 ellipsoid.
    pub const INTERNATIONAL_1924: Self = Self::new(6_378_388.0, 1.0 / 297.0);

    /// WGS84 sphere (for Web Mercator).
    pub const SPHERE: Self = Self::new(6_378_137.0, 0.0);
}
