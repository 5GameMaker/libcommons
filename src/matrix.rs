use std::{
    fmt::Debug,
    hint::black_box,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

/// Matrix creation shortcut.
#[macro_export]
macro_rules! mat {
    (<$t:ty, $r:tt, $c:tt> $($x:expr),+ $(,)?) => {
        $crate::matrix::Matrix::<$t, $r, $c>::new([$($x),*])
    };
    (<$matty:ty> $($x:expr),+ $(,)?) => {
        <$matty>::new([$($x),*])
    };
    ($($x:expr),+ $(,)?) => {
        $crate::matrix::Matrix::<_, _, _>::new([$($x),*])
    };
}

/// Panic in a const fn.
#[allow(unused)]
const fn const_panic() -> ! {
    #[allow(unconditional_panic)]
    loop {
        _ = black_box(1 / 0);
    }
}

// I'll come back to this when rustc will stop crashing.
//
// Define your identity matrixes yourself in the meantime!.

// const fn mat_identity<T: Num, const SIZE: usize>() -> Matrix<T, SIZE, SIZE>
// where
//     [T; SIZE * SIZE]:,
// {
//     let mut arr = [T::ZERO; SIZE * SIZE];
//     let mut p = 0usize;
//     while p < SIZE {
//         let mut v = T::ONE;
//         swap(&mut arr[p * SIZE + p], &mut v);
//         forget(v);
//         p += 1;
//     }
//     Matrix::new(arr)
// }

/// A number type.
pub trait Num:
    Clone
    + PartialEq
    + Sized
    + Mul<Self, Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Div<Self, Output = Self>
    + Rem<Self, Output = Self>
    + MulAssign
    + AddAssign
    + SubAssign
    + DivAssign
    + RemAssign
{
    const ZERO: Self;
    const ONE: Self;
}
macro_rules! impl_num_i {
    ($($ty:ty)*) => {$(
        impl Num for $ty {
            const ZERO: Self = 0;
            const ONE: Self = 1;
        }
    )*};
}
macro_rules! impl_num_f {
    ($($ty:ty)*) => {$(
        impl Num for $ty {
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;
        }
    )*};
}
impl_num_i! {
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
}
impl_num_f! {
    f32 f64
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Matrix<T, const ROWS: usize, const COLUMNS: usize>([T; ROWS * COLUMNS])
where
    [T; ROWS * COLUMNS]:,
    T: Num;
impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS>
where
    [T; ROWS * COLUMNS]:,
    T: Num,
{
    const ZERO: Self = Self::new([T::ZERO; ROWS * COLUMNS]);

    /// Create a new matrix.
    ///
    /// Matrix layout consists of arrays of rows.
    pub const fn new(value: [T; ROWS * COLUMNS]) -> Self {
        Self(value)
    }

    /// Obtain transposed matrix.
    ///
    /// Might be converted to a const fn in the future.
    pub fn transposed(&self) -> Matrix<T, COLUMNS, ROWS>
    where
        [T; COLUMNS * ROWS]:,
    {
        let mut mat = Matrix::<T, COLUMNS, ROWS>::ZERO;
        for x in 0..COLUMNS {
            for y in 0..ROWS {
                *mat.get_mut(x, y).unwrap() = self.get(y, x).unwrap().clone();
            }
        }
        mat
    }

    pub const fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= ROWS || col >= COLUMNS {
            None
        } else {
            Some(&self.0[row + col * ROWS])
        }
    }

    pub const fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row >= ROWS || col >= COLUMNS {
            None
        } else {
            Some(&mut self.0[row + col * ROWS])
        }
    }
}
impl<T, I, const ROWS: usize, const COLUMNS: usize> From<[I; ROWS * COLUMNS]>
    for Matrix<T, ROWS, COLUMNS>
where
    [I; ROWS * COLUMNS]:,
    I: Into<T>,
    T: Num,
{
    fn from(value: [I; ROWS * COLUMNS]) -> Self {
        Self(value.map(|x| x.into()))
    }
}
impl<T, const SIDE: usize, const EXTRA1: usize, const EXTRA2: usize> Mul<Matrix<T, EXTRA2, SIDE>>
    for Matrix<T, SIDE, EXTRA1>
