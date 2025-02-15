use crate::common::util::*;
use std::io::Read;

#[test]
fn test_rejects_nan() {
    let ts = TestScenario::new(util_name!());

    ts.ucmd().args(&["NaN"]).fails().stderr_only(format!(
        "{0}: invalid 'not-a-number' argument: 'NaN'\nTry '{1} {0} --help' for more information.",
        ts.util_name,
        ts.bin_path.to_string_lossy()
    ));
}

#[test]
fn test_rejects_non_floats() {
    let ts = TestScenario::new(util_name!());

    ts.ucmd().args(&["foo"]).fails().stderr_only(&format!(
        "{0}: invalid floating point argument: 'foo'\nTry '{1} {0} --help' for more information.",
        ts.util_name,
        ts.bin_path.to_string_lossy()
    ));
}

#[test]
fn test_invalid_float() {
    new_ucmd!()
        .args(&["1e2.3"])
        .fails()
        .no_stdout()
        .stderr_contains("invalid floating point argument: '1e2.3'")
        .stderr_contains("for more information.");
    new_ucmd!()
        .args(&["1e2.3", "2"])
        .fails()
        .no_stdout()
        .stderr_contains("invalid floating point argument: '1e2.3'")
        .stderr_contains("for more information.");
    new_ucmd!()
        .args(&["1", "1e2.3"])
        .fails()
        .no_stdout()
        .stderr_contains("invalid floating point argument: '1e2.3'")
        .stderr_contains("for more information.");
    new_ucmd!()
        .args(&["1e2.3", "2", "3"])
        .fails()
        .no_stdout()
        .stderr_contains("invalid floating point argument: '1e2.3'")
        .stderr_contains("for more information.");
    new_ucmd!()
        .args(&["1", "1e2.3", "3"])
        .fails()
        .no_stdout()
        .stderr_contains("invalid floating point argument: '1e2.3'")
        .stderr_contains("for more information.");
    new_ucmd!()
        .args(&["1", "2", "1e2.3"])
        .fails()
        .no_stdout()
        .stderr_contains("invalid floating point argument: '1e2.3'")
        .stderr_contains("for more information.");
}

#[test]
fn test_width_invalid_float() {
    new_ucmd!()
        .args(&["-w", "1e2.3"])
        .fails()
        .no_stdout()
        .stderr_contains("invalid floating point argument: '1e2.3'")
        .stderr_contains("for more information.");
}

// ---- Tests for the big integer based path ----

#[test]
fn test_count_up() {
    new_ucmd!()
        .args(&["10"])
        .run()
        .stdout_is("1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n");
}

#[test]
fn test_count_down() {
    new_ucmd!()
        .args(&["--", "5", "-1", "1"])
        .run()
        .stdout_is("5\n4\n3\n2\n1\n");
    new_ucmd!()
        .args(&["5", "-1", "1"])
        .run()
        .stdout_is("5\n4\n3\n2\n1\n");
}

#[test]
fn test_separator_and_terminator() {
    new_ucmd!()
        .args(&["-s", ",", "-t", "!", "2", "6"])
        .run()
        .stdout_is("2,3,4,5,6!");
    new_ucmd!()
        .args(&["-s", ",", "2", "6"])
        .run()
        .stdout_is("2,3,4,5,6\n");
    new_ucmd!()
        .args(&["-s", "\n", "2", "6"])
        .run()
        .stdout_is("2\n3\n4\n5\n6\n");
    new_ucmd!()
        .args(&["-s", "\\n", "2", "6"])
        .run()
        .stdout_is("2\\n3\\n4\\n5\\n6\n");
}

#[test]
fn test_equalize_widths() {
    new_ucmd!()
        .args(&["-w", "5", "10"])
        .run()
        .stdout_is("05\n06\n07\n08\n09\n10\n");
}

#[test]
fn test_seq_wrong_arg() {
    new_ucmd!().args(&["-w", "5", "10", "33", "32"]).fails();
}

#[test]
fn test_zero_step() {
    new_ucmd!().args(&["10", "0", "32"]).fails();
}

#[test]
fn test_big_numbers() {
    new_ucmd!()
        .args(&[
            "1000000000000000000000000000",
            "1000000000000000000000000001",
        ])
        .succeeds()
        .stdout_only("1000000000000000000000000000\n1000000000000000000000000001\n");
}

// ---- Tests for the floating point based path ----

#[test]
fn test_count_up_floats() {
    new_ucmd!()
        .args(&["10.0"])
        .run()
        .stdout_is("1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n");
}

