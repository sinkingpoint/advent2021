pub struct Vec2D {
    pub x: i64,
    pub y: i64
}

impl Vec2D {
    pub fn new(x: i64, y: i64) -> Self {
        return Self {
            x,
            y
        }
    }
}

pub struct Vec3D {
    pub x: i64,
    pub y: i64,
    pub z: i64
}

impl Vec3D {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        return Self {
            x,
            y,
            z
        }
    }
}

pub struct Vec4D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub w: i64
}

impl Vec4D {
    pub fn new(x: i64, y: i64, z: i64, w: i64) -> Self {
        return Self {
            x,
            y,
            z,
            w
        }
    }
}