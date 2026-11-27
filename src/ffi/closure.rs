#![allow(clippy::too_many_arguments)]

use std::{ffi::c_void, marker::PhantomData, mem::transmute};

/// A more explicit syntax for [Closure].
///
/// Equivalent to `Closure<A0, A1, A2..., R>`.
#[macro_export]
macro_rules! Closure {
    ($($a:ty),* $(,)? => $r:ty) => {
        $crate::ffi::closure::Closure<($($a,),* $r,)>
    };
}
/// A reference to a closure.
///
/// On nightly implements [Fn], [FnMut] and [FnOnce].
#[repr(C)]
pub struct Closure<'a, F: ?Sized> {
    ptr: *const c_void,
    data: *const c_void,
    _phantom: PhantomData<&'a F>,
}

/// A more explicit syntax for [ClosureMut].
///
/// Equivalent to `ClosureMut<A0, A1, A2..., R>`.
#[macro_export]
macro_rules! ClosureMut {
    ($($a:ty),* $(,)? => $r:ty) => {
        $crate::ffi::closure::ClosureMut<($($a,),* $r,)>
    };
}
/// A reference to a closure.
///
/// On nightly implements [FnMut] and [FnOnce].
#[repr(C)]
pub struct ClosureMut<'a, F: ?Sized> {
    ptr: *const c_void,
    data: *mut c_void,
    _phantom: PhantomData<&'a F>,
}

/// A more explicit syntax for [ClosureOnce].
///
/// Equivalent to `ClosureOnce<A0, A1, A2..., R>`.
#[macro_export]
macro_rules! ClosureOnce {
    ($($a:ty),* $(,)? => $r:ty) => {
        $crate::ffi::closure::ClosureOnce<($($a,),* $r,)>
    };
}
/// A reference to a closure.
///
/// On nightly implements [FnOnce].
#[repr(C)]
pub struct ClosureOnce<'a, F: ?Sized> {
    ptr: *const c_void,
    data: *mut c_void,
    drop: extern "C" fn(data: *mut c_void),
    _phantom: PhantomData<&'a F>,
}

