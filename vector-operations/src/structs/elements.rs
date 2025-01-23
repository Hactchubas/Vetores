use super::vector::Vector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineSegment {
    x: (f64, f64),
    y: (f64, f64),
}

impl LineSegment {
    pub fn new(x: (f64, f64), y: (f64, f64)) -> Self {
        LineSegment { x, y }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let (cx, cy) = other.x;
        let (dx, dy) = other.y;
        let (bx, by) = self.y;
        let (ax, ay) = self.x;

        let A = Vector::new(vec![ax, ay]);
        let AC = Vector::new(vec![cx, cy]) - A.to_owned();
        let AD = Vector::new(vec![dx, dy]) - A.to_owned();
        let AB = Vector::new(vec![bx, by]) - A.to_owned();

        let C = Vector::new(vec![ax, ay]);
        let CD = Vector::new(vec![dx, dy]) - C.to_owned();
        let CA = Vector::new(vec![ax, ay]) - C.to_owned();
        let CB = Vector::new(vec![bx, by]) - C.to_owned();

        match (
            (AB.cross_product(&AC), AB.cross_product(&AD)),
            (CD.cross_product(&CA), CD.cross_product(&CB)),
        ) {
            ((Some(ABxAC), Some(ABxAD)), (Some(CDxCA), Some(CDxCB)))
                if (ABxAC.signal_in_r(3) ^ ABxAD.signal_in_r(3))
                    && (CDxCA.signal_in_r(3) ^ CDxCB.signal_in_r(3)) =>
            {
                true
            }
            _ => false,
        }
    }
}