#[test]
fn test_count_down_floats() {
    new_ucmd!()
        .args(&["--", "5", "-1.0", "1"])
        .run()
        .stdout_is("5.0\n4.0\n3.0\n2.0\n1.0\n");
    new_ucmd!()
        .args(&["5", "-1", "1.0"])
        .run()
        .stdout_is("5\n4\n3\n2\n1\n");
}

#[test]
fn test_separator_and_terminator_floats() {
    new_ucmd!()
        .args(&["-s", ",", "-t", "!", "2.0", "6"])
        .run()
        .stdout_is("2.0,3.0,4.0,5.0,6.0!");
}

#[test]
fn test_equalize_widths_floats() {
    new_ucmd!()
        .args(&["-w", "5", "10.0"])
        .run()
        .stdout_is("05\n06\n07\n08\n09\n10\n");
}

#[test]
fn test_seq_wrong_arg_floats() {
    new_ucmd!().args(&["-w", "5", "10.0", "33", "32"]).fails();
}

#[test]
fn test_zero_step_floats() {
    new_ucmd!().args(&["10.0", "0", "32"]).fails();
}

#[test]
fn test_preserve_negative_zero_start() {
    new_ucmd!()
        .args(&["-0", "1"])
        .succeeds()
        .stdout_is("-0\n1\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-0", "1", "2"])
        .succeeds()
        .stdout_is("-0\n1\n2\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-0", "1", "2.0"])
        .succeeds()
        .stdout_is("-0\n1\n2\n")
        .no_stderr();
}

#[test]
fn test_drop_negative_zero_end() {
    new_ucmd!()
        .args(&["1", "-1", "-0"])
        .succeeds()
        .stdout_is("1\n0\n")
        .no_stderr();
}

#[test]
fn test_width_scientific_notation() {
    new_ucmd!()
        .args(&["-w", "999", "1e3"])
        .succeeds()
        .stdout_is("0999\n1000\n")
        .no_stderr();
}

#[test]
fn test_width_negative_zero() {
    new_ucmd!()
        .args(&["-w", "-0", "1"])
        .succeeds()
        .stdout_is("-0\n01\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0", "1", "2"])
        .succeeds()
        .stdout_is("-0\n01\n02\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0", "1", "2.0"])
        .succeeds()
        .stdout_is("-0\n01\n02\n")
        .no_stderr();
}

#[test]
fn test_width_negative_zero_decimal_notation() {
    new_ucmd!()
        .args(&["-w", "-0.0", "1"])
        .succeeds()
        .stdout_is("-0.0\n01.0\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0.0", "1.0"])
        .succeeds()
        .stdout_is("-0.0\n01.0\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0.0", "1", "2"])
        .succeeds()
        .stdout_is("-0.0\n01.0\n02.0\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0.0", "1", "2.0"])
        .succeeds()
        .stdout_is("-0.0\n01.0\n02.0\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0.0", "1.0", "2"])
        .succeeds()
        .stdout_is("-0.0\n01.0\n02.0\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0.0", "1.0", "2.0"])
        .succeeds()
        .stdout_is("-0.0\n01.0\n02.0\n")
        .no_stderr();
}

