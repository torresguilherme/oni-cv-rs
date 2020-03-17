/// Point2D
/// Implements the functionality of a point in an image or two-dimensional space
pub struct Point2D {
    pub x: f64,
    pub y: f64
}

impl Default for Point2D {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0
        }
    }
}

/* Indexes for points won't be implemented for now
 * because I can't think of a way to do this effectively without the possibility of panicking...
impl Index<usize> for Point2D {
    type Output = f64;

    fn index(&self, num: usize) -> &Self::Output {
        match num {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds for Point2D")
        }
    }
}
 */

/// Point3D
/// Implements the functionality of a point in a three-dimensional space
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Default for Point3D {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
}
