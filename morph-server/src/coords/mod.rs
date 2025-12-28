#[derive(Debug)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Coords {
    pub fn create(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn owned_coords(coords: &Self) -> Self {
        Self {
            x: coords.x,
            y: coords.y,
            z: coords.z
        }
    }

    pub fn get_distance_between(&self, coords: Self) -> f64 {
        (
            (coords.x-self.x).powf(2.0) + 
            (coords.y-self.y).powf(2.0) + 
            (coords.z-self.z).powf(2.0)
        ).sqrt()
    }
}