#[test]
fn test_width_negative_zero_scientific_notation() {
    new_ucmd!()
        .args(&["-w", "-0e0", "1"])
        .succeeds()
        .stdout_is("-0\n01\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0e0", "1", "2"])
        .succeeds()
        .stdout_is("-0\n01\n02\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0e0", "1", "2.0"])
        .succeeds()
        .stdout_is("-0\n01\n02\n")
        .no_stderr();

    new_ucmd!()
        .args(&["-w", "-0e+1", "1"])
        .succeeds()
        .stdout_is("-00\n001\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0e+1", "1", "2"])
        .succeeds()
        .stdout_is("-00\n001\n002\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0e+1", "1", "2.0"])
        .succeeds()
        .stdout_is("-00\n001\n002\n")
        .no_stderr();

    new_ucmd!()
        .args(&["-w", "-0.000e0", "1"])
        .succeeds()
        .stdout_is("-0.000\n01.000\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0.000e0", "1", "2"])
        .succeeds()
        .stdout_is("-0.000\n01.000\n02.000\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0.000e0", "1", "2.0"])
        .succeeds()
        .stdout_is("-0.000\n01.000\n02.000\n")
        .no_stderr();

    new_ucmd!()
        .args(&["-w", "-0.000e-2", "1"])
        .succeeds()
        .stdout_is("-0.00000\n01.00000\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0.000e-2", "1", "2"])
        .succeeds()
        .stdout_is("-0.00000\n01.00000\n02.00000\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0.000e-2", "1", "2.0"])
        .succeeds()
        .stdout_is("-0.00000\n01.00000\n02.00000\n")
        .no_stderr();

    new_ucmd!()
        .args(&["-w", "-0.000e5", "1"])
        .succeeds()
        .stdout_is("-000000\n0000001\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0.000e5", "1", "2"])
        .succeeds()
        .stdout_is("-000000\n0000001\n0000002\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0.000e5", "1", "2.0"])
        .succeeds()
        .stdout_is("-000000\n0000001\n0000002\n")
        .no_stderr();

    new_ucmd!()
        .args(&["-w", "-0.000e5", "1"])
        .succeeds()
        .stdout_is("-000000\n0000001\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0.000e5", "1", "2"])
        .succeeds()
        .stdout_is("-000000\n0000001\n0000002\n")
        .no_stderr();
    new_ucmd!()
        .args(&["-w", "-0.000e5", "1", "2.0"])
        .succeeds()
        .stdout_is("-000000\n0000001\n0000002\n")
        .no_stderr();
}

#[test]
fn test_width_decimal_scientific_notation_increment() {
    new_ucmd!()
        .args(&["-w", ".1", "1e-2", ".11"])
        .succeeds()
        .stdout_is("0.10\n0.11\n")
        .no_stderr();

    new_ucmd!()
        .args(&["-w", ".0", "1.500e-1", ".2"])
        .succeeds()
        .stdout_is("0.0000\n0.1500\n")
        .no_stderr();
}

/// Test that trailing zeros in the start argument contribute to precision.
#[test]
fn test_width_decimal_scientific_notation_trailing_zeros_start() {
    new_ucmd!()
        .args(&["-w", ".1000", "1e-2", ".11"])
        .succeeds()
        .stdout_is("0.1000\n0.1100\n")
        .no_stderr();
}

/// Test that trailing zeros in the increment argument contribute to precision.
#[test]
fn test_width_decimal_scientific_notation_trailing_zeros_increment() {
    new_ucmd!()
        .args(&["-w", "1e-1", "0.0100", ".11"])
        .succeeds()
        .stdout_is("0.1000\n0.1100\n")
        .no_stderr();
}

/// Test that trailing zeros in the end argument do not contribute to width.
#[test]
fn test_width_decimal_scientific_notation_trailing_zeros_end() {
    new_ucmd!()
        .args(&["-w", "1e-1", "1e-2", ".1100"])
        .succeeds()
        .stdout_is("0.10\n0.11\n")
        .no_stderr();
}

#[test]
fn test_width_floats() {
    new_ucmd!()
        .args(&["-w", "9.0", "10.0"])
        .succeeds()
        .stdout_is("09.0\n10.0\n")
        .no_stderr();
}

// TODO This is duplicated from `test_yes.rs`; refactor them.
/// Run `seq`, capture some of the output, close the pipe, and verify it.
fn run(args: &[&str], expected: &[u8]) {
    let mut cmd = new_ucmd!();
    let mut child = cmd.args(args).run_no_wait();
    let mut stdout = child.stdout.take().unwrap();
    let mut buf = vec![0; expected.len()];
    stdout.read_exact(&mut buf).unwrap();
    drop(stdout);
    assert!(child.wait().unwrap().success());
    assert_eq!(buf.as_slice(), expected);
}

#[test]
fn test_neg_inf() {
    run(&["--", "-inf", "0"], b"-inf\n-inf\n-inf\n");
}

#[test]
fn test_inf() {
    run(&["inf"], b"1\n2\n3\n");
}

#[test]
fn test_ignore_leading_whitespace() {
    new_ucmd!()
        .arg("   1")
        .succeeds()
        .stdout_is("1\n")
        .no_stderr();
}

#[test]
fn test_trailing_whitespace_error() {
    // In some locales, the GNU error message has curly quotes (‘)
    // instead of straight quotes ('). We just test the straight single
    // quotes.
    new_ucmd!()
        .arg("1 ")
        .fails()
        .no_stdout()
        .stderr_contains("seq: invalid floating point argument: '1 '")
        // FIXME The second line of the error message is "Try 'seq
        // --help' for more information."
        .stderr_contains("for more information.");
}
