/// EPoint2D
/// Implements the functionality of an euclidean point in an image or two-dimensional space
pub struct EPoint2D {
    pub x: f64,
    pub y: f64
}

impl Default for EPoint2D {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0
        }
    }
}

/// fn homogenize2d
/// Turns an euclidean 2D point into a homogenous one
pub fn homogenize2d(point: &EPoint2D) -> HPoint2D {
    HPoint2D {
        x: point.x,
        y: point.y,
        w: 1.0
    }
}

/// HPoint2D
/// Implements a homogenized 2D point
pub struct HPoint2D {
    pub x: f64,
    pub y: f64,
    pub w: f64
}

impl Default for HPoint2D {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 1.0
        }
    }
}

/// fn dehomogenize2d
/// Divides the homogenous' coordinates of a point by the 'w' coordinate, de-homogenizing it
pub fn dehomogenize2d(point: &HPoint2D) -> EPoint2D {
    EPoint2D {
        x: point.x / point.w,
        y: point.y / point.w
    }
}

/// EPoint3D
/// Implements the functionality of an euclidean point in a three-dimensional space
pub struct EPoint3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Default for EPoint3D {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
}

/// fn homogenize3d
/// Turns an euclidean 3D point into a homogenous one
pub fn homogenize3d(point: &EPoint3D) -> HPoint3D {
    HPoint3D {
        x: point.x,
        y: point.y,
        z: point.z,
        w: 1.0
    }
}

/// HPoint3D
/// Implements an homogenized 3D point
pub struct HPoint3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Default for HPoint3D {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0
        }
    }
}

/// fn dehomogenize3d
/// Divides the homogenous' coordinates of a point by the 'w' coordinate, de-homogenizing it
pub fn dehomogenize3d(point: &HPoint3D) -> EPoint3D {
    EPoint3D {
        x: point.x / point.w,
        y: point.y / point.w,
        z: point.z / point.w
    }
}

/// enum Point2D
/// Wraps the value of a 2D point that can be either represented in euclidean or homogenous coordinates
pub enum Point2D {
    EuclideanPoint(EPoint2D),
    HomogenousPoint(HPoint2D)
}

/// enum Point3D
/// Wraps the value of a 3D point that can be either represented in euclidean or homogenous coordinates
pub enum Point3D {
    EuclideanPoint(EPoint3D),
    HomogenousPoint(HPoint3D)
}
