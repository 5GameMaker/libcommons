//! libcommons args parsing utils.

use std::borrow::Cow;

use crate::util::Fun;

/// Split a single arg into a parameter and a value.
///
/// ```
/// use libcommons::args::{double_dash, double_dash_param, FullArg};
/// use std::{process::exit, iter::once};
///
/// fn print_help() {
///     println!("usage: --hello WHO");
/// }
///
/// let mut hello = None;
///
/// let mut iter = once(FullArg { text: "--hello=kitty".to_string(), verbatim_params: false });
/// let fullarg = iter.next().unwrap();
/// if let Some(arg) = fullarg.text.strip_prefix("--") {
///     let (arg, param) = double_dash(arg);
///
///     match arg {
///         "hello" => {
///             let param = double_dash_param(&mut iter, param).expect("no parameter was specified");
///             hello = Some(param.to_string());
///         }
///         "help" => {
///             print_help();
///             if param.is_some() {
///                 println!("--help: no parameter required");
///                 exit(1);
///             }
///             exit(0);
///         }
///         x => {
///             print_help();
///             println!("unknown param {x:?}");
///             exit(0);
///         }
///     }
/// }
///
/// assert_eq!(hello, Some(String::from("kitty")));
/// ```
#[must_use]
pub fn double_dash(arg: &str) -> (&str, Option<&str>) {
    arg.split_once('=').map(|(a, b)| (a, Some(b))).unwrap_or((arg, None))
}

/// Obtain a parameter for an arg.
#[must_use]
pub fn double_dash_param<I: Iterator<Item = FullArg>>(mut iter: I, param: Option<&str>) -> Option<Cow<'_, str>> {
    match param {
        Some(x) => Some(Cow::Borrowed(x)),
        None => iter.next().map(|x| Cow::Owned(x.text)),
    }
}

/// Get the name of the parameter, including the double bash.
#[must_use]
pub fn double_dash_name(fullarg: &FullArg) -> &str {
    fullarg.text.split_once('=').map(|x| x.0).unwrap_or(&fullarg.text)
}

/// Obtain a shell-friendly string.
///
/// ```
/// use std::borrow::Cow;
/// use libcommons::args::sys_str;
///
/// assert_eq!(sys_str("hello"), Cow::Borrowed("hello"));
/// assert_eq!(sys_str("hello'").as_ref(), "\"hello'\"");
/// assert_eq!(sys_str("hello\"").as_ref(), "'hello\"'");
/// assert_eq!(sys_str("a#a").as_ref(), "a#a");
/// assert_eq!(sys_str("#aa").as_ref(), "'#aa'");
/// assert_eq!(sys_str("-").as_ref(), "'-'");
/// ```
///
/// ## Consistency
///
/// Output of this function should be considered to be undefined. It may
/// change between operating systems, shells, and versions of libcommons
/// without major version increase.
#[must_use]
pub fn sys_str(s: &str) -> Cow<'_, str> {
    if !s.is_empty() && s != "-" && s != ":" &&
        s.chars().enumerate().all(|(i, x)| if i == 0 {
            "./_-+@:".chars().any(|y| x == y) || !x.is_ascii_punctuation()
        } else {
            "./_-+@:^#%".chars().any(|y| x == y) || !x.is_ascii_punctuation()
        } && !x.is_whitespace())
    { Cow::Borrowed(s) }
    else if s.chars().all(|x| x != '\'') { Cow::Owned(format!("'{s}'")) }
    else { Cow::Owned(format!("{s:?}")) }
}

pub struct FullArg {
    pub text: String,
    /// Whether prefixes like '--' or '-' should not change the type of parameter.
    pub verbatim_params: bool,
}

pub struct ArgsIter {
    iter: std::env::Args,
    verbatim_params: bool,
}
impl ArgsIter {
    /// Create a new [ArgsIter] wrapping an instance of [Args][std::env::Args].
    pub fn new() -> Self { Self {
        iter: std::env::args(),
        verbatim_params: false,
    } }

