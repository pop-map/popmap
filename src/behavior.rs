use super::{Area, GetPep, GetPop, Latitude, Location, Longitude, PostPep, PostPop};

impl From<Latitude> for (i16, u8, u8) {
    fn from(Latitude { deg, min, sec }: Latitude) -> Self {
        (deg, min, sec)
    }
}

impl From<Longitude> for (i16, u8, u8) {
    fn from(Longitude { deg, min, sec }: Longitude) -> Self {
        (deg, min, sec)
    }
}

impl TryFrom<(i16, u8, u8)> for Latitude {
    type Error = &'static str;

    fn try_from((deg, min, sec): (i16, u8, u8)) -> Result<Self, Self::Error> {
        match (deg, min, sec) {
            (-89..=89, 0..=59, 0..=59) => Ok(Self { deg, min, sec }),
            (-90 | 90, 0, 0) => Ok(Self { deg, min, sec }),
            _ => Err("latitude not in range -90_00_00..=90_00_00"),
        }
    }
}

impl TryFrom<(i16, u8, u8)> for Longitude {
    type Error = &'static str;

    fn try_from((deg, min, sec): (i16, u8, u8)) -> Result<Self, Self::Error> {
        match (deg, min, sec) {
            (-179..=179, 0..=59, 0..=59) => Ok(Self { deg, min, sec }),
            (180, 0, 0) => Ok(Self { deg, min, sec }),
            _ => Err("longitude not in range -179_59_59..=180_00_00"),
        }
    }
}

pub trait Angle: Into<(i16, u8, u8)> {
    fn radian(self) -> f64 {
        let (deg, min, sec) = self.into();
        let sign = deg.signum() as f64;
        let deg = deg.abs_diff(0);
        let (deg, min, sec) = (deg as f64, min as f64, sec as f64);
        let rot = (deg + (min + sec / 60.0) / 60.0) / 360.0;
        let rad = rot * std::f64::consts::TAU;
        sign * rad
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
            user: telegram_auth::fake_auth(user),
        }
    }
}

impl From<GetPep> for PostPep {
    fn from(GetPep { content, user, .. }: GetPep) -> Self {
        Self {
            content,
            user: telegram_auth::fake_auth(user),
        }
    }
}
