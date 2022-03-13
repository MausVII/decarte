use std::cmp::{PartialEq};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Matrix3D {
    M: [[i32; 3]; 3],
}

impl Matrix3D {
    pub fn from(a: [[i32; 3]; 3]) -> Self {
        Matrix3D { M:
            [[a[0][0], a[0][1], a[0][2]],
             [a[1][0], a[1][1], a[1][2]],
             [a[2][0], a[2][1], a[2][2]]]
        }
    }

    pub fn from_zeroes() -> Self {
        Matrix3D {
            M: [[0, 0, 0],
                [0, 0, 0],
                [0, 0, 0]]
        }
    }

    pub fn identity_matrix() -> Self {
        Matrix3D::from([[1, 0, 0], [0, 1, 0], [0, 0, 1]])
    }
}

impl PartialEq for Matrix3D {
    fn eq(&self, other: &Matrix3D) -> bool {
        self.M == other.M
    }
}

impl Display for Matrix3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{:?}\n {:?}\n {:?}]", self.M[0], self.M[1], self.M[2])
    }
}

#[cfg(test)]
mod tests {
    use crate::Matrix3D;

    #[test]
    fn get_empty_matrix() {
        assert_eq!(Matrix3D::from_zeroes(), Matrix3D { M: 
            [[0, 0, 0],
             [0, 0, 0],
             [0, 0, 0]]});
    }
    #[test]
    fn get_identity_matrix() {
        assert_eq!(Matrix3D::identity_matrix(), Matrix3D::from([[1, 0, 0], [0, 1, 0], [0, 0, 1]]));
    }
    
}
