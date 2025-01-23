use std::ops::{Add, Mul, Sub};

use serde::{Deserialize, Serialize};

use super::elements::LineSegment;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector {
    dimensions: Vec<f64>,
}

impl Vector {
    fn new(dims: Vec<f64>) -> Self {
        Vector { dimensions: dims }
    }

    pub fn subtract(&self, other: &Self) -> Self {
        let (new_self, new_other) = self.equalize_dimensions(&other);

        Vector::new(
            new_self
                .dimensions
                .iter()
                .zip(new_other.dimensions.iter())
                .map(|(s_di, o_di)| s_di - o_di)
                .collect(),
        )
    }

    pub fn cardinality(&self) -> usize {
        self.dimensions.len()
    }

    pub fn signal_in_r(&self, r: usize) -> bool {
        if r > self.cardinality() {
            false
        } else {
            matches!(self.dimensions.get(r-1), Some(&value) if value > 0.0)
        }
    }

    fn modulus(&self) -> f64 {
        self.dimensions
            .iter()
            .map(|&di| di.powi(2))
            .sum::<f64>()
            .sqrt()
    }

    // fn invert(&self) -> Self {
    //     Vector::new(self.dimensions.iter().map(|&di| -di).collect())
    // }

    pub fn scale(&self, m: f64) -> Self {
        let result = self.dimensions.iter().map(|&di| di * m).collect();

        Vector::new(result)
    }

    fn unit(&self) -> Self {
        self.scale(1 as f64 / self.modulus()).clone()
    }

    fn equalize_dimensions(&self, other: &Self) -> (Self, Self) {
        let this_length = self.dimensions.len();
        let other_length = other.dimensions.len();

        let mut smaller: Vector;
        if this_length < other_length {
            smaller = self.clone();
            smaller
                .dimensions
                .extend(vec![0 as f64; other_length - this_length]);

            (smaller.clone(), other.clone())
        } else if this_length > other_length {
            smaller = other.clone();
            smaller
                .dimensions
                .extend(vec![0 as f64; this_length - other_length]);

            (self.clone(), smaller)
        } else {
            (self.clone(), other.clone())
        }
    }

    // fn not_zero(&self) -> bool {
    //     !self.dimensions.iter().all(|&di| di == 0 as f64)
    // }

    pub fn cross_product(&self, other: &Self) -> Option<Self> {
        match (self.dimensions.len(), other.dimensions.len()) {
            (2, 2) => {
                let result: Vec<f64> = vec![
                    0.0,
                    0.0,
                    self.dimensions[0] * other.dimensions[1]
                        - self.dimensions[1] * other.dimensions[0],
                ];
                Some(Vector::new(result))
            }
            (3, 3) => {
                let result = vec![
                    self.dimensions[1] * other.dimensions[2]
                        - self.dimensions[2] * other.dimensions[1],
                    self.dimensions[2] * other.dimensions[0]
                        - self.dimensions[0] * other.dimensions[2],
                    self.dimensions[0] * other.dimensions[1]
                        - self.dimensions[1] * other.dimensions[0],
                ];
                Some(Vector::new(result))
            }
            _ => None,
        }
    }

    pub fn dot_product(&self, other: &Vector) -> Option<f64> {
        let (new_self, new_other) = self.equalize_dimensions(other);

        let result = new_self
            .dimensions
            .iter()
            .zip(new_other.dimensions.iter())
            .map(|(s_di, o_di)| s_di * o_di)
            .sum();

        Some(result)
    }

    pub fn projected_at(&self, other: &Self) -> Option<Self> {
        let unitary_other = other.unit();
        if let Some(p) = self.dot_product(&unitary_other) {
            let res = unitary_other.scale(p);
            Some(res)
        } else {
            None
        }
    }

    // pub fn reflected_at(&self, other: &Self) -> Option<Self> {
    //     if let Some(projected) = self.projected_at(other) {
    //         Some(self.clone() - projected.scale(2.0))
    //     } else {
    //         None
    //     }
    // }

    pub fn decompose(&self, other: &Self) -> Option<(Self, Self)> {
        if let Some(projected) = self.projected_at(other) {
            let orthogonal = self.clone() - projected.clone();
            Some((orthogonal, projected))
        } else {
            None
        }
    }

    pub fn parameterized_reaction(&self, alpha: f64, other: &Self, beta: f64) -> Option<Self> {
        if let Some((vn, vp)) = self.decompose(other) {
            let n = vn.scale(alpha);
            let p = vp.scale(-beta);
            let reac = n + p;
            Some(reac)
        } else {
            None
        }
    }

    pub fn to_line_segment(&self, other: &Self) -> LineSegment {
        LineSegment::new(self.clone(), other.clone())
    }

    pub fn normal_vec(&self) -> Option<Self> {
        let mut dims = vec![0.0; self.cardinality() - 1];
        dims.insert(0, 1.0);
        let other = &Vector::new(dims);
        if let Some((vn, vp)) = self.decompose(other) {
            let n = vp - vn;
            Some(n.unit())
        } else {
            None
        }
    }
}

impl Add<Self> for Vector {
    type Output = Vector;

    fn add(self, other: Self) -> Self::Output {
        let (new_self, new_other) = self.equalize_dimensions(&other);

        Vector::new(
            new_self
                .dimensions
                .iter()
                .zip(new_other.dimensions.iter())
                .map(|(s_di, o_di)| s_di + o_di)
                .collect(),
        )
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Self) -> Self::Output {
        let (new_self, new_other) = self.equalize_dimensions(&other);

        Vector::new(
            new_self
                .dimensions
                .iter()
                .zip(new_other.dimensions.iter())
                .map(|(s_di, o_di)| s_di - o_di)
                .collect(),
        )
    }
}

impl Mul for Vector {
    type Output = f64;

    fn mul(self, other: Self) -> Self::Output {
        let (new_self, new_other) = self.equalize_dimensions(&other);

        new_self
            .dimensions
            .iter()
            .zip(new_other.dimensions.iter())
            .map(|(s_di, o_di)| s_di - o_di)
            .sum::<f64>()
    }
}
