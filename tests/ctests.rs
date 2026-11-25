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

#[cfg(unix)]
#[cfg(feature = "dl")]
#[cfg_attr(unix, test)]
fn open_dl() {
    let src = Path::new(file!())
        .parent()
        .unwrap()
        .join("testlib.c");
    let dst = Path::new(env!("CARGO_TARGET_TMPDIR")).join(if cfg!(target_os = "windows") {
        "test_dl.dll"
    } else {
        "libtest_dl.so"
    });

    fn actual_test(path: &Path) {
        use libcommons::dl::LibraryFile;

        let dl = LibraryFile::open(path).expect("failed to open dl");
        let data = dl.section_data_by_name(".custom-section").expect("could not find section '.custom-section'");
        assert_eq!(data, b"oh hello\0");
    }

    if let Ok(x) = std::env::var("CC") {
        if !Command::new(&x)
            .arg(&src)
            .arg("-o")
            .arg(&dst)
            .arg("-shared")
            .arg("-Wall")
            .arg("-g")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success()
        {
            panic!("failed to build test (compiler: {x:?})");
        }
        actual_test(&dst);
        return;
    }
    macro_rules! try_compilers {
        ($($compiler:expr),* $(,)?) => {$(
            if let Ok(mut x) = Command::new($compiler)
                .arg(&src)
                .arg("-o")
                .arg(&dst)
                .arg("-shared")
                .arg("-Wall")
                .arg("-g")
                .spawn() {
                if !x.wait().unwrap().success() {
                    panic!("failed to compile test");
                }
                actual_test(&dst);
                return;
            }
        )*};
    }
    try_compilers!("clang", "gcc", "cl");

    panic!("could not find a C compiler");
}
