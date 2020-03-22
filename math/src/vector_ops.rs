use crate::point::{Point2D, Point3D, HPoint2D, EPoint3D};
use crate::point::{homogenize2d, homogenize3d};

/// fn dot2d
/// Calculates the dot product between two 2D points, homogenous or not.
pub fn dot2d(pa: &Point2D, pb: &Point2D) -> f64 {
    match (pa, pb) {
        (Point2D::EuclideanPoint(a), Point2D::EuclideanPoint(b)) => {
            a.x * b.x + a.y * b.y
        },
        (Point2D::EuclideanPoint(a), Point2D::HomogenousPoint(b)) => {
            let homo_a = homogenize2d(a);
            homo_a.x * b.x + homo_a.y * b.y + homo_a.w * b.w
        }
        (Point2D::HomogenousPoint(a), Point2D::EuclideanPoint(b)) => {
            let homo_b = homogenize2d(b);
            a.x * homo_b.x + a.y * homo_b.y + a.w * homo_b.w
        },
        (Point2D::HomogenousPoint(a), Point2D::HomogenousPoint(b)) => {
            a.x * b.x + a.y * b.y + a.w * b.w
        }
    }
}

/// fn dot3d
/// Calculates the dot product between two 3D points, homogenous or not.
pub fn dot3d(pa: &Point3D, pb: &Point3D) -> f64 {
    match (pa, pb) {
        (Point3D::EuclideanPoint(a), Point3D::EuclideanPoint(b)) => {
            a.x * b.x + a.y * b.y + a.z * b.z
        },
        (Point3D::EuclideanPoint(a), Point3D::HomogenousPoint(b)) => {
            let homo_a = homogenize3d(a);
            homo_a.x * b.x + homo_a.y * b.y + homo_a.z * b.z + homo_a.w * b.w
        }
        (Point3D::HomogenousPoint(a), Point3D::EuclideanPoint(b)) => {
            let homo_b = homogenize3d(b);
            a.x * homo_b.x + a.y * homo_b.y + a.z * homo_b.z + a.w * homo_b.w
        },
        (Point3D::HomogenousPoint(a), Point3D::HomogenousPoint(b)) => {
            a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
        }
    }
}

/// Vector3
/// Contains any value that can be considered a three-dimensional vector and defines operations for it.
/// It can be either a homogeous 2D point (they can be cross-multiplied in order to find line intersections in 2D for example), or an euclidean 3D point.
pub enum Vector3 {
    Homogenous2D(HPoint2D),
    Euclidean3D(EPoint3D)
}

pub enum Vector4 {
    HPoint3D
}