macro_rules! impl_closure {
    ($([$idx:tt] $arg:ident $ge:ident)*) => {
        impl<'a, $($ge,)* R> Closure<'a, ($($ge,)* R,)> {
            pub fn new<F: Fn($($ge),*) -> R>(closure: &F) -> Closure<'a, F> {
                Closure {
                    data: (closure as *const F).cast(),
                    ptr: {
                        extern "C" fn run<$($ge,)* R, F: Fn($($ge),*) -> R>(data: *const c_void $(,$arg: $ge)*) -> R {
                            unsafe {
                                let data = data.cast::<F>().as_ref().unwrap();
                                data($($arg),*)
                            }
                        }
                        run::<$($ge,)* R, F> as *const c_void
                    },
                    _phantom: PhantomData,
                }
            }

            pub fn raw_data(&self) -> *const c_void { self.data }
            /// Obtain the function this closure wraps around.
            pub fn raw_fn(&self) -> unsafe extern "C" fn(data: *const c_void $(,$arg: $ge)*) -> R {
                unsafe { transmute::<
                    *const c_void,
                    extern "C" fn(data: *const c_void $(,$arg: $ge)*) -> R>(self.ptr) }
            }

            /// Run this closure.
            pub fn run(&self $(,$arg: $ge)*) -> R {
                unsafe { self.raw_fn()(self.raw_data() $(,$arg)*) }
            }
        }
        #[cfg(libcommons_rustc_nightly = "y")]
        impl<'a, $($ge,)* R> FnOnce<($($ge,)*)> for Closure<'a, ($($ge,)* R,)> {
            type Output = R;
            extern "rust-call" fn call_once(self, #[allow(unused)] args: ($($ge,)*)) -> Self::Output {
                self.run($(args.$idx),*)
            }
        }
        #[cfg(libcommons_rustc_nightly = "y")]
        impl<'a, $($ge,)* R> FnMut<($($ge,)*)> for Closure<'a, ($($ge,)* R,)> {
            extern "rust-call" fn call_mut(&mut self, #[allow(unused)] args: ($($ge,)*)) -> Self::Output {
                self.run($(args.$idx),*)
            }
        }
        #[cfg(libcommons_rustc_nightly = "y")]
        impl<'a, $($ge,)* R> Fn<($($ge,)*)> for Closure<'a, ($($ge,)* R,)> {
            extern "rust-call" fn call(&self, #[allow(unused)] args: ($($ge,)*)) -> Self::Output {
                self.run($(args.$idx),*)
            }
        }

        impl<'a, $($ge,)* R> ClosureMut<'a, ($($ge,)* R,)> {
            pub fn new<F: FnMut($($ge),*) -> R>(closure: &mut F) -> ClosureMut<'a, F> {
                ClosureMut {
                    data: (closure as *mut F).cast(),
                    ptr: {
                        extern "C" fn run<$($ge,)* R, F: FnMut($($ge),*) -> R>(data: *mut c_void $(,$arg: $ge)*) -> R {
                            unsafe {
                                let data = data.cast::<F>().as_mut().unwrap();
                                data($($arg),*)
                            }
                        }
                        run::<$($ge,)* R, F> as *const c_void
                    },
                    _phantom: PhantomData,
                }
            }

            pub fn raw_data(&self) -> *mut c_void { self.data }
            /// Obtain the function this closure wraps around.
            pub fn raw_fn(&self) -> unsafe extern "C" fn(data: *mut c_void $(,$arg: $ge)*) -> R {
                unsafe { transmute::<
                    *const c_void,
                    extern "C" fn(data: *mut c_void $(,$arg: $ge)*) -> R>(self.ptr) }
            }

            /// Run this closure.
            pub fn run(&mut self $(,$arg: $ge)*) -> R {
                unsafe { self.raw_fn()(self.raw_data() $(,$arg)*) }
            }
        }
        #[cfg(libcommons_rustc_nightly = "y")]
        impl<'a, $($ge,)* R> FnOnce<($($ge,)*)> for ClosureMut<'a, ($($ge,)* R,)> {
            type Output = R;
            extern "rust-call" fn call_once(mut self, #[allow(unused)] args: ($($ge,)*)) -> Self::Output {
                self.run($(args.$idx),*)
            }
        }
        #[cfg(libcommons_rustc_nightly = "y")]
        impl<'a, $($ge,)* R> FnMut<($($ge,)*)> for ClosureMut<'a, ($($ge,)* R,)> {
            extern "rust-call" fn call_mut(&mut self, #[allow(unused)] args: ($($ge,)*)) -> Self::Output {
                self.run($(args.$idx),*)
            }
        }

        impl<'a, $($ge,)* R> ClosureOnce<'a, ($($ge,)* R,)> {
            pub fn new<F: FnOnce($($ge),*) -> R>(closure: F) -> ClosureOnce<'a, F> {
                ClosureOnce {
                    data: (Box::leak(Box::new(closure)) as *mut F).cast(),
                    ptr: {
                        extern "C" fn run<$($ge,)* R, F: FnOnce($($ge),*) -> R>(data: *mut c_void $(,$arg: $ge)*) -> R {
                            unsafe {
                                let data = Box::from_raw(data.cast::<F>());
                                data($($arg),*)
                            }
                        }
                        run::<$($ge,)* R, F> as *const c_void
                    },
                    drop: {
                        extern "C" fn run<T>(data: *mut c_void) {
                            unsafe {
                                std::ptr::drop_in_place(data.cast::<T>());
                            }
                        }
                        run::<Self>
                    },
                    _phantom: PhantomData,
                }
            }

            pub fn raw_data(&self) -> *mut c_void { self.data }
            /// Obtain the function this closure wraps around.
            pub fn raw_fn(&self) -> unsafe extern "C" fn(data: *mut c_void $(,$arg: $ge)*) -> R {
                unsafe { transmute::<
                    *const c_void,
                    extern "C" fn(data: *mut c_void $(,$arg: $ge)*) -> R>(self.ptr) }
            }

            /// Run this closure.
            pub fn run(self $(,$arg: $ge)*) -> R {
                let r = unsafe { self.raw_fn()(self.raw_data() $(,$arg)*) };
                std::mem::forget(self);
                r
            }
        }
        #[cfg(libcommons_rustc_nightly = "y")]
        impl<'a, $($ge,)* R> FnOnce<($($ge,)*)> for ClosureOnce<'a, ($($ge,)* R,)> {
            type Output = R;
            extern "rust-call" fn call_once(self, #[allow(unused)] args: ($($ge,)*)) -> Self::Output {
                self.run($(args.$idx),*)
            }
        }
    };
}
impl_closure!();
impl_closure!([0] arg0 A0);
impl_closure!([0] arg0 A0 [1] arg1 A1);
impl_closure!([0] arg0 A0 [1] arg1 A1 [2] arg2 A2);
impl_closure!([0] arg0 A0 [1] arg1 A1 [2] arg2 A2 [3] arg3 A3);
impl_closure!([0] arg0 A0 [1] arg1 A1 [2] arg2 A2 [3] arg3 A3 [4] arg4 A4);
impl_closure!([0] arg0 A0 [1] arg1 A1 [2] arg2 A2 [3] arg3 A3 [4] arg4 A4 [5] arg5 A5);
impl_closure!([0] arg0 A0 [1] arg1 A1 [2] arg2 A2 [3] arg3 A3 [4] arg4 A4 [5] arg5 A5 [6] arg6 A6);
impl_closure!([0] arg0 A0 [1] arg1 A1 [2] arg2 A2 [3] arg3 A3 [4] arg4 A4 [5] arg5 A5 [6] arg6 A6 [7] arg7 A7);
impl_closure!([0] arg0 A0 [1] arg1 A1 [2] arg2 A2 [3] arg3 A3 [4] arg4 A4 [5] arg5 A5 [6] arg6 A6 [7] arg7 A7 [8] arg8 A8);
impl_closure!([0] arg0 A0 [1] arg1 A1 [2] arg2 A2 [3] arg3 A3 [4] arg4 A4 [5] arg5 A5 [6] arg6 A6 [7] arg7 A7 [8] arg8 A8 [9] arg9 A9);
impl_closure!([0] arg0 A0 [1] arg1 A1 [2] arg2 A2 [3] arg3 A3 [4] arg4 A4 [5] arg5 A5 [6] arg6 A6 [7] arg7 A7 [8] arg8 A8 [9] arg9 A9 [10] arg10 A10);
impl_closure!([0] arg0 A0 [1] arg1 A1 [2] arg2 A2 [3] arg3 A3 [4] arg4 A4 [5] arg5 A5 [6] arg6 A6 [7] arg7 A7 [8] arg8 A8 [9] arg9 A9 [10] arg10 A10 [11] arg11 A11);
impl_closure!([0] arg0 A0 [1] arg1 A1 [2] arg2 A2 [3] arg3 A3 [4] arg4 A4 [5] arg5 A5 [6] arg6 A6 [7] arg7 A7 [8] arg8 A8 [9] arg9 A9 [10] arg10 A10 [11] arg11 A11 [12] arg12 A12);

impl<'a, T: ?Sized> Drop for ClosureOnce<'a, T> {
    fn drop(&mut self) { (self.drop)(self.data) }
}
