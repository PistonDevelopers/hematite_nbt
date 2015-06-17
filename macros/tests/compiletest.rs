extern crate compiletest_rs as compiletest;

use compiletest::common::Mode;

use std::path::PathBuf;

#[test]
fn test_compilation() {
    let mut config = compiletest::default_config();

    // Test compilation failures.

    config.mode = Mode::CompileFail;
    config.target_rustcflags = Some("-L target/debug/ -L target/debug/deps".to_string());
    config.src_base = PathBuf::from("tests/compile-fail");
    config.verbose = true;

    compiletest::run_tests(&config);

    // Test compilation successes.

    config.mode = Mode::RunPass;
    config.src_base = PathBuf::from("tests/run-pass");

    compiletest::run_tests(&config);
}