    /// Whether prefixes like '--' or '-' should not change the type of parameter.
    pub fn verbatim_params(&self) -> bool { self.verbatim_params }

    /// Should be called after a '--'.
    ///
    /// Sets [verbatim_params][FullArg::verbatim_params] to `true`.
    pub fn set_verbatim_params(&mut self) { self.verbatim_params = true; }
}
impl Iterator for ArgsIter {
    type Item = FullArg;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|text| FullArg {
            text,
            verbatim_params: self.verbatim_params,
        })
    }
}

pub enum SomeArgsIter {
    Args(ArgsIter),
}
impl SomeArgsIter {
    /// Whether prefixes like '--' or '-' should not change the type of parameter.
    pub fn verbatim_params(&self) -> bool {
        match self {
            Self::Args(x) => x.verbatim_params(),
        }
    }

    /// Should be called after a '--'.
    ///
    /// Sets [verbatim_params][FullArg::verbatim_params] to `true`.
    pub fn set_verbatim_params(&mut self) {
        match self {
            SomeArgsIter::Args(x) => x.set_verbatim_params(),
        }
    }
}
impl Iterator for SomeArgsIter {
    type Item = FullArg;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SomeArgsIter::Args(x) => x.next(),
        }
    }
}

/// A collection of [SomeArgsIter].
///
/// This iterator requires that the user calls [pop_if_done][ArgsIters::pop_if_done]
/// before each time the next command line argument is requested.
///
/// ```
/// use libcommons::args::ArgsIters;
///
/// let mut iters = ArgsIters::new_args();
///
/// while let Some(fullarg) = iters.next_and_pop() {
///     if let Some(arg) = fullarg.text.strip_prefix("--") {
///         
///     }
/// }
/// ```
pub struct ArgsIters {
    iters: Vec<SomeArgsIter>,
    pop_if_done: bool,
}
impl ArgsIters {
    /// Create an instance of [ArgsIters] containing an [ArgsIter].
    pub fn new_args() -> Self { Self {
        iters: vec![SomeArgsIter::Args(ArgsIter::new())],
        pop_if_done: false,
    } }

    /// Whether prefixes like '--' or '-' should not change the type of parameter.
    pub fn verbatim_params(&self) -> bool {
        self.iters.last().is_some_and(|x| x.verbatim_params())
    }

    /// Should be called after a '--'.
    ///
    /// Sets [verbatim_params][FullArg::verbatim_params] to `true`.
    ///
    /// Is only going to be enabled for the current iterator, so when
    /// it ends the flag mey get unset.
    pub fn set_verbatim_params(&mut self) {
        if let Some(x) = self.iters.last_mut() {
            x.set_verbatim_params();
        }
    }

    /// Append an args iterator.
    ///
    /// i.e. if the application supports `@<file>` syntax, the specified file
    /// will be parsed and pushed to this collector. It will be current until
    /// it is exhausted.
    pub fn push(&mut self, iter: SomeArgsIter) {
        self.pop_if_done = false;
        self.iters.push(iter);
    }

    /// Get the next element while popping all exhausted iterators.
    pub fn next_and_pop(&mut self) -> Option<FullArg> {
        self.pop_if_done();
        self.next()
    }

    /// Pop all exhausted iterators.
    ///
    /// This is not done automatically to avoid params with args consuming
    /// freestanding args from parent iterators.
    pub fn pop_if_done(&mut self) {
        self.pop_if_done = true;
    }
}
impl Iterator for ArgsIters {
    type Item = FullArg;

    fn next(&mut self) -> Option<Self::Item> {
        let pop_if_done = self.pop_if_done;
        self.pop_if_done = false;

        while let Some(x) = self.iters.last_mut() {
            let v = x.next();
            if v.is_none() && pop_if_done {
                self.iters.pop();
                continue;
            }
            return v;
        }

        None
    }
}
