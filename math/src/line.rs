use crate::point::{Point2D, HPoint2D};
use crate::vector_ops::{dot2d};

/// Line2D
/// A line in a 2D space. It has been implemented as the dot product between a position 'x' and a line represented by homogeous coordinates 'l'
/// There are three degrees of freedom
/// The plane equation is given as x * l = 0, or ax + by + c = 0
pub struct Line2D {
    l: HPoint2D
}

/// fn is_point_in_line
/// Operates a dot product between the point, homogenous or not, in order to find out if it's in the line, in the two-dimensional plane
pub fn is_point_in_line(line: Line2D, point: Point2D) -> bool {
    return dot2d(&Point2D::HomogenousPoint(line.l), &point) == 0.0
}

/// fn line_intersection
/// Operates a cross product between the two lines' coefficients, finding the 2D homogenous point where they meet or would meet if extended
pub fn line_intersection(line1: Line2D, line2: Line2D) {
    unimplemented!();
}

/// fn line_joining_points
/// Operates a cross product beetween two 2D homogenous points, finding the line that joins them
pub fn line_joining_points(point1: HPoint2D, point2: HPoint2D) {
    unimplemented!();
}
