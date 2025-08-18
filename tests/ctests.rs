use std::{path::Path, process::Command};

fn compile(test: &str) {
    let src = Path::new(file!())
        .parent()
        .unwrap()
        .join(format!("{test}.c"));
    let dst = Path::new(env!("CARGO_TARGET_TMPDIR")).join(if cfg!(target_os = "windows") {
        format!("test_{test}.exe")
    } else {
        format!("test_{test}")
    });

    if let Ok(x) = std::env::var("CC") {
        if !Command::new(&x)
            .arg(&src)
            .arg("-o")
            .arg(&dst)
            .arg("-Wall")
            .arg("-g")
            .status()
            .unwrap()
            .success()
        {
            panic!("failed to build test (compiler: {x:?})");
        }
        if !Command::new(&dst).status().unwrap().success() {
            panic!("failed to run test");
        }
        return;
    }
    macro_rules! try_compilers {
        ($($compiler:expr),* $(,)?) => {$(
            if let Ok(mut x) = Command::new($compiler).arg(&src).arg("-o").arg(&dst).spawn() {
                if !x.wait().unwrap().success() {
                    panic!("failed to compile test");
                }
                if !Command::new(&dst).status().unwrap().success() {
                    panic!("failed to run test");
                }
                return;
            }
        )*};
    }
    try_compilers!("clang", "gcc", "cl");

    panic!("could not find a C compiler");
}

macro_rules! ctests {
    ($($name:ident)*, $(,)?) => {$(
        #[test]
        fn $name() {
            compile(stringify!($name));
        }
    )*};
}

ctests! {
    str,
}
