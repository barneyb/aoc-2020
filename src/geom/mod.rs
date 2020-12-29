use std::fmt::Debug;

#[macro_export]
macro_rules! vector_type {
    ( $n:ident, $t:ty, $( $d:ident ),* ) => {
        #[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
        struct $n {
            $( $d: $t, )*
        }

        #[allow(unused)]
        impl $n {
            const fn new($( $d: $t, )*) -> $n {
                $n { $( $d, )* }
            }

            const fn origin() -> $n {
                $n { $( $d: 0, )* }
            }

            fn rectilinear_min(&self, other: &$n) -> $n {
                $n { $( $d: self.$d.min(other.$d), )* }
            }

            fn rectilinear_max(&self, other: &$n) -> $n {
                $n { $( $d: self.$d.max(other.$d), )* }
            }

            fn manhattan_distance(&self, p: &$n) -> usize {
                let mut d: usize = 0;
                $( d += (self.$d - p.$d).abs() as usize; )*
                d
            }
        }

        #[allow(unused)]
        impl std::ops::Add for $n {
            type Output = $n;

            fn add(self, rhs: Self) -> Self::Output {
                $n { $( $d: self.$d + rhs.$d, )* }
            }
        }

        impl std::fmt::Display for $n {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "(")?;
                // I cannot figure out how to lose the final trailing comma...
                $( write!(f, "{},", self.$d)?; )*
                write!(f, ")")
            }
        }
    };
}

vector_type!(Vec2, i32, x, y);
vector_type!(Vec3, i32, x, y, z);

impl From<&[i32]> for Vec3 {
    fn from(values: &[i32]) -> Self {
        assert_eq!(3, values.len());
        Vec3::new(values[0], values[1], values[2])
    }
}

impl From<&Vec<i32>> for Vec3 {
    fn from(values: &Vec<i32>) -> Self {
        Vec3::from(&values[..])
    }
}

// pub struct Neighbors<'a, T>
// {
//     p: &'a Vec3<T>,
//     i: usize,
// }
//
// impl<T> Iterator for Neighbors<'_, T>
// where
//     T: Add<Output = T>,
// {
//     type Item = Vec3<T>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.i >= NEIGHBOR_OFFSETS.len() {
//             return None;
//         }
//         let d = NEIGHBOR_OFFSETS[self.i];
//         self.i += 1;
//         Some(*self.p + d)
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basics() {
        let dims = vec![1, 2, 3];
        let v = Vec3::from(&dims);
        assert_eq!(2, v.y);
        assert_eq!(Vec3::new(1, 2, 3), v);
        assert_eq!("(1,2,3,)", format!("{}", v));
    }

    #[test]
    fn test_add() {
        let a = Vec2::new(1, 2);
        let b = Vec2::new(3, 4);
        assert_eq!(Vec2::new(4, 6), a + b);
    }

    #[test]
    fn test_rectilinear_min() {
        let a = Vec2::new(1, 2);
        let b = Vec2::new(3, 4);
        assert_eq!(Vec2::new(1, 2), a.rectilinear_min(&b));
        let b = Vec2::new(-3, 4);
        assert_eq!(Vec2::new(-3, 2), a.rectilinear_min(&b));
        let b = Vec2::new(3, -4);
        assert_eq!(Vec2::new(1, -4), a.rectilinear_min(&b));
    }

    #[test]
    fn test_rectilinear_max() {
        let a = Vec2::new(1, 2);
        let b = Vec2::new(3, 4);
        assert_eq!(Vec2::new(3, 4), a.rectilinear_max(&b));
        let b = Vec2::new(-3, 4);
        assert_eq!(Vec2::new(1, 4), a.rectilinear_max(&b));
        let b = Vec2::new(3, -4);
        assert_eq!(Vec2::new(3, 2), a.rectilinear_max(&b));
    }

    #[test]
    fn test_manhattan_distance() {
        let origin = Vec2::origin();
        assert_eq!(25, Vec2::new(17, 8).manhattan_distance(&origin));
        assert_eq!(25, Vec2::new(17, -8).manhattan_distance(&origin));
        assert_eq!(25, Vec2::new(-17, 8).manhattan_distance(&origin));
        assert_eq!(25, Vec2::new(-17, -8).manhattan_distance(&origin));
    }
}
