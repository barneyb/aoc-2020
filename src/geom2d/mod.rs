/// Describes rectilinear directions on a coordinate plane.
#[derive(Copy, Clone, Debug)]
pub enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    pub fn clockwise(&self) -> Dir {
        use Dir::*;
        match self {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }
}
