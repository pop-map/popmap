use super::{Area, GetPep, GetPop, Latitude, Location, Longitude, PostPep, PostPop};

impl From<Latitude> for i32 {
    fn from(Latitude(sec_arc): Latitude) -> Self {
        sec_arc
    }
}

impl From<Longitude> for i32 {
    fn from(Longitude(sec_arc): Longitude) -> Self {
        sec_arc
    }
}

impl TryFrom<i32> for Latitude {
    type Error = &'static str;

    fn try_from(sec_arc: i32) -> Result<Self, Self::Error> {
        if (-90 * 60 * 60..90 * 60 * 60).contains(&sec_arc) {
            Ok(Self(sec_arc))
        } else {
            Err("latitude not in range -90_00_00..=90_00_00 (base 60)")
        }
    }
}

impl TryFrom<i32> for Longitude {
    type Error = &'static str;

    fn try_from(sec_arc: i32) -> Result<Self, Self::Error> {
        if (-180 * 60 * 60 + 1..180 * 60 * 60).contains(&sec_arc) {
            Ok(Self(sec_arc))
        } else {
            Err("longitude not in range -179_59_59..=180_00_00 (base 60)")
        }
    }
}

pub trait Angle: Into<i32> {
    fn radian(self) -> f64 {
        let sec_arc: i32 = self.into();
        let rot = ((sec_arc as f64 / 60.0) / 60.0) / 360.0;
        rot * std::f64::consts::TAU
    }
}

impl Angle for Longitude {}
impl Angle for Latitude {}

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
            user,
            ..
        }: GetPop,
    ) -> Self {
        Self {
            title,
            description,
            location,
            expire,
            user: user.fake_auth(),
        }
    }
}

impl From<GetPep> for PostPep {
    fn from(GetPep { content, user, .. }: GetPep) -> Self {
        Self {
            content,
            user: user.fake_auth(),
        }
    }
}
