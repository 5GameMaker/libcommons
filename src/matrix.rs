use std::{
    fmt::Debug,
    hint::black_box,
    mem::MaybeUninit,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub,
        SubAssign,
    },
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

        impl Matrix<$ty, 2, 2> {
            /// Identity matrix.
            ///
            /// Multiplying a matrix by identity results in that matrix.
            ///
            /// ## Future compatibility.
            /// As of right now, identity matrixes are hardcoded for
            /// each [Num] type.
            ///
            /// When generating a matrix at compile time will stop
            /// crashing the compiler, that will be used instead.
            pub const IDENTITY: Self = mat! {
                1, 0,
                0, 1,
            };
        }
        impl Matrix<$ty, 3, 3> {
            /// Identity matrix.
            ///
            /// Multiplying a matrix by identity results in that matrix.
            ///
            /// ## Future compatibility.
            /// As of right now, identity matrixes are hardcoded for
            /// each [Num] type.
            ///
            /// When generating a matrix at compile time will stop
            /// crashing the compiler, that will be used instead.
            pub const IDENTITY: Self = mat! {
                1, 0, 0,
                0, 1, 0,
                0, 0, 1,
            };
        }
        impl Matrix<$ty, 4, 4> {
            /// Identity matrix.
            ///
            /// Multiplying a matrix by identity results in that matrix.
            ///
            /// ## Future compatibility.
            /// As of right now, identity matrixes are hardcoded for
            /// each [Num] type.
            ///
            /// When generating a matrix at compile time will stop
            /// crashing the compiler, that will be used instead.
            pub const IDENTITY: Self = mat! {
                1, 0, 0, 0,
                0, 1, 0, 0,
                0, 0, 1, 0,
                0, 0, 0, 1,
            };
        }
    )*};
}
macro_rules! impl_num_f {
    ($($ty:ty)*) => {$(
        impl Num for $ty {
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;
        }

        impl Matrix<$ty, 2, 2> {
            /// Identity matrix.
            ///
            /// Multiplying a matrix by identity results in that matrix.
            ///
            /// ## Future compatibility.
            /// As of right now, identity matrixes are hardcoded for
            /// each [Num] type.
            ///
            /// When generating a matrix at compile time will stop
            /// crashing the compiler, that will be used instead.
            pub const IDENTITY: Self = mat! {
                1.0, 0.0,
                0.0, 1.0,
            };
        }
        impl Matrix<$ty, 3, 3> {
            /// Identity matrix.
            ///
            /// Multiplying a matrix by identity results in that matrix.
            ///
            /// ## Future compatibility.
            /// As of right now, identity matrixes are hardcoded for
            /// each [Num] type.
            ///
            /// When generating a matrix at compile time will stop
            /// crashing the compiler, that will be used instead.
            pub const IDENTITY: Self = mat! {
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0,
            };
        }
        impl Matrix<$ty, 4, 4> {
            /// Identity matrix.
            ///
            /// Multiplying a matrix by identity results in that matrix.
            ///
            /// ## Future compatibility.
            /// As of right now, identity matrixes are hardcoded for
            /// each [Num] type.
            ///
            /// When generating a matrix at compile time will stop
            /// crashing the compiler, that will be used instead.
            pub const IDENTITY: Self = mat! {
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            };
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

/// A matrix.
///
/// Rows and columns represent the y and x coordinates respectively.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct Matrix<T, const ROWS: usize, const COLUMNS: usize>([T; ROWS * COLUMNS])
where
    [T; ROWS * COLUMNS]:;
impl<T, const SIZE: usize> Matrix<T, SIZE, SIZE>
where
    [T; SIZE * SIZE]:,
    T: Num,
{
    /// Obtain the identity matrix of this size.
    ///
    /// ## Future compatibility
    /// This function may get converted to a const fn when
    /// Rust will stop crashing when compiling this.
    pub fn identity() -> Self {
        let mut matrix = Self::ZERO;
        let mut i = 0usize;
        while i < SIZE {
            let mut t = T::ONE;
            std::mem::swap(&mut t, &mut matrix.0[i + i * SIZE]);
            std::mem::forget(t);
            i += 1;
        }
        matrix
    }
}
impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<MaybeUninit<T>, ROWS, COLUMNS>
where
    [MaybeUninit<T>; ROWS * COLUMNS]:,
{
    /// Assume that this matrix is initialized.
    ///
    /// ## Safety
    /// Caller must make sure that all cells of the matrix
    /// are actually initialized.
    ///
    /// Calling this on a matrix with uninitialized cells
    /// is undefined behavior.
    pub unsafe fn assume_init(self) -> Matrix<T, ROWS, COLUMNS>
    where
        [T; ROWS * COLUMNS]:,
    {
        unsafe { Matrix::new(MaybeUninit::array_assume_init(self.0)) }
    }
}
impl<'a, T, const ROWS: usize, const COLUMNS: usize> Matrix<&'a T, ROWS, COLUMNS>
where
    [T; ROWS * COLUMNS]:,
    [&'a T; ROWS * COLUMNS]:,
    T: Copy,
{
    pub fn copied(&self) -> Matrix<T, ROWS, COLUMNS> {
        unsafe {
            let mut mat: [MaybeUninit<T>; ROWS * COLUMNS] =
                [const { MaybeUninit::uninit() }; ROWS * COLUMNS];
            for (i, x) in self.0.iter().copied().copied().enumerate() {
                mat[i].write(x);
            }
            Matrix::new(MaybeUninit::array_assume_init(mat))
        }
    }
}
impl<'a, T, const ROWS: usize, const COLUMNS: usize> Matrix<&'a T, ROWS, COLUMNS>
where
    [T; ROWS * COLUMNS]:,
    [&'a T; ROWS * COLUMNS]:,
    T: Clone,
{
    pub fn cloned(&self) -> Matrix<T, ROWS, COLUMNS> {
        unsafe {
            let mut mat: [MaybeUninit<T>; ROWS * COLUMNS] =
                [const { MaybeUninit::uninit() }; ROWS * COLUMNS];
            for (i, x) in self.0.iter().copied().cloned().enumerate() {
                mat[i].write(x);
            }
            Matrix::new(MaybeUninit::array_assume_init(mat))
        }
    }
}
impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS>
where
    [T; ROWS * COLUMNS]:,
{
    /// Create a new matrix.
    ///
    /// Matrix layout is a flattened array of rows.
    pub const fn new(value: [T; ROWS * COLUMNS]) -> Self {
        Self(value)
    }

    pub fn new_uninit() -> Matrix<MaybeUninit<T>, ROWS, COLUMNS>
    where
        [MaybeUninit<T>; ROWS * COLUMNS]:,
    {
        Matrix::new([const { MaybeUninit::<T>::uninit() }; ROWS * COLUMNS])
    }

    /// Obtain transposed matrix.
    ///
    /// Might be converted to a const fn in the future.
    pub const fn transposed<'a>(&'a self) -> Matrix<&'a T, COLUMNS, ROWS>
    where
        [&'a T; COLUMNS * ROWS]:,
        [MaybeUninit<&'a T>; COLUMNS * ROWS]:,
    {
        unsafe {
            let mut mat: [MaybeUninit<&'a T>; COLUMNS * ROWS] =
                [MaybeUninit::uninit(); COLUMNS * ROWS];
            let mut x = 0usize;
            while x < COLUMNS {
                let mut y = 0usize;
                while y < ROWS {
                    mat[x + y * COLUMNS].write(self.get(y, x).unwrap_unchecked());
                    y += 1;
                }
                x += 1;
            }
            Matrix::new(MaybeUninit::array_assume_init(mat))
        }
    }

    /// Get a reference to a cell.
    pub const fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= ROWS || col >= COLUMNS {
            None
        } else {
            Some(&self.0[row + col * ROWS])
        }
    }

    /// Get a mutable reference to a cell.
    pub const fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row >= ROWS || col >= COLUMNS {
            None
        } else {
            Some(&mut self.0[row + col * ROWS])
        }
    }

    /// Get a reference to a cell without bounds checks.
    ///
    /// ## Safety
    /// Caller must ensure that provided cell does not exceed
    /// the bounds of the matrix.
    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
        unsafe { self.0.get_unchecked(row + col * ROWS) }
    }

    /// Get a mutable reference to a cell without bounds checks.
    ///
    /// ## Safety
    /// Caller must ensure that provided cell does not exceed
    /// the bounds of the matrix.
    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
        unsafe { self.0.get_unchecked_mut(row + col * ROWS) }
    }

    pub fn map<Y, F>(self, map: F) -> Matrix<Y, ROWS, COLUMNS>
    where
        [Y; ROWS * COLUMNS]:,
        F: FnMut(T) -> Y,
    {
        Matrix::new(self.0.map(map))
    }

    pub const fn as_ref<'a>(&'a self) -> Matrix<&'a T, ROWS, COLUMNS>
    where
        [&'a T; ROWS * COLUMNS]:,
    {
        unsafe {
            let mut mat: [MaybeUninit<&'a T>; ROWS * COLUMNS] =
                [const { MaybeUninit::uninit() }; ROWS * COLUMNS];
            let mut i = 0usize;
            while i < ROWS * COLUMNS {
                mat[i].write(&self.0[i]);
                i += 1;
            }
            Matrix::new(MaybeUninit::array_assume_init(mat))
        }
    }

    pub const fn as_mut<'a>(&'a mut self) -> Matrix<&'a mut T, ROWS, COLUMNS>
    where
        [&'a mut T; ROWS * COLUMNS]:,
    {
        unsafe {
            let mut mat: [MaybeUninit<&'a mut T>; ROWS * COLUMNS] =
                [const { MaybeUninit::uninit() }; ROWS * COLUMNS];
            let mut i = 0usize;
            while i < ROWS * COLUMNS {
                mat[i].write((&mut self.0[i] as *mut T).as_mut().unwrap_unchecked());
                i += 1;
            }
            Matrix::new(MaybeUninit::array_assume_init(mat))
        }
    }
}
impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS>
where
    [T; ROWS * COLUMNS]:,
    T: Num,
{
    const ZERO: Self = Self::new([T::ZERO; ROWS * COLUMNS]);
}
// impl<T, I, const ROWS: usize, const COLUMNS: usize> From<[I; ROWS * COLUMNS]>
//     for Matrix<T, ROWS, COLUMNS>
// where
//     [I; ROWS * COLUMNS]:,
//     I: Into<T>,
// {
//     fn from(value: [I; ROWS * COLUMNS]) -> Self {
//         Self(value.map(|x| x.into()))
//     }
// }
impl<T, I, const ROWS: usize, const COLUMNS: usize> TryFrom<[I; ROWS * COLUMNS]>
    for Matrix<T, ROWS, COLUMNS>
where
    [I; ROWS * COLUMNS]:,
    I: TryInto<T>,
{
    type Error = I::Error;

    fn try_from(value: [I; ROWS * COLUMNS]) -> Result<Self, Self::Error> {
        Ok(Self(value.try_map(|x| x.try_into())?))
    }
}
impl<A, B, const ROWS: usize, const COLUMNS: usize> PartialEq<Matrix<B, ROWS, COLUMNS>>
    for Matrix<A, ROWS, COLUMNS>
where
    [A; ROWS * COLUMNS]:,
    [B; ROWS * COLUMNS]:,
    A: PartialEq<B>,
{
    fn eq(&self, other: &Matrix<B, ROWS, COLUMNS>) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a == b)
    }
}
impl<T, R, const ROWS: usize, const COLUMNS: usize> Mul<T> for Matrix<T, ROWS, COLUMNS>
where
    [T; ROWS * COLUMNS]:,
    T: Mul<Output = R> + Clone,
{
    type Output = Matrix<R, ROWS, COLUMNS>;

    fn mul(self, rhs: T) -> Self::Output {
        self.map(|x| x * rhs.clone())
    }
}
impl<T, const SIDE: usize, const EXTRA1: usize, const EXTRA2: usize> Mul<Matrix<T, EXTRA2, SIDE>>
    for Matrix<T, SIDE, EXTRA1>
where
    [T; SIDE * EXTRA1]:,
    [T; EXTRA2 * SIDE]:,
    [T; EXTRA2 * EXTRA1]:,
    T: Mul<Output = T> + Add<Output = T> + Clone,
{
    type Output = Matrix<T, EXTRA2, EXTRA1>;

    fn mul(self, rhs: Matrix<T, EXTRA2, SIDE>) -> Self::Output {
        unsafe {
            let mut new = Self::Output::new_uninit();
            for x in 0..EXTRA2 {
                for y in 0..EXTRA1 {
                    let mut sum: Option<T> = None;
                    for i in 0..SIDE {
                        match &mut sum {
                            Some(sum) => {
                                #[allow(clippy::uninit_assumed_init)]
                                let mut t = MaybeUninit::uninit().assume_init();
                                std::mem::swap(sum, &mut t);
                                t = self.get(i, y).unwrap().clone()
                                    * rhs.get(x, i).unwrap().clone()
                                    + t;
                                std::mem::swap(sum, &mut t);
                                std::mem::forget(t);
                            }
                            None => {
                                sum = Some(
                                    T::clone(self.get(i, y).unwrap())
                                        * T::clone(rhs.get(x, i).unwrap()),
                                )
                            }
                        }
                    }
                    new.get_unchecked_mut(x, y).write(sum.unwrap_unchecked());
                }
            }
            new.assume_init()
        }
    }
}
impl<T, const ROWS: usize, const COLUMNS: usize> Index<(usize, usize)> for Matrix<T, ROWS, COLUMNS>
where
    [T; ROWS * COLUMNS]:,
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1).unwrap_or_else(|| {
            panic!(
                "Indexing a matrix ({ROWS}x{COLUMNS}) out of bounds ({}, {})",
                index.0, index.1
            )
        })
    }
}
impl<T, const ROWS: usize, const COLUMNS: usize> IndexMut<(usize, usize)>
    for Matrix<T, ROWS, COLUMNS>
where
    [T; ROWS * COLUMNS]:,
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1).unwrap_or_else(|| {
            panic!(
                "Indexing a matrix ({ROWS}x{COLUMNS}) out of bounds ({}, {})",
                index.0, index.1
            )
        })
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

pub type Mat4f = Matrix<f32, 4, 4>;
/// Identity matrix for f32 4x4 matrix.
pub const MAT4F_IDENTITY: Mat4f = Mat4f::IDENTITY;
pub type Mat3f = Matrix<f32, 3, 3>;
/// Identity matrix for f32 3x3 matrix.
pub const MAT3F_IDENTITY: Mat3f = Mat3f::IDENTITY;

// https://www.mathsisfun.com/algebra/matrix-multiplying.html

#[cfg(test)]
mod tests {
    use super::{Mat3f, Matrix};

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
            mat! { <f32, 2, 3>
                1.0, 4.0,
                2.0, 5.0,
                3.0, 6.0,
            },
            mat! { <f32, 3, 2>
                1.0, 2.0, 3.0,
                4.0, 5.0, 6.0,
            }
            .transposed()
            .copied(),
        );
    }

    #[test]
    fn identity_test() {
        let matrix: Matrix<f32, 3, 2> = mat! {
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        };

        assert_eq!(matrix * Mat3f::IDENTITY, matrix);
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