where
    [T; SIDE * EXTRA1]:,
    [T; EXTRA2 * SIDE]:,
    [T; EXTRA2 * EXTRA1]:,
    T: Num + Debug,
{
    type Output = Matrix<T, EXTRA2, EXTRA1>;

    fn mul(self, rhs: Matrix<T, EXTRA2, SIDE>) -> Self::Output {
        let mut new = Matrix::<T, EXTRA2, EXTRA1>::ZERO;
        for x in 0..EXTRA2 {
            for y in 0..EXTRA1 {
                let mut sum = T::ZERO;
                for i in 0..SIDE {
                    sum += self.get(i, y).unwrap().clone() * rhs.get(x, i).unwrap().clone();
                }
                *new.get_mut(x, y).unwrap() = sum;
            }
        }
        new
    }
}
impl<T, const ROWS: usize, const COLUMNS: usize> Add<Matrix<T, ROWS, COLUMNS>>
    for Matrix<T, ROWS, COLUMNS>
where
    [T; ROWS * COLUMNS]:,
    T: Num,
{
    type Output = Matrix<T, ROWS, COLUMNS>;

    fn add(mut self, rhs: Matrix<T, ROWS, COLUMNS>) -> Self::Output {
        self.0.iter_mut().zip(rhs.0).for_each(|(x, v)| *x += v);
        self
    }
}
// impl<T, const SIZE: usize> Matrix<T, SIZE, SIZE>
// where
//     [T; SIZE * SIZE]:,
//     T: Num,
// {
//     pub const IDENTITY: Self = mat_identity::<T, SIZE>();
// }
// impl<T, const SIZE: usize> Default for Matrix<T, SIZE, SIZE>
// where
//     [T; SIZE * SIZE]:,
// {
//     fn default() -> Self {
//         Self::IDENTITY
//     }
// }

pub type Mat4f = Matrix<f32, 4, 4>;
/// Identity matrix for f32 4x4 matrix.
///
/// May be swapped for `Mat4f::IDENTITY` in the future.
pub const MAT4F_IDENTITY: Mat4f = mat! {
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0,
};
pub type Mat3f = Matrix<f32, 3, 3>;
/// Identity matrix for f32 3x3 matrix.
///
/// May be swapped for `Mat3f::IDENTITY` in the future.
pub const MAT3F_IDENTITY: Mat3f = mat! {
    1.0, 0.0, 0.0,
    0.0, 1.0, 0.0,
    0.0, 0.0, 1.0,
};

// https://www.mathsisfun.com/algebra/matrix-multiplying.html

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn get_test() {
        assert_eq!(
            mat! { <f32, 3, 2>
                1.0, 2.0, 3.0,
                4.0, 5.0, 6.0,
            }
            .get(1, 1),
            Some(&5.0)
        );
    }

    #[test]
    fn transpose_test() {
        assert_eq!(
            mat! { <f32, 3, 2>
                1.0, 2.0, 3.0,
                4.0, 5.0, 6.0,
            }
            .transposed(),
            mat! { <f32, 2, 3>
                1.0, 4.0,
                2.0, 5.0,
                3.0, 6.0,
            }
        );
    }

    #[test]
    fn multiply_test() {
        let first: Matrix<f32, 3, 2> = mat! {
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        };
        let second: Matrix<f32, 2, 3> = mat! {
            7.0,  8.0,
            9.0,  10.0,
            11.0, 12.0,
        };

        assert_eq!(
            first * second,
            mat! {
                58.0,  64.0,
                139.0, 154.0
            }
        );
    }

    #[test]
    fn add_test() {
        let first: Matrix<f32, 2, 2> = mat! {
            1.0, 2.0,
            4.0, 5.0,
        };
        let second: Matrix<f32, 2, 2> = mat! {
            7.0, 8.0,
            9.0, 10.0,
        };

        assert_eq!(
            first + second,
            mat! {
                8.0,  10.0,
                13.0, 15.0
            }
        );
    }
}
