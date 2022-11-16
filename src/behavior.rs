use super::{Area, Degree, GetPep, GetPop, Location, PostPep, PostPop};

impl Degree {
    /// Create a new `Degree`
    ///
    /// The only valid initerval is `-179'59'59 .. 180'00'00`
    pub fn new(degree: i16, minute: u8, second: u8) -> Self {
        let value = Self(degree, minute, second);
        assert!(value.is_valid());
        value
    }
    fn is_valid(self) -> bool {
        matches!(self, Degree(-179..=179, 0..=59, 0..=59) | Degree(180, 0, 0))
    }
    pub fn radian(self) -> f64 {
        assert!(self.is_valid());
        let Self(degree, minute, second) = self;
        let degree = if degree < 0 { degree + 360 } else { degree };
        let float = degree as f64 + (minute as f64) / 60.0 + (second as f64) / 3600.0;
        float / 360.0 * std::f64::consts::TAU
    }
}

impl Location {
    /// Compute distance in meters between two location
    ///
    /// The length of the shortest geodesic between point A and B
    pub fn distance(self, other: Self) -> u32 {
        use scalararray::{VectorOps, VectorRotateOps};
        const EARTH_RADIUS: f64 = 6_371_000.0;
        // start with the X vector as Greenwich on equator
        // then apply latitude and longitude rotations
        let vec1 = [1.0, 0.0, 0.0]
            .vector_rotate_y(self.lat.radian())
            .vector_rotate_z(self.lng.radian());
        let vec2 = [1.0, 0.0, 0.0]
            .vector_rotate_y(other.lat.radian())
            .vector_rotate_z(other.lng.radian());
        // compute angle between the two vectors
        let angle = vec1.vector_dot(vec2).acos();
        (angle * EARTH_RADIUS) as u32
    }
}

impl Area {
    /// Test whether a location is contained in the area
    pub fn contains(self, location: Location) -> bool {
        location.distance(self.into()) <= self.radius
    }
}

impl From<Area> for Location {
    fn from(Area { lat, lng, .. }: Area) -> Self {
        Self { lat, lng }
    }
}

impl From<GetPop> for PostPop {
    fn from(
        GetPop {
            title,
            description,
            location,
            expire,
            ..
        }: GetPop,
    ) -> Self {
        Self {
            title,
            description,
            location,
            expire,
        }
    }
}

impl From<GetPep> for PostPep {
    fn from(GetPep { content, .. }: GetPep) -> Self {
        Self { content }
    }
}
